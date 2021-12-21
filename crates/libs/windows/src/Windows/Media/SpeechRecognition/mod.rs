#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechContinuousRecognitionCompletedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechContinuousRecognitionCompletedEventArgs {
    type Vtable = ISpeechContinuousRecognitionCompletedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3d069bb_e30c_5e18_424b_7fbe81f8fbd0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechContinuousRecognitionCompletedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionResultStatus) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechContinuousRecognitionResultGeneratedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechContinuousRecognitionResultGeneratedEventArgs {
    type Vtable = ISpeechContinuousRecognitionResultGeneratedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x19091e1e_6e7e_5a46_40fb_76594f786504);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechContinuousRecognitionResultGeneratedEventArgsVtbl(
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
pub struct ISpeechContinuousRecognitionSession(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechContinuousRecognitionSession {
    type Vtable = ISpeechContinuousRecognitionSessionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a213c04_6614_49f8_99a2_b5e9b3a085c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechContinuousRecognitionSessionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: SpeechContinuousRecognitionMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognitionCompilationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognitionCompilationResult {
    type Vtable = ISpeechRecognitionCompilationResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x407e6c5d_6ac7_4da4_9cc1_2fce32cf7489);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionCompilationResultVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionResultStatus) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Media_SpeechRecognition'*"]
#[repr(transparent)]
pub struct ISpeechRecognitionConstraint(::windows::core::IUnknown);
impl ISpeechRecognitionConstraint {
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn SetTag<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn Type(&self) -> ::windows::core::Result<SpeechRecognitionConstraintType> {
        let this = self;
        unsafe {
            let mut result__: SpeechRecognitionConstraintType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionConstraintType>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn Probability(&self) -> ::windows::core::Result<SpeechRecognitionConstraintProbability> {
        let this = self;
        unsafe {
            let mut result__: SpeechRecognitionConstraintProbability = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionConstraintProbability>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn SetProbability(&self, value: SpeechRecognitionConstraintProbability) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::convert::From<ISpeechRecognitionConstraint> for ::windows::core::IInspectable {
    fn from(value: ISpeechRecognitionConstraint) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpeechRecognitionConstraint> for ::windows::core::IInspectable {
    fn from(value: &ISpeechRecognitionConstraint) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ISpeechRecognitionConstraint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ISpeechRecognitionConstraint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISpeechRecognitionConstraint> for ::windows::core::IUnknown {
    fn from(value: ISpeechRecognitionConstraint) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpeechRecognitionConstraint> for ::windows::core::IUnknown {
    fn from(value: &ISpeechRecognitionConstraint) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpeechRecognitionConstraint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISpeechRecognitionConstraint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISpeechRecognitionConstraint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISpeechRecognitionConstraint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpeechRecognitionConstraint {}
impl ::core::fmt::Debug for ISpeechRecognitionConstraint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechRecognitionConstraint").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ISpeechRecognitionConstraint {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{79ac1628-4d68-43c4-8911-40dc4101b55b}");
}
unsafe impl ::windows::core::Interface for ISpeechRecognitionConstraint {
    type Vtable = ISpeechRecognitionConstraintVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79ac1628_4d68_43c4_8911_40dc4101b55b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionConstraintVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionConstraintType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionConstraintProbability) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SpeechRecognitionConstraintProbability) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognitionGrammarFileConstraint(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognitionGrammarFileConstraint {
    type Vtable = ISpeechRecognitionGrammarFileConstraintVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5031a8f_85ca_4fa4_b11a_474fc41b3835);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionGrammarFileConstraintVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognitionGrammarFileConstraintFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognitionGrammarFileConstraintFactory {
    type Vtable = ISpeechRecognitionGrammarFileConstraintFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3da770eb_c479_4c27_9f19_89974ef392d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionGrammarFileConstraintFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, tag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognitionHypothesis(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognitionHypothesis {
    type Vtable = ISpeechRecognitionHypothesisVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a7b25b0_99c5_4f7d_bf84_10aa1302b634);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionHypothesisVtbl(
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
pub struct ISpeechRecognitionHypothesisGeneratedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognitionHypothesisGeneratedEventArgs {
    type Vtable = ISpeechRecognitionHypothesisGeneratedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55161a7a_8023_5866_411d_1213bb271476);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionHypothesisGeneratedEventArgsVtbl(
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
pub struct ISpeechRecognitionListConstraint(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognitionListConstraint {
    type Vtable = ISpeechRecognitionListConstraintVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09c487e9_e4ad_4526_81f2_4946fb481d98);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionListConstraintVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognitionListConstraintFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognitionListConstraintFactory {
    type Vtable = ISpeechRecognitionListConstraintFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40f3cdc7_562a_426a_9f3b_3b4e282be1d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionListConstraintFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commands: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commands: ::windows::core::RawPtr, tag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognitionQualityDegradingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognitionQualityDegradingEventArgs {
    type Vtable = ISpeechRecognitionQualityDegradingEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4fe24105_8c3a_4c7e_8d0a_5bd4f5b14ad8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionQualityDegradingEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionAudioProblem) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognitionResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognitionResult {
    type Vtable = ISpeechRecognitionResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e303157_034e_4652_857e_d0454cc4beec);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionResultVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionResultStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionConfidence) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxalternates: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognitionResult2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognitionResult2 {
    type Vtable = ISpeechRecognitionResult2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf7ed1ba_451b_4166_a0c1_1ffe84032d03);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionResult2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognitionSemanticInterpretation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognitionSemanticInterpretation {
    type Vtable = ISpeechRecognitionSemanticInterpretationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaae1da9b_7e32_4c1f_89fe_0c65f486f52e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionSemanticInterpretationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognitionTopicConstraint(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognitionTopicConstraint {
    type Vtable = ISpeechRecognitionTopicConstraintVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf6fdf19_825d_4e69_a681_36e48cf1c93e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionTopicConstraintVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionScenario) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognitionTopicConstraintFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognitionTopicConstraintFactory {
    type Vtable = ISpeechRecognitionTopicConstraintFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e6863df_ec05_47d7_a5df_56a3431e58d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionTopicConstraintFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scenario: SpeechRecognitionScenario, topichint: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scenario: SpeechRecognitionScenario, topichint: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, tag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognitionVoiceCommandDefinitionConstraint(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognitionVoiceCommandDefinitionConstraint {
    type Vtable = ISpeechRecognitionVoiceCommandDefinitionConstraintVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2791c2b_1ef4_4ae7_9d77_b6ff10b8a3c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognitionVoiceCommandDefinitionConstraintVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognizer(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognizer {
    type Vtable = ISpeechRecognizerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0bc3c9cb_c26a_40f2_aeb5_8096b2e48073);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Globalization"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, speechrecognitionqualitydegradinghandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statechangedhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognizer2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognizer2 {
    type Vtable = ISpeechRecognizer2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63c9baf1_91e3_4ea4_86a1_7c3867d084a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizer2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognizerState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognizerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognizerFactory {
    type Vtable = ISpeechRecognizerFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x60c488dd_7fb8_4033_ac70_d046f64818e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizerFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Globalization"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognizerStateChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognizerStateChangedEventArgs {
    type Vtable = ISpeechRecognizerStateChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x563d4f09_ba03_4bad_ad81_ddc6c4dab0c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizerStateChangedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognizerState) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognizerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognizerStatics {
    type Vtable = ISpeechRecognizerStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87a35eac_a7dc_4b0b_bcc9_24f47c0b7ebf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizerStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Globalization"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Globalization")))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Globalization")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognizerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognizerStatics2 {
    type Vtable = ISpeechRecognizerStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d1b0d95_7565_4ef9_a2f3_ba15162a96cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizerStatics2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, speechlanguage: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Globalization")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognizerTimeouts(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognizerTimeouts {
    type Vtable = ISpeechRecognizerTimeoutsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ef76fca_6a3c_4dca_a153_df1bc88a79af);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizerTimeoutsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechRecognizerUIOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechRecognizerUIOptions {
    type Vtable = ISpeechRecognizerUIOptionsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7888d641_b92b_44ba_a25f_d1864630641f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizerUIOptionsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoiceCommandManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVoiceCommandManager {
    type Vtable = IVoiceCommandManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa3a8dd5_b6e7_4ee2_baa9_dd6baced0a2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoiceCommandSet(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVoiceCommandSet {
    type Vtable = IVoiceCommandSetVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0bedda75_46e6_4b11_a088_5c68632899b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandSetVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phraselistname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, phraselist: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[doc = "*Required features: 'Media_SpeechRecognition'*"]
#[repr(transparent)]
pub struct SpeechContinuousRecognitionCompletedEventArgs(::windows::core::IUnknown);
impl SpeechContinuousRecognitionCompletedEventArgs {
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn Status(&self) -> ::windows::core::Result<SpeechRecognitionResultStatus> {
        let this = self;
        unsafe {
            let mut result__: SpeechRecognitionResultStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionResultStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for SpeechContinuousRecognitionCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpeechContinuousRecognitionCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechContinuousRecognitionCompletedEventArgs {}
impl ::core::fmt::Debug for SpeechContinuousRecognitionCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechContinuousRecognitionCompletedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpeechContinuousRecognitionCompletedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechContinuousRecognitionCompletedEventArgs;{e3d069bb-e30c-5e18-424b-7fbe81f8fbd0})");
}
unsafe impl ::windows::core::Interface for SpeechContinuousRecognitionCompletedEventArgs {
    type Vtable = ISpeechContinuousRecognitionCompletedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3d069bb_e30c_5e18_424b_7fbe81f8fbd0);
}
impl ::windows::core::RuntimeName for SpeechContinuousRecognitionCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechContinuousRecognitionCompletedEventArgs";
}
impl ::core::convert::From<SpeechContinuousRecognitionCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: SpeechContinuousRecognitionCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechContinuousRecognitionCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SpeechContinuousRecognitionCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpeechContinuousRecognitionCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SpeechContinuousRecognitionCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpeechContinuousRecognitionCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: SpeechContinuousRecognitionCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechContinuousRecognitionCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SpeechContinuousRecognitionCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpeechContinuousRecognitionCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SpeechContinuousRecognitionCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpeechContinuousRecognitionCompletedEventArgs {}
unsafe impl ::core::marker::Sync for SpeechContinuousRecognitionCompletedEventArgs {}
#[doc = "*Required features: 'Media_SpeechRecognition'*"]
#[repr(transparent)]
pub struct SpeechContinuousRecognitionMode(pub i32);
impl SpeechContinuousRecognitionMode {
    pub const Default: Self = Self(0i32);
    pub const PauseOnRecognition: Self = Self(1i32);
}
impl ::core::marker::Copy for SpeechContinuousRecognitionMode {}
impl ::core::clone::Clone for SpeechContinuousRecognitionMode {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SpeechContinuousRecognitionMode {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SpeechContinuousRecognitionMode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechContinuousRecognitionMode {}
impl ::core::fmt::Debug for SpeechContinuousRecognitionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechContinuousRecognitionMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpeechContinuousRecognitionMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechContinuousRecognitionMode;i4)");
}
impl ::windows::core::DefaultType for SpeechContinuousRecognitionMode {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_SpeechRecognition'*"]
#[repr(transparent)]
pub struct SpeechContinuousRecognitionResultGeneratedEventArgs(::windows::core::IUnknown);
impl SpeechContinuousRecognitionResultGeneratedEventArgs {
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn Result(&self) -> ::windows::core::Result<SpeechRecognitionResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionResult>(result__)
        }
    }
}
impl ::core::clone::Clone for SpeechContinuousRecognitionResultGeneratedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpeechContinuousRecognitionResultGeneratedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechContinuousRecognitionResultGeneratedEventArgs {}
impl ::core::fmt::Debug for SpeechContinuousRecognitionResultGeneratedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechContinuousRecognitionResultGeneratedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpeechContinuousRecognitionResultGeneratedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechContinuousRecognitionResultGeneratedEventArgs;{19091e1e-6e7e-5a46-40fb-76594f786504})");
}
unsafe impl ::windows::core::Interface for SpeechContinuousRecognitionResultGeneratedEventArgs {
    type Vtable = ISpeechContinuousRecognitionResultGeneratedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x19091e1e_6e7e_5a46_40fb_76594f786504);
}
impl ::windows::core::RuntimeName for SpeechContinuousRecognitionResultGeneratedEventArgs {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechContinuousRecognitionResultGeneratedEventArgs";
}
impl ::core::convert::From<SpeechContinuousRecognitionResultGeneratedEventArgs> for ::windows::core::IUnknown {
    fn from(value: SpeechContinuousRecognitionResultGeneratedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechContinuousRecognitionResultGeneratedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SpeechContinuousRecognitionResultGeneratedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpeechContinuousRecognitionResultGeneratedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SpeechContinuousRecognitionResultGeneratedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpeechContinuousRecognitionResultGeneratedEventArgs> for ::windows::core::IInspectable {
    fn from(value: SpeechContinuousRecognitionResultGeneratedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechContinuousRecognitionResultGeneratedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SpeechContinuousRecognitionResultGeneratedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpeechContinuousRecognitionResultGeneratedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SpeechContinuousRecognitionResultGeneratedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpeechContinuousRecognitionResultGeneratedEventArgs {}
unsafe impl ::core::marker::Sync for SpeechContinuousRecognitionResultGeneratedEventArgs {}
#[doc = "*Required features: 'Media_SpeechRecognition'*"]
#[repr(transparent)]
pub struct SpeechContinuousRecognitionSession(::windows::core::IUnknown);
impl SpeechContinuousRecognitionSession {
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn AutoStopSilenceTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetAutoStopSilenceTimeout<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StartAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StartWithModeAsync(&self, mode: SpeechContinuousRecognitionMode) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), mode, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StopAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn CancelAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn PauseAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn Resume(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Completed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SpeechContinuousRecognitionSession, SpeechContinuousRecognitionCompletedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ResultGenerated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SpeechContinuousRecognitionSession, SpeechContinuousRecognitionResultGeneratedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveResultGenerated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for SpeechContinuousRecognitionSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpeechContinuousRecognitionSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechContinuousRecognitionSession {}
impl ::core::fmt::Debug for SpeechContinuousRecognitionSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechContinuousRecognitionSession").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpeechContinuousRecognitionSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechContinuousRecognitionSession;{6a213c04-6614-49f8-99a2-b5e9b3a085c8})");
}
unsafe impl ::windows::core::Interface for SpeechContinuousRecognitionSession {
    type Vtable = ISpeechContinuousRecognitionSessionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a213c04_6614_49f8_99a2_b5e9b3a085c8);
}
impl ::windows::core::RuntimeName for SpeechContinuousRecognitionSession {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechContinuousRecognitionSession";
}
impl ::core::convert::From<SpeechContinuousRecognitionSession> for ::windows::core::IUnknown {
    fn from(value: SpeechContinuousRecognitionSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechContinuousRecognitionSession> for ::windows::core::IUnknown {
    fn from(value: &SpeechContinuousRecognitionSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpeechContinuousRecognitionSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SpeechContinuousRecognitionSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpeechContinuousRecognitionSession> for ::windows::core::IInspectable {
    fn from(value: SpeechContinuousRecognitionSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechContinuousRecognitionSession> for ::windows::core::IInspectable {
    fn from(value: &SpeechContinuousRecognitionSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpeechContinuousRecognitionSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SpeechContinuousRecognitionSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpeechContinuousRecognitionSession {}
unsafe impl ::core::marker::Sync for SpeechContinuousRecognitionSession {}
#[doc = "*Required features: 'Media_SpeechRecognition'*"]
#[repr(transparent)]
pub struct SpeechRecognitionAudioProblem(pub i32);
impl SpeechRecognitionAudioProblem {
    pub const None: Self = Self(0i32);
    pub const TooNoisy: Self = Self(1i32);
    pub const NoSignal: Self = Self(2i32);
    pub const TooLoud: Self = Self(3i32);
    pub const TooQuiet: Self = Self(4i32);
    pub const TooFast: Self = Self(5i32);
    pub const TooSlow: Self = Self(6i32);
}
impl ::core::marker::Copy for SpeechRecognitionAudioProblem {}
impl ::core::clone::Clone for SpeechRecognitionAudioProblem {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SpeechRecognitionAudioProblem {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SpeechRecognitionAudioProblem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechRecognitionAudioProblem {}
impl ::core::fmt::Debug for SpeechRecognitionAudioProblem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionAudioProblem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpeechRecognitionAudioProblem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechRecognitionAudioProblem;i4)");
}
impl ::windows::core::DefaultType for SpeechRecognitionAudioProblem {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_SpeechRecognition'*"]
#[repr(transparent)]
pub struct SpeechRecognitionCompilationResult(::windows::core::IUnknown);
impl SpeechRecognitionCompilationResult {
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn Status(&self) -> ::windows::core::Result<SpeechRecognitionResultStatus> {
        let this = self;
        unsafe {
            let mut result__: SpeechRecognitionResultStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionResultStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for SpeechRecognitionCompilationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpeechRecognitionCompilationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechRecognitionCompilationResult {}
impl ::core::fmt::Debug for SpeechRecognitionCompilationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionCompilationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpeechRecognitionCompilationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionCompilationResult;{407e6c5d-6ac7-4da4-9cc1-2fce32cf7489})");
}
unsafe impl ::windows::core::Interface for SpeechRecognitionCompilationResult {
    type Vtable = ISpeechRecognitionCompilationResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x407e6c5d_6ac7_4da4_9cc1_2fce32cf7489);
}
impl ::windows::core::RuntimeName for SpeechRecognitionCompilationResult {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionCompilationResult";
}
impl ::core::convert::From<SpeechRecognitionCompilationResult> for ::windows::core::IUnknown {
    fn from(value: SpeechRecognitionCompilationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechRecognitionCompilationResult> for ::windows::core::IUnknown {
    fn from(value: &SpeechRecognitionCompilationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpeechRecognitionCompilationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SpeechRecognitionCompilationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpeechRecognitionCompilationResult> for ::windows::core::IInspectable {
    fn from(value: SpeechRecognitionCompilationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechRecognitionCompilationResult> for ::windows::core::IInspectable {
    fn from(value: &SpeechRecognitionCompilationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpeechRecognitionCompilationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SpeechRecognitionCompilationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpeechRecognitionCompilationResult {}
unsafe impl ::core::marker::Sync for SpeechRecognitionCompilationResult {}
#[doc = "*Required features: 'Media_SpeechRecognition'*"]
#[repr(transparent)]
pub struct SpeechRecognitionConfidence(pub i32);
impl SpeechRecognitionConfidence {
    pub const High: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const Low: Self = Self(2i32);
    pub const Rejected: Self = Self(3i32);
}
impl ::core::marker::Copy for SpeechRecognitionConfidence {}
impl ::core::clone::Clone for SpeechRecognitionConfidence {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SpeechRecognitionConfidence {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SpeechRecognitionConfidence {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechRecognitionConfidence {}
impl ::core::fmt::Debug for SpeechRecognitionConfidence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionConfidence").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpeechRecognitionConfidence {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechRecognitionConfidence;i4)");
}
impl ::windows::core::DefaultType for SpeechRecognitionConfidence {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_SpeechRecognition'*"]
#[repr(transparent)]
pub struct SpeechRecognitionConstraintProbability(pub i32);
impl SpeechRecognitionConstraintProbability {
    pub const Default: Self = Self(0i32);
    pub const Min: Self = Self(1i32);
    pub const Max: Self = Self(2i32);
}
impl ::core::marker::Copy for SpeechRecognitionConstraintProbability {}
impl ::core::clone::Clone for SpeechRecognitionConstraintProbability {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SpeechRecognitionConstraintProbability {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SpeechRecognitionConstraintProbability {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechRecognitionConstraintProbability {}
impl ::core::fmt::Debug for SpeechRecognitionConstraintProbability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionConstraintProbability").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpeechRecognitionConstraintProbability {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechRecognitionConstraintProbability;i4)");
}
impl ::windows::core::DefaultType for SpeechRecognitionConstraintProbability {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_SpeechRecognition'*"]
#[repr(transparent)]
pub struct SpeechRecognitionConstraintType(pub i32);
impl SpeechRecognitionConstraintType {
    pub const Topic: Self = Self(0i32);
    pub const List: Self = Self(1i32);
    pub const Grammar: Self = Self(2i32);
    pub const VoiceCommandDefinition: Self = Self(3i32);
}
impl ::core::marker::Copy for SpeechRecognitionConstraintType {}
impl ::core::clone::Clone for SpeechRecognitionConstraintType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SpeechRecognitionConstraintType {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SpeechRecognitionConstraintType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechRecognitionConstraintType {}
impl ::core::fmt::Debug for SpeechRecognitionConstraintType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionConstraintType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpeechRecognitionConstraintType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechRecognitionConstraintType;i4)");
}
impl ::windows::core::DefaultType for SpeechRecognitionConstraintType {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_SpeechRecognition'*"]
#[repr(transparent)]
pub struct SpeechRecognitionGrammarFileConstraint(::windows::core::IUnknown);
impl SpeechRecognitionGrammarFileConstraint {
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn SetTag<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn Type(&self) -> ::windows::core::Result<SpeechRecognitionConstraintType> {
        let this = &::windows::core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__: SpeechRecognitionConstraintType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionConstraintType>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn Probability(&self) -> ::windows::core::Result<SpeechRecognitionConstraintProbability> {
        let this = &::windows::core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__: SpeechRecognitionConstraintProbability = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionConstraintProbability>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn SetProbability(&self, value: SpeechRecognitionConstraintProbability) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Storage'*"]
    #[cfg(feature = "Storage")]
    pub fn GrammarFile(&self) -> ::windows::core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Storage'*"]
    #[cfg(feature = "Storage")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::StorageFile>>(file: Param0) -> ::windows::core::Result<SpeechRecognitionGrammarFileConstraint> {
        Self::ISpeechRecognitionGrammarFileConstraintFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<SpeechRecognitionGrammarFileConstraint>(result__)
        })
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Storage'*"]
    #[cfg(feature = "Storage")]
    pub fn CreateWithTag<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::StorageFile>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(file: Param0, tag: Param1) -> ::windows::core::Result<SpeechRecognitionGrammarFileConstraint> {
        Self::ISpeechRecognitionGrammarFileConstraintFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), file.into_param().abi(), tag.into_param().abi(), &mut result__).from_abi::<SpeechRecognitionGrammarFileConstraint>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpeechRecognitionGrammarFileConstraintFactory<R, F: FnOnce(&ISpeechRecognitionGrammarFileConstraintFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpeechRecognitionGrammarFileConstraint, ISpeechRecognitionGrammarFileConstraintFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SpeechRecognitionGrammarFileConstraint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpeechRecognitionGrammarFileConstraint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechRecognitionGrammarFileConstraint {}
impl ::core::fmt::Debug for SpeechRecognitionGrammarFileConstraint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionGrammarFileConstraint").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpeechRecognitionGrammarFileConstraint {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionGrammarFileConstraint;{b5031a8f-85ca-4fa4-b11a-474fc41b3835})");
}
unsafe impl ::windows::core::Interface for SpeechRecognitionGrammarFileConstraint {
    type Vtable = ISpeechRecognitionGrammarFileConstraintVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5031a8f_85ca_4fa4_b11a_474fc41b3835);
}
impl ::windows::core::RuntimeName for SpeechRecognitionGrammarFileConstraint {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionGrammarFileConstraint";
}
impl ::core::convert::From<SpeechRecognitionGrammarFileConstraint> for ::windows::core::IUnknown {
    fn from(value: SpeechRecognitionGrammarFileConstraint) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechRecognitionGrammarFileConstraint> for ::windows::core::IUnknown {
    fn from(value: &SpeechRecognitionGrammarFileConstraint) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpeechRecognitionGrammarFileConstraint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SpeechRecognitionGrammarFileConstraint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpeechRecognitionGrammarFileConstraint> for ::windows::core::IInspectable {
    fn from(value: SpeechRecognitionGrammarFileConstraint) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechRecognitionGrammarFileConstraint> for ::windows::core::IInspectable {
    fn from(value: &SpeechRecognitionGrammarFileConstraint) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpeechRecognitionGrammarFileConstraint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SpeechRecognitionGrammarFileConstraint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SpeechRecognitionGrammarFileConstraint> for ISpeechRecognitionConstraint {
    type Error = ::windows::core::Error;
    fn try_from(value: SpeechRecognitionGrammarFileConstraint) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SpeechRecognitionGrammarFileConstraint> for ISpeechRecognitionConstraint {
    type Error = ::windows::core::Error;
    fn try_from(value: &SpeechRecognitionGrammarFileConstraint) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISpeechRecognitionConstraint> for SpeechRecognitionGrammarFileConstraint {
    fn into_param(self) -> ::windows::core::Param<'a, ISpeechRecognitionConstraint> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISpeechRecognitionConstraint> for &SpeechRecognitionGrammarFileConstraint {
    fn into_param(self) -> ::windows::core::Param<'a, ISpeechRecognitionConstraint> {
        ::core::convert::TryInto::<ISpeechRecognitionConstraint>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SpeechRecognitionGrammarFileConstraint {}
unsafe impl ::core::marker::Sync for SpeechRecognitionGrammarFileConstraint {}
#[doc = "*Required features: 'Media_SpeechRecognition'*"]
#[repr(transparent)]
pub struct SpeechRecognitionHypothesis(::windows::core::IUnknown);
impl SpeechRecognitionHypothesis {
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for SpeechRecognitionHypothesis {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpeechRecognitionHypothesis {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechRecognitionHypothesis {}
impl ::core::fmt::Debug for SpeechRecognitionHypothesis {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionHypothesis").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpeechRecognitionHypothesis {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionHypothesis;{7a7b25b0-99c5-4f7d-bf84-10aa1302b634})");
}
unsafe impl ::windows::core::Interface for SpeechRecognitionHypothesis {
    type Vtable = ISpeechRecognitionHypothesisVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a7b25b0_99c5_4f7d_bf84_10aa1302b634);
}
impl ::windows::core::RuntimeName for SpeechRecognitionHypothesis {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionHypothesis";
}
impl ::core::convert::From<SpeechRecognitionHypothesis> for ::windows::core::IUnknown {
    fn from(value: SpeechRecognitionHypothesis) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechRecognitionHypothesis> for ::windows::core::IUnknown {
    fn from(value: &SpeechRecognitionHypothesis) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpeechRecognitionHypothesis {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SpeechRecognitionHypothesis {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpeechRecognitionHypothesis> for ::windows::core::IInspectable {
    fn from(value: SpeechRecognitionHypothesis) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechRecognitionHypothesis> for ::windows::core::IInspectable {
    fn from(value: &SpeechRecognitionHypothesis) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpeechRecognitionHypothesis {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SpeechRecognitionHypothesis {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpeechRecognitionHypothesis {}
unsafe impl ::core::marker::Sync for SpeechRecognitionHypothesis {}
#[doc = "*Required features: 'Media_SpeechRecognition'*"]
#[repr(transparent)]
pub struct SpeechRecognitionHypothesisGeneratedEventArgs(::windows::core::IUnknown);
impl SpeechRecognitionHypothesisGeneratedEventArgs {
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn Hypothesis(&self) -> ::windows::core::Result<SpeechRecognitionHypothesis> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionHypothesis>(result__)
        }
    }
}
impl ::core::clone::Clone for SpeechRecognitionHypothesisGeneratedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpeechRecognitionHypothesisGeneratedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechRecognitionHypothesisGeneratedEventArgs {}
impl ::core::fmt::Debug for SpeechRecognitionHypothesisGeneratedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionHypothesisGeneratedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpeechRecognitionHypothesisGeneratedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionHypothesisGeneratedEventArgs;{55161a7a-8023-5866-411d-1213bb271476})");
}
unsafe impl ::windows::core::Interface for SpeechRecognitionHypothesisGeneratedEventArgs {
    type Vtable = ISpeechRecognitionHypothesisGeneratedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55161a7a_8023_5866_411d_1213bb271476);
}
impl ::windows::core::RuntimeName for SpeechRecognitionHypothesisGeneratedEventArgs {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionHypothesisGeneratedEventArgs";
}
impl ::core::convert::From<SpeechRecognitionHypothesisGeneratedEventArgs> for ::windows::core::IUnknown {
    fn from(value: SpeechRecognitionHypothesisGeneratedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechRecognitionHypothesisGeneratedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SpeechRecognitionHypothesisGeneratedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpeechRecognitionHypothesisGeneratedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SpeechRecognitionHypothesisGeneratedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpeechRecognitionHypothesisGeneratedEventArgs> for ::windows::core::IInspectable {
    fn from(value: SpeechRecognitionHypothesisGeneratedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechRecognitionHypothesisGeneratedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SpeechRecognitionHypothesisGeneratedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpeechRecognitionHypothesisGeneratedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SpeechRecognitionHypothesisGeneratedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpeechRecognitionHypothesisGeneratedEventArgs {}
unsafe impl ::core::marker::Sync for SpeechRecognitionHypothesisGeneratedEventArgs {}
#[doc = "*Required features: 'Media_SpeechRecognition'*"]
#[repr(transparent)]
pub struct SpeechRecognitionListConstraint(::windows::core::IUnknown);
impl SpeechRecognitionListConstraint {
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn SetTag<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn Type(&self) -> ::windows::core::Result<SpeechRecognitionConstraintType> {
        let this = &::windows::core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__: SpeechRecognitionConstraintType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionConstraintType>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn Probability(&self) -> ::windows::core::Result<SpeechRecognitionConstraintProbability> {
        let this = &::windows::core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__: SpeechRecognitionConstraintProbability = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionConstraintProbability>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn SetProbability(&self, value: SpeechRecognitionConstraintProbability) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Commands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(commands: Param0) -> ::windows::core::Result<SpeechRecognitionListConstraint> {
        Self::ISpeechRecognitionListConstraintFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), commands.into_param().abi(), &mut result__).from_abi::<SpeechRecognitionListConstraint>(result__)
        })
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithTag<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(commands: Param0, tag: Param1) -> ::windows::core::Result<SpeechRecognitionListConstraint> {
        Self::ISpeechRecognitionListConstraintFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), commands.into_param().abi(), tag.into_param().abi(), &mut result__).from_abi::<SpeechRecognitionListConstraint>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpeechRecognitionListConstraintFactory<R, F: FnOnce(&ISpeechRecognitionListConstraintFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpeechRecognitionListConstraint, ISpeechRecognitionListConstraintFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SpeechRecognitionListConstraint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpeechRecognitionListConstraint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechRecognitionListConstraint {}
impl ::core::fmt::Debug for SpeechRecognitionListConstraint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionListConstraint").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpeechRecognitionListConstraint {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionListConstraint;{09c487e9-e4ad-4526-81f2-4946fb481d98})");
}
unsafe impl ::windows::core::Interface for SpeechRecognitionListConstraint {
    type Vtable = ISpeechRecognitionListConstraintVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09c487e9_e4ad_4526_81f2_4946fb481d98);
}
impl ::windows::core::RuntimeName for SpeechRecognitionListConstraint {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionListConstraint";
}
impl ::core::convert::From<SpeechRecognitionListConstraint> for ::windows::core::IUnknown {
    fn from(value: SpeechRecognitionListConstraint) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechRecognitionListConstraint> for ::windows::core::IUnknown {
    fn from(value: &SpeechRecognitionListConstraint) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpeechRecognitionListConstraint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SpeechRecognitionListConstraint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpeechRecognitionListConstraint> for ::windows::core::IInspectable {
    fn from(value: SpeechRecognitionListConstraint) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechRecognitionListConstraint> for ::windows::core::IInspectable {
    fn from(value: &SpeechRecognitionListConstraint) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpeechRecognitionListConstraint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SpeechRecognitionListConstraint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SpeechRecognitionListConstraint> for ISpeechRecognitionConstraint {
    type Error = ::windows::core::Error;
    fn try_from(value: SpeechRecognitionListConstraint) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SpeechRecognitionListConstraint> for ISpeechRecognitionConstraint {
    type Error = ::windows::core::Error;
    fn try_from(value: &SpeechRecognitionListConstraint) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISpeechRecognitionConstraint> for SpeechRecognitionListConstraint {
    fn into_param(self) -> ::windows::core::Param<'a, ISpeechRecognitionConstraint> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISpeechRecognitionConstraint> for &SpeechRecognitionListConstraint {
    fn into_param(self) -> ::windows::core::Param<'a, ISpeechRecognitionConstraint> {
        ::core::convert::TryInto::<ISpeechRecognitionConstraint>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SpeechRecognitionListConstraint {}
unsafe impl ::core::marker::Sync for SpeechRecognitionListConstraint {}
#[doc = "*Required features: 'Media_SpeechRecognition'*"]
#[repr(transparent)]
pub struct SpeechRecognitionQualityDegradingEventArgs(::windows::core::IUnknown);
impl SpeechRecognitionQualityDegradingEventArgs {
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn Problem(&self) -> ::windows::core::Result<SpeechRecognitionAudioProblem> {
        let this = self;
        unsafe {
            let mut result__: SpeechRecognitionAudioProblem = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionAudioProblem>(result__)
        }
    }
}
impl ::core::clone::Clone for SpeechRecognitionQualityDegradingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpeechRecognitionQualityDegradingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechRecognitionQualityDegradingEventArgs {}
impl ::core::fmt::Debug for SpeechRecognitionQualityDegradingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionQualityDegradingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpeechRecognitionQualityDegradingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionQualityDegradingEventArgs;{4fe24105-8c3a-4c7e-8d0a-5bd4f5b14ad8})");
}
unsafe impl ::windows::core::Interface for SpeechRecognitionQualityDegradingEventArgs {
    type Vtable = ISpeechRecognitionQualityDegradingEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4fe24105_8c3a_4c7e_8d0a_5bd4f5b14ad8);
}
impl ::windows::core::RuntimeName for SpeechRecognitionQualityDegradingEventArgs {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionQualityDegradingEventArgs";
}
impl ::core::convert::From<SpeechRecognitionQualityDegradingEventArgs> for ::windows::core::IUnknown {
    fn from(value: SpeechRecognitionQualityDegradingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechRecognitionQualityDegradingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SpeechRecognitionQualityDegradingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpeechRecognitionQualityDegradingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SpeechRecognitionQualityDegradingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpeechRecognitionQualityDegradingEventArgs> for ::windows::core::IInspectable {
    fn from(value: SpeechRecognitionQualityDegradingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechRecognitionQualityDegradingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SpeechRecognitionQualityDegradingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpeechRecognitionQualityDegradingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SpeechRecognitionQualityDegradingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpeechRecognitionQualityDegradingEventArgs {}
unsafe impl ::core::marker::Sync for SpeechRecognitionQualityDegradingEventArgs {}
#[doc = "*Required features: 'Media_SpeechRecognition'*"]
#[repr(transparent)]
pub struct SpeechRecognitionResult(::windows::core::IUnknown);
impl SpeechRecognitionResult {
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn Status(&self) -> ::windows::core::Result<SpeechRecognitionResultStatus> {
        let this = self;
        unsafe {
            let mut result__: SpeechRecognitionResultStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionResultStatus>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn Confidence(&self) -> ::windows::core::Result<SpeechRecognitionConfidence> {
        let this = self;
        unsafe {
            let mut result__: SpeechRecognitionConfidence = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionConfidence>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn SemanticInterpretation(&self) -> ::windows::core::Result<SpeechRecognitionSemanticInterpretation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionSemanticInterpretation>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAlternates(&self, maxalternates: u32) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SpeechRecognitionResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), maxalternates, &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<SpeechRecognitionResult>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn Constraint(&self) -> ::windows::core::Result<ISpeechRecognitionConstraint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ISpeechRecognitionConstraint>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RulePath(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn RawConfidence(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn PhraseStartTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<ISpeechRecognitionResult2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn PhraseDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<ISpeechRecognitionResult2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for SpeechRecognitionResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpeechRecognitionResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechRecognitionResult {}
impl ::core::fmt::Debug for SpeechRecognitionResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpeechRecognitionResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionResult;{4e303157-034e-4652-857e-d0454cc4beec})");
}
unsafe impl ::windows::core::Interface for SpeechRecognitionResult {
    type Vtable = ISpeechRecognitionResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e303157_034e_4652_857e_d0454cc4beec);
}
impl ::windows::core::RuntimeName for SpeechRecognitionResult {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionResult";
}
impl ::core::convert::From<SpeechRecognitionResult> for ::windows::core::IUnknown {
    fn from(value: SpeechRecognitionResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechRecognitionResult> for ::windows::core::IUnknown {
    fn from(value: &SpeechRecognitionResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpeechRecognitionResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SpeechRecognitionResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpeechRecognitionResult> for ::windows::core::IInspectable {
    fn from(value: SpeechRecognitionResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechRecognitionResult> for ::windows::core::IInspectable {
    fn from(value: &SpeechRecognitionResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpeechRecognitionResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SpeechRecognitionResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpeechRecognitionResult {}
unsafe impl ::core::marker::Sync for SpeechRecognitionResult {}
#[doc = "*Required features: 'Media_SpeechRecognition'*"]
#[repr(transparent)]
pub struct SpeechRecognitionResultStatus(pub i32);
impl SpeechRecognitionResultStatus {
    pub const Success: Self = Self(0i32);
    pub const TopicLanguageNotSupported: Self = Self(1i32);
    pub const GrammarLanguageMismatch: Self = Self(2i32);
    pub const GrammarCompilationFailure: Self = Self(3i32);
    pub const AudioQualityFailure: Self = Self(4i32);
    pub const UserCanceled: Self = Self(5i32);
    pub const Unknown: Self = Self(6i32);
    pub const TimeoutExceeded: Self = Self(7i32);
    pub const PauseLimitExceeded: Self = Self(8i32);
    pub const NetworkFailure: Self = Self(9i32);
    pub const MicrophoneUnavailable: Self = Self(10i32);
}
impl ::core::marker::Copy for SpeechRecognitionResultStatus {}
impl ::core::clone::Clone for SpeechRecognitionResultStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SpeechRecognitionResultStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SpeechRecognitionResultStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechRecognitionResultStatus {}
impl ::core::fmt::Debug for SpeechRecognitionResultStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionResultStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpeechRecognitionResultStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechRecognitionResultStatus;i4)");
}
impl ::windows::core::DefaultType for SpeechRecognitionResultStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_SpeechRecognition'*"]
#[repr(transparent)]
pub struct SpeechRecognitionScenario(pub i32);
impl SpeechRecognitionScenario {
    pub const WebSearch: Self = Self(0i32);
    pub const Dictation: Self = Self(1i32);
    pub const FormFilling: Self = Self(2i32);
}
impl ::core::marker::Copy for SpeechRecognitionScenario {}
impl ::core::clone::Clone for SpeechRecognitionScenario {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SpeechRecognitionScenario {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SpeechRecognitionScenario {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechRecognitionScenario {}
impl ::core::fmt::Debug for SpeechRecognitionScenario {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionScenario").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpeechRecognitionScenario {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechRecognitionScenario;i4)");
}
impl ::windows::core::DefaultType for SpeechRecognitionScenario {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_SpeechRecognition'*"]
#[repr(transparent)]
pub struct SpeechRecognitionSemanticInterpretation(::windows::core::IUnknown);
impl SpeechRecognitionSemanticInterpretation {
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>(result__)
        }
    }
}
impl ::core::clone::Clone for SpeechRecognitionSemanticInterpretation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpeechRecognitionSemanticInterpretation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechRecognitionSemanticInterpretation {}
impl ::core::fmt::Debug for SpeechRecognitionSemanticInterpretation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionSemanticInterpretation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpeechRecognitionSemanticInterpretation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionSemanticInterpretation;{aae1da9b-7e32-4c1f-89fe-0c65f486f52e})");
}
unsafe impl ::windows::core::Interface for SpeechRecognitionSemanticInterpretation {
    type Vtable = ISpeechRecognitionSemanticInterpretationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaae1da9b_7e32_4c1f_89fe_0c65f486f52e);
}
impl ::windows::core::RuntimeName for SpeechRecognitionSemanticInterpretation {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionSemanticInterpretation";
}
impl ::core::convert::From<SpeechRecognitionSemanticInterpretation> for ::windows::core::IUnknown {
    fn from(value: SpeechRecognitionSemanticInterpretation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechRecognitionSemanticInterpretation> for ::windows::core::IUnknown {
    fn from(value: &SpeechRecognitionSemanticInterpretation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpeechRecognitionSemanticInterpretation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SpeechRecognitionSemanticInterpretation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpeechRecognitionSemanticInterpretation> for ::windows::core::IInspectable {
    fn from(value: SpeechRecognitionSemanticInterpretation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechRecognitionSemanticInterpretation> for ::windows::core::IInspectable {
    fn from(value: &SpeechRecognitionSemanticInterpretation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpeechRecognitionSemanticInterpretation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SpeechRecognitionSemanticInterpretation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpeechRecognitionSemanticInterpretation {}
unsafe impl ::core::marker::Sync for SpeechRecognitionSemanticInterpretation {}
#[doc = "*Required features: 'Media_SpeechRecognition'*"]
#[repr(transparent)]
pub struct SpeechRecognitionTopicConstraint(::windows::core::IUnknown);
impl SpeechRecognitionTopicConstraint {
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn SetTag<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn Type(&self) -> ::windows::core::Result<SpeechRecognitionConstraintType> {
        let this = &::windows::core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__: SpeechRecognitionConstraintType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionConstraintType>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn Probability(&self) -> ::windows::core::Result<SpeechRecognitionConstraintProbability> {
        let this = &::windows::core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__: SpeechRecognitionConstraintProbability = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionConstraintProbability>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn SetProbability(&self, value: SpeechRecognitionConstraintProbability) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn Scenario(&self) -> ::windows::core::Result<SpeechRecognitionScenario> {
        let this = self;
        unsafe {
            let mut result__: SpeechRecognitionScenario = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionScenario>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn TopicHint(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn Create<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(scenario: SpeechRecognitionScenario, topichint: Param1) -> ::windows::core::Result<SpeechRecognitionTopicConstraint> {
        Self::ISpeechRecognitionTopicConstraintFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), scenario, topichint.into_param().abi(), &mut result__).from_abi::<SpeechRecognitionTopicConstraint>(result__)
        })
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn CreateWithTag<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(scenario: SpeechRecognitionScenario, topichint: Param1, tag: Param2) -> ::windows::core::Result<SpeechRecognitionTopicConstraint> {
        Self::ISpeechRecognitionTopicConstraintFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), scenario, topichint.into_param().abi(), tag.into_param().abi(), &mut result__).from_abi::<SpeechRecognitionTopicConstraint>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpeechRecognitionTopicConstraintFactory<R, F: FnOnce(&ISpeechRecognitionTopicConstraintFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpeechRecognitionTopicConstraint, ISpeechRecognitionTopicConstraintFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SpeechRecognitionTopicConstraint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpeechRecognitionTopicConstraint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechRecognitionTopicConstraint {}
impl ::core::fmt::Debug for SpeechRecognitionTopicConstraint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionTopicConstraint").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpeechRecognitionTopicConstraint {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionTopicConstraint;{bf6fdf19-825d-4e69-a681-36e48cf1c93e})");
}
unsafe impl ::windows::core::Interface for SpeechRecognitionTopicConstraint {
    type Vtable = ISpeechRecognitionTopicConstraintVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf6fdf19_825d_4e69_a681_36e48cf1c93e);
}
impl ::windows::core::RuntimeName for SpeechRecognitionTopicConstraint {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionTopicConstraint";
}
impl ::core::convert::From<SpeechRecognitionTopicConstraint> for ::windows::core::IUnknown {
    fn from(value: SpeechRecognitionTopicConstraint) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechRecognitionTopicConstraint> for ::windows::core::IUnknown {
    fn from(value: &SpeechRecognitionTopicConstraint) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpeechRecognitionTopicConstraint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SpeechRecognitionTopicConstraint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpeechRecognitionTopicConstraint> for ::windows::core::IInspectable {
    fn from(value: SpeechRecognitionTopicConstraint) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechRecognitionTopicConstraint> for ::windows::core::IInspectable {
    fn from(value: &SpeechRecognitionTopicConstraint) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpeechRecognitionTopicConstraint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SpeechRecognitionTopicConstraint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SpeechRecognitionTopicConstraint> for ISpeechRecognitionConstraint {
    type Error = ::windows::core::Error;
    fn try_from(value: SpeechRecognitionTopicConstraint) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SpeechRecognitionTopicConstraint> for ISpeechRecognitionConstraint {
    type Error = ::windows::core::Error;
    fn try_from(value: &SpeechRecognitionTopicConstraint) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISpeechRecognitionConstraint> for SpeechRecognitionTopicConstraint {
    fn into_param(self) -> ::windows::core::Param<'a, ISpeechRecognitionConstraint> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISpeechRecognitionConstraint> for &SpeechRecognitionTopicConstraint {
    fn into_param(self) -> ::windows::core::Param<'a, ISpeechRecognitionConstraint> {
        ::core::convert::TryInto::<ISpeechRecognitionConstraint>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SpeechRecognitionTopicConstraint {}
unsafe impl ::core::marker::Sync for SpeechRecognitionTopicConstraint {}
#[doc = "*Required features: 'Media_SpeechRecognition'*"]
#[repr(transparent)]
pub struct SpeechRecognitionVoiceCommandDefinitionConstraint(::windows::core::IUnknown);
impl SpeechRecognitionVoiceCommandDefinitionConstraint {
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn SetTag<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn Type(&self) -> ::windows::core::Result<SpeechRecognitionConstraintType> {
        let this = &::windows::core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__: SpeechRecognitionConstraintType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionConstraintType>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn Probability(&self) -> ::windows::core::Result<SpeechRecognitionConstraintProbability> {
        let this = &::windows::core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe {
            let mut result__: SpeechRecognitionConstraintProbability = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognitionConstraintProbability>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn SetProbability(&self, value: SpeechRecognitionConstraintProbability) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISpeechRecognitionConstraint>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for SpeechRecognitionVoiceCommandDefinitionConstraint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpeechRecognitionVoiceCommandDefinitionConstraint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechRecognitionVoiceCommandDefinitionConstraint {}
impl ::core::fmt::Debug for SpeechRecognitionVoiceCommandDefinitionConstraint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionVoiceCommandDefinitionConstraint").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpeechRecognitionVoiceCommandDefinitionConstraint {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognitionVoiceCommandDefinitionConstraint;{f2791c2b-1ef4-4ae7-9d77-b6ff10b8a3c2})");
}
unsafe impl ::windows::core::Interface for SpeechRecognitionVoiceCommandDefinitionConstraint {
    type Vtable = ISpeechRecognitionVoiceCommandDefinitionConstraintVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2791c2b_1ef4_4ae7_9d77_b6ff10b8a3c2);
}
impl ::windows::core::RuntimeName for SpeechRecognitionVoiceCommandDefinitionConstraint {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognitionVoiceCommandDefinitionConstraint";
}
impl ::core::convert::From<SpeechRecognitionVoiceCommandDefinitionConstraint> for ::windows::core::IUnknown {
    fn from(value: SpeechRecognitionVoiceCommandDefinitionConstraint) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechRecognitionVoiceCommandDefinitionConstraint> for ::windows::core::IUnknown {
    fn from(value: &SpeechRecognitionVoiceCommandDefinitionConstraint) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpeechRecognitionVoiceCommandDefinitionConstraint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SpeechRecognitionVoiceCommandDefinitionConstraint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpeechRecognitionVoiceCommandDefinitionConstraint> for ::windows::core::IInspectable {
    fn from(value: SpeechRecognitionVoiceCommandDefinitionConstraint) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechRecognitionVoiceCommandDefinitionConstraint> for ::windows::core::IInspectable {
    fn from(value: &SpeechRecognitionVoiceCommandDefinitionConstraint) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpeechRecognitionVoiceCommandDefinitionConstraint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SpeechRecognitionVoiceCommandDefinitionConstraint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SpeechRecognitionVoiceCommandDefinitionConstraint> for ISpeechRecognitionConstraint {
    type Error = ::windows::core::Error;
    fn try_from(value: SpeechRecognitionVoiceCommandDefinitionConstraint) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SpeechRecognitionVoiceCommandDefinitionConstraint> for ISpeechRecognitionConstraint {
    type Error = ::windows::core::Error;
    fn try_from(value: &SpeechRecognitionVoiceCommandDefinitionConstraint) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISpeechRecognitionConstraint> for SpeechRecognitionVoiceCommandDefinitionConstraint {
    fn into_param(self) -> ::windows::core::Param<'a, ISpeechRecognitionConstraint> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISpeechRecognitionConstraint> for &SpeechRecognitionVoiceCommandDefinitionConstraint {
    fn into_param(self) -> ::windows::core::Param<'a, ISpeechRecognitionConstraint> {
        ::core::convert::TryInto::<ISpeechRecognitionConstraint>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SpeechRecognitionVoiceCommandDefinitionConstraint {}
unsafe impl ::core::marker::Sync for SpeechRecognitionVoiceCommandDefinitionConstraint {}
#[doc = "*Required features: 'Media_SpeechRecognition'*"]
#[repr(transparent)]
pub struct SpeechRecognizer(::windows::core::IUnknown);
impl SpeechRecognizer {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpeechRecognizer, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Globalization'*"]
    #[cfg(feature = "Globalization")]
    pub fn CurrentLanguage(&self) -> ::windows::core::Result<super::super::Globalization::Language> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Globalization::Language>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Constraints(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ISpeechRecognitionConstraint>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<ISpeechRecognitionConstraint>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn Timeouts(&self) -> ::windows::core::Result<SpeechRecognizerTimeouts> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognizerTimeouts>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn UIOptions(&self) -> ::windows::core::Result<SpeechRecognizerUIOptions> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognizerUIOptions>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn CompileConstraintsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpeechRecognitionCompilationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SpeechRecognitionCompilationResult>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RecognizeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpeechRecognitionResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SpeechRecognitionResult>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RecognizeWithUIAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpeechRecognitionResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SpeechRecognitionResult>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RecognitionQualityDegrading<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SpeechRecognizer, SpeechRecognitionQualityDegradingEventArgs>>>(&self, speechrecognitionqualitydegradinghandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), speechrecognitionqualitydegradinghandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRecognitionQualityDegrading<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SpeechRecognizer, SpeechRecognizerStateChangedEventArgs>>>(&self, statechangedhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), statechangedhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn ContinuousRecognitionSession(&self) -> ::windows::core::Result<SpeechContinuousRecognitionSession> {
        let this = &::windows::core::Interface::cast::<ISpeechRecognizer2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpeechContinuousRecognitionSession>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn State(&self) -> ::windows::core::Result<SpeechRecognizerState> {
        let this = &::windows::core::Interface::cast::<ISpeechRecognizer2>(self)?;
        unsafe {
            let mut result__: SpeechRecognizerState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognizerState>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StopRecognitionAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<ISpeechRecognizer2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn HypothesisGenerated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SpeechRecognizer, SpeechRecognitionHypothesisGeneratedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ISpeechRecognizer2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveHypothesisGenerated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISpeechRecognizer2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Globalization'*"]
    #[cfg(feature = "Globalization")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::super::Globalization::Language>>(language: Param0) -> ::windows::core::Result<SpeechRecognizer> {
        Self::ISpeechRecognizerFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), language.into_param().abi(), &mut result__).from_abi::<SpeechRecognizer>(result__)
        })
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Globalization'*"]
    #[cfg(feature = "Globalization")]
    pub fn SystemSpeechLanguage() -> ::windows::core::Result<super::super::Globalization::Language> {
        Self::ISpeechRecognizerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Globalization::Language>(result__)
        })
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation_Collections', 'Globalization'*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    pub fn SupportedTopicLanguages() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Globalization::Language>> {
        Self::ISpeechRecognizerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Globalization::Language>>(result__)
        })
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation_Collections', 'Globalization'*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    pub fn SupportedGrammarLanguages() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Globalization::Language>> {
        Self::ISpeechRecognizerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Globalization::Language>>(result__)
        })
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation', 'Globalization'*"]
    #[cfg(all(feature = "Foundation", feature = "Globalization"))]
    pub fn TrySetSystemSpeechLanguageAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Globalization::Language>>(speechlanguage: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::ISpeechRecognizerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), speechlanguage.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpeechRecognizerFactory<R, F: FnOnce(&ISpeechRecognizerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpeechRecognizer, ISpeechRecognizerFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ISpeechRecognizerStatics<R, F: FnOnce(&ISpeechRecognizerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpeechRecognizer, ISpeechRecognizerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ISpeechRecognizerStatics2<R, F: FnOnce(&ISpeechRecognizerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpeechRecognizer, ISpeechRecognizerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SpeechRecognizer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpeechRecognizer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechRecognizer {}
impl ::core::fmt::Debug for SpeechRecognizer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognizer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpeechRecognizer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognizer;{0bc3c9cb-c26a-40f2-aeb5-8096b2e48073})");
}
unsafe impl ::windows::core::Interface for SpeechRecognizer {
    type Vtable = ISpeechRecognizerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0bc3c9cb_c26a_40f2_aeb5_8096b2e48073);
}
impl ::windows::core::RuntimeName for SpeechRecognizer {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognizer";
}
impl ::core::convert::From<SpeechRecognizer> for ::windows::core::IUnknown {
    fn from(value: SpeechRecognizer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechRecognizer> for ::windows::core::IUnknown {
    fn from(value: &SpeechRecognizer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpeechRecognizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SpeechRecognizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpeechRecognizer> for ::windows::core::IInspectable {
    fn from(value: SpeechRecognizer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechRecognizer> for ::windows::core::IInspectable {
    fn from(value: &SpeechRecognizer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpeechRecognizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SpeechRecognizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<SpeechRecognizer> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: SpeechRecognizer) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&SpeechRecognizer> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &SpeechRecognizer) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for SpeechRecognizer {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &SpeechRecognizer {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SpeechRecognizer {}
unsafe impl ::core::marker::Sync for SpeechRecognizer {}
#[doc = "*Required features: 'Media_SpeechRecognition'*"]
#[repr(transparent)]
pub struct SpeechRecognizerState(pub i32);
impl SpeechRecognizerState {
    pub const Idle: Self = Self(0i32);
    pub const Capturing: Self = Self(1i32);
    pub const Processing: Self = Self(2i32);
    pub const SoundStarted: Self = Self(3i32);
    pub const SoundEnded: Self = Self(4i32);
    pub const SpeechDetected: Self = Self(5i32);
    pub const Paused: Self = Self(6i32);
}
impl ::core::marker::Copy for SpeechRecognizerState {}
impl ::core::clone::Clone for SpeechRecognizerState {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SpeechRecognizerState {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SpeechRecognizerState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechRecognizerState {}
impl ::core::fmt::Debug for SpeechRecognizerState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognizerState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpeechRecognizerState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechRecognition.SpeechRecognizerState;i4)");
}
impl ::windows::core::DefaultType for SpeechRecognizerState {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_SpeechRecognition'*"]
#[repr(transparent)]
pub struct SpeechRecognizerStateChangedEventArgs(::windows::core::IUnknown);
impl SpeechRecognizerStateChangedEventArgs {
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn State(&self) -> ::windows::core::Result<SpeechRecognizerState> {
        let this = self;
        unsafe {
            let mut result__: SpeechRecognizerState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpeechRecognizerState>(result__)
        }
    }
}
impl ::core::clone::Clone for SpeechRecognizerStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpeechRecognizerStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechRecognizerStateChangedEventArgs {}
impl ::core::fmt::Debug for SpeechRecognizerStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognizerStateChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpeechRecognizerStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognizerStateChangedEventArgs;{563d4f09-ba03-4bad-ad81-ddc6c4dab0c3})");
}
unsafe impl ::windows::core::Interface for SpeechRecognizerStateChangedEventArgs {
    type Vtable = ISpeechRecognizerStateChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x563d4f09_ba03_4bad_ad81_ddc6c4dab0c3);
}
impl ::windows::core::RuntimeName for SpeechRecognizerStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognizerStateChangedEventArgs";
}
impl ::core::convert::From<SpeechRecognizerStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: SpeechRecognizerStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechRecognizerStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SpeechRecognizerStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpeechRecognizerStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SpeechRecognizerStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpeechRecognizerStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: SpeechRecognizerStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechRecognizerStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SpeechRecognizerStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpeechRecognizerStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SpeechRecognizerStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpeechRecognizerStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for SpeechRecognizerStateChangedEventArgs {}
#[doc = "*Required features: 'Media_SpeechRecognition'*"]
#[repr(transparent)]
pub struct SpeechRecognizerTimeouts(::windows::core::IUnknown);
impl SpeechRecognizerTimeouts {
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn InitialSilenceTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetInitialSilenceTimeout<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn EndSilenceTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetEndSilenceTimeout<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn BabbleTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBabbleTimeout<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for SpeechRecognizerTimeouts {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpeechRecognizerTimeouts {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechRecognizerTimeouts {}
impl ::core::fmt::Debug for SpeechRecognizerTimeouts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognizerTimeouts").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpeechRecognizerTimeouts {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognizerTimeouts;{2ef76fca-6a3c-4dca-a153-df1bc88a79af})");
}
unsafe impl ::windows::core::Interface for SpeechRecognizerTimeouts {
    type Vtable = ISpeechRecognizerTimeoutsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ef76fca_6a3c_4dca_a153_df1bc88a79af);
}
impl ::windows::core::RuntimeName for SpeechRecognizerTimeouts {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognizerTimeouts";
}
impl ::core::convert::From<SpeechRecognizerTimeouts> for ::windows::core::IUnknown {
    fn from(value: SpeechRecognizerTimeouts) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechRecognizerTimeouts> for ::windows::core::IUnknown {
    fn from(value: &SpeechRecognizerTimeouts) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpeechRecognizerTimeouts {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SpeechRecognizerTimeouts {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpeechRecognizerTimeouts> for ::windows::core::IInspectable {
    fn from(value: SpeechRecognizerTimeouts) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechRecognizerTimeouts> for ::windows::core::IInspectable {
    fn from(value: &SpeechRecognizerTimeouts) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpeechRecognizerTimeouts {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SpeechRecognizerTimeouts {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpeechRecognizerTimeouts {}
unsafe impl ::core::marker::Sync for SpeechRecognizerTimeouts {}
#[doc = "*Required features: 'Media_SpeechRecognition'*"]
#[repr(transparent)]
pub struct SpeechRecognizerUIOptions(::windows::core::IUnknown);
impl SpeechRecognizerUIOptions {
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn ExampleText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn SetExampleText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn AudiblePrompt(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn SetAudiblePrompt<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn IsReadBackEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn SetIsReadBackEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn ShowConfirmation(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn SetShowConfirmation(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for SpeechRecognizerUIOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpeechRecognizerUIOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechRecognizerUIOptions {}
impl ::core::fmt::Debug for SpeechRecognizerUIOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognizerUIOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpeechRecognizerUIOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.SpeechRecognizerUIOptions;{7888d641-b92b-44ba-a25f-d1864630641f})");
}
unsafe impl ::windows::core::Interface for SpeechRecognizerUIOptions {
    type Vtable = ISpeechRecognizerUIOptionsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7888d641_b92b_44ba_a25f_d1864630641f);
}
impl ::windows::core::RuntimeName for SpeechRecognizerUIOptions {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.SpeechRecognizerUIOptions";
}
impl ::core::convert::From<SpeechRecognizerUIOptions> for ::windows::core::IUnknown {
    fn from(value: SpeechRecognizerUIOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechRecognizerUIOptions> for ::windows::core::IUnknown {
    fn from(value: &SpeechRecognizerUIOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpeechRecognizerUIOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SpeechRecognizerUIOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpeechRecognizerUIOptions> for ::windows::core::IInspectable {
    fn from(value: SpeechRecognizerUIOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpeechRecognizerUIOptions> for ::windows::core::IInspectable {
    fn from(value: &SpeechRecognizerUIOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpeechRecognizerUIOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SpeechRecognizerUIOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpeechRecognizerUIOptions {}
unsafe impl ::core::marker::Sync for SpeechRecognizerUIOptions {}
#[doc = "*Required features: 'Media_SpeechRecognition'*"]
pub struct VoiceCommandManager {}
impl VoiceCommandManager {
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation', 'Storage'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn InstallCommandSetsFromStorageFileAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::StorageFile>>(file: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IVoiceCommandManager(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InstalledCommandSets() -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, VoiceCommandSet>> {
        Self::IVoiceCommandManager(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, VoiceCommandSet>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVoiceCommandManager<R, F: FnOnce(&IVoiceCommandManager) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VoiceCommandManager, IVoiceCommandManager> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for VoiceCommandManager {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.VoiceCommandManager";
}
#[doc = "*Required features: 'Media_SpeechRecognition'*"]
#[repr(transparent)]
pub struct VoiceCommandSet(::windows::core::IUnknown);
impl VoiceCommandSet {
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition'*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_SpeechRecognition', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn SetPhraseListAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, phraselistname: Param0, phraselist: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), phraselistname.into_param().abi(), phraselist.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for VoiceCommandSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VoiceCommandSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VoiceCommandSet {}
impl ::core::fmt::Debug for VoiceCommandSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoiceCommandSet").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VoiceCommandSet {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechRecognition.VoiceCommandSet;{0bedda75-46e6-4b11-a088-5c68632899b5})");
}
unsafe impl ::windows::core::Interface for VoiceCommandSet {
    type Vtable = IVoiceCommandSetVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0bedda75_46e6_4b11_a088_5c68632899b5);
}
impl ::windows::core::RuntimeName for VoiceCommandSet {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.VoiceCommandSet";
}
impl ::core::convert::From<VoiceCommandSet> for ::windows::core::IUnknown {
    fn from(value: VoiceCommandSet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VoiceCommandSet> for ::windows::core::IUnknown {
    fn from(value: &VoiceCommandSet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VoiceCommandSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &VoiceCommandSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VoiceCommandSet> for ::windows::core::IInspectable {
    fn from(value: VoiceCommandSet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VoiceCommandSet> for ::windows::core::IInspectable {
    fn from(value: &VoiceCommandSet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VoiceCommandSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &VoiceCommandSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for VoiceCommandSet {}
unsafe impl ::core::marker::Sync for VoiceCommandSet {}
