#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILearningModelDeviceFactoryNative(::windows_core::IUnknown);
impl ILearningModelDeviceFactoryNative {
    #[doc = "Required features: `\"Win32_Graphics_Direct3D12\"`"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CreateFromD3D12CommandQueue<P0>(&self, value: P0) -> ::windows_core::Result<::windows_core::IUnknown>
    where
        P0: ::windows_core::IntoParam<super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateFromD3D12CommandQueue)(::windows_core::Interface::as_raw(self), value.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(ILearningModelDeviceFactoryNative, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILearningModelDeviceFactoryNative {
    type Vtable = ILearningModelDeviceFactoryNative_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILearningModelDeviceFactoryNative {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1e9b31a1_662e_4ae0_af67_f63bb337e634);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelDeviceFactoryNative_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CreateFromD3D12CommandQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CreateFromD3D12CommandQueue: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILearningModelOperatorProviderNative(::windows_core::IUnknown);
impl ILearningModelOperatorProviderNative {
    #[doc = "Required features: `\"Win32_AI_MachineLearning_WinML\"`"]
    #[cfg(feature = "Win32_AI_MachineLearning_WinML")]
    pub unsafe fn GetRegistry(&self) -> ::windows_core::Result<super::super::super::AI::MachineLearning::WinML::IMLOperatorRegistry> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRegistry)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(ILearningModelOperatorProviderNative, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILearningModelOperatorProviderNative {
    type Vtable = ILearningModelOperatorProviderNative_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILearningModelOperatorProviderNative {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1adaa23a_eb67_41f3_aad8_5d984e9bacd4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelOperatorProviderNative_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_AI_MachineLearning_WinML")]
    pub GetRegistry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppoperatorregistry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_AI_MachineLearning_WinML"))]
    GetRegistry: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILearningModelSessionOptionsNative(::windows_core::IUnknown);
impl ILearningModelSessionOptionsNative {
    pub unsafe fn SetIntraOpNumThreadsOverride(&self, intraopnumthreads: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIntraOpNumThreadsOverride)(::windows_core::Interface::as_raw(self), intraopnumthreads).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ILearningModelSessionOptionsNative, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILearningModelSessionOptionsNative {
    type Vtable = ILearningModelSessionOptionsNative_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILearningModelSessionOptionsNative {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc71e953f_37b4_4564_8658_d8396866db0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSessionOptionsNative_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetIntraOpNumThreadsOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, intraopnumthreads: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILearningModelSessionOptionsNative1(::windows_core::IUnknown);
impl ILearningModelSessionOptionsNative1 {
    pub unsafe fn SetIntraOpThreadSpinning(&self, allowspinning: u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIntraOpThreadSpinning)(::windows_core::Interface::as_raw(self), allowspinning).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ILearningModelSessionOptionsNative1, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILearningModelSessionOptionsNative1 {
    type Vtable = ILearningModelSessionOptionsNative1_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILearningModelSessionOptionsNative1 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5da37a26_0526_414b_91e4_2a0fa3ddba40);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSessionOptionsNative1_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetIntraOpThreadSpinning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allowspinning: u8) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorNative(::windows_core::IUnknown);
impl ITensorNative {
    pub unsafe fn GetBuffer(&self, value: *mut *mut u8, capacity: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetBuffer)(::windows_core::Interface::as_raw(self), value, capacity).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Direct3D12\"`"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetD3D12Resource(&self) -> ::windows_core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetD3D12Resource)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(ITensorNative, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorNative {
    type Vtable = ITensorNative_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorNative {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x52f547ef_5b03_49b5_82d6_565f1ee0dd49);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorNative_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut u8, capacity: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub GetD3D12Resource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    GetD3D12Resource: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorStaticsNative(::windows_core::IUnknown);
impl ITensorStaticsNative {
    #[doc = "Required features: `\"Win32_Graphics_Direct3D12\"`"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CreateFromD3D12Resource<P0>(&self, value: P0, shape: *mut i64, shapecount: i32, result: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
    {
        (::windows_core::Interface::vtable(self).CreateFromD3D12Resource)(::windows_core::Interface::as_raw(self), value.into_param().abi(), shape, shapecount, ::core::mem::transmute(result)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ITensorStaticsNative, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorStaticsNative {
    type Vtable = ITensorStaticsNative_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorStaticsNative {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x39d055a4_66f6_4ebc_95d9_7a29ebe7690a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorStaticsNative_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CreateFromD3D12Resource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, shape: *mut i64, shapecount: i32, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CreateFromD3D12Resource: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
