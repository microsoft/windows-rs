#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ILearningModelDeviceFactoryNative(pub ::windows::core::IUnknown);
impl ILearningModelDeviceFactoryNative {
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CreateFromD3D12CommandQueue<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>>(&self, value: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), value.into_param().abi(), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
}
unsafe impl ::windows::core::Interface for ILearningModelDeviceFactoryNative {
    type Vtable = ILearningModelDeviceFactoryNative_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e9b31a1_662e_4ae0_af67_f63bb337e634);
}
impl ::core::convert::From<ILearningModelDeviceFactoryNative> for ::windows::core::IUnknown {
    fn from(value: ILearningModelDeviceFactoryNative) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ILearningModelDeviceFactoryNative> for ::windows::core::IUnknown {
    fn from(value: &ILearningModelDeviceFactoryNative) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILearningModelDeviceFactoryNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ILearningModelDeviceFactoryNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelDeviceFactoryNative_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ILearningModelOperatorProviderNative(pub ::windows::core::IUnknown);
impl ILearningModelOperatorProviderNative {
    #[cfg(feature = "Win32_AI_MachineLearning_WinML")]
    pub unsafe fn GetRegistry(&self) -> ::windows::core::Result<super::super::super::AI::MachineLearning::WinML::IMLOperatorRegistry> {
        let mut result__: <super::super::super::AI::MachineLearning::WinML::IMLOperatorRegistry as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::AI::MachineLearning::WinML::IMLOperatorRegistry>(result__)
    }
}
unsafe impl ::windows::core::Interface for ILearningModelOperatorProviderNative {
    type Vtable = ILearningModelOperatorProviderNative_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1adaa23a_eb67_41f3_aad8_5d984e9bacd4);
}
impl ::core::convert::From<ILearningModelOperatorProviderNative> for ::windows::core::IUnknown {
    fn from(value: ILearningModelOperatorProviderNative) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ILearningModelOperatorProviderNative> for ::windows::core::IUnknown {
    fn from(value: &ILearningModelOperatorProviderNative) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILearningModelOperatorProviderNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ILearningModelOperatorProviderNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelOperatorProviderNative_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_AI_MachineLearning_WinML")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppoperatorregistry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_AI_MachineLearning_WinML"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ILearningModelSessionOptionsNative(pub ::windows::core::IUnknown);
impl ILearningModelSessionOptionsNative {
    pub unsafe fn SetIntraOpNumThreadsOverride(&self, intraopnumthreads: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(intraopnumthreads)).ok()
    }
}
unsafe impl ::windows::core::Interface for ILearningModelSessionOptionsNative {
    type Vtable = ILearningModelSessionOptionsNative_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc71e953f_37b4_4564_8658_d8396866db0d);
}
impl ::core::convert::From<ILearningModelSessionOptionsNative> for ::windows::core::IUnknown {
    fn from(value: ILearningModelSessionOptionsNative) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ILearningModelSessionOptionsNative> for ::windows::core::IUnknown {
    fn from(value: &ILearningModelSessionOptionsNative) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILearningModelSessionOptionsNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ILearningModelSessionOptionsNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSessionOptionsNative_abi(pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr, intraopnumthreads: u32) -> ::windows::core::HRESULT);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITensorNative(pub ::windows::core::IUnknown);
impl ITensorNative {
    pub unsafe fn GetBuffer(&self, value: *mut *mut u8, capacity: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(value), ::core::mem::transmute(capacity)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetD3D12Resource(&self) -> ::windows::core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource> {
        let mut result__: <super::super::super::Graphics::Direct3D12::ID3D12Resource as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Graphics::Direct3D12::ID3D12Resource>(result__)
    }
}
unsafe impl ::windows::core::Interface for ITensorNative {
    type Vtable = ITensorNative_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52f547ef_5b03_49b5_82d6_565f1ee0dd49);
}
impl ::core::convert::From<ITensorNative> for ::windows::core::IUnknown {
    fn from(value: ITensorNative) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITensorNative> for ::windows::core::IUnknown {
    fn from(value: &ITensorNative) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITensorNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITensorNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorNative_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut *mut u8, capacity: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITensorStaticsNative(pub ::windows::core::IUnknown);
impl ITensorStaticsNative {
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CreateFromD3D12Resource<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Resource>>(&self, value: Param0, shape: *mut i64, shapecount: i32, result: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), value.into_param().abi(), ::core::mem::transmute(shape), ::core::mem::transmute(shapecount), ::core::mem::transmute(result)).ok()
    }
}
unsafe impl ::windows::core::Interface for ITensorStaticsNative {
    type Vtable = ITensorStaticsNative_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39d055a4_66f6_4ebc_95d9_7a29ebe7690a);
}
impl ::core::convert::From<ITensorStaticsNative> for ::windows::core::IUnknown {
    fn from(value: ITensorStaticsNative) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITensorStaticsNative> for ::windows::core::IUnknown {
    fn from(value: &ITensorStaticsNative) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITensorStaticsNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITensorStaticsNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorStaticsNative_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, shape: *mut i64, shapecount: i32, result: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
);
