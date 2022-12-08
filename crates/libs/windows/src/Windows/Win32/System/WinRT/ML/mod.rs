#[doc = "*Required features: `\"Win32_System_WinRT_ML\"`*"]
#[repr(transparent)]
pub struct ILearningModelDeviceFactoryNative(::windows::core::IUnknown);
impl ILearningModelDeviceFactoryNative {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CreateFromD3D12CommandQueue<P0>(&self, value: P0) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateFromD3D12CommandQueue)(::windows::core::Vtable::as_raw(self), value.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ILearningModelDeviceFactoryNative, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Vtable for ILearningModelDeviceFactoryNative {
    type Vtable = ILearningModelDeviceFactoryNative_Vtbl;
}
unsafe impl ::windows::core::Interface for ILearningModelDeviceFactoryNative {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e9b31a1_662e_4ae0_af67_f63bb337e634);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelDeviceFactoryNative_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CreateFromD3D12CommandQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CreateFromD3D12CommandQueue: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT_ML\"`*"]
#[repr(transparent)]
pub struct ILearningModelOperatorProviderNative(::windows::core::IUnknown);
impl ILearningModelOperatorProviderNative {
    #[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
    #[cfg(feature = "Win32_AI_MachineLearning_WinML")]
    pub unsafe fn GetRegistry(&self) -> ::windows::core::Result<super::super::super::AI::MachineLearning::WinML::IMLOperatorRegistry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetRegistry)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ILearningModelOperatorProviderNative, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Vtable for ILearningModelOperatorProviderNative {
    type Vtable = ILearningModelOperatorProviderNative_Vtbl;
}
unsafe impl ::windows::core::Interface for ILearningModelOperatorProviderNative {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1adaa23a_eb67_41f3_aad8_5d984e9bacd4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelOperatorProviderNative_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_AI_MachineLearning_WinML")]
    pub GetRegistry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppoperatorregistry: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_AI_MachineLearning_WinML"))]
    GetRegistry: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT_ML\"`*"]
#[repr(transparent)]
pub struct ILearningModelSessionOptionsNative(::windows::core::IUnknown);
impl ILearningModelSessionOptionsNative {
    pub unsafe fn SetIntraOpNumThreadsOverride(&self, intraopnumthreads: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetIntraOpNumThreadsOverride)(::windows::core::Vtable::as_raw(self), intraopnumthreads).ok()
    }
}
::windows::core::interface_hierarchy!(ILearningModelSessionOptionsNative, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Vtable for ILearningModelSessionOptionsNative {
    type Vtable = ILearningModelSessionOptionsNative_Vtbl;
}
unsafe impl ::windows::core::Interface for ILearningModelSessionOptionsNative {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc71e953f_37b4_4564_8658_d8396866db0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSessionOptionsNative_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetIntraOpNumThreadsOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, intraopnumthreads: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_ML\"`*"]
#[repr(transparent)]
pub struct ITensorNative(::windows::core::IUnknown);
impl ITensorNative {
    pub unsafe fn GetBuffer(&self, value: *mut *mut u8, capacity: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetBuffer)(::windows::core::Vtable::as_raw(self), value, capacity).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetD3D12Resource(&self) -> ::windows::core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetD3D12Resource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ITensorNative, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Vtable for ITensorNative {
    type Vtable = ITensorNative_Vtbl;
}
unsafe impl ::windows::core::Interface for ITensorNative {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52f547ef_5b03_49b5_82d6_565f1ee0dd49);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorNative_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut u8, capacity: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub GetD3D12Resource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    GetD3D12Resource: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT_ML\"`*"]
#[repr(transparent)]
pub struct ITensorStaticsNative(::windows::core::IUnknown);
impl ITensorStaticsNative {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CreateFromD3D12Resource<P0>(&self, value: P0, shape: *mut i64, shapecount: i32, result: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::Graphics::Direct3D12::ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).CreateFromD3D12Resource)(::windows::core::Vtable::as_raw(self), value.into().abi(), shape, shapecount, ::core::mem::transmute(result)).ok()
    }
}
::windows::core::interface_hierarchy!(ITensorStaticsNative, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Vtable for ITensorStaticsNative {
    type Vtable = ITensorStaticsNative_Vtbl;
}
unsafe impl ::windows::core::Interface for ITensorStaticsNative {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39d055a4_66f6_4ebc_95d9_7a29ebe7690a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorStaticsNative_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CreateFromD3D12Resource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, shape: *mut i64, shapecount: i32, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CreateFromD3D12Resource: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
