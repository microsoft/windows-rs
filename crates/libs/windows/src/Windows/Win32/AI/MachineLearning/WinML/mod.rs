#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
#[inline]
pub unsafe fn MLCreateOperatorRegistry() -> ::windows::core::Result<IMLOperatorRegistry> {
    ::windows::imp::link ! ( "windows.ai.machinelearning.dll""system" fn MLCreateOperatorRegistry ( registry : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IMLOperatorRegistry>();
    MLCreateOperatorRegistry(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
#[inline]
pub unsafe fn WinMLCreateRuntime() -> ::windows::core::Result<IWinMLRuntime> {
    ::windows::imp::link ! ( "winml.dll""system" fn WinMLCreateRuntime ( runtime : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IWinMLRuntime>();
    WinMLCreateRuntime(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
#[repr(transparent)]
pub struct IMLOperatorAttributes(::windows::core::IUnknown);
impl IMLOperatorAttributes {
    pub unsafe fn GetAttributeElementCount<P0>(&self, name: P0, r#type: MLOperatorAttributeType) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetAttributeElementCount)(::windows::core::Interface::as_raw(self), name.into_param().abi(), r#type, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAttribute<P0>(&self, name: P0, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).GetAttribute)(::windows::core::Interface::as_raw(self), name.into_param().abi(), r#type, elementcount, elementbytesize, value).ok()
    }
    pub unsafe fn GetStringAttributeElementLength<P0>(&self, name: P0, elementindex: u32) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetStringAttributeElementLength)(::windows::core::Interface::as_raw(self), name.into_param().abi(), elementindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStringAttributeElement<P0>(&self, name: P0, elementindex: u32, attributeelement: &mut [u8]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).GetStringAttributeElement)(::windows::core::Interface::as_raw(self), name.into_param().abi(), elementindex, attributeelement.len() as _, ::core::mem::transmute(attributeelement.as_ptr())).ok()
    }
}
::windows::imp::interface_hierarchy!(IMLOperatorAttributes, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IMLOperatorAttributes {
    type Vtable = IMLOperatorAttributes_Vtbl;
}
impl ::core::clone::Clone for IMLOperatorAttributes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMLOperatorAttributes {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b1b1759_ec40_466c_aab4_beb5347fd24c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorAttributes_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetAttributeElementCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR, r#type: MLOperatorAttributeType, elementcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetStringAttributeElementLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR, elementindex: u32, attributeelementbytesize: *mut u32) -> ::windows::core::HRESULT,
    pub GetStringAttributeElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCSTR, elementindex: u32, attributeelementbytesize: u32, attributeelement: ::windows::core::PSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
#[repr(transparent)]
pub struct IMLOperatorKernel(::windows::core::IUnknown);
impl IMLOperatorKernel {
    pub unsafe fn Compute<P0>(&self, context: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMLOperatorKernelContext>,
    {
        (::windows::core::Interface::vtable(self).Compute)(::windows::core::Interface::as_raw(self), context.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IMLOperatorKernel, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IMLOperatorKernel {
    type Vtable = IMLOperatorKernel_Vtbl;
}
impl ::core::clone::Clone for IMLOperatorKernel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMLOperatorKernel {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11c4b4a0_b467_4eaa_a1a6_b961d8d0ed79);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorKernel_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Compute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
#[repr(transparent)]
pub struct IMLOperatorKernelContext(::windows::core::IUnknown);
impl IMLOperatorKernelContext {
    pub unsafe fn GetInputTensor(&self, inputindex: u32) -> ::windows::core::Result<IMLOperatorTensor> {
        let mut result__ = ::windows::core::zeroed::<IMLOperatorTensor>();
        (::windows::core::Interface::vtable(self).GetInputTensor)(::windows::core::Interface::as_raw(self), inputindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetOutputTensor(&self, outputindex: u32, dimensionsizes: &[u32]) -> ::windows::core::Result<IMLOperatorTensor> {
        let mut result__ = ::windows::core::zeroed::<IMLOperatorTensor>();
        (::windows::core::Interface::vtable(self).GetOutputTensor)(::windows::core::Interface::as_raw(self), outputindex, dimensionsizes.len() as _, ::core::mem::transmute(dimensionsizes.as_ptr()), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetOutputTensor2(&self, outputindex: u32) -> ::windows::core::Result<IMLOperatorTensor> {
        let mut result__ = ::windows::core::zeroed::<IMLOperatorTensor>();
        (::windows::core::Interface::vtable(self).GetOutputTensor2)(::windows::core::Interface::as_raw(self), outputindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn AllocateTemporaryData(&self, size: usize) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).AllocateTemporaryData)(::windows::core::Interface::as_raw(self), size, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetExecutionInterface(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).GetExecutionInterface)(::windows::core::Interface::as_raw(self), &mut result__);
        ::windows::core::from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMLOperatorKernelContext, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IMLOperatorKernelContext {
    type Vtable = IMLOperatorKernelContext_Vtbl;
}
impl ::core::clone::Clone for IMLOperatorKernelContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMLOperatorKernelContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82536a28_f022_4769_9d3f_8b278f84c0c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorKernelContext_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetInputTensor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputindex: u32, tensor: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetOutputTensor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputindex: u32, dimensioncount: u32, dimensionsizes: *const u32, tensor: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetOutputTensor2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputindex: u32, tensor: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AllocateTemporaryData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: usize, data: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetExecutionInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, executionobject: *mut *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
#[repr(transparent)]
pub struct IMLOperatorKernelCreationContext(::windows::core::IUnknown);
impl IMLOperatorKernelCreationContext {
    pub unsafe fn GetAttributeElementCount<P0>(&self, name: P0, r#type: MLOperatorAttributeType) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetAttributeElementCount)(::windows::core::Interface::as_raw(self), name.into_param().abi(), r#type, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAttribute<P0>(&self, name: P0, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetAttribute)(::windows::core::Interface::as_raw(self), name.into_param().abi(), r#type, elementcount, elementbytesize, value).ok()
    }
    pub unsafe fn GetStringAttributeElementLength<P0>(&self, name: P0, elementindex: u32) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetStringAttributeElementLength)(::windows::core::Interface::as_raw(self), name.into_param().abi(), elementindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStringAttributeElement<P0>(&self, name: P0, elementindex: u32, attributeelement: &mut [u8]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetStringAttributeElement)(::windows::core::Interface::as_raw(self), name.into_param().abi(), elementindex, attributeelement.len() as _, ::core::mem::transmute(attributeelement.as_ptr())).ok()
    }
    pub unsafe fn GetInputCount(&self) -> u32 {
        (::windows::core::Interface::vtable(self).GetInputCount)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetOutputCount(&self) -> u32 {
        (::windows::core::Interface::vtable(self).GetOutputCount)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn IsInputValid(&self, inputindex: u32) -> bool {
        (::windows::core::Interface::vtable(self).IsInputValid)(::windows::core::Interface::as_raw(self), inputindex)
    }
    pub unsafe fn IsOutputValid(&self, outputindex: u32) -> bool {
        (::windows::core::Interface::vtable(self).IsOutputValid)(::windows::core::Interface::as_raw(self), outputindex)
    }
    pub unsafe fn GetInputEdgeDescription(&self, inputindex: u32) -> ::windows::core::Result<MLOperatorEdgeDescription> {
        let mut result__ = ::windows::core::zeroed::<MLOperatorEdgeDescription>();
        (::windows::core::Interface::vtable(self).GetInputEdgeDescription)(::windows::core::Interface::as_raw(self), inputindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetOutputEdgeDescription(&self, outputindex: u32) -> ::windows::core::Result<MLOperatorEdgeDescription> {
        let mut result__ = ::windows::core::zeroed::<MLOperatorEdgeDescription>();
        (::windows::core::Interface::vtable(self).GetOutputEdgeDescription)(::windows::core::Interface::as_raw(self), outputindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn HasTensorShapeDescription(&self) -> bool {
        (::windows::core::Interface::vtable(self).HasTensorShapeDescription)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetTensorShapeDescription(&self) -> ::windows::core::Result<IMLOperatorTensorShapeDescription> {
        let mut result__ = ::windows::core::zeroed::<IMLOperatorTensorShapeDescription>();
        (::windows::core::Interface::vtable(self).GetTensorShapeDescription)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetExecutionInterface(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).GetExecutionInterface)(::windows::core::Interface::as_raw(self), &mut result__);
        ::windows::core::from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMLOperatorKernelCreationContext, ::windows::core::IUnknown, IMLOperatorAttributes);
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
unsafe impl ::windows::core::Interface for IMLOperatorKernelCreationContext {
    type Vtable = IMLOperatorKernelCreationContext_Vtbl;
}
impl ::core::clone::Clone for IMLOperatorKernelCreationContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMLOperatorKernelCreationContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5459b53d_a0fc_4665_addd_70171ef7e631);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorKernelCreationContext_Vtbl {
    pub base__: IMLOperatorAttributes_Vtbl,
    pub GetInputCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetOutputCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub IsInputValid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputindex: u32) -> bool,
    pub IsOutputValid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputindex: u32) -> bool,
    pub GetInputEdgeDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows::core::HRESULT,
    pub GetOutputEdgeDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows::core::HRESULT,
    pub HasTensorShapeDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> bool,
    pub GetTensorShapeDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shapedescription: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetExecutionInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, executionobject: *mut *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
#[repr(transparent)]
pub struct IMLOperatorKernelFactory(::windows::core::IUnknown);
impl IMLOperatorKernelFactory {
    pub unsafe fn CreateKernel<P0>(&self, context: P0) -> ::windows::core::Result<IMLOperatorKernel>
    where
        P0: ::windows::core::IntoParam<IMLOperatorKernelCreationContext>,
    {
        let mut result__ = ::windows::core::zeroed::<IMLOperatorKernel>();
        (::windows::core::Interface::vtable(self).CreateKernel)(::windows::core::Interface::as_raw(self), context.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMLOperatorKernelFactory, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IMLOperatorKernelFactory {
    type Vtable = IMLOperatorKernelFactory_Vtbl;
}
impl ::core::clone::Clone for IMLOperatorKernelFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMLOperatorKernelFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef15ad6f_0dc9_4908_ab35_a575a30dfbf8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorKernelFactory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateKernel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void, kernel: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
#[repr(transparent)]
pub struct IMLOperatorRegistry(::windows::core::IUnknown);
impl IMLOperatorRegistry {
    pub unsafe fn RegisterOperatorSetSchema<P0, P1>(&self, operatorsetid: *const MLOperatorSetId, baselineversion: i32, schema: ::core::option::Option<&[*const MLOperatorSchemaDescription]>, typeinferrer: P0, shapeinferrer: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMLOperatorTypeInferrer>,
        P1: ::windows::core::IntoParam<IMLOperatorShapeInferrer>,
    {
        (::windows::core::Interface::vtable(self).RegisterOperatorSetSchema)(::windows::core::Interface::as_raw(self), operatorsetid, baselineversion, ::core::mem::transmute(schema.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), schema.as_deref().map_or(0, |slice| slice.len() as _), typeinferrer.into_param().abi(), shapeinferrer.into_param().abi()).ok()
    }
    pub unsafe fn RegisterOperatorKernel<P0, P1>(&self, operatorkernel: *const MLOperatorKernelDescription, operatorkernelfactory: P0, shapeinferrer: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMLOperatorKernelFactory>,
        P1: ::windows::core::IntoParam<IMLOperatorShapeInferrer>,
    {
        (::windows::core::Interface::vtable(self).RegisterOperatorKernel)(::windows::core::Interface::as_raw(self), operatorkernel, operatorkernelfactory.into_param().abi(), shapeinferrer.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IMLOperatorRegistry, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IMLOperatorRegistry {
    type Vtable = IMLOperatorRegistry_Vtbl;
}
impl ::core::clone::Clone for IMLOperatorRegistry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMLOperatorRegistry {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2af9dd2d_b516_4672_9ab5_530c208493ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorRegistry_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub RegisterOperatorSetSchema: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operatorsetid: *const MLOperatorSetId, baselineversion: i32, schema: *const *const MLOperatorSchemaDescription, schemacount: u32, typeinferrer: *mut ::core::ffi::c_void, shapeinferrer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RegisterOperatorKernel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operatorkernel: *const MLOperatorKernelDescription, operatorkernelfactory: *mut ::core::ffi::c_void, shapeinferrer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
#[repr(transparent)]
pub struct IMLOperatorShapeInferenceContext(::windows::core::IUnknown);
impl IMLOperatorShapeInferenceContext {
    pub unsafe fn GetAttributeElementCount<P0>(&self, name: P0, r#type: MLOperatorAttributeType) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetAttributeElementCount)(::windows::core::Interface::as_raw(self), name.into_param().abi(), r#type, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAttribute<P0>(&self, name: P0, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetAttribute)(::windows::core::Interface::as_raw(self), name.into_param().abi(), r#type, elementcount, elementbytesize, value).ok()
    }
    pub unsafe fn GetStringAttributeElementLength<P0>(&self, name: P0, elementindex: u32) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetStringAttributeElementLength)(::windows::core::Interface::as_raw(self), name.into_param().abi(), elementindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStringAttributeElement<P0>(&self, name: P0, elementindex: u32, attributeelement: &mut [u8]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetStringAttributeElement)(::windows::core::Interface::as_raw(self), name.into_param().abi(), elementindex, attributeelement.len() as _, ::core::mem::transmute(attributeelement.as_ptr())).ok()
    }
    pub unsafe fn GetInputCount(&self) -> u32 {
        (::windows::core::Interface::vtable(self).GetInputCount)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetOutputCount(&self) -> u32 {
        (::windows::core::Interface::vtable(self).GetOutputCount)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn IsInputValid(&self, inputindex: u32) -> bool {
        (::windows::core::Interface::vtable(self).IsInputValid)(::windows::core::Interface::as_raw(self), inputindex)
    }
    pub unsafe fn IsOutputValid(&self, outputindex: u32) -> bool {
        (::windows::core::Interface::vtable(self).IsOutputValid)(::windows::core::Interface::as_raw(self), outputindex)
    }
    pub unsafe fn GetInputEdgeDescription(&self, inputindex: u32) -> ::windows::core::Result<MLOperatorEdgeDescription> {
        let mut result__ = ::windows::core::zeroed::<MLOperatorEdgeDescription>();
        (::windows::core::Interface::vtable(self).GetInputEdgeDescription)(::windows::core::Interface::as_raw(self), inputindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInputTensorDimensionCount(&self, inputindex: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetInputTensorDimensionCount)(::windows::core::Interface::as_raw(self), inputindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInputTensorShape(&self, inputindex: u32, dimensions: &mut [u32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetInputTensorShape)(::windows::core::Interface::as_raw(self), inputindex, dimensions.len() as _, ::core::mem::transmute(dimensions.as_ptr())).ok()
    }
    pub unsafe fn SetOutputTensorShape(&self, outputindex: u32, dimensioncount: u32, dimensions: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOutputTensorShape)(::windows::core::Interface::as_raw(self), outputindex, dimensioncount, dimensions).ok()
    }
}
::windows::imp::interface_hierarchy!(IMLOperatorShapeInferenceContext, ::windows::core::IUnknown, IMLOperatorAttributes);
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
unsafe impl ::windows::core::Interface for IMLOperatorShapeInferenceContext {
    type Vtable = IMLOperatorShapeInferenceContext_Vtbl;
}
impl ::core::clone::Clone for IMLOperatorShapeInferenceContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMLOperatorShapeInferenceContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x105b6b29_5408_4a68_9959_09b5955a3492);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorShapeInferenceContext_Vtbl {
    pub base__: IMLOperatorAttributes_Vtbl,
    pub GetInputCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetOutputCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub IsInputValid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputindex: u32) -> bool,
    pub IsOutputValid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputindex: u32) -> bool,
    pub GetInputEdgeDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows::core::HRESULT,
    pub GetInputTensorDimensionCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputindex: u32, dimensioncount: *mut u32) -> ::windows::core::HRESULT,
    pub GetInputTensorShape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::HRESULT,
    pub SetOutputTensorShape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputindex: u32, dimensioncount: u32, dimensions: *const u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
#[repr(transparent)]
pub struct IMLOperatorShapeInferrer(::windows::core::IUnknown);
impl IMLOperatorShapeInferrer {
    pub unsafe fn InferOutputShapes<P0>(&self, context: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMLOperatorShapeInferenceContext>,
    {
        (::windows::core::Interface::vtable(self).InferOutputShapes)(::windows::core::Interface::as_raw(self), context.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IMLOperatorShapeInferrer, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IMLOperatorShapeInferrer {
    type Vtable = IMLOperatorShapeInferrer_Vtbl;
}
impl ::core::clone::Clone for IMLOperatorShapeInferrer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMLOperatorShapeInferrer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x540be5be_a6c9_40ee_83f6_d2b8b40a7798);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorShapeInferrer_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub InferOutputShapes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
#[repr(transparent)]
pub struct IMLOperatorTensor(::windows::core::IUnknown);
impl IMLOperatorTensor {
    pub unsafe fn GetDimensionCount(&self) -> u32 {
        (::windows::core::Interface::vtable(self).GetDimensionCount)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetShape(&self, dimensions: &mut [u32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetShape)(::windows::core::Interface::as_raw(self), dimensions.len() as _, ::core::mem::transmute(dimensions.as_ptr())).ok()
    }
    pub unsafe fn GetTensorDataType(&self) -> MLOperatorTensorDataType {
        (::windows::core::Interface::vtable(self).GetTensorDataType)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn IsCpuData(&self) -> bool {
        (::windows::core::Interface::vtable(self).IsCpuData)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn IsDataInterface(&self) -> bool {
        (::windows::core::Interface::vtable(self).IsDataInterface)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetData(&self) -> *mut ::core::ffi::c_void {
        (::windows::core::Interface::vtable(self).GetData)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetDataInterface(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).GetDataInterface)(::windows::core::Interface::as_raw(self), &mut result__);
        ::windows::core::from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMLOperatorTensor, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IMLOperatorTensor {
    type Vtable = IMLOperatorTensor_Vtbl;
}
impl ::core::clone::Clone for IMLOperatorTensor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMLOperatorTensor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fe41f41_f430_440e_aece_54416dc8b9db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorTensor_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDimensionCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetShape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::HRESULT,
    pub GetTensorDataType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> MLOperatorTensorDataType,
    pub IsCpuData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> bool,
    pub IsDataInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> bool,
    pub GetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
    pub GetDataInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datainterface: *mut *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
#[repr(transparent)]
pub struct IMLOperatorTensorShapeDescription(::windows::core::IUnknown);
impl IMLOperatorTensorShapeDescription {
    pub unsafe fn GetInputTensorDimensionCount(&self, inputindex: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetInputTensorDimensionCount)(::windows::core::Interface::as_raw(self), inputindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInputTensorShape(&self, inputindex: u32, dimensions: &mut [u32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetInputTensorShape)(::windows::core::Interface::as_raw(self), inputindex, dimensions.len() as _, ::core::mem::transmute(dimensions.as_ptr())).ok()
    }
    pub unsafe fn HasOutputShapeDescription(&self) -> bool {
        (::windows::core::Interface::vtable(self).HasOutputShapeDescription)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetOutputTensorDimensionCount(&self, outputindex: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetOutputTensorDimensionCount)(::windows::core::Interface::as_raw(self), outputindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetOutputTensorShape(&self, outputindex: u32, dimensions: &mut [u32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOutputTensorShape)(::windows::core::Interface::as_raw(self), outputindex, dimensions.len() as _, ::core::mem::transmute(dimensions.as_ptr())).ok()
    }
}
::windows::imp::interface_hierarchy!(IMLOperatorTensorShapeDescription, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IMLOperatorTensorShapeDescription {
    type Vtable = IMLOperatorTensorShapeDescription_Vtbl;
}
impl ::core::clone::Clone for IMLOperatorTensorShapeDescription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMLOperatorTensorShapeDescription {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf20e8cbe_3b28_4248_be95_f96fbc6e4643);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorTensorShapeDescription_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetInputTensorDimensionCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputindex: u32, dimensioncount: *mut u32) -> ::windows::core::HRESULT,
    pub GetInputTensorShape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::HRESULT,
    pub HasOutputShapeDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> bool,
    pub GetOutputTensorDimensionCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputindex: u32, dimensioncount: *mut u32) -> ::windows::core::HRESULT,
    pub GetOutputTensorShape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
#[repr(transparent)]
pub struct IMLOperatorTypeInferenceContext(::windows::core::IUnknown);
impl IMLOperatorTypeInferenceContext {
    pub unsafe fn GetAttributeElementCount<P0>(&self, name: P0, r#type: MLOperatorAttributeType) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetAttributeElementCount)(::windows::core::Interface::as_raw(self), name.into_param().abi(), r#type, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAttribute<P0>(&self, name: P0, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetAttribute)(::windows::core::Interface::as_raw(self), name.into_param().abi(), r#type, elementcount, elementbytesize, value).ok()
    }
    pub unsafe fn GetStringAttributeElementLength<P0>(&self, name: P0, elementindex: u32) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetStringAttributeElementLength)(::windows::core::Interface::as_raw(self), name.into_param().abi(), elementindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStringAttributeElement<P0>(&self, name: P0, elementindex: u32, attributeelement: &mut [u8]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetStringAttributeElement)(::windows::core::Interface::as_raw(self), name.into_param().abi(), elementindex, attributeelement.len() as _, ::core::mem::transmute(attributeelement.as_ptr())).ok()
    }
    pub unsafe fn GetInputCount(&self) -> u32 {
        (::windows::core::Interface::vtable(self).GetInputCount)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetOutputCount(&self) -> u32 {
        (::windows::core::Interface::vtable(self).GetOutputCount)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn IsInputValid(&self, inputindex: u32) -> bool {
        (::windows::core::Interface::vtable(self).IsInputValid)(::windows::core::Interface::as_raw(self), inputindex)
    }
    pub unsafe fn IsOutputValid(&self, outputindex: u32) -> bool {
        (::windows::core::Interface::vtable(self).IsOutputValid)(::windows::core::Interface::as_raw(self), outputindex)
    }
    pub unsafe fn GetInputEdgeDescription(&self, inputindex: u32) -> ::windows::core::Result<MLOperatorEdgeDescription> {
        let mut result__ = ::windows::core::zeroed::<MLOperatorEdgeDescription>();
        (::windows::core::Interface::vtable(self).GetInputEdgeDescription)(::windows::core::Interface::as_raw(self), inputindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOutputEdgeDescription(&self, outputindex: u32, edgedescription: *const MLOperatorEdgeDescription) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOutputEdgeDescription)(::windows::core::Interface::as_raw(self), outputindex, edgedescription).ok()
    }
}
::windows::imp::interface_hierarchy!(IMLOperatorTypeInferenceContext, ::windows::core::IUnknown, IMLOperatorAttributes);
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
unsafe impl ::windows::core::Interface for IMLOperatorTypeInferenceContext {
    type Vtable = IMLOperatorTypeInferenceContext_Vtbl;
}
impl ::core::clone::Clone for IMLOperatorTypeInferenceContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMLOperatorTypeInferenceContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec893bb1_f938_427b_8488_c8dcf775f138);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorTypeInferenceContext_Vtbl {
    pub base__: IMLOperatorAttributes_Vtbl,
    pub GetInputCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetOutputCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub IsInputValid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputindex: u32) -> bool,
    pub IsOutputValid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputindex: u32) -> bool,
    pub GetInputEdgeDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows::core::HRESULT,
    pub SetOutputEdgeDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputindex: u32, edgedescription: *const MLOperatorEdgeDescription) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
#[repr(transparent)]
pub struct IMLOperatorTypeInferrer(::windows::core::IUnknown);
impl IMLOperatorTypeInferrer {
    pub unsafe fn InferOutputTypes<P0>(&self, context: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMLOperatorTypeInferenceContext>,
    {
        (::windows::core::Interface::vtable(self).InferOutputTypes)(::windows::core::Interface::as_raw(self), context.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IMLOperatorTypeInferrer, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IMLOperatorTypeInferrer {
    type Vtable = IMLOperatorTypeInferrer_Vtbl;
}
impl ::core::clone::Clone for IMLOperatorTypeInferrer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMLOperatorTypeInferrer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x781aeb48_9bcb_4797_bf77_8bf455217beb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMLOperatorTypeInferrer_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub InferOutputTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
#[repr(transparent)]
pub struct IWinMLEvaluationContext(::windows::core::IUnknown);
impl IWinMLEvaluationContext {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn BindValue(&self, pdescriptor: *const WINML_BINDING_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BindValue)(::windows::core::Interface::as_raw(self), pdescriptor).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetValueByName<P0>(&self, name: P0) -> ::windows::core::Result<*mut WINML_BINDING_DESC>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<*mut WINML_BINDING_DESC>();
        (::windows::core::Interface::vtable(self).GetValueByName)(::windows::core::Interface::as_raw(self), name.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Clear)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IWinMLEvaluationContext, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IWinMLEvaluationContext {
    type Vtable = IWinMLEvaluationContext_Vtbl;
}
impl ::core::clone::Clone for IWinMLEvaluationContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWinMLEvaluationContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95848f9e_583d_4054_af12_916387cd8426);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinMLEvaluationContext_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub BindValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdescriptor: *const WINML_BINDING_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    BindValue: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub GetValueByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, pdescriptor: *mut *mut WINML_BINDING_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    GetValueByName: usize,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
#[repr(transparent)]
pub struct IWinMLModel(::windows::core::IUnknown);
impl IWinMLModel {
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<*mut WINML_MODEL_DESC> {
        let mut result__ = ::windows::core::zeroed::<*mut WINML_MODEL_DESC>();
        (::windows::core::Interface::vtable(self).GetDescription)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumerateMetadata(&self, index: u32, pkey: *mut ::windows::core::PWSTR, pvalue: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumerateMetadata)(::windows::core::Interface::as_raw(self), index, pkey, pvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumerateModelInputs(&self, index: u32) -> ::windows::core::Result<*mut WINML_VARIABLE_DESC> {
        let mut result__ = ::windows::core::zeroed::<*mut WINML_VARIABLE_DESC>();
        (::windows::core::Interface::vtable(self).EnumerateModelInputs)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumerateModelOutputs(&self, index: u32) -> ::windows::core::Result<*mut WINML_VARIABLE_DESC> {
        let mut result__ = ::windows::core::zeroed::<*mut WINML_VARIABLE_DESC>();
        (::windows::core::Interface::vtable(self).EnumerateModelOutputs)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWinMLModel, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IWinMLModel {
    type Vtable = IWinMLModel_Vtbl;
}
impl ::core::clone::Clone for IWinMLModel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWinMLModel {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2eeb6a9_f31f_4055_a521_e30b5b33664a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinMLModel_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdescription: *mut *mut WINML_MODEL_DESC) -> ::windows::core::HRESULT,
    pub EnumerateMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pkey: *mut ::windows::core::PWSTR, pvalue: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumerateModelInputs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, ppinputdescriptor: *mut *mut WINML_VARIABLE_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumerateModelInputs: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumerateModelOutputs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, ppoutputdescriptor: *mut *mut WINML_VARIABLE_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumerateModelOutputs: usize,
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
#[repr(transparent)]
pub struct IWinMLRuntime(::windows::core::IUnknown);
impl IWinMLRuntime {
    pub unsafe fn LoadModel<P0>(&self, path: P0) -> ::windows::core::Result<IWinMLModel>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IWinMLModel>();
        (::windows::core::Interface::vtable(self).LoadModel)(::windows::core::Interface::as_raw(self), path.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CreateEvaluationContext<P0>(&self, device: P0) -> ::windows::core::Result<IWinMLEvaluationContext>
    where
        P0: ::windows::core::IntoParam<super::super::super::Graphics::Direct3D12::ID3D12Device>,
    {
        let mut result__ = ::windows::core::zeroed::<IWinMLEvaluationContext>();
        (::windows::core::Interface::vtable(self).CreateEvaluationContext)(::windows::core::Interface::as_raw(self), device.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EvaluateModel<P0>(&self, pcontext: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWinMLEvaluationContext>,
    {
        (::windows::core::Interface::vtable(self).EvaluateModel)(::windows::core::Interface::as_raw(self), pcontext.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IWinMLRuntime, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IWinMLRuntime {
    type Vtable = IWinMLRuntime_Vtbl;
}
impl ::core::clone::Clone for IWinMLRuntime {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWinMLRuntime {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0425329_40ae_48d9_bce3_829ef7b8a41a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinMLRuntime_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub LoadModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::windows::core::PCWSTR, ppmodel: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CreateEvaluationContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CreateEvaluationContext: usize,
    pub EvaluateModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
#[repr(transparent)]
pub struct IWinMLRuntimeFactory(::windows::core::IUnknown);
impl IWinMLRuntimeFactory {
    pub unsafe fn CreateRuntime(&self, runtimetype: WINML_RUNTIME_TYPE) -> ::windows::core::Result<IWinMLRuntime> {
        let mut result__ = ::windows::core::zeroed::<IWinMLRuntime>();
        (::windows::core::Interface::vtable(self).CreateRuntime)(::windows::core::Interface::as_raw(self), runtimetype, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWinMLRuntimeFactory, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IWinMLRuntimeFactory {
    type Vtable = IWinMLRuntimeFactory_Vtbl;
}
impl ::core::clone::Clone for IWinMLRuntimeFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWinMLRuntimeFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa807b84d_4ae5_4bc0_a76a_941aa246bd41);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinMLRuntimeFactory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateRuntime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, runtimetype: WINML_RUNTIME_TYPE, ppruntime: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_TENSOR_DIMENSION_COUNT_MAX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MLOperatorAttributeType(pub u32);
impl MLOperatorAttributeType {
    pub const Undefined: Self = Self(0u32);
    pub const Float: Self = Self(2u32);
    pub const Int: Self = Self(3u32);
    pub const String: Self = Self(4u32);
    pub const FloatArray: Self = Self(7u32);
    pub const IntArray: Self = Self(8u32);
    pub const StringArray: Self = Self(9u32);
}
impl ::core::marker::Copy for MLOperatorAttributeType {}
impl ::core::clone::Clone for MLOperatorAttributeType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MLOperatorAttributeType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MLOperatorAttributeType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MLOperatorAttributeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MLOperatorAttributeType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MLOperatorEdgeType(pub u32);
impl MLOperatorEdgeType {
    pub const Undefined: Self = Self(0u32);
    pub const Tensor: Self = Self(1u32);
}
impl ::core::marker::Copy for MLOperatorEdgeType {}
impl ::core::clone::Clone for MLOperatorEdgeType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MLOperatorEdgeType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MLOperatorEdgeType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MLOperatorEdgeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MLOperatorEdgeType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MLOperatorExecutionType(pub u32);
impl MLOperatorExecutionType {
    pub const Undefined: Self = Self(0u32);
    pub const Cpu: Self = Self(1u32);
    pub const D3D12: Self = Self(2u32);
}
impl ::core::marker::Copy for MLOperatorExecutionType {}
impl ::core::clone::Clone for MLOperatorExecutionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MLOperatorExecutionType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MLOperatorExecutionType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MLOperatorExecutionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MLOperatorExecutionType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MLOperatorKernelOptions(pub u32);
impl MLOperatorKernelOptions {
    pub const None: Self = Self(0u32);
    pub const AllowDynamicInputShapes: Self = Self(1u32);
}
impl ::core::marker::Copy for MLOperatorKernelOptions {}
impl ::core::clone::Clone for MLOperatorKernelOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MLOperatorKernelOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MLOperatorKernelOptions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MLOperatorKernelOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MLOperatorKernelOptions").field(&self.0).finish()
    }
}
impl MLOperatorKernelOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
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
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MLOperatorParameterOptions(pub u32);
impl MLOperatorParameterOptions {
    pub const Single: Self = Self(0u32);
    pub const Optional: Self = Self(1u32);
    pub const Variadic: Self = Self(2u32);
}
impl ::core::marker::Copy for MLOperatorParameterOptions {}
impl ::core::clone::Clone for MLOperatorParameterOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MLOperatorParameterOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MLOperatorParameterOptions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MLOperatorParameterOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MLOperatorParameterOptions").field(&self.0).finish()
    }
}
impl MLOperatorParameterOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
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
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MLOperatorSchemaEdgeTypeFormat(pub i32);
impl MLOperatorSchemaEdgeTypeFormat {
    pub const EdgeDescription: Self = Self(0i32);
    pub const Label: Self = Self(1i32);
}
impl ::core::marker::Copy for MLOperatorSchemaEdgeTypeFormat {}
impl ::core::clone::Clone for MLOperatorSchemaEdgeTypeFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MLOperatorSchemaEdgeTypeFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MLOperatorSchemaEdgeTypeFormat {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MLOperatorSchemaEdgeTypeFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MLOperatorSchemaEdgeTypeFormat").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MLOperatorTensorDataType(pub u32);
impl MLOperatorTensorDataType {
    pub const Undefined: Self = Self(0u32);
    pub const Float: Self = Self(1u32);
    pub const UInt8: Self = Self(2u32);
    pub const Int8: Self = Self(3u32);
    pub const UInt16: Self = Self(4u32);
    pub const Int16: Self = Self(5u32);
    pub const Int32: Self = Self(6u32);
    pub const Int64: Self = Self(7u32);
    pub const String: Self = Self(8u32);
    pub const Bool: Self = Self(9u32);
    pub const Float16: Self = Self(10u32);
    pub const Double: Self = Self(11u32);
    pub const UInt32: Self = Self(12u32);
    pub const UInt64: Self = Self(13u32);
    pub const Complex64: Self = Self(14u32);
    pub const Complex128: Self = Self(15u32);
}
impl ::core::marker::Copy for MLOperatorTensorDataType {}
impl ::core::clone::Clone for MLOperatorTensorDataType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MLOperatorTensorDataType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MLOperatorTensorDataType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MLOperatorTensorDataType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MLOperatorTensorDataType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINML_BINDING_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_BINDING_UNDEFINED: WINML_BINDING_TYPE = WINML_BINDING_TYPE(0i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_BINDING_TENSOR: WINML_BINDING_TYPE = WINML_BINDING_TYPE(1i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_BINDING_SEQUENCE: WINML_BINDING_TYPE = WINML_BINDING_TYPE(2i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_BINDING_MAP: WINML_BINDING_TYPE = WINML_BINDING_TYPE(3i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_BINDING_IMAGE: WINML_BINDING_TYPE = WINML_BINDING_TYPE(4i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_BINDING_RESOURCE: WINML_BINDING_TYPE = WINML_BINDING_TYPE(5i32);
impl ::core::marker::Copy for WINML_BINDING_TYPE {}
impl ::core::clone::Clone for WINML_BINDING_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINML_BINDING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WINML_BINDING_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WINML_BINDING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINML_BINDING_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINML_FEATURE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_FEATURE_UNDEFINED: WINML_FEATURE_TYPE = WINML_FEATURE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_FEATURE_TENSOR: WINML_FEATURE_TYPE = WINML_FEATURE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_FEATURE_SEQUENCE: WINML_FEATURE_TYPE = WINML_FEATURE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_FEATURE_MAP: WINML_FEATURE_TYPE = WINML_FEATURE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_FEATURE_IMAGE: WINML_FEATURE_TYPE = WINML_FEATURE_TYPE(4i32);
impl ::core::marker::Copy for WINML_FEATURE_TYPE {}
impl ::core::clone::Clone for WINML_FEATURE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINML_FEATURE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WINML_FEATURE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WINML_FEATURE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINML_FEATURE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINML_RUNTIME_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_RUNTIME_CNTK: WINML_RUNTIME_TYPE = WINML_RUNTIME_TYPE(0i32);
impl ::core::marker::Copy for WINML_RUNTIME_TYPE {}
impl ::core::clone::Clone for WINML_RUNTIME_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINML_RUNTIME_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WINML_RUNTIME_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WINML_RUNTIME_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINML_RUNTIME_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINML_TENSOR_DATA_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_TENSOR_UNDEFINED: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(0i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_TENSOR_FLOAT: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(1i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_TENSOR_UINT8: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(2i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_TENSOR_INT8: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(3i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_TENSOR_UINT16: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(4i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_TENSOR_INT16: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(5i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_TENSOR_INT32: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(6i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_TENSOR_INT64: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(7i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_TENSOR_STRING: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(8i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_TENSOR_BOOLEAN: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(9i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_TENSOR_FLOAT16: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(10i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_TENSOR_DOUBLE: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(11i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_TENSOR_UINT32: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(12i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_TENSOR_UINT64: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(13i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_TENSOR_COMPLEX64: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(14i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_TENSOR_COMPLEX128: WINML_TENSOR_DATA_TYPE = WINML_TENSOR_DATA_TYPE(15i32);
impl ::core::marker::Copy for WINML_TENSOR_DATA_TYPE {}
impl ::core::clone::Clone for WINML_TENSOR_DATA_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINML_TENSOR_DATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WINML_TENSOR_DATA_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WINML_TENSOR_DATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINML_TENSOR_DATA_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub struct MLOperatorAttribute {
    pub name: ::windows::core::PCSTR,
    pub r#type: MLOperatorAttributeType,
    pub required: u8,
}
impl ::core::marker::Copy for MLOperatorAttribute {}
impl ::core::clone::Clone for MLOperatorAttribute {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MLOperatorAttribute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MLOperatorAttribute").field("name", &self.name).field("type", &self.r#type).field("required", &self.required).finish()
    }
}
impl ::windows::core::TypeKind for MLOperatorAttribute {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MLOperatorAttribute {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.r#type == other.r#type && self.required == other.required
    }
}
impl ::core::cmp::Eq for MLOperatorAttribute {}
impl ::core::default::Default for MLOperatorAttribute {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub struct MLOperatorAttributeNameValue {
    pub name: ::windows::core::PCSTR,
    pub r#type: MLOperatorAttributeType,
    pub valueCount: u32,
    pub Anonymous: MLOperatorAttributeNameValue_0,
}
impl ::core::marker::Copy for MLOperatorAttributeNameValue {}
impl ::core::clone::Clone for MLOperatorAttributeNameValue {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MLOperatorAttributeNameValue {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MLOperatorAttributeNameValue {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub union MLOperatorAttributeNameValue_0 {
    pub reserved: *const ::core::ffi::c_void,
    pub ints: *const i64,
    pub strings: *const *const i8,
    pub floats: *const f32,
}
impl ::core::marker::Copy for MLOperatorAttributeNameValue_0 {}
impl ::core::clone::Clone for MLOperatorAttributeNameValue_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MLOperatorAttributeNameValue_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MLOperatorAttributeNameValue_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub struct MLOperatorEdgeDescription {
    pub edgeType: MLOperatorEdgeType,
    pub Anonymous: MLOperatorEdgeDescription_0,
}
impl ::core::marker::Copy for MLOperatorEdgeDescription {}
impl ::core::clone::Clone for MLOperatorEdgeDescription {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MLOperatorEdgeDescription {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MLOperatorEdgeDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub union MLOperatorEdgeDescription_0 {
    pub reserved: u64,
    pub tensorDataType: MLOperatorTensorDataType,
}
impl ::core::marker::Copy for MLOperatorEdgeDescription_0 {}
impl ::core::clone::Clone for MLOperatorEdgeDescription_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MLOperatorEdgeDescription_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MLOperatorEdgeDescription_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub struct MLOperatorEdgeTypeConstraint {
    pub typeLabel: ::windows::core::PCSTR,
    pub allowedTypes: *const MLOperatorEdgeDescription,
    pub allowedTypeCount: u32,
}
impl ::core::marker::Copy for MLOperatorEdgeTypeConstraint {}
impl ::core::clone::Clone for MLOperatorEdgeTypeConstraint {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MLOperatorEdgeTypeConstraint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MLOperatorEdgeTypeConstraint").field("typeLabel", &self.typeLabel).field("allowedTypes", &self.allowedTypes).field("allowedTypeCount", &self.allowedTypeCount).finish()
    }
}
impl ::windows::core::TypeKind for MLOperatorEdgeTypeConstraint {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MLOperatorEdgeTypeConstraint {
    fn eq(&self, other: &Self) -> bool {
        self.typeLabel == other.typeLabel && self.allowedTypes == other.allowedTypes && self.allowedTypeCount == other.allowedTypeCount
    }
}
impl ::core::cmp::Eq for MLOperatorEdgeTypeConstraint {}
impl ::core::default::Default for MLOperatorEdgeTypeConstraint {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub struct MLOperatorKernelDescription {
    pub domain: ::windows::core::PCSTR,
    pub name: ::windows::core::PCSTR,
    pub minimumOperatorSetVersion: i32,
    pub executionType: MLOperatorExecutionType,
    pub typeConstraints: *const MLOperatorEdgeTypeConstraint,
    pub typeConstraintCount: u32,
    pub defaultAttributes: *const MLOperatorAttributeNameValue,
    pub defaultAttributeCount: u32,
    pub options: MLOperatorKernelOptions,
    pub executionOptions: u32,
}
impl ::core::marker::Copy for MLOperatorKernelDescription {}
impl ::core::clone::Clone for MLOperatorKernelDescription {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::windows::core::TypeKind for MLOperatorKernelDescription {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MLOperatorKernelDescription {
    fn eq(&self, other: &Self) -> bool {
        self.domain == other.domain && self.name == other.name && self.minimumOperatorSetVersion == other.minimumOperatorSetVersion && self.executionType == other.executionType && self.typeConstraints == other.typeConstraints && self.typeConstraintCount == other.typeConstraintCount && self.defaultAttributes == other.defaultAttributes && self.defaultAttributeCount == other.defaultAttributeCount && self.options == other.options && self.executionOptions == other.executionOptions
    }
}
impl ::core::cmp::Eq for MLOperatorKernelDescription {}
impl ::core::default::Default for MLOperatorKernelDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub struct MLOperatorSchemaDescription {
    pub name: ::windows::core::PCSTR,
    pub operatorSetVersionAtLastChange: i32,
    pub inputs: *const MLOperatorSchemaEdgeDescription,
    pub inputCount: u32,
    pub outputs: *const MLOperatorSchemaEdgeDescription,
    pub outputCount: u32,
    pub typeConstraints: *const MLOperatorEdgeTypeConstraint,
    pub typeConstraintCount: u32,
    pub attributes: *const MLOperatorAttribute,
    pub attributeCount: u32,
    pub defaultAttributes: *const MLOperatorAttributeNameValue,
    pub defaultAttributeCount: u32,
}
impl ::core::marker::Copy for MLOperatorSchemaDescription {}
impl ::core::clone::Clone for MLOperatorSchemaDescription {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::windows::core::TypeKind for MLOperatorSchemaDescription {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MLOperatorSchemaDescription {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.operatorSetVersionAtLastChange == other.operatorSetVersionAtLastChange && self.inputs == other.inputs && self.inputCount == other.inputCount && self.outputs == other.outputs && self.outputCount == other.outputCount && self.typeConstraints == other.typeConstraints && self.typeConstraintCount == other.typeConstraintCount && self.attributes == other.attributes && self.attributeCount == other.attributeCount && self.defaultAttributes == other.defaultAttributes && self.defaultAttributeCount == other.defaultAttributeCount
    }
}
impl ::core::cmp::Eq for MLOperatorSchemaDescription {}
impl ::core::default::Default for MLOperatorSchemaDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub struct MLOperatorSchemaEdgeDescription {
    pub options: MLOperatorParameterOptions,
    pub typeFormat: MLOperatorSchemaEdgeTypeFormat,
    pub Anonymous: MLOperatorSchemaEdgeDescription_0,
}
impl ::core::marker::Copy for MLOperatorSchemaEdgeDescription {}
impl ::core::clone::Clone for MLOperatorSchemaEdgeDescription {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MLOperatorSchemaEdgeDescription {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MLOperatorSchemaEdgeDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub union MLOperatorSchemaEdgeDescription_0 {
    pub reserved: *const ::core::ffi::c_void,
    pub typeLabel: ::windows::core::PCSTR,
    pub edgeDescription: MLOperatorEdgeDescription,
}
impl ::core::marker::Copy for MLOperatorSchemaEdgeDescription_0 {}
impl ::core::clone::Clone for MLOperatorSchemaEdgeDescription_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MLOperatorSchemaEdgeDescription_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MLOperatorSchemaEdgeDescription_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub struct MLOperatorSetId {
    pub domain: ::windows::core::PCSTR,
    pub version: i32,
}
impl ::core::marker::Copy for MLOperatorSetId {}
impl ::core::clone::Clone for MLOperatorSetId {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MLOperatorSetId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MLOperatorSetId").field("domain", &self.domain).field("version", &self.version).finish()
    }
}
impl ::windows::core::TypeKind for MLOperatorSetId {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MLOperatorSetId {
    fn eq(&self, other: &Self) -> bool {
        self.domain == other.domain && self.version == other.version
    }
}
impl ::core::cmp::Eq for MLOperatorSetId {}
impl ::core::default::Default for MLOperatorSetId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`, `\"Win32_Graphics_Direct3D12\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub struct WINML_BINDING_DESC {
    pub Name: ::windows::core::PCWSTR,
    pub BindType: WINML_BINDING_TYPE,
    pub Anonymous: WINML_BINDING_DESC_0,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::clone::Clone for WINML_BINDING_DESC {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::windows::core::TypeKind for WINML_BINDING_DESC {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::default::Default for WINML_BINDING_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`, `\"Win32_Graphics_Direct3D12\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub union WINML_BINDING_DESC_0 {
    pub Tensor: WINML_TENSOR_BINDING_DESC,
    pub Sequence: WINML_SEQUENCE_BINDING_DESC,
    pub Map: WINML_MAP_BINDING_DESC,
    pub Image: WINML_IMAGE_BINDING_DESC,
    pub Resource: ::std::mem::ManuallyDrop<WINML_RESOURCE_BINDING_DESC>,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::clone::Clone for WINML_BINDING_DESC_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::windows::core::TypeKind for WINML_BINDING_DESC_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::default::Default for WINML_BINDING_DESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub struct WINML_IMAGE_BINDING_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
    pub DataSize: u32,
    pub pData: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WINML_IMAGE_BINDING_DESC {}
impl ::core::clone::Clone for WINML_IMAGE_BINDING_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINML_IMAGE_BINDING_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINML_IMAGE_BINDING_DESC").field("ElementType", &self.ElementType).field("NumDimensions", &self.NumDimensions).field("pShape", &self.pShape).field("DataSize", &self.DataSize).field("pData", &self.pData).finish()
    }
}
impl ::windows::core::TypeKind for WINML_IMAGE_BINDING_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINML_IMAGE_BINDING_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ElementType == other.ElementType && self.NumDimensions == other.NumDimensions && self.pShape == other.pShape && self.DataSize == other.DataSize && self.pData == other.pData
    }
}
impl ::core::cmp::Eq for WINML_IMAGE_BINDING_DESC {}
impl ::core::default::Default for WINML_IMAGE_BINDING_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub struct WINML_IMAGE_VARIABLE_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
}
impl ::core::marker::Copy for WINML_IMAGE_VARIABLE_DESC {}
impl ::core::clone::Clone for WINML_IMAGE_VARIABLE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINML_IMAGE_VARIABLE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINML_IMAGE_VARIABLE_DESC").field("ElementType", &self.ElementType).field("NumDimensions", &self.NumDimensions).field("pShape", &self.pShape).finish()
    }
}
impl ::windows::core::TypeKind for WINML_IMAGE_VARIABLE_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINML_IMAGE_VARIABLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ElementType == other.ElementType && self.NumDimensions == other.NumDimensions && self.pShape == other.pShape
    }
}
impl ::core::cmp::Eq for WINML_IMAGE_VARIABLE_DESC {}
impl ::core::default::Default for WINML_IMAGE_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub struct WINML_MAP_BINDING_DESC {
    pub ElementCount: u32,
    pub KeyType: WINML_TENSOR_DATA_TYPE,
    pub Anonymous1: WINML_MAP_BINDING_DESC_0,
    pub Fields: WINML_TENSOR_DATA_TYPE,
    pub Anonymous2: WINML_MAP_BINDING_DESC_1,
}
impl ::core::marker::Copy for WINML_MAP_BINDING_DESC {}
impl ::core::clone::Clone for WINML_MAP_BINDING_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WINML_MAP_BINDING_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WINML_MAP_BINDING_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub union WINML_MAP_BINDING_DESC_0 {
    pub pStringKeys: *mut ::windows::core::PWSTR,
    pub pIntKeys: *mut i64,
}
impl ::core::marker::Copy for WINML_MAP_BINDING_DESC_0 {}
impl ::core::clone::Clone for WINML_MAP_BINDING_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WINML_MAP_BINDING_DESC_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WINML_MAP_BINDING_DESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub union WINML_MAP_BINDING_DESC_1 {
    pub pStringFields: *mut ::windows::core::PWSTR,
    pub pIntFields: *mut i64,
    pub pFloatFields: *mut f32,
    pub pDoubleFields: *mut f64,
}
impl ::core::marker::Copy for WINML_MAP_BINDING_DESC_1 {}
impl ::core::clone::Clone for WINML_MAP_BINDING_DESC_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WINML_MAP_BINDING_DESC_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WINML_MAP_BINDING_DESC_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub struct WINML_MAP_VARIABLE_DESC {
    pub KeyType: WINML_TENSOR_DATA_TYPE,
    pub Fields: WINML_TENSOR_DATA_TYPE,
}
impl ::core::marker::Copy for WINML_MAP_VARIABLE_DESC {}
impl ::core::clone::Clone for WINML_MAP_VARIABLE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINML_MAP_VARIABLE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINML_MAP_VARIABLE_DESC").field("KeyType", &self.KeyType).field("Fields", &self.Fields).finish()
    }
}
impl ::windows::core::TypeKind for WINML_MAP_VARIABLE_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINML_MAP_VARIABLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.KeyType == other.KeyType && self.Fields == other.Fields
    }
}
impl ::core::cmp::Eq for WINML_MAP_VARIABLE_DESC {}
impl ::core::default::Default for WINML_MAP_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub struct WINML_MODEL_DESC {
    pub Author: ::windows::core::PWSTR,
    pub Name: ::windows::core::PWSTR,
    pub Domain: ::windows::core::PWSTR,
    pub Description: ::windows::core::PWSTR,
    pub Version: usize,
}
impl ::core::marker::Copy for WINML_MODEL_DESC {}
impl ::core::clone::Clone for WINML_MODEL_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINML_MODEL_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINML_MODEL_DESC").field("Author", &self.Author).field("Name", &self.Name).field("Domain", &self.Domain).field("Description", &self.Description).field("Version", &self.Version).finish()
    }
}
impl ::windows::core::TypeKind for WINML_MODEL_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINML_MODEL_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Author == other.Author && self.Name == other.Name && self.Domain == other.Domain && self.Description == other.Description && self.Version == other.Version
    }
}
impl ::core::cmp::Eq for WINML_MODEL_DESC {}
impl ::core::default::Default for WINML_MODEL_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`, `\"Win32_Graphics_Direct3D12\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub struct WINML_RESOURCE_BINDING_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
    pub pResource: ::std::mem::ManuallyDrop<::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Resource>>,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::clone::Clone for WINML_RESOURCE_BINDING_DESC {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for WINML_RESOURCE_BINDING_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINML_RESOURCE_BINDING_DESC").field("ElementType", &self.ElementType).field("NumDimensions", &self.NumDimensions).field("pShape", &self.pShape).field("pResource", &self.pResource).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::windows::core::TypeKind for WINML_RESOURCE_BINDING_DESC {
    type TypeKind = ::windows::core::CopyType;
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
impl ::core::default::Default for WINML_RESOURCE_BINDING_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub struct WINML_SEQUENCE_BINDING_DESC {
    pub ElementCount: u32,
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub Anonymous: WINML_SEQUENCE_BINDING_DESC_0,
}
impl ::core::marker::Copy for WINML_SEQUENCE_BINDING_DESC {}
impl ::core::clone::Clone for WINML_SEQUENCE_BINDING_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WINML_SEQUENCE_BINDING_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WINML_SEQUENCE_BINDING_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub union WINML_SEQUENCE_BINDING_DESC_0 {
    pub pStrings: *mut ::windows::core::PWSTR,
    pub pInts: *mut i64,
    pub pFloats: *mut f32,
    pub pDoubles: *mut f64,
}
impl ::core::marker::Copy for WINML_SEQUENCE_BINDING_DESC_0 {}
impl ::core::clone::Clone for WINML_SEQUENCE_BINDING_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WINML_SEQUENCE_BINDING_DESC_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WINML_SEQUENCE_BINDING_DESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub struct WINML_SEQUENCE_VARIABLE_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
}
impl ::core::marker::Copy for WINML_SEQUENCE_VARIABLE_DESC {}
impl ::core::clone::Clone for WINML_SEQUENCE_VARIABLE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINML_SEQUENCE_VARIABLE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINML_SEQUENCE_VARIABLE_DESC").field("ElementType", &self.ElementType).finish()
    }
}
impl ::windows::core::TypeKind for WINML_SEQUENCE_VARIABLE_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINML_SEQUENCE_VARIABLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ElementType == other.ElementType
    }
}
impl ::core::cmp::Eq for WINML_SEQUENCE_VARIABLE_DESC {}
impl ::core::default::Default for WINML_SEQUENCE_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub struct WINML_TENSOR_BINDING_DESC {
    pub DataType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
    pub DataSize: u32,
    pub pData: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WINML_TENSOR_BINDING_DESC {}
impl ::core::clone::Clone for WINML_TENSOR_BINDING_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINML_TENSOR_BINDING_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINML_TENSOR_BINDING_DESC").field("DataType", &self.DataType).field("NumDimensions", &self.NumDimensions).field("pShape", &self.pShape).field("DataSize", &self.DataSize).field("pData", &self.pData).finish()
    }
}
impl ::windows::core::TypeKind for WINML_TENSOR_BINDING_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINML_TENSOR_BINDING_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.DataType == other.DataType && self.NumDimensions == other.NumDimensions && self.pShape == other.pShape && self.DataSize == other.DataSize && self.pData == other.pData
    }
}
impl ::core::cmp::Eq for WINML_TENSOR_BINDING_DESC {}
impl ::core::default::Default for WINML_TENSOR_BINDING_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub struct WINML_TENSOR_VARIABLE_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
}
impl ::core::marker::Copy for WINML_TENSOR_VARIABLE_DESC {}
impl ::core::clone::Clone for WINML_TENSOR_VARIABLE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINML_TENSOR_VARIABLE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINML_TENSOR_VARIABLE_DESC").field("ElementType", &self.ElementType).field("NumDimensions", &self.NumDimensions).field("pShape", &self.pShape).finish()
    }
}
impl ::windows::core::TypeKind for WINML_TENSOR_VARIABLE_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINML_TENSOR_VARIABLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ElementType == other.ElementType && self.NumDimensions == other.NumDimensions && self.pShape == other.pShape
    }
}
impl ::core::cmp::Eq for WINML_TENSOR_VARIABLE_DESC {}
impl ::core::default::Default for WINML_TENSOR_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINML_VARIABLE_DESC {
    pub Name: ::windows::core::PWSTR,
    pub Description: ::windows::core::PWSTR,
    pub FeatureType: WINML_FEATURE_TYPE,
    pub Required: super::super::super::Foundation::BOOL,
    pub Anonymous: WINML_VARIABLE_DESC_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINML_VARIABLE_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINML_VARIABLE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINML_VARIABLE_DESC {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINML_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union WINML_VARIABLE_DESC_0 {
    pub Tensor: WINML_TENSOR_VARIABLE_DESC,
    pub Sequence: WINML_SEQUENCE_VARIABLE_DESC,
    pub Map: WINML_MAP_VARIABLE_DESC,
    pub Image: WINML_IMAGE_VARIABLE_DESC,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINML_VARIABLE_DESC_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINML_VARIABLE_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WINML_VARIABLE_DESC_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINML_VARIABLE_DESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
