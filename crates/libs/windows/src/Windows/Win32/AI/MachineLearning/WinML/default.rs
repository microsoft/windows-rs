impl ::core::cmp::PartialEq for IMLOperatorAttributes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLOperatorAttributes {}
impl ::core::fmt::Debug for IMLOperatorAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLOperatorAttributes").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMLOperatorKernel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLOperatorKernel {}
impl ::core::fmt::Debug for IMLOperatorKernel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLOperatorKernel").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMLOperatorKernelContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLOperatorKernelContext {}
impl ::core::fmt::Debug for IMLOperatorKernelContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLOperatorKernelContext").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMLOperatorKernelCreationContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLOperatorKernelCreationContext {}
impl ::core::fmt::Debug for IMLOperatorKernelCreationContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLOperatorKernelCreationContext").field(&self.0).finish()
    }
}
impl IMLOperatorKernelCreationContext {
    pub unsafe fn GetAttributeElementCount<P0>(&self, name: P0, r#type: MLOperatorAttributeType) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetAttributeElementCount)(::windows::core::Vtable::as_raw(self), name.into().abi(), r#type, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAttribute<P0>(&self, name: P0, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetAttribute)(::windows::core::Vtable::as_raw(self), name.into().abi(), r#type, elementcount, elementbytesize, value).ok()
    }
    pub unsafe fn GetStringAttributeElementLength<P0>(&self, name: P0, elementindex: u32) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStringAttributeElementLength)(::windows::core::Vtable::as_raw(self), name.into().abi(), elementindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStringAttributeElement<P0>(&self, name: P0, elementindex: u32, attributeelement: &mut [u8]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetStringAttributeElement)(::windows::core::Vtable::as_raw(self), name.into().abi(), elementindex, attributeelement.len() as _, ::core::mem::transmute(attributeelement.as_ptr())).ok()
    }
}
impl ::core::cmp::PartialEq for IMLOperatorKernelFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLOperatorKernelFactory {}
impl ::core::fmt::Debug for IMLOperatorKernelFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLOperatorKernelFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMLOperatorRegistry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLOperatorRegistry {}
impl ::core::fmt::Debug for IMLOperatorRegistry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLOperatorRegistry").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMLOperatorShapeInferenceContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLOperatorShapeInferenceContext {}
impl ::core::fmt::Debug for IMLOperatorShapeInferenceContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLOperatorShapeInferenceContext").field(&self.0).finish()
    }
}
impl IMLOperatorShapeInferenceContext {
    pub unsafe fn GetAttributeElementCount<P0>(&self, name: P0, r#type: MLOperatorAttributeType) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetAttributeElementCount)(::windows::core::Vtable::as_raw(self), name.into().abi(), r#type, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAttribute<P0>(&self, name: P0, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetAttribute)(::windows::core::Vtable::as_raw(self), name.into().abi(), r#type, elementcount, elementbytesize, value).ok()
    }
    pub unsafe fn GetStringAttributeElementLength<P0>(&self, name: P0, elementindex: u32) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStringAttributeElementLength)(::windows::core::Vtable::as_raw(self), name.into().abi(), elementindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStringAttributeElement<P0>(&self, name: P0, elementindex: u32, attributeelement: &mut [u8]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetStringAttributeElement)(::windows::core::Vtable::as_raw(self), name.into().abi(), elementindex, attributeelement.len() as _, ::core::mem::transmute(attributeelement.as_ptr())).ok()
    }
}
impl ::core::cmp::PartialEq for IMLOperatorShapeInferrer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLOperatorShapeInferrer {}
impl ::core::fmt::Debug for IMLOperatorShapeInferrer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLOperatorShapeInferrer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMLOperatorTensor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLOperatorTensor {}
impl ::core::fmt::Debug for IMLOperatorTensor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLOperatorTensor").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMLOperatorTensorShapeDescription {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLOperatorTensorShapeDescription {}
impl ::core::fmt::Debug for IMLOperatorTensorShapeDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLOperatorTensorShapeDescription").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMLOperatorTypeInferenceContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLOperatorTypeInferenceContext {}
impl ::core::fmt::Debug for IMLOperatorTypeInferenceContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLOperatorTypeInferenceContext").field(&self.0).finish()
    }
}
impl IMLOperatorTypeInferenceContext {
    pub unsafe fn GetAttributeElementCount<P0>(&self, name: P0, r#type: MLOperatorAttributeType) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetAttributeElementCount)(::windows::core::Vtable::as_raw(self), name.into().abi(), r#type, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAttribute<P0>(&self, name: P0, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetAttribute)(::windows::core::Vtable::as_raw(self), name.into().abi(), r#type, elementcount, elementbytesize, value).ok()
    }
    pub unsafe fn GetStringAttributeElementLength<P0>(&self, name: P0, elementindex: u32) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStringAttributeElementLength)(::windows::core::Vtable::as_raw(self), name.into().abi(), elementindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStringAttributeElement<P0>(&self, name: P0, elementindex: u32, attributeelement: &mut [u8]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetStringAttributeElement)(::windows::core::Vtable::as_raw(self), name.into().abi(), elementindex, attributeelement.len() as _, ::core::mem::transmute(attributeelement.as_ptr())).ok()
    }
}
impl ::core::cmp::PartialEq for IMLOperatorTypeInferrer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMLOperatorTypeInferrer {}
impl ::core::fmt::Debug for IMLOperatorTypeInferrer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMLOperatorTypeInferrer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWinMLEvaluationContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWinMLEvaluationContext {}
impl ::core::fmt::Debug for IWinMLEvaluationContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWinMLEvaluationContext").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWinMLModel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWinMLModel {}
impl ::core::fmt::Debug for IWinMLModel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWinMLModel").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWinMLRuntime {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWinMLRuntime {}
impl ::core::fmt::Debug for IWinMLRuntime {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWinMLRuntime").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWinMLRuntimeFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWinMLRuntimeFactory {}
impl ::core::fmt::Debug for IWinMLRuntimeFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWinMLRuntimeFactory").field(&self.0).finish()
    }
}
impl ::core::default::Default for MLOperatorAttribute {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MLOperatorAttribute {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.r#type == other.r#type && self.required == other.required
    }
}
impl ::core::cmp::Eq for MLOperatorAttribute {}
impl ::core::fmt::Debug for MLOperatorAttribute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MLOperatorAttribute").field("name", &self.name).field("type", &self.r#type).field("required", &self.required).finish()
    }
}
impl ::core::default::Default for MLOperatorAttributeNameValue {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MLOperatorAttributeType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MLOperatorAttributeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MLOperatorAttributeType").field(&self.0).finish()
    }
}
impl ::core::default::Default for MLOperatorEdgeDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MLOperatorEdgeType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MLOperatorEdgeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MLOperatorEdgeType").field(&self.0).finish()
    }
}
impl ::core::default::Default for MLOperatorEdgeTypeConstraint {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MLOperatorEdgeTypeConstraint {
    fn eq(&self, other: &Self) -> bool {
        self.typeLabel == other.typeLabel && self.allowedTypes == other.allowedTypes && self.allowedTypeCount == other.allowedTypeCount
    }
}
impl ::core::cmp::Eq for MLOperatorEdgeTypeConstraint {}
impl ::core::fmt::Debug for MLOperatorEdgeTypeConstraint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MLOperatorEdgeTypeConstraint").field("typeLabel", &self.typeLabel).field("allowedTypes", &self.allowedTypes).field("allowedTypeCount", &self.allowedTypeCount).finish()
    }
}
impl ::core::default::Default for MLOperatorExecutionType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MLOperatorExecutionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MLOperatorExecutionType").field(&self.0).finish()
    }
}
impl ::core::default::Default for MLOperatorKernelDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MLOperatorKernelDescription {
    fn eq(&self, other: &Self) -> bool {
        self.domain == other.domain && self.name == other.name && self.minimumOperatorSetVersion == other.minimumOperatorSetVersion && self.executionType == other.executionType && self.typeConstraints == other.typeConstraints && self.typeConstraintCount == other.typeConstraintCount && self.defaultAttributes == other.defaultAttributes && self.defaultAttributeCount == other.defaultAttributeCount && self.options == other.options && self.executionOptions == other.executionOptions
    }
}
impl ::core::cmp::Eq for MLOperatorKernelDescription {}
impl ::core::fmt::Debug for MLOperatorKernelDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MLOperatorKernelDescription")
            .field("domain", &self.domain)
            .field("name", &self.name)
            .field("minimumOperatorSetVersion", &self.minimumOperatorSetVersion)
            .field("executionType", &self.executionType)
            .field("typeConstraints", &self.typeConstraints)
            .field("typeConstraintCount", &self.typeConstraintCount)
            .field("defaultAttributes", &self.defaultAttributes)
            .field("defaultAttributeCount", &self.defaultAttributeCount)
            .field("options", &self.options)
            .field("executionOptions", &self.executionOptions)
            .finish()
    }
}
impl ::core::default::Default for MLOperatorKernelOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MLOperatorKernelOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MLOperatorKernelOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MLOperatorKernelOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MLOperatorKernelOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MLOperatorKernelOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MLOperatorKernelOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MLOperatorKernelOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for MLOperatorParameterOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MLOperatorParameterOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MLOperatorParameterOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MLOperatorParameterOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MLOperatorParameterOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MLOperatorParameterOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MLOperatorParameterOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MLOperatorParameterOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for MLOperatorSchemaDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MLOperatorSchemaDescription {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.operatorSetVersionAtLastChange == other.operatorSetVersionAtLastChange && self.inputs == other.inputs && self.inputCount == other.inputCount && self.outputs == other.outputs && self.outputCount == other.outputCount && self.typeConstraints == other.typeConstraints && self.typeConstraintCount == other.typeConstraintCount && self.attributes == other.attributes && self.attributeCount == other.attributeCount && self.defaultAttributes == other.defaultAttributes && self.defaultAttributeCount == other.defaultAttributeCount
    }
}
impl ::core::cmp::Eq for MLOperatorSchemaDescription {}
impl ::core::fmt::Debug for MLOperatorSchemaDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MLOperatorSchemaDescription")
            .field("name", &self.name)
            .field("operatorSetVersionAtLastChange", &self.operatorSetVersionAtLastChange)
            .field("inputs", &self.inputs)
            .field("inputCount", &self.inputCount)
            .field("outputs", &self.outputs)
            .field("outputCount", &self.outputCount)
            .field("typeConstraints", &self.typeConstraints)
            .field("typeConstraintCount", &self.typeConstraintCount)
            .field("attributes", &self.attributes)
            .field("attributeCount", &self.attributeCount)
            .field("defaultAttributes", &self.defaultAttributes)
            .field("defaultAttributeCount", &self.defaultAttributeCount)
            .finish()
    }
}
impl ::core::default::Default for MLOperatorSchemaEdgeDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MLOperatorSchemaEdgeTypeFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MLOperatorSchemaEdgeTypeFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MLOperatorSchemaEdgeTypeFormat").field(&self.0).finish()
    }
}
impl ::core::default::Default for MLOperatorSetId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MLOperatorSetId {
    fn eq(&self, other: &Self) -> bool {
        self.domain == other.domain && self.version == other.version
    }
}
impl ::core::cmp::Eq for MLOperatorSetId {}
impl ::core::fmt::Debug for MLOperatorSetId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MLOperatorSetId").field("domain", &self.domain).field("version", &self.version).finish()
    }
}
impl ::core::default::Default for MLOperatorTensorDataType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MLOperatorTensorDataType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MLOperatorTensorDataType").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::default::Default for WINML_BINDING_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WINML_BINDING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINML_BINDING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINML_BINDING_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WINML_FEATURE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINML_FEATURE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINML_FEATURE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WINML_IMAGE_BINDING_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINML_IMAGE_BINDING_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ElementType == other.ElementType && self.NumDimensions == other.NumDimensions && self.pShape == other.pShape && self.DataSize == other.DataSize && self.pData == other.pData
    }
}
impl ::core::cmp::Eq for WINML_IMAGE_BINDING_DESC {}
impl ::core::fmt::Debug for WINML_IMAGE_BINDING_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINML_IMAGE_BINDING_DESC").field("ElementType", &self.ElementType).field("NumDimensions", &self.NumDimensions).field("pShape", &self.pShape).field("DataSize", &self.DataSize).field("pData", &self.pData).finish()
    }
}
impl ::core::default::Default for WINML_IMAGE_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINML_IMAGE_VARIABLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ElementType == other.ElementType && self.NumDimensions == other.NumDimensions && self.pShape == other.pShape
    }
}
impl ::core::cmp::Eq for WINML_IMAGE_VARIABLE_DESC {}
impl ::core::fmt::Debug for WINML_IMAGE_VARIABLE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINML_IMAGE_VARIABLE_DESC").field("ElementType", &self.ElementType).field("NumDimensions", &self.NumDimensions).field("pShape", &self.pShape).finish()
    }
}
impl ::core::default::Default for WINML_MAP_BINDING_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WINML_MAP_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINML_MAP_VARIABLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.KeyType == other.KeyType && self.Fields == other.Fields
    }
}
impl ::core::cmp::Eq for WINML_MAP_VARIABLE_DESC {}
impl ::core::fmt::Debug for WINML_MAP_VARIABLE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINML_MAP_VARIABLE_DESC").field("KeyType", &self.KeyType).field("Fields", &self.Fields).finish()
    }
}
impl ::core::default::Default for WINML_MODEL_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINML_MODEL_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Author == other.Author && self.Name == other.Name && self.Domain == other.Domain && self.Description == other.Description && self.Version == other.Version
    }
}
impl ::core::cmp::Eq for WINML_MODEL_DESC {}
impl ::core::fmt::Debug for WINML_MODEL_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINML_MODEL_DESC").field("Author", &self.Author).field("Name", &self.Name).field("Domain", &self.Domain).field("Description", &self.Description).field("Version", &self.Version).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::default::Default for WINML_RESOURCE_BINDING_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for WINML_RESOURCE_BINDING_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ElementType == other.ElementType && self.NumDimensions == other.NumDimensions && self.pShape == other.pShape && self.pResource == other.pResource
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for WINML_RESOURCE_BINDING_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for WINML_RESOURCE_BINDING_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINML_RESOURCE_BINDING_DESC").field("ElementType", &self.ElementType).field("NumDimensions", &self.NumDimensions).field("pShape", &self.pShape).field("pResource", &self.pResource).finish()
    }
}
impl ::core::default::Default for WINML_RUNTIME_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINML_RUNTIME_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINML_RUNTIME_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WINML_SEQUENCE_BINDING_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WINML_SEQUENCE_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINML_SEQUENCE_VARIABLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ElementType == other.ElementType
    }
}
impl ::core::cmp::Eq for WINML_SEQUENCE_VARIABLE_DESC {}
impl ::core::fmt::Debug for WINML_SEQUENCE_VARIABLE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINML_SEQUENCE_VARIABLE_DESC").field("ElementType", &self.ElementType).finish()
    }
}
impl ::core::default::Default for WINML_TENSOR_BINDING_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINML_TENSOR_BINDING_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.DataType == other.DataType && self.NumDimensions == other.NumDimensions && self.pShape == other.pShape && self.DataSize == other.DataSize && self.pData == other.pData
    }
}
impl ::core::cmp::Eq for WINML_TENSOR_BINDING_DESC {}
impl ::core::fmt::Debug for WINML_TENSOR_BINDING_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINML_TENSOR_BINDING_DESC").field("DataType", &self.DataType).field("NumDimensions", &self.NumDimensions).field("pShape", &self.pShape).field("DataSize", &self.DataSize).field("pData", &self.pData).finish()
    }
}
impl ::core::default::Default for WINML_TENSOR_DATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINML_TENSOR_DATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINML_TENSOR_DATA_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WINML_TENSOR_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINML_TENSOR_VARIABLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ElementType == other.ElementType && self.NumDimensions == other.NumDimensions && self.pShape == other.pShape
    }
}
impl ::core::cmp::Eq for WINML_TENSOR_VARIABLE_DESC {}
impl ::core::fmt::Debug for WINML_TENSOR_VARIABLE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINML_TENSOR_VARIABLE_DESC").field("ElementType", &self.ElementType).field("NumDimensions", &self.NumDimensions).field("pShape", &self.pShape).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINML_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
