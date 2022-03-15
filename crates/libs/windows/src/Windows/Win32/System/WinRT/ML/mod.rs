#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_System_WinRT_ML\"`*"]
#[repr(transparent)]
pub struct ILearningModelDeviceFactoryNative(::windows::core::IUnknown);
impl ILearningModelDeviceFactoryNative {
    #[doc = "*Required features: `\"Win32_System_WinRT_ML\"`, `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CreateFromD3D12CommandQueue<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>>(&self, value: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateFromD3D12CommandQueue)(::core::mem::transmute_copy(self), value.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
}
impl ::core::convert::From<ILearningModelDeviceFactoryNative> for ::windows::core::IUnknown {
    fn from(value: ILearningModelDeviceFactoryNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILearningModelDeviceFactoryNative> for ::windows::core::IUnknown {
    fn from(value: &ILearningModelDeviceFactoryNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILearningModelDeviceFactoryNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ILearningModelDeviceFactoryNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ILearningModelDeviceFactoryNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILearningModelDeviceFactoryNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILearningModelDeviceFactoryNative {}
impl ::core::fmt::Debug for ILearningModelDeviceFactoryNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILearningModelDeviceFactoryNative").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ILearningModelDeviceFactoryNative {
    type Vtable = ILearningModelDeviceFactoryNative_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e9b31a1_662e_4ae0_af67_f63bb337e634);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelDeviceFactoryNative_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CreateFromD3D12CommandQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CreateFromD3D12CommandQueue: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT_ML\"`*"]
#[repr(transparent)]
pub struct ILearningModelOperatorProviderNative(::windows::core::IUnknown);
impl ILearningModelOperatorProviderNative {
    #[doc = "*Required features: `\"Win32_System_WinRT_ML\"`, `\"Win32_AI_MachineLearning_WinML\"`*"]
    #[cfg(feature = "Win32_AI_MachineLearning_WinML")]
    pub unsafe fn GetRegistry(&self) -> ::windows::core::Result<super::super::super::AI::MachineLearning::WinML::IMLOperatorRegistry> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRegistry)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::AI::MachineLearning::WinML::IMLOperatorRegistry>(result__)
    }
}
impl ::core::convert::From<ILearningModelOperatorProviderNative> for ::windows::core::IUnknown {
    fn from(value: ILearningModelOperatorProviderNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILearningModelOperatorProviderNative> for ::windows::core::IUnknown {
    fn from(value: &ILearningModelOperatorProviderNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILearningModelOperatorProviderNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ILearningModelOperatorProviderNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ILearningModelOperatorProviderNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILearningModelOperatorProviderNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILearningModelOperatorProviderNative {}
impl ::core::fmt::Debug for ILearningModelOperatorProviderNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILearningModelOperatorProviderNative").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ILearningModelOperatorProviderNative {
    type Vtable = ILearningModelOperatorProviderNative_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1adaa23a_eb67_41f3_aad8_5d984e9bacd4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelOperatorProviderNative_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_AI_MachineLearning_WinML")]
    pub GetRegistry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppoperatorregistry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_AI_MachineLearning_WinML"))]
    GetRegistry: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT_ML\"`*"]
#[repr(transparent)]
pub struct ILearningModelSessionOptionsNative(::windows::core::IUnknown);
impl ILearningModelSessionOptionsNative {
    #[doc = "*Required features: `\"Win32_System_WinRT_ML\"`*"]
    pub unsafe fn SetIntraOpNumThreadsOverride(&self, intraopnumthreads: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetIntraOpNumThreadsOverride)(::core::mem::transmute_copy(self), ::core::mem::transmute(intraopnumthreads)).ok()
    }
}
impl ::core::convert::From<ILearningModelSessionOptionsNative> for ::windows::core::IUnknown {
    fn from(value: ILearningModelSessionOptionsNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILearningModelSessionOptionsNative> for ::windows::core::IUnknown {
    fn from(value: &ILearningModelSessionOptionsNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILearningModelSessionOptionsNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ILearningModelSessionOptionsNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ILearningModelSessionOptionsNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILearningModelSessionOptionsNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILearningModelSessionOptionsNative {}
impl ::core::fmt::Debug for ILearningModelSessionOptionsNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILearningModelSessionOptionsNative").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ILearningModelSessionOptionsNative {
    type Vtable = ILearningModelSessionOptionsNative_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc71e953f_37b4_4564_8658_d8396866db0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSessionOptionsNative_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetIntraOpNumThreadsOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, intraopnumthreads: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_ML\"`*"]
#[repr(transparent)]
pub struct ITensorNative(::windows::core::IUnknown);
impl ITensorNative {
    #[doc = "*Required features: `\"Win32_System_WinRT_ML\"`*"]
    pub unsafe fn GetBuffer(&self, value: *mut *mut u8, capacity: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(value), ::core::mem::transmute(capacity)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_WinRT_ML\"`, `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetD3D12Resource(&self) -> ::windows::core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetD3D12Resource)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Graphics::Direct3D12::ID3D12Resource>(result__)
    }
}
impl ::core::convert::From<ITensorNative> for ::windows::core::IUnknown {
    fn from(value: ITensorNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITensorNative> for ::windows::core::IUnknown {
    fn from(value: &ITensorNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITensorNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITensorNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITensorNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITensorNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITensorNative {}
impl ::core::fmt::Debug for ITensorNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITensorNative").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITensorNative {
    type Vtable = ITensorNative_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52f547ef_5b03_49b5_82d6_565f1ee0dd49);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorNative_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut u8, capacity: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub GetD3D12Resource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    GetD3D12Resource: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT_ML\"`*"]
#[repr(transparent)]
pub struct ITensorStaticsNative(::windows::core::IUnknown);
impl ITensorStaticsNative {
    #[doc = "*Required features: `\"Win32_System_WinRT_ML\"`, `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CreateFromD3D12Resource<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Resource>>(&self, value: Param0, shape: *mut i64, shapecount: i32, result: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateFromD3D12Resource)(::core::mem::transmute_copy(self), value.into_param().abi(), ::core::mem::transmute(shape), ::core::mem::transmute(shapecount), ::core::mem::transmute(result)).ok()
    }
}
impl ::core::convert::From<ITensorStaticsNative> for ::windows::core::IUnknown {
    fn from(value: ITensorStaticsNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITensorStaticsNative> for ::windows::core::IUnknown {
    fn from(value: &ITensorStaticsNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITensorStaticsNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITensorStaticsNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITensorStaticsNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITensorStaticsNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITensorStaticsNative {}
impl ::core::fmt::Debug for ITensorStaticsNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITensorStaticsNative").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITensorStaticsNative {
    type Vtable = ITensorStaticsNative_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39d055a4_66f6_4ebc_95d9_7a29ebe7690a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorStaticsNative_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CreateFromD3D12Resource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, shape: *mut i64, shapecount: i32, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CreateFromD3D12Resource: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
