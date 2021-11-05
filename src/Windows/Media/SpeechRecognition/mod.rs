#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpeechContinuousRecognitionCompletedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpeechContinuousRecognitionCompletedEventArgs {
    type Vtable = ISpeechContinuousRecognitionCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3822086587, 58124, 24088, [66, 75, 127, 190, 129, 248, 251, 208]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechContinuousRecognitionCompletedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SpeechRecognitionResultStatus) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpeechContinuousRecognitionResultGeneratedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpeechContinuousRecognitionResultGeneratedEventArgs {
    type Vtable = ISpeechContinuousRecognitionResultGeneratedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(420027934, 28286, 23110, [64, 251, 118, 89, 79, 120, 101, 4]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechContinuousRecognitionResultGeneratedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpeechContinuousRecognitionSession(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpeechContinuousRecognitionSession {
    type Vtable = ISpeechContinuousRecognitionSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1780562948, 26132, 18936, [153, 162, 181, 233, 179, 160, 133, 200]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechContinuousRecognitionSession_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mode: SpeechContinuousRecognitionMode, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpeechRecognitionCompilationResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpeechRecognitionCompilationResult {
    type Vtable = ISpeechRecognitionCompilationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1082027101, 27335, 19876, [156, 193, 47, 206, 50, 207, 116, 137]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionCompilationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SpeechRecognitionResultStatus) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Media_SpeechRecognition`*"]
pub struct ISpeechRecognitionConstraint(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpeechRecognitionConstraint {
    type Vtable = ISpeechRecognitionConstraint_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2041321000, 19816, 17348, [137, 17, 64, 220, 65, 1, 181, 91]);
}
impl ISpeechRecognitionConstraint {
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn IsEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn Tag(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn SetTag<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<SpeechRecognitionConstraintType> {
        let this = self;
        unsafe {
            let mut result__: SpeechRecognitionConstraintType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionConstraintType>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn Probability(&self) -> ::windows::runtime::Result<SpeechRecognitionConstraintProbability> {
        let this = self;
        unsafe {
            let mut result__: SpeechRecognitionConstraintProbability = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionConstraintProbability>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn SetProbability(&self, value: SpeechRecognitionConstraintProbability) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ISpeechRecognitionConstraint {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{79ac1628-4d68-43c4-8911-40dc4101b55b}");
}
impl ::std::convert::From<ISpeechRecognitionConstraint> for ::windows::runtime::IUnknown {
    fn from(value: ISpeechRecognitionConstraint) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ISpeechRecognitionConstraint> for ::windows::runtime::IUnknown {
    fn from(value: &ISpeechRecognitionConstraint) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISpeechRecognitionConstraint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISpeechRecognitionConstraint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ISpeechRecognitionConstraint> for ::windows::runtime::IInspectable {
    fn from(value: ISpeechRecognitionConstraint) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISpeechRecognitionConstraint> for ::windows::runtime::IInspectable {
    fn from(value: &ISpeechRecognitionConstraint) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ISpeechRecognitionConstraint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ISpeechRecognitionConstraint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionConstraint_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SpeechRecognitionConstraintType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SpeechRecognitionConstraintProbability) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: SpeechRecognitionConstraintProbability) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpeechRecognitionGrammarFileConstraint(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpeechRecognitionGrammarFileConstraint {
    type Vtable = ISpeechRecognitionGrammarFileConstraint_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3036879503, 34250, 20388, [177, 26, 71, 79, 196, 27, 56, 53]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionGrammarFileConstraint_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpeechRecognitionGrammarFileConstraintFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpeechRecognitionGrammarFileConstraintFactory {
    type Vtable = ISpeechRecognitionGrammarFileConstraintFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1034383595, 50297, 19495, [159, 25, 137, 151, 78, 243, 146, 209]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionGrammarFileConstraintFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, file: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, file: ::windows::runtime::RawPtr, tag: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpeechRecognitionHypothesis(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpeechRecognitionHypothesis {
    type Vtable = ISpeechRecognitionHypothesis_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2054890928, 39365, 20349, [191, 132, 16, 170, 19, 2, 182, 52]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionHypothesis_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpeechRecognitionHypothesisGeneratedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpeechRecognitionHypothesisGeneratedEventArgs {
    type Vtable = ISpeechRecognitionHypothesisGeneratedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1427511930, 32803, 22630, [65, 29, 18, 19, 187, 39, 20, 118]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionHypothesisGeneratedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpeechRecognitionListConstraint(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpeechRecognitionListConstraint {
    type Vtable = ISpeechRecognitionListConstraint_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(163874793, 58541, 17702, [129, 242, 73, 70, 251, 72, 29, 152]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionListConstraint_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpeechRecognitionListConstraintFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpeechRecognitionListConstraintFactory {
    type Vtable = ISpeechRecognitionListConstraintFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1089719751, 22058, 17002, [159, 59, 59, 78, 40, 43, 225, 213]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionListConstraintFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, commands: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, commands: ::windows::runtime::RawPtr, tag: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpeechRecognitionQualityDegradingEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpeechRecognitionQualityDegradingEventArgs {
    type Vtable = ISpeechRecognitionQualityDegradingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1340227845, 35898, 19582, [141, 10, 91, 212, 245, 177, 74, 216]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionQualityDegradingEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SpeechRecognitionAudioProblem) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpeechRecognitionResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpeechRecognitionResult {
    type Vtable = ISpeechRecognitionResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1311781207, 846, 18002, [133, 126, 208, 69, 76, 196, 190, 236]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SpeechRecognitionResultStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SpeechRecognitionConfidence) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, maxalternates: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpeechRecognitionResult2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpeechRecognitionResult2 {
    type Vtable = ISpeechRecognitionResult2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2944324026, 17691, 16742, [160, 193, 31, 254, 132, 3, 45, 3]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionResult2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpeechRecognitionSemanticInterpretation(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpeechRecognitionSemanticInterpretation {
    type Vtable = ISpeechRecognitionSemanticInterpretation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2866928283, 32306, 19487, [137, 254, 12, 101, 244, 134, 245, 46]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionSemanticInterpretation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpeechRecognitionTopicConstraint(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpeechRecognitionTopicConstraint {
    type Vtable = ISpeechRecognitionTopicConstraint_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3211779865, 33373, 20073, [166, 129, 54, 228, 140, 241, 201, 62]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionTopicConstraint_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SpeechRecognitionScenario) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpeechRecognitionTopicConstraintFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpeechRecognitionTopicConstraintFactory {
    type Vtable = ISpeechRecognitionTopicConstraintFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1852335071, 60421, 18391, [165, 223, 86, 163, 67, 30, 88, 210]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionTopicConstraintFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scenario: SpeechRecognitionScenario, topichint: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scenario: SpeechRecognitionScenario, topichint: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, tag: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpeechRecognitionVoiceCommandDefinitionConstraint(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpeechRecognitionVoiceCommandDefinitionConstraint {
    type Vtable = ISpeechRecognitionVoiceCommandDefinitionConstraint_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4068023339, 7924, 19175, [157, 119, 182, 255, 16, 184, 163, 194]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionVoiceCommandDefinitionConstraint_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpeechRecognizer(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpeechRecognizer {
    type Vtable = ISpeechRecognizer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(197380555, 49770, 16626, [174, 181, 128, 150, 178, 228, 128, 115]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Globalization")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Globalization"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, speechrecognitionqualitydegradinghandler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, statechangedhandler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpeechRecognizer2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpeechRecognizer2 {
    type Vtable = ISpeechRecognizer2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1674164977, 37347, 20132, [134, 161, 124, 56, 103, 208, 132, 166]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizer2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SpeechRecognizerState) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpeechRecognizerFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpeechRecognizerFactory {
    type Vtable = ISpeechRecognizerFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1623492829, 32696, 16435, [172, 112, 208, 70, 246, 72, 24, 225]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizerFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Globalization")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, language: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Globalization"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpeechRecognizerStateChangedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpeechRecognizerStateChangedEventArgs {
    type Vtable = ISpeechRecognizerStateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1446858505, 47619, 19373, [173, 129, 221, 198, 196, 218, 176, 195]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizerStateChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SpeechRecognizerState) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpeechRecognizerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpeechRecognizerStatics {
    type Vtable = ISpeechRecognizerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2275630764, 42972, 19211, [188, 201, 36, 244, 124, 11, 126, 191]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Globalization")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Globalization"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Globalization")))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Globalization")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpeechRecognizerStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpeechRecognizerStatics2 {
    type Vtable = ISpeechRecognizerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(488312213, 30053, 20217, [162, 243, 186, 21, 22, 42, 150, 207]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Globalization"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, speechlanguage: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Globalization")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpeechRecognizerTimeouts(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpeechRecognizerTimeouts {
    type Vtable = ISpeechRecognizerTimeouts_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(787967946, 27196, 19914, [161, 83, 223, 27, 200, 138, 121, 175]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizerTimeouts_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpeechRecognizerUIOptions(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpeechRecognizerUIOptions {
    type Vtable = ISpeechRecognizerUIOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2022233665, 47403, 17594, [162, 95, 209, 134, 70, 48, 100, 31]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizerUIOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVoiceCommandManager(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVoiceCommandManager {
    type Vtable = IVoiceCommandManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2855964117, 46823, 20194, [186, 169, 221, 107, 172, 237, 10, 43]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, file: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVoiceCommandSet(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVoiceCommandSet {
    type Vtable = IVoiceCommandSet_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(200137333, 18150, 19217, [160, 136, 92, 104, 99, 40, 153, 181]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandSet_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phraselistname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, phraselist: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[doc = "*Required features: `Media_SpeechRecognition`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpeechContinuousRecognitionCompletedEventArgs(pub ::windows::runtime::IInspectable);
impl SpeechContinuousRecognitionCompletedEventArgs {
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<SpeechRecognitionResultStatus> {
        let this = self;
        unsafe {
            let mut result__: SpeechRecognitionResultStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionResultStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpeechContinuousRecognitionCompletedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechContinuousRecognitionCompletedEventArgs;{e3d069bb-e30c-5e18-424b-7fbe81f8fbd0})");
}
unsafe impl ::windows::runtime::Interface for SpeechContinuousRecognitionCompletedEventArgs {
    type Vtable = ISpeechContinuousRecognitionCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3822086587, 58124, 24088, [66, 75, 127, 190, 129, 248, 251, 208]);
}
impl ::windows::runtime::RuntimeName for SpeechContinuousRecognitionCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechContinuousRecognitionCompletedEventArgs";
}
impl ::std::convert::From<SpeechContinuousRecognitionCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpeechContinuousRecognitionCompletedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SpeechContinuousRecognitionCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpeechContinuousRecognitionCompletedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpeechContinuousRecognitionCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpeechContinuousRecognitionCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SpeechContinuousRecognitionCompletedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpeechContinuousRecognitionCompletedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpeechContinuousRecognitionCompletedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SpeechContinuousRecognitionCompletedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpeechContinuousRecognitionCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpeechContinuousRecognitionCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SpeechContinuousRecognitionCompletedEventArgs {}
unsafe impl ::std::marker::Sync for SpeechContinuousRecognitionCompletedEventArgs {}
#[doc = "*Required features: `Media_SpeechRecognition`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SpeechContinuousRecognitionMode(pub i32);
impl SpeechContinuousRecognitionMode {
    pub const Default: SpeechContinuousRecognitionMode = SpeechContinuousRecognitionMode(0i32);
    pub const PauseOnRecognition: SpeechContinuousRecognitionMode = SpeechContinuousRecognitionMode(1i32);
}
impl ::std::convert::From<i32> for SpeechContinuousRecognitionMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SpeechContinuousRecognitionMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SpeechContinuousRecognitionMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechContinuousRecognitionMode;i4)");
}
impl ::windows::runtime::DefaultType for SpeechContinuousRecognitionMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_SpeechRecognition`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpeechContinuousRecognitionResultGeneratedEventArgs(pub ::windows::runtime::IInspectable);
impl SpeechContinuousRecognitionResultGeneratedEventArgs {
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn Result(&self) -> ::windows::runtime::Result<SpeechRecognitionResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionResult>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpeechContinuousRecognitionResultGeneratedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechContinuousRecognitionResultGeneratedEventArgs;{19091e1e-6e7e-5a46-40fb-76594f786504})");
}
unsafe impl ::windows::runtime::Interface for SpeechContinuousRecognitionResultGeneratedEventArgs {
    type Vtable = ISpeechContinuousRecognitionResultGeneratedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(420027934, 28286, 23110, [64, 251, 118, 89, 79, 120, 101, 4]);
}
impl ::windows::runtime::RuntimeName for SpeechContinuousRecognitionResultGeneratedEventArgs {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechContinuousRecognitionResultGeneratedEventArgs";
}
impl ::std::convert::From<SpeechContinuousRecognitionResultGeneratedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpeechContinuousRecognitionResultGeneratedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SpeechContinuousRecognitionResultGeneratedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpeechContinuousRecognitionResultGeneratedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpeechContinuousRecognitionResultGeneratedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpeechContinuousRecognitionResultGeneratedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SpeechContinuousRecognitionResultGeneratedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpeechContinuousRecognitionResultGeneratedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpeechContinuousRecognitionResultGeneratedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SpeechContinuousRecognitionResultGeneratedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpeechContinuousRecognitionResultGeneratedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpeechContinuousRecognitionResultGeneratedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SpeechContinuousRecognitionResultGeneratedEventArgs {}
unsafe impl ::std::marker::Sync for SpeechContinuousRecognitionResultGeneratedEventArgs {}
#[doc = "*Required features: `Media_SpeechRecognition`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpeechContinuousRecognitionSession(pub ::windows::runtime::IInspectable);
impl SpeechContinuousRecognitionSession {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation`*"]
    pub fn AutoStopSilenceTimeout(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation`*"]
    pub fn SetAutoStopSilenceTimeout<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation`*"]
    pub fn StartAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation`*"]
    pub fn StartWithModeAsync(&self, mode: SpeechContinuousRecognitionMode) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), mode, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation`*"]
    pub fn StopAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation`*"]
    pub fn CancelAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation`*"]
    pub fn PauseAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn Resume(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation`*"]
    pub fn Completed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<SpeechContinuousRecognitionSession, SpeechContinuousRecognitionCompletedEventArgs>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation`*"]
    pub fn RemoveCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation`*"]
    pub fn ResultGenerated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<SpeechContinuousRecognitionSession, SpeechContinuousRecognitionResultGeneratedEventArgs>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation`*"]
    pub fn RemoveResultGenerated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpeechContinuousRecognitionSession {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechContinuousRecognitionSession;{6a213c04-6614-49f8-99a2-b5e9b3a085c8})");
}
unsafe impl ::windows::runtime::Interface for SpeechContinuousRecognitionSession {
    type Vtable = ISpeechContinuousRecognitionSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1780562948, 26132, 18936, [153, 162, 181, 233, 179, 160, 133, 200]);
}
impl ::windows::runtime::RuntimeName for SpeechContinuousRecognitionSession {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechContinuousRecognitionSession";
}
impl ::std::convert::From<SpeechContinuousRecognitionSession> for ::windows::runtime::IUnknown {
    fn from(value: SpeechContinuousRecognitionSession) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SpeechContinuousRecognitionSession> for ::windows::runtime::IUnknown {
    fn from(value: &SpeechContinuousRecognitionSession) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpeechContinuousRecognitionSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpeechContinuousRecognitionSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SpeechContinuousRecognitionSession> for ::windows::runtime::IInspectable {
    fn from(value: SpeechContinuousRecognitionSession) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpeechContinuousRecognitionSession> for ::windows::runtime::IInspectable {
    fn from(value: &SpeechContinuousRecognitionSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpeechContinuousRecognitionSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpeechContinuousRecognitionSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SpeechContinuousRecognitionSession {}
unsafe impl ::std::marker::Sync for SpeechContinuousRecognitionSession {}
#[doc = "*Required features: `Media_SpeechRecognition`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SpeechRecognitionAudioProblem(pub i32);
impl SpeechRecognitionAudioProblem {
    pub const None: SpeechRecognitionAudioProblem = SpeechRecognitionAudioProblem(0i32);
    pub const TooNoisy: SpeechRecognitionAudioProblem = SpeechRecognitionAudioProblem(1i32);
    pub const NoSignal: SpeechRecognitionAudioProblem = SpeechRecognitionAudioProblem(2i32);
    pub const TooLoud: SpeechRecognitionAudioProblem = SpeechRecognitionAudioProblem(3i32);
    pub const TooQuiet: SpeechRecognitionAudioProblem = SpeechRecognitionAudioProblem(4i32);
    pub const TooFast: SpeechRecognitionAudioProblem = SpeechRecognitionAudioProblem(5i32);
    pub const TooSlow: SpeechRecognitionAudioProblem = SpeechRecognitionAudioProblem(6i32);
}
impl ::std::convert::From<i32> for SpeechRecognitionAudioProblem {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SpeechRecognitionAudioProblem {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SpeechRecognitionAudioProblem {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechRecognitionAudioProblem;i4)");
}
impl ::windows::runtime::DefaultType for SpeechRecognitionAudioProblem {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_SpeechRecognition`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpeechRecognitionCompilationResult(pub ::windows::runtime::IInspectable);
impl SpeechRecognitionCompilationResult {
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<SpeechRecognitionResultStatus> {
        let this = self;
        unsafe {
            let mut result__: SpeechRecognitionResultStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionResultStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpeechRecognitionCompilationResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionCompilationResult;{407e6c5d-6ac7-4da4-9cc1-2fce32cf7489})");
}
unsafe impl ::windows::runtime::Interface for SpeechRecognitionCompilationResult {
    type Vtable = ISpeechRecognitionCompilationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1082027101, 27335, 19876, [156, 193, 47, 206, 50, 207, 116, 137]);
}
impl ::windows::runtime::RuntimeName for SpeechRecognitionCompilationResult {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionCompilationResult";
}
impl ::std::convert::From<SpeechRecognitionCompilationResult> for ::windows::runtime::IUnknown {
    fn from(value: SpeechRecognitionCompilationResult) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SpeechRecognitionCompilationResult> for ::windows::runtime::IUnknown {
    fn from(value: &SpeechRecognitionCompilationResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpeechRecognitionCompilationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpeechRecognitionCompilationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SpeechRecognitionCompilationResult> for ::windows::runtime::IInspectable {
    fn from(value: SpeechRecognitionCompilationResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpeechRecognitionCompilationResult> for ::windows::runtime::IInspectable {
    fn from(value: &SpeechRecognitionCompilationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpeechRecognitionCompilationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpeechRecognitionCompilationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SpeechRecognitionCompilationResult {}
unsafe impl ::std::marker::Sync for SpeechRecognitionCompilationResult {}
#[doc = "*Required features: `Media_SpeechRecognition`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SpeechRecognitionConfidence(pub i32);
impl SpeechRecognitionConfidence {
    pub const High: SpeechRecognitionConfidence = SpeechRecognitionConfidence(0i32);
    pub const Medium: SpeechRecognitionConfidence = SpeechRecognitionConfidence(1i32);
    pub const Low: SpeechRecognitionConfidence = SpeechRecognitionConfidence(2i32);
    pub const Rejected: SpeechRecognitionConfidence = SpeechRecognitionConfidence(3i32);
}
impl ::std::convert::From<i32> for SpeechRecognitionConfidence {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SpeechRecognitionConfidence {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SpeechRecognitionConfidence {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechRecognitionConfidence;i4)");
}
impl ::windows::runtime::DefaultType for SpeechRecognitionConfidence {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_SpeechRecognition`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SpeechRecognitionConstraintProbability(pub i32);
impl SpeechRecognitionConstraintProbability {
    pub const Default: SpeechRecognitionConstraintProbability = SpeechRecognitionConstraintProbability(0i32);
    pub const Min: SpeechRecognitionConstraintProbability = SpeechRecognitionConstraintProbability(1i32);
    pub const Max: SpeechRecognitionConstraintProbability = SpeechRecognitionConstraintProbability(2i32);
}
impl ::std::convert::From<i32> for SpeechRecognitionConstraintProbability {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SpeechRecognitionConstraintProbability {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SpeechRecognitionConstraintProbability {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechRecognitionConstraintProbability;i4)");
}
impl ::windows::runtime::DefaultType for SpeechRecognitionConstraintProbability {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_SpeechRecognition`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SpeechRecognitionConstraintType(pub i32);
impl SpeechRecognitionConstraintType {
    pub const Topic: SpeechRecognitionConstraintType = SpeechRecognitionConstraintType(0i32);
    pub const List: SpeechRecognitionConstraintType = SpeechRecognitionConstraintType(1i32);
    pub const Grammar: SpeechRecognitionConstraintType = SpeechRecognitionConstraintType(2i32);
    pub const VoiceCommandDefinition: SpeechRecognitionConstraintType = SpeechRecognitionConstraintType(3i32);
}
impl ::std::convert::From<i32> for SpeechRecognitionConstraintType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SpeechRecognitionConstraintType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SpeechRecognitionConstraintType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechRecognitionConstraintType;i4)");
}
impl ::windows::runtime::DefaultType for SpeechRecognitionConstraintType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_SpeechRecognition`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpeechRecognitionGrammarFileConstraint(pub ::windows::runtime::IInspectable);
impl SpeechRecognitionGrammarFileConstraint {
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Storage`*"]
    pub fn GrammarFile(&self) -> ::windows::runtime::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn IsEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn Tag(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn SetTag<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<SpeechRecognitionConstraintType> {
        let this = &::windows::runtime::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__: SpeechRecognitionConstraintType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionConstraintType>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn Probability(&self) -> ::windows::runtime::Result<SpeechRecognitionConstraintProbability> {
        let this = &::windows::runtime::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__: SpeechRecognitionConstraintProbability = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionConstraintProbability>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn SetProbability(&self, value: SpeechRecognitionConstraintProbability) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Storage`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::StorageFile>>(file: Param0) -> ::windows::runtime::Result<SpeechRecognitionGrammarFileConstraint> {
        Self::ISpeechRecognitionGrammarFileConstraintFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<SpeechRecognitionGrammarFileConstraint>(result__)
        })
    }
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Storage`*"]
    pub fn CreateWithTag<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::StorageFile>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(file: Param0, tag: Param1) -> ::windows::runtime::Result<SpeechRecognitionGrammarFileConstraint> {
        Self::ISpeechRecognitionGrammarFileConstraintFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), file.into_param().abi(), tag.into_param().abi(), &mut result__).from_abi::<SpeechRecognitionGrammarFileConstraint>(result__)
        })
    }
    pub fn ISpeechRecognitionGrammarFileConstraintFactory<R, F: FnOnce(&ISpeechRecognitionGrammarFileConstraintFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SpeechRecognitionGrammarFileConstraint, ISpeechRecognitionGrammarFileConstraintFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpeechRecognitionGrammarFileConstraint {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionGrammarFileConstraint;{b5031a8f-85ca-4fa4-b11a-474fc41b3835})");
}
unsafe impl ::windows::runtime::Interface for SpeechRecognitionGrammarFileConstraint {
    type Vtable = ISpeechRecognitionGrammarFileConstraint_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3036879503, 34250, 20388, [177, 26, 71, 79, 196, 27, 56, 53]);
}
impl ::windows::runtime::RuntimeName for SpeechRecognitionGrammarFileConstraint {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionGrammarFileConstraint";
}
impl ::std::convert::From<SpeechRecognitionGrammarFileConstraint> for ::windows::runtime::IUnknown {
    fn from(value: SpeechRecognitionGrammarFileConstraint) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SpeechRecognitionGrammarFileConstraint> for ::windows::runtime::IUnknown {
    fn from(value: &SpeechRecognitionGrammarFileConstraint) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpeechRecognitionGrammarFileConstraint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpeechRecognitionGrammarFileConstraint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SpeechRecognitionGrammarFileConstraint> for ::windows::runtime::IInspectable {
    fn from(value: SpeechRecognitionGrammarFileConstraint) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpeechRecognitionGrammarFileConstraint> for ::windows::runtime::IInspectable {
    fn from(value: &SpeechRecognitionGrammarFileConstraint) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpeechRecognitionGrammarFileConstraint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpeechRecognitionGrammarFileConstraint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SpeechRecognitionGrammarFileConstraint> for ISpeechRecognitionConstraint {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SpeechRecognitionGrammarFileConstraint) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SpeechRecognitionGrammarFileConstraint> for ISpeechRecognitionConstraint {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SpeechRecognitionGrammarFileConstraint) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISpeechRecognitionConstraint> for SpeechRecognitionGrammarFileConstraint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISpeechRecognitionConstraint> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISpeechRecognitionConstraint> for &SpeechRecognitionGrammarFileConstraint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISpeechRecognitionConstraint> {
        ::std::convert::TryInto::<ISpeechRecognitionConstraint>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for SpeechRecognitionGrammarFileConstraint {}
unsafe impl ::std::marker::Sync for SpeechRecognitionGrammarFileConstraint {}
#[doc = "*Required features: `Media_SpeechRecognition`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpeechRecognitionHypothesis(pub ::windows::runtime::IInspectable);
impl SpeechRecognitionHypothesis {
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn Text(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpeechRecognitionHypothesis {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionHypothesis;{7a7b25b0-99c5-4f7d-bf84-10aa1302b634})");
}
unsafe impl ::windows::runtime::Interface for SpeechRecognitionHypothesis {
    type Vtable = ISpeechRecognitionHypothesis_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2054890928, 39365, 20349, [191, 132, 16, 170, 19, 2, 182, 52]);
}
impl ::windows::runtime::RuntimeName for SpeechRecognitionHypothesis {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionHypothesis";
}
impl ::std::convert::From<SpeechRecognitionHypothesis> for ::windows::runtime::IUnknown {
    fn from(value: SpeechRecognitionHypothesis) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SpeechRecognitionHypothesis> for ::windows::runtime::IUnknown {
    fn from(value: &SpeechRecognitionHypothesis) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpeechRecognitionHypothesis {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpeechRecognitionHypothesis {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SpeechRecognitionHypothesis> for ::windows::runtime::IInspectable {
    fn from(value: SpeechRecognitionHypothesis) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpeechRecognitionHypothesis> for ::windows::runtime::IInspectable {
    fn from(value: &SpeechRecognitionHypothesis) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpeechRecognitionHypothesis {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpeechRecognitionHypothesis {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SpeechRecognitionHypothesis {}
unsafe impl ::std::marker::Sync for SpeechRecognitionHypothesis {}
#[doc = "*Required features: `Media_SpeechRecognition`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpeechRecognitionHypothesisGeneratedEventArgs(pub ::windows::runtime::IInspectable);
impl SpeechRecognitionHypothesisGeneratedEventArgs {
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn Hypothesis(&self) -> ::windows::runtime::Result<SpeechRecognitionHypothesis> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionHypothesis>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpeechRecognitionHypothesisGeneratedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionHypothesisGeneratedEventArgs;{55161a7a-8023-5866-411d-1213bb271476})");
}
unsafe impl ::windows::runtime::Interface for SpeechRecognitionHypothesisGeneratedEventArgs {
    type Vtable = ISpeechRecognitionHypothesisGeneratedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1427511930, 32803, 22630, [65, 29, 18, 19, 187, 39, 20, 118]);
}
impl ::windows::runtime::RuntimeName for SpeechRecognitionHypothesisGeneratedEventArgs {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionHypothesisGeneratedEventArgs";
}
impl ::std::convert::From<SpeechRecognitionHypothesisGeneratedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpeechRecognitionHypothesisGeneratedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SpeechRecognitionHypothesisGeneratedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpeechRecognitionHypothesisGeneratedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpeechRecognitionHypothesisGeneratedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpeechRecognitionHypothesisGeneratedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SpeechRecognitionHypothesisGeneratedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpeechRecognitionHypothesisGeneratedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpeechRecognitionHypothesisGeneratedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SpeechRecognitionHypothesisGeneratedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpeechRecognitionHypothesisGeneratedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpeechRecognitionHypothesisGeneratedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SpeechRecognitionHypothesisGeneratedEventArgs {}
unsafe impl ::std::marker::Sync for SpeechRecognitionHypothesisGeneratedEventArgs {}
#[doc = "*Required features: `Media_SpeechRecognition`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpeechRecognitionListConstraint(pub ::windows::runtime::IInspectable);
impl SpeechRecognitionListConstraint {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation_Collections`*"]
    pub fn Commands(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn IsEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn Tag(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn SetTag<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<SpeechRecognitionConstraintType> {
        let this = &::windows::runtime::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__: SpeechRecognitionConstraintType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionConstraintType>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn Probability(&self) -> ::windows::runtime::Result<SpeechRecognitionConstraintProbability> {
        let this = &::windows::runtime::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__: SpeechRecognitionConstraintProbability = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionConstraintProbability>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn SetProbability(&self, value: SpeechRecognitionConstraintProbability) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation_Collections`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(commands: Param0) -> ::windows::runtime::Result<SpeechRecognitionListConstraint> {
        Self::ISpeechRecognitionListConstraintFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), commands.into_param().abi(), &mut result__).from_abi::<SpeechRecognitionListConstraint>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation_Collections`*"]
    pub fn CreateWithTag<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(commands: Param0, tag: Param1) -> ::windows::runtime::Result<SpeechRecognitionListConstraint> {
        Self::ISpeechRecognitionListConstraintFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), commands.into_param().abi(), tag.into_param().abi(), &mut result__).from_abi::<SpeechRecognitionListConstraint>(result__)
        })
    }
    pub fn ISpeechRecognitionListConstraintFactory<R, F: FnOnce(&ISpeechRecognitionListConstraintFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SpeechRecognitionListConstraint, ISpeechRecognitionListConstraintFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpeechRecognitionListConstraint {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionListConstraint;{09c487e9-e4ad-4526-81f2-4946fb481d98})");
}
unsafe impl ::windows::runtime::Interface for SpeechRecognitionListConstraint {
    type Vtable = ISpeechRecognitionListConstraint_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(163874793, 58541, 17702, [129, 242, 73, 70, 251, 72, 29, 152]);
}
impl ::windows::runtime::RuntimeName for SpeechRecognitionListConstraint {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionListConstraint";
}
impl ::std::convert::From<SpeechRecognitionListConstraint> for ::windows::runtime::IUnknown {
    fn from(value: SpeechRecognitionListConstraint) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SpeechRecognitionListConstraint> for ::windows::runtime::IUnknown {
    fn from(value: &SpeechRecognitionListConstraint) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpeechRecognitionListConstraint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpeechRecognitionListConstraint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SpeechRecognitionListConstraint> for ::windows::runtime::IInspectable {
    fn from(value: SpeechRecognitionListConstraint) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpeechRecognitionListConstraint> for ::windows::runtime::IInspectable {
    fn from(value: &SpeechRecognitionListConstraint) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpeechRecognitionListConstraint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpeechRecognitionListConstraint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SpeechRecognitionListConstraint> for ISpeechRecognitionConstraint {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SpeechRecognitionListConstraint) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SpeechRecognitionListConstraint> for ISpeechRecognitionConstraint {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SpeechRecognitionListConstraint) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISpeechRecognitionConstraint> for SpeechRecognitionListConstraint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISpeechRecognitionConstraint> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISpeechRecognitionConstraint> for &SpeechRecognitionListConstraint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISpeechRecognitionConstraint> {
        ::std::convert::TryInto::<ISpeechRecognitionConstraint>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for SpeechRecognitionListConstraint {}
unsafe impl ::std::marker::Sync for SpeechRecognitionListConstraint {}
#[doc = "*Required features: `Media_SpeechRecognition`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpeechRecognitionQualityDegradingEventArgs(pub ::windows::runtime::IInspectable);
impl SpeechRecognitionQualityDegradingEventArgs {
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn Problem(&self) -> ::windows::runtime::Result<SpeechRecognitionAudioProblem> {
        let this = self;
        unsafe {
            let mut result__: SpeechRecognitionAudioProblem = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionAudioProblem>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpeechRecognitionQualityDegradingEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionQualityDegradingEventArgs;{4fe24105-8c3a-4c7e-8d0a-5bd4f5b14ad8})");
}
unsafe impl ::windows::runtime::Interface for SpeechRecognitionQualityDegradingEventArgs {
    type Vtable = ISpeechRecognitionQualityDegradingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1340227845, 35898, 19582, [141, 10, 91, 212, 245, 177, 74, 216]);
}
impl ::windows::runtime::RuntimeName for SpeechRecognitionQualityDegradingEventArgs {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionQualityDegradingEventArgs";
}
impl ::std::convert::From<SpeechRecognitionQualityDegradingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpeechRecognitionQualityDegradingEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SpeechRecognitionQualityDegradingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpeechRecognitionQualityDegradingEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpeechRecognitionQualityDegradingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpeechRecognitionQualityDegradingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SpeechRecognitionQualityDegradingEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpeechRecognitionQualityDegradingEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpeechRecognitionQualityDegradingEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SpeechRecognitionQualityDegradingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpeechRecognitionQualityDegradingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpeechRecognitionQualityDegradingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SpeechRecognitionQualityDegradingEventArgs {}
unsafe impl ::std::marker::Sync for SpeechRecognitionQualityDegradingEventArgs {}
#[doc = "*Required features: `Media_SpeechRecognition`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpeechRecognitionResult(pub ::windows::runtime::IInspectable);
impl SpeechRecognitionResult {
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<SpeechRecognitionResultStatus> {
        let this = self;
        unsafe {
            let mut result__: SpeechRecognitionResultStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionResultStatus>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn Text(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn Confidence(&self) -> ::windows::runtime::Result<SpeechRecognitionConfidence> {
        let this = self;
        unsafe {
            let mut result__: SpeechRecognitionConfidence = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionConfidence>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn SemanticInterpretation(&self) -> ::windows::runtime::Result<SpeechRecognitionSemanticInterpretation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionSemanticInterpretation>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation_Collections`*"]
    pub fn GetAlternates(&self, maxalternates: u32) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<SpeechRecognitionResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), maxalternates, &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<SpeechRecognitionResult>>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn Constraint(&self) -> ::windows::runtime::Result<ISpeechRecognitionConstraint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ISpeechRecognitionConstraint>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation_Collections`*"]
    pub fn RulePath(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn RawConfidence(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation`*"]
    pub fn PhraseStartTime(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = &::windows::runtime::Interface::cast::<ISpeechRecognitionResult2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation`*"]
    pub fn PhraseDuration(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::runtime::Interface::cast::<ISpeechRecognitionResult2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpeechRecognitionResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionResult;{4e303157-034e-4652-857e-d0454cc4beec})");
}
unsafe impl ::windows::runtime::Interface for SpeechRecognitionResult {
    type Vtable = ISpeechRecognitionResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1311781207, 846, 18002, [133, 126, 208, 69, 76, 196, 190, 236]);
}
impl ::windows::runtime::RuntimeName for SpeechRecognitionResult {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionResult";
}
impl ::std::convert::From<SpeechRecognitionResult> for ::windows::runtime::IUnknown {
    fn from(value: SpeechRecognitionResult) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SpeechRecognitionResult> for ::windows::runtime::IUnknown {
    fn from(value: &SpeechRecognitionResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpeechRecognitionResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpeechRecognitionResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SpeechRecognitionResult> for ::windows::runtime::IInspectable {
    fn from(value: SpeechRecognitionResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpeechRecognitionResult> for ::windows::runtime::IInspectable {
    fn from(value: &SpeechRecognitionResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpeechRecognitionResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpeechRecognitionResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SpeechRecognitionResult {}
unsafe impl ::std::marker::Sync for SpeechRecognitionResult {}
#[doc = "*Required features: `Media_SpeechRecognition`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SpeechRecognitionResultStatus(pub i32);
impl SpeechRecognitionResultStatus {
    pub const Success: SpeechRecognitionResultStatus = SpeechRecognitionResultStatus(0i32);
    pub const TopicLanguageNotSupported: SpeechRecognitionResultStatus = SpeechRecognitionResultStatus(1i32);
    pub const GrammarLanguageMismatch: SpeechRecognitionResultStatus = SpeechRecognitionResultStatus(2i32);
    pub const GrammarCompilationFailure: SpeechRecognitionResultStatus = SpeechRecognitionResultStatus(3i32);
    pub const AudioQualityFailure: SpeechRecognitionResultStatus = SpeechRecognitionResultStatus(4i32);
    pub const UserCanceled: SpeechRecognitionResultStatus = SpeechRecognitionResultStatus(5i32);
    pub const Unknown: SpeechRecognitionResultStatus = SpeechRecognitionResultStatus(6i32);
    pub const TimeoutExceeded: SpeechRecognitionResultStatus = SpeechRecognitionResultStatus(7i32);
    pub const PauseLimitExceeded: SpeechRecognitionResultStatus = SpeechRecognitionResultStatus(8i32);
    pub const NetworkFailure: SpeechRecognitionResultStatus = SpeechRecognitionResultStatus(9i32);
    pub const MicrophoneUnavailable: SpeechRecognitionResultStatus = SpeechRecognitionResultStatus(10i32);
}
impl ::std::convert::From<i32> for SpeechRecognitionResultStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SpeechRecognitionResultStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SpeechRecognitionResultStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechRecognitionResultStatus;i4)");
}
impl ::windows::runtime::DefaultType for SpeechRecognitionResultStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_SpeechRecognition`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SpeechRecognitionScenario(pub i32);
impl SpeechRecognitionScenario {
    pub const WebSearch: SpeechRecognitionScenario = SpeechRecognitionScenario(0i32);
    pub const Dictation: SpeechRecognitionScenario = SpeechRecognitionScenario(1i32);
    pub const FormFilling: SpeechRecognitionScenario = SpeechRecognitionScenario(2i32);
}
impl ::std::convert::From<i32> for SpeechRecognitionScenario {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SpeechRecognitionScenario {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SpeechRecognitionScenario {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechRecognitionScenario;i4)");
}
impl ::windows::runtime::DefaultType for SpeechRecognitionScenario {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_SpeechRecognition`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpeechRecognitionSemanticInterpretation(pub ::windows::runtime::IInspectable);
impl SpeechRecognitionSemanticInterpretation {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpeechRecognitionSemanticInterpretation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionSemanticInterpretation;{aae1da9b-7e32-4c1f-89fe-0c65f486f52e})");
}
unsafe impl ::windows::runtime::Interface for SpeechRecognitionSemanticInterpretation {
    type Vtable = ISpeechRecognitionSemanticInterpretation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2866928283, 32306, 19487, [137, 254, 12, 101, 244, 134, 245, 46]);
}
impl ::windows::runtime::RuntimeName for SpeechRecognitionSemanticInterpretation {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionSemanticInterpretation";
}
impl ::std::convert::From<SpeechRecognitionSemanticInterpretation> for ::windows::runtime::IUnknown {
    fn from(value: SpeechRecognitionSemanticInterpretation) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SpeechRecognitionSemanticInterpretation> for ::windows::runtime::IUnknown {
    fn from(value: &SpeechRecognitionSemanticInterpretation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpeechRecognitionSemanticInterpretation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpeechRecognitionSemanticInterpretation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SpeechRecognitionSemanticInterpretation> for ::windows::runtime::IInspectable {
    fn from(value: SpeechRecognitionSemanticInterpretation) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpeechRecognitionSemanticInterpretation> for ::windows::runtime::IInspectable {
    fn from(value: &SpeechRecognitionSemanticInterpretation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpeechRecognitionSemanticInterpretation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpeechRecognitionSemanticInterpretation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SpeechRecognitionSemanticInterpretation {}
unsafe impl ::std::marker::Sync for SpeechRecognitionSemanticInterpretation {}
#[doc = "*Required features: `Media_SpeechRecognition`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpeechRecognitionTopicConstraint(pub ::windows::runtime::IInspectable);
impl SpeechRecognitionTopicConstraint {
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn Scenario(&self) -> ::windows::runtime::Result<SpeechRecognitionScenario> {
        let this = self;
        unsafe {
            let mut result__: SpeechRecognitionScenario = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionScenario>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn TopicHint(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn IsEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn Tag(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn SetTag<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<SpeechRecognitionConstraintType> {
        let this = &::windows::runtime::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__: SpeechRecognitionConstraintType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionConstraintType>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn Probability(&self) -> ::windows::runtime::Result<SpeechRecognitionConstraintProbability> {
        let this = &::windows::runtime::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__: SpeechRecognitionConstraintProbability = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionConstraintProbability>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn SetProbability(&self, value: SpeechRecognitionConstraintProbability) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn Create<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(scenario: SpeechRecognitionScenario, topichint: Param1) -> ::windows::runtime::Result<SpeechRecognitionTopicConstraint> {
        Self::ISpeechRecognitionTopicConstraintFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), scenario, topichint.into_param().abi(), &mut result__).from_abi::<SpeechRecognitionTopicConstraint>(result__)
        })
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn CreateWithTag<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(scenario: SpeechRecognitionScenario, topichint: Param1, tag: Param2) -> ::windows::runtime::Result<SpeechRecognitionTopicConstraint> {
        Self::ISpeechRecognitionTopicConstraintFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), scenario, topichint.into_param().abi(), tag.into_param().abi(), &mut result__).from_abi::<SpeechRecognitionTopicConstraint>(result__)
        })
    }
    pub fn ISpeechRecognitionTopicConstraintFactory<R, F: FnOnce(&ISpeechRecognitionTopicConstraintFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SpeechRecognitionTopicConstraint, ISpeechRecognitionTopicConstraintFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpeechRecognitionTopicConstraint {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionTopicConstraint;{bf6fdf19-825d-4e69-a681-36e48cf1c93e})");
}
unsafe impl ::windows::runtime::Interface for SpeechRecognitionTopicConstraint {
    type Vtable = ISpeechRecognitionTopicConstraint_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3211779865, 33373, 20073, [166, 129, 54, 228, 140, 241, 201, 62]);
}
impl ::windows::runtime::RuntimeName for SpeechRecognitionTopicConstraint {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionTopicConstraint";
}
impl ::std::convert::From<SpeechRecognitionTopicConstraint> for ::windows::runtime::IUnknown {
    fn from(value: SpeechRecognitionTopicConstraint) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SpeechRecognitionTopicConstraint> for ::windows::runtime::IUnknown {
    fn from(value: &SpeechRecognitionTopicConstraint) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpeechRecognitionTopicConstraint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpeechRecognitionTopicConstraint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SpeechRecognitionTopicConstraint> for ::windows::runtime::IInspectable {
    fn from(value: SpeechRecognitionTopicConstraint) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpeechRecognitionTopicConstraint> for ::windows::runtime::IInspectable {
    fn from(value: &SpeechRecognitionTopicConstraint) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpeechRecognitionTopicConstraint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpeechRecognitionTopicConstraint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SpeechRecognitionTopicConstraint> for ISpeechRecognitionConstraint {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SpeechRecognitionTopicConstraint) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SpeechRecognitionTopicConstraint> for ISpeechRecognitionConstraint {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SpeechRecognitionTopicConstraint) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISpeechRecognitionConstraint> for SpeechRecognitionTopicConstraint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISpeechRecognitionConstraint> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISpeechRecognitionConstraint> for &SpeechRecognitionTopicConstraint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISpeechRecognitionConstraint> {
        ::std::convert::TryInto::<ISpeechRecognitionConstraint>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for SpeechRecognitionTopicConstraint {}
unsafe impl ::std::marker::Sync for SpeechRecognitionTopicConstraint {}
#[doc = "*Required features: `Media_SpeechRecognition`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpeechRecognitionVoiceCommandDefinitionConstraint(pub ::windows::runtime::IInspectable);
impl SpeechRecognitionVoiceCommandDefinitionConstraint {
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn IsEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn Tag(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn SetTag<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<SpeechRecognitionConstraintType> {
        let this = &::windows::runtime::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__: SpeechRecognitionConstraintType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionConstraintType>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn Probability(&self) -> ::windows::runtime::Result<SpeechRecognitionConstraintProbability> {
        let this = &::windows::runtime::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__: SpeechRecognitionConstraintProbability = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionConstraintProbability>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn SetProbability(&self, value: SpeechRecognitionConstraintProbability) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpeechRecognitionVoiceCommandDefinitionConstraint {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionVoiceCommandDefinitionConstraint;{f2791c2b-1ef4-4ae7-9d77-b6ff10b8a3c2})");
}
unsafe impl ::windows::runtime::Interface for SpeechRecognitionVoiceCommandDefinitionConstraint {
    type Vtable = ISpeechRecognitionVoiceCommandDefinitionConstraint_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4068023339, 7924, 19175, [157, 119, 182, 255, 16, 184, 163, 194]);
}
impl ::windows::runtime::RuntimeName for SpeechRecognitionVoiceCommandDefinitionConstraint {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionVoiceCommandDefinitionConstraint";
}
impl ::std::convert::From<SpeechRecognitionVoiceCommandDefinitionConstraint> for ::windows::runtime::IUnknown {
    fn from(value: SpeechRecognitionVoiceCommandDefinitionConstraint) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SpeechRecognitionVoiceCommandDefinitionConstraint> for ::windows::runtime::IUnknown {
    fn from(value: &SpeechRecognitionVoiceCommandDefinitionConstraint) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpeechRecognitionVoiceCommandDefinitionConstraint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpeechRecognitionVoiceCommandDefinitionConstraint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SpeechRecognitionVoiceCommandDefinitionConstraint> for ::windows::runtime::IInspectable {
    fn from(value: SpeechRecognitionVoiceCommandDefinitionConstraint) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpeechRecognitionVoiceCommandDefinitionConstraint> for ::windows::runtime::IInspectable {
    fn from(value: &SpeechRecognitionVoiceCommandDefinitionConstraint) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpeechRecognitionVoiceCommandDefinitionConstraint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpeechRecognitionVoiceCommandDefinitionConstraint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SpeechRecognitionVoiceCommandDefinitionConstraint> for ISpeechRecognitionConstraint {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SpeechRecognitionVoiceCommandDefinitionConstraint) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SpeechRecognitionVoiceCommandDefinitionConstraint> for ISpeechRecognitionConstraint {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SpeechRecognitionVoiceCommandDefinitionConstraint) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISpeechRecognitionConstraint> for SpeechRecognitionVoiceCommandDefinitionConstraint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISpeechRecognitionConstraint> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISpeechRecognitionConstraint> for &SpeechRecognitionVoiceCommandDefinitionConstraint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISpeechRecognitionConstraint> {
        ::std::convert::TryInto::<ISpeechRecognitionConstraint>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for SpeechRecognitionVoiceCommandDefinitionConstraint {}
unsafe impl ::std::marker::Sync for SpeechRecognitionVoiceCommandDefinitionConstraint {}
#[doc = "*Required features: `Media_SpeechRecognition`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpeechRecognizer(pub ::windows::runtime::IInspectable);
impl SpeechRecognizer {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SpeechRecognizer, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Globalization")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Globalization`*"]
    pub fn CurrentLanguage(&self) -> ::windows::runtime::Result<super::super::Globalization::Language> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Globalization::Language>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation_Collections`*"]
    pub fn Constraints(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<ISpeechRecognitionConstraint>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<ISpeechRecognitionConstraint>>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn Timeouts(&self) -> ::windows::runtime::Result<SpeechRecognizerTimeouts> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognizerTimeouts>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn UIOptions(&self) -> ::windows::runtime::Result<SpeechRecognizerUIOptions> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognizerUIOptions>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation`*"]
    pub fn CompileConstraintsAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SpeechRecognitionCompilationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SpeechRecognitionCompilationResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation`*"]
    pub fn RecognizeAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SpeechRecognitionResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SpeechRecognitionResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation`*"]
    pub fn RecognizeWithUIAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SpeechRecognitionResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SpeechRecognitionResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation`*"]
    pub fn RecognitionQualityDegrading<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<SpeechRecognizer, SpeechRecognitionQualityDegradingEventArgs>>>(&self, speechrecognitionqualitydegradinghandler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), speechrecognitionqualitydegradinghandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation`*"]
    pub fn RemoveRecognitionQualityDegrading<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation`*"]
    pub fn StateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<SpeechRecognizer, SpeechRecognizerStateChangedEventArgs>>>(&self, statechangedhandler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), statechangedhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation`*"]
    pub fn RemoveStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn ContinuousRecognitionSession(&self) -> ::windows::runtime::Result<SpeechContinuousRecognitionSession> {
        let this = &::windows::runtime::Interface::cast::<ISpeechRecognizer2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpeechContinuousRecognitionSession>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn State(&self) -> ::windows::runtime::Result<SpeechRecognizerState> {
        let this = &::windows::runtime::Interface::cast::<ISpeechRecognizer2>(self)?;
        unsafe {
            let mut result__: SpeechRecognizerState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognizerState>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation`*"]
    pub fn StopRecognitionAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<ISpeechRecognizer2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation`*"]
    pub fn HypothesisGenerated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<SpeechRecognizer, SpeechRecognitionHypothesisGeneratedEventArgs>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ISpeechRecognizer2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation`*"]
    pub fn RemoveHypothesisGenerated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISpeechRecognizer2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Globalization")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Globalization`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Globalization::Language>>(language: Param0) -> ::windows::runtime::Result<SpeechRecognizer> {
        Self::ISpeechRecognizerFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), language.into_param().abi(), &mut result__).from_abi::<SpeechRecognizer>(result__)
        })
    }
    #[cfg(feature = "Globalization")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Globalization`*"]
    pub fn SystemSpeechLanguage() -> ::windows::runtime::Result<super::super::Globalization::Language> {
        Self::ISpeechRecognizerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Globalization::Language>(result__)
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation_Collections`, `Globalization`*"]
    pub fn SupportedTopicLanguages() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<super::super::Globalization::Language>> {
        Self::ISpeechRecognizerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Globalization::Language>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation_Collections`, `Globalization`*"]
    pub fn SupportedGrammarLanguages() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<super::super::Globalization::Language>> {
        Self::ISpeechRecognizerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Globalization::Language>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Globalization"))]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation`, `Globalization`*"]
    pub fn TrySetSystemSpeechLanguageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Globalization::Language>>(speechlanguage: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::ISpeechRecognizerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), speechlanguage.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    pub fn ISpeechRecognizerFactory<R, F: FnOnce(&ISpeechRecognizerFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SpeechRecognizer, ISpeechRecognizerFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISpeechRecognizerStatics<R, F: FnOnce(&ISpeechRecognizerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SpeechRecognizer, ISpeechRecognizerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISpeechRecognizerStatics2<R, F: FnOnce(&ISpeechRecognizerStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SpeechRecognizer, ISpeechRecognizerStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpeechRecognizer {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognizer;{0bc3c9cb-c26a-40f2-aeb5-8096b2e48073})");
}
unsafe impl ::windows::runtime::Interface for SpeechRecognizer {
    type Vtable = ISpeechRecognizer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(197380555, 49770, 16626, [174, 181, 128, 150, 178, 228, 128, 115]);
}
impl ::windows::runtime::RuntimeName for SpeechRecognizer {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognizer";
}
impl ::std::convert::From<SpeechRecognizer> for ::windows::runtime::IUnknown {
    fn from(value: SpeechRecognizer) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SpeechRecognizer> for ::windows::runtime::IUnknown {
    fn from(value: &SpeechRecognizer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpeechRecognizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpeechRecognizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SpeechRecognizer> for ::windows::runtime::IInspectable {
    fn from(value: SpeechRecognizer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpeechRecognizer> for ::windows::runtime::IInspectable {
    fn from(value: &SpeechRecognizer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpeechRecognizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpeechRecognizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<SpeechRecognizer> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SpeechRecognizer) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&SpeechRecognizer> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SpeechRecognizer) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for SpeechRecognizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &SpeechRecognizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for SpeechRecognizer {}
unsafe impl ::std::marker::Sync for SpeechRecognizer {}
#[doc = "*Required features: `Media_SpeechRecognition`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SpeechRecognizerState(pub i32);
impl SpeechRecognizerState {
    pub const Idle: SpeechRecognizerState = SpeechRecognizerState(0i32);
    pub const Capturing: SpeechRecognizerState = SpeechRecognizerState(1i32);
    pub const Processing: SpeechRecognizerState = SpeechRecognizerState(2i32);
    pub const SoundStarted: SpeechRecognizerState = SpeechRecognizerState(3i32);
    pub const SoundEnded: SpeechRecognizerState = SpeechRecognizerState(4i32);
    pub const SpeechDetected: SpeechRecognizerState = SpeechRecognizerState(5i32);
    pub const Paused: SpeechRecognizerState = SpeechRecognizerState(6i32);
}
impl ::std::convert::From<i32> for SpeechRecognizerState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SpeechRecognizerState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SpeechRecognizerState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechRecognizerState;i4)");
}
impl ::windows::runtime::DefaultType for SpeechRecognizerState {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_SpeechRecognition`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpeechRecognizerStateChangedEventArgs(pub ::windows::runtime::IInspectable);
impl SpeechRecognizerStateChangedEventArgs {
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn State(&self) -> ::windows::runtime::Result<SpeechRecognizerState> {
        let this = self;
        unsafe {
            let mut result__: SpeechRecognizerState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognizerState>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpeechRecognizerStateChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognizerStateChangedEventArgs;{563d4f09-ba03-4bad-ad81-ddc6c4dab0c3})");
}
unsafe impl ::windows::runtime::Interface for SpeechRecognizerStateChangedEventArgs {
    type Vtable = ISpeechRecognizerStateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1446858505, 47619, 19373, [173, 129, 221, 198, 196, 218, 176, 195]);
}
impl ::windows::runtime::RuntimeName for SpeechRecognizerStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognizerStateChangedEventArgs";
}
impl ::std::convert::From<SpeechRecognizerStateChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpeechRecognizerStateChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SpeechRecognizerStateChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpeechRecognizerStateChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpeechRecognizerStateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpeechRecognizerStateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SpeechRecognizerStateChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpeechRecognizerStateChangedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpeechRecognizerStateChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SpeechRecognizerStateChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpeechRecognizerStateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpeechRecognizerStateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SpeechRecognizerStateChangedEventArgs {}
unsafe impl ::std::marker::Sync for SpeechRecognizerStateChangedEventArgs {}
#[doc = "*Required features: `Media_SpeechRecognition`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpeechRecognizerTimeouts(pub ::windows::runtime::IInspectable);
impl SpeechRecognizerTimeouts {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation`*"]
    pub fn InitialSilenceTimeout(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation`*"]
    pub fn SetInitialSilenceTimeout<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation`*"]
    pub fn EndSilenceTimeout(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation`*"]
    pub fn SetEndSilenceTimeout<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation`*"]
    pub fn BabbleTimeout(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation`*"]
    pub fn SetBabbleTimeout<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpeechRecognizerTimeouts {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognizerTimeouts;{2ef76fca-6a3c-4dca-a153-df1bc88a79af})");
}
unsafe impl ::windows::runtime::Interface for SpeechRecognizerTimeouts {
    type Vtable = ISpeechRecognizerTimeouts_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(787967946, 27196, 19914, [161, 83, 223, 27, 200, 138, 121, 175]);
}
impl ::windows::runtime::RuntimeName for SpeechRecognizerTimeouts {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognizerTimeouts";
}
impl ::std::convert::From<SpeechRecognizerTimeouts> for ::windows::runtime::IUnknown {
    fn from(value: SpeechRecognizerTimeouts) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SpeechRecognizerTimeouts> for ::windows::runtime::IUnknown {
    fn from(value: &SpeechRecognizerTimeouts) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpeechRecognizerTimeouts {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpeechRecognizerTimeouts {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SpeechRecognizerTimeouts> for ::windows::runtime::IInspectable {
    fn from(value: SpeechRecognizerTimeouts) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpeechRecognizerTimeouts> for ::windows::runtime::IInspectable {
    fn from(value: &SpeechRecognizerTimeouts) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpeechRecognizerTimeouts {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpeechRecognizerTimeouts {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SpeechRecognizerTimeouts {}
unsafe impl ::std::marker::Sync for SpeechRecognizerTimeouts {}
#[doc = "*Required features: `Media_SpeechRecognition`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SpeechRecognizerUIOptions(pub ::windows::runtime::IInspectable);
impl SpeechRecognizerUIOptions {
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn ExampleText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn SetExampleText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn AudiblePrompt(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn SetAudiblePrompt<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn IsReadBackEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn SetIsReadBackEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn ShowConfirmation(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn SetShowConfirmation(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpeechRecognizerUIOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognizerUIOptions;{7888d641-b92b-44ba-a25f-d1864630641f})");
}
unsafe impl ::windows::runtime::Interface for SpeechRecognizerUIOptions {
    type Vtable = ISpeechRecognizerUIOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2022233665, 47403, 17594, [162, 95, 209, 134, 70, 48, 100, 31]);
}
impl ::windows::runtime::RuntimeName for SpeechRecognizerUIOptions {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognizerUIOptions";
}
impl ::std::convert::From<SpeechRecognizerUIOptions> for ::windows::runtime::IUnknown {
    fn from(value: SpeechRecognizerUIOptions) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SpeechRecognizerUIOptions> for ::windows::runtime::IUnknown {
    fn from(value: &SpeechRecognizerUIOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpeechRecognizerUIOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SpeechRecognizerUIOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SpeechRecognizerUIOptions> for ::windows::runtime::IInspectable {
    fn from(value: SpeechRecognizerUIOptions) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpeechRecognizerUIOptions> for ::windows::runtime::IInspectable {
    fn from(value: &SpeechRecognizerUIOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpeechRecognizerUIOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpeechRecognizerUIOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SpeechRecognizerUIOptions {}
unsafe impl ::std::marker::Sync for SpeechRecognizerUIOptions {}
#[doc = "*Required features: `Media_SpeechRecognition`*"]
pub struct VoiceCommandManager {}
impl VoiceCommandManager {
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation`, `Storage`*"]
    pub fn InstallCommandSetsFromStorageFileAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::StorageFile>>(file: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        Self::IVoiceCommandManager(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation_Collections`*"]
    pub fn InstalledCommandSets() -> ::windows::runtime::Result<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, VoiceCommandSet>> {
        Self::IVoiceCommandManager(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, VoiceCommandSet>>(result__)
        })
    }
    pub fn IVoiceCommandManager<R, F: FnOnce(&IVoiceCommandManager) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<VoiceCommandManager, IVoiceCommandManager> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for VoiceCommandManager {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.VoiceCommandManager";
}
#[doc = "*Required features: `Media_SpeechRecognition`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct VoiceCommandSet(pub ::windows::runtime::IInspectable);
impl VoiceCommandSet {
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_SpeechRecognition`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Media_SpeechRecognition`, `Foundation`, `Foundation_Collections`*"]
    pub fn SetPhraseListAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(&self, phraselistname: Param0, phraselist: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), phraselistname.into_param().abi(), phraselist.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for VoiceCommandSet {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.VoiceCommandSet;{0bedda75-46e6-4b11-a088-5c68632899b5})");
}
unsafe impl ::windows::runtime::Interface for VoiceCommandSet {
    type Vtable = IVoiceCommandSet_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(200137333, 18150, 19217, [160, 136, 92, 104, 99, 40, 153, 181]);
}
impl ::windows::runtime::RuntimeName for VoiceCommandSet {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.VoiceCommandSet";
}
impl ::std::convert::From<VoiceCommandSet> for ::windows::runtime::IUnknown {
    fn from(value: VoiceCommandSet) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&VoiceCommandSet> for ::windows::runtime::IUnknown {
    fn from(value: &VoiceCommandSet) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for VoiceCommandSet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a VoiceCommandSet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<VoiceCommandSet> for ::windows::runtime::IInspectable {
    fn from(value: VoiceCommandSet) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VoiceCommandSet> for ::windows::runtime::IInspectable {
    fn from(value: &VoiceCommandSet) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for VoiceCommandSet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a VoiceCommandSet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for VoiceCommandSet {}
unsafe impl ::std::marker::Sync for VoiceCommandSet {}
