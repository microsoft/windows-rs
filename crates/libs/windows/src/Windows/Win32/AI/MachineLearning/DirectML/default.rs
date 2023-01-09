impl ::core::default::Default for DML_ACTIVATION_CELU_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_CELU_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Alpha == other.Alpha
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_CELU_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ACTIVATION_CELU_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_CELU_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Alpha", &self.Alpha).finish()
    }
}
impl ::core::default::Default for DML_ACTIVATION_ELU_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_ELU_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Alpha == other.Alpha
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_ELU_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ACTIVATION_ELU_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_ELU_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Alpha", &self.Alpha).finish()
    }
}
impl ::core::default::Default for DML_ACTIVATION_HARDMAX_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_HARDMAX_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_HARDMAX_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ACTIVATION_HARDMAX_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_HARDMAX_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ACTIVATION_HARD_SIGMOID_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_HARD_SIGMOID_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Alpha == other.Alpha && self.Beta == other.Beta
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_HARD_SIGMOID_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ACTIVATION_HARD_SIGMOID_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_HARD_SIGMOID_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Alpha", &self.Alpha).field("Beta", &self.Beta).finish()
    }
}
impl ::core::default::Default for DML_ACTIVATION_IDENTITY_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_IDENTITY_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_IDENTITY_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ACTIVATION_IDENTITY_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_IDENTITY_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ACTIVATION_LEAKY_RELU_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_LEAKY_RELU_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Alpha == other.Alpha
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_LEAKY_RELU_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ACTIVATION_LEAKY_RELU_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_LEAKY_RELU_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Alpha", &self.Alpha).finish()
    }
}
impl ::core::default::Default for DML_ACTIVATION_LINEAR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_LINEAR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Alpha == other.Alpha && self.Beta == other.Beta
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_LINEAR_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ACTIVATION_LINEAR_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_LINEAR_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Alpha", &self.Alpha).field("Beta", &self.Beta).finish()
    }
}
impl ::core::default::Default for DML_ACTIVATION_LOG_SOFTMAX_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_LOG_SOFTMAX_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_LOG_SOFTMAX_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ACTIVATION_LOG_SOFTMAX_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_LOG_SOFTMAX_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ACTIVATION_PARAMETERIZED_RELU_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_PARAMETERIZED_RELU_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.SlopeTensor == other.SlopeTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_PARAMETERIZED_RELU_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ACTIVATION_PARAMETERIZED_RELU_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_PARAMETERIZED_RELU_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("SlopeTensor", &self.SlopeTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ACTIVATION_PARAMETRIC_SOFTPLUS_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_PARAMETRIC_SOFTPLUS_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Alpha == other.Alpha && self.Beta == other.Beta
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_PARAMETRIC_SOFTPLUS_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ACTIVATION_PARAMETRIC_SOFTPLUS_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_PARAMETRIC_SOFTPLUS_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Alpha", &self.Alpha).field("Beta", &self.Beta).finish()
    }
}
impl ::core::default::Default for DML_ACTIVATION_RELU_GRAD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_RELU_GRAD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.InputGradientTensor == other.InputGradientTensor && self.OutputGradientTensor == other.OutputGradientTensor
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_RELU_GRAD_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ACTIVATION_RELU_GRAD_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_RELU_GRAD_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("InputGradientTensor", &self.InputGradientTensor).field("OutputGradientTensor", &self.OutputGradientTensor).finish()
    }
}
impl ::core::default::Default for DML_ACTIVATION_RELU_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_RELU_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_RELU_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ACTIVATION_RELU_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_RELU_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ACTIVATION_SCALED_ELU_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_SCALED_ELU_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Alpha == other.Alpha && self.Gamma == other.Gamma
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_SCALED_ELU_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ACTIVATION_SCALED_ELU_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_SCALED_ELU_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Alpha", &self.Alpha).field("Gamma", &self.Gamma).finish()
    }
}
impl ::core::default::Default for DML_ACTIVATION_SCALED_TANH_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_SCALED_TANH_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Alpha == other.Alpha && self.Beta == other.Beta
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_SCALED_TANH_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ACTIVATION_SCALED_TANH_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_SCALED_TANH_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Alpha", &self.Alpha).field("Beta", &self.Beta).finish()
    }
}
impl ::core::default::Default for DML_ACTIVATION_SHRINK_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_SHRINK_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Bias == other.Bias && self.Threshold == other.Threshold
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_SHRINK_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ACTIVATION_SHRINK_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_SHRINK_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Bias", &self.Bias).field("Threshold", &self.Threshold).finish()
    }
}
impl ::core::default::Default for DML_ACTIVATION_SIGMOID_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_SIGMOID_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_SIGMOID_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ACTIVATION_SIGMOID_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_SIGMOID_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ACTIVATION_SOFTMAX_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_SOFTMAX_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_SOFTMAX_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ACTIVATION_SOFTMAX_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_SOFTMAX_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ACTIVATION_SOFTPLUS_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_SOFTPLUS_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Steepness == other.Steepness
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_SOFTPLUS_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ACTIVATION_SOFTPLUS_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_SOFTPLUS_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Steepness", &self.Steepness).finish()
    }
}
impl ::core::default::Default for DML_ACTIVATION_SOFTSIGN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_SOFTSIGN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_SOFTSIGN_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ACTIVATION_SOFTSIGN_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_SOFTSIGN_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ACTIVATION_TANH_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_TANH_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_TANH_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ACTIVATION_TANH_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_TANH_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ACTIVATION_THRESHOLDED_RELU_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_THRESHOLDED_RELU_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Alpha == other.Alpha
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_THRESHOLDED_RELU_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ACTIVATION_THRESHOLDED_RELU_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_THRESHOLDED_RELU_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Alpha", &self.Alpha).finish()
    }
}
impl ::core::default::Default for DML_ADAM_OPTIMIZER_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ADAM_OPTIMIZER_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputParametersTensor == other.InputParametersTensor && self.InputFirstMomentTensor == other.InputFirstMomentTensor && self.InputSecondMomentTensor == other.InputSecondMomentTensor && self.GradientTensor == other.GradientTensor && self.TrainingStepTensor == other.TrainingStepTensor && self.OutputParametersTensor == other.OutputParametersTensor && self.OutputFirstMomentTensor == other.OutputFirstMomentTensor && self.OutputSecondMomentTensor == other.OutputSecondMomentTensor && self.LearningRate == other.LearningRate && self.Beta1 == other.Beta1 && self.Beta2 == other.Beta2 && self.Epsilon == other.Epsilon
    }
}
impl ::core::cmp::Eq for DML_ADAM_OPTIMIZER_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ADAM_OPTIMIZER_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ADAM_OPTIMIZER_OPERATOR_DESC")
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
impl ::core::default::Default for DML_ARGMAX_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ARGMAX_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.AxisCount == other.AxisCount && self.Axes == other.Axes && self.AxisDirection == other.AxisDirection
    }
}
impl ::core::cmp::Eq for DML_ARGMAX_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ARGMAX_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ARGMAX_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("AxisCount", &self.AxisCount).field("Axes", &self.Axes).field("AxisDirection", &self.AxisDirection).finish()
    }
}
impl ::core::default::Default for DML_ARGMIN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ARGMIN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.AxisCount == other.AxisCount && self.Axes == other.Axes && self.AxisDirection == other.AxisDirection
    }
}
impl ::core::cmp::Eq for DML_ARGMIN_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ARGMIN_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ARGMIN_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("AxisCount", &self.AxisCount).field("Axes", &self.Axes).field("AxisDirection", &self.AxisDirection).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_AVERAGE_POOLING_GRAD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DML_AVERAGE_POOLING_GRAD_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_AVERAGE_POOLING_GRAD_OPERATOR_DESC").field("InputGradientTensor", &self.InputGradientTensor).field("OutputGradientTensor", &self.OutputGradientTensor).field("DimensionCount", &self.DimensionCount).field("Strides", &self.Strides).field("WindowSize", &self.WindowSize).field("StartPadding", &self.StartPadding).field("EndPadding", &self.EndPadding).field("IncludePadding", &self.IncludePadding).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_AVERAGE_POOLING_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DML_AVERAGE_POOLING_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_AVERAGE_POOLING_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("DimensionCount", &self.DimensionCount).field("Strides", &self.Strides).field("WindowSize", &self.WindowSize).field("StartPadding", &self.StartPadding).field("EndPadding", &self.EndPadding).field("IncludePadding", &self.IncludePadding).finish()
    }
}
impl ::core::default::Default for DML_AXIS_DIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DML_AXIS_DIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_AXIS_DIRECTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for DML_BATCH_NORMALIZATION_GRAD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_BATCH_NORMALIZATION_GRAD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.InputGradientTensor == other.InputGradientTensor && self.MeanTensor == other.MeanTensor && self.VarianceTensor == other.VarianceTensor && self.ScaleTensor == other.ScaleTensor && self.OutputGradientTensor == other.OutputGradientTensor && self.OutputScaleGradientTensor == other.OutputScaleGradientTensor && self.OutputBiasGradientTensor == other.OutputBiasGradientTensor && self.Epsilon == other.Epsilon
    }
}
impl ::core::cmp::Eq for DML_BATCH_NORMALIZATION_GRAD_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_BATCH_NORMALIZATION_GRAD_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_BATCH_NORMALIZATION_GRAD_OPERATOR_DESC")
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_BATCH_NORMALIZATION_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DML_BATCH_NORMALIZATION_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_BATCH_NORMALIZATION_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("MeanTensor", &self.MeanTensor).field("VarianceTensor", &self.VarianceTensor).field("ScaleTensor", &self.ScaleTensor).field("BiasTensor", &self.BiasTensor).field("OutputTensor", &self.OutputTensor).field("Spatial", &self.Spatial).field("Epsilon", &self.Epsilon).field("FusedActivation", &self.FusedActivation).finish()
    }
}
impl ::core::default::Default for DML_BINDING_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_BINDING_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Desc == other.Desc
    }
}
impl ::core::cmp::Eq for DML_BINDING_DESC {}
impl ::core::fmt::Debug for DML_BINDING_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_BINDING_DESC").field("Type", &self.Type).field("Desc", &self.Desc).finish()
    }
}
impl ::core::default::Default for DML_BINDING_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_BINDING_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.RequiredDescriptorCount == other.RequiredDescriptorCount && self.TemporaryResourceSize == other.TemporaryResourceSize && self.PersistentResourceSize == other.PersistentResourceSize
    }
}
impl ::core::cmp::Eq for DML_BINDING_PROPERTIES {}
impl ::core::fmt::Debug for DML_BINDING_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_BINDING_PROPERTIES").field("RequiredDescriptorCount", &self.RequiredDescriptorCount).field("TemporaryResourceSize", &self.TemporaryResourceSize).field("PersistentResourceSize", &self.PersistentResourceSize).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::default::Default for DML_BINDING_TABLE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DML_BINDING_TABLE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_BINDING_TABLE_DESC").field("Dispatchable", &self.Dispatchable).field("CPUDescriptorHandle", &self.CPUDescriptorHandle).field("GPUDescriptorHandle", &self.GPUDescriptorHandle).field("SizeInDescriptors", &self.SizeInDescriptors).finish()
    }
}
impl ::core::default::Default for DML_BINDING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DML_BINDING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_BINDING_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::default::Default for DML_BUFFER_ARRAY_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DML_BUFFER_ARRAY_BINDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_BUFFER_ARRAY_BINDING").field("BindingCount", &self.BindingCount).field("Bindings", &self.Bindings).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::default::Default for DML_BUFFER_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DML_BUFFER_BINDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_BUFFER_BINDING").field("Buffer", &self.Buffer).field("Offset", &self.Offset).field("SizeInBytes", &self.SizeInBytes).finish()
    }
}
impl ::core::default::Default for DML_BUFFER_TENSOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_BUFFER_TENSOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.DataType == other.DataType && self.Flags == other.Flags && self.DimensionCount == other.DimensionCount && self.Sizes == other.Sizes && self.Strides == other.Strides && self.TotalTensorSizeInBytes == other.TotalTensorSizeInBytes && self.GuaranteedBaseOffsetAlignment == other.GuaranteedBaseOffsetAlignment
    }
}
impl ::core::cmp::Eq for DML_BUFFER_TENSOR_DESC {}
impl ::core::fmt::Debug for DML_BUFFER_TENSOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_BUFFER_TENSOR_DESC").field("DataType", &self.DataType).field("Flags", &self.Flags).field("DimensionCount", &self.DimensionCount).field("Sizes", &self.Sizes).field("Strides", &self.Strides).field("TotalTensorSizeInBytes", &self.TotalTensorSizeInBytes).field("GuaranteedBaseOffsetAlignment", &self.GuaranteedBaseOffsetAlignment).finish()
    }
}
impl ::core::default::Default for DML_CAST_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_CAST_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_CAST_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_CAST_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_CAST_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_CONVOLUTION_DIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DML_CONVOLUTION_DIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_CONVOLUTION_DIRECTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for DML_CONVOLUTION_INTEGER_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_CONVOLUTION_INTEGER_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.InputZeroPointTensor == other.InputZeroPointTensor && self.FilterTensor == other.FilterTensor && self.FilterZeroPointTensor == other.FilterZeroPointTensor && self.OutputTensor == other.OutputTensor && self.DimensionCount == other.DimensionCount && self.Strides == other.Strides && self.Dilations == other.Dilations && self.StartPadding == other.StartPadding && self.EndPadding == other.EndPadding && self.GroupCount == other.GroupCount
    }
}
impl ::core::cmp::Eq for DML_CONVOLUTION_INTEGER_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_CONVOLUTION_INTEGER_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_CONVOLUTION_INTEGER_OPERATOR_DESC")
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
impl ::core::default::Default for DML_CONVOLUTION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DML_CONVOLUTION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_CONVOLUTION_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DML_CONVOLUTION_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_CONVOLUTION_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.FilterTensor == other.FilterTensor && self.BiasTensor == other.BiasTensor && self.OutputTensor == other.OutputTensor && self.Mode == other.Mode && self.Direction == other.Direction && self.DimensionCount == other.DimensionCount && self.Strides == other.Strides && self.Dilations == other.Dilations && self.StartPadding == other.StartPadding && self.EndPadding == other.EndPadding && self.OutputPadding == other.OutputPadding && self.GroupCount == other.GroupCount && self.FusedActivation == other.FusedActivation
    }
}
impl ::core::cmp::Eq for DML_CONVOLUTION_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_CONVOLUTION_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_CONVOLUTION_OPERATOR_DESC")
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
impl ::core::default::Default for DML_CREATE_DEVICE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DML_CREATE_DEVICE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_CREATE_DEVICE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DML_CREATE_DEVICE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DML_CREATE_DEVICE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DML_CREATE_DEVICE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DML_CREATE_DEVICE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DML_CREATE_DEVICE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_CUMULATIVE_PRODUCT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DML_CUMULATIVE_PRODUCT_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_CUMULATIVE_PRODUCT_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Axis", &self.Axis).field("AxisDirection", &self.AxisDirection).field("HasExclusiveProduct", &self.HasExclusiveProduct).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_CUMULATIVE_SUMMATION_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DML_CUMULATIVE_SUMMATION_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_CUMULATIVE_SUMMATION_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Axis", &self.Axis).field("AxisDirection", &self.AxisDirection).field("HasExclusiveSum", &self.HasExclusiveSum).finish()
    }
}
impl ::core::default::Default for DML_DEPTH_SPACE_ORDER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DML_DEPTH_SPACE_ORDER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_DEPTH_SPACE_ORDER").field(&self.0).finish()
    }
}
impl ::core::default::Default for DML_DEPTH_TO_SPACE1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_DEPTH_TO_SPACE1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.BlockSize == other.BlockSize && self.Order == other.Order
    }
}
impl ::core::cmp::Eq for DML_DEPTH_TO_SPACE1_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_DEPTH_TO_SPACE1_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_DEPTH_TO_SPACE1_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("BlockSize", &self.BlockSize).field("Order", &self.Order).finish()
    }
}
impl ::core::default::Default for DML_DEPTH_TO_SPACE_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_DEPTH_TO_SPACE_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.BlockSize == other.BlockSize
    }
}
impl ::core::cmp::Eq for DML_DEPTH_TO_SPACE_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_DEPTH_TO_SPACE_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_DEPTH_TO_SPACE_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("BlockSize", &self.BlockSize).finish()
    }
}
impl ::core::default::Default for DML_DIAGONAL_MATRIX_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_DIAGONAL_MATRIX_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.OutputTensor == other.OutputTensor && self.Offset == other.Offset && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for DML_DIAGONAL_MATRIX_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_DIAGONAL_MATRIX_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_DIAGONAL_MATRIX_OPERATOR_DESC").field("OutputTensor", &self.OutputTensor).field("Offset", &self.Offset).field("Value", &self.Value).finish()
    }
}
impl ::core::default::Default for DML_DYNAMIC_QUANTIZE_LINEAR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_DYNAMIC_QUANTIZE_LINEAR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.OutputScaleTensor == other.OutputScaleTensor && self.OutputZeroPointTensor == other.OutputZeroPointTensor
    }
}
impl ::core::cmp::Eq for DML_DYNAMIC_QUANTIZE_LINEAR_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_DYNAMIC_QUANTIZE_LINEAR_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_DYNAMIC_QUANTIZE_LINEAR_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("OutputScaleTensor", &self.OutputScaleTensor).field("OutputZeroPointTensor", &self.OutputZeroPointTensor).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_ABS_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ABS_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ABS_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_ABS_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_ABS_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_ACOSH_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ACOSH_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ACOSH_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_ACOSH_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_ACOSH_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_ACOS_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ACOS_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ACOS_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_ACOS_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_ACOS_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_ADD1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ADD1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor && self.FusedActivation == other.FusedActivation
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ADD1_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_ADD1_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_ADD1_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).field("FusedActivation", &self.FusedActivation).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_ADD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ADD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ADD_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_ADD_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_ADD_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_ASINH_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ASINH_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ASINH_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_ASINH_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_ASINH_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_ASIN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ASIN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ASIN_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_ASIN_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_ASIN_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_ATANH_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ATANH_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ATANH_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_ATANH_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_ATANH_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_ATAN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ATAN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ATAN_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_ATAN_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_ATAN_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_ATAN_YX_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ATAN_YX_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ATAN_YX_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_ATAN_YX_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_ATAN_YX_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_BIT_AND_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_BIT_AND_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_BIT_AND_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_BIT_AND_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_BIT_AND_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_BIT_COUNT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_BIT_COUNT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_BIT_COUNT_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_BIT_COUNT_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_BIT_COUNT_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_BIT_NOT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_BIT_NOT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_BIT_NOT_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_BIT_NOT_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_BIT_NOT_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_BIT_OR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_BIT_OR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_BIT_OR_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_BIT_OR_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_BIT_OR_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_BIT_SHIFT_LEFT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_BIT_SHIFT_LEFT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_BIT_SHIFT_LEFT_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_BIT_SHIFT_LEFT_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_BIT_SHIFT_LEFT_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_BIT_SHIFT_RIGHT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_BIT_SHIFT_RIGHT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_BIT_SHIFT_RIGHT_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_BIT_SHIFT_RIGHT_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_BIT_SHIFT_RIGHT_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_BIT_XOR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_BIT_XOR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_BIT_XOR_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_BIT_XOR_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_BIT_XOR_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_CEIL_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_CEIL_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_CEIL_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_CEIL_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_CEIL_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_CLIP_GRAD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_CLIP_GRAD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.InputGradientTensor == other.InputGradientTensor && self.OutputGradientTensor == other.OutputGradientTensor && self.Min == other.Min && self.Max == other.Max
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_CLIP_GRAD_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_CLIP_GRAD_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_CLIP_GRAD_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("InputGradientTensor", &self.InputGradientTensor).field("OutputGradientTensor", &self.OutputGradientTensor).field("Min", &self.Min).field("Max", &self.Max).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_CLIP_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_CLIP_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias && self.Min == other.Min && self.Max == other.Max
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_CLIP_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_CLIP_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_CLIP_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).field("Min", &self.Min).field("Max", &self.Max).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias && self.Exponent == other.Exponent
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).field("Exponent", &self.Exponent).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_COSH_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_COSH_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_COSH_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_COSH_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_COSH_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_COS_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_COS_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_COS_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_COS_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_COS_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_DEQUANTIZE_LINEAR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_DEQUANTIZE_LINEAR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.ScaleTensor == other.ScaleTensor && self.ZeroPointTensor == other.ZeroPointTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_DEQUANTIZE_LINEAR_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_DEQUANTIZE_LINEAR_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_DEQUANTIZE_LINEAR_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("ScaleTensor", &self.ScaleTensor).field("ZeroPointTensor", &self.ZeroPointTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_DIFFERENCE_SQUARE_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_DIFFERENCE_SQUARE_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_DIFFERENCE_SQUARE_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_DIFFERENCE_SQUARE_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_DIFFERENCE_SQUARE_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_DIVIDE_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_DIVIDE_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_DIVIDE_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_DIVIDE_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_DIVIDE_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_ERF_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ERF_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ERF_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_ERF_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_ERF_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_EXP_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_EXP_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_EXP_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_EXP_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_EXP_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_FLOOR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_FLOOR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_FLOOR_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_FLOOR_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_FLOOR_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_IDENTITY_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_IDENTITY_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_IDENTITY_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_IDENTITY_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_IDENTITY_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_IF_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_IF_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ConditionTensor == other.ConditionTensor && self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_IF_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_IF_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_IF_OPERATOR_DESC").field("ConditionTensor", &self.ConditionTensor).field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_IS_INFINITY_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_IS_INFINITY_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.InfinityMode == other.InfinityMode
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_IS_INFINITY_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_IS_INFINITY_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_IS_INFINITY_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("InfinityMode", &self.InfinityMode).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_IS_NAN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_IS_NAN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_IS_NAN_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_IS_NAN_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_IS_NAN_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_LOGICAL_AND_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOGICAL_AND_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOGICAL_AND_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_LOGICAL_AND_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_LOGICAL_AND_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_LOGICAL_EQUALS_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOGICAL_EQUALS_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOGICAL_EQUALS_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_LOGICAL_EQUALS_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_LOGICAL_EQUALS_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_LOGICAL_NOT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOGICAL_NOT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOGICAL_NOT_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_LOGICAL_NOT_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_LOGICAL_NOT_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_LOGICAL_OR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOGICAL_OR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOGICAL_OR_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_LOGICAL_OR_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_LOGICAL_OR_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_LOGICAL_XOR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOGICAL_XOR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOGICAL_XOR_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_LOGICAL_XOR_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_LOGICAL_XOR_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_LOG_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOG_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOG_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_LOG_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_LOG_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_MAX_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_MAX_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_MAX_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_MAX_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_MAX_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_MEAN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_MEAN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_MEAN_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_MEAN_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_MEAN_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_MIN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_MIN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_MIN_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_MIN_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_MIN_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_MODULUS_FLOOR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_MODULUS_FLOOR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_MODULUS_FLOOR_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_MODULUS_FLOOR_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_MODULUS_FLOOR_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_MODULUS_TRUNCATE_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_MODULUS_TRUNCATE_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_MODULUS_TRUNCATE_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_MODULUS_TRUNCATE_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_MODULUS_TRUNCATE_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_MULTIPLY_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_MULTIPLY_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_MULTIPLY_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_MULTIPLY_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_MULTIPLY_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_POW_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_POW_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.ExponentTensor == other.ExponentTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_POW_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_POW_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_POW_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("ExponentTensor", &self.ExponentTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_QUANTIZED_LINEAR_ADD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_QUANTIZED_LINEAR_ADD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.AScaleTensor == other.AScaleTensor && self.AZeroPointTensor == other.AZeroPointTensor && self.BTensor == other.BTensor && self.BScaleTensor == other.BScaleTensor && self.BZeroPointTensor == other.BZeroPointTensor && self.OutputScaleTensor == other.OutputScaleTensor && self.OutputZeroPointTensor == other.OutputZeroPointTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_QUANTIZED_LINEAR_ADD_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_QUANTIZED_LINEAR_ADD_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_QUANTIZED_LINEAR_ADD_OPERATOR_DESC").field("ATensor", &self.ATensor).field("AScaleTensor", &self.AScaleTensor).field("AZeroPointTensor", &self.AZeroPointTensor).field("BTensor", &self.BTensor).field("BScaleTensor", &self.BScaleTensor).field("BZeroPointTensor", &self.BZeroPointTensor).field("OutputScaleTensor", &self.OutputScaleTensor).field("OutputZeroPointTensor", &self.OutputZeroPointTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_QUANTIZE_LINEAR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_QUANTIZE_LINEAR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.ScaleTensor == other.ScaleTensor && self.ZeroPointTensor == other.ZeroPointTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_QUANTIZE_LINEAR_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_QUANTIZE_LINEAR_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_QUANTIZE_LINEAR_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("ScaleTensor", &self.ScaleTensor).field("ZeroPointTensor", &self.ZeroPointTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_RECIP_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_RECIP_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_RECIP_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_RECIP_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_RECIP_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_ROUND_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ROUND_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.RoundingMode == other.RoundingMode
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ROUND_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_ROUND_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_ROUND_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("RoundingMode", &self.RoundingMode).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_SIGN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_SIGN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_SIGN_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_SIGN_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_SIGN_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_SINH_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_SINH_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_SINH_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_SINH_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_SINH_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_SIN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_SIN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_SIN_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_SIN_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_SIN_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_SQRT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_SQRT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_SQRT_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_SQRT_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_SQRT_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_SUBTRACT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_SUBTRACT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_SUBTRACT_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_SUBTRACT_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_SUBTRACT_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_TANH_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_TANH_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_TANH_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_TANH_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_TANH_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_TAN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_TAN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_TAN_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_TAN_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_TAN_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::core::default::Default for DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias && self.Min == other.Min
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).field("Min", &self.Min).finish()
    }
}
impl ::core::default::Default for DML_EXECUTION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DML_EXECUTION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_EXECUTION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DML_EXECUTION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DML_EXECUTION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DML_EXECUTION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DML_EXECUTION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DML_EXECUTION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for DML_FEATURE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DML_FEATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_FEATURE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DML_FEATURE_DATA_FEATURE_LEVELS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_FEATURE_DATA_FEATURE_LEVELS {
    fn eq(&self, other: &Self) -> bool {
        self.MaxSupportedFeatureLevel == other.MaxSupportedFeatureLevel
    }
}
impl ::core::cmp::Eq for DML_FEATURE_DATA_FEATURE_LEVELS {}
impl ::core::fmt::Debug for DML_FEATURE_DATA_FEATURE_LEVELS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_FEATURE_DATA_FEATURE_LEVELS").field("MaxSupportedFeatureLevel", &self.MaxSupportedFeatureLevel).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_FEATURE_DATA_TENSOR_DATA_TYPE_SUPPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DML_FEATURE_DATA_TENSOR_DATA_TYPE_SUPPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_FEATURE_DATA_TENSOR_DATA_TYPE_SUPPORT").field("IsSupported", &self.IsSupported).finish()
    }
}
impl ::core::default::Default for DML_FEATURE_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DML_FEATURE_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_FEATURE_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for DML_FEATURE_QUERY_FEATURE_LEVELS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_FEATURE_QUERY_FEATURE_LEVELS {
    fn eq(&self, other: &Self) -> bool {
        self.RequestedFeatureLevelCount == other.RequestedFeatureLevelCount && self.RequestedFeatureLevels == other.RequestedFeatureLevels
    }
}
impl ::core::cmp::Eq for DML_FEATURE_QUERY_FEATURE_LEVELS {}
impl ::core::fmt::Debug for DML_FEATURE_QUERY_FEATURE_LEVELS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_FEATURE_QUERY_FEATURE_LEVELS").field("RequestedFeatureLevelCount", &self.RequestedFeatureLevelCount).field("RequestedFeatureLevels", &self.RequestedFeatureLevels).finish()
    }
}
impl ::core::default::Default for DML_FEATURE_QUERY_TENSOR_DATA_TYPE_SUPPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_FEATURE_QUERY_TENSOR_DATA_TYPE_SUPPORT {
    fn eq(&self, other: &Self) -> bool {
        self.DataType == other.DataType
    }
}
impl ::core::cmp::Eq for DML_FEATURE_QUERY_TENSOR_DATA_TYPE_SUPPORT {}
impl ::core::fmt::Debug for DML_FEATURE_QUERY_TENSOR_DATA_TYPE_SUPPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_FEATURE_QUERY_TENSOR_DATA_TYPE_SUPPORT").field("DataType", &self.DataType).finish()
    }
}
impl ::core::default::Default for DML_FILL_VALUE_CONSTANT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DML_FILL_VALUE_SEQUENCE_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DML_GATHER_ELEMENTS_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_GATHER_ELEMENTS_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.IndicesTensor == other.IndicesTensor && self.OutputTensor == other.OutputTensor && self.Axis == other.Axis
    }
}
impl ::core::cmp::Eq for DML_GATHER_ELEMENTS_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_GATHER_ELEMENTS_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_GATHER_ELEMENTS_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("IndicesTensor", &self.IndicesTensor).field("OutputTensor", &self.OutputTensor).field("Axis", &self.Axis).finish()
    }
}
impl ::core::default::Default for DML_GATHER_ND1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_GATHER_ND1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.IndicesTensor == other.IndicesTensor && self.OutputTensor == other.OutputTensor && self.InputDimensionCount == other.InputDimensionCount && self.IndicesDimensionCount == other.IndicesDimensionCount && self.BatchDimensionCount == other.BatchDimensionCount
    }
}
impl ::core::cmp::Eq for DML_GATHER_ND1_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_GATHER_ND1_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_GATHER_ND1_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("IndicesTensor", &self.IndicesTensor).field("OutputTensor", &self.OutputTensor).field("InputDimensionCount", &self.InputDimensionCount).field("IndicesDimensionCount", &self.IndicesDimensionCount).field("BatchDimensionCount", &self.BatchDimensionCount).finish()
    }
}
impl ::core::default::Default for DML_GATHER_ND_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_GATHER_ND_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.IndicesTensor == other.IndicesTensor && self.OutputTensor == other.OutputTensor && self.InputDimensionCount == other.InputDimensionCount && self.IndicesDimensionCount == other.IndicesDimensionCount
    }
}
impl ::core::cmp::Eq for DML_GATHER_ND_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_GATHER_ND_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_GATHER_ND_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("IndicesTensor", &self.IndicesTensor).field("OutputTensor", &self.OutputTensor).field("InputDimensionCount", &self.InputDimensionCount).field("IndicesDimensionCount", &self.IndicesDimensionCount).finish()
    }
}
impl ::core::default::Default for DML_GATHER_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_GATHER_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.IndicesTensor == other.IndicesTensor && self.OutputTensor == other.OutputTensor && self.Axis == other.Axis && self.IndexDimensions == other.IndexDimensions
    }
}
impl ::core::cmp::Eq for DML_GATHER_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_GATHER_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_GATHER_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("IndicesTensor", &self.IndicesTensor).field("OutputTensor", &self.OutputTensor).field("Axis", &self.Axis).field("IndexDimensions", &self.IndexDimensions).finish()
    }
}
impl ::core::default::Default for DML_GEMM_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_GEMM_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.CTensor == other.CTensor && self.OutputTensor == other.OutputTensor && self.TransA == other.TransA && self.TransB == other.TransB && self.Alpha == other.Alpha && self.Beta == other.Beta && self.FusedActivation == other.FusedActivation
    }
}
impl ::core::cmp::Eq for DML_GEMM_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_GEMM_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_GEMM_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("CTensor", &self.CTensor).field("OutputTensor", &self.OutputTensor).field("TransA", &self.TransA).field("TransB", &self.TransB).field("Alpha", &self.Alpha).field("Beta", &self.Beta).field("FusedActivation", &self.FusedActivation).finish()
    }
}
impl ::core::default::Default for DML_GRAPH_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_GRAPH_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputCount == other.InputCount && self.OutputCount == other.OutputCount && self.NodeCount == other.NodeCount && self.Nodes == other.Nodes && self.InputEdgeCount == other.InputEdgeCount && self.InputEdges == other.InputEdges && self.OutputEdgeCount == other.OutputEdgeCount && self.OutputEdges == other.OutputEdges && self.IntermediateEdgeCount == other.IntermediateEdgeCount && self.IntermediateEdges == other.IntermediateEdges
    }
}
impl ::core::cmp::Eq for DML_GRAPH_DESC {}
impl ::core::fmt::Debug for DML_GRAPH_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_GRAPH_DESC").field("InputCount", &self.InputCount).field("OutputCount", &self.OutputCount).field("NodeCount", &self.NodeCount).field("Nodes", &self.Nodes).field("InputEdgeCount", &self.InputEdgeCount).field("InputEdges", &self.InputEdges).field("OutputEdgeCount", &self.OutputEdgeCount).field("OutputEdges", &self.OutputEdges).field("IntermediateEdgeCount", &self.IntermediateEdgeCount).field("IntermediateEdges", &self.IntermediateEdges).finish()
    }
}
impl ::core::default::Default for DML_GRAPH_EDGE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_GRAPH_EDGE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Desc == other.Desc
    }
}
impl ::core::cmp::Eq for DML_GRAPH_EDGE_DESC {}
impl ::core::fmt::Debug for DML_GRAPH_EDGE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_GRAPH_EDGE_DESC").field("Type", &self.Type).field("Desc", &self.Desc).finish()
    }
}
impl ::core::default::Default for DML_GRAPH_EDGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DML_GRAPH_EDGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_GRAPH_EDGE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DML_GRAPH_NODE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_GRAPH_NODE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Desc == other.Desc
    }
}
impl ::core::cmp::Eq for DML_GRAPH_NODE_DESC {}
impl ::core::fmt::Debug for DML_GRAPH_NODE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_GRAPH_NODE_DESC").field("Type", &self.Type).field("Desc", &self.Desc).finish()
    }
}
impl ::core::default::Default for DML_GRAPH_NODE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DML_GRAPH_NODE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_GRAPH_NODE_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_GRU_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_GRU_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.WeightTensor == other.WeightTensor && self.RecurrenceTensor == other.RecurrenceTensor && self.BiasTensor == other.BiasTensor && self.HiddenInitTensor == other.HiddenInitTensor && self.SequenceLengthsTensor == other.SequenceLengthsTensor && self.OutputSequenceTensor == other.OutputSequenceTensor && self.OutputSingleTensor == other.OutputSingleTensor && self.ActivationDescCount == other.ActivationDescCount && self.ActivationDescs == other.ActivationDescs && self.Direction == other.Direction && self.LinearBeforeReset == other.LinearBeforeReset
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_GRU_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DML_GRU_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_GRU_OPERATOR_DESC")
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
impl ::core::default::Default for DML_INPUT_GRAPH_EDGE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_INPUT_GRAPH_EDGE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.GraphInputIndex == other.GraphInputIndex && self.ToNodeIndex == other.ToNodeIndex && self.ToNodeInputIndex == other.ToNodeInputIndex && self.Name == other.Name
    }
}
impl ::core::cmp::Eq for DML_INPUT_GRAPH_EDGE_DESC {}
impl ::core::fmt::Debug for DML_INPUT_GRAPH_EDGE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_INPUT_GRAPH_EDGE_DESC").field("GraphInputIndex", &self.GraphInputIndex).field("ToNodeIndex", &self.ToNodeIndex).field("ToNodeInputIndex", &self.ToNodeInputIndex).field("Name", &self.Name).finish()
    }
}
impl ::core::default::Default for DML_INTERMEDIATE_GRAPH_EDGE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_INTERMEDIATE_GRAPH_EDGE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.FromNodeIndex == other.FromNodeIndex && self.FromNodeOutputIndex == other.FromNodeOutputIndex && self.ToNodeIndex == other.ToNodeIndex && self.ToNodeInputIndex == other.ToNodeInputIndex && self.Name == other.Name
    }
}
impl ::core::cmp::Eq for DML_INTERMEDIATE_GRAPH_EDGE_DESC {}
impl ::core::fmt::Debug for DML_INTERMEDIATE_GRAPH_EDGE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_INTERMEDIATE_GRAPH_EDGE_DESC").field("FromNodeIndex", &self.FromNodeIndex).field("FromNodeOutputIndex", &self.FromNodeOutputIndex).field("ToNodeIndex", &self.ToNodeIndex).field("ToNodeInputIndex", &self.ToNodeInputIndex).field("Name", &self.Name).finish()
    }
}
impl ::core::default::Default for DML_INTERPOLATION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DML_INTERPOLATION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_INTERPOLATION_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DML_IS_INFINITY_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DML_IS_INFINITY_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_IS_INFINITY_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DML_JOIN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_JOIN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputCount == other.InputCount && self.InputTensors == other.InputTensors && self.OutputTensor == other.OutputTensor && self.Axis == other.Axis
    }
}
impl ::core::cmp::Eq for DML_JOIN_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_JOIN_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_JOIN_OPERATOR_DESC").field("InputCount", &self.InputCount).field("InputTensors", &self.InputTensors).field("OutputTensor", &self.OutputTensor).field("Axis", &self.Axis).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_LOCAL_RESPONSE_NORMALIZATION_GRAD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DML_LOCAL_RESPONSE_NORMALIZATION_GRAD_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_LOCAL_RESPONSE_NORMALIZATION_GRAD_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("InputGradientTensor", &self.InputGradientTensor).field("OutputGradientTensor", &self.OutputGradientTensor).field("CrossChannel", &self.CrossChannel).field("LocalSize", &self.LocalSize).field("Alpha", &self.Alpha).field("Beta", &self.Beta).field("Bias", &self.Bias).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_LOCAL_RESPONSE_NORMALIZATION_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DML_LOCAL_RESPONSE_NORMALIZATION_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_LOCAL_RESPONSE_NORMALIZATION_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("CrossChannel", &self.CrossChannel).field("LocalSize", &self.LocalSize).field("Alpha", &self.Alpha).field("Beta", &self.Beta).field("Bias", &self.Bias).finish()
    }
}
impl ::core::default::Default for DML_LP_NORMALIZATION_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_LP_NORMALIZATION_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Axis == other.Axis && self.Epsilon == other.Epsilon && self.P == other.P
    }
}
impl ::core::cmp::Eq for DML_LP_NORMALIZATION_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_LP_NORMALIZATION_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_LP_NORMALIZATION_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Axis", &self.Axis).field("Epsilon", &self.Epsilon).field("P", &self.P).finish()
    }
}
impl ::core::default::Default for DML_LP_POOLING_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_LP_POOLING_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.DimensionCount == other.DimensionCount && self.Strides == other.Strides && self.WindowSize == other.WindowSize && self.StartPadding == other.StartPadding && self.EndPadding == other.EndPadding && self.P == other.P
    }
}
impl ::core::cmp::Eq for DML_LP_POOLING_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_LP_POOLING_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_LP_POOLING_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("DimensionCount", &self.DimensionCount).field("Strides", &self.Strides).field("WindowSize", &self.WindowSize).field("StartPadding", &self.StartPadding).field("EndPadding", &self.EndPadding).field("P", &self.P).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_LSTM_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DML_LSTM_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_LSTM_OPERATOR_DESC")
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
impl ::core::default::Default for DML_MATRIX_MULTIPLY_INTEGER_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_MATRIX_MULTIPLY_INTEGER_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.AZeroPointTensor == other.AZeroPointTensor && self.BTensor == other.BTensor && self.BZeroPointTensor == other.BZeroPointTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_MATRIX_MULTIPLY_INTEGER_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_MATRIX_MULTIPLY_INTEGER_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_MATRIX_MULTIPLY_INTEGER_OPERATOR_DESC").field("ATensor", &self.ATensor).field("AZeroPointTensor", &self.AZeroPointTensor).field("BTensor", &self.BTensor).field("BZeroPointTensor", &self.BZeroPointTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_MATRIX_TRANSFORM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DML_MATRIX_TRANSFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_MATRIX_TRANSFORM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DML_MAX_POOLING1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_MAX_POOLING1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.OutputIndicesTensor == other.OutputIndicesTensor && self.DimensionCount == other.DimensionCount && self.Strides == other.Strides && self.WindowSize == other.WindowSize && self.StartPadding == other.StartPadding && self.EndPadding == other.EndPadding
    }
}
impl ::core::cmp::Eq for DML_MAX_POOLING1_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_MAX_POOLING1_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_MAX_POOLING1_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("OutputIndicesTensor", &self.OutputIndicesTensor).field("DimensionCount", &self.DimensionCount).field("Strides", &self.Strides).field("WindowSize", &self.WindowSize).field("StartPadding", &self.StartPadding).field("EndPadding", &self.EndPadding).finish()
    }
}
impl ::core::default::Default for DML_MAX_POOLING2_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_MAX_POOLING2_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.OutputIndicesTensor == other.OutputIndicesTensor && self.DimensionCount == other.DimensionCount && self.Strides == other.Strides && self.WindowSize == other.WindowSize && self.StartPadding == other.StartPadding && self.EndPadding == other.EndPadding && self.Dilations == other.Dilations
    }
}
impl ::core::cmp::Eq for DML_MAX_POOLING2_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_MAX_POOLING2_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_MAX_POOLING2_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("OutputIndicesTensor", &self.OutputIndicesTensor).field("DimensionCount", &self.DimensionCount).field("Strides", &self.Strides).field("WindowSize", &self.WindowSize).field("StartPadding", &self.StartPadding).field("EndPadding", &self.EndPadding).field("Dilations", &self.Dilations).finish()
    }
}
impl ::core::default::Default for DML_MAX_POOLING_GRAD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_MAX_POOLING_GRAD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.InputGradientTensor == other.InputGradientTensor && self.OutputGradientTensor == other.OutputGradientTensor && self.DimensionCount == other.DimensionCount && self.Strides == other.Strides && self.WindowSize == other.WindowSize && self.StartPadding == other.StartPadding && self.EndPadding == other.EndPadding && self.Dilations == other.Dilations
    }
}
impl ::core::cmp::Eq for DML_MAX_POOLING_GRAD_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_MAX_POOLING_GRAD_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_MAX_POOLING_GRAD_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("InputGradientTensor", &self.InputGradientTensor).field("OutputGradientTensor", &self.OutputGradientTensor).field("DimensionCount", &self.DimensionCount).field("Strides", &self.Strides).field("WindowSize", &self.WindowSize).field("StartPadding", &self.StartPadding).field("EndPadding", &self.EndPadding).field("Dilations", &self.Dilations).finish()
    }
}
impl ::core::default::Default for DML_MAX_POOLING_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_MAX_POOLING_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.DimensionCount == other.DimensionCount && self.Strides == other.Strides && self.WindowSize == other.WindowSize && self.StartPadding == other.StartPadding && self.EndPadding == other.EndPadding
    }
}
impl ::core::cmp::Eq for DML_MAX_POOLING_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_MAX_POOLING_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_MAX_POOLING_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("DimensionCount", &self.DimensionCount).field("Strides", &self.Strides).field("WindowSize", &self.WindowSize).field("StartPadding", &self.StartPadding).field("EndPadding", &self.EndPadding).finish()
    }
}
impl ::core::default::Default for DML_MAX_UNPOOLING_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_MAX_UNPOOLING_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.IndicesTensor == other.IndicesTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_MAX_UNPOOLING_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_MAX_UNPOOLING_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_MAX_UNPOOLING_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("IndicesTensor", &self.IndicesTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_MEAN_VARIANCE_NORMALIZATION1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DML_MEAN_VARIANCE_NORMALIZATION1_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_MEAN_VARIANCE_NORMALIZATION1_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("ScaleTensor", &self.ScaleTensor).field("BiasTensor", &self.BiasTensor).field("OutputTensor", &self.OutputTensor).field("AxisCount", &self.AxisCount).field("Axes", &self.Axes).field("NormalizeVariance", &self.NormalizeVariance).field("Epsilon", &self.Epsilon).field("FusedActivation", &self.FusedActivation).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_MEAN_VARIANCE_NORMALIZATION_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DML_MEAN_VARIANCE_NORMALIZATION_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_MEAN_VARIANCE_NORMALIZATION_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("ScaleTensor", &self.ScaleTensor).field("BiasTensor", &self.BiasTensor).field("OutputTensor", &self.OutputTensor).field("CrossChannel", &self.CrossChannel).field("NormalizeVariance", &self.NormalizeVariance).field("Epsilon", &self.Epsilon).field("FusedActivation", &self.FusedActivation).finish()
    }
}
impl ::core::default::Default for DML_NONZERO_COORDINATES_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_NONZERO_COORDINATES_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputCountTensor == other.OutputCountTensor && self.OutputCoordinatesTensor == other.OutputCoordinatesTensor
    }
}
impl ::core::cmp::Eq for DML_NONZERO_COORDINATES_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_NONZERO_COORDINATES_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_NONZERO_COORDINATES_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputCountTensor", &self.OutputCountTensor).field("OutputCoordinatesTensor", &self.OutputCoordinatesTensor).finish()
    }
}
impl ::core::default::Default for DML_ONE_HOT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ONE_HOT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.IndicesTensor == other.IndicesTensor && self.ValuesTensor == other.ValuesTensor && self.OutputTensor == other.OutputTensor && self.Axis == other.Axis
    }
}
impl ::core::cmp::Eq for DML_ONE_HOT_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ONE_HOT_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ONE_HOT_OPERATOR_DESC").field("IndicesTensor", &self.IndicesTensor).field("ValuesTensor", &self.ValuesTensor).field("OutputTensor", &self.OutputTensor).field("Axis", &self.Axis).finish()
    }
}
impl ::core::default::Default for DML_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Desc == other.Desc
    }
}
impl ::core::cmp::Eq for DML_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_OPERATOR_DESC").field("Type", &self.Type).field("Desc", &self.Desc).finish()
    }
}
impl ::core::default::Default for DML_OPERATOR_GRAPH_NODE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_OPERATOR_GRAPH_NODE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Operator == other.Operator && self.Name == other.Name
    }
}
impl ::core::cmp::Eq for DML_OPERATOR_GRAPH_NODE_DESC {}
impl ::core::fmt::Debug for DML_OPERATOR_GRAPH_NODE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_OPERATOR_GRAPH_NODE_DESC").field("Operator", &self.Operator).field("Name", &self.Name).finish()
    }
}
impl ::core::default::Default for DML_OPERATOR_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DML_OPERATOR_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_OPERATOR_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DML_OUTPUT_GRAPH_EDGE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_OUTPUT_GRAPH_EDGE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.FromNodeIndex == other.FromNodeIndex && self.FromNodeOutputIndex == other.FromNodeOutputIndex && self.GraphOutputIndex == other.GraphOutputIndex && self.Name == other.Name
    }
}
impl ::core::cmp::Eq for DML_OUTPUT_GRAPH_EDGE_DESC {}
impl ::core::fmt::Debug for DML_OUTPUT_GRAPH_EDGE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_OUTPUT_GRAPH_EDGE_DESC").field("FromNodeIndex", &self.FromNodeIndex).field("FromNodeOutputIndex", &self.FromNodeOutputIndex).field("GraphOutputIndex", &self.GraphOutputIndex).field("Name", &self.Name).finish()
    }
}
impl ::core::default::Default for DML_PADDING_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DML_PADDING_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_PADDING_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DML_PADDING_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_PADDING_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.PaddingMode == other.PaddingMode && self.PaddingValue == other.PaddingValue && self.DimensionCount == other.DimensionCount && self.StartPadding == other.StartPadding && self.EndPadding == other.EndPadding
    }
}
impl ::core::cmp::Eq for DML_PADDING_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_PADDING_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_PADDING_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("PaddingMode", &self.PaddingMode).field("PaddingValue", &self.PaddingValue).field("DimensionCount", &self.DimensionCount).field("StartPadding", &self.StartPadding).field("EndPadding", &self.EndPadding).finish()
    }
}
impl ::core::default::Default for DML_QUANTIZED_LINEAR_CONVOLUTION_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_QUANTIZED_LINEAR_CONVOLUTION_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.InputScaleTensor == other.InputScaleTensor && self.InputZeroPointTensor == other.InputZeroPointTensor && self.FilterTensor == other.FilterTensor && self.FilterScaleTensor == other.FilterScaleTensor && self.FilterZeroPointTensor == other.FilterZeroPointTensor && self.BiasTensor == other.BiasTensor && self.OutputScaleTensor == other.OutputScaleTensor && self.OutputZeroPointTensor == other.OutputZeroPointTensor && self.OutputTensor == other.OutputTensor && self.DimensionCount == other.DimensionCount && self.Strides == other.Strides && self.Dilations == other.Dilations && self.StartPadding == other.StartPadding && self.EndPadding == other.EndPadding && self.GroupCount == other.GroupCount
    }
}
impl ::core::cmp::Eq for DML_QUANTIZED_LINEAR_CONVOLUTION_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_QUANTIZED_LINEAR_CONVOLUTION_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_QUANTIZED_LINEAR_CONVOLUTION_OPERATOR_DESC")
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
impl ::core::default::Default for DML_QUANTIZED_LINEAR_MATRIX_MULTIPLY_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_QUANTIZED_LINEAR_MATRIX_MULTIPLY_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.AScaleTensor == other.AScaleTensor && self.AZeroPointTensor == other.AZeroPointTensor && self.BTensor == other.BTensor && self.BScaleTensor == other.BScaleTensor && self.BZeroPointTensor == other.BZeroPointTensor && self.OutputScaleTensor == other.OutputScaleTensor && self.OutputZeroPointTensor == other.OutputZeroPointTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_QUANTIZED_LINEAR_MATRIX_MULTIPLY_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_QUANTIZED_LINEAR_MATRIX_MULTIPLY_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_QUANTIZED_LINEAR_MATRIX_MULTIPLY_OPERATOR_DESC").field("ATensor", &self.ATensor).field("AScaleTensor", &self.AScaleTensor).field("AZeroPointTensor", &self.AZeroPointTensor).field("BTensor", &self.BTensor).field("BScaleTensor", &self.BScaleTensor).field("BZeroPointTensor", &self.BZeroPointTensor).field("OutputScaleTensor", &self.OutputScaleTensor).field("OutputZeroPointTensor", &self.OutputZeroPointTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::core::default::Default for DML_RANDOM_GENERATOR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_RANDOM_GENERATOR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputStateTensor == other.InputStateTensor && self.OutputTensor == other.OutputTensor && self.OutputStateTensor == other.OutputStateTensor && self.Type == other.Type
    }
}
impl ::core::cmp::Eq for DML_RANDOM_GENERATOR_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_RANDOM_GENERATOR_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_RANDOM_GENERATOR_OPERATOR_DESC").field("InputStateTensor", &self.InputStateTensor).field("OutputTensor", &self.OutputTensor).field("OutputStateTensor", &self.OutputStateTensor).field("Type", &self.Type).finish()
    }
}
impl ::core::default::Default for DML_RANDOM_GENERATOR_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DML_RANDOM_GENERATOR_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_RANDOM_GENERATOR_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DML_RECURRENT_NETWORK_DIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DML_RECURRENT_NETWORK_DIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_RECURRENT_NETWORK_DIRECTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for DML_REDUCE_FUNCTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DML_REDUCE_FUNCTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_REDUCE_FUNCTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for DML_REDUCE_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_REDUCE_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Function == other.Function && self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.AxisCount == other.AxisCount && self.Axes == other.Axes
    }
}
impl ::core::cmp::Eq for DML_REDUCE_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_REDUCE_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_REDUCE_OPERATOR_DESC").field("Function", &self.Function).field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("AxisCount", &self.AxisCount).field("Axes", &self.Axes).finish()
    }
}
impl ::core::default::Default for DML_RESAMPLE1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_RESAMPLE1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.InterpolationMode == other.InterpolationMode && self.DimensionCount == other.DimensionCount && self.Scales == other.Scales && self.InputPixelOffsets == other.InputPixelOffsets && self.OutputPixelOffsets == other.OutputPixelOffsets
    }
}
impl ::core::cmp::Eq for DML_RESAMPLE1_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_RESAMPLE1_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_RESAMPLE1_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("InterpolationMode", &self.InterpolationMode).field("DimensionCount", &self.DimensionCount).field("Scales", &self.Scales).field("InputPixelOffsets", &self.InputPixelOffsets).field("OutputPixelOffsets", &self.OutputPixelOffsets).finish()
    }
}
impl ::core::default::Default for DML_RESAMPLE_GRAD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_RESAMPLE_GRAD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputGradientTensor == other.InputGradientTensor && self.OutputGradientTensor == other.OutputGradientTensor && self.InterpolationMode == other.InterpolationMode && self.DimensionCount == other.DimensionCount && self.Scales == other.Scales && self.InputPixelOffsets == other.InputPixelOffsets && self.OutputPixelOffsets == other.OutputPixelOffsets
    }
}
impl ::core::cmp::Eq for DML_RESAMPLE_GRAD_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_RESAMPLE_GRAD_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_RESAMPLE_GRAD_OPERATOR_DESC").field("InputGradientTensor", &self.InputGradientTensor).field("OutputGradientTensor", &self.OutputGradientTensor).field("InterpolationMode", &self.InterpolationMode).field("DimensionCount", &self.DimensionCount).field("Scales", &self.Scales).field("InputPixelOffsets", &self.InputPixelOffsets).field("OutputPixelOffsets", &self.OutputPixelOffsets).finish()
    }
}
impl ::core::default::Default for DML_RESAMPLE_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_RESAMPLE_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.InterpolationMode == other.InterpolationMode && self.ScaleCount == other.ScaleCount && self.Scales == other.Scales
    }
}
impl ::core::cmp::Eq for DML_RESAMPLE_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_RESAMPLE_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_RESAMPLE_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("InterpolationMode", &self.InterpolationMode).field("ScaleCount", &self.ScaleCount).field("Scales", &self.Scales).finish()
    }
}
impl ::core::default::Default for DML_REVERSE_SUBSEQUENCES_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_REVERSE_SUBSEQUENCES_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.SequenceLengthsTensor == other.SequenceLengthsTensor && self.OutputTensor == other.OutputTensor && self.Axis == other.Axis
    }
}
impl ::core::cmp::Eq for DML_REVERSE_SUBSEQUENCES_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_REVERSE_SUBSEQUENCES_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_REVERSE_SUBSEQUENCES_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("SequenceLengthsTensor", &self.SequenceLengthsTensor).field("OutputTensor", &self.OutputTensor).field("Axis", &self.Axis).finish()
    }
}
impl ::core::default::Default for DML_RNN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_RNN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.WeightTensor == other.WeightTensor && self.RecurrenceTensor == other.RecurrenceTensor && self.BiasTensor == other.BiasTensor && self.HiddenInitTensor == other.HiddenInitTensor && self.SequenceLengthsTensor == other.SequenceLengthsTensor && self.OutputSequenceTensor == other.OutputSequenceTensor && self.OutputSingleTensor == other.OutputSingleTensor && self.ActivationDescCount == other.ActivationDescCount && self.ActivationDescs == other.ActivationDescs && self.Direction == other.Direction
    }
}
impl ::core::cmp::Eq for DML_RNN_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_RNN_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_RNN_OPERATOR_DESC")
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_ROI_ALIGN1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_ROI_ALIGN1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.ROITensor == other.ROITensor && self.BatchIndicesTensor == other.BatchIndicesTensor && self.OutputTensor == other.OutputTensor && self.ReductionFunction == other.ReductionFunction && self.InterpolationMode == other.InterpolationMode && self.SpatialScaleX == other.SpatialScaleX && self.SpatialScaleY == other.SpatialScaleY && self.InputPixelOffset == other.InputPixelOffset && self.OutputPixelOffset == other.OutputPixelOffset && self.OutOfBoundsInputValue == other.OutOfBoundsInputValue && self.MinimumSamplesPerOutput == other.MinimumSamplesPerOutput && self.MaximumSamplesPerOutput == other.MaximumSamplesPerOutput && self.AlignRegionsToCorners == other.AlignRegionsToCorners
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_ROI_ALIGN1_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DML_ROI_ALIGN1_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ROI_ALIGN1_OPERATOR_DESC")
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
impl ::core::default::Default for DML_ROI_ALIGN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ROI_ALIGN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.ROITensor == other.ROITensor && self.BatchIndicesTensor == other.BatchIndicesTensor && self.OutputTensor == other.OutputTensor && self.ReductionFunction == other.ReductionFunction && self.InterpolationMode == other.InterpolationMode && self.SpatialScaleX == other.SpatialScaleX && self.SpatialScaleY == other.SpatialScaleY && self.OutOfBoundsInputValue == other.OutOfBoundsInputValue && self.MinimumSamplesPerOutput == other.MinimumSamplesPerOutput && self.MaximumSamplesPerOutput == other.MaximumSamplesPerOutput
    }
}
impl ::core::cmp::Eq for DML_ROI_ALIGN_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ROI_ALIGN_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ROI_ALIGN_OPERATOR_DESC")
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
impl ::core::default::Default for DML_ROI_POOLING_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_ROI_POOLING_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.ROITensor == other.ROITensor && self.OutputTensor == other.OutputTensor && self.SpatialScale == other.SpatialScale && self.PooledSize == other.PooledSize
    }
}
impl ::core::cmp::Eq for DML_ROI_POOLING_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_ROI_POOLING_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ROI_POOLING_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("ROITensor", &self.ROITensor).field("OutputTensor", &self.OutputTensor).field("SpatialScale", &self.SpatialScale).field("PooledSize", &self.PooledSize).finish()
    }
}
impl ::core::default::Default for DML_ROUNDING_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DML_ROUNDING_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_ROUNDING_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DML_SCALAR_UNION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DML_SCALE_BIAS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_SCALE_BIAS {
    fn eq(&self, other: &Self) -> bool {
        self.Scale == other.Scale && self.Bias == other.Bias
    }
}
impl ::core::cmp::Eq for DML_SCALE_BIAS {}
impl ::core::fmt::Debug for DML_SCALE_BIAS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_SCALE_BIAS").field("Scale", &self.Scale).field("Bias", &self.Bias).finish()
    }
}
impl ::core::default::Default for DML_SCATTER_ND_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_SCATTER_ND_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.IndicesTensor == other.IndicesTensor && self.UpdatesTensor == other.UpdatesTensor && self.OutputTensor == other.OutputTensor && self.InputDimensionCount == other.InputDimensionCount && self.IndicesDimensionCount == other.IndicesDimensionCount
    }
}
impl ::core::cmp::Eq for DML_SCATTER_ND_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_SCATTER_ND_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_SCATTER_ND_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("IndicesTensor", &self.IndicesTensor).field("UpdatesTensor", &self.UpdatesTensor).field("OutputTensor", &self.OutputTensor).field("InputDimensionCount", &self.InputDimensionCount).field("IndicesDimensionCount", &self.IndicesDimensionCount).finish()
    }
}
impl ::core::default::Default for DML_SCATTER_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_SCATTER_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.IndicesTensor == other.IndicesTensor && self.UpdatesTensor == other.UpdatesTensor && self.OutputTensor == other.OutputTensor && self.Axis == other.Axis
    }
}
impl ::core::cmp::Eq for DML_SCATTER_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_SCATTER_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_SCATTER_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("IndicesTensor", &self.IndicesTensor).field("UpdatesTensor", &self.UpdatesTensor).field("OutputTensor", &self.OutputTensor).field("Axis", &self.Axis).finish()
    }
}
impl ::core::default::Default for DML_SIZE_2D {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_SIZE_2D {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height
    }
}
impl ::core::cmp::Eq for DML_SIZE_2D {}
impl ::core::fmt::Debug for DML_SIZE_2D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_SIZE_2D").field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
impl ::core::default::Default for DML_SLICE1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_SLICE1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.DimensionCount == other.DimensionCount && self.InputWindowOffsets == other.InputWindowOffsets && self.InputWindowSizes == other.InputWindowSizes && self.InputWindowStrides == other.InputWindowStrides
    }
}
impl ::core::cmp::Eq for DML_SLICE1_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_SLICE1_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_SLICE1_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("DimensionCount", &self.DimensionCount).field("InputWindowOffsets", &self.InputWindowOffsets).field("InputWindowSizes", &self.InputWindowSizes).field("InputWindowStrides", &self.InputWindowStrides).finish()
    }
}
impl ::core::default::Default for DML_SLICE_GRAD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_SLICE_GRAD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputGradientTensor == other.InputGradientTensor && self.OutputGradientTensor == other.OutputGradientTensor && self.DimensionCount == other.DimensionCount && self.InputWindowOffsets == other.InputWindowOffsets && self.InputWindowSizes == other.InputWindowSizes && self.InputWindowStrides == other.InputWindowStrides
    }
}
impl ::core::cmp::Eq for DML_SLICE_GRAD_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_SLICE_GRAD_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_SLICE_GRAD_OPERATOR_DESC").field("InputGradientTensor", &self.InputGradientTensor).field("OutputGradientTensor", &self.OutputGradientTensor).field("DimensionCount", &self.DimensionCount).field("InputWindowOffsets", &self.InputWindowOffsets).field("InputWindowSizes", &self.InputWindowSizes).field("InputWindowStrides", &self.InputWindowStrides).finish()
    }
}
impl ::core::default::Default for DML_SLICE_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_SLICE_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.DimensionCount == other.DimensionCount && self.Offsets == other.Offsets && self.Sizes == other.Sizes && self.Strides == other.Strides
    }
}
impl ::core::cmp::Eq for DML_SLICE_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_SLICE_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_SLICE_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("DimensionCount", &self.DimensionCount).field("Offsets", &self.Offsets).field("Sizes", &self.Sizes).field("Strides", &self.Strides).finish()
    }
}
impl ::core::default::Default for DML_SPACE_TO_DEPTH1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_SPACE_TO_DEPTH1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.BlockSize == other.BlockSize && self.Order == other.Order
    }
}
impl ::core::cmp::Eq for DML_SPACE_TO_DEPTH1_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_SPACE_TO_DEPTH1_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_SPACE_TO_DEPTH1_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("BlockSize", &self.BlockSize).field("Order", &self.Order).finish()
    }
}
impl ::core::default::Default for DML_SPACE_TO_DEPTH_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_SPACE_TO_DEPTH_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.BlockSize == other.BlockSize
    }
}
impl ::core::cmp::Eq for DML_SPACE_TO_DEPTH_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_SPACE_TO_DEPTH_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_SPACE_TO_DEPTH_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("BlockSize", &self.BlockSize).finish()
    }
}
impl ::core::default::Default for DML_SPLIT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_SPLIT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputCount == other.OutputCount && self.OutputTensors == other.OutputTensors && self.Axis == other.Axis
    }
}
impl ::core::cmp::Eq for DML_SPLIT_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_SPLIT_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_SPLIT_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputCount", &self.OutputCount).field("OutputTensors", &self.OutputTensors).field("Axis", &self.Axis).finish()
    }
}
impl ::core::default::Default for DML_TENSOR_DATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DML_TENSOR_DATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_TENSOR_DATA_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DML_TENSOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_TENSOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Desc == other.Desc
    }
}
impl ::core::cmp::Eq for DML_TENSOR_DESC {}
impl ::core::fmt::Debug for DML_TENSOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_TENSOR_DESC").field("Type", &self.Type).field("Desc", &self.Desc).finish()
    }
}
impl ::core::default::Default for DML_TENSOR_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DML_TENSOR_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_TENSOR_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DML_TENSOR_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DML_TENSOR_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DML_TENSOR_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DML_TENSOR_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DML_TENSOR_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for DML_TENSOR_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DML_TENSOR_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_TENSOR_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DML_TILE_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_TILE_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.RepeatsCount == other.RepeatsCount && self.Repeats == other.Repeats
    }
}
impl ::core::cmp::Eq for DML_TILE_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_TILE_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_TILE_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("RepeatsCount", &self.RepeatsCount).field("Repeats", &self.Repeats).finish()
    }
}
impl ::core::default::Default for DML_TOP_K1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_TOP_K1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputValueTensor == other.OutputValueTensor && self.OutputIndexTensor == other.OutputIndexTensor && self.Axis == other.Axis && self.K == other.K && self.AxisDirection == other.AxisDirection
    }
}
impl ::core::cmp::Eq for DML_TOP_K1_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_TOP_K1_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_TOP_K1_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputValueTensor", &self.OutputValueTensor).field("OutputIndexTensor", &self.OutputIndexTensor).field("Axis", &self.Axis).field("K", &self.K).field("AxisDirection", &self.AxisDirection).finish()
    }
}
impl ::core::default::Default for DML_TOP_K_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_TOP_K_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputValueTensor == other.OutputValueTensor && self.OutputIndexTensor == other.OutputIndexTensor && self.Axis == other.Axis && self.K == other.K
    }
}
impl ::core::cmp::Eq for DML_TOP_K_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_TOP_K_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_TOP_K_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputValueTensor", &self.OutputValueTensor).field("OutputIndexTensor", &self.OutputIndexTensor).field("Axis", &self.Axis).field("K", &self.K).finish()
    }
}
impl ::core::default::Default for DML_UPSAMPLE_2D_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_UPSAMPLE_2D_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleSize == other.ScaleSize && self.InterpolationMode == other.InterpolationMode
    }
}
impl ::core::cmp::Eq for DML_UPSAMPLE_2D_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_UPSAMPLE_2D_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_UPSAMPLE_2D_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleSize", &self.ScaleSize).field("InterpolationMode", &self.InterpolationMode).finish()
    }
}
impl ::core::default::Default for DML_VALUE_SCALE_2D_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DML_VALUE_SCALE_2D_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Scale == other.Scale && self.ChannelCount == other.ChannelCount && self.Bias == other.Bias
    }
}
impl ::core::cmp::Eq for DML_VALUE_SCALE_2D_OPERATOR_DESC {}
impl ::core::fmt::Debug for DML_VALUE_SCALE_2D_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_VALUE_SCALE_2D_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Scale", &self.Scale).field("ChannelCount", &self.ChannelCount).field("Bias", &self.Bias).finish()
    }
}
impl ::core::cmp::PartialEq for IDMLBindingTable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMLBindingTable {}
impl ::core::fmt::Debug for IDMLBindingTable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDMLBindingTable").field(&self.0).finish()
    }
}
impl IDMLBindingTable {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: *mut u32, data: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, data: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, data.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDMLCommandRecorder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMLCommandRecorder {}
impl ::core::fmt::Debug for IDMLCommandRecorder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDMLCommandRecorder").field(&self.0).finish()
    }
}
impl IDMLCommandRecorder {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: *mut u32, data: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, data: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, data.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDMLCompiledOperator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMLCompiledOperator {}
impl ::core::fmt::Debug for IDMLCompiledOperator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDMLCompiledOperator").field(&self.0).finish()
    }
}
impl IDMLCompiledOperator {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: *mut u32, data: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, data: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, data.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetBindingProperties(&self) -> DML_BINDING_PROPERTIES {
        let mut result__: DML_BINDING_PROPERTIES = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBindingProperties)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
}
impl ::core::cmp::PartialEq for IDMLDebugDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMLDebugDevice {}
impl ::core::fmt::Debug for IDMLDebugDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDMLDebugDevice").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDMLDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMLDevice {}
impl ::core::fmt::Debug for IDMLDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDMLDevice").field(&self.0).finish()
    }
}
impl IDMLDevice {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: *mut u32, data: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, data: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, data.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IDMLDevice1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMLDevice1 {}
impl ::core::fmt::Debug for IDMLDevice1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDMLDevice1").field(&self.0).finish()
    }
}
impl IDMLDevice1 {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: *mut u32, data: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, data: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, data.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn CheckFeatureSupport(&self, feature: DML_FEATURE, featurequerydatasize: u32, featurequerydata: ::core::option::Option<*const ::core::ffi::c_void>, featuresupportdatasize: u32, featuresupportdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CheckFeatureSupport)(::windows::core::Vtable::as_raw(self), feature, featurequerydatasize, ::core::mem::transmute(featurequerydata.unwrap_or(::std::ptr::null())), featuresupportdatasize, featuresupportdata).ok()
    }
    pub unsafe fn CreateOperator<T>(&self, desc: *const DML_OPERATOR_DESC, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateOperator)(::windows::core::Vtable::as_raw(self), desc, &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn CompileOperator<P0, T>(&self, op: P0, flags: DML_EXECUTION_FLAGS, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDMLOperator>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.CompileOperator)(::windows::core::Vtable::as_raw(self), op.into().abi(), flags, &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn CreateOperatorInitializer<T>(&self, operators: ::core::option::Option<&[IDMLCompiledOperator]>) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateOperatorInitializer)(::windows::core::Vtable::as_raw(self), operators.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(operators.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCommandRecorder<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateCommandRecorder)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CreateBindingTable<T>(&self, desc: ::core::option::Option<*const DML_BINDING_TABLE_DESC>) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateBindingTable)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(desc.unwrap_or(::std::ptr::null())), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Evict(&self, ppobjects: &[IDMLPageable]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Evict)(::windows::core::Vtable::as_raw(self), ppobjects.len() as _, ::core::mem::transmute(ppobjects.as_ptr())).ok()
    }
    pub unsafe fn MakeResident(&self, ppobjects: &[IDMLPageable]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.MakeResident)(::windows::core::Vtable::as_raw(self), ppobjects.len() as _, ::core::mem::transmute(ppobjects.as_ptr())).ok()
    }
    pub unsafe fn GetDeviceRemovedReason(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDeviceRemovedReason)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetParentDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetParentDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDMLDeviceChild {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMLDeviceChild {}
impl ::core::fmt::Debug for IDMLDeviceChild {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDMLDeviceChild").field(&self.0).finish()
    }
}
impl IDMLDeviceChild {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: *mut u32, data: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, data: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, data.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IDMLDispatchable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMLDispatchable {}
impl ::core::fmt::Debug for IDMLDispatchable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDMLDispatchable").field(&self.0).finish()
    }
}
impl IDMLDispatchable {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: *mut u32, data: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, data: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, data.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDMLObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMLObject {}
impl ::core::fmt::Debug for IDMLObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDMLObject").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDMLOperator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMLOperator {}
impl ::core::fmt::Debug for IDMLOperator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDMLOperator").field(&self.0).finish()
    }
}
impl IDMLOperator {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: *mut u32, data: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, data: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, data.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDMLOperatorInitializer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMLOperatorInitializer {}
impl ::core::fmt::Debug for IDMLOperatorInitializer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDMLOperatorInitializer").field(&self.0).finish()
    }
}
impl IDMLOperatorInitializer {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: *mut u32, data: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, data: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, data.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetBindingProperties(&self) -> DML_BINDING_PROPERTIES {
        let mut result__: DML_BINDING_PROPERTIES = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBindingProperties)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
}
impl ::core::cmp::PartialEq for IDMLPageable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMLPageable {}
impl ::core::fmt::Debug for IDMLPageable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDMLPageable").field(&self.0).finish()
    }
}
impl IDMLPageable {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: *mut u32, data: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, data: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, data.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
