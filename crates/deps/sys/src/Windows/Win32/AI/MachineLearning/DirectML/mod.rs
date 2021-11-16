#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub fn DMLCreateDevice(d3d12device: super::super::super::Graphics::Direct3D12::ID3D12Device, flags: DML_CREATE_DEVICE_FLAGS, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub fn DMLCreateDevice1(d3d12device: super::super::super::Graphics::Direct3D12::ID3D12Device, flags: DML_CREATE_DEVICE_FLAGS, minimumfeaturelevel: DML_FEATURE_LEVEL, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
}
#[repr(C)]
pub struct DML_ACTIVATION_CELU_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Alpha: f32,
}
impl ::core::marker::Copy for DML_ACTIVATION_CELU_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_CELU_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_ELU_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Alpha: f32,
}
impl ::core::marker::Copy for DML_ACTIVATION_ELU_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_ELU_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_HARDMAX_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ACTIVATION_HARDMAX_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_HARDMAX_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_HARD_SIGMOID_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Alpha: f32,
    pub Beta: f32,
}
impl ::core::marker::Copy for DML_ACTIVATION_HARD_SIGMOID_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_HARD_SIGMOID_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_IDENTITY_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ACTIVATION_IDENTITY_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_IDENTITY_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_LEAKY_RELU_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Alpha: f32,
}
impl ::core::marker::Copy for DML_ACTIVATION_LEAKY_RELU_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_LEAKY_RELU_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_LINEAR_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Alpha: f32,
    pub Beta: f32,
}
impl ::core::marker::Copy for DML_ACTIVATION_LINEAR_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_LINEAR_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_LOG_SOFTMAX_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ACTIVATION_LOG_SOFTMAX_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_LOG_SOFTMAX_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_PARAMETERIZED_RELU_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub SlopeTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ACTIVATION_PARAMETERIZED_RELU_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_PARAMETERIZED_RELU_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_PARAMETRIC_SOFTPLUS_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Alpha: f32,
    pub Beta: f32,
}
impl ::core::marker::Copy for DML_ACTIVATION_PARAMETRIC_SOFTPLUS_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_PARAMETRIC_SOFTPLUS_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_RELU_GRAD_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub InputGradientTensor: *mut DML_TENSOR_DESC,
    pub OutputGradientTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ACTIVATION_RELU_GRAD_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_RELU_GRAD_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_RELU_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ACTIVATION_RELU_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_RELU_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_SCALED_ELU_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Alpha: f32,
    pub Gamma: f32,
}
impl ::core::marker::Copy for DML_ACTIVATION_SCALED_ELU_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_SCALED_ELU_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_SCALED_TANH_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Alpha: f32,
    pub Beta: f32,
}
impl ::core::marker::Copy for DML_ACTIVATION_SCALED_TANH_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_SCALED_TANH_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_SHRINK_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Bias: f32,
    pub Threshold: f32,
}
impl ::core::marker::Copy for DML_ACTIVATION_SHRINK_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_SHRINK_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_SIGMOID_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ACTIVATION_SIGMOID_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_SIGMOID_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_SOFTMAX_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ACTIVATION_SOFTMAX_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_SOFTMAX_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_SOFTPLUS_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Steepness: f32,
}
impl ::core::marker::Copy for DML_ACTIVATION_SOFTPLUS_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_SOFTPLUS_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_SOFTSIGN_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ACTIVATION_SOFTSIGN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_SOFTSIGN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_TANH_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ACTIVATION_TANH_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_TANH_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_THRESHOLDED_RELU_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Alpha: f32,
}
impl ::core::marker::Copy for DML_ACTIVATION_THRESHOLDED_RELU_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_THRESHOLDED_RELU_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
impl ::core::marker::Copy for DML_ADAM_OPTIMIZER_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ADAM_OPTIMIZER_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ARGMAX_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub AxisCount: u32,
    pub Axes: *mut u32,
    pub AxisDirection: DML_AXIS_DIRECTION,
}
impl ::core::marker::Copy for DML_ARGMAX_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ARGMAX_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ARGMIN_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub AxisCount: u32,
    pub Axes: *mut u32,
    pub AxisDirection: DML_AXIS_DIRECTION,
}
impl ::core::marker::Copy for DML_ARGMIN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ARGMIN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
impl ::core::marker::Copy for DML_AVERAGE_POOLING_GRAD_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_AVERAGE_POOLING_GRAD_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
impl ::core::marker::Copy for DML_AVERAGE_POOLING_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_AVERAGE_POOLING_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DML_AXIS_DIRECTION_INCREASING: i32 = 0i32;
pub const DML_AXIS_DIRECTION_DECREASING: i32 = 1i32;
#[repr(C)]
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
impl ::core::marker::Copy for DML_BATCH_NORMALIZATION_GRAD_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_BATCH_NORMALIZATION_GRAD_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
impl ::core::marker::Copy for DML_BATCH_NORMALIZATION_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_BATCH_NORMALIZATION_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_BINDING_DESC {
    pub Type: DML_BINDING_TYPE,
    pub Desc: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DML_BINDING_DESC {}
impl ::core::clone::Clone for DML_BINDING_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_BINDING_PROPERTIES {
    pub RequiredDescriptorCount: u32,
    pub TemporaryResourceSize: u64,
    pub PersistentResourceSize: u64,
}
impl ::core::marker::Copy for DML_BINDING_PROPERTIES {}
impl ::core::clone::Clone for DML_BINDING_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub struct DML_BINDING_TABLE_DESC {
    pub Dispatchable: IDMLDispatchable,
    pub CPUDescriptorHandle: super::super::super::Graphics::Direct3D12::D3D12_CPU_DESCRIPTOR_HANDLE,
    pub GPUDescriptorHandle: super::super::super::Graphics::Direct3D12::D3D12_GPU_DESCRIPTOR_HANDLE,
    pub SizeInDescriptors: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::marker::Copy for DML_BINDING_TABLE_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::clone::Clone for DML_BINDING_TABLE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DML_BINDING_TYPE_NONE: i32 = 0i32;
pub const DML_BINDING_TYPE_BUFFER: i32 = 1i32;
pub const DML_BINDING_TYPE_BUFFER_ARRAY: i32 = 2i32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub struct DML_BUFFER_ARRAY_BINDING {
    pub BindingCount: u32,
    pub Bindings: *mut DML_BUFFER_BINDING,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::marker::Copy for DML_BUFFER_ARRAY_BINDING {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::clone::Clone for DML_BUFFER_ARRAY_BINDING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub struct DML_BUFFER_BINDING {
    pub Buffer: super::super::super::Graphics::Direct3D12::ID3D12Resource,
    pub Offset: u64,
    pub SizeInBytes: u64,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::marker::Copy for DML_BUFFER_BINDING {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::clone::Clone for DML_BUFFER_BINDING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_BUFFER_TENSOR_DESC {
    pub DataType: DML_TENSOR_DATA_TYPE,
    pub Flags: DML_TENSOR_FLAGS,
    pub DimensionCount: u32,
    pub Sizes: *mut u32,
    pub Strides: *mut u32,
    pub TotalTensorSizeInBytes: u64,
    pub GuaranteedBaseOffsetAlignment: u32,
}
impl ::core::marker::Copy for DML_BUFFER_TENSOR_DESC {}
impl ::core::clone::Clone for DML_BUFFER_TENSOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_CAST_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_CAST_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_CAST_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DML_CONVOLUTION_DIRECTION_FORWARD: i32 = 0i32;
pub const DML_CONVOLUTION_DIRECTION_BACKWARD: i32 = 1i32;
#[repr(C)]
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
impl ::core::marker::Copy for DML_CONVOLUTION_INTEGER_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_CONVOLUTION_INTEGER_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DML_CONVOLUTION_MODE_CONVOLUTION: i32 = 0i32;
pub const DML_CONVOLUTION_MODE_CROSS_CORRELATION: i32 = 1i32;
#[repr(C)]
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
impl ::core::marker::Copy for DML_CONVOLUTION_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_CONVOLUTION_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DML_CREATE_DEVICE_FLAG_NONE: u32 = 0u32;
pub const DML_CREATE_DEVICE_FLAG_DEBUG: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DML_CUMULATIVE_PRODUCT_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Axis: u32,
    pub AxisDirection: DML_AXIS_DIRECTION,
    pub HasExclusiveProduct: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DML_CUMULATIVE_PRODUCT_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_CUMULATIVE_PRODUCT_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DML_CUMULATIVE_SUMMATION_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Axis: u32,
    pub AxisDirection: DML_AXIS_DIRECTION,
    pub HasExclusiveSum: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DML_CUMULATIVE_SUMMATION_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_CUMULATIVE_SUMMATION_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DML_DEPTH_SPACE_ORDER_DEPTH_COLUMN_ROW: i32 = 0i32;
pub const DML_DEPTH_SPACE_ORDER_COLUMN_ROW_DEPTH: i32 = 1i32;
#[repr(C)]
pub struct DML_DEPTH_TO_SPACE1_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub BlockSize: u32,
    pub Order: DML_DEPTH_SPACE_ORDER,
}
impl ::core::marker::Copy for DML_DEPTH_TO_SPACE1_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_DEPTH_TO_SPACE1_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_DEPTH_TO_SPACE_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub BlockSize: u32,
}
impl ::core::marker::Copy for DML_DEPTH_TO_SPACE_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_DEPTH_TO_SPACE_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_DIAGONAL_MATRIX_OPERATOR_DESC {
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Offset: i32,
    pub Value: f32,
}
impl ::core::marker::Copy for DML_DIAGONAL_MATRIX_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_DIAGONAL_MATRIX_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_DYNAMIC_QUANTIZE_LINEAR_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub OutputScaleTensor: *mut DML_TENSOR_DESC,
    pub OutputZeroPointTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_DYNAMIC_QUANTIZE_LINEAR_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_DYNAMIC_QUANTIZE_LINEAR_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_ABS_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_ABS_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_ABS_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_ACOSH_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_ACOSH_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_ACOSH_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_ACOS_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_ACOS_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_ACOS_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_ADD1_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub FusedActivation: *mut DML_OPERATOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_ADD1_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_ADD1_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_ADD_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_ADD_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_ADD_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_ASINH_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_ASINH_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_ASINH_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_ASIN_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_ASIN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_ASIN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_ATANH_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_ATANH_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_ATANH_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_ATAN_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_ATAN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_ATAN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_ATAN_YX_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_ATAN_YX_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_ATAN_YX_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_BIT_AND_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_BIT_AND_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_BIT_AND_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_BIT_COUNT_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_BIT_COUNT_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_BIT_COUNT_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_BIT_NOT_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_BIT_NOT_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_BIT_NOT_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_BIT_OR_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_BIT_OR_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_BIT_OR_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_BIT_SHIFT_LEFT_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_BIT_SHIFT_LEFT_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_BIT_SHIFT_LEFT_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_BIT_SHIFT_RIGHT_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_BIT_SHIFT_RIGHT_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_BIT_SHIFT_RIGHT_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_BIT_XOR_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_BIT_XOR_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_BIT_XOR_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_CEIL_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_CEIL_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_CEIL_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_CLIP_GRAD_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub InputGradientTensor: *mut DML_TENSOR_DESC,
    pub OutputGradientTensor: *mut DML_TENSOR_DESC,
    pub Min: f32,
    pub Max: f32,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_CLIP_GRAD_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_CLIP_GRAD_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_CLIP_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
    pub Min: f32,
    pub Max: f32,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_CLIP_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_CLIP_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
    pub Exponent: f32,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_COSH_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_COSH_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_COSH_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_COS_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_COS_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_COS_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_DEQUANTIZE_LINEAR_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub ScaleTensor: *mut DML_TENSOR_DESC,
    pub ZeroPointTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_DEQUANTIZE_LINEAR_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_DEQUANTIZE_LINEAR_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_DIFFERENCE_SQUARE_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_DIFFERENCE_SQUARE_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_DIFFERENCE_SQUARE_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_DIVIDE_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_DIVIDE_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_DIVIDE_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_ERF_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_ERF_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_ERF_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_EXP_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_EXP_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_EXP_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_FLOOR_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_FLOOR_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_FLOOR_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_IDENTITY_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_IDENTITY_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_IDENTITY_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_IF_OPERATOR_DESC {
    pub ConditionTensor: *mut DML_TENSOR_DESC,
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_IF_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_IF_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_IS_INFINITY_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub InfinityMode: DML_IS_INFINITY_MODE,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_IS_INFINITY_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_IS_INFINITY_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_IS_NAN_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_IS_NAN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_IS_NAN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_LOGICAL_AND_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_LOGICAL_AND_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_LOGICAL_AND_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_LOGICAL_EQUALS_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_LOGICAL_EQUALS_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_LOGICAL_EQUALS_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_LOGICAL_NOT_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_LOGICAL_NOT_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_LOGICAL_NOT_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_LOGICAL_OR_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_LOGICAL_OR_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_LOGICAL_OR_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_LOGICAL_XOR_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_LOGICAL_XOR_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_LOGICAL_XOR_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_LOG_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_LOG_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_LOG_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_MAX_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_MAX_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_MAX_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_MEAN_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_MEAN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_MEAN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_MIN_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_MIN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_MIN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_MODULUS_FLOOR_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_MODULUS_FLOOR_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_MODULUS_FLOOR_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_MODULUS_TRUNCATE_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_MODULUS_TRUNCATE_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_MODULUS_TRUNCATE_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_MULTIPLY_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_MULTIPLY_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_MULTIPLY_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_POW_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub ExponentTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_POW_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_POW_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
impl ::core::marker::Copy for DML_ELEMENT_WISE_QUANTIZED_LINEAR_ADD_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_QUANTIZED_LINEAR_ADD_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_QUANTIZE_LINEAR_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub ScaleTensor: *mut DML_TENSOR_DESC,
    pub ZeroPointTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_QUANTIZE_LINEAR_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_QUANTIZE_LINEAR_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_RECIP_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_RECIP_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_RECIP_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_ROUND_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub RoundingMode: DML_ROUNDING_MODE,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_ROUND_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_ROUND_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_SIGN_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_SIGN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_SIGN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_SINH_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_SINH_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_SINH_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_SIN_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_SIN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_SIN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_SQRT_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_SQRT_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_SQRT_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_SUBTRACT_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_SUBTRACT_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_SUBTRACT_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_TANH_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_TANH_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_TANH_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_TAN_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_TAN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_TAN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
    pub Min: f32,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DML_EXECUTION_FLAG_NONE: u32 = 0u32;
pub const DML_EXECUTION_FLAG_ALLOW_HALF_PRECISION_COMPUTATION: u32 = 1u32;
pub const DML_EXECUTION_FLAG_DISABLE_META_COMMANDS: u32 = 2u32;
pub const DML_EXECUTION_FLAG_DESCRIPTORS_VOLATILE: u32 = 4u32;
pub const DML_FEATURE_TENSOR_DATA_TYPE_SUPPORT: i32 = 0i32;
pub const DML_FEATURE_FEATURE_LEVELS: i32 = 1i32;
#[repr(C)]
pub struct DML_FEATURE_DATA_FEATURE_LEVELS {
    pub MaxSupportedFeatureLevel: DML_FEATURE_LEVEL,
}
impl ::core::marker::Copy for DML_FEATURE_DATA_FEATURE_LEVELS {}
impl ::core::clone::Clone for DML_FEATURE_DATA_FEATURE_LEVELS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DML_FEATURE_DATA_TENSOR_DATA_TYPE_SUPPORT {
    pub IsSupported: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DML_FEATURE_DATA_TENSOR_DATA_TYPE_SUPPORT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_FEATURE_DATA_TENSOR_DATA_TYPE_SUPPORT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DML_FEATURE_LEVEL_1_0: i32 = 4096i32;
pub const DML_FEATURE_LEVEL_2_0: i32 = 8192i32;
pub const DML_FEATURE_LEVEL_2_1: i32 = 8448i32;
pub const DML_FEATURE_LEVEL_3_0: i32 = 12288i32;
pub const DML_FEATURE_LEVEL_3_1: i32 = 12544i32;
pub const DML_FEATURE_LEVEL_4_0: i32 = 16384i32;
#[repr(C)]
pub struct DML_FEATURE_QUERY_FEATURE_LEVELS {
    pub RequestedFeatureLevelCount: u32,
    pub RequestedFeatureLevels: *mut DML_FEATURE_LEVEL,
}
impl ::core::marker::Copy for DML_FEATURE_QUERY_FEATURE_LEVELS {}
impl ::core::clone::Clone for DML_FEATURE_QUERY_FEATURE_LEVELS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_FEATURE_QUERY_TENSOR_DATA_TYPE_SUPPORT {
    pub DataType: DML_TENSOR_DATA_TYPE,
}
impl ::core::marker::Copy for DML_FEATURE_QUERY_TENSOR_DATA_TYPE_SUPPORT {}
impl ::core::clone::Clone for DML_FEATURE_QUERY_TENSOR_DATA_TYPE_SUPPORT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_FILL_VALUE_CONSTANT_OPERATOR_DESC {
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ValueDataType: DML_TENSOR_DATA_TYPE,
    pub Value: DML_SCALAR_UNION,
}
impl ::core::marker::Copy for DML_FILL_VALUE_CONSTANT_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_FILL_VALUE_CONSTANT_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_FILL_VALUE_SEQUENCE_OPERATOR_DESC {
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ValueDataType: DML_TENSOR_DATA_TYPE,
    pub ValueStart: DML_SCALAR_UNION,
    pub ValueDelta: DML_SCALAR_UNION,
}
impl ::core::marker::Copy for DML_FILL_VALUE_SEQUENCE_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_FILL_VALUE_SEQUENCE_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_GATHER_ELEMENTS_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub IndicesTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Axis: u32,
}
impl ::core::marker::Copy for DML_GATHER_ELEMENTS_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_GATHER_ELEMENTS_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_GATHER_ND1_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub IndicesTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub InputDimensionCount: u32,
    pub IndicesDimensionCount: u32,
    pub BatchDimensionCount: u32,
}
impl ::core::marker::Copy for DML_GATHER_ND1_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_GATHER_ND1_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_GATHER_ND_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub IndicesTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub InputDimensionCount: u32,
    pub IndicesDimensionCount: u32,
}
impl ::core::marker::Copy for DML_GATHER_ND_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_GATHER_ND_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_GATHER_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub IndicesTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Axis: u32,
    pub IndexDimensions: u32,
}
impl ::core::marker::Copy for DML_GATHER_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_GATHER_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
impl ::core::marker::Copy for DML_GEMM_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_GEMM_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
impl ::core::marker::Copy for DML_GRAPH_DESC {}
impl ::core::clone::Clone for DML_GRAPH_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_GRAPH_EDGE_DESC {
    pub Type: DML_GRAPH_EDGE_TYPE,
    pub Desc: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DML_GRAPH_EDGE_DESC {}
impl ::core::clone::Clone for DML_GRAPH_EDGE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DML_GRAPH_EDGE_TYPE_INVALID: i32 = 0i32;
pub const DML_GRAPH_EDGE_TYPE_INPUT: i32 = 1i32;
pub const DML_GRAPH_EDGE_TYPE_OUTPUT: i32 = 2i32;
pub const DML_GRAPH_EDGE_TYPE_INTERMEDIATE: i32 = 3i32;
#[repr(C)]
pub struct DML_GRAPH_NODE_DESC {
    pub Type: DML_GRAPH_NODE_TYPE,
    pub Desc: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DML_GRAPH_NODE_DESC {}
impl ::core::clone::Clone for DML_GRAPH_NODE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DML_GRAPH_NODE_TYPE_INVALID: i32 = 0i32;
pub const DML_GRAPH_NODE_TYPE_OPERATOR: i32 = 1i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
impl ::core::marker::Copy for DML_GRU_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_GRU_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DML_INPUT_GRAPH_EDGE_DESC {
    pub GraphInputIndex: u32,
    pub ToNodeIndex: u32,
    pub ToNodeInputIndex: u32,
    pub Name: super::super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DML_INPUT_GRAPH_EDGE_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_INPUT_GRAPH_EDGE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DML_INTERMEDIATE_GRAPH_EDGE_DESC {
    pub FromNodeIndex: u32,
    pub FromNodeOutputIndex: u32,
    pub ToNodeIndex: u32,
    pub ToNodeInputIndex: u32,
    pub Name: super::super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DML_INTERMEDIATE_GRAPH_EDGE_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_INTERMEDIATE_GRAPH_EDGE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DML_INTERPOLATION_MODE_NEAREST_NEIGHBOR: i32 = 0i32;
pub const DML_INTERPOLATION_MODE_LINEAR: i32 = 1i32;
pub const DML_IS_INFINITY_MODE_EITHER: i32 = 0i32;
pub const DML_IS_INFINITY_MODE_POSITIVE: i32 = 1i32;
pub const DML_IS_INFINITY_MODE_NEGATIVE: i32 = 2i32;
#[repr(C)]
pub struct DML_JOIN_OPERATOR_DESC {
    pub InputCount: u32,
    pub InputTensors: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Axis: u32,
}
impl ::core::marker::Copy for DML_JOIN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_JOIN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
impl ::core::marker::Copy for DML_LOCAL_RESPONSE_NORMALIZATION_GRAD_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_LOCAL_RESPONSE_NORMALIZATION_GRAD_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
impl ::core::marker::Copy for DML_LOCAL_RESPONSE_NORMALIZATION_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_LOCAL_RESPONSE_NORMALIZATION_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_LP_NORMALIZATION_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Axis: u32,
    pub Epsilon: f32,
    pub P: u32,
}
impl ::core::marker::Copy for DML_LP_NORMALIZATION_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_LP_NORMALIZATION_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
impl ::core::marker::Copy for DML_LP_POOLING_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_LP_POOLING_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
impl ::core::marker::Copy for DML_LSTM_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_LSTM_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_MATRIX_MULTIPLY_INTEGER_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub AZeroPointTensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub BZeroPointTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_MATRIX_MULTIPLY_INTEGER_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_MATRIX_MULTIPLY_INTEGER_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DML_MATRIX_TRANSFORM_NONE: i32 = 0i32;
pub const DML_MATRIX_TRANSFORM_TRANSPOSE: i32 = 1i32;
#[repr(C)]
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
impl ::core::marker::Copy for DML_MAX_POOLING1_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_MAX_POOLING1_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
impl ::core::marker::Copy for DML_MAX_POOLING2_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_MAX_POOLING2_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
impl ::core::marker::Copy for DML_MAX_POOLING_GRAD_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_MAX_POOLING_GRAD_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_MAX_POOLING_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub Strides: *mut u32,
    pub WindowSize: *mut u32,
    pub StartPadding: *mut u32,
    pub EndPadding: *mut u32,
}
impl ::core::marker::Copy for DML_MAX_POOLING_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_MAX_POOLING_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_MAX_UNPOOLING_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub IndicesTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_MAX_UNPOOLING_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_MAX_UNPOOLING_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
impl ::core::marker::Copy for DML_MEAN_VARIANCE_NORMALIZATION1_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_MEAN_VARIANCE_NORMALIZATION1_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
impl ::core::marker::Copy for DML_MEAN_VARIANCE_NORMALIZATION_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_MEAN_VARIANCE_NORMALIZATION_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DML_MINIMUM_BUFFER_TENSOR_ALIGNMENT: u32 = 16u32;
#[repr(C)]
pub struct DML_NONZERO_COORDINATES_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputCountTensor: *mut DML_TENSOR_DESC,
    pub OutputCoordinatesTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_NONZERO_COORDINATES_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_NONZERO_COORDINATES_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ONE_HOT_OPERATOR_DESC {
    pub IndicesTensor: *mut DML_TENSOR_DESC,
    pub ValuesTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Axis: u32,
}
impl ::core::marker::Copy for DML_ONE_HOT_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ONE_HOT_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_OPERATOR_DESC {
    pub Type: DML_OPERATOR_TYPE,
    pub Desc: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DML_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DML_OPERATOR_GRAPH_NODE_DESC {
    pub Operator: IDMLOperator,
    pub Name: super::super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DML_OPERATOR_GRAPH_NODE_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_OPERATOR_GRAPH_NODE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DML_OPERATOR_INVALID: i32 = 0i32;
pub const DML_OPERATOR_ELEMENT_WISE_IDENTITY: i32 = 1i32;
pub const DML_OPERATOR_ELEMENT_WISE_ABS: i32 = 2i32;
pub const DML_OPERATOR_ELEMENT_WISE_ACOS: i32 = 3i32;
pub const DML_OPERATOR_ELEMENT_WISE_ADD: i32 = 4i32;
pub const DML_OPERATOR_ELEMENT_WISE_ASIN: i32 = 5i32;
pub const DML_OPERATOR_ELEMENT_WISE_ATAN: i32 = 6i32;
pub const DML_OPERATOR_ELEMENT_WISE_CEIL: i32 = 7i32;
pub const DML_OPERATOR_ELEMENT_WISE_CLIP: i32 = 8i32;
pub const DML_OPERATOR_ELEMENT_WISE_COS: i32 = 9i32;
pub const DML_OPERATOR_ELEMENT_WISE_DIVIDE: i32 = 10i32;
pub const DML_OPERATOR_ELEMENT_WISE_EXP: i32 = 11i32;
pub const DML_OPERATOR_ELEMENT_WISE_FLOOR: i32 = 12i32;
pub const DML_OPERATOR_ELEMENT_WISE_LOG: i32 = 13i32;
pub const DML_OPERATOR_ELEMENT_WISE_LOGICAL_AND: i32 = 14i32;
pub const DML_OPERATOR_ELEMENT_WISE_LOGICAL_EQUALS: i32 = 15i32;
pub const DML_OPERATOR_ELEMENT_WISE_LOGICAL_GREATER_THAN: i32 = 16i32;
pub const DML_OPERATOR_ELEMENT_WISE_LOGICAL_LESS_THAN: i32 = 17i32;
pub const DML_OPERATOR_ELEMENT_WISE_LOGICAL_NOT: i32 = 18i32;
pub const DML_OPERATOR_ELEMENT_WISE_LOGICAL_OR: i32 = 19i32;
pub const DML_OPERATOR_ELEMENT_WISE_LOGICAL_XOR: i32 = 20i32;
pub const DML_OPERATOR_ELEMENT_WISE_MAX: i32 = 21i32;
pub const DML_OPERATOR_ELEMENT_WISE_MEAN: i32 = 22i32;
pub const DML_OPERATOR_ELEMENT_WISE_MIN: i32 = 23i32;
pub const DML_OPERATOR_ELEMENT_WISE_MULTIPLY: i32 = 24i32;
pub const DML_OPERATOR_ELEMENT_WISE_POW: i32 = 25i32;
pub const DML_OPERATOR_ELEMENT_WISE_CONSTANT_POW: i32 = 26i32;
pub const DML_OPERATOR_ELEMENT_WISE_RECIP: i32 = 27i32;
pub const DML_OPERATOR_ELEMENT_WISE_SIN: i32 = 28i32;
pub const DML_OPERATOR_ELEMENT_WISE_SQRT: i32 = 29i32;
pub const DML_OPERATOR_ELEMENT_WISE_SUBTRACT: i32 = 30i32;
pub const DML_OPERATOR_ELEMENT_WISE_TAN: i32 = 31i32;
pub const DML_OPERATOR_ELEMENT_WISE_THRESHOLD: i32 = 32i32;
pub const DML_OPERATOR_ELEMENT_WISE_QUANTIZE_LINEAR: i32 = 33i32;
pub const DML_OPERATOR_ELEMENT_WISE_DEQUANTIZE_LINEAR: i32 = 34i32;
pub const DML_OPERATOR_ACTIVATION_ELU: i32 = 35i32;
pub const DML_OPERATOR_ACTIVATION_HARDMAX: i32 = 36i32;
pub const DML_OPERATOR_ACTIVATION_HARD_SIGMOID: i32 = 37i32;
pub const DML_OPERATOR_ACTIVATION_IDENTITY: i32 = 38i32;
pub const DML_OPERATOR_ACTIVATION_LEAKY_RELU: i32 = 39i32;
pub const DML_OPERATOR_ACTIVATION_LINEAR: i32 = 40i32;
pub const DML_OPERATOR_ACTIVATION_LOG_SOFTMAX: i32 = 41i32;
pub const DML_OPERATOR_ACTIVATION_PARAMETERIZED_RELU: i32 = 42i32;
pub const DML_OPERATOR_ACTIVATION_PARAMETRIC_SOFTPLUS: i32 = 43i32;
pub const DML_OPERATOR_ACTIVATION_RELU: i32 = 44i32;
pub const DML_OPERATOR_ACTIVATION_SCALED_ELU: i32 = 45i32;
pub const DML_OPERATOR_ACTIVATION_SCALED_TANH: i32 = 46i32;
pub const DML_OPERATOR_ACTIVATION_SIGMOID: i32 = 47i32;
pub const DML_OPERATOR_ACTIVATION_SOFTMAX: i32 = 48i32;
pub const DML_OPERATOR_ACTIVATION_SOFTPLUS: i32 = 49i32;
pub const DML_OPERATOR_ACTIVATION_SOFTSIGN: i32 = 50i32;
pub const DML_OPERATOR_ACTIVATION_TANH: i32 = 51i32;
pub const DML_OPERATOR_ACTIVATION_THRESHOLDED_RELU: i32 = 52i32;
pub const DML_OPERATOR_CONVOLUTION: i32 = 53i32;
pub const DML_OPERATOR_GEMM: i32 = 54i32;
pub const DML_OPERATOR_REDUCE: i32 = 55i32;
pub const DML_OPERATOR_AVERAGE_POOLING: i32 = 56i32;
pub const DML_OPERATOR_LP_POOLING: i32 = 57i32;
pub const DML_OPERATOR_MAX_POOLING: i32 = 58i32;
pub const DML_OPERATOR_ROI_POOLING: i32 = 59i32;
pub const DML_OPERATOR_SLICE: i32 = 60i32;
pub const DML_OPERATOR_CAST: i32 = 61i32;
pub const DML_OPERATOR_SPLIT: i32 = 62i32;
pub const DML_OPERATOR_JOIN: i32 = 63i32;
pub const DML_OPERATOR_PADDING: i32 = 64i32;
pub const DML_OPERATOR_VALUE_SCALE_2D: i32 = 65i32;
pub const DML_OPERATOR_UPSAMPLE_2D: i32 = 66i32;
pub const DML_OPERATOR_GATHER: i32 = 67i32;
pub const DML_OPERATOR_SPACE_TO_DEPTH: i32 = 68i32;
pub const DML_OPERATOR_DEPTH_TO_SPACE: i32 = 69i32;
pub const DML_OPERATOR_TILE: i32 = 70i32;
pub const DML_OPERATOR_TOP_K: i32 = 71i32;
pub const DML_OPERATOR_BATCH_NORMALIZATION: i32 = 72i32;
pub const DML_OPERATOR_MEAN_VARIANCE_NORMALIZATION: i32 = 73i32;
pub const DML_OPERATOR_LOCAL_RESPONSE_NORMALIZATION: i32 = 74i32;
pub const DML_OPERATOR_LP_NORMALIZATION: i32 = 75i32;
pub const DML_OPERATOR_RNN: i32 = 76i32;
pub const DML_OPERATOR_LSTM: i32 = 77i32;
pub const DML_OPERATOR_GRU: i32 = 78i32;
pub const DML_OPERATOR_ELEMENT_WISE_SIGN: i32 = 79i32;
pub const DML_OPERATOR_ELEMENT_WISE_IS_NAN: i32 = 80i32;
pub const DML_OPERATOR_ELEMENT_WISE_ERF: i32 = 81i32;
pub const DML_OPERATOR_ELEMENT_WISE_SINH: i32 = 82i32;
pub const DML_OPERATOR_ELEMENT_WISE_COSH: i32 = 83i32;
pub const DML_OPERATOR_ELEMENT_WISE_TANH: i32 = 84i32;
pub const DML_OPERATOR_ELEMENT_WISE_ASINH: i32 = 85i32;
pub const DML_OPERATOR_ELEMENT_WISE_ACOSH: i32 = 86i32;
pub const DML_OPERATOR_ELEMENT_WISE_ATANH: i32 = 87i32;
pub const DML_OPERATOR_ELEMENT_WISE_IF: i32 = 88i32;
pub const DML_OPERATOR_ELEMENT_WISE_ADD1: i32 = 89i32;
pub const DML_OPERATOR_ACTIVATION_SHRINK: i32 = 90i32;
pub const DML_OPERATOR_MAX_POOLING1: i32 = 91i32;
pub const DML_OPERATOR_MAX_UNPOOLING: i32 = 92i32;
pub const DML_OPERATOR_DIAGONAL_MATRIX: i32 = 93i32;
pub const DML_OPERATOR_SCATTER_ELEMENTS: i32 = 94i32;
pub const DML_OPERATOR_SCATTER: i32 = 94i32;
pub const DML_OPERATOR_ONE_HOT: i32 = 95i32;
pub const DML_OPERATOR_RESAMPLE: i32 = 96i32;
pub const DML_OPERATOR_ELEMENT_WISE_BIT_SHIFT_LEFT: i32 = 97i32;
pub const DML_OPERATOR_ELEMENT_WISE_BIT_SHIFT_RIGHT: i32 = 98i32;
pub const DML_OPERATOR_ELEMENT_WISE_ROUND: i32 = 99i32;
pub const DML_OPERATOR_ELEMENT_WISE_IS_INFINITY: i32 = 100i32;
pub const DML_OPERATOR_ELEMENT_WISE_MODULUS_TRUNCATE: i32 = 101i32;
pub const DML_OPERATOR_ELEMENT_WISE_MODULUS_FLOOR: i32 = 102i32;
pub const DML_OPERATOR_FILL_VALUE_CONSTANT: i32 = 103i32;
pub const DML_OPERATOR_FILL_VALUE_SEQUENCE: i32 = 104i32;
pub const DML_OPERATOR_CUMULATIVE_SUMMATION: i32 = 105i32;
pub const DML_OPERATOR_REVERSE_SUBSEQUENCES: i32 = 106i32;
pub const DML_OPERATOR_GATHER_ELEMENTS: i32 = 107i32;
pub const DML_OPERATOR_GATHER_ND: i32 = 108i32;
pub const DML_OPERATOR_SCATTER_ND: i32 = 109i32;
pub const DML_OPERATOR_MAX_POOLING2: i32 = 110i32;
pub const DML_OPERATOR_SLICE1: i32 = 111i32;
pub const DML_OPERATOR_TOP_K1: i32 = 112i32;
pub const DML_OPERATOR_DEPTH_TO_SPACE1: i32 = 113i32;
pub const DML_OPERATOR_SPACE_TO_DEPTH1: i32 = 114i32;
pub const DML_OPERATOR_MEAN_VARIANCE_NORMALIZATION1: i32 = 115i32;
pub const DML_OPERATOR_RESAMPLE1: i32 = 116i32;
pub const DML_OPERATOR_MATRIX_MULTIPLY_INTEGER: i32 = 117i32;
pub const DML_OPERATOR_QUANTIZED_LINEAR_MATRIX_MULTIPLY: i32 = 118i32;
pub const DML_OPERATOR_CONVOLUTION_INTEGER: i32 = 119i32;
pub const DML_OPERATOR_QUANTIZED_LINEAR_CONVOLUTION: i32 = 120i32;
pub const DML_OPERATOR_ELEMENT_WISE_BIT_AND: i32 = 121i32;
pub const DML_OPERATOR_ELEMENT_WISE_BIT_OR: i32 = 122i32;
pub const DML_OPERATOR_ELEMENT_WISE_BIT_XOR: i32 = 123i32;
pub const DML_OPERATOR_ELEMENT_WISE_BIT_NOT: i32 = 124i32;
pub const DML_OPERATOR_ELEMENT_WISE_BIT_COUNT: i32 = 125i32;
pub const DML_OPERATOR_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL: i32 = 126i32;
pub const DML_OPERATOR_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL: i32 = 127i32;
pub const DML_OPERATOR_ACTIVATION_CELU: i32 = 128i32;
pub const DML_OPERATOR_ACTIVATION_RELU_GRAD: i32 = 129i32;
pub const DML_OPERATOR_AVERAGE_POOLING_GRAD: i32 = 130i32;
pub const DML_OPERATOR_MAX_POOLING_GRAD: i32 = 131i32;
pub const DML_OPERATOR_RANDOM_GENERATOR: i32 = 132i32;
pub const DML_OPERATOR_NONZERO_COORDINATES: i32 = 133i32;
pub const DML_OPERATOR_RESAMPLE_GRAD: i32 = 134i32;
pub const DML_OPERATOR_SLICE_GRAD: i32 = 135i32;
pub const DML_OPERATOR_ADAM_OPTIMIZER: i32 = 136i32;
pub const DML_OPERATOR_ARGMIN: i32 = 137i32;
pub const DML_OPERATOR_ARGMAX: i32 = 138i32;
pub const DML_OPERATOR_ROI_ALIGN: i32 = 139i32;
pub const DML_OPERATOR_GATHER_ND1: i32 = 140i32;
pub const DML_OPERATOR_ELEMENT_WISE_ATAN_YX: i32 = 141i32;
pub const DML_OPERATOR_ELEMENT_WISE_CLIP_GRAD: i32 = 142i32;
pub const DML_OPERATOR_ELEMENT_WISE_DIFFERENCE_SQUARE: i32 = 143i32;
pub const DML_OPERATOR_LOCAL_RESPONSE_NORMALIZATION_GRAD: i32 = 144i32;
pub const DML_OPERATOR_CUMULATIVE_PRODUCT: i32 = 145i32;
pub const DML_OPERATOR_BATCH_NORMALIZATION_GRAD: i32 = 146i32;
pub const DML_OPERATOR_ELEMENT_WISE_QUANTIZED_LINEAR_ADD: i32 = 147i32;
pub const DML_OPERATOR_DYNAMIC_QUANTIZE_LINEAR: i32 = 148i32;
pub const DML_OPERATOR_ROI_ALIGN1: i32 = 149i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DML_OUTPUT_GRAPH_EDGE_DESC {
    pub FromNodeIndex: u32,
    pub FromNodeOutputIndex: u32,
    pub GraphOutputIndex: u32,
    pub Name: super::super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DML_OUTPUT_GRAPH_EDGE_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_OUTPUT_GRAPH_EDGE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DML_PADDING_MODE_CONSTANT: i32 = 0i32;
pub const DML_PADDING_MODE_EDGE: i32 = 1i32;
pub const DML_PADDING_MODE_REFLECTION: i32 = 2i32;
pub const DML_PADDING_MODE_SYMMETRIC: i32 = 3i32;
#[repr(C)]
pub struct DML_PADDING_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub PaddingMode: DML_PADDING_MODE,
    pub PaddingValue: f32,
    pub DimensionCount: u32,
    pub StartPadding: *mut u32,
    pub EndPadding: *mut u32,
}
impl ::core::marker::Copy for DML_PADDING_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_PADDING_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DML_PERSISTENT_BUFFER_ALIGNMENT: u32 = 256u32;
#[repr(C)]
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
impl ::core::marker::Copy for DML_QUANTIZED_LINEAR_CONVOLUTION_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_QUANTIZED_LINEAR_CONVOLUTION_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
impl ::core::marker::Copy for DML_QUANTIZED_LINEAR_MATRIX_MULTIPLY_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_QUANTIZED_LINEAR_MATRIX_MULTIPLY_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_RANDOM_GENERATOR_OPERATOR_DESC {
    pub InputStateTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub OutputStateTensor: *mut DML_TENSOR_DESC,
    pub Type: DML_RANDOM_GENERATOR_TYPE,
}
impl ::core::marker::Copy for DML_RANDOM_GENERATOR_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_RANDOM_GENERATOR_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DML_RANDOM_GENERATOR_TYPE_PHILOX_4X32_10: i32 = 0i32;
pub const DML_RECURRENT_NETWORK_DIRECTION_FORWARD: i32 = 0i32;
pub const DML_RECURRENT_NETWORK_DIRECTION_BACKWARD: i32 = 1i32;
pub const DML_RECURRENT_NETWORK_DIRECTION_BIDIRECTIONAL: i32 = 2i32;
pub const DML_REDUCE_FUNCTION_ARGMAX: i32 = 0i32;
pub const DML_REDUCE_FUNCTION_ARGMIN: i32 = 1i32;
pub const DML_REDUCE_FUNCTION_AVERAGE: i32 = 2i32;
pub const DML_REDUCE_FUNCTION_L1: i32 = 3i32;
pub const DML_REDUCE_FUNCTION_L2: i32 = 4i32;
pub const DML_REDUCE_FUNCTION_LOG_SUM: i32 = 5i32;
pub const DML_REDUCE_FUNCTION_LOG_SUM_EXP: i32 = 6i32;
pub const DML_REDUCE_FUNCTION_MAX: i32 = 7i32;
pub const DML_REDUCE_FUNCTION_MIN: i32 = 8i32;
pub const DML_REDUCE_FUNCTION_MULTIPLY: i32 = 9i32;
pub const DML_REDUCE_FUNCTION_SUM: i32 = 10i32;
pub const DML_REDUCE_FUNCTION_SUM_SQUARE: i32 = 11i32;
#[repr(C)]
pub struct DML_REDUCE_OPERATOR_DESC {
    pub Function: DML_REDUCE_FUNCTION,
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub AxisCount: u32,
    pub Axes: *mut u32,
}
impl ::core::marker::Copy for DML_REDUCE_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_REDUCE_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_RESAMPLE1_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub InterpolationMode: DML_INTERPOLATION_MODE,
    pub DimensionCount: u32,
    pub Scales: *mut f32,
    pub InputPixelOffsets: *mut f32,
    pub OutputPixelOffsets: *mut f32,
}
impl ::core::marker::Copy for DML_RESAMPLE1_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_RESAMPLE1_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_RESAMPLE_GRAD_OPERATOR_DESC {
    pub InputGradientTensor: *mut DML_TENSOR_DESC,
    pub OutputGradientTensor: *mut DML_TENSOR_DESC,
    pub InterpolationMode: DML_INTERPOLATION_MODE,
    pub DimensionCount: u32,
    pub Scales: *mut f32,
    pub InputPixelOffsets: *mut f32,
    pub OutputPixelOffsets: *mut f32,
}
impl ::core::marker::Copy for DML_RESAMPLE_GRAD_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_RESAMPLE_GRAD_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_RESAMPLE_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub InterpolationMode: DML_INTERPOLATION_MODE,
    pub ScaleCount: u32,
    pub Scales: *mut f32,
}
impl ::core::marker::Copy for DML_RESAMPLE_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_RESAMPLE_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_REVERSE_SUBSEQUENCES_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub SequenceLengthsTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Axis: u32,
}
impl ::core::marker::Copy for DML_REVERSE_SUBSEQUENCES_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_REVERSE_SUBSEQUENCES_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
impl ::core::marker::Copy for DML_RNN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_RNN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
impl ::core::marker::Copy for DML_ROI_ALIGN1_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_ROI_ALIGN1_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
impl ::core::marker::Copy for DML_ROI_ALIGN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ROI_ALIGN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_ROI_POOLING_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub ROITensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub SpatialScale: f32,
    pub PooledSize: DML_SIZE_2D,
}
impl ::core::marker::Copy for DML_ROI_POOLING_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ROI_POOLING_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DML_ROUNDING_MODE_HALVES_TO_NEAREST_EVEN: i32 = 0i32;
pub const DML_ROUNDING_MODE_TOWARD_ZERO: i32 = 1i32;
pub const DML_ROUNDING_MODE_TOWARD_INFINITY: i32 = 2i32;
#[repr(C)]
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
impl ::core::marker::Copy for DML_SCALAR_UNION {}
impl ::core::clone::Clone for DML_SCALAR_UNION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_SCALE_BIAS {
    pub Scale: f32,
    pub Bias: f32,
}
impl ::core::marker::Copy for DML_SCALE_BIAS {}
impl ::core::clone::Clone for DML_SCALE_BIAS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_SCATTER_ND_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub IndicesTensor: *mut DML_TENSOR_DESC,
    pub UpdatesTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub InputDimensionCount: u32,
    pub IndicesDimensionCount: u32,
}
impl ::core::marker::Copy for DML_SCATTER_ND_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_SCATTER_ND_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_SCATTER_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub IndicesTensor: *mut DML_TENSOR_DESC,
    pub UpdatesTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Axis: u32,
}
impl ::core::marker::Copy for DML_SCATTER_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_SCATTER_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_SIZE_2D {
    pub Width: u32,
    pub Height: u32,
}
impl ::core::marker::Copy for DML_SIZE_2D {}
impl ::core::clone::Clone for DML_SIZE_2D {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_SLICE1_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub InputWindowOffsets: *mut u32,
    pub InputWindowSizes: *mut u32,
    pub InputWindowStrides: *mut i32,
}
impl ::core::marker::Copy for DML_SLICE1_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_SLICE1_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_SLICE_GRAD_OPERATOR_DESC {
    pub InputGradientTensor: *mut DML_TENSOR_DESC,
    pub OutputGradientTensor: *mut DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub InputWindowOffsets: *mut u32,
    pub InputWindowSizes: *mut u32,
    pub InputWindowStrides: *mut i32,
}
impl ::core::marker::Copy for DML_SLICE_GRAD_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_SLICE_GRAD_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_SLICE_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub Offsets: *mut u32,
    pub Sizes: *mut u32,
    pub Strides: *mut u32,
}
impl ::core::marker::Copy for DML_SLICE_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_SLICE_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_SPACE_TO_DEPTH1_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub BlockSize: u32,
    pub Order: DML_DEPTH_SPACE_ORDER,
}
impl ::core::marker::Copy for DML_SPACE_TO_DEPTH1_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_SPACE_TO_DEPTH1_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_SPACE_TO_DEPTH_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub BlockSize: u32,
}
impl ::core::marker::Copy for DML_SPACE_TO_DEPTH_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_SPACE_TO_DEPTH_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_SPLIT_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputCount: u32,
    pub OutputTensors: *mut DML_TENSOR_DESC,
    pub Axis: u32,
}
impl ::core::marker::Copy for DML_SPLIT_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_SPLIT_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DML_TARGET_VERSION: u32 = 16384u32;
pub const DML_TEMPORARY_BUFFER_ALIGNMENT: u32 = 256u32;
pub const DML_TENSOR_DATA_TYPE_UNKNOWN: i32 = 0i32;
pub const DML_TENSOR_DATA_TYPE_FLOAT32: i32 = 1i32;
pub const DML_TENSOR_DATA_TYPE_FLOAT16: i32 = 2i32;
pub const DML_TENSOR_DATA_TYPE_UINT32: i32 = 3i32;
pub const DML_TENSOR_DATA_TYPE_UINT16: i32 = 4i32;
pub const DML_TENSOR_DATA_TYPE_UINT8: i32 = 5i32;
pub const DML_TENSOR_DATA_TYPE_INT32: i32 = 6i32;
pub const DML_TENSOR_DATA_TYPE_INT16: i32 = 7i32;
pub const DML_TENSOR_DATA_TYPE_INT8: i32 = 8i32;
pub const DML_TENSOR_DATA_TYPE_FLOAT64: i32 = 9i32;
pub const DML_TENSOR_DATA_TYPE_UINT64: i32 = 10i32;
pub const DML_TENSOR_DATA_TYPE_INT64: i32 = 11i32;
#[repr(C)]
pub struct DML_TENSOR_DESC {
    pub Type: DML_TENSOR_TYPE,
    pub Desc: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DML_TENSOR_DESC {}
impl ::core::clone::Clone for DML_TENSOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DML_TENSOR_DIMENSION_COUNT_MAX: u32 = 5u32;
pub const DML_TENSOR_DIMENSION_COUNT_MAX1: u32 = 8u32;
pub const DML_TENSOR_FLAG_NONE: u32 = 0u32;
pub const DML_TENSOR_FLAG_OWNED_BY_DML: u32 = 1u32;
pub const DML_TENSOR_TYPE_INVALID: i32 = 0i32;
pub const DML_TENSOR_TYPE_BUFFER: i32 = 1i32;
#[repr(C)]
pub struct DML_TILE_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub RepeatsCount: u32,
    pub Repeats: *mut u32,
}
impl ::core::marker::Copy for DML_TILE_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_TILE_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_TOP_K1_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputValueTensor: *mut DML_TENSOR_DESC,
    pub OutputIndexTensor: *mut DML_TENSOR_DESC,
    pub Axis: u32,
    pub K: u32,
    pub AxisDirection: DML_AXIS_DIRECTION,
}
impl ::core::marker::Copy for DML_TOP_K1_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_TOP_K1_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_TOP_K_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputValueTensor: *mut DML_TENSOR_DESC,
    pub OutputIndexTensor: *mut DML_TENSOR_DESC,
    pub Axis: u32,
    pub K: u32,
}
impl ::core::marker::Copy for DML_TOP_K_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_TOP_K_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_UPSAMPLE_2D_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleSize: DML_SIZE_2D,
    pub InterpolationMode: DML_INTERPOLATION_MODE,
}
impl ::core::marker::Copy for DML_UPSAMPLE_2D_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_UPSAMPLE_2D_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DML_VALUE_SCALE_2D_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Scale: f32,
    pub ChannelCount: u32,
    pub Bias: *mut f32,
}
impl ::core::marker::Copy for DML_VALUE_SCALE_2D_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_VALUE_SCALE_2D_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDMLBindingTable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDMLBindingTable {}
impl ::core::clone::Clone for IDMLBindingTable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDMLCommandRecorder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDMLCommandRecorder {}
impl ::core::clone::Clone for IDMLCommandRecorder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDMLCompiledOperator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDMLCompiledOperator {}
impl ::core::clone::Clone for IDMLCompiledOperator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDMLDebugDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDMLDebugDevice {}
impl ::core::clone::Clone for IDMLDebugDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDMLDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDMLDevice {}
impl ::core::clone::Clone for IDMLDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDMLDevice1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDMLDevice1 {}
impl ::core::clone::Clone for IDMLDevice1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDMLDeviceChild(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDMLDeviceChild {}
impl ::core::clone::Clone for IDMLDeviceChild {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDMLDispatchable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDMLDispatchable {}
impl ::core::clone::Clone for IDMLDispatchable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDMLObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDMLObject {}
impl ::core::clone::Clone for IDMLObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDMLOperator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDMLOperator {}
impl ::core::clone::Clone for IDMLOperator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDMLOperatorInitializer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDMLOperatorInitializer {}
impl ::core::clone::Clone for IDMLOperatorInitializer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDMLPageable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDMLPageable {}
impl ::core::clone::Clone for IDMLPageable {
    fn clone(&self) -> Self {
        *self
    }
}
