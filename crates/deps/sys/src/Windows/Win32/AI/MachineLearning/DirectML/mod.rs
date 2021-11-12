#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub fn DMLCreateDevice(d3d12device: super::super::super::Graphics::Direct3D12::ID3D12Device, flags: DML_CREATE_DEVICE_FLAGS, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub fn DMLCreateDevice1(d3d12device: super::super::super::Graphics::Direct3D12::ID3D12Device, flags: DML_CREATE_DEVICE_FLAGS, minimumfeaturelevel: DML_FEATURE_LEVEL, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
}
#[repr(C)]
pub struct DML_ACTIVATION_CELU_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ACTIVATION_ELU_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ACTIVATION_HARDMAX_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ACTIVATION_HARD_SIGMOID_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ACTIVATION_IDENTITY_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ACTIVATION_LEAKY_RELU_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ACTIVATION_LINEAR_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ACTIVATION_LOG_SOFTMAX_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ACTIVATION_PARAMETERIZED_RELU_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ACTIVATION_PARAMETRIC_SOFTPLUS_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ACTIVATION_RELU_GRAD_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ACTIVATION_RELU_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ACTIVATION_SCALED_ELU_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ACTIVATION_SCALED_TANH_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ACTIVATION_SHRINK_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ACTIVATION_SIGMOID_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ACTIVATION_SOFTMAX_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ACTIVATION_SOFTPLUS_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ACTIVATION_SOFTSIGN_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ACTIVATION_TANH_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ACTIVATION_THRESHOLDED_RELU_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ADAM_OPTIMIZER_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ARGMAX_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ARGMIN_OPERATOR_DESC(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DML_AVERAGE_POOLING_GRAD_OPERATOR_DESC(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DML_AVERAGE_POOLING_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_AXIS_DIRECTION(i32);
#[repr(C)]
pub struct DML_BATCH_NORMALIZATION_GRAD_OPERATOR_DESC(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DML_BATCH_NORMALIZATION_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_BINDING_DESC(i32);
#[repr(C)]
pub struct DML_BINDING_PROPERTIES(i32);
#[cfg(feature = "Win32_Graphics_Direct3D12")]
#[repr(C)]
pub struct DML_BINDING_TABLE_DESC(i32);
#[repr(C)]
pub struct DML_BINDING_TYPE(i32);
#[cfg(feature = "Win32_Graphics_Direct3D12")]
#[repr(C)]
pub struct DML_BUFFER_ARRAY_BINDING(i32);
#[cfg(feature = "Win32_Graphics_Direct3D12")]
#[repr(C)]
pub struct DML_BUFFER_BINDING(i32);
#[repr(C)]
pub struct DML_BUFFER_TENSOR_DESC(i32);
#[repr(C)]
pub struct DML_CAST_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_CONVOLUTION_DIRECTION(i32);
#[repr(C)]
pub struct DML_CONVOLUTION_INTEGER_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_CONVOLUTION_MODE(i32);
#[repr(C)]
pub struct DML_CONVOLUTION_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_CREATE_DEVICE_FLAGS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DML_CUMULATIVE_PRODUCT_OPERATOR_DESC(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DML_CUMULATIVE_SUMMATION_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_DEPTH_SPACE_ORDER(i32);
#[repr(C)]
pub struct DML_DEPTH_TO_SPACE1_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_DEPTH_TO_SPACE_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_DIAGONAL_MATRIX_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_DYNAMIC_QUANTIZE_LINEAR_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_ABS_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_ACOSH_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_ACOS_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_ADD1_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_ADD_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_ASINH_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_ASIN_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_ATANH_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_ATAN_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_ATAN_YX_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_BIT_AND_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_BIT_COUNT_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_BIT_NOT_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_BIT_OR_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_BIT_SHIFT_LEFT_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_BIT_SHIFT_RIGHT_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_BIT_XOR_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_CEIL_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_CLIP_GRAD_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_CLIP_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_COSH_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_COS_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_DEQUANTIZE_LINEAR_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_DIFFERENCE_SQUARE_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_DIVIDE_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_ERF_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_EXP_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_FLOOR_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_IDENTITY_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_IF_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_IS_INFINITY_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_IS_NAN_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_LOGICAL_AND_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_LOGICAL_EQUALS_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_LOGICAL_NOT_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_LOGICAL_OR_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_LOGICAL_XOR_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_LOG_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_MAX_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_MEAN_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_MIN_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_MODULUS_FLOOR_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_MODULUS_TRUNCATE_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_MULTIPLY_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_POW_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_QUANTIZED_LINEAR_ADD_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_QUANTIZE_LINEAR_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_RECIP_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_ROUND_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_SIGN_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_SINH_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_SIN_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_SQRT_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_SUBTRACT_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_TANH_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_TAN_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_EXECUTION_FLAGS(i32);
#[repr(C)]
pub struct DML_FEATURE(i32);
#[repr(C)]
pub struct DML_FEATURE_DATA_FEATURE_LEVELS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DML_FEATURE_DATA_TENSOR_DATA_TYPE_SUPPORT(i32);
#[repr(C)]
pub struct DML_FEATURE_LEVEL(i32);
#[repr(C)]
pub struct DML_FEATURE_QUERY_FEATURE_LEVELS(i32);
#[repr(C)]
pub struct DML_FEATURE_QUERY_TENSOR_DATA_TYPE_SUPPORT(i32);
#[repr(C)]
pub struct DML_FILL_VALUE_CONSTANT_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_FILL_VALUE_SEQUENCE_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_GATHER_ELEMENTS_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_GATHER_ND1_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_GATHER_ND_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_GATHER_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_GEMM_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_GRAPH_DESC(i32);
#[repr(C)]
pub struct DML_GRAPH_EDGE_DESC(i32);
#[repr(C)]
pub struct DML_GRAPH_EDGE_TYPE(i32);
#[repr(C)]
pub struct DML_GRAPH_NODE_DESC(i32);
#[repr(C)]
pub struct DML_GRAPH_NODE_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DML_GRU_OPERATOR_DESC(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DML_INPUT_GRAPH_EDGE_DESC(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DML_INTERMEDIATE_GRAPH_EDGE_DESC(i32);
#[repr(C)]
pub struct DML_INTERPOLATION_MODE(i32);
#[repr(C)]
pub struct DML_IS_INFINITY_MODE(i32);
#[repr(C)]
pub struct DML_JOIN_OPERATOR_DESC(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DML_LOCAL_RESPONSE_NORMALIZATION_GRAD_OPERATOR_DESC(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DML_LOCAL_RESPONSE_NORMALIZATION_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_LP_NORMALIZATION_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_LP_POOLING_OPERATOR_DESC(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DML_LSTM_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_MATRIX_MULTIPLY_INTEGER_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_MATRIX_TRANSFORM(i32);
#[repr(C)]
pub struct DML_MAX_POOLING1_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_MAX_POOLING2_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_MAX_POOLING_GRAD_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_MAX_POOLING_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_MAX_UNPOOLING_OPERATOR_DESC(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DML_MEAN_VARIANCE_NORMALIZATION1_OPERATOR_DESC(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DML_MEAN_VARIANCE_NORMALIZATION_OPERATOR_DESC(i32);
pub const DML_MINIMUM_BUFFER_TENSOR_ALIGNMENT: u32 = 16u32;
#[repr(C)]
pub struct DML_NONZERO_COORDINATES_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ONE_HOT_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_OPERATOR_DESC(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DML_OPERATOR_GRAPH_NODE_DESC(i32);
#[repr(C)]
pub struct DML_OPERATOR_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DML_OUTPUT_GRAPH_EDGE_DESC(i32);
#[repr(C)]
pub struct DML_PADDING_MODE(i32);
#[repr(C)]
pub struct DML_PADDING_OPERATOR_DESC(i32);
pub const DML_PERSISTENT_BUFFER_ALIGNMENT: u32 = 256u32;
#[repr(C)]
pub struct DML_QUANTIZED_LINEAR_CONVOLUTION_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_QUANTIZED_LINEAR_MATRIX_MULTIPLY_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_RANDOM_GENERATOR_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_RANDOM_GENERATOR_TYPE(i32);
#[repr(C)]
pub struct DML_RECURRENT_NETWORK_DIRECTION(i32);
#[repr(C)]
pub struct DML_REDUCE_FUNCTION(i32);
#[repr(C)]
pub struct DML_REDUCE_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_RESAMPLE1_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_RESAMPLE_GRAD_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_RESAMPLE_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_REVERSE_SUBSEQUENCES_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_RNN_OPERATOR_DESC(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DML_ROI_ALIGN1_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ROI_ALIGN_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ROI_POOLING_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_ROUNDING_MODE(i32);
#[repr(C)]
pub struct DML_SCALAR_UNION(i32);
#[repr(C)]
pub struct DML_SCALE_BIAS(i32);
#[repr(C)]
pub struct DML_SCATTER_ND_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_SCATTER_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_SIZE_2D(i32);
#[repr(C)]
pub struct DML_SLICE1_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_SLICE_GRAD_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_SLICE_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_SPACE_TO_DEPTH1_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_SPACE_TO_DEPTH_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_SPLIT_OPERATOR_DESC(i32);
pub const DML_TARGET_VERSION: u32 = 16384u32;
pub const DML_TEMPORARY_BUFFER_ALIGNMENT: u32 = 256u32;
#[repr(C)]
pub struct DML_TENSOR_DATA_TYPE(i32);
#[repr(C)]
pub struct DML_TENSOR_DESC(i32);
pub const DML_TENSOR_DIMENSION_COUNT_MAX: u32 = 5u32;
pub const DML_TENSOR_DIMENSION_COUNT_MAX1: u32 = 8u32;
#[repr(C)]
pub struct DML_TENSOR_FLAGS(i32);
#[repr(C)]
pub struct DML_TENSOR_TYPE(i32);
#[repr(C)]
pub struct DML_TILE_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_TOP_K1_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_TOP_K_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_UPSAMPLE_2D_OPERATOR_DESC(i32);
#[repr(C)]
pub struct DML_VALUE_SCALE_2D_OPERATOR_DESC(i32);
#[repr(transparent)]
pub struct IDMLBindingTable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDMLCommandRecorder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDMLCompiledOperator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDMLDebugDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDMLDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDMLDevice1(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDMLDeviceChild(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDMLDispatchable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDMLObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDMLOperator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDMLOperatorInitializer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDMLPageable(pub *mut ::core::ffi::c_void);
