#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Graphics_Direct3D12`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
#[inline]
pub unsafe fn DMLCreateDevice<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Device>, T: ::windows::runtime::Interface>(d3d12device: Param0, flags: DML_CREATE_DEVICE_FLAGS, result__: *mut ::core::option::Option<T>) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DMLCreateDevice(d3d12device: ::windows::runtime::RawPtr, flags: DML_CREATE_DEVICE_FLAGS, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        DMLCreateDevice(d3d12device.into_param().abi(), ::core::mem::transmute(flags), &<T as ::windows::runtime::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Graphics_Direct3D12`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
#[inline]
pub unsafe fn DMLCreateDevice1<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Device>, T: ::windows::runtime::Interface>(d3d12device: Param0, flags: DML_CREATE_DEVICE_FLAGS, minimumfeaturelevel: DML_FEATURE_LEVEL, result__: *mut ::core::option::Option<T>) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DMLCreateDevice1(d3d12device: ::windows::runtime::RawPtr, flags: DML_CREATE_DEVICE_FLAGS, minimumfeaturelevel: DML_FEATURE_LEVEL, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        DMLCreateDevice1(d3d12device.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(minimumfeaturelevel), &<T as ::windows::runtime::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ACTIVATION_CELU_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Alpha: f32,
}
impl DML_ACTIVATION_CELU_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_CELU_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_CELU_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ACTIVATION_CELU_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Alpha", &self.Alpha).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_CELU_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Alpha == other.Alpha
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_CELU_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ACTIVATION_CELU_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ACTIVATION_ELU_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Alpha: f32,
}
impl DML_ACTIVATION_ELU_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_ELU_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_ELU_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ACTIVATION_ELU_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Alpha", &self.Alpha).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_ELU_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Alpha == other.Alpha
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_ELU_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ACTIVATION_ELU_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ACTIVATION_HARDMAX_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ACTIVATION_HARDMAX_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_HARDMAX_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_HARDMAX_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ACTIVATION_HARDMAX_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_HARDMAX_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_HARDMAX_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ACTIVATION_HARDMAX_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ACTIVATION_HARD_SIGMOID_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Alpha: f32,
    pub Beta: f32,
}
impl DML_ACTIVATION_HARD_SIGMOID_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_HARD_SIGMOID_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_HARD_SIGMOID_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ACTIVATION_HARD_SIGMOID_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Alpha", &self.Alpha).field("Beta", &self.Beta).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_HARD_SIGMOID_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Alpha == other.Alpha && self.Beta == other.Beta
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_HARD_SIGMOID_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ACTIVATION_HARD_SIGMOID_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ACTIVATION_IDENTITY_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ACTIVATION_IDENTITY_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_IDENTITY_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_IDENTITY_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ACTIVATION_IDENTITY_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_IDENTITY_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_IDENTITY_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ACTIVATION_IDENTITY_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ACTIVATION_LEAKY_RELU_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Alpha: f32,
}
impl DML_ACTIVATION_LEAKY_RELU_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_LEAKY_RELU_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_LEAKY_RELU_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ACTIVATION_LEAKY_RELU_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Alpha", &self.Alpha).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_LEAKY_RELU_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Alpha == other.Alpha
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_LEAKY_RELU_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ACTIVATION_LEAKY_RELU_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ACTIVATION_LINEAR_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Alpha: f32,
    pub Beta: f32,
}
impl DML_ACTIVATION_LINEAR_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_LINEAR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_LINEAR_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ACTIVATION_LINEAR_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Alpha", &self.Alpha).field("Beta", &self.Beta).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_LINEAR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Alpha == other.Alpha && self.Beta == other.Beta
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_LINEAR_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ACTIVATION_LINEAR_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ACTIVATION_LOG_SOFTMAX_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ACTIVATION_LOG_SOFTMAX_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_LOG_SOFTMAX_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_LOG_SOFTMAX_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ACTIVATION_LOG_SOFTMAX_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_LOG_SOFTMAX_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_LOG_SOFTMAX_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ACTIVATION_LOG_SOFTMAX_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ACTIVATION_PARAMETERIZED_RELU_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub SlopeTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ACTIVATION_PARAMETERIZED_RELU_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_PARAMETERIZED_RELU_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_PARAMETERIZED_RELU_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ACTIVATION_PARAMETERIZED_RELU_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("SlopeTensor", &self.SlopeTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_PARAMETERIZED_RELU_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.SlopeTensor == other.SlopeTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_PARAMETERIZED_RELU_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ACTIVATION_PARAMETERIZED_RELU_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ACTIVATION_PARAMETRIC_SOFTPLUS_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Alpha: f32,
    pub Beta: f32,
}
impl DML_ACTIVATION_PARAMETRIC_SOFTPLUS_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_PARAMETRIC_SOFTPLUS_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_PARAMETRIC_SOFTPLUS_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ACTIVATION_PARAMETRIC_SOFTPLUS_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Alpha", &self.Alpha).field("Beta", &self.Beta).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_PARAMETRIC_SOFTPLUS_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Alpha == other.Alpha && self.Beta == other.Beta
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_PARAMETRIC_SOFTPLUS_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ACTIVATION_PARAMETRIC_SOFTPLUS_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ACTIVATION_RELU_GRAD_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub InputGradientTensor: *mut DML_TENSOR_DESC,
    pub OutputGradientTensor: *mut DML_TENSOR_DESC,
}
impl DML_ACTIVATION_RELU_GRAD_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_RELU_GRAD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_RELU_GRAD_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ACTIVATION_RELU_GRAD_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("InputGradientTensor", &self.InputGradientTensor).field("OutputGradientTensor", &self.OutputGradientTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_RELU_GRAD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.InputGradientTensor == other.InputGradientTensor && self.OutputGradientTensor == other.OutputGradientTensor
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_RELU_GRAD_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ACTIVATION_RELU_GRAD_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ACTIVATION_RELU_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ACTIVATION_RELU_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_RELU_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_RELU_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ACTIVATION_RELU_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_RELU_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_RELU_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ACTIVATION_RELU_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ACTIVATION_SCALED_ELU_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Alpha: f32,
    pub Gamma: f32,
}
impl DML_ACTIVATION_SCALED_ELU_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_SCALED_ELU_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_SCALED_ELU_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ACTIVATION_SCALED_ELU_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Alpha", &self.Alpha).field("Gamma", &self.Gamma).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_SCALED_ELU_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Alpha == other.Alpha && self.Gamma == other.Gamma
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_SCALED_ELU_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ACTIVATION_SCALED_ELU_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ACTIVATION_SCALED_TANH_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Alpha: f32,
    pub Beta: f32,
}
impl DML_ACTIVATION_SCALED_TANH_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_SCALED_TANH_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_SCALED_TANH_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ACTIVATION_SCALED_TANH_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Alpha", &self.Alpha).field("Beta", &self.Beta).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_SCALED_TANH_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Alpha == other.Alpha && self.Beta == other.Beta
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_SCALED_TANH_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ACTIVATION_SCALED_TANH_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ACTIVATION_SHRINK_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Bias: f32,
    pub Threshold: f32,
}
impl DML_ACTIVATION_SHRINK_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_SHRINK_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_SHRINK_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ACTIVATION_SHRINK_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Bias", &self.Bias).field("Threshold", &self.Threshold).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_SHRINK_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Bias == other.Bias && self.Threshold == other.Threshold
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_SHRINK_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ACTIVATION_SHRINK_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ACTIVATION_SIGMOID_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ACTIVATION_SIGMOID_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_SIGMOID_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_SIGMOID_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ACTIVATION_SIGMOID_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_SIGMOID_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_SIGMOID_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ACTIVATION_SIGMOID_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ACTIVATION_SOFTMAX_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ACTIVATION_SOFTMAX_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_SOFTMAX_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_SOFTMAX_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ACTIVATION_SOFTMAX_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_SOFTMAX_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_SOFTMAX_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ACTIVATION_SOFTMAX_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ACTIVATION_SOFTPLUS_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Steepness: f32,
}
impl DML_ACTIVATION_SOFTPLUS_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_SOFTPLUS_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_SOFTPLUS_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ACTIVATION_SOFTPLUS_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Steepness", &self.Steepness).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_SOFTPLUS_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Steepness == other.Steepness
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_SOFTPLUS_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ACTIVATION_SOFTPLUS_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ACTIVATION_SOFTSIGN_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ACTIVATION_SOFTSIGN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_SOFTSIGN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_SOFTSIGN_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ACTIVATION_SOFTSIGN_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_SOFTSIGN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_SOFTSIGN_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ACTIVATION_SOFTSIGN_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ACTIVATION_TANH_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ACTIVATION_TANH_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_TANH_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_TANH_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ACTIVATION_TANH_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_TANH_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_TANH_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ACTIVATION_TANH_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ACTIVATION_THRESHOLDED_RELU_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Alpha: f32,
}
impl DML_ACTIVATION_THRESHOLDED_RELU_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_THRESHOLDED_RELU_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_THRESHOLDED_RELU_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ACTIVATION_THRESHOLDED_RELU_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Alpha", &self.Alpha).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_THRESHOLDED_RELU_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Alpha == other.Alpha
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_THRESHOLDED_RELU_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ACTIVATION_THRESHOLDED_RELU_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ADAM_OPTIMIZER_OPERATOR_DESC {
    pub InputParametersTensor: *mut DML_TENSOR_DESC,
    pub InputFirstMomentTensor: *mut DML_TENSOR_DESC,
    pub InputSecondMomentTensor: *mut DML_TENSOR_DESC,
    pub GradientTensor: *mut DML_TENSOR_DESC,
    pub TrainingStepTensor: *mut DML_TENSOR_DESC,
    pub OutputParametersTensor: *mut DML_TENSOR_DESC,
    pub OutputFirstMomentTensor: *mut DML_TENSOR_DESC,
    pub OutputSecondMomentTensor: *mut DML_TENSOR_DESC,
    pub LearningRate: f32,
    pub Beta1: f32,
    pub Beta2: f32,
    pub Epsilon: f32,
}
impl DML_ADAM_OPTIMIZER_OPERATOR_DESC {}
impl ::core::default::Default for DML_ADAM_OPTIMIZER_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ADAM_OPTIMIZER_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ADAM_OPTIMIZER_OPERATOR_DESC")
            .field("InputParametersTensor", &self.InputParametersTensor)
            .field("InputFirstMomentTensor", &self.InputFirstMomentTensor)
            .field("InputSecondMomentTensor", &self.InputSecondMomentTensor)
            .field("GradientTensor", &self.GradientTensor)
            .field("TrainingStepTensor", &self.TrainingStepTensor)
            .field("OutputParametersTensor", &self.OutputParametersTensor)
            .field("OutputFirstMomentTensor", &self.OutputFirstMomentTensor)
            .field("OutputSecondMomentTensor", &self.OutputSecondMomentTensor)
            .field("LearningRate", &self.LearningRate)
            .field("Beta1", &self.Beta1)
            .field("Beta2", &self.Beta2)
            .field("Epsilon", &self.Epsilon)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DML_ADAM_OPTIMIZER_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputParametersTensor == other.InputParametersTensor
            && self.InputFirstMomentTensor == other.InputFirstMomentTensor
            && self.InputSecondMomentTensor == other.InputSecondMomentTensor
            && self.GradientTensor == other.GradientTensor
            && self.TrainingStepTensor == other.TrainingStepTensor
            && self.OutputParametersTensor == other.OutputParametersTensor
            && self.OutputFirstMomentTensor == other.OutputFirstMomentTensor
            && self.OutputSecondMomentTensor == other.OutputSecondMomentTensor
            && self.LearningRate == other.LearningRate
            && self.Beta1 == other.Beta1
            && self.Beta2 == other.Beta2
            && self.Epsilon == other.Epsilon
    }
}
impl ::core::cmp::Eq for DML_ADAM_OPTIMIZER_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ADAM_OPTIMIZER_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ARGMAX_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub AxisCount: u32,
    pub Axes: *mut u32,
    pub AxisDirection: DML_AXIS_DIRECTION,
}
impl DML_ARGMAX_OPERATOR_DESC {}
impl ::core::default::Default for DML_ARGMAX_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ARGMAX_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ARGMAX_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("AxisCount", &self.AxisCount).field("Axes", &self.Axes).field("AxisDirection", &self.AxisDirection).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ARGMAX_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.AxisCount == other.AxisCount && self.Axes == other.Axes && self.AxisDirection == other.AxisDirection
    }
}
impl ::core::cmp::Eq for DML_ARGMAX_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ARGMAX_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ARGMIN_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub AxisCount: u32,
    pub Axes: *mut u32,
    pub AxisDirection: DML_AXIS_DIRECTION,
}
impl DML_ARGMIN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ARGMIN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ARGMIN_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ARGMIN_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("AxisCount", &self.AxisCount).field("Axes", &self.Axes).field("AxisDirection", &self.AxisDirection).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ARGMIN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.AxisCount == other.AxisCount && self.Axes == other.Axes && self.AxisDirection == other.AxisDirection
    }
}
impl ::core::cmp::Eq for DML_ARGMIN_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ARGMIN_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Foundation`*"]
pub struct DML_AVERAGE_POOLING_GRAD_OPERATOR_DESC {
    pub InputGradientTensor: *mut DML_TENSOR_DESC,
    pub OutputGradientTensor: *mut DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub Strides: *mut u32,
    pub WindowSize: *mut u32,
    pub StartPadding: *mut u32,
    pub EndPadding: *mut u32,
    pub IncludePadding: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl DML_AVERAGE_POOLING_GRAD_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_AVERAGE_POOLING_GRAD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DML_AVERAGE_POOLING_GRAD_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_AVERAGE_POOLING_GRAD_OPERATOR_DESC")
            .field("InputGradientTensor", &self.InputGradientTensor)
            .field("OutputGradientTensor", &self.OutputGradientTensor)
            .field("DimensionCount", &self.DimensionCount)
            .field("Strides", &self.Strides)
            .field("WindowSize", &self.WindowSize)
            .field("StartPadding", &self.StartPadding)
            .field("EndPadding", &self.EndPadding)
            .field("IncludePadding", &self.IncludePadding)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_AVERAGE_POOLING_GRAD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputGradientTensor == other.InputGradientTensor && self.OutputGradientTensor == other.OutputGradientTensor && self.DimensionCount == other.DimensionCount && self.Strides == other.Strides && self.WindowSize == other.WindowSize && self.StartPadding == other.StartPadding && self.EndPadding == other.EndPadding && self.IncludePadding == other.IncludePadding
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_AVERAGE_POOLING_GRAD_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DML_AVERAGE_POOLING_GRAD_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Foundation`*"]
pub struct DML_AVERAGE_POOLING_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub Strides: *mut u32,
    pub WindowSize: *mut u32,
    pub StartPadding: *mut u32,
    pub EndPadding: *mut u32,
    pub IncludePadding: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl DML_AVERAGE_POOLING_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_AVERAGE_POOLING_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DML_AVERAGE_POOLING_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_AVERAGE_POOLING_OPERATOR_DESC")
            .field("InputTensor", &self.InputTensor)
            .field("OutputTensor", &self.OutputTensor)
            .field("DimensionCount", &self.DimensionCount)
            .field("Strides", &self.Strides)
            .field("WindowSize", &self.WindowSize)
            .field("StartPadding", &self.StartPadding)
            .field("EndPadding", &self.EndPadding)
            .field("IncludePadding", &self.IncludePadding)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_AVERAGE_POOLING_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.DimensionCount == other.DimensionCount && self.Strides == other.Strides && self.WindowSize == other.WindowSize && self.StartPadding == other.StartPadding && self.EndPadding == other.EndPadding && self.IncludePadding == other.IncludePadding
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_AVERAGE_POOLING_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DML_AVERAGE_POOLING_OPERATOR_DESC {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DML_AXIS_DIRECTION(pub i32);
pub const DML_AXIS_DIRECTION_INCREASING: DML_AXIS_DIRECTION = DML_AXIS_DIRECTION(0i32);
pub const DML_AXIS_DIRECTION_DECREASING: DML_AXIS_DIRECTION = DML_AXIS_DIRECTION(1i32);
impl ::core::convert::From<i32> for DML_AXIS_DIRECTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DML_AXIS_DIRECTION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_BATCH_NORMALIZATION_GRAD_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub InputGradientTensor: *mut DML_TENSOR_DESC,
    pub MeanTensor: *mut DML_TENSOR_DESC,
    pub VarianceTensor: *mut DML_TENSOR_DESC,
    pub ScaleTensor: *mut DML_TENSOR_DESC,
    pub OutputGradientTensor: *mut DML_TENSOR_DESC,
    pub OutputScaleGradientTensor: *mut DML_TENSOR_DESC,
    pub OutputBiasGradientTensor: *mut DML_TENSOR_DESC,
    pub Epsilon: f32,
}
impl DML_BATCH_NORMALIZATION_GRAD_OPERATOR_DESC {}
impl ::core::default::Default for DML_BATCH_NORMALIZATION_GRAD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_BATCH_NORMALIZATION_GRAD_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_BATCH_NORMALIZATION_GRAD_OPERATOR_DESC")
            .field("InputTensor", &self.InputTensor)
            .field("InputGradientTensor", &self.InputGradientTensor)
            .field("MeanTensor", &self.MeanTensor)
            .field("VarianceTensor", &self.VarianceTensor)
            .field("ScaleTensor", &self.ScaleTensor)
            .field("OutputGradientTensor", &self.OutputGradientTensor)
            .field("OutputScaleGradientTensor", &self.OutputScaleGradientTensor)
            .field("OutputBiasGradientTensor", &self.OutputBiasGradientTensor)
            .field("Epsilon", &self.Epsilon)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DML_BATCH_NORMALIZATION_GRAD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.InputGradientTensor == other.InputGradientTensor && self.MeanTensor == other.MeanTensor && self.VarianceTensor == other.VarianceTensor && self.ScaleTensor == other.ScaleTensor && self.OutputGradientTensor == other.OutputGradientTensor && self.OutputScaleGradientTensor == other.OutputScaleGradientTensor && self.OutputBiasGradientTensor == other.OutputBiasGradientTensor && self.Epsilon == other.Epsilon
    }
}
impl ::core::cmp::Eq for DML_BATCH_NORMALIZATION_GRAD_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_BATCH_NORMALIZATION_GRAD_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Foundation`*"]
pub struct DML_BATCH_NORMALIZATION_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub MeanTensor: *mut DML_TENSOR_DESC,
    pub VarianceTensor: *mut DML_TENSOR_DESC,
    pub ScaleTensor: *mut DML_TENSOR_DESC,
    pub BiasTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Spatial: super::super::super::Foundation::BOOL,
    pub Epsilon: f32,
    pub FusedActivation: *mut DML_OPERATOR_DESC,
}
#[cfg(feature = "Win32_Foundation")]
impl DML_BATCH_NORMALIZATION_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_BATCH_NORMALIZATION_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DML_BATCH_NORMALIZATION_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_BATCH_NORMALIZATION_OPERATOR_DESC")
            .field("InputTensor", &self.InputTensor)
            .field("MeanTensor", &self.MeanTensor)
            .field("VarianceTensor", &self.VarianceTensor)
            .field("ScaleTensor", &self.ScaleTensor)
            .field("BiasTensor", &self.BiasTensor)
            .field("OutputTensor", &self.OutputTensor)
            .field("Spatial", &self.Spatial)
            .field("Epsilon", &self.Epsilon)
            .field("FusedActivation", &self.FusedActivation)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_BATCH_NORMALIZATION_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.MeanTensor == other.MeanTensor && self.VarianceTensor == other.VarianceTensor && self.ScaleTensor == other.ScaleTensor && self.BiasTensor == other.BiasTensor && self.OutputTensor == other.OutputTensor && self.Spatial == other.Spatial && self.Epsilon == other.Epsilon && self.FusedActivation == other.FusedActivation
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_BATCH_NORMALIZATION_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DML_BATCH_NORMALIZATION_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_BINDING_DESC {
    pub Type: DML_BINDING_TYPE,
    pub Desc: *mut ::core::ffi::c_void,
}
impl DML_BINDING_DESC {}
impl ::core::default::Default for DML_BINDING_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_BINDING_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_BINDING_DESC").field("Type", &self.Type).field("Desc", &self.Desc).finish()
    }
}
impl ::core::cmp::PartialEq for DML_BINDING_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Desc == other.Desc
    }
}
impl ::core::cmp::Eq for DML_BINDING_DESC {}
unsafe impl ::windows::runtime::Abi for DML_BINDING_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_BINDING_PROPERTIES {
    pub RequiredDescriptorCount: u32,
    pub TemporaryResourceSize: u64,
    pub PersistentResourceSize: u64,
}
impl DML_BINDING_PROPERTIES {}
impl ::core::default::Default for DML_BINDING_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_BINDING_PROPERTIES {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_BINDING_PROPERTIES").field("RequiredDescriptorCount", &self.RequiredDescriptorCount).field("TemporaryResourceSize", &self.TemporaryResourceSize).field("PersistentResourceSize", &self.PersistentResourceSize).finish()
    }
}
impl ::core::cmp::PartialEq for DML_BINDING_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.RequiredDescriptorCount == other.RequiredDescriptorCount && self.TemporaryResourceSize == other.TemporaryResourceSize && self.PersistentResourceSize == other.PersistentResourceSize
    }
}
impl ::core::cmp::Eq for DML_BINDING_PROPERTIES {}
unsafe impl ::windows::runtime::Abi for DML_BINDING_PROPERTIES {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Graphics_Direct3D12`*"]
pub struct DML_BINDING_TABLE_DESC {
    pub Dispatchable: ::core::option::Option<IDMLDispatchable>,
    pub CPUDescriptorHandle: super::super::super::Graphics::Direct3D12::D3D12_CPU_DESCRIPTOR_HANDLE,
    pub GPUDescriptorHandle: super::super::super::Graphics::Direct3D12::D3D12_GPU_DESCRIPTOR_HANDLE,
    pub SizeInDescriptors: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl DML_BINDING_TABLE_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::default::Default for DML_BINDING_TABLE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for DML_BINDING_TABLE_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_BINDING_TABLE_DESC").field("Dispatchable", &self.Dispatchable).field("CPUDescriptorHandle", &self.CPUDescriptorHandle).field("GPUDescriptorHandle", &self.GPUDescriptorHandle).field("SizeInDescriptors", &self.SizeInDescriptors).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for DML_BINDING_TABLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Dispatchable == other.Dispatchable && self.CPUDescriptorHandle == other.CPUDescriptorHandle && self.GPUDescriptorHandle == other.GPUDescriptorHandle && self.SizeInDescriptors == other.SizeInDescriptors
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for DML_BINDING_TABLE_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
unsafe impl ::windows::runtime::Abi for DML_BINDING_TABLE_DESC {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DML_BINDING_TYPE(pub i32);
pub const DML_BINDING_TYPE_NONE: DML_BINDING_TYPE = DML_BINDING_TYPE(0i32);
pub const DML_BINDING_TYPE_BUFFER: DML_BINDING_TYPE = DML_BINDING_TYPE(1i32);
pub const DML_BINDING_TYPE_BUFFER_ARRAY: DML_BINDING_TYPE = DML_BINDING_TYPE(2i32);
impl ::core::convert::From<i32> for DML_BINDING_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DML_BINDING_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Graphics_Direct3D12`*"]
pub struct DML_BUFFER_ARRAY_BINDING {
    pub BindingCount: u32,
    pub Bindings: *mut DML_BUFFER_BINDING,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl DML_BUFFER_ARRAY_BINDING {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::default::Default for DML_BUFFER_ARRAY_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for DML_BUFFER_ARRAY_BINDING {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_BUFFER_ARRAY_BINDING").field("BindingCount", &self.BindingCount).field("Bindings", &self.Bindings).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for DML_BUFFER_ARRAY_BINDING {
    fn eq(&self, other: &Self) -> bool {
        self.BindingCount == other.BindingCount && self.Bindings == other.Bindings
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for DML_BUFFER_ARRAY_BINDING {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
unsafe impl ::windows::runtime::Abi for DML_BUFFER_ARRAY_BINDING {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Graphics_Direct3D12`*"]
pub struct DML_BUFFER_BINDING {
    pub Buffer: ::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
    pub Offset: u64,
    pub SizeInBytes: u64,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl DML_BUFFER_BINDING {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::default::Default for DML_BUFFER_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for DML_BUFFER_BINDING {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_BUFFER_BINDING").field("Buffer", &self.Buffer).field("Offset", &self.Offset).field("SizeInBytes", &self.SizeInBytes).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for DML_BUFFER_BINDING {
    fn eq(&self, other: &Self) -> bool {
        self.Buffer == other.Buffer && self.Offset == other.Offset && self.SizeInBytes == other.SizeInBytes
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for DML_BUFFER_BINDING {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
unsafe impl ::windows::runtime::Abi for DML_BUFFER_BINDING {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_BUFFER_TENSOR_DESC {
    pub DataType: DML_TENSOR_DATA_TYPE,
    pub Flags: DML_TENSOR_FLAGS,
    pub DimensionCount: u32,
    pub Sizes: *mut u32,
    pub Strides: *mut u32,
    pub TotalTensorSizeInBytes: u64,
    pub GuaranteedBaseOffsetAlignment: u32,
}
impl DML_BUFFER_TENSOR_DESC {}
impl ::core::default::Default for DML_BUFFER_TENSOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_BUFFER_TENSOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_BUFFER_TENSOR_DESC")
            .field("DataType", &self.DataType)
            .field("Flags", &self.Flags)
            .field("DimensionCount", &self.DimensionCount)
            .field("Sizes", &self.Sizes)
            .field("Strides", &self.Strides)
            .field("TotalTensorSizeInBytes", &self.TotalTensorSizeInBytes)
            .field("GuaranteedBaseOffsetAlignment", &self.GuaranteedBaseOffsetAlignment)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DML_BUFFER_TENSOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.DataType == other.DataType && self.Flags == other.Flags && self.DimensionCount == other.DimensionCount && self.Sizes == other.Sizes && self.Strides == other.Strides && self.TotalTensorSizeInBytes == other.TotalTensorSizeInBytes && self.GuaranteedBaseOffsetAlignment == other.GuaranteedBaseOffsetAlignment
    }
}
impl ::core::cmp::Eq for DML_BUFFER_TENSOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_BUFFER_TENSOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_CAST_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_CAST_OPERATOR_DESC {}
impl ::core::default::Default for DML_CAST_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_CAST_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_CAST_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_CAST_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_CAST_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_CAST_OPERATOR_DESC {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DML_CONVOLUTION_DIRECTION(pub i32);
pub const DML_CONVOLUTION_DIRECTION_FORWARD: DML_CONVOLUTION_DIRECTION = DML_CONVOLUTION_DIRECTION(0i32);
pub const DML_CONVOLUTION_DIRECTION_BACKWARD: DML_CONVOLUTION_DIRECTION = DML_CONVOLUTION_DIRECTION(1i32);
impl ::core::convert::From<i32> for DML_CONVOLUTION_DIRECTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DML_CONVOLUTION_DIRECTION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_CONVOLUTION_INTEGER_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub InputZeroPointTensor: *mut DML_TENSOR_DESC,
    pub FilterTensor: *mut DML_TENSOR_DESC,
    pub FilterZeroPointTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub Strides: *mut u32,
    pub Dilations: *mut u32,
    pub StartPadding: *mut u32,
    pub EndPadding: *mut u32,
    pub GroupCount: u32,
}
impl DML_CONVOLUTION_INTEGER_OPERATOR_DESC {}
impl ::core::default::Default for DML_CONVOLUTION_INTEGER_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_CONVOLUTION_INTEGER_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_CONVOLUTION_INTEGER_OPERATOR_DESC")
            .field("InputTensor", &self.InputTensor)
            .field("InputZeroPointTensor", &self.InputZeroPointTensor)
            .field("FilterTensor", &self.FilterTensor)
            .field("FilterZeroPointTensor", &self.FilterZeroPointTensor)
            .field("OutputTensor", &self.OutputTensor)
            .field("DimensionCount", &self.DimensionCount)
            .field("Strides", &self.Strides)
            .field("Dilations", &self.Dilations)
            .field("StartPadding", &self.StartPadding)
            .field("EndPadding", &self.EndPadding)
            .field("GroupCount", &self.GroupCount)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DML_CONVOLUTION_INTEGER_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.InputZeroPointTensor == other.InputZeroPointTensor && self.FilterTensor == other.FilterTensor && self.FilterZeroPointTensor == other.FilterZeroPointTensor && self.OutputTensor == other.OutputTensor && self.DimensionCount == other.DimensionCount && self.Strides == other.Strides && self.Dilations == other.Dilations && self.StartPadding == other.StartPadding && self.EndPadding == other.EndPadding && self.GroupCount == other.GroupCount
    }
}
impl ::core::cmp::Eq for DML_CONVOLUTION_INTEGER_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_CONVOLUTION_INTEGER_OPERATOR_DESC {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DML_CONVOLUTION_MODE(pub i32);
pub const DML_CONVOLUTION_MODE_CONVOLUTION: DML_CONVOLUTION_MODE = DML_CONVOLUTION_MODE(0i32);
pub const DML_CONVOLUTION_MODE_CROSS_CORRELATION: DML_CONVOLUTION_MODE = DML_CONVOLUTION_MODE(1i32);
impl ::core::convert::From<i32> for DML_CONVOLUTION_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DML_CONVOLUTION_MODE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_CONVOLUTION_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub FilterTensor: *mut DML_TENSOR_DESC,
    pub BiasTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Mode: DML_CONVOLUTION_MODE,
    pub Direction: DML_CONVOLUTION_DIRECTION,
    pub DimensionCount: u32,
    pub Strides: *mut u32,
    pub Dilations: *mut u32,
    pub StartPadding: *mut u32,
    pub EndPadding: *mut u32,
    pub OutputPadding: *mut u32,
    pub GroupCount: u32,
    pub FusedActivation: *mut DML_OPERATOR_DESC,
}
impl DML_CONVOLUTION_OPERATOR_DESC {}
impl ::core::default::Default for DML_CONVOLUTION_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_CONVOLUTION_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_CONVOLUTION_OPERATOR_DESC")
            .field("InputTensor", &self.InputTensor)
            .field("FilterTensor", &self.FilterTensor)
            .field("BiasTensor", &self.BiasTensor)
            .field("OutputTensor", &self.OutputTensor)
            .field("Mode", &self.Mode)
            .field("Direction", &self.Direction)
            .field("DimensionCount", &self.DimensionCount)
            .field("Strides", &self.Strides)
            .field("Dilations", &self.Dilations)
            .field("StartPadding", &self.StartPadding)
            .field("EndPadding", &self.EndPadding)
            .field("OutputPadding", &self.OutputPadding)
            .field("GroupCount", &self.GroupCount)
            .field("FusedActivation", &self.FusedActivation)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DML_CONVOLUTION_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor
            && self.FilterTensor == other.FilterTensor
            && self.BiasTensor == other.BiasTensor
            && self.OutputTensor == other.OutputTensor
            && self.Mode == other.Mode
            && self.Direction == other.Direction
            && self.DimensionCount == other.DimensionCount
            && self.Strides == other.Strides
            && self.Dilations == other.Dilations
            && self.StartPadding == other.StartPadding
            && self.EndPadding == other.EndPadding
            && self.OutputPadding == other.OutputPadding
            && self.GroupCount == other.GroupCount
            && self.FusedActivation == other.FusedActivation
    }
}
impl ::core::cmp::Eq for DML_CONVOLUTION_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_CONVOLUTION_OPERATOR_DESC {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DML_CREATE_DEVICE_FLAGS(pub u32);
pub const DML_CREATE_DEVICE_FLAG_NONE: DML_CREATE_DEVICE_FLAGS = DML_CREATE_DEVICE_FLAGS(0u32);
pub const DML_CREATE_DEVICE_FLAG_DEBUG: DML_CREATE_DEVICE_FLAGS = DML_CREATE_DEVICE_FLAGS(1u32);
impl ::core::convert::From<u32> for DML_CREATE_DEVICE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DML_CREATE_DEVICE_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for DML_CREATE_DEVICE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for DML_CREATE_DEVICE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for DML_CREATE_DEVICE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for DML_CREATE_DEVICE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for DML_CREATE_DEVICE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Foundation`*"]
pub struct DML_CUMULATIVE_PRODUCT_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Axis: u32,
    pub AxisDirection: DML_AXIS_DIRECTION,
    pub HasExclusiveProduct: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl DML_CUMULATIVE_PRODUCT_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_CUMULATIVE_PRODUCT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DML_CUMULATIVE_PRODUCT_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_CUMULATIVE_PRODUCT_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Axis", &self.Axis).field("AxisDirection", &self.AxisDirection).field("HasExclusiveProduct", &self.HasExclusiveProduct).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_CUMULATIVE_PRODUCT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Axis == other.Axis && self.AxisDirection == other.AxisDirection && self.HasExclusiveProduct == other.HasExclusiveProduct
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_CUMULATIVE_PRODUCT_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DML_CUMULATIVE_PRODUCT_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Foundation`*"]
pub struct DML_CUMULATIVE_SUMMATION_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Axis: u32,
    pub AxisDirection: DML_AXIS_DIRECTION,
    pub HasExclusiveSum: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl DML_CUMULATIVE_SUMMATION_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_CUMULATIVE_SUMMATION_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DML_CUMULATIVE_SUMMATION_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_CUMULATIVE_SUMMATION_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Axis", &self.Axis).field("AxisDirection", &self.AxisDirection).field("HasExclusiveSum", &self.HasExclusiveSum).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_CUMULATIVE_SUMMATION_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Axis == other.Axis && self.AxisDirection == other.AxisDirection && self.HasExclusiveSum == other.HasExclusiveSum
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_CUMULATIVE_SUMMATION_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DML_CUMULATIVE_SUMMATION_OPERATOR_DESC {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DML_DEPTH_SPACE_ORDER(pub i32);
pub const DML_DEPTH_SPACE_ORDER_DEPTH_COLUMN_ROW: DML_DEPTH_SPACE_ORDER = DML_DEPTH_SPACE_ORDER(0i32);
pub const DML_DEPTH_SPACE_ORDER_COLUMN_ROW_DEPTH: DML_DEPTH_SPACE_ORDER = DML_DEPTH_SPACE_ORDER(1i32);
impl ::core::convert::From<i32> for DML_DEPTH_SPACE_ORDER {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DML_DEPTH_SPACE_ORDER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_DEPTH_TO_SPACE1_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub BlockSize: u32,
    pub Order: DML_DEPTH_SPACE_ORDER,
}
impl DML_DEPTH_TO_SPACE1_OPERATOR_DESC {}
impl ::core::default::Default for DML_DEPTH_TO_SPACE1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_DEPTH_TO_SPACE1_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_DEPTH_TO_SPACE1_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("BlockSize", &self.BlockSize).field("Order", &self.Order).finish()
    }
}
impl ::core::cmp::PartialEq for DML_DEPTH_TO_SPACE1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.BlockSize == other.BlockSize && self.Order == other.Order
    }
}
impl ::core::cmp::Eq for DML_DEPTH_TO_SPACE1_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_DEPTH_TO_SPACE1_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_DEPTH_TO_SPACE_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub BlockSize: u32,
}
impl DML_DEPTH_TO_SPACE_OPERATOR_DESC {}
impl ::core::default::Default for DML_DEPTH_TO_SPACE_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_DEPTH_TO_SPACE_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_DEPTH_TO_SPACE_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("BlockSize", &self.BlockSize).finish()
    }
}
impl ::core::cmp::PartialEq for DML_DEPTH_TO_SPACE_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.BlockSize == other.BlockSize
    }
}
impl ::core::cmp::Eq for DML_DEPTH_TO_SPACE_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_DEPTH_TO_SPACE_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_DIAGONAL_MATRIX_OPERATOR_DESC {
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Offset: i32,
    pub Value: f32,
}
impl DML_DIAGONAL_MATRIX_OPERATOR_DESC {}
impl ::core::default::Default for DML_DIAGONAL_MATRIX_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_DIAGONAL_MATRIX_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_DIAGONAL_MATRIX_OPERATOR_DESC").field("OutputTensor", &self.OutputTensor).field("Offset", &self.Offset).field("Value", &self.Value).finish()
    }
}
impl ::core::cmp::PartialEq for DML_DIAGONAL_MATRIX_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.OutputTensor == other.OutputTensor && self.Offset == other.Offset && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for DML_DIAGONAL_MATRIX_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_DIAGONAL_MATRIX_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_DYNAMIC_QUANTIZE_LINEAR_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub OutputScaleTensor: *mut DML_TENSOR_DESC,
    pub OutputZeroPointTensor: *mut DML_TENSOR_DESC,
}
impl DML_DYNAMIC_QUANTIZE_LINEAR_OPERATOR_DESC {}
impl ::core::default::Default for DML_DYNAMIC_QUANTIZE_LINEAR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_DYNAMIC_QUANTIZE_LINEAR_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_DYNAMIC_QUANTIZE_LINEAR_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("OutputScaleTensor", &self.OutputScaleTensor).field("OutputZeroPointTensor", &self.OutputZeroPointTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_DYNAMIC_QUANTIZE_LINEAR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.OutputScaleTensor == other.OutputScaleTensor && self.OutputZeroPointTensor == other.OutputZeroPointTensor
    }
}
impl ::core::cmp::Eq for DML_DYNAMIC_QUANTIZE_LINEAR_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_DYNAMIC_QUANTIZE_LINEAR_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_ABS_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl DML_ELEMENT_WISE_ABS_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_ABS_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_ABS_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_ABS_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ABS_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ABS_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_ABS_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_ACOSH_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl DML_ELEMENT_WISE_ACOSH_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_ACOSH_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_ACOSH_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_ACOSH_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ACOSH_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ACOSH_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_ACOSH_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_ACOS_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl DML_ELEMENT_WISE_ACOS_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_ACOS_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_ACOS_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_ACOS_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ACOS_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ACOS_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_ACOS_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_ADD1_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub FusedActivation: *mut DML_OPERATOR_DESC,
}
impl DML_ELEMENT_WISE_ADD1_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_ADD1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_ADD1_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_ADD1_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).field("FusedActivation", &self.FusedActivation).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ADD1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor && self.FusedActivation == other.FusedActivation
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ADD1_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_ADD1_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_ADD_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ELEMENT_WISE_ADD_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_ADD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_ADD_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_ADD_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ADD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ADD_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_ADD_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_ASINH_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl DML_ELEMENT_WISE_ASINH_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_ASINH_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_ASINH_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_ASINH_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ASINH_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ASINH_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_ASINH_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_ASIN_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl DML_ELEMENT_WISE_ASIN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_ASIN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_ASIN_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_ASIN_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ASIN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ASIN_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_ASIN_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_ATANH_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl DML_ELEMENT_WISE_ATANH_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_ATANH_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_ATANH_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_ATANH_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ATANH_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ATANH_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_ATANH_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_ATAN_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl DML_ELEMENT_WISE_ATAN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_ATAN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_ATAN_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_ATAN_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ATAN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ATAN_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_ATAN_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_ATAN_YX_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ELEMENT_WISE_ATAN_YX_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_ATAN_YX_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_ATAN_YX_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_ATAN_YX_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ATAN_YX_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ATAN_YX_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_ATAN_YX_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_BIT_AND_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ELEMENT_WISE_BIT_AND_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_BIT_AND_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_BIT_AND_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_BIT_AND_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_BIT_AND_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_BIT_AND_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_BIT_AND_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_BIT_COUNT_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ELEMENT_WISE_BIT_COUNT_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_BIT_COUNT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_BIT_COUNT_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_BIT_COUNT_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_BIT_COUNT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_BIT_COUNT_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_BIT_COUNT_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_BIT_NOT_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ELEMENT_WISE_BIT_NOT_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_BIT_NOT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_BIT_NOT_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_BIT_NOT_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_BIT_NOT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_BIT_NOT_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_BIT_NOT_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_BIT_OR_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ELEMENT_WISE_BIT_OR_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_BIT_OR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_BIT_OR_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_BIT_OR_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_BIT_OR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_BIT_OR_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_BIT_OR_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_BIT_SHIFT_LEFT_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ELEMENT_WISE_BIT_SHIFT_LEFT_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_BIT_SHIFT_LEFT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_BIT_SHIFT_LEFT_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_BIT_SHIFT_LEFT_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_BIT_SHIFT_LEFT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_BIT_SHIFT_LEFT_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_BIT_SHIFT_LEFT_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_BIT_SHIFT_RIGHT_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ELEMENT_WISE_BIT_SHIFT_RIGHT_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_BIT_SHIFT_RIGHT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_BIT_SHIFT_RIGHT_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_BIT_SHIFT_RIGHT_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_BIT_SHIFT_RIGHT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_BIT_SHIFT_RIGHT_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_BIT_SHIFT_RIGHT_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_BIT_XOR_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ELEMENT_WISE_BIT_XOR_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_BIT_XOR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_BIT_XOR_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_BIT_XOR_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_BIT_XOR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_BIT_XOR_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_BIT_XOR_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_CEIL_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl DML_ELEMENT_WISE_CEIL_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_CEIL_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_CEIL_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_CEIL_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_CEIL_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_CEIL_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_CEIL_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_CLIP_GRAD_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub InputGradientTensor: *mut DML_TENSOR_DESC,
    pub OutputGradientTensor: *mut DML_TENSOR_DESC,
    pub Min: f32,
    pub Max: f32,
}
impl DML_ELEMENT_WISE_CLIP_GRAD_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_CLIP_GRAD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_CLIP_GRAD_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_CLIP_GRAD_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("InputGradientTensor", &self.InputGradientTensor).field("OutputGradientTensor", &self.OutputGradientTensor).field("Min", &self.Min).field("Max", &self.Max).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_CLIP_GRAD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.InputGradientTensor == other.InputGradientTensor && self.OutputGradientTensor == other.OutputGradientTensor && self.Min == other.Min && self.Max == other.Max
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_CLIP_GRAD_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_CLIP_GRAD_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_CLIP_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
    pub Min: f32,
    pub Max: f32,
}
impl DML_ELEMENT_WISE_CLIP_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_CLIP_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_CLIP_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_CLIP_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).field("Min", &self.Min).field("Max", &self.Max).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_CLIP_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias && self.Min == other.Min && self.Max == other.Max
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_CLIP_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_CLIP_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
    pub Exponent: f32,
}
impl DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).field("Exponent", &self.Exponent).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias && self.Exponent == other.Exponent
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_COSH_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl DML_ELEMENT_WISE_COSH_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_COSH_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_COSH_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_COSH_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_COSH_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_COSH_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_COSH_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_COS_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl DML_ELEMENT_WISE_COS_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_COS_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_COS_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_COS_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_COS_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_COS_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_COS_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_DEQUANTIZE_LINEAR_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub ScaleTensor: *mut DML_TENSOR_DESC,
    pub ZeroPointTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ELEMENT_WISE_DEQUANTIZE_LINEAR_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_DEQUANTIZE_LINEAR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_DEQUANTIZE_LINEAR_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_DEQUANTIZE_LINEAR_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("ScaleTensor", &self.ScaleTensor).field("ZeroPointTensor", &self.ZeroPointTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_DEQUANTIZE_LINEAR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.ScaleTensor == other.ScaleTensor && self.ZeroPointTensor == other.ZeroPointTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_DEQUANTIZE_LINEAR_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_DEQUANTIZE_LINEAR_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_DIFFERENCE_SQUARE_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ELEMENT_WISE_DIFFERENCE_SQUARE_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_DIFFERENCE_SQUARE_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_DIFFERENCE_SQUARE_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_DIFFERENCE_SQUARE_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_DIFFERENCE_SQUARE_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_DIFFERENCE_SQUARE_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_DIFFERENCE_SQUARE_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_DIVIDE_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ELEMENT_WISE_DIVIDE_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_DIVIDE_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_DIVIDE_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_DIVIDE_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_DIVIDE_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_DIVIDE_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_DIVIDE_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_ERF_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl DML_ELEMENT_WISE_ERF_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_ERF_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_ERF_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_ERF_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ERF_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ERF_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_ERF_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_EXP_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl DML_ELEMENT_WISE_EXP_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_EXP_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_EXP_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_EXP_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_EXP_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_EXP_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_EXP_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_FLOOR_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl DML_ELEMENT_WISE_FLOOR_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_FLOOR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_FLOOR_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_FLOOR_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_FLOOR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_FLOOR_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_FLOOR_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_IDENTITY_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl DML_ELEMENT_WISE_IDENTITY_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_IDENTITY_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_IDENTITY_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_IDENTITY_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_IDENTITY_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_IDENTITY_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_IDENTITY_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_IF_OPERATOR_DESC {
    pub ConditionTensor: *mut DML_TENSOR_DESC,
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ELEMENT_WISE_IF_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_IF_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_IF_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_IF_OPERATOR_DESC").field("ConditionTensor", &self.ConditionTensor).field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_IF_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ConditionTensor == other.ConditionTensor && self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_IF_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_IF_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_IS_INFINITY_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub InfinityMode: DML_IS_INFINITY_MODE,
}
impl DML_ELEMENT_WISE_IS_INFINITY_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_IS_INFINITY_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_IS_INFINITY_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_IS_INFINITY_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("InfinityMode", &self.InfinityMode).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_IS_INFINITY_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.InfinityMode == other.InfinityMode
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_IS_INFINITY_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_IS_INFINITY_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_IS_NAN_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ELEMENT_WISE_IS_NAN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_IS_NAN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_IS_NAN_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_IS_NAN_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_IS_NAN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_IS_NAN_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_IS_NAN_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_LOGICAL_AND_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ELEMENT_WISE_LOGICAL_AND_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_LOGICAL_AND_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_LOGICAL_AND_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_LOGICAL_AND_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOGICAL_AND_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOGICAL_AND_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_LOGICAL_AND_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_LOGICAL_EQUALS_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ELEMENT_WISE_LOGICAL_EQUALS_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_LOGICAL_EQUALS_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_LOGICAL_EQUALS_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_LOGICAL_EQUALS_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOGICAL_EQUALS_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOGICAL_EQUALS_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_LOGICAL_EQUALS_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_LOGICAL_NOT_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ELEMENT_WISE_LOGICAL_NOT_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_LOGICAL_NOT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_LOGICAL_NOT_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_LOGICAL_NOT_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOGICAL_NOT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOGICAL_NOT_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_LOGICAL_NOT_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_LOGICAL_OR_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ELEMENT_WISE_LOGICAL_OR_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_LOGICAL_OR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_LOGICAL_OR_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_LOGICAL_OR_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOGICAL_OR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOGICAL_OR_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_LOGICAL_OR_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_LOGICAL_XOR_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ELEMENT_WISE_LOGICAL_XOR_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_LOGICAL_XOR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_LOGICAL_XOR_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_LOGICAL_XOR_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOGICAL_XOR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOGICAL_XOR_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_LOGICAL_XOR_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_LOG_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl DML_ELEMENT_WISE_LOG_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_LOG_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_LOG_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_LOG_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOG_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOG_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_LOG_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_MAX_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ELEMENT_WISE_MAX_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_MAX_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_MAX_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_MAX_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_MAX_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_MAX_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_MAX_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_MEAN_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ELEMENT_WISE_MEAN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_MEAN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_MEAN_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_MEAN_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_MEAN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_MEAN_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_MEAN_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_MIN_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ELEMENT_WISE_MIN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_MIN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_MIN_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_MIN_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_MIN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_MIN_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_MIN_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_MODULUS_FLOOR_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ELEMENT_WISE_MODULUS_FLOOR_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_MODULUS_FLOOR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_MODULUS_FLOOR_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_MODULUS_FLOOR_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_MODULUS_FLOOR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_MODULUS_FLOOR_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_MODULUS_FLOOR_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_MODULUS_TRUNCATE_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ELEMENT_WISE_MODULUS_TRUNCATE_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_MODULUS_TRUNCATE_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_MODULUS_TRUNCATE_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_MODULUS_TRUNCATE_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_MODULUS_TRUNCATE_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_MODULUS_TRUNCATE_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_MODULUS_TRUNCATE_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_MULTIPLY_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ELEMENT_WISE_MULTIPLY_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_MULTIPLY_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_MULTIPLY_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_MULTIPLY_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_MULTIPLY_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_MULTIPLY_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_MULTIPLY_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_POW_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub ExponentTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl DML_ELEMENT_WISE_POW_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_POW_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_POW_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_POW_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("ExponentTensor", &self.ExponentTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_POW_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.ExponentTensor == other.ExponentTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_POW_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_POW_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_QUANTIZED_LINEAR_ADD_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub AScaleTensor: *mut DML_TENSOR_DESC,
    pub AZeroPointTensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub BScaleTensor: *mut DML_TENSOR_DESC,
    pub BZeroPointTensor: *mut DML_TENSOR_DESC,
    pub OutputScaleTensor: *mut DML_TENSOR_DESC,
    pub OutputZeroPointTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ELEMENT_WISE_QUANTIZED_LINEAR_ADD_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_QUANTIZED_LINEAR_ADD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_QUANTIZED_LINEAR_ADD_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_QUANTIZED_LINEAR_ADD_OPERATOR_DESC")
            .field("ATensor", &self.ATensor)
            .field("AScaleTensor", &self.AScaleTensor)
            .field("AZeroPointTensor", &self.AZeroPointTensor)
            .field("BTensor", &self.BTensor)
            .field("BScaleTensor", &self.BScaleTensor)
            .field("BZeroPointTensor", &self.BZeroPointTensor)
            .field("OutputScaleTensor", &self.OutputScaleTensor)
            .field("OutputZeroPointTensor", &self.OutputZeroPointTensor)
            .field("OutputTensor", &self.OutputTensor)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_QUANTIZED_LINEAR_ADD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.AScaleTensor == other.AScaleTensor && self.AZeroPointTensor == other.AZeroPointTensor && self.BTensor == other.BTensor && self.BScaleTensor == other.BScaleTensor && self.BZeroPointTensor == other.BZeroPointTensor && self.OutputScaleTensor == other.OutputScaleTensor && self.OutputZeroPointTensor == other.OutputZeroPointTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_QUANTIZED_LINEAR_ADD_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_QUANTIZED_LINEAR_ADD_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_QUANTIZE_LINEAR_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub ScaleTensor: *mut DML_TENSOR_DESC,
    pub ZeroPointTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ELEMENT_WISE_QUANTIZE_LINEAR_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_QUANTIZE_LINEAR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_QUANTIZE_LINEAR_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_QUANTIZE_LINEAR_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("ScaleTensor", &self.ScaleTensor).field("ZeroPointTensor", &self.ZeroPointTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_QUANTIZE_LINEAR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.ScaleTensor == other.ScaleTensor && self.ZeroPointTensor == other.ZeroPointTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_QUANTIZE_LINEAR_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_QUANTIZE_LINEAR_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_RECIP_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl DML_ELEMENT_WISE_RECIP_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_RECIP_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_RECIP_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_RECIP_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_RECIP_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_RECIP_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_RECIP_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_ROUND_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub RoundingMode: DML_ROUNDING_MODE,
}
impl DML_ELEMENT_WISE_ROUND_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_ROUND_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_ROUND_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_ROUND_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("RoundingMode", &self.RoundingMode).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ROUND_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.RoundingMode == other.RoundingMode
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ROUND_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_ROUND_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_SIGN_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ELEMENT_WISE_SIGN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_SIGN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_SIGN_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_SIGN_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_SIGN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_SIGN_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_SIGN_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_SINH_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl DML_ELEMENT_WISE_SINH_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_SINH_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_SINH_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_SINH_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_SINH_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_SINH_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_SINH_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_SIN_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl DML_ELEMENT_WISE_SIN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_SIN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_SIN_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_SIN_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_SIN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_SIN_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_SIN_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_SQRT_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl DML_ELEMENT_WISE_SQRT_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_SQRT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_SQRT_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_SQRT_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_SQRT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_SQRT_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_SQRT_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_SUBTRACT_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_ELEMENT_WISE_SUBTRACT_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_SUBTRACT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_SUBTRACT_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_SUBTRACT_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_SUBTRACT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_SUBTRACT_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_SUBTRACT_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_TANH_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl DML_ELEMENT_WISE_TANH_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_TANH_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_TANH_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_TANH_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_TANH_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_TANH_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_TANH_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_TAN_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl DML_ELEMENT_WISE_TAN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_TAN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_TAN_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_TAN_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_TAN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_TAN_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_TAN_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
    pub Min: f32,
}
impl DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).field("Min", &self.Min).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias && self.Min == other.Min
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DML_EXECUTION_FLAGS(pub u32);
pub const DML_EXECUTION_FLAG_NONE: DML_EXECUTION_FLAGS = DML_EXECUTION_FLAGS(0u32);
pub const DML_EXECUTION_FLAG_ALLOW_HALF_PRECISION_COMPUTATION: DML_EXECUTION_FLAGS = DML_EXECUTION_FLAGS(1u32);
pub const DML_EXECUTION_FLAG_DISABLE_META_COMMANDS: DML_EXECUTION_FLAGS = DML_EXECUTION_FLAGS(2u32);
pub const DML_EXECUTION_FLAG_DESCRIPTORS_VOLATILE: DML_EXECUTION_FLAGS = DML_EXECUTION_FLAGS(4u32);
impl ::core::convert::From<u32> for DML_EXECUTION_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DML_EXECUTION_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for DML_EXECUTION_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for DML_EXECUTION_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for DML_EXECUTION_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for DML_EXECUTION_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for DML_EXECUTION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DML_FEATURE(pub i32);
pub const DML_FEATURE_TENSOR_DATA_TYPE_SUPPORT: DML_FEATURE = DML_FEATURE(0i32);
pub const DML_FEATURE_FEATURE_LEVELS: DML_FEATURE = DML_FEATURE(1i32);
impl ::core::convert::From<i32> for DML_FEATURE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DML_FEATURE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_FEATURE_DATA_FEATURE_LEVELS {
    pub MaxSupportedFeatureLevel: DML_FEATURE_LEVEL,
}
impl DML_FEATURE_DATA_FEATURE_LEVELS {}
impl ::core::default::Default for DML_FEATURE_DATA_FEATURE_LEVELS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_FEATURE_DATA_FEATURE_LEVELS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_FEATURE_DATA_FEATURE_LEVELS").field("MaxSupportedFeatureLevel", &self.MaxSupportedFeatureLevel).finish()
    }
}
impl ::core::cmp::PartialEq for DML_FEATURE_DATA_FEATURE_LEVELS {
    fn eq(&self, other: &Self) -> bool {
        self.MaxSupportedFeatureLevel == other.MaxSupportedFeatureLevel
    }
}
impl ::core::cmp::Eq for DML_FEATURE_DATA_FEATURE_LEVELS {}
unsafe impl ::windows::runtime::Abi for DML_FEATURE_DATA_FEATURE_LEVELS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Foundation`*"]
pub struct DML_FEATURE_DATA_TENSOR_DATA_TYPE_SUPPORT {
    pub IsSupported: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl DML_FEATURE_DATA_TENSOR_DATA_TYPE_SUPPORT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_FEATURE_DATA_TENSOR_DATA_TYPE_SUPPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DML_FEATURE_DATA_TENSOR_DATA_TYPE_SUPPORT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_FEATURE_DATA_TENSOR_DATA_TYPE_SUPPORT").field("IsSupported", &self.IsSupported).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_FEATURE_DATA_TENSOR_DATA_TYPE_SUPPORT {
    fn eq(&self, other: &Self) -> bool {
        self.IsSupported == other.IsSupported
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_FEATURE_DATA_TENSOR_DATA_TYPE_SUPPORT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DML_FEATURE_DATA_TENSOR_DATA_TYPE_SUPPORT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DML_FEATURE_LEVEL(pub i32);
pub const DML_FEATURE_LEVEL_1_0: DML_FEATURE_LEVEL = DML_FEATURE_LEVEL(4096i32);
pub const DML_FEATURE_LEVEL_2_0: DML_FEATURE_LEVEL = DML_FEATURE_LEVEL(8192i32);
pub const DML_FEATURE_LEVEL_2_1: DML_FEATURE_LEVEL = DML_FEATURE_LEVEL(8448i32);
pub const DML_FEATURE_LEVEL_3_0: DML_FEATURE_LEVEL = DML_FEATURE_LEVEL(12288i32);
pub const DML_FEATURE_LEVEL_3_1: DML_FEATURE_LEVEL = DML_FEATURE_LEVEL(12544i32);
pub const DML_FEATURE_LEVEL_4_0: DML_FEATURE_LEVEL = DML_FEATURE_LEVEL(16384i32);
impl ::core::convert::From<i32> for DML_FEATURE_LEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DML_FEATURE_LEVEL {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_FEATURE_QUERY_FEATURE_LEVELS {
    pub RequestedFeatureLevelCount: u32,
    pub RequestedFeatureLevels: *mut DML_FEATURE_LEVEL,
}
impl DML_FEATURE_QUERY_FEATURE_LEVELS {}
impl ::core::default::Default for DML_FEATURE_QUERY_FEATURE_LEVELS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_FEATURE_QUERY_FEATURE_LEVELS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_FEATURE_QUERY_FEATURE_LEVELS").field("RequestedFeatureLevelCount", &self.RequestedFeatureLevelCount).field("RequestedFeatureLevels", &self.RequestedFeatureLevels).finish()
    }
}
impl ::core::cmp::PartialEq for DML_FEATURE_QUERY_FEATURE_LEVELS {
    fn eq(&self, other: &Self) -> bool {
        self.RequestedFeatureLevelCount == other.RequestedFeatureLevelCount && self.RequestedFeatureLevels == other.RequestedFeatureLevels
    }
}
impl ::core::cmp::Eq for DML_FEATURE_QUERY_FEATURE_LEVELS {}
unsafe impl ::windows::runtime::Abi for DML_FEATURE_QUERY_FEATURE_LEVELS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_FEATURE_QUERY_TENSOR_DATA_TYPE_SUPPORT {
    pub DataType: DML_TENSOR_DATA_TYPE,
}
impl DML_FEATURE_QUERY_TENSOR_DATA_TYPE_SUPPORT {}
impl ::core::default::Default for DML_FEATURE_QUERY_TENSOR_DATA_TYPE_SUPPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_FEATURE_QUERY_TENSOR_DATA_TYPE_SUPPORT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_FEATURE_QUERY_TENSOR_DATA_TYPE_SUPPORT").field("DataType", &self.DataType).finish()
    }
}
impl ::core::cmp::PartialEq for DML_FEATURE_QUERY_TENSOR_DATA_TYPE_SUPPORT {
    fn eq(&self, other: &Self) -> bool {
        self.DataType == other.DataType
    }
}
impl ::core::cmp::Eq for DML_FEATURE_QUERY_TENSOR_DATA_TYPE_SUPPORT {}
unsafe impl ::windows::runtime::Abi for DML_FEATURE_QUERY_TENSOR_DATA_TYPE_SUPPORT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_FILL_VALUE_CONSTANT_OPERATOR_DESC {
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ValueDataType: DML_TENSOR_DATA_TYPE,
    pub Value: DML_SCALAR_UNION,
}
impl DML_FILL_VALUE_CONSTANT_OPERATOR_DESC {}
impl ::core::default::Default for DML_FILL_VALUE_CONSTANT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_FILL_VALUE_CONSTANT_OPERATOR_DESC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for DML_FILL_VALUE_CONSTANT_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_FILL_VALUE_CONSTANT_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_FILL_VALUE_SEQUENCE_OPERATOR_DESC {
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ValueDataType: DML_TENSOR_DATA_TYPE,
    pub ValueStart: DML_SCALAR_UNION,
    pub ValueDelta: DML_SCALAR_UNION,
}
impl DML_FILL_VALUE_SEQUENCE_OPERATOR_DESC {}
impl ::core::default::Default for DML_FILL_VALUE_SEQUENCE_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_FILL_VALUE_SEQUENCE_OPERATOR_DESC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for DML_FILL_VALUE_SEQUENCE_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_FILL_VALUE_SEQUENCE_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_GATHER_ELEMENTS_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub IndicesTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Axis: u32,
}
impl DML_GATHER_ELEMENTS_OPERATOR_DESC {}
impl ::core::default::Default for DML_GATHER_ELEMENTS_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_GATHER_ELEMENTS_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_GATHER_ELEMENTS_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("IndicesTensor", &self.IndicesTensor).field("OutputTensor", &self.OutputTensor).field("Axis", &self.Axis).finish()
    }
}
impl ::core::cmp::PartialEq for DML_GATHER_ELEMENTS_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.IndicesTensor == other.IndicesTensor && self.OutputTensor == other.OutputTensor && self.Axis == other.Axis
    }
}
impl ::core::cmp::Eq for DML_GATHER_ELEMENTS_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_GATHER_ELEMENTS_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_GATHER_ND1_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub IndicesTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub InputDimensionCount: u32,
    pub IndicesDimensionCount: u32,
    pub BatchDimensionCount: u32,
}
impl DML_GATHER_ND1_OPERATOR_DESC {}
impl ::core::default::Default for DML_GATHER_ND1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_GATHER_ND1_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_GATHER_ND1_OPERATOR_DESC")
            .field("InputTensor", &self.InputTensor)
            .field("IndicesTensor", &self.IndicesTensor)
            .field("OutputTensor", &self.OutputTensor)
            .field("InputDimensionCount", &self.InputDimensionCount)
            .field("IndicesDimensionCount", &self.IndicesDimensionCount)
            .field("BatchDimensionCount", &self.BatchDimensionCount)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DML_GATHER_ND1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.IndicesTensor == other.IndicesTensor && self.OutputTensor == other.OutputTensor && self.InputDimensionCount == other.InputDimensionCount && self.IndicesDimensionCount == other.IndicesDimensionCount && self.BatchDimensionCount == other.BatchDimensionCount
    }
}
impl ::core::cmp::Eq for DML_GATHER_ND1_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_GATHER_ND1_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_GATHER_ND_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub IndicesTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub InputDimensionCount: u32,
    pub IndicesDimensionCount: u32,
}
impl DML_GATHER_ND_OPERATOR_DESC {}
impl ::core::default::Default for DML_GATHER_ND_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_GATHER_ND_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_GATHER_ND_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("IndicesTensor", &self.IndicesTensor).field("OutputTensor", &self.OutputTensor).field("InputDimensionCount", &self.InputDimensionCount).field("IndicesDimensionCount", &self.IndicesDimensionCount).finish()
    }
}
impl ::core::cmp::PartialEq for DML_GATHER_ND_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.IndicesTensor == other.IndicesTensor && self.OutputTensor == other.OutputTensor && self.InputDimensionCount == other.InputDimensionCount && self.IndicesDimensionCount == other.IndicesDimensionCount
    }
}
impl ::core::cmp::Eq for DML_GATHER_ND_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_GATHER_ND_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_GATHER_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub IndicesTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Axis: u32,
    pub IndexDimensions: u32,
}
impl DML_GATHER_OPERATOR_DESC {}
impl ::core::default::Default for DML_GATHER_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_GATHER_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_GATHER_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("IndicesTensor", &self.IndicesTensor).field("OutputTensor", &self.OutputTensor).field("Axis", &self.Axis).field("IndexDimensions", &self.IndexDimensions).finish()
    }
}
impl ::core::cmp::PartialEq for DML_GATHER_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.IndicesTensor == other.IndicesTensor && self.OutputTensor == other.OutputTensor && self.Axis == other.Axis && self.IndexDimensions == other.IndexDimensions
    }
}
impl ::core::cmp::Eq for DML_GATHER_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_GATHER_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_GEMM_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub CTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub TransA: DML_MATRIX_TRANSFORM,
    pub TransB: DML_MATRIX_TRANSFORM,
    pub Alpha: f32,
    pub Beta: f32,
    pub FusedActivation: *mut DML_OPERATOR_DESC,
}
impl DML_GEMM_OPERATOR_DESC {}
impl ::core::default::Default for DML_GEMM_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_GEMM_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_GEMM_OPERATOR_DESC")
            .field("ATensor", &self.ATensor)
            .field("BTensor", &self.BTensor)
            .field("CTensor", &self.CTensor)
            .field("OutputTensor", &self.OutputTensor)
            .field("TransA", &self.TransA)
            .field("TransB", &self.TransB)
            .field("Alpha", &self.Alpha)
            .field("Beta", &self.Beta)
            .field("FusedActivation", &self.FusedActivation)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DML_GEMM_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.CTensor == other.CTensor && self.OutputTensor == other.OutputTensor && self.TransA == other.TransA && self.TransB == other.TransB && self.Alpha == other.Alpha && self.Beta == other.Beta && self.FusedActivation == other.FusedActivation
    }
}
impl ::core::cmp::Eq for DML_GEMM_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_GEMM_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_GRAPH_DESC {
    pub InputCount: u32,
    pub OutputCount: u32,
    pub NodeCount: u32,
    pub Nodes: *mut DML_GRAPH_NODE_DESC,
    pub InputEdgeCount: u32,
    pub InputEdges: *mut DML_GRAPH_EDGE_DESC,
    pub OutputEdgeCount: u32,
    pub OutputEdges: *mut DML_GRAPH_EDGE_DESC,
    pub IntermediateEdgeCount: u32,
    pub IntermediateEdges: *mut DML_GRAPH_EDGE_DESC,
}
impl DML_GRAPH_DESC {}
impl ::core::default::Default for DML_GRAPH_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_GRAPH_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_GRAPH_DESC")
            .field("InputCount", &self.InputCount)
            .field("OutputCount", &self.OutputCount)
            .field("NodeCount", &self.NodeCount)
            .field("Nodes", &self.Nodes)
            .field("InputEdgeCount", &self.InputEdgeCount)
            .field("InputEdges", &self.InputEdges)
            .field("OutputEdgeCount", &self.OutputEdgeCount)
            .field("OutputEdges", &self.OutputEdges)
            .field("IntermediateEdgeCount", &self.IntermediateEdgeCount)
            .field("IntermediateEdges", &self.IntermediateEdges)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DML_GRAPH_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputCount == other.InputCount && self.OutputCount == other.OutputCount && self.NodeCount == other.NodeCount && self.Nodes == other.Nodes && self.InputEdgeCount == other.InputEdgeCount && self.InputEdges == other.InputEdges && self.OutputEdgeCount == other.OutputEdgeCount && self.OutputEdges == other.OutputEdges && self.IntermediateEdgeCount == other.IntermediateEdgeCount && self.IntermediateEdges == other.IntermediateEdges
    }
}
impl ::core::cmp::Eq for DML_GRAPH_DESC {}
unsafe impl ::windows::runtime::Abi for DML_GRAPH_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_GRAPH_EDGE_DESC {
    pub Type: DML_GRAPH_EDGE_TYPE,
    pub Desc: *mut ::core::ffi::c_void,
}
impl DML_GRAPH_EDGE_DESC {}
impl ::core::default::Default for DML_GRAPH_EDGE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_GRAPH_EDGE_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_GRAPH_EDGE_DESC").field("Type", &self.Type).field("Desc", &self.Desc).finish()
    }
}
impl ::core::cmp::PartialEq for DML_GRAPH_EDGE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Desc == other.Desc
    }
}
impl ::core::cmp::Eq for DML_GRAPH_EDGE_DESC {}
unsafe impl ::windows::runtime::Abi for DML_GRAPH_EDGE_DESC {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DML_GRAPH_EDGE_TYPE(pub i32);
pub const DML_GRAPH_EDGE_TYPE_INVALID: DML_GRAPH_EDGE_TYPE = DML_GRAPH_EDGE_TYPE(0i32);
pub const DML_GRAPH_EDGE_TYPE_INPUT: DML_GRAPH_EDGE_TYPE = DML_GRAPH_EDGE_TYPE(1i32);
pub const DML_GRAPH_EDGE_TYPE_OUTPUT: DML_GRAPH_EDGE_TYPE = DML_GRAPH_EDGE_TYPE(2i32);
pub const DML_GRAPH_EDGE_TYPE_INTERMEDIATE: DML_GRAPH_EDGE_TYPE = DML_GRAPH_EDGE_TYPE(3i32);
impl ::core::convert::From<i32> for DML_GRAPH_EDGE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DML_GRAPH_EDGE_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_GRAPH_NODE_DESC {
    pub Type: DML_GRAPH_NODE_TYPE,
    pub Desc: *mut ::core::ffi::c_void,
}
impl DML_GRAPH_NODE_DESC {}
impl ::core::default::Default for DML_GRAPH_NODE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_GRAPH_NODE_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_GRAPH_NODE_DESC").field("Type", &self.Type).field("Desc", &self.Desc).finish()
    }
}
impl ::core::cmp::PartialEq for DML_GRAPH_NODE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Desc == other.Desc
    }
}
impl ::core::cmp::Eq for DML_GRAPH_NODE_DESC {}
unsafe impl ::windows::runtime::Abi for DML_GRAPH_NODE_DESC {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DML_GRAPH_NODE_TYPE(pub i32);
pub const DML_GRAPH_NODE_TYPE_INVALID: DML_GRAPH_NODE_TYPE = DML_GRAPH_NODE_TYPE(0i32);
pub const DML_GRAPH_NODE_TYPE_OPERATOR: DML_GRAPH_NODE_TYPE = DML_GRAPH_NODE_TYPE(1i32);
impl ::core::convert::From<i32> for DML_GRAPH_NODE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DML_GRAPH_NODE_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Foundation`*"]
pub struct DML_GRU_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub WeightTensor: *mut DML_TENSOR_DESC,
    pub RecurrenceTensor: *mut DML_TENSOR_DESC,
    pub BiasTensor: *mut DML_TENSOR_DESC,
    pub HiddenInitTensor: *mut DML_TENSOR_DESC,
    pub SequenceLengthsTensor: *mut DML_TENSOR_DESC,
    pub OutputSequenceTensor: *mut DML_TENSOR_DESC,
    pub OutputSingleTensor: *mut DML_TENSOR_DESC,
    pub ActivationDescCount: u32,
    pub ActivationDescs: *mut DML_OPERATOR_DESC,
    pub Direction: DML_RECURRENT_NETWORK_DIRECTION,
    pub LinearBeforeReset: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl DML_GRU_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_GRU_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DML_GRU_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_GRU_OPERATOR_DESC")
            .field("InputTensor", &self.InputTensor)
            .field("WeightTensor", &self.WeightTensor)
            .field("RecurrenceTensor", &self.RecurrenceTensor)
            .field("BiasTensor", &self.BiasTensor)
            .field("HiddenInitTensor", &self.HiddenInitTensor)
            .field("SequenceLengthsTensor", &self.SequenceLengthsTensor)
            .field("OutputSequenceTensor", &self.OutputSequenceTensor)
            .field("OutputSingleTensor", &self.OutputSingleTensor)
            .field("ActivationDescCount", &self.ActivationDescCount)
            .field("ActivationDescs", &self.ActivationDescs)
            .field("Direction", &self.Direction)
            .field("LinearBeforeReset", &self.LinearBeforeReset)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_GRU_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor
            && self.WeightTensor == other.WeightTensor
            && self.RecurrenceTensor == other.RecurrenceTensor
            && self.BiasTensor == other.BiasTensor
            && self.HiddenInitTensor == other.HiddenInitTensor
            && self.SequenceLengthsTensor == other.SequenceLengthsTensor
            && self.OutputSequenceTensor == other.OutputSequenceTensor
            && self.OutputSingleTensor == other.OutputSingleTensor
            && self.ActivationDescCount == other.ActivationDescCount
            && self.ActivationDescs == other.ActivationDescs
            && self.Direction == other.Direction
            && self.LinearBeforeReset == other.LinearBeforeReset
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_GRU_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DML_GRU_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Foundation`*"]
pub struct DML_INPUT_GRAPH_EDGE_DESC {
    pub GraphInputIndex: u32,
    pub ToNodeIndex: u32,
    pub ToNodeInputIndex: u32,
    pub Name: super::super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl DML_INPUT_GRAPH_EDGE_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_INPUT_GRAPH_EDGE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DML_INPUT_GRAPH_EDGE_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_INPUT_GRAPH_EDGE_DESC").field("GraphInputIndex", &self.GraphInputIndex).field("ToNodeIndex", &self.ToNodeIndex).field("ToNodeInputIndex", &self.ToNodeInputIndex).field("Name", &self.Name).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_INPUT_GRAPH_EDGE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.GraphInputIndex == other.GraphInputIndex && self.ToNodeIndex == other.ToNodeIndex && self.ToNodeInputIndex == other.ToNodeInputIndex && self.Name == other.Name
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_INPUT_GRAPH_EDGE_DESC {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DML_INPUT_GRAPH_EDGE_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Foundation`*"]
pub struct DML_INTERMEDIATE_GRAPH_EDGE_DESC {
    pub FromNodeIndex: u32,
    pub FromNodeOutputIndex: u32,
    pub ToNodeIndex: u32,
    pub ToNodeInputIndex: u32,
    pub Name: super::super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl DML_INTERMEDIATE_GRAPH_EDGE_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_INTERMEDIATE_GRAPH_EDGE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DML_INTERMEDIATE_GRAPH_EDGE_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_INTERMEDIATE_GRAPH_EDGE_DESC").field("FromNodeIndex", &self.FromNodeIndex).field("FromNodeOutputIndex", &self.FromNodeOutputIndex).field("ToNodeIndex", &self.ToNodeIndex).field("ToNodeInputIndex", &self.ToNodeInputIndex).field("Name", &self.Name).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_INTERMEDIATE_GRAPH_EDGE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.FromNodeIndex == other.FromNodeIndex && self.FromNodeOutputIndex == other.FromNodeOutputIndex && self.ToNodeIndex == other.ToNodeIndex && self.ToNodeInputIndex == other.ToNodeInputIndex && self.Name == other.Name
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_INTERMEDIATE_GRAPH_EDGE_DESC {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DML_INTERMEDIATE_GRAPH_EDGE_DESC {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DML_INTERPOLATION_MODE(pub i32);
pub const DML_INTERPOLATION_MODE_NEAREST_NEIGHBOR: DML_INTERPOLATION_MODE = DML_INTERPOLATION_MODE(0i32);
pub const DML_INTERPOLATION_MODE_LINEAR: DML_INTERPOLATION_MODE = DML_INTERPOLATION_MODE(1i32);
impl ::core::convert::From<i32> for DML_INTERPOLATION_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DML_INTERPOLATION_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DML_IS_INFINITY_MODE(pub i32);
pub const DML_IS_INFINITY_MODE_EITHER: DML_IS_INFINITY_MODE = DML_IS_INFINITY_MODE(0i32);
pub const DML_IS_INFINITY_MODE_POSITIVE: DML_IS_INFINITY_MODE = DML_IS_INFINITY_MODE(1i32);
pub const DML_IS_INFINITY_MODE_NEGATIVE: DML_IS_INFINITY_MODE = DML_IS_INFINITY_MODE(2i32);
impl ::core::convert::From<i32> for DML_IS_INFINITY_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DML_IS_INFINITY_MODE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_JOIN_OPERATOR_DESC {
    pub InputCount: u32,
    pub InputTensors: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Axis: u32,
}
impl DML_JOIN_OPERATOR_DESC {}
impl ::core::default::Default for DML_JOIN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_JOIN_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_JOIN_OPERATOR_DESC").field("InputCount", &self.InputCount).field("InputTensors", &self.InputTensors).field("OutputTensor", &self.OutputTensor).field("Axis", &self.Axis).finish()
    }
}
impl ::core::cmp::PartialEq for DML_JOIN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputCount == other.InputCount && self.InputTensors == other.InputTensors && self.OutputTensor == other.OutputTensor && self.Axis == other.Axis
    }
}
impl ::core::cmp::Eq for DML_JOIN_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_JOIN_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Foundation`*"]
pub struct DML_LOCAL_RESPONSE_NORMALIZATION_GRAD_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub InputGradientTensor: *mut DML_TENSOR_DESC,
    pub OutputGradientTensor: *mut DML_TENSOR_DESC,
    pub CrossChannel: super::super::super::Foundation::BOOL,
    pub LocalSize: u32,
    pub Alpha: f32,
    pub Beta: f32,
    pub Bias: f32,
}
#[cfg(feature = "Win32_Foundation")]
impl DML_LOCAL_RESPONSE_NORMALIZATION_GRAD_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_LOCAL_RESPONSE_NORMALIZATION_GRAD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DML_LOCAL_RESPONSE_NORMALIZATION_GRAD_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_LOCAL_RESPONSE_NORMALIZATION_GRAD_OPERATOR_DESC")
            .field("InputTensor", &self.InputTensor)
            .field("InputGradientTensor", &self.InputGradientTensor)
            .field("OutputGradientTensor", &self.OutputGradientTensor)
            .field("CrossChannel", &self.CrossChannel)
            .field("LocalSize", &self.LocalSize)
            .field("Alpha", &self.Alpha)
            .field("Beta", &self.Beta)
            .field("Bias", &self.Bias)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_LOCAL_RESPONSE_NORMALIZATION_GRAD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.InputGradientTensor == other.InputGradientTensor && self.OutputGradientTensor == other.OutputGradientTensor && self.CrossChannel == other.CrossChannel && self.LocalSize == other.LocalSize && self.Alpha == other.Alpha && self.Beta == other.Beta && self.Bias == other.Bias
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_LOCAL_RESPONSE_NORMALIZATION_GRAD_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DML_LOCAL_RESPONSE_NORMALIZATION_GRAD_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Foundation`*"]
pub struct DML_LOCAL_RESPONSE_NORMALIZATION_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub CrossChannel: super::super::super::Foundation::BOOL,
    pub LocalSize: u32,
    pub Alpha: f32,
    pub Beta: f32,
    pub Bias: f32,
}
#[cfg(feature = "Win32_Foundation")]
impl DML_LOCAL_RESPONSE_NORMALIZATION_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_LOCAL_RESPONSE_NORMALIZATION_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DML_LOCAL_RESPONSE_NORMALIZATION_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_LOCAL_RESPONSE_NORMALIZATION_OPERATOR_DESC")
            .field("InputTensor", &self.InputTensor)
            .field("OutputTensor", &self.OutputTensor)
            .field("CrossChannel", &self.CrossChannel)
            .field("LocalSize", &self.LocalSize)
            .field("Alpha", &self.Alpha)
            .field("Beta", &self.Beta)
            .field("Bias", &self.Bias)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_LOCAL_RESPONSE_NORMALIZATION_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.CrossChannel == other.CrossChannel && self.LocalSize == other.LocalSize && self.Alpha == other.Alpha && self.Beta == other.Beta && self.Bias == other.Bias
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_LOCAL_RESPONSE_NORMALIZATION_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DML_LOCAL_RESPONSE_NORMALIZATION_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_LP_NORMALIZATION_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Axis: u32,
    pub Epsilon: f32,
    pub P: u32,
}
impl DML_LP_NORMALIZATION_OPERATOR_DESC {}
impl ::core::default::Default for DML_LP_NORMALIZATION_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_LP_NORMALIZATION_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_LP_NORMALIZATION_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Axis", &self.Axis).field("Epsilon", &self.Epsilon).field("P", &self.P).finish()
    }
}
impl ::core::cmp::PartialEq for DML_LP_NORMALIZATION_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Axis == other.Axis && self.Epsilon == other.Epsilon && self.P == other.P
    }
}
impl ::core::cmp::Eq for DML_LP_NORMALIZATION_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_LP_NORMALIZATION_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_LP_POOLING_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub Strides: *mut u32,
    pub WindowSize: *mut u32,
    pub StartPadding: *mut u32,
    pub EndPadding: *mut u32,
    pub P: u32,
}
impl DML_LP_POOLING_OPERATOR_DESC {}
impl ::core::default::Default for DML_LP_POOLING_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_LP_POOLING_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_LP_POOLING_OPERATOR_DESC")
            .field("InputTensor", &self.InputTensor)
            .field("OutputTensor", &self.OutputTensor)
            .field("DimensionCount", &self.DimensionCount)
            .field("Strides", &self.Strides)
            .field("WindowSize", &self.WindowSize)
            .field("StartPadding", &self.StartPadding)
            .field("EndPadding", &self.EndPadding)
            .field("P", &self.P)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DML_LP_POOLING_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.DimensionCount == other.DimensionCount && self.Strides == other.Strides && self.WindowSize == other.WindowSize && self.StartPadding == other.StartPadding && self.EndPadding == other.EndPadding && self.P == other.P
    }
}
impl ::core::cmp::Eq for DML_LP_POOLING_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_LP_POOLING_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Foundation`*"]
pub struct DML_LSTM_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub WeightTensor: *mut DML_TENSOR_DESC,
    pub RecurrenceTensor: *mut DML_TENSOR_DESC,
    pub BiasTensor: *mut DML_TENSOR_DESC,
    pub HiddenInitTensor: *mut DML_TENSOR_DESC,
    pub CellMemInitTensor: *mut DML_TENSOR_DESC,
    pub SequenceLengthsTensor: *mut DML_TENSOR_DESC,
    pub PeepholeTensor: *mut DML_TENSOR_DESC,
    pub OutputSequenceTensor: *mut DML_TENSOR_DESC,
    pub OutputSingleTensor: *mut DML_TENSOR_DESC,
    pub OutputCellSingleTensor: *mut DML_TENSOR_DESC,
    pub ActivationDescCount: u32,
    pub ActivationDescs: *mut DML_OPERATOR_DESC,
    pub Direction: DML_RECURRENT_NETWORK_DIRECTION,
    pub ClipThreshold: f32,
    pub UseClipThreshold: super::super::super::Foundation::BOOL,
    pub CoupleInputForget: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl DML_LSTM_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_LSTM_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DML_LSTM_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_LSTM_OPERATOR_DESC")
            .field("InputTensor", &self.InputTensor)
            .field("WeightTensor", &self.WeightTensor)
            .field("RecurrenceTensor", &self.RecurrenceTensor)
            .field("BiasTensor", &self.BiasTensor)
            .field("HiddenInitTensor", &self.HiddenInitTensor)
            .field("CellMemInitTensor", &self.CellMemInitTensor)
            .field("SequenceLengthsTensor", &self.SequenceLengthsTensor)
            .field("PeepholeTensor", &self.PeepholeTensor)
            .field("OutputSequenceTensor", &self.OutputSequenceTensor)
            .field("OutputSingleTensor", &self.OutputSingleTensor)
            .field("OutputCellSingleTensor", &self.OutputCellSingleTensor)
            .field("ActivationDescCount", &self.ActivationDescCount)
            .field("ActivationDescs", &self.ActivationDescs)
            .field("Direction", &self.Direction)
            .field("ClipThreshold", &self.ClipThreshold)
            .field("UseClipThreshold", &self.UseClipThreshold)
            .field("CoupleInputForget", &self.CoupleInputForget)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_LSTM_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor
            && self.WeightTensor == other.WeightTensor
            && self.RecurrenceTensor == other.RecurrenceTensor
            && self.BiasTensor == other.BiasTensor
            && self.HiddenInitTensor == other.HiddenInitTensor
            && self.CellMemInitTensor == other.CellMemInitTensor
            && self.SequenceLengthsTensor == other.SequenceLengthsTensor
            && self.PeepholeTensor == other.PeepholeTensor
            && self.OutputSequenceTensor == other.OutputSequenceTensor
            && self.OutputSingleTensor == other.OutputSingleTensor
            && self.OutputCellSingleTensor == other.OutputCellSingleTensor
            && self.ActivationDescCount == other.ActivationDescCount
            && self.ActivationDescs == other.ActivationDescs
            && self.Direction == other.Direction
            && self.ClipThreshold == other.ClipThreshold
            && self.UseClipThreshold == other.UseClipThreshold
            && self.CoupleInputForget == other.CoupleInputForget
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_LSTM_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DML_LSTM_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_MATRIX_MULTIPLY_INTEGER_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub AZeroPointTensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub BZeroPointTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_MATRIX_MULTIPLY_INTEGER_OPERATOR_DESC {}
impl ::core::default::Default for DML_MATRIX_MULTIPLY_INTEGER_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_MATRIX_MULTIPLY_INTEGER_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_MATRIX_MULTIPLY_INTEGER_OPERATOR_DESC").field("ATensor", &self.ATensor).field("AZeroPointTensor", &self.AZeroPointTensor).field("BTensor", &self.BTensor).field("BZeroPointTensor", &self.BZeroPointTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_MATRIX_MULTIPLY_INTEGER_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.AZeroPointTensor == other.AZeroPointTensor && self.BTensor == other.BTensor && self.BZeroPointTensor == other.BZeroPointTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_MATRIX_MULTIPLY_INTEGER_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_MATRIX_MULTIPLY_INTEGER_OPERATOR_DESC {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DML_MATRIX_TRANSFORM(pub i32);
pub const DML_MATRIX_TRANSFORM_NONE: DML_MATRIX_TRANSFORM = DML_MATRIX_TRANSFORM(0i32);
pub const DML_MATRIX_TRANSFORM_TRANSPOSE: DML_MATRIX_TRANSFORM = DML_MATRIX_TRANSFORM(1i32);
impl ::core::convert::From<i32> for DML_MATRIX_TRANSFORM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DML_MATRIX_TRANSFORM {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_MAX_POOLING1_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub OutputIndicesTensor: *mut DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub Strides: *mut u32,
    pub WindowSize: *mut u32,
    pub StartPadding: *mut u32,
    pub EndPadding: *mut u32,
}
impl DML_MAX_POOLING1_OPERATOR_DESC {}
impl ::core::default::Default for DML_MAX_POOLING1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_MAX_POOLING1_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_MAX_POOLING1_OPERATOR_DESC")
            .field("InputTensor", &self.InputTensor)
            .field("OutputTensor", &self.OutputTensor)
            .field("OutputIndicesTensor", &self.OutputIndicesTensor)
            .field("DimensionCount", &self.DimensionCount)
            .field("Strides", &self.Strides)
            .field("WindowSize", &self.WindowSize)
            .field("StartPadding", &self.StartPadding)
            .field("EndPadding", &self.EndPadding)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DML_MAX_POOLING1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.OutputIndicesTensor == other.OutputIndicesTensor && self.DimensionCount == other.DimensionCount && self.Strides == other.Strides && self.WindowSize == other.WindowSize && self.StartPadding == other.StartPadding && self.EndPadding == other.EndPadding
    }
}
impl ::core::cmp::Eq for DML_MAX_POOLING1_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_MAX_POOLING1_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_MAX_POOLING2_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub OutputIndicesTensor: *mut DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub Strides: *mut u32,
    pub WindowSize: *mut u32,
    pub StartPadding: *mut u32,
    pub EndPadding: *mut u32,
    pub Dilations: *mut u32,
}
impl DML_MAX_POOLING2_OPERATOR_DESC {}
impl ::core::default::Default for DML_MAX_POOLING2_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_MAX_POOLING2_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_MAX_POOLING2_OPERATOR_DESC")
            .field("InputTensor", &self.InputTensor)
            .field("OutputTensor", &self.OutputTensor)
            .field("OutputIndicesTensor", &self.OutputIndicesTensor)
            .field("DimensionCount", &self.DimensionCount)
            .field("Strides", &self.Strides)
            .field("WindowSize", &self.WindowSize)
            .field("StartPadding", &self.StartPadding)
            .field("EndPadding", &self.EndPadding)
            .field("Dilations", &self.Dilations)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DML_MAX_POOLING2_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.OutputIndicesTensor == other.OutputIndicesTensor && self.DimensionCount == other.DimensionCount && self.Strides == other.Strides && self.WindowSize == other.WindowSize && self.StartPadding == other.StartPadding && self.EndPadding == other.EndPadding && self.Dilations == other.Dilations
    }
}
impl ::core::cmp::Eq for DML_MAX_POOLING2_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_MAX_POOLING2_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_MAX_POOLING_GRAD_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub InputGradientTensor: *mut DML_TENSOR_DESC,
    pub OutputGradientTensor: *mut DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub Strides: *mut u32,
    pub WindowSize: *mut u32,
    pub StartPadding: *mut u32,
    pub EndPadding: *mut u32,
    pub Dilations: *mut u32,
}
impl DML_MAX_POOLING_GRAD_OPERATOR_DESC {}
impl ::core::default::Default for DML_MAX_POOLING_GRAD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_MAX_POOLING_GRAD_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_MAX_POOLING_GRAD_OPERATOR_DESC")
            .field("InputTensor", &self.InputTensor)
            .field("InputGradientTensor", &self.InputGradientTensor)
            .field("OutputGradientTensor", &self.OutputGradientTensor)
            .field("DimensionCount", &self.DimensionCount)
            .field("Strides", &self.Strides)
            .field("WindowSize", &self.WindowSize)
            .field("StartPadding", &self.StartPadding)
            .field("EndPadding", &self.EndPadding)
            .field("Dilations", &self.Dilations)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DML_MAX_POOLING_GRAD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.InputGradientTensor == other.InputGradientTensor && self.OutputGradientTensor == other.OutputGradientTensor && self.DimensionCount == other.DimensionCount && self.Strides == other.Strides && self.WindowSize == other.WindowSize && self.StartPadding == other.StartPadding && self.EndPadding == other.EndPadding && self.Dilations == other.Dilations
    }
}
impl ::core::cmp::Eq for DML_MAX_POOLING_GRAD_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_MAX_POOLING_GRAD_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_MAX_POOLING_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub Strides: *mut u32,
    pub WindowSize: *mut u32,
    pub StartPadding: *mut u32,
    pub EndPadding: *mut u32,
}
impl DML_MAX_POOLING_OPERATOR_DESC {}
impl ::core::default::Default for DML_MAX_POOLING_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_MAX_POOLING_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_MAX_POOLING_OPERATOR_DESC")
            .field("InputTensor", &self.InputTensor)
            .field("OutputTensor", &self.OutputTensor)
            .field("DimensionCount", &self.DimensionCount)
            .field("Strides", &self.Strides)
            .field("WindowSize", &self.WindowSize)
            .field("StartPadding", &self.StartPadding)
            .field("EndPadding", &self.EndPadding)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DML_MAX_POOLING_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.DimensionCount == other.DimensionCount && self.Strides == other.Strides && self.WindowSize == other.WindowSize && self.StartPadding == other.StartPadding && self.EndPadding == other.EndPadding
    }
}
impl ::core::cmp::Eq for DML_MAX_POOLING_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_MAX_POOLING_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_MAX_UNPOOLING_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub IndicesTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_MAX_UNPOOLING_OPERATOR_DESC {}
impl ::core::default::Default for DML_MAX_UNPOOLING_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_MAX_UNPOOLING_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_MAX_UNPOOLING_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("IndicesTensor", &self.IndicesTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_MAX_UNPOOLING_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.IndicesTensor == other.IndicesTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_MAX_UNPOOLING_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_MAX_UNPOOLING_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Foundation`*"]
pub struct DML_MEAN_VARIANCE_NORMALIZATION1_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub ScaleTensor: *mut DML_TENSOR_DESC,
    pub BiasTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub AxisCount: u32,
    pub Axes: *mut u32,
    pub NormalizeVariance: super::super::super::Foundation::BOOL,
    pub Epsilon: f32,
    pub FusedActivation: *mut DML_OPERATOR_DESC,
}
#[cfg(feature = "Win32_Foundation")]
impl DML_MEAN_VARIANCE_NORMALIZATION1_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_MEAN_VARIANCE_NORMALIZATION1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DML_MEAN_VARIANCE_NORMALIZATION1_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_MEAN_VARIANCE_NORMALIZATION1_OPERATOR_DESC")
            .field("InputTensor", &self.InputTensor)
            .field("ScaleTensor", &self.ScaleTensor)
            .field("BiasTensor", &self.BiasTensor)
            .field("OutputTensor", &self.OutputTensor)
            .field("AxisCount", &self.AxisCount)
            .field("Axes", &self.Axes)
            .field("NormalizeVariance", &self.NormalizeVariance)
            .field("Epsilon", &self.Epsilon)
            .field("FusedActivation", &self.FusedActivation)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_MEAN_VARIANCE_NORMALIZATION1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.ScaleTensor == other.ScaleTensor && self.BiasTensor == other.BiasTensor && self.OutputTensor == other.OutputTensor && self.AxisCount == other.AxisCount && self.Axes == other.Axes && self.NormalizeVariance == other.NormalizeVariance && self.Epsilon == other.Epsilon && self.FusedActivation == other.FusedActivation
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_MEAN_VARIANCE_NORMALIZATION1_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DML_MEAN_VARIANCE_NORMALIZATION1_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Foundation`*"]
pub struct DML_MEAN_VARIANCE_NORMALIZATION_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub ScaleTensor: *mut DML_TENSOR_DESC,
    pub BiasTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub CrossChannel: super::super::super::Foundation::BOOL,
    pub NormalizeVariance: super::super::super::Foundation::BOOL,
    pub Epsilon: f32,
    pub FusedActivation: *mut DML_OPERATOR_DESC,
}
#[cfg(feature = "Win32_Foundation")]
impl DML_MEAN_VARIANCE_NORMALIZATION_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_MEAN_VARIANCE_NORMALIZATION_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DML_MEAN_VARIANCE_NORMALIZATION_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_MEAN_VARIANCE_NORMALIZATION_OPERATOR_DESC")
            .field("InputTensor", &self.InputTensor)
            .field("ScaleTensor", &self.ScaleTensor)
            .field("BiasTensor", &self.BiasTensor)
            .field("OutputTensor", &self.OutputTensor)
            .field("CrossChannel", &self.CrossChannel)
            .field("NormalizeVariance", &self.NormalizeVariance)
            .field("Epsilon", &self.Epsilon)
            .field("FusedActivation", &self.FusedActivation)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_MEAN_VARIANCE_NORMALIZATION_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.ScaleTensor == other.ScaleTensor && self.BiasTensor == other.BiasTensor && self.OutputTensor == other.OutputTensor && self.CrossChannel == other.CrossChannel && self.NormalizeVariance == other.NormalizeVariance && self.Epsilon == other.Epsilon && self.FusedActivation == other.FusedActivation
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_MEAN_VARIANCE_NORMALIZATION_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DML_MEAN_VARIANCE_NORMALIZATION_OPERATOR_DESC {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub const DML_MINIMUM_BUFFER_TENSOR_ALIGNMENT: u32 = 16u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_NONZERO_COORDINATES_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputCountTensor: *mut DML_TENSOR_DESC,
    pub OutputCoordinatesTensor: *mut DML_TENSOR_DESC,
}
impl DML_NONZERO_COORDINATES_OPERATOR_DESC {}
impl ::core::default::Default for DML_NONZERO_COORDINATES_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_NONZERO_COORDINATES_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_NONZERO_COORDINATES_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputCountTensor", &self.OutputCountTensor).field("OutputCoordinatesTensor", &self.OutputCoordinatesTensor).finish()
    }
}
impl ::core::cmp::PartialEq for DML_NONZERO_COORDINATES_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputCountTensor == other.OutputCountTensor && self.OutputCoordinatesTensor == other.OutputCoordinatesTensor
    }
}
impl ::core::cmp::Eq for DML_NONZERO_COORDINATES_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_NONZERO_COORDINATES_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ONE_HOT_OPERATOR_DESC {
    pub IndicesTensor: *mut DML_TENSOR_DESC,
    pub ValuesTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Axis: u32,
}
impl DML_ONE_HOT_OPERATOR_DESC {}
impl ::core::default::Default for DML_ONE_HOT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ONE_HOT_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ONE_HOT_OPERATOR_DESC").field("IndicesTensor", &self.IndicesTensor).field("ValuesTensor", &self.ValuesTensor).field("OutputTensor", &self.OutputTensor).field("Axis", &self.Axis).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ONE_HOT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.IndicesTensor == other.IndicesTensor && self.ValuesTensor == other.ValuesTensor && self.OutputTensor == other.OutputTensor && self.Axis == other.Axis
    }
}
impl ::core::cmp::Eq for DML_ONE_HOT_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ONE_HOT_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_OPERATOR_DESC {
    pub Type: DML_OPERATOR_TYPE,
    pub Desc: *mut ::core::ffi::c_void,
}
impl DML_OPERATOR_DESC {}
impl ::core::default::Default for DML_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_OPERATOR_DESC").field("Type", &self.Type).field("Desc", &self.Desc).finish()
    }
}
impl ::core::cmp::PartialEq for DML_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Desc == other.Desc
    }
}
impl ::core::cmp::Eq for DML_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Foundation`*"]
pub struct DML_OPERATOR_GRAPH_NODE_DESC {
    pub Operator: ::core::option::Option<IDMLOperator>,
    pub Name: super::super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl DML_OPERATOR_GRAPH_NODE_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_OPERATOR_GRAPH_NODE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DML_OPERATOR_GRAPH_NODE_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_OPERATOR_GRAPH_NODE_DESC").field("Operator", &self.Operator).field("Name", &self.Name).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_OPERATOR_GRAPH_NODE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Operator == other.Operator && self.Name == other.Name
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_OPERATOR_GRAPH_NODE_DESC {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DML_OPERATOR_GRAPH_NODE_DESC {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DML_OPERATOR_TYPE(pub i32);
pub const DML_OPERATOR_INVALID: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(0i32);
pub const DML_OPERATOR_ELEMENT_WISE_IDENTITY: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(1i32);
pub const DML_OPERATOR_ELEMENT_WISE_ABS: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(2i32);
pub const DML_OPERATOR_ELEMENT_WISE_ACOS: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(3i32);
pub const DML_OPERATOR_ELEMENT_WISE_ADD: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(4i32);
pub const DML_OPERATOR_ELEMENT_WISE_ASIN: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(5i32);
pub const DML_OPERATOR_ELEMENT_WISE_ATAN: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(6i32);
pub const DML_OPERATOR_ELEMENT_WISE_CEIL: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(7i32);
pub const DML_OPERATOR_ELEMENT_WISE_CLIP: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(8i32);
pub const DML_OPERATOR_ELEMENT_WISE_COS: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(9i32);
pub const DML_OPERATOR_ELEMENT_WISE_DIVIDE: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(10i32);
pub const DML_OPERATOR_ELEMENT_WISE_EXP: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(11i32);
pub const DML_OPERATOR_ELEMENT_WISE_FLOOR: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(12i32);
pub const DML_OPERATOR_ELEMENT_WISE_LOG: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(13i32);
pub const DML_OPERATOR_ELEMENT_WISE_LOGICAL_AND: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(14i32);
pub const DML_OPERATOR_ELEMENT_WISE_LOGICAL_EQUALS: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(15i32);
pub const DML_OPERATOR_ELEMENT_WISE_LOGICAL_GREATER_THAN: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(16i32);
pub const DML_OPERATOR_ELEMENT_WISE_LOGICAL_LESS_THAN: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(17i32);
pub const DML_OPERATOR_ELEMENT_WISE_LOGICAL_NOT: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(18i32);
pub const DML_OPERATOR_ELEMENT_WISE_LOGICAL_OR: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(19i32);
pub const DML_OPERATOR_ELEMENT_WISE_LOGICAL_XOR: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(20i32);
pub const DML_OPERATOR_ELEMENT_WISE_MAX: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(21i32);
pub const DML_OPERATOR_ELEMENT_WISE_MEAN: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(22i32);
pub const DML_OPERATOR_ELEMENT_WISE_MIN: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(23i32);
pub const DML_OPERATOR_ELEMENT_WISE_MULTIPLY: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(24i32);
pub const DML_OPERATOR_ELEMENT_WISE_POW: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(25i32);
pub const DML_OPERATOR_ELEMENT_WISE_CONSTANT_POW: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(26i32);
pub const DML_OPERATOR_ELEMENT_WISE_RECIP: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(27i32);
pub const DML_OPERATOR_ELEMENT_WISE_SIN: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(28i32);
pub const DML_OPERATOR_ELEMENT_WISE_SQRT: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(29i32);
pub const DML_OPERATOR_ELEMENT_WISE_SUBTRACT: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(30i32);
pub const DML_OPERATOR_ELEMENT_WISE_TAN: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(31i32);
pub const DML_OPERATOR_ELEMENT_WISE_THRESHOLD: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(32i32);
pub const DML_OPERATOR_ELEMENT_WISE_QUANTIZE_LINEAR: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(33i32);
pub const DML_OPERATOR_ELEMENT_WISE_DEQUANTIZE_LINEAR: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(34i32);
pub const DML_OPERATOR_ACTIVATION_ELU: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(35i32);
pub const DML_OPERATOR_ACTIVATION_HARDMAX: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(36i32);
pub const DML_OPERATOR_ACTIVATION_HARD_SIGMOID: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(37i32);
pub const DML_OPERATOR_ACTIVATION_IDENTITY: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(38i32);
pub const DML_OPERATOR_ACTIVATION_LEAKY_RELU: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(39i32);
pub const DML_OPERATOR_ACTIVATION_LINEAR: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(40i32);
pub const DML_OPERATOR_ACTIVATION_LOG_SOFTMAX: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(41i32);
pub const DML_OPERATOR_ACTIVATION_PARAMETERIZED_RELU: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(42i32);
pub const DML_OPERATOR_ACTIVATION_PARAMETRIC_SOFTPLUS: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(43i32);
pub const DML_OPERATOR_ACTIVATION_RELU: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(44i32);
pub const DML_OPERATOR_ACTIVATION_SCALED_ELU: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(45i32);
pub const DML_OPERATOR_ACTIVATION_SCALED_TANH: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(46i32);
pub const DML_OPERATOR_ACTIVATION_SIGMOID: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(47i32);
pub const DML_OPERATOR_ACTIVATION_SOFTMAX: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(48i32);
pub const DML_OPERATOR_ACTIVATION_SOFTPLUS: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(49i32);
pub const DML_OPERATOR_ACTIVATION_SOFTSIGN: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(50i32);
pub const DML_OPERATOR_ACTIVATION_TANH: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(51i32);
pub const DML_OPERATOR_ACTIVATION_THRESHOLDED_RELU: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(52i32);
pub const DML_OPERATOR_CONVOLUTION: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(53i32);
pub const DML_OPERATOR_GEMM: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(54i32);
pub const DML_OPERATOR_REDUCE: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(55i32);
pub const DML_OPERATOR_AVERAGE_POOLING: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(56i32);
pub const DML_OPERATOR_LP_POOLING: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(57i32);
pub const DML_OPERATOR_MAX_POOLING: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(58i32);
pub const DML_OPERATOR_ROI_POOLING: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(59i32);
pub const DML_OPERATOR_SLICE: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(60i32);
pub const DML_OPERATOR_CAST: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(61i32);
pub const DML_OPERATOR_SPLIT: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(62i32);
pub const DML_OPERATOR_JOIN: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(63i32);
pub const DML_OPERATOR_PADDING: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(64i32);
pub const DML_OPERATOR_VALUE_SCALE_2D: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(65i32);
pub const DML_OPERATOR_UPSAMPLE_2D: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(66i32);
pub const DML_OPERATOR_GATHER: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(67i32);
pub const DML_OPERATOR_SPACE_TO_DEPTH: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(68i32);
pub const DML_OPERATOR_DEPTH_TO_SPACE: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(69i32);
pub const DML_OPERATOR_TILE: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(70i32);
pub const DML_OPERATOR_TOP_K: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(71i32);
pub const DML_OPERATOR_BATCH_NORMALIZATION: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(72i32);
pub const DML_OPERATOR_MEAN_VARIANCE_NORMALIZATION: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(73i32);
pub const DML_OPERATOR_LOCAL_RESPONSE_NORMALIZATION: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(74i32);
pub const DML_OPERATOR_LP_NORMALIZATION: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(75i32);
pub const DML_OPERATOR_RNN: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(76i32);
pub const DML_OPERATOR_LSTM: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(77i32);
pub const DML_OPERATOR_GRU: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(78i32);
pub const DML_OPERATOR_ELEMENT_WISE_SIGN: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(79i32);
pub const DML_OPERATOR_ELEMENT_WISE_IS_NAN: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(80i32);
pub const DML_OPERATOR_ELEMENT_WISE_ERF: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(81i32);
pub const DML_OPERATOR_ELEMENT_WISE_SINH: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(82i32);
pub const DML_OPERATOR_ELEMENT_WISE_COSH: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(83i32);
pub const DML_OPERATOR_ELEMENT_WISE_TANH: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(84i32);
pub const DML_OPERATOR_ELEMENT_WISE_ASINH: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(85i32);
pub const DML_OPERATOR_ELEMENT_WISE_ACOSH: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(86i32);
pub const DML_OPERATOR_ELEMENT_WISE_ATANH: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(87i32);
pub const DML_OPERATOR_ELEMENT_WISE_IF: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(88i32);
pub const DML_OPERATOR_ELEMENT_WISE_ADD1: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(89i32);
pub const DML_OPERATOR_ACTIVATION_SHRINK: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(90i32);
pub const DML_OPERATOR_MAX_POOLING1: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(91i32);
pub const DML_OPERATOR_MAX_UNPOOLING: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(92i32);
pub const DML_OPERATOR_DIAGONAL_MATRIX: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(93i32);
pub const DML_OPERATOR_SCATTER_ELEMENTS: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(94i32);
pub const DML_OPERATOR_SCATTER: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(94i32);
pub const DML_OPERATOR_ONE_HOT: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(95i32);
pub const DML_OPERATOR_RESAMPLE: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(96i32);
pub const DML_OPERATOR_ELEMENT_WISE_BIT_SHIFT_LEFT: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(97i32);
pub const DML_OPERATOR_ELEMENT_WISE_BIT_SHIFT_RIGHT: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(98i32);
pub const DML_OPERATOR_ELEMENT_WISE_ROUND: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(99i32);
pub const DML_OPERATOR_ELEMENT_WISE_IS_INFINITY: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(100i32);
pub const DML_OPERATOR_ELEMENT_WISE_MODULUS_TRUNCATE: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(101i32);
pub const DML_OPERATOR_ELEMENT_WISE_MODULUS_FLOOR: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(102i32);
pub const DML_OPERATOR_FILL_VALUE_CONSTANT: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(103i32);
pub const DML_OPERATOR_FILL_VALUE_SEQUENCE: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(104i32);
pub const DML_OPERATOR_CUMULATIVE_SUMMATION: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(105i32);
pub const DML_OPERATOR_REVERSE_SUBSEQUENCES: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(106i32);
pub const DML_OPERATOR_GATHER_ELEMENTS: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(107i32);
pub const DML_OPERATOR_GATHER_ND: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(108i32);
pub const DML_OPERATOR_SCATTER_ND: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(109i32);
pub const DML_OPERATOR_MAX_POOLING2: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(110i32);
pub const DML_OPERATOR_SLICE1: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(111i32);
pub const DML_OPERATOR_TOP_K1: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(112i32);
pub const DML_OPERATOR_DEPTH_TO_SPACE1: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(113i32);
pub const DML_OPERATOR_SPACE_TO_DEPTH1: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(114i32);
pub const DML_OPERATOR_MEAN_VARIANCE_NORMALIZATION1: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(115i32);
pub const DML_OPERATOR_RESAMPLE1: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(116i32);
pub const DML_OPERATOR_MATRIX_MULTIPLY_INTEGER: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(117i32);
pub const DML_OPERATOR_QUANTIZED_LINEAR_MATRIX_MULTIPLY: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(118i32);
pub const DML_OPERATOR_CONVOLUTION_INTEGER: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(119i32);
pub const DML_OPERATOR_QUANTIZED_LINEAR_CONVOLUTION: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(120i32);
pub const DML_OPERATOR_ELEMENT_WISE_BIT_AND: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(121i32);
pub const DML_OPERATOR_ELEMENT_WISE_BIT_OR: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(122i32);
pub const DML_OPERATOR_ELEMENT_WISE_BIT_XOR: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(123i32);
pub const DML_OPERATOR_ELEMENT_WISE_BIT_NOT: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(124i32);
pub const DML_OPERATOR_ELEMENT_WISE_BIT_COUNT: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(125i32);
pub const DML_OPERATOR_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(126i32);
pub const DML_OPERATOR_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(127i32);
pub const DML_OPERATOR_ACTIVATION_CELU: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(128i32);
pub const DML_OPERATOR_ACTIVATION_RELU_GRAD: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(129i32);
pub const DML_OPERATOR_AVERAGE_POOLING_GRAD: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(130i32);
pub const DML_OPERATOR_MAX_POOLING_GRAD: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(131i32);
pub const DML_OPERATOR_RANDOM_GENERATOR: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(132i32);
pub const DML_OPERATOR_NONZERO_COORDINATES: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(133i32);
pub const DML_OPERATOR_RESAMPLE_GRAD: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(134i32);
pub const DML_OPERATOR_SLICE_GRAD: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(135i32);
pub const DML_OPERATOR_ADAM_OPTIMIZER: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(136i32);
pub const DML_OPERATOR_ARGMIN: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(137i32);
pub const DML_OPERATOR_ARGMAX: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(138i32);
pub const DML_OPERATOR_ROI_ALIGN: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(139i32);
pub const DML_OPERATOR_GATHER_ND1: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(140i32);
pub const DML_OPERATOR_ELEMENT_WISE_ATAN_YX: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(141i32);
pub const DML_OPERATOR_ELEMENT_WISE_CLIP_GRAD: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(142i32);
pub const DML_OPERATOR_ELEMENT_WISE_DIFFERENCE_SQUARE: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(143i32);
pub const DML_OPERATOR_LOCAL_RESPONSE_NORMALIZATION_GRAD: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(144i32);
pub const DML_OPERATOR_CUMULATIVE_PRODUCT: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(145i32);
pub const DML_OPERATOR_BATCH_NORMALIZATION_GRAD: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(146i32);
pub const DML_OPERATOR_ELEMENT_WISE_QUANTIZED_LINEAR_ADD: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(147i32);
pub const DML_OPERATOR_DYNAMIC_QUANTIZE_LINEAR: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(148i32);
pub const DML_OPERATOR_ROI_ALIGN1: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(149i32);
impl ::core::convert::From<i32> for DML_OPERATOR_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DML_OPERATOR_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Foundation`*"]
pub struct DML_OUTPUT_GRAPH_EDGE_DESC {
    pub FromNodeIndex: u32,
    pub FromNodeOutputIndex: u32,
    pub GraphOutputIndex: u32,
    pub Name: super::super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl DML_OUTPUT_GRAPH_EDGE_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_OUTPUT_GRAPH_EDGE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DML_OUTPUT_GRAPH_EDGE_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_OUTPUT_GRAPH_EDGE_DESC").field("FromNodeIndex", &self.FromNodeIndex).field("FromNodeOutputIndex", &self.FromNodeOutputIndex).field("GraphOutputIndex", &self.GraphOutputIndex).field("Name", &self.Name).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_OUTPUT_GRAPH_EDGE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.FromNodeIndex == other.FromNodeIndex && self.FromNodeOutputIndex == other.FromNodeOutputIndex && self.GraphOutputIndex == other.GraphOutputIndex && self.Name == other.Name
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_OUTPUT_GRAPH_EDGE_DESC {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DML_OUTPUT_GRAPH_EDGE_DESC {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DML_PADDING_MODE(pub i32);
pub const DML_PADDING_MODE_CONSTANT: DML_PADDING_MODE = DML_PADDING_MODE(0i32);
pub const DML_PADDING_MODE_EDGE: DML_PADDING_MODE = DML_PADDING_MODE(1i32);
pub const DML_PADDING_MODE_REFLECTION: DML_PADDING_MODE = DML_PADDING_MODE(2i32);
pub const DML_PADDING_MODE_SYMMETRIC: DML_PADDING_MODE = DML_PADDING_MODE(3i32);
impl ::core::convert::From<i32> for DML_PADDING_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DML_PADDING_MODE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_PADDING_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub PaddingMode: DML_PADDING_MODE,
    pub PaddingValue: f32,
    pub DimensionCount: u32,
    pub StartPadding: *mut u32,
    pub EndPadding: *mut u32,
}
impl DML_PADDING_OPERATOR_DESC {}
impl ::core::default::Default for DML_PADDING_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_PADDING_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_PADDING_OPERATOR_DESC")
            .field("InputTensor", &self.InputTensor)
            .field("OutputTensor", &self.OutputTensor)
            .field("PaddingMode", &self.PaddingMode)
            .field("PaddingValue", &self.PaddingValue)
            .field("DimensionCount", &self.DimensionCount)
            .field("StartPadding", &self.StartPadding)
            .field("EndPadding", &self.EndPadding)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DML_PADDING_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.PaddingMode == other.PaddingMode && self.PaddingValue == other.PaddingValue && self.DimensionCount == other.DimensionCount && self.StartPadding == other.StartPadding && self.EndPadding == other.EndPadding
    }
}
impl ::core::cmp::Eq for DML_PADDING_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_PADDING_OPERATOR_DESC {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub const DML_PERSISTENT_BUFFER_ALIGNMENT: u32 = 256u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_QUANTIZED_LINEAR_CONVOLUTION_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub InputScaleTensor: *mut DML_TENSOR_DESC,
    pub InputZeroPointTensor: *mut DML_TENSOR_DESC,
    pub FilterTensor: *mut DML_TENSOR_DESC,
    pub FilterScaleTensor: *mut DML_TENSOR_DESC,
    pub FilterZeroPointTensor: *mut DML_TENSOR_DESC,
    pub BiasTensor: *mut DML_TENSOR_DESC,
    pub OutputScaleTensor: *mut DML_TENSOR_DESC,
    pub OutputZeroPointTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub Strides: *mut u32,
    pub Dilations: *mut u32,
    pub StartPadding: *mut u32,
    pub EndPadding: *mut u32,
    pub GroupCount: u32,
}
impl DML_QUANTIZED_LINEAR_CONVOLUTION_OPERATOR_DESC {}
impl ::core::default::Default for DML_QUANTIZED_LINEAR_CONVOLUTION_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_QUANTIZED_LINEAR_CONVOLUTION_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_QUANTIZED_LINEAR_CONVOLUTION_OPERATOR_DESC")
            .field("InputTensor", &self.InputTensor)
            .field("InputScaleTensor", &self.InputScaleTensor)
            .field("InputZeroPointTensor", &self.InputZeroPointTensor)
            .field("FilterTensor", &self.FilterTensor)
            .field("FilterScaleTensor", &self.FilterScaleTensor)
            .field("FilterZeroPointTensor", &self.FilterZeroPointTensor)
            .field("BiasTensor", &self.BiasTensor)
            .field("OutputScaleTensor", &self.OutputScaleTensor)
            .field("OutputZeroPointTensor", &self.OutputZeroPointTensor)
            .field("OutputTensor", &self.OutputTensor)
            .field("DimensionCount", &self.DimensionCount)
            .field("Strides", &self.Strides)
            .field("Dilations", &self.Dilations)
            .field("StartPadding", &self.StartPadding)
            .field("EndPadding", &self.EndPadding)
            .field("GroupCount", &self.GroupCount)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DML_QUANTIZED_LINEAR_CONVOLUTION_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor
            && self.InputScaleTensor == other.InputScaleTensor
            && self.InputZeroPointTensor == other.InputZeroPointTensor
            && self.FilterTensor == other.FilterTensor
            && self.FilterScaleTensor == other.FilterScaleTensor
            && self.FilterZeroPointTensor == other.FilterZeroPointTensor
            && self.BiasTensor == other.BiasTensor
            && self.OutputScaleTensor == other.OutputScaleTensor
            && self.OutputZeroPointTensor == other.OutputZeroPointTensor
            && self.OutputTensor == other.OutputTensor
            && self.DimensionCount == other.DimensionCount
            && self.Strides == other.Strides
            && self.Dilations == other.Dilations
            && self.StartPadding == other.StartPadding
            && self.EndPadding == other.EndPadding
            && self.GroupCount == other.GroupCount
    }
}
impl ::core::cmp::Eq for DML_QUANTIZED_LINEAR_CONVOLUTION_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_QUANTIZED_LINEAR_CONVOLUTION_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_QUANTIZED_LINEAR_MATRIX_MULTIPLY_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub AScaleTensor: *mut DML_TENSOR_DESC,
    pub AZeroPointTensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub BScaleTensor: *mut DML_TENSOR_DESC,
    pub BZeroPointTensor: *mut DML_TENSOR_DESC,
    pub OutputScaleTensor: *mut DML_TENSOR_DESC,
    pub OutputZeroPointTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl DML_QUANTIZED_LINEAR_MATRIX_MULTIPLY_OPERATOR_DESC {}
impl ::core::default::Default for DML_QUANTIZED_LINEAR_MATRIX_MULTIPLY_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_QUANTIZED_LINEAR_MATRIX_MULTIPLY_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_QUANTIZED_LINEAR_MATRIX_MULTIPLY_OPERATOR_DESC")
            .field("ATensor", &self.ATensor)
            .field("AScaleTensor", &self.AScaleTensor)
            .field("AZeroPointTensor", &self.AZeroPointTensor)
            .field("BTensor", &self.BTensor)
            .field("BScaleTensor", &self.BScaleTensor)
            .field("BZeroPointTensor", &self.BZeroPointTensor)
            .field("OutputScaleTensor", &self.OutputScaleTensor)
            .field("OutputZeroPointTensor", &self.OutputZeroPointTensor)
            .field("OutputTensor", &self.OutputTensor)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DML_QUANTIZED_LINEAR_MATRIX_MULTIPLY_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.AScaleTensor == other.AScaleTensor && self.AZeroPointTensor == other.AZeroPointTensor && self.BTensor == other.BTensor && self.BScaleTensor == other.BScaleTensor && self.BZeroPointTensor == other.BZeroPointTensor && self.OutputScaleTensor == other.OutputScaleTensor && self.OutputZeroPointTensor == other.OutputZeroPointTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_QUANTIZED_LINEAR_MATRIX_MULTIPLY_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_QUANTIZED_LINEAR_MATRIX_MULTIPLY_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_RANDOM_GENERATOR_OPERATOR_DESC {
    pub InputStateTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub OutputStateTensor: *mut DML_TENSOR_DESC,
    pub Type: DML_RANDOM_GENERATOR_TYPE,
}
impl DML_RANDOM_GENERATOR_OPERATOR_DESC {}
impl ::core::default::Default for DML_RANDOM_GENERATOR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_RANDOM_GENERATOR_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_RANDOM_GENERATOR_OPERATOR_DESC").field("InputStateTensor", &self.InputStateTensor).field("OutputTensor", &self.OutputTensor).field("OutputStateTensor", &self.OutputStateTensor).field("Type", &self.Type).finish()
    }
}
impl ::core::cmp::PartialEq for DML_RANDOM_GENERATOR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputStateTensor == other.InputStateTensor && self.OutputTensor == other.OutputTensor && self.OutputStateTensor == other.OutputStateTensor && self.Type == other.Type
    }
}
impl ::core::cmp::Eq for DML_RANDOM_GENERATOR_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_RANDOM_GENERATOR_OPERATOR_DESC {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DML_RANDOM_GENERATOR_TYPE(pub i32);
pub const DML_RANDOM_GENERATOR_TYPE_PHILOX_4X32_10: DML_RANDOM_GENERATOR_TYPE = DML_RANDOM_GENERATOR_TYPE(0i32);
impl ::core::convert::From<i32> for DML_RANDOM_GENERATOR_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DML_RANDOM_GENERATOR_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DML_RECURRENT_NETWORK_DIRECTION(pub i32);
pub const DML_RECURRENT_NETWORK_DIRECTION_FORWARD: DML_RECURRENT_NETWORK_DIRECTION = DML_RECURRENT_NETWORK_DIRECTION(0i32);
pub const DML_RECURRENT_NETWORK_DIRECTION_BACKWARD: DML_RECURRENT_NETWORK_DIRECTION = DML_RECURRENT_NETWORK_DIRECTION(1i32);
pub const DML_RECURRENT_NETWORK_DIRECTION_BIDIRECTIONAL: DML_RECURRENT_NETWORK_DIRECTION = DML_RECURRENT_NETWORK_DIRECTION(2i32);
impl ::core::convert::From<i32> for DML_RECURRENT_NETWORK_DIRECTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DML_RECURRENT_NETWORK_DIRECTION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DML_REDUCE_FUNCTION(pub i32);
pub const DML_REDUCE_FUNCTION_ARGMAX: DML_REDUCE_FUNCTION = DML_REDUCE_FUNCTION(0i32);
pub const DML_REDUCE_FUNCTION_ARGMIN: DML_REDUCE_FUNCTION = DML_REDUCE_FUNCTION(1i32);
pub const DML_REDUCE_FUNCTION_AVERAGE: DML_REDUCE_FUNCTION = DML_REDUCE_FUNCTION(2i32);
pub const DML_REDUCE_FUNCTION_L1: DML_REDUCE_FUNCTION = DML_REDUCE_FUNCTION(3i32);
pub const DML_REDUCE_FUNCTION_L2: DML_REDUCE_FUNCTION = DML_REDUCE_FUNCTION(4i32);
pub const DML_REDUCE_FUNCTION_LOG_SUM: DML_REDUCE_FUNCTION = DML_REDUCE_FUNCTION(5i32);
pub const DML_REDUCE_FUNCTION_LOG_SUM_EXP: DML_REDUCE_FUNCTION = DML_REDUCE_FUNCTION(6i32);
pub const DML_REDUCE_FUNCTION_MAX: DML_REDUCE_FUNCTION = DML_REDUCE_FUNCTION(7i32);
pub const DML_REDUCE_FUNCTION_MIN: DML_REDUCE_FUNCTION = DML_REDUCE_FUNCTION(8i32);
pub const DML_REDUCE_FUNCTION_MULTIPLY: DML_REDUCE_FUNCTION = DML_REDUCE_FUNCTION(9i32);
pub const DML_REDUCE_FUNCTION_SUM: DML_REDUCE_FUNCTION = DML_REDUCE_FUNCTION(10i32);
pub const DML_REDUCE_FUNCTION_SUM_SQUARE: DML_REDUCE_FUNCTION = DML_REDUCE_FUNCTION(11i32);
impl ::core::convert::From<i32> for DML_REDUCE_FUNCTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DML_REDUCE_FUNCTION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_REDUCE_OPERATOR_DESC {
    pub Function: DML_REDUCE_FUNCTION,
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub AxisCount: u32,
    pub Axes: *mut u32,
}
impl DML_REDUCE_OPERATOR_DESC {}
impl ::core::default::Default for DML_REDUCE_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_REDUCE_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_REDUCE_OPERATOR_DESC").field("Function", &self.Function).field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("AxisCount", &self.AxisCount).field("Axes", &self.Axes).finish()
    }
}
impl ::core::cmp::PartialEq for DML_REDUCE_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Function == other.Function && self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.AxisCount == other.AxisCount && self.Axes == other.Axes
    }
}
impl ::core::cmp::Eq for DML_REDUCE_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_REDUCE_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_RESAMPLE1_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub InterpolationMode: DML_INTERPOLATION_MODE,
    pub DimensionCount: u32,
    pub Scales: *mut f32,
    pub InputPixelOffsets: *mut f32,
    pub OutputPixelOffsets: *mut f32,
}
impl DML_RESAMPLE1_OPERATOR_DESC {}
impl ::core::default::Default for DML_RESAMPLE1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_RESAMPLE1_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_RESAMPLE1_OPERATOR_DESC")
            .field("InputTensor", &self.InputTensor)
            .field("OutputTensor", &self.OutputTensor)
            .field("InterpolationMode", &self.InterpolationMode)
            .field("DimensionCount", &self.DimensionCount)
            .field("Scales", &self.Scales)
            .field("InputPixelOffsets", &self.InputPixelOffsets)
            .field("OutputPixelOffsets", &self.OutputPixelOffsets)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DML_RESAMPLE1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.InterpolationMode == other.InterpolationMode && self.DimensionCount == other.DimensionCount && self.Scales == other.Scales && self.InputPixelOffsets == other.InputPixelOffsets && self.OutputPixelOffsets == other.OutputPixelOffsets
    }
}
impl ::core::cmp::Eq for DML_RESAMPLE1_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_RESAMPLE1_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_RESAMPLE_GRAD_OPERATOR_DESC {
    pub InputGradientTensor: *mut DML_TENSOR_DESC,
    pub OutputGradientTensor: *mut DML_TENSOR_DESC,
    pub InterpolationMode: DML_INTERPOLATION_MODE,
    pub DimensionCount: u32,
    pub Scales: *mut f32,
    pub InputPixelOffsets: *mut f32,
    pub OutputPixelOffsets: *mut f32,
}
impl DML_RESAMPLE_GRAD_OPERATOR_DESC {}
impl ::core::default::Default for DML_RESAMPLE_GRAD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_RESAMPLE_GRAD_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_RESAMPLE_GRAD_OPERATOR_DESC")
            .field("InputGradientTensor", &self.InputGradientTensor)
            .field("OutputGradientTensor", &self.OutputGradientTensor)
            .field("InterpolationMode", &self.InterpolationMode)
            .field("DimensionCount", &self.DimensionCount)
            .field("Scales", &self.Scales)
            .field("InputPixelOffsets", &self.InputPixelOffsets)
            .field("OutputPixelOffsets", &self.OutputPixelOffsets)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DML_RESAMPLE_GRAD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputGradientTensor == other.InputGradientTensor && self.OutputGradientTensor == other.OutputGradientTensor && self.InterpolationMode == other.InterpolationMode && self.DimensionCount == other.DimensionCount && self.Scales == other.Scales && self.InputPixelOffsets == other.InputPixelOffsets && self.OutputPixelOffsets == other.OutputPixelOffsets
    }
}
impl ::core::cmp::Eq for DML_RESAMPLE_GRAD_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_RESAMPLE_GRAD_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_RESAMPLE_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub InterpolationMode: DML_INTERPOLATION_MODE,
    pub ScaleCount: u32,
    pub Scales: *mut f32,
}
impl DML_RESAMPLE_OPERATOR_DESC {}
impl ::core::default::Default for DML_RESAMPLE_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_RESAMPLE_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_RESAMPLE_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("InterpolationMode", &self.InterpolationMode).field("ScaleCount", &self.ScaleCount).field("Scales", &self.Scales).finish()
    }
}
impl ::core::cmp::PartialEq for DML_RESAMPLE_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.InterpolationMode == other.InterpolationMode && self.ScaleCount == other.ScaleCount && self.Scales == other.Scales
    }
}
impl ::core::cmp::Eq for DML_RESAMPLE_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_RESAMPLE_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_REVERSE_SUBSEQUENCES_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub SequenceLengthsTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Axis: u32,
}
impl DML_REVERSE_SUBSEQUENCES_OPERATOR_DESC {}
impl ::core::default::Default for DML_REVERSE_SUBSEQUENCES_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_REVERSE_SUBSEQUENCES_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_REVERSE_SUBSEQUENCES_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("SequenceLengthsTensor", &self.SequenceLengthsTensor).field("OutputTensor", &self.OutputTensor).field("Axis", &self.Axis).finish()
    }
}
impl ::core::cmp::PartialEq for DML_REVERSE_SUBSEQUENCES_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.SequenceLengthsTensor == other.SequenceLengthsTensor && self.OutputTensor == other.OutputTensor && self.Axis == other.Axis
    }
}
impl ::core::cmp::Eq for DML_REVERSE_SUBSEQUENCES_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_REVERSE_SUBSEQUENCES_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_RNN_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub WeightTensor: *mut DML_TENSOR_DESC,
    pub RecurrenceTensor: *mut DML_TENSOR_DESC,
    pub BiasTensor: *mut DML_TENSOR_DESC,
    pub HiddenInitTensor: *mut DML_TENSOR_DESC,
    pub SequenceLengthsTensor: *mut DML_TENSOR_DESC,
    pub OutputSequenceTensor: *mut DML_TENSOR_DESC,
    pub OutputSingleTensor: *mut DML_TENSOR_DESC,
    pub ActivationDescCount: u32,
    pub ActivationDescs: *mut DML_OPERATOR_DESC,
    pub Direction: DML_RECURRENT_NETWORK_DIRECTION,
}
impl DML_RNN_OPERATOR_DESC {}
impl ::core::default::Default for DML_RNN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_RNN_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_RNN_OPERATOR_DESC")
            .field("InputTensor", &self.InputTensor)
            .field("WeightTensor", &self.WeightTensor)
            .field("RecurrenceTensor", &self.RecurrenceTensor)
            .field("BiasTensor", &self.BiasTensor)
            .field("HiddenInitTensor", &self.HiddenInitTensor)
            .field("SequenceLengthsTensor", &self.SequenceLengthsTensor)
            .field("OutputSequenceTensor", &self.OutputSequenceTensor)
            .field("OutputSingleTensor", &self.OutputSingleTensor)
            .field("ActivationDescCount", &self.ActivationDescCount)
            .field("ActivationDescs", &self.ActivationDescs)
            .field("Direction", &self.Direction)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DML_RNN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor
            && self.WeightTensor == other.WeightTensor
            && self.RecurrenceTensor == other.RecurrenceTensor
            && self.BiasTensor == other.BiasTensor
            && self.HiddenInitTensor == other.HiddenInitTensor
            && self.SequenceLengthsTensor == other.SequenceLengthsTensor
            && self.OutputSequenceTensor == other.OutputSequenceTensor
            && self.OutputSingleTensor == other.OutputSingleTensor
            && self.ActivationDescCount == other.ActivationDescCount
            && self.ActivationDescs == other.ActivationDescs
            && self.Direction == other.Direction
    }
}
impl ::core::cmp::Eq for DML_RNN_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_RNN_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Foundation`*"]
pub struct DML_ROI_ALIGN1_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub ROITensor: *mut DML_TENSOR_DESC,
    pub BatchIndicesTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ReductionFunction: DML_REDUCE_FUNCTION,
    pub InterpolationMode: DML_INTERPOLATION_MODE,
    pub SpatialScaleX: f32,
    pub SpatialScaleY: f32,
    pub InputPixelOffset: f32,
    pub OutputPixelOffset: f32,
    pub OutOfBoundsInputValue: f32,
    pub MinimumSamplesPerOutput: u32,
    pub MaximumSamplesPerOutput: u32,
    pub AlignRegionsToCorners: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl DML_ROI_ALIGN1_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_ROI_ALIGN1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DML_ROI_ALIGN1_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ROI_ALIGN1_OPERATOR_DESC")
            .field("InputTensor", &self.InputTensor)
            .field("ROITensor", &self.ROITensor)
            .field("BatchIndicesTensor", &self.BatchIndicesTensor)
            .field("OutputTensor", &self.OutputTensor)
            .field("ReductionFunction", &self.ReductionFunction)
            .field("InterpolationMode", &self.InterpolationMode)
            .field("SpatialScaleX", &self.SpatialScaleX)
            .field("SpatialScaleY", &self.SpatialScaleY)
            .field("InputPixelOffset", &self.InputPixelOffset)
            .field("OutputPixelOffset", &self.OutputPixelOffset)
            .field("OutOfBoundsInputValue", &self.OutOfBoundsInputValue)
            .field("MinimumSamplesPerOutput", &self.MinimumSamplesPerOutput)
            .field("MaximumSamplesPerOutput", &self.MaximumSamplesPerOutput)
            .field("AlignRegionsToCorners", &self.AlignRegionsToCorners)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_ROI_ALIGN1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor
            && self.ROITensor == other.ROITensor
            && self.BatchIndicesTensor == other.BatchIndicesTensor
            && self.OutputTensor == other.OutputTensor
            && self.ReductionFunction == other.ReductionFunction
            && self.InterpolationMode == other.InterpolationMode
            && self.SpatialScaleX == other.SpatialScaleX
            && self.SpatialScaleY == other.SpatialScaleY
            && self.InputPixelOffset == other.InputPixelOffset
            && self.OutputPixelOffset == other.OutputPixelOffset
            && self.OutOfBoundsInputValue == other.OutOfBoundsInputValue
            && self.MinimumSamplesPerOutput == other.MinimumSamplesPerOutput
            && self.MaximumSamplesPerOutput == other.MaximumSamplesPerOutput
            && self.AlignRegionsToCorners == other.AlignRegionsToCorners
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_ROI_ALIGN1_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DML_ROI_ALIGN1_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ROI_ALIGN_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub ROITensor: *mut DML_TENSOR_DESC,
    pub BatchIndicesTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ReductionFunction: DML_REDUCE_FUNCTION,
    pub InterpolationMode: DML_INTERPOLATION_MODE,
    pub SpatialScaleX: f32,
    pub SpatialScaleY: f32,
    pub OutOfBoundsInputValue: f32,
    pub MinimumSamplesPerOutput: u32,
    pub MaximumSamplesPerOutput: u32,
}
impl DML_ROI_ALIGN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ROI_ALIGN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ROI_ALIGN_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ROI_ALIGN_OPERATOR_DESC")
            .field("InputTensor", &self.InputTensor)
            .field("ROITensor", &self.ROITensor)
            .field("BatchIndicesTensor", &self.BatchIndicesTensor)
            .field("OutputTensor", &self.OutputTensor)
            .field("ReductionFunction", &self.ReductionFunction)
            .field("InterpolationMode", &self.InterpolationMode)
            .field("SpatialScaleX", &self.SpatialScaleX)
            .field("SpatialScaleY", &self.SpatialScaleY)
            .field("OutOfBoundsInputValue", &self.OutOfBoundsInputValue)
            .field("MinimumSamplesPerOutput", &self.MinimumSamplesPerOutput)
            .field("MaximumSamplesPerOutput", &self.MaximumSamplesPerOutput)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DML_ROI_ALIGN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor
            && self.ROITensor == other.ROITensor
            && self.BatchIndicesTensor == other.BatchIndicesTensor
            && self.OutputTensor == other.OutputTensor
            && self.ReductionFunction == other.ReductionFunction
            && self.InterpolationMode == other.InterpolationMode
            && self.SpatialScaleX == other.SpatialScaleX
            && self.SpatialScaleY == other.SpatialScaleY
            && self.OutOfBoundsInputValue == other.OutOfBoundsInputValue
            && self.MinimumSamplesPerOutput == other.MinimumSamplesPerOutput
            && self.MaximumSamplesPerOutput == other.MaximumSamplesPerOutput
    }
}
impl ::core::cmp::Eq for DML_ROI_ALIGN_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ROI_ALIGN_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_ROI_POOLING_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub ROITensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub SpatialScale: f32,
    pub PooledSize: DML_SIZE_2D,
}
impl DML_ROI_POOLING_OPERATOR_DESC {}
impl ::core::default::Default for DML_ROI_POOLING_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_ROI_POOLING_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_ROI_POOLING_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("ROITensor", &self.ROITensor).field("OutputTensor", &self.OutputTensor).field("SpatialScale", &self.SpatialScale).field("PooledSize", &self.PooledSize).finish()
    }
}
impl ::core::cmp::PartialEq for DML_ROI_POOLING_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.ROITensor == other.ROITensor && self.OutputTensor == other.OutputTensor && self.SpatialScale == other.SpatialScale && self.PooledSize == other.PooledSize
    }
}
impl ::core::cmp::Eq for DML_ROI_POOLING_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_ROI_POOLING_OPERATOR_DESC {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DML_ROUNDING_MODE(pub i32);
pub const DML_ROUNDING_MODE_HALVES_TO_NEAREST_EVEN: DML_ROUNDING_MODE = DML_ROUNDING_MODE(0i32);
pub const DML_ROUNDING_MODE_TOWARD_ZERO: DML_ROUNDING_MODE = DML_ROUNDING_MODE(1i32);
pub const DML_ROUNDING_MODE_TOWARD_INFINITY: DML_ROUNDING_MODE = DML_ROUNDING_MODE(2i32);
impl ::core::convert::From<i32> for DML_ROUNDING_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DML_ROUNDING_MODE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub union DML_SCALAR_UNION {
    pub Bytes: [u8; 8],
    pub Int8: i8,
    pub UInt8: u8,
    pub Int16: i16,
    pub UInt16: u16,
    pub Int32: i32,
    pub UInt32: u32,
    pub Int64: i64,
    pub UInt64: u64,
    pub Float32: f32,
    pub Float64: f64,
}
impl DML_SCALAR_UNION {}
impl ::core::default::Default for DML_SCALAR_UNION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_SCALAR_UNION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for DML_SCALAR_UNION {}
unsafe impl ::windows::runtime::Abi for DML_SCALAR_UNION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_SCALE_BIAS {
    pub Scale: f32,
    pub Bias: f32,
}
impl DML_SCALE_BIAS {}
impl ::core::default::Default for DML_SCALE_BIAS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_SCALE_BIAS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_SCALE_BIAS").field("Scale", &self.Scale).field("Bias", &self.Bias).finish()
    }
}
impl ::core::cmp::PartialEq for DML_SCALE_BIAS {
    fn eq(&self, other: &Self) -> bool {
        self.Scale == other.Scale && self.Bias == other.Bias
    }
}
impl ::core::cmp::Eq for DML_SCALE_BIAS {}
unsafe impl ::windows::runtime::Abi for DML_SCALE_BIAS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_SCATTER_ND_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub IndicesTensor: *mut DML_TENSOR_DESC,
    pub UpdatesTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub InputDimensionCount: u32,
    pub IndicesDimensionCount: u32,
}
impl DML_SCATTER_ND_OPERATOR_DESC {}
impl ::core::default::Default for DML_SCATTER_ND_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_SCATTER_ND_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_SCATTER_ND_OPERATOR_DESC")
            .field("InputTensor", &self.InputTensor)
            .field("IndicesTensor", &self.IndicesTensor)
            .field("UpdatesTensor", &self.UpdatesTensor)
            .field("OutputTensor", &self.OutputTensor)
            .field("InputDimensionCount", &self.InputDimensionCount)
            .field("IndicesDimensionCount", &self.IndicesDimensionCount)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DML_SCATTER_ND_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.IndicesTensor == other.IndicesTensor && self.UpdatesTensor == other.UpdatesTensor && self.OutputTensor == other.OutputTensor && self.InputDimensionCount == other.InputDimensionCount && self.IndicesDimensionCount == other.IndicesDimensionCount
    }
}
impl ::core::cmp::Eq for DML_SCATTER_ND_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_SCATTER_ND_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_SCATTER_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub IndicesTensor: *mut DML_TENSOR_DESC,
    pub UpdatesTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Axis: u32,
}
impl DML_SCATTER_OPERATOR_DESC {}
impl ::core::default::Default for DML_SCATTER_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_SCATTER_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_SCATTER_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("IndicesTensor", &self.IndicesTensor).field("UpdatesTensor", &self.UpdatesTensor).field("OutputTensor", &self.OutputTensor).field("Axis", &self.Axis).finish()
    }
}
impl ::core::cmp::PartialEq for DML_SCATTER_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.IndicesTensor == other.IndicesTensor && self.UpdatesTensor == other.UpdatesTensor && self.OutputTensor == other.OutputTensor && self.Axis == other.Axis
    }
}
impl ::core::cmp::Eq for DML_SCATTER_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_SCATTER_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_SIZE_2D {
    pub Width: u32,
    pub Height: u32,
}
impl DML_SIZE_2D {}
impl ::core::default::Default for DML_SIZE_2D {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_SIZE_2D {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_SIZE_2D").field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
impl ::core::cmp::PartialEq for DML_SIZE_2D {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height
    }
}
impl ::core::cmp::Eq for DML_SIZE_2D {}
unsafe impl ::windows::runtime::Abi for DML_SIZE_2D {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_SLICE1_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub InputWindowOffsets: *mut u32,
    pub InputWindowSizes: *mut u32,
    pub InputWindowStrides: *mut i32,
}
impl DML_SLICE1_OPERATOR_DESC {}
impl ::core::default::Default for DML_SLICE1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_SLICE1_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_SLICE1_OPERATOR_DESC")
            .field("InputTensor", &self.InputTensor)
            .field("OutputTensor", &self.OutputTensor)
            .field("DimensionCount", &self.DimensionCount)
            .field("InputWindowOffsets", &self.InputWindowOffsets)
            .field("InputWindowSizes", &self.InputWindowSizes)
            .field("InputWindowStrides", &self.InputWindowStrides)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DML_SLICE1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.DimensionCount == other.DimensionCount && self.InputWindowOffsets == other.InputWindowOffsets && self.InputWindowSizes == other.InputWindowSizes && self.InputWindowStrides == other.InputWindowStrides
    }
}
impl ::core::cmp::Eq for DML_SLICE1_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_SLICE1_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_SLICE_GRAD_OPERATOR_DESC {
    pub InputGradientTensor: *mut DML_TENSOR_DESC,
    pub OutputGradientTensor: *mut DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub InputWindowOffsets: *mut u32,
    pub InputWindowSizes: *mut u32,
    pub InputWindowStrides: *mut i32,
}
impl DML_SLICE_GRAD_OPERATOR_DESC {}
impl ::core::default::Default for DML_SLICE_GRAD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_SLICE_GRAD_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_SLICE_GRAD_OPERATOR_DESC")
            .field("InputGradientTensor", &self.InputGradientTensor)
            .field("OutputGradientTensor", &self.OutputGradientTensor)
            .field("DimensionCount", &self.DimensionCount)
            .field("InputWindowOffsets", &self.InputWindowOffsets)
            .field("InputWindowSizes", &self.InputWindowSizes)
            .field("InputWindowStrides", &self.InputWindowStrides)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DML_SLICE_GRAD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputGradientTensor == other.InputGradientTensor && self.OutputGradientTensor == other.OutputGradientTensor && self.DimensionCount == other.DimensionCount && self.InputWindowOffsets == other.InputWindowOffsets && self.InputWindowSizes == other.InputWindowSizes && self.InputWindowStrides == other.InputWindowStrides
    }
}
impl ::core::cmp::Eq for DML_SLICE_GRAD_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_SLICE_GRAD_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_SLICE_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub Offsets: *mut u32,
    pub Sizes: *mut u32,
    pub Strides: *mut u32,
}
impl DML_SLICE_OPERATOR_DESC {}
impl ::core::default::Default for DML_SLICE_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_SLICE_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_SLICE_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("DimensionCount", &self.DimensionCount).field("Offsets", &self.Offsets).field("Sizes", &self.Sizes).field("Strides", &self.Strides).finish()
    }
}
impl ::core::cmp::PartialEq for DML_SLICE_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.DimensionCount == other.DimensionCount && self.Offsets == other.Offsets && self.Sizes == other.Sizes && self.Strides == other.Strides
    }
}
impl ::core::cmp::Eq for DML_SLICE_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_SLICE_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_SPACE_TO_DEPTH1_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub BlockSize: u32,
    pub Order: DML_DEPTH_SPACE_ORDER,
}
impl DML_SPACE_TO_DEPTH1_OPERATOR_DESC {}
impl ::core::default::Default for DML_SPACE_TO_DEPTH1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_SPACE_TO_DEPTH1_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_SPACE_TO_DEPTH1_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("BlockSize", &self.BlockSize).field("Order", &self.Order).finish()
    }
}
impl ::core::cmp::PartialEq for DML_SPACE_TO_DEPTH1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.BlockSize == other.BlockSize && self.Order == other.Order
    }
}
impl ::core::cmp::Eq for DML_SPACE_TO_DEPTH1_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_SPACE_TO_DEPTH1_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_SPACE_TO_DEPTH_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub BlockSize: u32,
}
impl DML_SPACE_TO_DEPTH_OPERATOR_DESC {}
impl ::core::default::Default for DML_SPACE_TO_DEPTH_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_SPACE_TO_DEPTH_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_SPACE_TO_DEPTH_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("BlockSize", &self.BlockSize).finish()
    }
}
impl ::core::cmp::PartialEq for DML_SPACE_TO_DEPTH_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.BlockSize == other.BlockSize
    }
}
impl ::core::cmp::Eq for DML_SPACE_TO_DEPTH_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_SPACE_TO_DEPTH_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_SPLIT_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputCount: u32,
    pub OutputTensors: *mut DML_TENSOR_DESC,
    pub Axis: u32,
}
impl DML_SPLIT_OPERATOR_DESC {}
impl ::core::default::Default for DML_SPLIT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_SPLIT_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_SPLIT_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputCount", &self.OutputCount).field("OutputTensors", &self.OutputTensors).field("Axis", &self.Axis).finish()
    }
}
impl ::core::cmp::PartialEq for DML_SPLIT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputCount == other.OutputCount && self.OutputTensors == other.OutputTensors && self.Axis == other.Axis
    }
}
impl ::core::cmp::Eq for DML_SPLIT_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_SPLIT_OPERATOR_DESC {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub const DML_TARGET_VERSION: u32 = 16384u32;
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub const DML_TEMPORARY_BUFFER_ALIGNMENT: u32 = 256u32;
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DML_TENSOR_DATA_TYPE(pub i32);
pub const DML_TENSOR_DATA_TYPE_UNKNOWN: DML_TENSOR_DATA_TYPE = DML_TENSOR_DATA_TYPE(0i32);
pub const DML_TENSOR_DATA_TYPE_FLOAT32: DML_TENSOR_DATA_TYPE = DML_TENSOR_DATA_TYPE(1i32);
pub const DML_TENSOR_DATA_TYPE_FLOAT16: DML_TENSOR_DATA_TYPE = DML_TENSOR_DATA_TYPE(2i32);
pub const DML_TENSOR_DATA_TYPE_UINT32: DML_TENSOR_DATA_TYPE = DML_TENSOR_DATA_TYPE(3i32);
pub const DML_TENSOR_DATA_TYPE_UINT16: DML_TENSOR_DATA_TYPE = DML_TENSOR_DATA_TYPE(4i32);
pub const DML_TENSOR_DATA_TYPE_UINT8: DML_TENSOR_DATA_TYPE = DML_TENSOR_DATA_TYPE(5i32);
pub const DML_TENSOR_DATA_TYPE_INT32: DML_TENSOR_DATA_TYPE = DML_TENSOR_DATA_TYPE(6i32);
pub const DML_TENSOR_DATA_TYPE_INT16: DML_TENSOR_DATA_TYPE = DML_TENSOR_DATA_TYPE(7i32);
pub const DML_TENSOR_DATA_TYPE_INT8: DML_TENSOR_DATA_TYPE = DML_TENSOR_DATA_TYPE(8i32);
pub const DML_TENSOR_DATA_TYPE_FLOAT64: DML_TENSOR_DATA_TYPE = DML_TENSOR_DATA_TYPE(9i32);
pub const DML_TENSOR_DATA_TYPE_UINT64: DML_TENSOR_DATA_TYPE = DML_TENSOR_DATA_TYPE(10i32);
pub const DML_TENSOR_DATA_TYPE_INT64: DML_TENSOR_DATA_TYPE = DML_TENSOR_DATA_TYPE(11i32);
impl ::core::convert::From<i32> for DML_TENSOR_DATA_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DML_TENSOR_DATA_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_TENSOR_DESC {
    pub Type: DML_TENSOR_TYPE,
    pub Desc: *mut ::core::ffi::c_void,
}
impl DML_TENSOR_DESC {}
impl ::core::default::Default for DML_TENSOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_TENSOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_TENSOR_DESC").field("Type", &self.Type).field("Desc", &self.Desc).finish()
    }
}
impl ::core::cmp::PartialEq for DML_TENSOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Desc == other.Desc
    }
}
impl ::core::cmp::Eq for DML_TENSOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_TENSOR_DESC {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub const DML_TENSOR_DIMENSION_COUNT_MAX: u32 = 5u32;
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub const DML_TENSOR_DIMENSION_COUNT_MAX1: u32 = 8u32;
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DML_TENSOR_FLAGS(pub u32);
pub const DML_TENSOR_FLAG_NONE: DML_TENSOR_FLAGS = DML_TENSOR_FLAGS(0u32);
pub const DML_TENSOR_FLAG_OWNED_BY_DML: DML_TENSOR_FLAGS = DML_TENSOR_FLAGS(1u32);
impl ::core::convert::From<u32> for DML_TENSOR_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DML_TENSOR_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for DML_TENSOR_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for DML_TENSOR_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for DML_TENSOR_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for DML_TENSOR_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for DML_TENSOR_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DML_TENSOR_TYPE(pub i32);
pub const DML_TENSOR_TYPE_INVALID: DML_TENSOR_TYPE = DML_TENSOR_TYPE(0i32);
pub const DML_TENSOR_TYPE_BUFFER: DML_TENSOR_TYPE = DML_TENSOR_TYPE(1i32);
impl ::core::convert::From<i32> for DML_TENSOR_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DML_TENSOR_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_TILE_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub RepeatsCount: u32,
    pub Repeats: *mut u32,
}
impl DML_TILE_OPERATOR_DESC {}
impl ::core::default::Default for DML_TILE_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_TILE_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_TILE_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("RepeatsCount", &self.RepeatsCount).field("Repeats", &self.Repeats).finish()
    }
}
impl ::core::cmp::PartialEq for DML_TILE_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.RepeatsCount == other.RepeatsCount && self.Repeats == other.Repeats
    }
}
impl ::core::cmp::Eq for DML_TILE_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_TILE_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_TOP_K1_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputValueTensor: *mut DML_TENSOR_DESC,
    pub OutputIndexTensor: *mut DML_TENSOR_DESC,
    pub Axis: u32,
    pub K: u32,
    pub AxisDirection: DML_AXIS_DIRECTION,
}
impl DML_TOP_K1_OPERATOR_DESC {}
impl ::core::default::Default for DML_TOP_K1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_TOP_K1_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_TOP_K1_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputValueTensor", &self.OutputValueTensor).field("OutputIndexTensor", &self.OutputIndexTensor).field("Axis", &self.Axis).field("K", &self.K).field("AxisDirection", &self.AxisDirection).finish()
    }
}
impl ::core::cmp::PartialEq for DML_TOP_K1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputValueTensor == other.OutputValueTensor && self.OutputIndexTensor == other.OutputIndexTensor && self.Axis == other.Axis && self.K == other.K && self.AxisDirection == other.AxisDirection
    }
}
impl ::core::cmp::Eq for DML_TOP_K1_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_TOP_K1_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_TOP_K_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputValueTensor: *mut DML_TENSOR_DESC,
    pub OutputIndexTensor: *mut DML_TENSOR_DESC,
    pub Axis: u32,
    pub K: u32,
}
impl DML_TOP_K_OPERATOR_DESC {}
impl ::core::default::Default for DML_TOP_K_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_TOP_K_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_TOP_K_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputValueTensor", &self.OutputValueTensor).field("OutputIndexTensor", &self.OutputIndexTensor).field("Axis", &self.Axis).field("K", &self.K).finish()
    }
}
impl ::core::cmp::PartialEq for DML_TOP_K_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputValueTensor == other.OutputValueTensor && self.OutputIndexTensor == other.OutputIndexTensor && self.Axis == other.Axis && self.K == other.K
    }
}
impl ::core::cmp::Eq for DML_TOP_K_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_TOP_K_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_UPSAMPLE_2D_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleSize: DML_SIZE_2D,
    pub InterpolationMode: DML_INTERPOLATION_MODE,
}
impl DML_UPSAMPLE_2D_OPERATOR_DESC {}
impl ::core::default::Default for DML_UPSAMPLE_2D_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_UPSAMPLE_2D_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_UPSAMPLE_2D_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleSize", &self.ScaleSize).field("InterpolationMode", &self.InterpolationMode).finish()
    }
}
impl ::core::cmp::PartialEq for DML_UPSAMPLE_2D_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleSize == other.ScaleSize && self.InterpolationMode == other.InterpolationMode
    }
}
impl ::core::cmp::Eq for DML_UPSAMPLE_2D_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_UPSAMPLE_2D_OPERATOR_DESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
pub struct DML_VALUE_SCALE_2D_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Scale: f32,
    pub ChannelCount: u32,
    pub Bias: *mut f32,
}
impl DML_VALUE_SCALE_2D_OPERATOR_DESC {}
impl ::core::default::Default for DML_VALUE_SCALE_2D_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DML_VALUE_SCALE_2D_OPERATOR_DESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DML_VALUE_SCALE_2D_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Scale", &self.Scale).field("ChannelCount", &self.ChannelCount).field("Bias", &self.Bias).finish()
    }
}
impl ::core::cmp::PartialEq for DML_VALUE_SCALE_2D_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Scale == other.Scale && self.ChannelCount == other.ChannelCount && self.Bias == other.Bias
    }
}
impl ::core::cmp::Eq for DML_VALUE_SCALE_2D_OPERATOR_DESC {}
unsafe impl ::windows::runtime::Abi for DML_VALUE_SCALE_2D_OPERATOR_DESC {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDMLBindingTable(pub ::windows::runtime::IUnknown);
impl IDMLBindingTable {
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::runtime::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::runtime::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, guid: *const ::windows::runtime::GUID, data: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), data.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Foundation`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn GetDevice<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn BindInputs(&self, bindingcount: u32, bindings: *const DML_BINDING_DESC) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(bindingcount), ::core::mem::transmute(bindings)))
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn BindOutputs(&self, bindingcount: u32, bindings: *const DML_BINDING_DESC) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(bindingcount), ::core::mem::transmute(bindings)))
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn BindTemporaryResource(&self, binding: *const DML_BINDING_DESC) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(binding)))
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn BindPersistentResource(&self, binding: *const DML_BINDING_DESC) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(binding)))
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn Reset(&self, desc: *const DML_BINDING_TABLE_DESC) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(desc)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDMLBindingTable {
    type Vtable = IDMLBindingTable_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(700876764, 56948, 20027, [171, 0, 17, 104, 242, 252, 60, 252]);
}
impl ::core::convert::From<IDMLBindingTable> for ::windows::runtime::IUnknown {
    fn from(value: IDMLBindingTable) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDMLBindingTable> for ::windows::runtime::IUnknown {
    fn from(value: &IDMLBindingTable) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDMLBindingTable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDMLBindingTable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IDMLBindingTable> for IDMLDeviceChild {
    fn from(value: IDMLBindingTable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLBindingTable> for IDMLDeviceChild {
    fn from(value: &IDMLBindingTable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLDeviceChild> for IDMLBindingTable {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLDeviceChild> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLDeviceChild> for &IDMLBindingTable {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLDeviceChild> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDMLBindingTable> for IDMLObject {
    fn from(value: IDMLBindingTable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLBindingTable> for IDMLObject {
    fn from(value: &IDMLBindingTable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLObject> for IDMLBindingTable {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLObject> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLObject> for &IDMLBindingTable {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLObject> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMLBindingTable_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: *const ::windows::runtime::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: *const ::windows::runtime::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: *const ::windows::runtime::GUID, data: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bindingcount: u32, bindings: *const DML_BINDING_DESC),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bindingcount: u32, bindings: *const DML_BINDING_DESC),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, binding: *const DML_BINDING_DESC),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, binding: *const DML_BINDING_DESC),
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, desc: *const ::core::mem::ManuallyDrop<DML_BINDING_TABLE_DESC>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
);
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDMLCommandRecorder(pub ::windows::runtime::IUnknown);
impl IDMLCommandRecorder {
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::runtime::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::runtime::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, guid: *const ::windows::runtime::GUID, data: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), data.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Foundation`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn GetDevice<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn RecordDispatch<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12CommandList>, Param1: ::windows::runtime::IntoParam<'a, IDMLDispatchable>, Param2: ::windows::runtime::IntoParam<'a, IDMLBindingTable>>(&self, commandlist: Param0, dispatchable: Param1, bindings: Param2) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), commandlist.into_param().abi(), dispatchable.into_param().abi(), bindings.into_param().abi()))
    }
}
unsafe impl ::windows::runtime::Interface for IDMLCommandRecorder {
    type Vtable = IDMLCommandRecorder_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3867507318, 11838, 20445, [191, 244, 93, 43, 161, 15, 180, 83]);
}
impl ::core::convert::From<IDMLCommandRecorder> for ::windows::runtime::IUnknown {
    fn from(value: IDMLCommandRecorder) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDMLCommandRecorder> for ::windows::runtime::IUnknown {
    fn from(value: &IDMLCommandRecorder) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDMLCommandRecorder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDMLCommandRecorder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IDMLCommandRecorder> for IDMLDeviceChild {
    fn from(value: IDMLCommandRecorder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLCommandRecorder> for IDMLDeviceChild {
    fn from(value: &IDMLCommandRecorder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLDeviceChild> for IDMLCommandRecorder {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLDeviceChild> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLDeviceChild> for &IDMLCommandRecorder {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLDeviceChild> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDMLCommandRecorder> for IDMLObject {
    fn from(value: IDMLCommandRecorder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLCommandRecorder> for IDMLObject {
    fn from(value: &IDMLCommandRecorder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLObject> for IDMLCommandRecorder {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLObject> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLObject> for &IDMLCommandRecorder {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLObject> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMLCommandRecorder_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: *const ::windows::runtime::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: *const ::windows::runtime::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: *const ::windows::runtime::GUID, data: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, commandlist: ::windows::runtime::RawPtr, dispatchable: ::windows::runtime::RawPtr, bindings: ::windows::runtime::RawPtr),
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
);
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDMLCompiledOperator(pub ::windows::runtime::IUnknown);
impl IDMLCompiledOperator {
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::runtime::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::runtime::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, guid: *const ::windows::runtime::GUID, data: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), data.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Foundation`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn GetDevice<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn GetBindingProperties(&self) -> DML_BINDING_PROPERTIES {
        let mut result__: DML_BINDING_PROPERTIES = ::core::default::Default::default();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__);
        result__
    }
}
unsafe impl ::windows::runtime::Interface for IDMLCompiledOperator {
    type Vtable = IDMLCompiledOperator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1796597098, 48988, 18690, [146, 216, 218, 58, 101, 10, 254, 164]);
}
impl ::core::convert::From<IDMLCompiledOperator> for ::windows::runtime::IUnknown {
    fn from(value: IDMLCompiledOperator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDMLCompiledOperator> for ::windows::runtime::IUnknown {
    fn from(value: &IDMLCompiledOperator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDMLCompiledOperator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDMLCompiledOperator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IDMLCompiledOperator> for IDMLDispatchable {
    fn from(value: IDMLCompiledOperator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLCompiledOperator> for IDMLDispatchable {
    fn from(value: &IDMLCompiledOperator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLDispatchable> for IDMLCompiledOperator {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLDispatchable> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLDispatchable> for &IDMLCompiledOperator {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLDispatchable> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDMLCompiledOperator> for IDMLPageable {
    fn from(value: IDMLCompiledOperator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLCompiledOperator> for IDMLPageable {
    fn from(value: &IDMLCompiledOperator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLPageable> for IDMLCompiledOperator {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLPageable> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLPageable> for &IDMLCompiledOperator {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLPageable> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDMLCompiledOperator> for IDMLDeviceChild {
    fn from(value: IDMLCompiledOperator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLCompiledOperator> for IDMLDeviceChild {
    fn from(value: &IDMLCompiledOperator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLDeviceChild> for IDMLCompiledOperator {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLDeviceChild> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLDeviceChild> for &IDMLCompiledOperator {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLDeviceChild> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDMLCompiledOperator> for IDMLObject {
    fn from(value: IDMLCompiledOperator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLCompiledOperator> for IDMLObject {
    fn from(value: &IDMLCompiledOperator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLObject> for IDMLCompiledOperator {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLObject> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLObject> for &IDMLCompiledOperator {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLObject> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMLCompiledOperator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: *const ::windows::runtime::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: *const ::windows::runtime::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: *const ::windows::runtime::GUID, data: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DML_BINDING_PROPERTIES),
);
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDMLDebugDevice(pub ::windows::runtime::IUnknown);
impl IDMLDebugDevice {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Foundation`*"]
    pub unsafe fn SetMuteDebugOutput<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, mute: Param0) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), mute.into_param().abi()))
    }
}
unsafe impl ::windows::runtime::Interface for IDMLDebugDevice {
    type Vtable = IDMLDebugDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2104441545, 14666, 19139, [146, 167, 57, 12, 197, 122, 130, 23]);
}
impl ::core::convert::From<IDMLDebugDevice> for ::windows::runtime::IUnknown {
    fn from(value: IDMLDebugDevice) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDMLDebugDevice> for ::windows::runtime::IUnknown {
    fn from(value: &IDMLDebugDevice) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDMLDebugDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDMLDebugDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMLDebugDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mute: super::super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDMLDevice(pub ::windows::runtime::IUnknown);
impl IDMLDevice {
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::runtime::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::runtime::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, guid: *const ::windows::runtime::GUID, data: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), data.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Foundation`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn CheckFeatureSupport(&self, feature: DML_FEATURE, featurequerydatasize: u32, featurequerydata: *const ::core::ffi::c_void, featuresupportdatasize: u32, featuresupportdata: *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(feature), ::core::mem::transmute(featurequerydatasize), ::core::mem::transmute(featurequerydata), ::core::mem::transmute(featuresupportdatasize), ::core::mem::transmute(featuresupportdata)).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn CreateOperator<T: ::windows::runtime::Interface>(&self, desc: *const DML_OPERATOR_DESC, result__: *mut ::core::option::Option<T>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(desc), &<T as ::windows::runtime::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn CompileOperator<'a, Param0: ::windows::runtime::IntoParam<'a, IDMLOperator>, T: ::windows::runtime::Interface>(&self, op: Param0, flags: DML_EXECUTION_FLAGS, result__: *mut ::core::option::Option<T>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), op.into_param().abi(), ::core::mem::transmute(flags), &<T as ::windows::runtime::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn CreateOperatorInitializer<T: ::windows::runtime::Interface>(&self, operatorcount: u32, operators: *const ::core::option::Option<IDMLCompiledOperator>) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(operatorcount), ::core::mem::transmute(operators), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn CreateCommandRecorder<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn CreateBindingTable<T: ::windows::runtime::Interface>(&self, desc: *const DML_BINDING_TABLE_DESC) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(desc), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn Evict(&self, count: u32, ppobjects: *const ::core::option::Option<IDMLPageable>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(count), ::core::mem::transmute(ppobjects)).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn MakeResident(&self, count: u32, ppobjects: *const ::core::option::Option<IDMLPageable>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(count), ::core::mem::transmute(ppobjects)).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn GetDeviceRemovedReason(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn GetParentDevice<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDMLDevice {
    type Vtable = IDMLDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1841128503, 38653, 16959, [169, 140, 174, 94, 124, 42, 87, 63]);
}
impl ::core::convert::From<IDMLDevice> for ::windows::runtime::IUnknown {
    fn from(value: IDMLDevice) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDMLDevice> for ::windows::runtime::IUnknown {
    fn from(value: &IDMLDevice) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDMLDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDMLDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IDMLDevice> for IDMLObject {
    fn from(value: IDMLDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLDevice> for IDMLObject {
    fn from(value: &IDMLDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLObject> for IDMLDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLObject> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLObject> for &IDMLDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLObject> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMLDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: *const ::windows::runtime::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: *const ::windows::runtime::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: *const ::windows::runtime::GUID, data: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, feature: DML_FEATURE, featurequerydatasize: u32, featurequerydata: *const ::core::ffi::c_void, featuresupportdatasize: u32, featuresupportdata: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, desc: *const DML_OPERATOR_DESC, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, op: ::windows::runtime::RawPtr, flags: DML_EXECUTION_FLAGS, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, operatorcount: u32, operators: *const ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, desc: *const ::core::mem::ManuallyDrop<DML_BINDING_TABLE_DESC>, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: u32, ppobjects: *const ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: u32, ppobjects: *const ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDMLDevice1(pub ::windows::runtime::IUnknown);
impl IDMLDevice1 {
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::runtime::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::runtime::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, guid: *const ::windows::runtime::GUID, data: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), data.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Foundation`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn CheckFeatureSupport(&self, feature: DML_FEATURE, featurequerydatasize: u32, featurequerydata: *const ::core::ffi::c_void, featuresupportdatasize: u32, featuresupportdata: *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(feature), ::core::mem::transmute(featurequerydatasize), ::core::mem::transmute(featurequerydata), ::core::mem::transmute(featuresupportdatasize), ::core::mem::transmute(featuresupportdata)).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn CreateOperator<T: ::windows::runtime::Interface>(&self, desc: *const DML_OPERATOR_DESC, result__: *mut ::core::option::Option<T>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(desc), &<T as ::windows::runtime::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn CompileOperator<'a, Param0: ::windows::runtime::IntoParam<'a, IDMLOperator>, T: ::windows::runtime::Interface>(&self, op: Param0, flags: DML_EXECUTION_FLAGS, result__: *mut ::core::option::Option<T>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), op.into_param().abi(), ::core::mem::transmute(flags), &<T as ::windows::runtime::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn CreateOperatorInitializer<T: ::windows::runtime::Interface>(&self, operatorcount: u32, operators: *const ::core::option::Option<IDMLCompiledOperator>) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(operatorcount), ::core::mem::transmute(operators), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn CreateCommandRecorder<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn CreateBindingTable<T: ::windows::runtime::Interface>(&self, desc: *const DML_BINDING_TABLE_DESC) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(desc), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn Evict(&self, count: u32, ppobjects: *const ::core::option::Option<IDMLPageable>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(count), ::core::mem::transmute(ppobjects)).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn MakeResident(&self, count: u32, ppobjects: *const ::core::option::Option<IDMLPageable>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(count), ::core::mem::transmute(ppobjects)).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn GetDeviceRemovedReason(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn GetParentDevice<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn CompileGraph<T: ::windows::runtime::Interface>(&self, desc: *const DML_GRAPH_DESC, flags: DML_EXECUTION_FLAGS, result__: *mut ::core::option::Option<T>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(desc), ::core::mem::transmute(flags), &<T as ::windows::runtime::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDMLDevice1 {
    type Vtable = IDMLDevice1_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2693287834, 53950, 17237, [170, 93, 89, 1, 40, 26, 209, 210]);
}
impl ::core::convert::From<IDMLDevice1> for ::windows::runtime::IUnknown {
    fn from(value: IDMLDevice1) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDMLDevice1> for ::windows::runtime::IUnknown {
    fn from(value: &IDMLDevice1) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDMLDevice1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDMLDevice1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IDMLDevice1> for IDMLDevice {
    fn from(value: IDMLDevice1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLDevice1> for IDMLDevice {
    fn from(value: &IDMLDevice1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLDevice> for IDMLDevice1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLDevice> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLDevice> for &IDMLDevice1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLDevice> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDMLDevice1> for IDMLObject {
    fn from(value: IDMLDevice1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLDevice1> for IDMLObject {
    fn from(value: &IDMLDevice1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLObject> for IDMLDevice1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLObject> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLObject> for &IDMLDevice1 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLObject> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMLDevice1_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: *const ::windows::runtime::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: *const ::windows::runtime::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: *const ::windows::runtime::GUID, data: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, feature: DML_FEATURE, featurequerydatasize: u32, featurequerydata: *const ::core::ffi::c_void, featuresupportdatasize: u32, featuresupportdata: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, desc: *const DML_OPERATOR_DESC, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, op: ::windows::runtime::RawPtr, flags: DML_EXECUTION_FLAGS, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, operatorcount: u32, operators: *const ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, desc: *const ::core::mem::ManuallyDrop<DML_BINDING_TABLE_DESC>, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: u32, ppobjects: *const ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: u32, ppobjects: *const ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, desc: *const DML_GRAPH_DESC, flags: DML_EXECUTION_FLAGS, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDMLDeviceChild(pub ::windows::runtime::IUnknown);
impl IDMLDeviceChild {
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::runtime::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::runtime::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, guid: *const ::windows::runtime::GUID, data: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), data.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Foundation`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn GetDevice<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDMLDeviceChild {
    type Vtable = IDMLDeviceChild_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(669528386, 33125, 18915, [151, 78, 47, 214, 110, 76, 182, 157]);
}
impl ::core::convert::From<IDMLDeviceChild> for ::windows::runtime::IUnknown {
    fn from(value: IDMLDeviceChild) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDMLDeviceChild> for ::windows::runtime::IUnknown {
    fn from(value: &IDMLDeviceChild) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDMLDeviceChild {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDMLDeviceChild {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IDMLDeviceChild> for IDMLObject {
    fn from(value: IDMLDeviceChild) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLDeviceChild> for IDMLObject {
    fn from(value: &IDMLDeviceChild) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLObject> for IDMLDeviceChild {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLObject> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLObject> for &IDMLDeviceChild {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLObject> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMLDeviceChild_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: *const ::windows::runtime::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: *const ::windows::runtime::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: *const ::windows::runtime::GUID, data: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDMLDispatchable(pub ::windows::runtime::IUnknown);
impl IDMLDispatchable {
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::runtime::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::runtime::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, guid: *const ::windows::runtime::GUID, data: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), data.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Foundation`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn GetDevice<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn GetBindingProperties(&self) -> DML_BINDING_PROPERTIES {
        let mut result__: DML_BINDING_PROPERTIES = ::core::default::Default::default();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__);
        result__
    }
}
unsafe impl ::windows::runtime::Interface for IDMLDispatchable {
    type Vtable = IDMLDispatchable_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3703054760, 4153, 17438, [159, 28, 177, 117, 156, 47, 60, 236]);
}
impl ::core::convert::From<IDMLDispatchable> for ::windows::runtime::IUnknown {
    fn from(value: IDMLDispatchable) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDMLDispatchable> for ::windows::runtime::IUnknown {
    fn from(value: &IDMLDispatchable) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDMLDispatchable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDMLDispatchable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IDMLDispatchable> for IDMLPageable {
    fn from(value: IDMLDispatchable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLDispatchable> for IDMLPageable {
    fn from(value: &IDMLDispatchable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLPageable> for IDMLDispatchable {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLPageable> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLPageable> for &IDMLDispatchable {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLPageable> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDMLDispatchable> for IDMLDeviceChild {
    fn from(value: IDMLDispatchable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLDispatchable> for IDMLDeviceChild {
    fn from(value: &IDMLDispatchable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLDeviceChild> for IDMLDispatchable {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLDeviceChild> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLDeviceChild> for &IDMLDispatchable {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLDeviceChild> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDMLDispatchable> for IDMLObject {
    fn from(value: IDMLDispatchable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLDispatchable> for IDMLObject {
    fn from(value: &IDMLDispatchable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLObject> for IDMLDispatchable {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLObject> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLObject> for &IDMLDispatchable {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLObject> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMLDispatchable_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: *const ::windows::runtime::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: *const ::windows::runtime::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: *const ::windows::runtime::GUID, data: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DML_BINDING_PROPERTIES),
);
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDMLObject(pub ::windows::runtime::IUnknown);
impl IDMLObject {
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::runtime::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::runtime::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, guid: *const ::windows::runtime::GUID, data: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), data.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Foundation`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDMLObject {
    type Vtable = IDMLObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3357948588, 40460, 18989, [155, 142, 0, 117, 33, 163, 49, 124]);
}
impl ::core::convert::From<IDMLObject> for ::windows::runtime::IUnknown {
    fn from(value: IDMLObject) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDMLObject> for ::windows::runtime::IUnknown {
    fn from(value: &IDMLObject) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDMLObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDMLObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMLObject_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: *const ::windows::runtime::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: *const ::windows::runtime::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: *const ::windows::runtime::GUID, data: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDMLOperator(pub ::windows::runtime::IUnknown);
impl IDMLOperator {
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::runtime::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::runtime::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, guid: *const ::windows::runtime::GUID, data: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), data.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Foundation`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn GetDevice<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDMLOperator {
    type Vtable = IDMLOperator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(650817146, 12417, 17971, [149, 129, 34, 111, 190, 87, 105, 93]);
}
impl ::core::convert::From<IDMLOperator> for ::windows::runtime::IUnknown {
    fn from(value: IDMLOperator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDMLOperator> for ::windows::runtime::IUnknown {
    fn from(value: &IDMLOperator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDMLOperator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDMLOperator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IDMLOperator> for IDMLDeviceChild {
    fn from(value: IDMLOperator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLOperator> for IDMLDeviceChild {
    fn from(value: &IDMLOperator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLDeviceChild> for IDMLOperator {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLDeviceChild> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLDeviceChild> for &IDMLOperator {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLDeviceChild> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDMLOperator> for IDMLObject {
    fn from(value: IDMLOperator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLOperator> for IDMLObject {
    fn from(value: &IDMLOperator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLObject> for IDMLOperator {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLObject> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLObject> for &IDMLOperator {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLObject> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMLOperator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: *const ::windows::runtime::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: *const ::windows::runtime::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: *const ::windows::runtime::GUID, data: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDMLOperatorInitializer(pub ::windows::runtime::IUnknown);
impl IDMLOperatorInitializer {
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::runtime::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::runtime::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, guid: *const ::windows::runtime::GUID, data: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), data.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Foundation`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn GetDevice<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn GetBindingProperties(&self) -> DML_BINDING_PROPERTIES {
        let mut result__: DML_BINDING_PROPERTIES = ::core::default::Default::default();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__);
        result__
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn Reset(&self, operatorcount: u32, operators: *const ::core::option::Option<IDMLCompiledOperator>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(operatorcount), ::core::mem::transmute(operators)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDMLOperatorInitializer {
    type Vtable = IDMLOperatorInitializer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1115427091, 17244, 18076, [134, 118, 77, 93, 208, 114, 248, 19]);
}
impl ::core::convert::From<IDMLOperatorInitializer> for ::windows::runtime::IUnknown {
    fn from(value: IDMLOperatorInitializer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDMLOperatorInitializer> for ::windows::runtime::IUnknown {
    fn from(value: &IDMLOperatorInitializer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDMLOperatorInitializer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDMLOperatorInitializer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IDMLOperatorInitializer> for IDMLDispatchable {
    fn from(value: IDMLOperatorInitializer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLOperatorInitializer> for IDMLDispatchable {
    fn from(value: &IDMLOperatorInitializer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLDispatchable> for IDMLOperatorInitializer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLDispatchable> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLDispatchable> for &IDMLOperatorInitializer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLDispatchable> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDMLOperatorInitializer> for IDMLPageable {
    fn from(value: IDMLOperatorInitializer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLOperatorInitializer> for IDMLPageable {
    fn from(value: &IDMLOperatorInitializer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLPageable> for IDMLOperatorInitializer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLPageable> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLPageable> for &IDMLOperatorInitializer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLPageable> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDMLOperatorInitializer> for IDMLDeviceChild {
    fn from(value: IDMLOperatorInitializer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLOperatorInitializer> for IDMLDeviceChild {
    fn from(value: &IDMLOperatorInitializer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLDeviceChild> for IDMLOperatorInitializer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLDeviceChild> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLDeviceChild> for &IDMLOperatorInitializer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLDeviceChild> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDMLOperatorInitializer> for IDMLObject {
    fn from(value: IDMLOperatorInitializer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLOperatorInitializer> for IDMLObject {
    fn from(value: &IDMLOperatorInitializer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLObject> for IDMLOperatorInitializer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLObject> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLObject> for &IDMLOperatorInitializer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLObject> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMLOperatorInitializer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: *const ::windows::runtime::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: *const ::windows::runtime::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: *const ::windows::runtime::GUID, data: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DML_BINDING_PROPERTIES),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, operatorcount: u32, operators: *const ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDMLPageable(pub ::windows::runtime::IUnknown);
impl IDMLPageable {
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::runtime::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::runtime::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, guid: *const ::windows::runtime::GUID, data: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), data.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`, `Win32_Foundation`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_AI_MachineLearning_DirectML`*"]
    pub unsafe fn GetDevice<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDMLPageable {
    type Vtable = IDMLPageable_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2980775973, 17730, 19019, [134, 23, 109, 222, 110, 143, 98, 1]);
}
impl ::core::convert::From<IDMLPageable> for ::windows::runtime::IUnknown {
    fn from(value: IDMLPageable) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDMLPageable> for ::windows::runtime::IUnknown {
    fn from(value: &IDMLPageable) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDMLPageable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDMLPageable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IDMLPageable> for IDMLDeviceChild {
    fn from(value: IDMLPageable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLPageable> for IDMLDeviceChild {
    fn from(value: &IDMLPageable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLDeviceChild> for IDMLPageable {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLDeviceChild> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLDeviceChild> for &IDMLPageable {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLDeviceChild> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDMLPageable> for IDMLObject {
    fn from(value: IDMLPageable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLPageable> for IDMLObject {
    fn from(value: &IDMLPageable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLObject> for IDMLPageable {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLObject> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDMLObject> for &IDMLPageable {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDMLObject> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMLPageable_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: *const ::windows::runtime::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: *const ::windows::runtime::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: *const ::windows::runtime::GUID, data: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
);
