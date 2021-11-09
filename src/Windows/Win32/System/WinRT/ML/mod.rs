#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_WinRT_ML`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ILearningModelDeviceFactoryNative(pub ::windows::runtime::IUnknown);
impl ILearningModelDeviceFactoryNative {
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_System_WinRT_ML`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn CreateFromD3D12CommandQueue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>>(&self, value: Param0) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), value.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ILearningModelDeviceFactoryNative {
    type Vtable = ILearningModelDeviceFactoryNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(513487265, 26158, 19168, [175, 103, 246, 59, 179, 55, 230, 52]);
}
impl ::core::convert::From<ILearningModelDeviceFactoryNative> for ::windows::runtime::IUnknown {
    fn from(value: ILearningModelDeviceFactoryNative) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ILearningModelDeviceFactoryNative> for ::windows::runtime::IUnknown {
    fn from(value: &ILearningModelDeviceFactoryNative) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ILearningModelDeviceFactoryNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ILearningModelDeviceFactoryNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelDeviceFactoryNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT_ML`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ILearningModelOperatorProviderNative(pub ::windows::runtime::IUnknown);
impl ILearningModelOperatorProviderNative {
    #[cfg(feature = "Win32_AI_MachineLearning_WinML")]
    #[doc = "*Required features: `Win32_System_WinRT_ML`, `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn GetRegistry(&self) -> ::windows::runtime::Result<super::super::super::AI::MachineLearning::WinML::IMLOperatorRegistry> {
        let mut result__: <super::super::super::AI::MachineLearning::WinML::IMLOperatorRegistry as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::AI::MachineLearning::WinML::IMLOperatorRegistry>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ILearningModelOperatorProviderNative {
    type Vtable = ILearningModelOperatorProviderNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(450535994, 60263, 16883, [170, 216, 93, 152, 78, 155, 172, 212]);
}
impl ::core::convert::From<ILearningModelOperatorProviderNative> for ::windows::runtime::IUnknown {
    fn from(value: ILearningModelOperatorProviderNative) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ILearningModelOperatorProviderNative> for ::windows::runtime::IUnknown {
    fn from(value: &ILearningModelOperatorProviderNative) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ILearningModelOperatorProviderNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ILearningModelOperatorProviderNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelOperatorProviderNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_AI_MachineLearning_WinML")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppoperatorregistry: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_AI_MachineLearning_WinML"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT_ML`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ILearningModelSessionOptionsNative(pub ::windows::runtime::IUnknown);
impl ILearningModelSessionOptionsNative {
    #[doc = "*Required features: `Win32_System_WinRT_ML`*"]
    pub unsafe fn SetIntraOpNumThreadsOverride(&self, intraopnumthreads: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(intraopnumthreads)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ILearningModelSessionOptionsNative {
    type Vtable = ILearningModelSessionOptionsNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3340670271, 14260, 17764, [134, 88, 216, 57, 104, 102, 219, 13]);
}
impl ::core::convert::From<ILearningModelSessionOptionsNative> for ::windows::runtime::IUnknown {
    fn from(value: ILearningModelSessionOptionsNative) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ILearningModelSessionOptionsNative> for ::windows::runtime::IUnknown {
    fn from(value: &ILearningModelSessionOptionsNative) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ILearningModelSessionOptionsNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ILearningModelSessionOptionsNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSessionOptionsNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, intraopnumthreads: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT_ML`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITensorNative(pub ::windows::runtime::IUnknown);
impl ITensorNative {
    #[doc = "*Required features: `Win32_System_WinRT_ML`*"]
    pub unsafe fn GetBuffer(&self, value: *mut *mut u8, capacity: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(value), ::core::mem::transmute(capacity)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_System_WinRT_ML`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn GetD3D12Resource(&self) -> ::windows::runtime::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource> {
        let mut result__: <super::super::super::Graphics::Direct3D12::ID3D12Resource as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Graphics::Direct3D12::ID3D12Resource>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITensorNative {
    type Vtable = ITensorNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1391806447, 23299, 18869, [130, 214, 86, 95, 30, 224, 221, 73]);
}
impl ::core::convert::From<ITensorNative> for ::windows::runtime::IUnknown {
    fn from(value: ITensorNative) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITensorNative> for ::windows::runtime::IUnknown {
    fn from(value: &ITensorNative) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITensorNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITensorNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut *mut u8, capacity: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT_ML`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITensorStaticsNative(pub ::windows::runtime::IUnknown);
impl ITensorStaticsNative {
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_System_WinRT_ML`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn CreateFromD3D12Resource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Resource>>(&self, value: Param0, shape: *mut i64, shapecount: i32, result: *mut ::core::option::Option<::windows::runtime::IUnknown>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), value.into_param().abi(), ::core::mem::transmute(shape), ::core::mem::transmute(shapecount), ::core::mem::transmute(result)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITensorStaticsNative {
    type Vtable = ITensorStaticsNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(969954724, 26358, 20156, [149, 217, 122, 41, 235, 231, 105, 10]);
}
impl ::core::convert::From<ITensorStaticsNative> for ::windows::runtime::IUnknown {
    fn from(value: ITensorStaticsNative) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITensorStaticsNative> for ::windows::runtime::IUnknown {
    fn from(value: &ITensorStaticsNative) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITensorStaticsNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITensorStaticsNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorStaticsNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, shape: *mut i64, shapecount: i32, result: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
);
