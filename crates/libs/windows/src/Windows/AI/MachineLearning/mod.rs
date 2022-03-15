#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "AI_MachineLearning_Preview")]
pub mod Preview;
#[doc(hidden)]
#[repr(transparent)]
pub struct IImageFeatureDescriptor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IImageFeatureDescriptor {
    type Vtable = IImageFeatureDescriptor_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x365585a5_171a_4a2a_985f_265159d3895a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageFeatureDescriptor_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Graphics_Imaging")]
    pub BitmapPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    BitmapPixelFormat: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub BitmapAlphaMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapAlphaMode) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    BitmapAlphaMode: usize,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IImageFeatureDescriptor2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IImageFeatureDescriptor2 {
    type Vtable = IImageFeatureDescriptor2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b27cca7_d533_5862_bb98_1611b155b0e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageFeatureDescriptor2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PixelRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LearningModelPixelRange) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IImageFeatureValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IImageFeatureValue {
    type Vtable = IImageFeatureValue_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0414fd9_c9aa_4405_b7fb_94f87c8a3037);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageFeatureValue_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Media")]
    pub VideoFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media"))]
    VideoFrame: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IImageFeatureValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IImageFeatureValueStatics {
    type Vtable = IImageFeatureValueStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bc317fd_23cb_4610_b085_c8e1c87ebaa0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageFeatureValueStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Media")]
    pub CreateFromVideoFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media"))]
    CreateFromVideoFrame: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILearningModel(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILearningModel {
    type Vtable = ILearningModel_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b8e4920_489f_4e86_9128_265a327b78fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModel_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Author: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Domain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Metadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Metadata: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub InputFeatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InputFeatures: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub OutputFeatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    OutputFeatures: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILearningModelBinding(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILearningModelBinding {
    type Vtable = ILearningModelBinding_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea312f20_168f_4f8c_94fe_2e7ac31b4aa8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelBinding_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Bind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub BindWithProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: *mut ::core::ffi::c_void, props: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BindWithProperties: usize,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILearningModelBindingFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILearningModelBindingFactory {
    type Vtable = ILearningModelBindingFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc95f7a7a_e788_475e_8917_23aa381faf0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelBindingFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateFromSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, session: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILearningModelDevice(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILearningModelDevice {
    type Vtable = ILearningModelDevice_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5c2c8fe_3f56_4a8c_ac5f_fdb92d8b8252);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelDevice_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Graphics")]
    pub AdapterId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::DisplayAdapterId) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    AdapterId: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Direct3D11Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Direct3D11Device: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILearningModelDeviceFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILearningModelDeviceFactory {
    type Vtable = ILearningModelDeviceFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9cffd74d_b1e5_4f20_80ad_0a56690db06b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelDeviceFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, devicekind: LearningModelDeviceKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILearningModelDeviceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILearningModelDeviceStatics {
    type Vtable = ILearningModelDeviceStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49f32107_a8bf_42bb_92c7_10b12dc5d21f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelDeviceStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CreateFromDirect3D11Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, device: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CreateFromDirect3D11Device: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILearningModelEvaluationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILearningModelEvaluationResult {
    type Vtable = ILearningModelEvaluationResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2f9bfcd_960e_49c0_8593_eb190ae3eee2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelEvaluationResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ErrorStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub Succeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Outputs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Outputs: usize,
}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct ILearningModelFeatureDescriptor(::windows::core::IUnknown);
impl ILearningModelFeatureDescriptor {
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Description)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = self;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn IsRequired(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsRequired)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::convert::From<ILearningModelFeatureDescriptor> for ::windows::core::IUnknown {
    fn from(value: ILearningModelFeatureDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILearningModelFeatureDescriptor> for ::windows::core::IUnknown {
    fn from(value: &ILearningModelFeatureDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILearningModelFeatureDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ILearningModelFeatureDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ILearningModelFeatureDescriptor> for ::windows::core::IInspectable {
    fn from(value: ILearningModelFeatureDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILearningModelFeatureDescriptor> for ::windows::core::IInspectable {
    fn from(value: &ILearningModelFeatureDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ILearningModelFeatureDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ILearningModelFeatureDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ILearningModelFeatureDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILearningModelFeatureDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILearningModelFeatureDescriptor {}
impl ::core::fmt::Debug for ILearningModelFeatureDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILearningModelFeatureDescriptor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ILearningModelFeatureDescriptor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{bc08cf7c-6ed0-4004-97ba-b9a2eecd2b4f}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ILearningModelFeatureDescriptor {
    type Vtable = ILearningModelFeatureDescriptor_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc08cf7c_6ed0_4004_97ba_b9a2eecd2b4f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelFeatureDescriptor_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LearningModelFeatureKind) -> ::windows::core::HRESULT,
    pub IsRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct ILearningModelFeatureValue(::windows::core::IUnknown);
impl ILearningModelFeatureValue {
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = self;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
}
impl ::core::convert::From<ILearningModelFeatureValue> for ::windows::core::IUnknown {
    fn from(value: ILearningModelFeatureValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILearningModelFeatureValue> for ::windows::core::IUnknown {
    fn from(value: &ILearningModelFeatureValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILearningModelFeatureValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ILearningModelFeatureValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ILearningModelFeatureValue> for ::windows::core::IInspectable {
    fn from(value: ILearningModelFeatureValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILearningModelFeatureValue> for ::windows::core::IInspectable {
    fn from(value: &ILearningModelFeatureValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ILearningModelFeatureValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ILearningModelFeatureValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ILearningModelFeatureValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILearningModelFeatureValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILearningModelFeatureValue {}
impl ::core::fmt::Debug for ILearningModelFeatureValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILearningModelFeatureValue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ILearningModelFeatureValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{f51005db-4085-4dfe-9fed-95eb0c0cf75c}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ILearningModelFeatureValue {
    type Vtable = ILearningModelFeatureValue_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf51005db_4085_4dfe_9fed_95eb0c0cf75c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelFeatureValue_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LearningModelFeatureKind) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct ILearningModelOperatorProvider(::windows::core::IUnknown);
impl ILearningModelOperatorProvider {}
impl ::core::convert::From<ILearningModelOperatorProvider> for ::windows::core::IUnknown {
    fn from(value: ILearningModelOperatorProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILearningModelOperatorProvider> for ::windows::core::IUnknown {
    fn from(value: &ILearningModelOperatorProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILearningModelOperatorProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ILearningModelOperatorProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ILearningModelOperatorProvider> for ::windows::core::IInspectable {
    fn from(value: ILearningModelOperatorProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILearningModelOperatorProvider> for ::windows::core::IInspectable {
    fn from(value: &ILearningModelOperatorProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ILearningModelOperatorProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ILearningModelOperatorProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ILearningModelOperatorProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILearningModelOperatorProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILearningModelOperatorProvider {}
impl ::core::fmt::Debug for ILearningModelOperatorProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILearningModelOperatorProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ILearningModelOperatorProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{2a222e5d-afb1-47ed-bfad-b5b3a459ec04}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ILearningModelOperatorProvider {
    type Vtable = ILearningModelOperatorProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a222e5d_afb1_47ed_bfad_b5b3a459ec04);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelOperatorProvider_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILearningModelSession(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILearningModelSession {
    type Vtable = ILearningModelSession_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8e58f8f6_b787_4c11_90f0_7129aeca74a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSession_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Model: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub EvaluationProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EvaluationProperties: usize,
    #[cfg(feature = "Foundation")]
    pub EvaluateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bindings: ::windows::core::RawPtr, correlationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EvaluateAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub EvaluateFeaturesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, features: ::windows::core::RawPtr, correlationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EvaluateFeaturesAsync: usize,
    pub Evaluate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bindings: ::windows::core::RawPtr, correlationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub EvaluateFeatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, features: ::windows::core::RawPtr, correlationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EvaluateFeatures: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILearningModelSessionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILearningModelSessionFactory {
    type Vtable = ILearningModelSessionFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f6b881d_1c9b_47b6_bfe0_f1cf62a67579);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSessionFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateFromModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, model: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateFromModelOnDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, model: ::windows::core::RawPtr, devicetorunon: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILearningModelSessionFactory2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILearningModelSessionFactory2 {
    type Vtable = ILearningModelSessionFactory2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e5c88bf_0a1f_5fec_ade0_2fd91e4ef29b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSessionFactory2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateFromModelOnDeviceWithSessionOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, model: ::windows::core::RawPtr, devicetorunon: ::windows::core::RawPtr, learningmodelsessionoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILearningModelSessionOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILearningModelSessionOptions {
    type Vtable = ILearningModelSessionOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8f63fa1_134d_5133_8cff_3a5c3c263beb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSessionOptions_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub BatchSizeOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetBatchSizeOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILearningModelSessionOptions2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILearningModelSessionOptions2 {
    type Vtable = ILearningModelSessionOptions2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fcd1dc4_175f_5bd2_8de5_2f2006a25adf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSessionOptions2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CloseModelOnSessionCreation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCloseModelOnSessionCreation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILearningModelSessionOptions3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILearningModelSessionOptions3 {
    type Vtable = ILearningModelSessionOptions3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58e15cee_d8c2_56fc_92e8_76d751081086);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSessionOptions3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub OverrideNamedDimension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, dimension: u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILearningModelStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILearningModelStatics {
    type Vtable = ILearningModelStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3b977e8_6952_4e47_8ef4_1f7f07897c6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LoadFromStorageFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modelfile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LoadFromStorageFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub LoadFromStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modelstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    LoadFromStreamAsync: usize,
    pub LoadFromFilePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub LoadFromStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modelstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LoadFromStream: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LoadFromStorageFileWithOperatorProviderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modelfile: ::windows::core::RawPtr, operatorprovider: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LoadFromStorageFileWithOperatorProviderAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub LoadFromStreamWithOperatorProviderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modelstream: ::windows::core::RawPtr, operatorprovider: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    LoadFromStreamWithOperatorProviderAsync: usize,
    pub LoadFromFilePathWithOperatorProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, operatorprovider: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub LoadFromStreamWithOperatorProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modelstream: ::windows::core::RawPtr, operatorprovider: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LoadFromStreamWithOperatorProvider: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapFeatureDescriptor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapFeatureDescriptor {
    type Vtable = IMapFeatureDescriptor_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x530424bd_a257_436d_9e60_c2981f7cc5c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapFeatureDescriptor_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub KeyKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TensorKind) -> ::windows::core::HRESULT,
    pub ValueDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISequenceFeatureDescriptor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISequenceFeatureDescriptor {
    type Vtable = ISequenceFeatureDescriptor_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84f6945a_562b_4d62_a851_739aced96668);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISequenceFeatureDescriptor_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ElementDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct ITensor(::windows::core::IUnknown);
impl ITensor {
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn TensorKind(&self) -> ::windows::core::Result<TensorKind> {
        let this = self;
        unsafe {
            let mut result__: TensorKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TensorKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TensorKind>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Shape)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i64>>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
}
impl ::core::convert::From<ITensor> for ::windows::core::IUnknown {
    fn from(value: ITensor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITensor> for ::windows::core::IUnknown {
    fn from(value: &ITensor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITensor> for ::windows::core::IInspectable {
    fn from(value: ITensor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITensor> for ::windows::core::IInspectable {
    fn from(value: &ITensor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ITensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ITensor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ITensor> for ILearningModelFeatureValue {
    type Error = ::windows::core::Error;
    fn try_from(value: ITensor) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ITensor> for ILearningModelFeatureValue {
    type Error = ::windows::core::Error;
    fn try_from(value: &ITensor) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureValue> for ITensor {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureValue> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureValue> for &ITensor {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureValue> {
        ::core::convert::TryInto::<ILearningModelFeatureValue>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for ITensor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITensor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITensor {}
impl ::core::fmt::Debug for ITensor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITensor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ITensor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{05489593-a305-4a25-ad09-440119b4b7f6}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ITensor {
    type Vtable = ITensor_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05489593_a305_4a25_ad09_440119b4b7f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensor_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TensorKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TensorKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Shape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Shape: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorBoolean(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorBoolean {
    type Vtable = ITensorBoolean_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50f311ed_29e9_4a5c_a44d_8fc512584eed);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorBoolean_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorBooleanStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorBooleanStatics {
    type Vtable = ITensorBooleanStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2796862c_2357_49a7_b476_d0aa3dfe6866);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorBooleanStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data_array_size: u32, data: *const bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorBooleanStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorBooleanStatics2 {
    type Vtable = ITensorBooleanStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3a4a501_6a2d_52d7_b04b_c435baee0115);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorBooleanStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorDouble(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorDouble {
    type Vtable = ITensorDouble_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91e41252_7a8f_4f0e_a28f_9637ffc8a3d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorDouble_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorDoubleStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorDoubleStatics {
    type Vtable = ITensorDoubleStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa86693c5_9538_44e7_a3ca_5df374a5a70c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorDoubleStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data_array_size: u32, data: *const f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorDoubleStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorDoubleStatics2 {
    type Vtable = ITensorDoubleStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93a570de_5e9a_5094_85c8_592c655e68ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorDoubleStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorFeatureDescriptor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorFeatureDescriptor {
    type Vtable = ITensorFeatureDescriptor_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74455c80_946a_4310_a19c_ee0af028fce4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorFeatureDescriptor_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TensorKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TensorKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Shape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Shape: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorFloat(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorFloat {
    type Vtable = ITensorFloat_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2282d82_aa02_42c8_a0c8_df1efc9676e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorFloat_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorFloat16Bit(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorFloat16Bit {
    type Vtable = ITensorFloat16Bit_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ab994fc_5b89_4c3c_b5e4_5282a5316c0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorFloat16Bit_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorFloat16BitStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorFloat16BitStatics {
    type Vtable = ITensorFloat16BitStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa52db6f5_318a_44d4_820b_0cdc7054a84a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorFloat16BitStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data_array_size: u32, data: *const f32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorFloat16BitStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorFloat16BitStatics2 {
    type Vtable = ITensorFloat16BitStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68545726_2dc7_51bf_b470_0b344cc2a1bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorFloat16BitStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const f32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorFloatStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorFloatStatics {
    type Vtable = ITensorFloatStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdbcd395b_3ba3_452f_b10d_3c135e573fa9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorFloatStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data_array_size: u32, data: *const f32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorFloatStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorFloatStatics2 {
    type Vtable = ITensorFloatStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24610bc1_5e44_5713_b281_8f4ad4d555e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorFloatStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const f32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorInt16Bit(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorInt16Bit {
    type Vtable = ITensorInt16Bit_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98a32d39_e6d6_44af_8afa_baebc44dc020);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt16Bit_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorInt16BitStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorInt16BitStatics {
    type Vtable = ITensorInt16BitStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98646293_266e_4b1a_821f_e60d70898b91);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt16BitStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data_array_size: u32, data: *const i16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorInt16BitStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorInt16BitStatics2 {
    type Vtable = ITensorInt16BitStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cd70cf4_696c_5e5f_95d8_5ebf9670148b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt16BitStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const i16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorInt32Bit(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorInt32Bit {
    type Vtable = ITensorInt32Bit_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c0c28d3_207c_4486_a7d2_884522c5e589);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt32Bit_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorInt32BitStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorInt32BitStatics {
    type Vtable = ITensorInt32BitStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6539864b_52fa_4e35_907c_834cac417b50);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt32BitStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data_array_size: u32, data: *const i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorInt32BitStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorInt32BitStatics2 {
    type Vtable = ITensorInt32BitStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c4b079a_e956_5ce0_a3bd_157d9d79b5ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt32BitStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorInt64Bit(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorInt64Bit {
    type Vtable = ITensorInt64Bit_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x499665ba_1fa2_45ad_af25_a0bd9bda4c87);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt64Bit_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorInt64BitStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorInt64BitStatics {
    type Vtable = ITensorInt64BitStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9648ad9d_1198_4d74_9517_783ab62b9cc2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt64BitStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data_array_size: u32, data: *const i64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorInt64BitStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorInt64BitStatics2 {
    type Vtable = ITensorInt64BitStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d3d9dcb_ff40_5ec2_89fe_084e2b6bc6db);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt64BitStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const i64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorInt8Bit(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorInt8Bit {
    type Vtable = ITensorInt8Bit_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcddd97c5_ffd8_4fef_aefb_30e1a485b2ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt8Bit_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorInt8BitStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorInt8BitStatics {
    type Vtable = ITensorInt8BitStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1a12284_095c_4c76_a661_ac4cee1f3e8b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt8BitStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data_array_size: u32, data: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorInt8BitStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorInt8BitStatics2 {
    type Vtable = ITensorInt8BitStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0d59637_c468_56fb_9535_c052bdb93dc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt8BitStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorString(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorString {
    type Vtable = ITensorString_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x582335c8_bdb1_4610_bc75_35e9cbf009b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorString_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorStringStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorStringStatics {
    type Vtable = ITensorStringStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83623324_cf26_4f17_a2d4_20ef8d097d53);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorStringStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data_array_size: u32, data: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorStringStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorStringStatics2 {
    type Vtable = ITensorStringStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e355ed0_c8e2_5254_9137_0193a3668fd8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorStringStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorUInt16Bit(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorUInt16Bit {
    type Vtable = ITensorUInt16Bit_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68140f4b_23c0_42f3_81f6_a891c011bc3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt16Bit_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorUInt16BitStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorUInt16BitStatics {
    type Vtable = ITensorUInt16BitStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5df745dd_028a_481a_a27c_c7e6435e52dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt16BitStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data_array_size: u32, data: *const u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorUInt16BitStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorUInt16BitStatics2 {
    type Vtable = ITensorUInt16BitStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8af40c64_d69f_5315_9348_490877bbd642);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt16BitStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorUInt32Bit(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorUInt32Bit {
    type Vtable = ITensorUInt32Bit_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8c9c2ff_7511_45a3_bfac_c38f370d2237);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt32Bit_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorUInt32BitStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorUInt32BitStatics {
    type Vtable = ITensorUInt32BitStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x417c3837_e773_4378_8e7f_0cc33dbea697);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt32BitStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data_array_size: u32, data: *const u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorUInt32BitStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorUInt32BitStatics2 {
    type Vtable = ITensorUInt32BitStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef1a1f1c_314e_569d_b496_5c8447d20cd2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt32BitStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorUInt64Bit(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorUInt64Bit {
    type Vtable = ITensorUInt64Bit_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e70ffad_04bf_4825_839a_82baef8c7886);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt64Bit_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorUInt64BitStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorUInt64BitStatics {
    type Vtable = ITensorUInt64BitStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a7e20eb_242f_47cb_a9c6_f602ecfbfee4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt64BitStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data_array_size: u32, data: *const u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorUInt64BitStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorUInt64BitStatics2 {
    type Vtable = ITensorUInt64BitStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x085a687d_67e1_5b1e_b232_4fabe9ca20b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt64BitStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorUInt8Bit(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorUInt8Bit {
    type Vtable = ITensorUInt8Bit_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58e1ae27_622b_48e3_be22_d867aed1daac);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt8Bit_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorUInt8BitStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorUInt8BitStatics {
    type Vtable = ITensorUInt8BitStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05f67583_bc24_4220_8a41_2dcd8c5ed33c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt8BitStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data_array_size: u32, data: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorUInt8BitStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorUInt8BitStatics2 {
    type Vtable = ITensorUInt8BitStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ba042d6_373e_5a3a_a2fc_a6c41bd52789);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt8BitStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct ImageFeatureDescriptor(::windows::core::IUnknown);
impl ImageFeatureDescriptor {
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn BitmapPixelFormat(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapPixelFormat> {
        let this = self;
        unsafe {
            let mut result__: super::super::Graphics::Imaging::BitmapPixelFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BitmapPixelFormat)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Imaging::BitmapPixelFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn BitmapAlphaMode(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapAlphaMode> {
        let this = self;
        unsafe {
            let mut result__: super::super::Graphics::Imaging::BitmapAlphaMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BitmapAlphaMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Imaging::BitmapAlphaMode>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Width(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Width)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Height(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Height)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn PixelRange(&self) -> ::windows::core::Result<LearningModelPixelRange> {
        let this = &::windows::core::Interface::cast::<IImageFeatureDescriptor2>(self)?;
        unsafe {
            let mut result__: LearningModelPixelRange = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PixelRange)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelPixelRange>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Description)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn IsRequired(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsRequired)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for ImageFeatureDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ImageFeatureDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ImageFeatureDescriptor {}
impl ::core::fmt::Debug for ImageFeatureDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageFeatureDescriptor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ImageFeatureDescriptor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.ImageFeatureDescriptor;{365585a5-171a-4a2a-985f-265159d3895a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ImageFeatureDescriptor {
    type Vtable = IImageFeatureDescriptor_Vtbl;
    const IID: ::windows::core::GUID = <IImageFeatureDescriptor as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ImageFeatureDescriptor {
    const NAME: &'static str = "Windows.AI.MachineLearning.ImageFeatureDescriptor";
}
impl ::core::convert::From<ImageFeatureDescriptor> for ::windows::core::IUnknown {
    fn from(value: ImageFeatureDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ImageFeatureDescriptor> for ::windows::core::IUnknown {
    fn from(value: &ImageFeatureDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ImageFeatureDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ImageFeatureDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ImageFeatureDescriptor> for ::windows::core::IInspectable {
    fn from(value: ImageFeatureDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ImageFeatureDescriptor> for ::windows::core::IInspectable {
    fn from(value: &ImageFeatureDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ImageFeatureDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ImageFeatureDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ImageFeatureDescriptor> for ILearningModelFeatureDescriptor {
    type Error = ::windows::core::Error;
    fn try_from(value: ImageFeatureDescriptor) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ImageFeatureDescriptor> for ILearningModelFeatureDescriptor {
    type Error = ::windows::core::Error;
    fn try_from(value: &ImageFeatureDescriptor) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureDescriptor> for ImageFeatureDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureDescriptor> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureDescriptor> for &ImageFeatureDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureDescriptor> {
        ::core::convert::TryInto::<ILearningModelFeatureDescriptor>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ImageFeatureDescriptor {}
unsafe impl ::core::marker::Sync for ImageFeatureDescriptor {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct ImageFeatureValue(::windows::core::IUnknown);
impl ImageFeatureValue {
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Media\"`*"]
    #[cfg(feature = "Media")]
    pub fn VideoFrame(&self) -> ::windows::core::Result<super::super::Media::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoFrame)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::VideoFrame>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Media\"`*"]
    #[cfg(feature = "Media")]
    pub fn CreateFromVideoFrame<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::VideoFrame>>(image: Param0) -> ::windows::core::Result<ImageFeatureValue> {
        Self::IImageFeatureValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromVideoFrame)(::core::mem::transmute_copy(this), image.into_param().abi(), &mut result__).from_abi::<ImageFeatureValue>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IImageFeatureValueStatics<R, F: FnOnce(&IImageFeatureValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ImageFeatureValue, IImageFeatureValueStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ImageFeatureValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ImageFeatureValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ImageFeatureValue {}
impl ::core::fmt::Debug for ImageFeatureValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageFeatureValue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ImageFeatureValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.ImageFeatureValue;{f0414fd9-c9aa-4405-b7fb-94f87c8a3037})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ImageFeatureValue {
    type Vtable = IImageFeatureValue_Vtbl;
    const IID: ::windows::core::GUID = <IImageFeatureValue as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ImageFeatureValue {
    const NAME: &'static str = "Windows.AI.MachineLearning.ImageFeatureValue";
}
impl ::core::convert::From<ImageFeatureValue> for ::windows::core::IUnknown {
    fn from(value: ImageFeatureValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ImageFeatureValue> for ::windows::core::IUnknown {
    fn from(value: &ImageFeatureValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ImageFeatureValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ImageFeatureValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ImageFeatureValue> for ::windows::core::IInspectable {
    fn from(value: ImageFeatureValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ImageFeatureValue> for ::windows::core::IInspectable {
    fn from(value: &ImageFeatureValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ImageFeatureValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ImageFeatureValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ImageFeatureValue> for ILearningModelFeatureValue {
    type Error = ::windows::core::Error;
    fn try_from(value: ImageFeatureValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ImageFeatureValue> for ILearningModelFeatureValue {
    type Error = ::windows::core::Error;
    fn try_from(value: &ImageFeatureValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureValue> for ImageFeatureValue {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureValue> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureValue> for &ImageFeatureValue {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureValue> {
        ::core::convert::TryInto::<ILearningModelFeatureValue>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ImageFeatureValue {}
unsafe impl ::core::marker::Sync for ImageFeatureValue {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct LearningModel(::windows::core::IUnknown);
impl LearningModel {
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Author(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Author)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Domain(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Domain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Description)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Version(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Version)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Metadata(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Metadata)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InputFeatures(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ILearningModelFeatureDescriptor>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InputFeatures)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ILearningModelFeatureDescriptor>>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutputFeatures(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ILearningModelFeatureDescriptor>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OutputFeatures)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ILearningModelFeatureDescriptor>>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LoadFromStorageFileAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>>(modelfile: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LearningModel>> {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LoadFromStorageFileAsync)(::core::mem::transmute_copy(this), modelfile.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LearningModel>>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn LoadFromStreamAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>>(modelstream: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LearningModel>> {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LoadFromStreamAsync)(::core::mem::transmute_copy(this), modelstream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LearningModel>>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn LoadFromFilePath<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(filepath: Param0) -> ::windows::core::Result<LearningModel> {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LoadFromFilePath)(::core::mem::transmute_copy(this), filepath.into_param().abi(), &mut result__).from_abi::<LearningModel>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadFromStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>>(modelstream: Param0) -> ::windows::core::Result<LearningModel> {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LoadFromStream)(::core::mem::transmute_copy(this), modelstream.into_param().abi(), &mut result__).from_abi::<LearningModel>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LoadFromStorageFileWithOperatorProviderAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>, Param1: ::windows::core::IntoParam<'a, ILearningModelOperatorProvider>>(modelfile: Param0, operatorprovider: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LearningModel>> {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LoadFromStorageFileWithOperatorProviderAsync)(::core::mem::transmute_copy(this), modelfile.into_param().abi(), operatorprovider.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LearningModel>>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn LoadFromStreamWithOperatorProviderAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>, Param1: ::windows::core::IntoParam<'a, ILearningModelOperatorProvider>>(modelstream: Param0, operatorprovider: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LearningModel>> {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LoadFromStreamWithOperatorProviderAsync)(::core::mem::transmute_copy(this), modelstream.into_param().abi(), operatorprovider.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LearningModel>>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn LoadFromFilePathWithOperatorProvider<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ILearningModelOperatorProvider>>(filepath: Param0, operatorprovider: Param1) -> ::windows::core::Result<LearningModel> {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LoadFromFilePathWithOperatorProvider)(::core::mem::transmute_copy(this), filepath.into_param().abi(), operatorprovider.into_param().abi(), &mut result__).from_abi::<LearningModel>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadFromStreamWithOperatorProvider<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>, Param1: ::windows::core::IntoParam<'a, ILearningModelOperatorProvider>>(modelstream: Param0, operatorprovider: Param1) -> ::windows::core::Result<LearningModel> {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LoadFromStreamWithOperatorProvider)(::core::mem::transmute_copy(this), modelstream.into_param().abi(), operatorprovider.into_param().abi(), &mut result__).from_abi::<LearningModel>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILearningModelStatics<R, F: FnOnce(&ILearningModelStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LearningModel, ILearningModelStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LearningModel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LearningModel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LearningModel {}
impl ::core::fmt::Debug for LearningModel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LearningModel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LearningModel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.LearningModel;{5b8e4920-489f-4e86-9128-265a327b78fa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LearningModel {
    type Vtable = ILearningModel_Vtbl;
    const IID: ::windows::core::GUID = <ILearningModel as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LearningModel {
    const NAME: &'static str = "Windows.AI.MachineLearning.LearningModel";
}
impl ::core::convert::From<LearningModel> for ::windows::core::IUnknown {
    fn from(value: LearningModel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LearningModel> for ::windows::core::IUnknown {
    fn from(value: &LearningModel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LearningModel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LearningModel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LearningModel> for ::windows::core::IInspectable {
    fn from(value: LearningModel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LearningModel> for ::windows::core::IInspectable {
    fn from(value: &LearningModel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LearningModel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LearningModel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<LearningModel> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: LearningModel) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&LearningModel> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &LearningModel) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for LearningModel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &LearningModel {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LearningModel {}
unsafe impl ::core::marker::Sync for LearningModel {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct LearningModelBinding(::windows::core::IUnknown);
impl LearningModelBinding {
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).First)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Bind<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, name: Param0, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Bind)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn BindWithProperties<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(&self, name: Param0, value: Param1, props: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).BindWithProperties)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), props.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Clear)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn CreateFromSession<'a, Param0: ::windows::core::IntoParam<'a, LearningModelSession>>(session: Param0) -> ::windows::core::Result<LearningModelBinding> {
        Self::ILearningModelBindingFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromSession)(::core::mem::transmute_copy(this), session.into_param().abi(), &mut result__).from_abi::<LearningModelBinding>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Lookup)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Size)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasKey)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Split(&self, first: &mut ::core::option::Option<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>, second: &mut ::core::option::Option<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Split)(::core::mem::transmute_copy(this), first as *mut _ as _, second as *mut _ as _).ok() }
    }
    #[doc(hidden)]
    pub fn ILearningModelBindingFactory<R, F: FnOnce(&ILearningModelBindingFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LearningModelBinding, ILearningModelBindingFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LearningModelBinding {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LearningModelBinding {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LearningModelBinding {}
impl ::core::fmt::Debug for LearningModelBinding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LearningModelBinding").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LearningModelBinding {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.LearningModelBinding;{ea312f20-168f-4f8c-94fe-2e7ac31b4aa8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LearningModelBinding {
    type Vtable = ILearningModelBinding_Vtbl;
    const IID: ::windows::core::GUID = <ILearningModelBinding as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LearningModelBinding {
    const NAME: &'static str = "Windows.AI.MachineLearning.LearningModelBinding";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for LearningModelBinding {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &LearningModelBinding {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl ::core::convert::From<LearningModelBinding> for ::windows::core::IUnknown {
    fn from(value: LearningModelBinding) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LearningModelBinding> for ::windows::core::IUnknown {
    fn from(value: &LearningModelBinding) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LearningModelBinding {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LearningModelBinding {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LearningModelBinding> for ::windows::core::IInspectable {
    fn from(value: LearningModelBinding) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LearningModelBinding> for ::windows::core::IInspectable {
    fn from(value: &LearningModelBinding) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LearningModelBinding {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LearningModelBinding {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<LearningModelBinding> for super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>> {
    type Error = ::windows::core::Error;
    fn try_from(value: LearningModelBinding) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&LearningModelBinding> for super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &LearningModelBinding) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> for LearningModelBinding {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> for &LearningModelBinding {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
        ::core::convert::TryInto::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<LearningModelBinding> for super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable> {
    type Error = ::windows::core::Error;
    fn try_from(value: LearningModelBinding) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&LearningModelBinding> for super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &LearningModelBinding) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> for LearningModelBinding {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> for &LearningModelBinding {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        ::core::convert::TryInto::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LearningModelBinding {}
unsafe impl ::core::marker::Sync for LearningModelBinding {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct LearningModelDevice(::windows::core::IUnknown);
impl LearningModelDevice {
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Graphics\"`*"]
    #[cfg(feature = "Graphics")]
    pub fn AdapterId(&self) -> ::windows::core::Result<super::super::Graphics::DisplayAdapterId> {
        let this = self;
        unsafe {
            let mut result__: super::super::Graphics::DisplayAdapterId = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AdapterId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::DisplayAdapterId>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Direct3D11Device(&self) -> ::windows::core::Result<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Direct3D11Device)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Create(devicekind: LearningModelDeviceKind) -> ::windows::core::Result<LearningModelDevice> {
        Self::ILearningModelDeviceFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), devicekind, &mut result__).from_abi::<LearningModelDevice>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CreateFromDirect3D11Device<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>>(device: Param0) -> ::windows::core::Result<LearningModelDevice> {
        Self::ILearningModelDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromDirect3D11Device)(::core::mem::transmute_copy(this), device.into_param().abi(), &mut result__).from_abi::<LearningModelDevice>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILearningModelDeviceFactory<R, F: FnOnce(&ILearningModelDeviceFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LearningModelDevice, ILearningModelDeviceFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ILearningModelDeviceStatics<R, F: FnOnce(&ILearningModelDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LearningModelDevice, ILearningModelDeviceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LearningModelDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LearningModelDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LearningModelDevice {}
impl ::core::fmt::Debug for LearningModelDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LearningModelDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LearningModelDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.LearningModelDevice;{f5c2c8fe-3f56-4a8c-ac5f-fdb92d8b8252})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LearningModelDevice {
    type Vtable = ILearningModelDevice_Vtbl;
    const IID: ::windows::core::GUID = <ILearningModelDevice as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LearningModelDevice {
    const NAME: &'static str = "Windows.AI.MachineLearning.LearningModelDevice";
}
impl ::core::convert::From<LearningModelDevice> for ::windows::core::IUnknown {
    fn from(value: LearningModelDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LearningModelDevice> for ::windows::core::IUnknown {
    fn from(value: &LearningModelDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LearningModelDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LearningModelDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LearningModelDevice> for ::windows::core::IInspectable {
    fn from(value: LearningModelDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LearningModelDevice> for ::windows::core::IInspectable {
    fn from(value: &LearningModelDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LearningModelDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LearningModelDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LearningModelDevice {}
unsafe impl ::core::marker::Sync for LearningModelDevice {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LearningModelDeviceKind(pub i32);
impl LearningModelDeviceKind {
    pub const Default: Self = Self(0i32);
    pub const Cpu: Self = Self(1i32);
    pub const DirectX: Self = Self(2i32);
    pub const DirectXHighPerformance: Self = Self(3i32);
    pub const DirectXMinPower: Self = Self(4i32);
}
impl ::core::marker::Copy for LearningModelDeviceKind {}
impl ::core::clone::Clone for LearningModelDeviceKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LearningModelDeviceKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LearningModelDeviceKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for LearningModelDeviceKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LearningModelDeviceKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LearningModelDeviceKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.AI.MachineLearning.LearningModelDeviceKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct LearningModelEvaluationResult(::windows::core::IUnknown);
impl LearningModelEvaluationResult {
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CorrelationId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn ErrorStatus(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ErrorStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Succeeded(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Succeeded)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Outputs(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Outputs)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
}
impl ::core::clone::Clone for LearningModelEvaluationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LearningModelEvaluationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LearningModelEvaluationResult {}
impl ::core::fmt::Debug for LearningModelEvaluationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LearningModelEvaluationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LearningModelEvaluationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.LearningModelEvaluationResult;{b2f9bfcd-960e-49c0-8593-eb190ae3eee2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LearningModelEvaluationResult {
    type Vtable = ILearningModelEvaluationResult_Vtbl;
    const IID: ::windows::core::GUID = <ILearningModelEvaluationResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LearningModelEvaluationResult {
    const NAME: &'static str = "Windows.AI.MachineLearning.LearningModelEvaluationResult";
}
impl ::core::convert::From<LearningModelEvaluationResult> for ::windows::core::IUnknown {
    fn from(value: LearningModelEvaluationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LearningModelEvaluationResult> for ::windows::core::IUnknown {
    fn from(value: &LearningModelEvaluationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LearningModelEvaluationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LearningModelEvaluationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LearningModelEvaluationResult> for ::windows::core::IInspectable {
    fn from(value: LearningModelEvaluationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LearningModelEvaluationResult> for ::windows::core::IInspectable {
    fn from(value: &LearningModelEvaluationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LearningModelEvaluationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LearningModelEvaluationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LearningModelEvaluationResult {}
unsafe impl ::core::marker::Sync for LearningModelEvaluationResult {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LearningModelFeatureKind(pub i32);
impl LearningModelFeatureKind {
    pub const Tensor: Self = Self(0i32);
    pub const Sequence: Self = Self(1i32);
    pub const Map: Self = Self(2i32);
    pub const Image: Self = Self(3i32);
}
impl ::core::marker::Copy for LearningModelFeatureKind {}
impl ::core::clone::Clone for LearningModelFeatureKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LearningModelFeatureKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LearningModelFeatureKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for LearningModelFeatureKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LearningModelFeatureKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LearningModelFeatureKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.AI.MachineLearning.LearningModelFeatureKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LearningModelPixelRange(pub i32);
impl LearningModelPixelRange {
    pub const ZeroTo255: Self = Self(0i32);
    pub const ZeroToOne: Self = Self(1i32);
    pub const MinusOneToOne: Self = Self(2i32);
}
impl ::core::marker::Copy for LearningModelPixelRange {}
impl ::core::clone::Clone for LearningModelPixelRange {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LearningModelPixelRange {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LearningModelPixelRange {
    type Abi = Self;
}
impl ::core::fmt::Debug for LearningModelPixelRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LearningModelPixelRange").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LearningModelPixelRange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.AI.MachineLearning.LearningModelPixelRange;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct LearningModelSession(::windows::core::IUnknown);
impl LearningModelSession {
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Model(&self) -> ::windows::core::Result<LearningModel> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Model)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LearningModel>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Device(&self) -> ::windows::core::Result<LearningModelDevice> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Device)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelDevice>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn EvaluationProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EvaluationProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EvaluateAsync<'a, Param0: ::windows::core::IntoParam<'a, LearningModelBinding>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, bindings: Param0, correlationid: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LearningModelEvaluationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EvaluateAsync)(::core::mem::transmute_copy(this), bindings.into_param().abi(), correlationid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LearningModelEvaluationResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn EvaluateFeaturesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, features: Param0, correlationid: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LearningModelEvaluationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EvaluateFeaturesAsync)(::core::mem::transmute_copy(this), features.into_param().abi(), correlationid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LearningModelEvaluationResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Evaluate<'a, Param0: ::windows::core::IntoParam<'a, LearningModelBinding>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, bindings: Param0, correlationid: Param1) -> ::windows::core::Result<LearningModelEvaluationResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Evaluate)(::core::mem::transmute_copy(this), bindings.into_param().abi(), correlationid.into_param().abi(), &mut result__).from_abi::<LearningModelEvaluationResult>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn EvaluateFeatures<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, features: Param0, correlationid: Param1) -> ::windows::core::Result<LearningModelEvaluationResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EvaluateFeatures)(::core::mem::transmute_copy(this), features.into_param().abi(), correlationid.into_param().abi(), &mut result__).from_abi::<LearningModelEvaluationResult>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn CreateFromModel<'a, Param0: ::windows::core::IntoParam<'a, LearningModel>>(model: Param0) -> ::windows::core::Result<LearningModelSession> {
        Self::ILearningModelSessionFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromModel)(::core::mem::transmute_copy(this), model.into_param().abi(), &mut result__).from_abi::<LearningModelSession>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn CreateFromModelOnDevice<'a, Param0: ::windows::core::IntoParam<'a, LearningModel>, Param1: ::windows::core::IntoParam<'a, LearningModelDevice>>(model: Param0, devicetorunon: Param1) -> ::windows::core::Result<LearningModelSession> {
        Self::ILearningModelSessionFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromModelOnDevice)(::core::mem::transmute_copy(this), model.into_param().abi(), devicetorunon.into_param().abi(), &mut result__).from_abi::<LearningModelSession>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn CreateFromModelOnDeviceWithSessionOptions<'a, Param0: ::windows::core::IntoParam<'a, LearningModel>, Param1: ::windows::core::IntoParam<'a, LearningModelDevice>, Param2: ::windows::core::IntoParam<'a, LearningModelSessionOptions>>(model: Param0, devicetorunon: Param1, learningmodelsessionoptions: Param2) -> ::windows::core::Result<LearningModelSession> {
        Self::ILearningModelSessionFactory2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromModelOnDeviceWithSessionOptions)(::core::mem::transmute_copy(this), model.into_param().abi(), devicetorunon.into_param().abi(), learningmodelsessionoptions.into_param().abi(), &mut result__).from_abi::<LearningModelSession>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILearningModelSessionFactory<R, F: FnOnce(&ILearningModelSessionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LearningModelSession, ILearningModelSessionFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ILearningModelSessionFactory2<R, F: FnOnce(&ILearningModelSessionFactory2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LearningModelSession, ILearningModelSessionFactory2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LearningModelSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LearningModelSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LearningModelSession {}
impl ::core::fmt::Debug for LearningModelSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LearningModelSession").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LearningModelSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.LearningModelSession;{8e58f8f6-b787-4c11-90f0-7129aeca74a9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LearningModelSession {
    type Vtable = ILearningModelSession_Vtbl;
    const IID: ::windows::core::GUID = <ILearningModelSession as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LearningModelSession {
    const NAME: &'static str = "Windows.AI.MachineLearning.LearningModelSession";
}
impl ::core::convert::From<LearningModelSession> for ::windows::core::IUnknown {
    fn from(value: LearningModelSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LearningModelSession> for ::windows::core::IUnknown {
    fn from(value: &LearningModelSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LearningModelSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LearningModelSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LearningModelSession> for ::windows::core::IInspectable {
    fn from(value: LearningModelSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LearningModelSession> for ::windows::core::IInspectable {
    fn from(value: &LearningModelSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LearningModelSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LearningModelSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<LearningModelSession> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: LearningModelSession) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&LearningModelSession> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &LearningModelSession) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for LearningModelSession {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &LearningModelSession {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LearningModelSession {}
unsafe impl ::core::marker::Sync for LearningModelSession {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct LearningModelSessionOptions(::windows::core::IUnknown);
impl LearningModelSessionOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LearningModelSessionOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn BatchSizeOverride(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BatchSizeOverride)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn SetBatchSizeOverride(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBatchSizeOverride)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn CloseModelOnSessionCreation(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ILearningModelSessionOptions2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CloseModelOnSessionCreation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn SetCloseModelOnSessionCreation(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILearningModelSessionOptions2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCloseModelOnSessionCreation)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn OverrideNamedDimension<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, dimension: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILearningModelSessionOptions3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).OverrideNamedDimension)(::core::mem::transmute_copy(this), name.into_param().abi(), dimension).ok() }
    }
}
impl ::core::clone::Clone for LearningModelSessionOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LearningModelSessionOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LearningModelSessionOptions {}
impl ::core::fmt::Debug for LearningModelSessionOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LearningModelSessionOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LearningModelSessionOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.LearningModelSessionOptions;{b8f63fa1-134d-5133-8cff-3a5c3c263beb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LearningModelSessionOptions {
    type Vtable = ILearningModelSessionOptions_Vtbl;
    const IID: ::windows::core::GUID = <ILearningModelSessionOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LearningModelSessionOptions {
    const NAME: &'static str = "Windows.AI.MachineLearning.LearningModelSessionOptions";
}
impl ::core::convert::From<LearningModelSessionOptions> for ::windows::core::IUnknown {
    fn from(value: LearningModelSessionOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LearningModelSessionOptions> for ::windows::core::IUnknown {
    fn from(value: &LearningModelSessionOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LearningModelSessionOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LearningModelSessionOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LearningModelSessionOptions> for ::windows::core::IInspectable {
    fn from(value: LearningModelSessionOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LearningModelSessionOptions> for ::windows::core::IInspectable {
    fn from(value: &LearningModelSessionOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LearningModelSessionOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LearningModelSessionOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LearningModelSessionOptions {}
unsafe impl ::core::marker::Sync for LearningModelSessionOptions {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct MapFeatureDescriptor(::windows::core::IUnknown);
impl MapFeatureDescriptor {
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Description)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn IsRequired(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsRequired)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn KeyKind(&self) -> ::windows::core::Result<TensorKind> {
        let this = self;
        unsafe {
            let mut result__: TensorKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TensorKind>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn ValueDescriptor(&self) -> ::windows::core::Result<ILearningModelFeatureDescriptor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ValueDescriptor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ILearningModelFeatureDescriptor>(result__)
        }
    }
}
impl ::core::clone::Clone for MapFeatureDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MapFeatureDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapFeatureDescriptor {}
impl ::core::fmt::Debug for MapFeatureDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapFeatureDescriptor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MapFeatureDescriptor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.MapFeatureDescriptor;{530424bd-a257-436d-9e60-c2981f7cc5c4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MapFeatureDescriptor {
    type Vtable = IMapFeatureDescriptor_Vtbl;
    const IID: ::windows::core::GUID = <IMapFeatureDescriptor as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MapFeatureDescriptor {
    const NAME: &'static str = "Windows.AI.MachineLearning.MapFeatureDescriptor";
}
impl ::core::convert::From<MapFeatureDescriptor> for ::windows::core::IUnknown {
    fn from(value: MapFeatureDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapFeatureDescriptor> for ::windows::core::IUnknown {
    fn from(value: &MapFeatureDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MapFeatureDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MapFeatureDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MapFeatureDescriptor> for ::windows::core::IInspectable {
    fn from(value: MapFeatureDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MapFeatureDescriptor> for ::windows::core::IInspectable {
    fn from(value: &MapFeatureDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MapFeatureDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MapFeatureDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<MapFeatureDescriptor> for ILearningModelFeatureDescriptor {
    type Error = ::windows::core::Error;
    fn try_from(value: MapFeatureDescriptor) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MapFeatureDescriptor> for ILearningModelFeatureDescriptor {
    type Error = ::windows::core::Error;
    fn try_from(value: &MapFeatureDescriptor) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureDescriptor> for MapFeatureDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureDescriptor> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureDescriptor> for &MapFeatureDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureDescriptor> {
        ::core::convert::TryInto::<ILearningModelFeatureDescriptor>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MapFeatureDescriptor {}
unsafe impl ::core::marker::Sync for MapFeatureDescriptor {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct SequenceFeatureDescriptor(::windows::core::IUnknown);
impl SequenceFeatureDescriptor {
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Description)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn IsRequired(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsRequired)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn ElementDescriptor(&self) -> ::windows::core::Result<ILearningModelFeatureDescriptor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementDescriptor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ILearningModelFeatureDescriptor>(result__)
        }
    }
}
impl ::core::clone::Clone for SequenceFeatureDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SequenceFeatureDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SequenceFeatureDescriptor {}
impl ::core::fmt::Debug for SequenceFeatureDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SequenceFeatureDescriptor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SequenceFeatureDescriptor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.SequenceFeatureDescriptor;{84f6945a-562b-4d62-a851-739aced96668})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SequenceFeatureDescriptor {
    type Vtable = ISequenceFeatureDescriptor_Vtbl;
    const IID: ::windows::core::GUID = <ISequenceFeatureDescriptor as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SequenceFeatureDescriptor {
    const NAME: &'static str = "Windows.AI.MachineLearning.SequenceFeatureDescriptor";
}
impl ::core::convert::From<SequenceFeatureDescriptor> for ::windows::core::IUnknown {
    fn from(value: SequenceFeatureDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SequenceFeatureDescriptor> for ::windows::core::IUnknown {
    fn from(value: &SequenceFeatureDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SequenceFeatureDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SequenceFeatureDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SequenceFeatureDescriptor> for ::windows::core::IInspectable {
    fn from(value: SequenceFeatureDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SequenceFeatureDescriptor> for ::windows::core::IInspectable {
    fn from(value: &SequenceFeatureDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SequenceFeatureDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SequenceFeatureDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SequenceFeatureDescriptor> for ILearningModelFeatureDescriptor {
    type Error = ::windows::core::Error;
    fn try_from(value: SequenceFeatureDescriptor) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SequenceFeatureDescriptor> for ILearningModelFeatureDescriptor {
    type Error = ::windows::core::Error;
    fn try_from(value: &SequenceFeatureDescriptor) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureDescriptor> for SequenceFeatureDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureDescriptor> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureDescriptor> for &SequenceFeatureDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureDescriptor> {
        ::core::convert::TryInto::<ILearningModelFeatureDescriptor>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SequenceFeatureDescriptor {}
unsafe impl ::core::marker::Sync for SequenceFeatureDescriptor {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct TensorBoolean(::windows::core::IUnknown);
impl TensorBoolean {
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows::core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateReference)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IMemoryBufferReference>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn TensorKind(&self) -> ::windows::core::Result<TensorKind> {
        let this = &::windows::core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: TensorKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TensorKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TensorKind>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Shape)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i64>>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAsVectorView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Create() -> ::windows::core::Result<TensorBoolean> {
        Self::ITensorBooleanStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TensorBoolean>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0) -> ::windows::core::Result<TensorBoolean> {
        Self::ITensorBooleanStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create2)(::core::mem::transmute_copy(this), shape.into_param().abi(), &mut result__).from_abi::<TensorBoolean>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0, data: &[bool]) -> ::windows::core::Result<TensorBoolean> {
        Self::ITensorBooleanStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromArray)(::core::mem::transmute_copy(this), shape.into_param().abi(), data.len() as u32, ::core::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorBoolean>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<bool>>>(shape: Param0, data: Param1) -> ::windows::core::Result<TensorBoolean> {
        Self::ITensorBooleanStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromIterable)(::core::mem::transmute_copy(this), shape.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<TensorBoolean>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[bool]) -> ::windows::core::Result<TensorBoolean> {
        Self::ITensorBooleanStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::core::mem::transmute_copy(this), shape.len() as u32, ::core::mem::transmute(shape.as_ptr()), data.len() as u32, ::core::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorBoolean>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<'a, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(shape: &[i64], buffer: Param1) -> ::windows::core::Result<TensorBoolean> {
        Self::ITensorBooleanStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromBuffer)(::core::mem::transmute_copy(this), shape.len() as u32, ::core::mem::transmute(shape.as_ptr()), buffer.into_param().abi(), &mut result__).from_abi::<TensorBoolean>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorBooleanStatics<R, F: FnOnce(&ITensorBooleanStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TensorBoolean, ITensorBooleanStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ITensorBooleanStatics2<R, F: FnOnce(&ITensorBooleanStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TensorBoolean, ITensorBooleanStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TensorBoolean {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TensorBoolean {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TensorBoolean {}
impl ::core::fmt::Debug for TensorBoolean {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TensorBoolean").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TensorBoolean {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorBoolean;{50f311ed-29e9-4a5c-a44d-8fc512584eed})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for TensorBoolean {
    type Vtable = ITensorBoolean_Vtbl;
    const IID: ::windows::core::GUID = <ITensorBoolean as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TensorBoolean {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorBoolean";
}
impl ::core::convert::From<TensorBoolean> for ::windows::core::IUnknown {
    fn from(value: TensorBoolean) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TensorBoolean> for ::windows::core::IUnknown {
    fn from(value: &TensorBoolean) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TensorBoolean {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TensorBoolean {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TensorBoolean> for ::windows::core::IInspectable {
    fn from(value: TensorBoolean) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TensorBoolean> for ::windows::core::IInspectable {
    fn from(value: &TensorBoolean) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TensorBoolean {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TensorBoolean {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<TensorBoolean> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorBoolean) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&TensorBoolean> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorBoolean) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for TensorBoolean {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &TensorBoolean {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<TensorBoolean> for ILearningModelFeatureValue {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorBoolean) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TensorBoolean> for ILearningModelFeatureValue {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorBoolean) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureValue> for TensorBoolean {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureValue> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureValue> for &TensorBoolean {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureValue> {
        ::core::convert::TryInto::<ILearningModelFeatureValue>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<TensorBoolean> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorBoolean) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&TensorBoolean> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorBoolean) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for TensorBoolean {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for &TensorBoolean {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::core::convert::TryInto::<super::super::Foundation::IMemoryBuffer>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<TensorBoolean> for ITensor {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorBoolean) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TensorBoolean> for ITensor {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorBoolean) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITensor> for TensorBoolean {
    fn into_param(self) -> ::windows::core::Param<'a, ITensor> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITensor> for &TensorBoolean {
    fn into_param(self) -> ::windows::core::Param<'a, ITensor> {
        ::core::convert::TryInto::<ITensor>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for TensorBoolean {}
unsafe impl ::core::marker::Sync for TensorBoolean {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct TensorDouble(::windows::core::IUnknown);
impl TensorDouble {
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows::core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateReference)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IMemoryBufferReference>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn TensorKind(&self) -> ::windows::core::Result<TensorKind> {
        let this = &::windows::core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: TensorKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TensorKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TensorKind>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Shape)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i64>>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAsVectorView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<f64>>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Create() -> ::windows::core::Result<TensorDouble> {
        Self::ITensorDoubleStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TensorDouble>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0) -> ::windows::core::Result<TensorDouble> {
        Self::ITensorDoubleStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create2)(::core::mem::transmute_copy(this), shape.into_param().abi(), &mut result__).from_abi::<TensorDouble>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0, data: &[f64]) -> ::windows::core::Result<TensorDouble> {
        Self::ITensorDoubleStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromArray)(::core::mem::transmute_copy(this), shape.into_param().abi(), data.len() as u32, ::core::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorDouble>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<f64>>>(shape: Param0, data: Param1) -> ::windows::core::Result<TensorDouble> {
        Self::ITensorDoubleStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromIterable)(::core::mem::transmute_copy(this), shape.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<TensorDouble>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[f64]) -> ::windows::core::Result<TensorDouble> {
        Self::ITensorDoubleStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::core::mem::transmute_copy(this), shape.len() as u32, ::core::mem::transmute(shape.as_ptr()), data.len() as u32, ::core::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorDouble>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<'a, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(shape: &[i64], buffer: Param1) -> ::windows::core::Result<TensorDouble> {
        Self::ITensorDoubleStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromBuffer)(::core::mem::transmute_copy(this), shape.len() as u32, ::core::mem::transmute(shape.as_ptr()), buffer.into_param().abi(), &mut result__).from_abi::<TensorDouble>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorDoubleStatics<R, F: FnOnce(&ITensorDoubleStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TensorDouble, ITensorDoubleStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ITensorDoubleStatics2<R, F: FnOnce(&ITensorDoubleStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TensorDouble, ITensorDoubleStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TensorDouble {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TensorDouble {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TensorDouble {}
impl ::core::fmt::Debug for TensorDouble {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TensorDouble").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TensorDouble {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorDouble;{91e41252-7a8f-4f0e-a28f-9637ffc8a3d0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for TensorDouble {
    type Vtable = ITensorDouble_Vtbl;
    const IID: ::windows::core::GUID = <ITensorDouble as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TensorDouble {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorDouble";
}
impl ::core::convert::From<TensorDouble> for ::windows::core::IUnknown {
    fn from(value: TensorDouble) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TensorDouble> for ::windows::core::IUnknown {
    fn from(value: &TensorDouble) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TensorDouble {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TensorDouble {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TensorDouble> for ::windows::core::IInspectable {
    fn from(value: TensorDouble) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TensorDouble> for ::windows::core::IInspectable {
    fn from(value: &TensorDouble) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TensorDouble {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TensorDouble {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<TensorDouble> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorDouble) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&TensorDouble> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorDouble) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for TensorDouble {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &TensorDouble {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<TensorDouble> for ILearningModelFeatureValue {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorDouble) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TensorDouble> for ILearningModelFeatureValue {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorDouble) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureValue> for TensorDouble {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureValue> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureValue> for &TensorDouble {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureValue> {
        ::core::convert::TryInto::<ILearningModelFeatureValue>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<TensorDouble> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorDouble) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&TensorDouble> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorDouble) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for TensorDouble {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for &TensorDouble {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::core::convert::TryInto::<super::super::Foundation::IMemoryBuffer>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<TensorDouble> for ITensor {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorDouble) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TensorDouble> for ITensor {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorDouble) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITensor> for TensorDouble {
    fn into_param(self) -> ::windows::core::Param<'a, ITensor> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITensor> for &TensorDouble {
    fn into_param(self) -> ::windows::core::Param<'a, ITensor> {
        ::core::convert::TryInto::<ITensor>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for TensorDouble {}
unsafe impl ::core::marker::Sync for TensorDouble {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct TensorFeatureDescriptor(::windows::core::IUnknown);
impl TensorFeatureDescriptor {
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Description)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn IsRequired(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsRequired)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn TensorKind(&self) -> ::windows::core::Result<TensorKind> {
        let this = self;
        unsafe {
            let mut result__: TensorKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TensorKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TensorKind>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Shape)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i64>>(result__)
        }
    }
}
impl ::core::clone::Clone for TensorFeatureDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TensorFeatureDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TensorFeatureDescriptor {}
impl ::core::fmt::Debug for TensorFeatureDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TensorFeatureDescriptor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TensorFeatureDescriptor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorFeatureDescriptor;{74455c80-946a-4310-a19c-ee0af028fce4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for TensorFeatureDescriptor {
    type Vtable = ITensorFeatureDescriptor_Vtbl;
    const IID: ::windows::core::GUID = <ITensorFeatureDescriptor as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TensorFeatureDescriptor {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorFeatureDescriptor";
}
impl ::core::convert::From<TensorFeatureDescriptor> for ::windows::core::IUnknown {
    fn from(value: TensorFeatureDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TensorFeatureDescriptor> for ::windows::core::IUnknown {
    fn from(value: &TensorFeatureDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TensorFeatureDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TensorFeatureDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TensorFeatureDescriptor> for ::windows::core::IInspectable {
    fn from(value: TensorFeatureDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TensorFeatureDescriptor> for ::windows::core::IInspectable {
    fn from(value: &TensorFeatureDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TensorFeatureDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TensorFeatureDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<TensorFeatureDescriptor> for ILearningModelFeatureDescriptor {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorFeatureDescriptor) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TensorFeatureDescriptor> for ILearningModelFeatureDescriptor {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorFeatureDescriptor) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureDescriptor> for TensorFeatureDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureDescriptor> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureDescriptor> for &TensorFeatureDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureDescriptor> {
        ::core::convert::TryInto::<ILearningModelFeatureDescriptor>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for TensorFeatureDescriptor {}
unsafe impl ::core::marker::Sync for TensorFeatureDescriptor {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct TensorFloat(::windows::core::IUnknown);
impl TensorFloat {
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows::core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateReference)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IMemoryBufferReference>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn TensorKind(&self) -> ::windows::core::Result<TensorKind> {
        let this = &::windows::core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: TensorKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TensorKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TensorKind>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Shape)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i64>>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<f32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAsVectorView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<f32>>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Create() -> ::windows::core::Result<TensorFloat> {
        Self::ITensorFloatStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TensorFloat>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0) -> ::windows::core::Result<TensorFloat> {
        Self::ITensorFloatStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create2)(::core::mem::transmute_copy(this), shape.into_param().abi(), &mut result__).from_abi::<TensorFloat>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0, data: &[f32]) -> ::windows::core::Result<TensorFloat> {
        Self::ITensorFloatStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromArray)(::core::mem::transmute_copy(this), shape.into_param().abi(), data.len() as u32, ::core::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorFloat>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<f32>>>(shape: Param0, data: Param1) -> ::windows::core::Result<TensorFloat> {
        Self::ITensorFloatStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromIterable)(::core::mem::transmute_copy(this), shape.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<TensorFloat>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[f32]) -> ::windows::core::Result<TensorFloat> {
        Self::ITensorFloatStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::core::mem::transmute_copy(this), shape.len() as u32, ::core::mem::transmute(shape.as_ptr()), data.len() as u32, ::core::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorFloat>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<'a, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(shape: &[i64], buffer: Param1) -> ::windows::core::Result<TensorFloat> {
        Self::ITensorFloatStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromBuffer)(::core::mem::transmute_copy(this), shape.len() as u32, ::core::mem::transmute(shape.as_ptr()), buffer.into_param().abi(), &mut result__).from_abi::<TensorFloat>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorFloatStatics<R, F: FnOnce(&ITensorFloatStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TensorFloat, ITensorFloatStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ITensorFloatStatics2<R, F: FnOnce(&ITensorFloatStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TensorFloat, ITensorFloatStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TensorFloat {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TensorFloat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TensorFloat {}
impl ::core::fmt::Debug for TensorFloat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TensorFloat").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TensorFloat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorFloat;{f2282d82-aa02-42c8-a0c8-df1efc9676e1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for TensorFloat {
    type Vtable = ITensorFloat_Vtbl;
    const IID: ::windows::core::GUID = <ITensorFloat as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TensorFloat {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorFloat";
}
impl ::core::convert::From<TensorFloat> for ::windows::core::IUnknown {
    fn from(value: TensorFloat) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TensorFloat> for ::windows::core::IUnknown {
    fn from(value: &TensorFloat) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TensorFloat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TensorFloat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TensorFloat> for ::windows::core::IInspectable {
    fn from(value: TensorFloat) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TensorFloat> for ::windows::core::IInspectable {
    fn from(value: &TensorFloat) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TensorFloat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TensorFloat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<TensorFloat> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorFloat) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&TensorFloat> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorFloat) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for TensorFloat {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &TensorFloat {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<TensorFloat> for ILearningModelFeatureValue {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorFloat) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TensorFloat> for ILearningModelFeatureValue {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorFloat) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureValue> for TensorFloat {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureValue> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureValue> for &TensorFloat {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureValue> {
        ::core::convert::TryInto::<ILearningModelFeatureValue>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<TensorFloat> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorFloat) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&TensorFloat> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorFloat) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for TensorFloat {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for &TensorFloat {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::core::convert::TryInto::<super::super::Foundation::IMemoryBuffer>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<TensorFloat> for ITensor {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorFloat) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TensorFloat> for ITensor {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorFloat) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITensor> for TensorFloat {
    fn into_param(self) -> ::windows::core::Param<'a, ITensor> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITensor> for &TensorFloat {
    fn into_param(self) -> ::windows::core::Param<'a, ITensor> {
        ::core::convert::TryInto::<ITensor>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for TensorFloat {}
unsafe impl ::core::marker::Sync for TensorFloat {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct TensorFloat16Bit(::windows::core::IUnknown);
impl TensorFloat16Bit {
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows::core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateReference)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IMemoryBufferReference>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn TensorKind(&self) -> ::windows::core::Result<TensorKind> {
        let this = &::windows::core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: TensorKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TensorKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TensorKind>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Shape)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i64>>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<f32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAsVectorView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<f32>>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Create() -> ::windows::core::Result<TensorFloat16Bit> {
        Self::ITensorFloat16BitStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TensorFloat16Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0) -> ::windows::core::Result<TensorFloat16Bit> {
        Self::ITensorFloat16BitStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create2)(::core::mem::transmute_copy(this), shape.into_param().abi(), &mut result__).from_abi::<TensorFloat16Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0, data: &[f32]) -> ::windows::core::Result<TensorFloat16Bit> {
        Self::ITensorFloat16BitStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromArray)(::core::mem::transmute_copy(this), shape.into_param().abi(), data.len() as u32, ::core::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorFloat16Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<f32>>>(shape: Param0, data: Param1) -> ::windows::core::Result<TensorFloat16Bit> {
        Self::ITensorFloat16BitStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromIterable)(::core::mem::transmute_copy(this), shape.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<TensorFloat16Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[f32]) -> ::windows::core::Result<TensorFloat16Bit> {
        Self::ITensorFloat16BitStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::core::mem::transmute_copy(this), shape.len() as u32, ::core::mem::transmute(shape.as_ptr()), data.len() as u32, ::core::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorFloat16Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<'a, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(shape: &[i64], buffer: Param1) -> ::windows::core::Result<TensorFloat16Bit> {
        Self::ITensorFloat16BitStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromBuffer)(::core::mem::transmute_copy(this), shape.len() as u32, ::core::mem::transmute(shape.as_ptr()), buffer.into_param().abi(), &mut result__).from_abi::<TensorFloat16Bit>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorFloat16BitStatics<R, F: FnOnce(&ITensorFloat16BitStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TensorFloat16Bit, ITensorFloat16BitStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ITensorFloat16BitStatics2<R, F: FnOnce(&ITensorFloat16BitStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TensorFloat16Bit, ITensorFloat16BitStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TensorFloat16Bit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TensorFloat16Bit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TensorFloat16Bit {}
impl ::core::fmt::Debug for TensorFloat16Bit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TensorFloat16Bit").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TensorFloat16Bit {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorFloat16Bit;{0ab994fc-5b89-4c3c-b5e4-5282a5316c0a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for TensorFloat16Bit {
    type Vtable = ITensorFloat16Bit_Vtbl;
    const IID: ::windows::core::GUID = <ITensorFloat16Bit as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TensorFloat16Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorFloat16Bit";
}
impl ::core::convert::From<TensorFloat16Bit> for ::windows::core::IUnknown {
    fn from(value: TensorFloat16Bit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TensorFloat16Bit> for ::windows::core::IUnknown {
    fn from(value: &TensorFloat16Bit) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TensorFloat16Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TensorFloat16Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TensorFloat16Bit> for ::windows::core::IInspectable {
    fn from(value: TensorFloat16Bit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TensorFloat16Bit> for ::windows::core::IInspectable {
    fn from(value: &TensorFloat16Bit) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TensorFloat16Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TensorFloat16Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<TensorFloat16Bit> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorFloat16Bit) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&TensorFloat16Bit> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorFloat16Bit) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for TensorFloat16Bit {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &TensorFloat16Bit {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<TensorFloat16Bit> for ILearningModelFeatureValue {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorFloat16Bit) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TensorFloat16Bit> for ILearningModelFeatureValue {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorFloat16Bit) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureValue> for TensorFloat16Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureValue> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureValue> for &TensorFloat16Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureValue> {
        ::core::convert::TryInto::<ILearningModelFeatureValue>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<TensorFloat16Bit> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorFloat16Bit) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&TensorFloat16Bit> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorFloat16Bit) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for TensorFloat16Bit {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for &TensorFloat16Bit {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::core::convert::TryInto::<super::super::Foundation::IMemoryBuffer>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<TensorFloat16Bit> for ITensor {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorFloat16Bit) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TensorFloat16Bit> for ITensor {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorFloat16Bit) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITensor> for TensorFloat16Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ITensor> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITensor> for &TensorFloat16Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ITensor> {
        ::core::convert::TryInto::<ITensor>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for TensorFloat16Bit {}
unsafe impl ::core::marker::Sync for TensorFloat16Bit {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct TensorInt16Bit(::windows::core::IUnknown);
impl TensorInt16Bit {
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows::core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateReference)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IMemoryBufferReference>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn TensorKind(&self) -> ::windows::core::Result<TensorKind> {
        let this = &::windows::core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: TensorKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TensorKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TensorKind>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Shape)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i64>>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i16>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAsVectorView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i16>>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Create() -> ::windows::core::Result<TensorInt16Bit> {
        Self::ITensorInt16BitStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TensorInt16Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0) -> ::windows::core::Result<TensorInt16Bit> {
        Self::ITensorInt16BitStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create2)(::core::mem::transmute_copy(this), shape.into_param().abi(), &mut result__).from_abi::<TensorInt16Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0, data: &[i16]) -> ::windows::core::Result<TensorInt16Bit> {
        Self::ITensorInt16BitStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromArray)(::core::mem::transmute_copy(this), shape.into_param().abi(), data.len() as u32, ::core::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorInt16Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i16>>>(shape: Param0, data: Param1) -> ::windows::core::Result<TensorInt16Bit> {
        Self::ITensorInt16BitStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromIterable)(::core::mem::transmute_copy(this), shape.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<TensorInt16Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[i16]) -> ::windows::core::Result<TensorInt16Bit> {
        Self::ITensorInt16BitStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::core::mem::transmute_copy(this), shape.len() as u32, ::core::mem::transmute(shape.as_ptr()), data.len() as u32, ::core::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorInt16Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<'a, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(shape: &[i64], buffer: Param1) -> ::windows::core::Result<TensorInt16Bit> {
        Self::ITensorInt16BitStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromBuffer)(::core::mem::transmute_copy(this), shape.len() as u32, ::core::mem::transmute(shape.as_ptr()), buffer.into_param().abi(), &mut result__).from_abi::<TensorInt16Bit>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorInt16BitStatics<R, F: FnOnce(&ITensorInt16BitStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TensorInt16Bit, ITensorInt16BitStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ITensorInt16BitStatics2<R, F: FnOnce(&ITensorInt16BitStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TensorInt16Bit, ITensorInt16BitStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TensorInt16Bit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TensorInt16Bit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TensorInt16Bit {}
impl ::core::fmt::Debug for TensorInt16Bit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TensorInt16Bit").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TensorInt16Bit {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorInt16Bit;{98a32d39-e6d6-44af-8afa-baebc44dc020})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for TensorInt16Bit {
    type Vtable = ITensorInt16Bit_Vtbl;
    const IID: ::windows::core::GUID = <ITensorInt16Bit as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TensorInt16Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorInt16Bit";
}
impl ::core::convert::From<TensorInt16Bit> for ::windows::core::IUnknown {
    fn from(value: TensorInt16Bit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TensorInt16Bit> for ::windows::core::IUnknown {
    fn from(value: &TensorInt16Bit) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TensorInt16Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TensorInt16Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TensorInt16Bit> for ::windows::core::IInspectable {
    fn from(value: TensorInt16Bit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TensorInt16Bit> for ::windows::core::IInspectable {
    fn from(value: &TensorInt16Bit) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TensorInt16Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TensorInt16Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<TensorInt16Bit> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorInt16Bit) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&TensorInt16Bit> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorInt16Bit) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for TensorInt16Bit {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &TensorInt16Bit {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<TensorInt16Bit> for ILearningModelFeatureValue {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorInt16Bit) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TensorInt16Bit> for ILearningModelFeatureValue {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorInt16Bit) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureValue> for TensorInt16Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureValue> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureValue> for &TensorInt16Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureValue> {
        ::core::convert::TryInto::<ILearningModelFeatureValue>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<TensorInt16Bit> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorInt16Bit) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&TensorInt16Bit> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorInt16Bit) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for TensorInt16Bit {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for &TensorInt16Bit {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::core::convert::TryInto::<super::super::Foundation::IMemoryBuffer>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<TensorInt16Bit> for ITensor {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorInt16Bit) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TensorInt16Bit> for ITensor {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorInt16Bit) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITensor> for TensorInt16Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ITensor> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITensor> for &TensorInt16Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ITensor> {
        ::core::convert::TryInto::<ITensor>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for TensorInt16Bit {}
unsafe impl ::core::marker::Sync for TensorInt16Bit {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct TensorInt32Bit(::windows::core::IUnknown);
impl TensorInt32Bit {
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows::core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateReference)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IMemoryBufferReference>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn TensorKind(&self) -> ::windows::core::Result<TensorKind> {
        let this = &::windows::core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: TensorKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TensorKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TensorKind>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Shape)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i64>>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAsVectorView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Create() -> ::windows::core::Result<TensorInt32Bit> {
        Self::ITensorInt32BitStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TensorInt32Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0) -> ::windows::core::Result<TensorInt32Bit> {
        Self::ITensorInt32BitStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create2)(::core::mem::transmute_copy(this), shape.into_param().abi(), &mut result__).from_abi::<TensorInt32Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0, data: &[i32]) -> ::windows::core::Result<TensorInt32Bit> {
        Self::ITensorInt32BitStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromArray)(::core::mem::transmute_copy(this), shape.into_param().abi(), data.len() as u32, ::core::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorInt32Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i32>>>(shape: Param0, data: Param1) -> ::windows::core::Result<TensorInt32Bit> {
        Self::ITensorInt32BitStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromIterable)(::core::mem::transmute_copy(this), shape.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<TensorInt32Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[i32]) -> ::windows::core::Result<TensorInt32Bit> {
        Self::ITensorInt32BitStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::core::mem::transmute_copy(this), shape.len() as u32, ::core::mem::transmute(shape.as_ptr()), data.len() as u32, ::core::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorInt32Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<'a, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(shape: &[i64], buffer: Param1) -> ::windows::core::Result<TensorInt32Bit> {
        Self::ITensorInt32BitStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromBuffer)(::core::mem::transmute_copy(this), shape.len() as u32, ::core::mem::transmute(shape.as_ptr()), buffer.into_param().abi(), &mut result__).from_abi::<TensorInt32Bit>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorInt32BitStatics<R, F: FnOnce(&ITensorInt32BitStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TensorInt32Bit, ITensorInt32BitStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ITensorInt32BitStatics2<R, F: FnOnce(&ITensorInt32BitStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TensorInt32Bit, ITensorInt32BitStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TensorInt32Bit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TensorInt32Bit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TensorInt32Bit {}
impl ::core::fmt::Debug for TensorInt32Bit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TensorInt32Bit").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TensorInt32Bit {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorInt32Bit;{2c0c28d3-207c-4486-a7d2-884522c5e589})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for TensorInt32Bit {
    type Vtable = ITensorInt32Bit_Vtbl;
    const IID: ::windows::core::GUID = <ITensorInt32Bit as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TensorInt32Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorInt32Bit";
}
impl ::core::convert::From<TensorInt32Bit> for ::windows::core::IUnknown {
    fn from(value: TensorInt32Bit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TensorInt32Bit> for ::windows::core::IUnknown {
    fn from(value: &TensorInt32Bit) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TensorInt32Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TensorInt32Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TensorInt32Bit> for ::windows::core::IInspectable {
    fn from(value: TensorInt32Bit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TensorInt32Bit> for ::windows::core::IInspectable {
    fn from(value: &TensorInt32Bit) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TensorInt32Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TensorInt32Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<TensorInt32Bit> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorInt32Bit) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&TensorInt32Bit> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorInt32Bit) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for TensorInt32Bit {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &TensorInt32Bit {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<TensorInt32Bit> for ILearningModelFeatureValue {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorInt32Bit) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TensorInt32Bit> for ILearningModelFeatureValue {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorInt32Bit) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureValue> for TensorInt32Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureValue> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureValue> for &TensorInt32Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureValue> {
        ::core::convert::TryInto::<ILearningModelFeatureValue>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<TensorInt32Bit> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorInt32Bit) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&TensorInt32Bit> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorInt32Bit) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for TensorInt32Bit {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for &TensorInt32Bit {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::core::convert::TryInto::<super::super::Foundation::IMemoryBuffer>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<TensorInt32Bit> for ITensor {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorInt32Bit) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TensorInt32Bit> for ITensor {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorInt32Bit) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITensor> for TensorInt32Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ITensor> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITensor> for &TensorInt32Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ITensor> {
        ::core::convert::TryInto::<ITensor>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for TensorInt32Bit {}
unsafe impl ::core::marker::Sync for TensorInt32Bit {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct TensorInt64Bit(::windows::core::IUnknown);
impl TensorInt64Bit {
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows::core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateReference)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IMemoryBufferReference>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn TensorKind(&self) -> ::windows::core::Result<TensorKind> {
        let this = &::windows::core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: TensorKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TensorKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TensorKind>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Shape)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i64>>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAsVectorView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i64>>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Create() -> ::windows::core::Result<TensorInt64Bit> {
        Self::ITensorInt64BitStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TensorInt64Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0) -> ::windows::core::Result<TensorInt64Bit> {
        Self::ITensorInt64BitStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create2)(::core::mem::transmute_copy(this), shape.into_param().abi(), &mut result__).from_abi::<TensorInt64Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0, data: &[i64]) -> ::windows::core::Result<TensorInt64Bit> {
        Self::ITensorInt64BitStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromArray)(::core::mem::transmute_copy(this), shape.into_param().abi(), data.len() as u32, ::core::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorInt64Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0, data: Param1) -> ::windows::core::Result<TensorInt64Bit> {
        Self::ITensorInt64BitStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromIterable)(::core::mem::transmute_copy(this), shape.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<TensorInt64Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[i64]) -> ::windows::core::Result<TensorInt64Bit> {
        Self::ITensorInt64BitStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::core::mem::transmute_copy(this), shape.len() as u32, ::core::mem::transmute(shape.as_ptr()), data.len() as u32, ::core::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorInt64Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<'a, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(shape: &[i64], buffer: Param1) -> ::windows::core::Result<TensorInt64Bit> {
        Self::ITensorInt64BitStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromBuffer)(::core::mem::transmute_copy(this), shape.len() as u32, ::core::mem::transmute(shape.as_ptr()), buffer.into_param().abi(), &mut result__).from_abi::<TensorInt64Bit>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorInt64BitStatics<R, F: FnOnce(&ITensorInt64BitStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TensorInt64Bit, ITensorInt64BitStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ITensorInt64BitStatics2<R, F: FnOnce(&ITensorInt64BitStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TensorInt64Bit, ITensorInt64BitStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TensorInt64Bit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TensorInt64Bit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TensorInt64Bit {}
impl ::core::fmt::Debug for TensorInt64Bit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TensorInt64Bit").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TensorInt64Bit {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorInt64Bit;{499665ba-1fa2-45ad-af25-a0bd9bda4c87})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for TensorInt64Bit {
    type Vtable = ITensorInt64Bit_Vtbl;
    const IID: ::windows::core::GUID = <ITensorInt64Bit as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TensorInt64Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorInt64Bit";
}
impl ::core::convert::From<TensorInt64Bit> for ::windows::core::IUnknown {
    fn from(value: TensorInt64Bit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TensorInt64Bit> for ::windows::core::IUnknown {
    fn from(value: &TensorInt64Bit) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TensorInt64Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TensorInt64Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TensorInt64Bit> for ::windows::core::IInspectable {
    fn from(value: TensorInt64Bit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TensorInt64Bit> for ::windows::core::IInspectable {
    fn from(value: &TensorInt64Bit) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TensorInt64Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TensorInt64Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<TensorInt64Bit> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorInt64Bit) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&TensorInt64Bit> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorInt64Bit) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for TensorInt64Bit {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &TensorInt64Bit {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<TensorInt64Bit> for ILearningModelFeatureValue {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorInt64Bit) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TensorInt64Bit> for ILearningModelFeatureValue {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorInt64Bit) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureValue> for TensorInt64Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureValue> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureValue> for &TensorInt64Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureValue> {
        ::core::convert::TryInto::<ILearningModelFeatureValue>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<TensorInt64Bit> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorInt64Bit) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&TensorInt64Bit> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorInt64Bit) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for TensorInt64Bit {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for &TensorInt64Bit {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::core::convert::TryInto::<super::super::Foundation::IMemoryBuffer>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<TensorInt64Bit> for ITensor {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorInt64Bit) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TensorInt64Bit> for ITensor {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorInt64Bit) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITensor> for TensorInt64Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ITensor> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITensor> for &TensorInt64Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ITensor> {
        ::core::convert::TryInto::<ITensor>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for TensorInt64Bit {}
unsafe impl ::core::marker::Sync for TensorInt64Bit {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct TensorInt8Bit(::windows::core::IUnknown);
impl TensorInt8Bit {
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows::core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateReference)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IMemoryBufferReference>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn TensorKind(&self) -> ::windows::core::Result<TensorKind> {
        let this = &::windows::core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: TensorKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TensorKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TensorKind>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Shape)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i64>>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAsVectorView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u8>>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Create() -> ::windows::core::Result<TensorInt8Bit> {
        Self::ITensorInt8BitStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TensorInt8Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0) -> ::windows::core::Result<TensorInt8Bit> {
        Self::ITensorInt8BitStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create2)(::core::mem::transmute_copy(this), shape.into_param().abi(), &mut result__).from_abi::<TensorInt8Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0, data: &[u8]) -> ::windows::core::Result<TensorInt8Bit> {
        Self::ITensorInt8BitStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromArray)(::core::mem::transmute_copy(this), shape.into_param().abi(), data.len() as u32, ::core::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorInt8Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<u8>>>(shape: Param0, data: Param1) -> ::windows::core::Result<TensorInt8Bit> {
        Self::ITensorInt8BitStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromIterable)(::core::mem::transmute_copy(this), shape.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<TensorInt8Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[u8]) -> ::windows::core::Result<TensorInt8Bit> {
        Self::ITensorInt8BitStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::core::mem::transmute_copy(this), shape.len() as u32, ::core::mem::transmute(shape.as_ptr()), data.len() as u32, ::core::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorInt8Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<'a, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(shape: &[i64], buffer: Param1) -> ::windows::core::Result<TensorInt8Bit> {
        Self::ITensorInt8BitStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromBuffer)(::core::mem::transmute_copy(this), shape.len() as u32, ::core::mem::transmute(shape.as_ptr()), buffer.into_param().abi(), &mut result__).from_abi::<TensorInt8Bit>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorInt8BitStatics<R, F: FnOnce(&ITensorInt8BitStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TensorInt8Bit, ITensorInt8BitStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ITensorInt8BitStatics2<R, F: FnOnce(&ITensorInt8BitStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TensorInt8Bit, ITensorInt8BitStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TensorInt8Bit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TensorInt8Bit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TensorInt8Bit {}
impl ::core::fmt::Debug for TensorInt8Bit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TensorInt8Bit").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TensorInt8Bit {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorInt8Bit;{cddd97c5-ffd8-4fef-aefb-30e1a485b2ee})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for TensorInt8Bit {
    type Vtable = ITensorInt8Bit_Vtbl;
    const IID: ::windows::core::GUID = <ITensorInt8Bit as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TensorInt8Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorInt8Bit";
}
impl ::core::convert::From<TensorInt8Bit> for ::windows::core::IUnknown {
    fn from(value: TensorInt8Bit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TensorInt8Bit> for ::windows::core::IUnknown {
    fn from(value: &TensorInt8Bit) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TensorInt8Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TensorInt8Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TensorInt8Bit> for ::windows::core::IInspectable {
    fn from(value: TensorInt8Bit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TensorInt8Bit> for ::windows::core::IInspectable {
    fn from(value: &TensorInt8Bit) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TensorInt8Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TensorInt8Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<TensorInt8Bit> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorInt8Bit) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&TensorInt8Bit> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorInt8Bit) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for TensorInt8Bit {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &TensorInt8Bit {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<TensorInt8Bit> for ILearningModelFeatureValue {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorInt8Bit) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TensorInt8Bit> for ILearningModelFeatureValue {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorInt8Bit) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureValue> for TensorInt8Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureValue> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureValue> for &TensorInt8Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureValue> {
        ::core::convert::TryInto::<ILearningModelFeatureValue>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<TensorInt8Bit> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorInt8Bit) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&TensorInt8Bit> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorInt8Bit) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for TensorInt8Bit {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for &TensorInt8Bit {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::core::convert::TryInto::<super::super::Foundation::IMemoryBuffer>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<TensorInt8Bit> for ITensor {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorInt8Bit) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TensorInt8Bit> for ITensor {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorInt8Bit) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITensor> for TensorInt8Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ITensor> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITensor> for &TensorInt8Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ITensor> {
        ::core::convert::TryInto::<ITensor>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for TensorInt8Bit {}
unsafe impl ::core::marker::Sync for TensorInt8Bit {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TensorKind(pub i32);
impl TensorKind {
    pub const Undefined: Self = Self(0i32);
    pub const Float: Self = Self(1i32);
    pub const UInt8: Self = Self(2i32);
    pub const Int8: Self = Self(3i32);
    pub const UInt16: Self = Self(4i32);
    pub const Int16: Self = Self(5i32);
    pub const Int32: Self = Self(6i32);
    pub const Int64: Self = Self(7i32);
    pub const String: Self = Self(8i32);
    pub const Boolean: Self = Self(9i32);
    pub const Float16: Self = Self(10i32);
    pub const Double: Self = Self(11i32);
    pub const UInt32: Self = Self(12i32);
    pub const UInt64: Self = Self(13i32);
    pub const Complex64: Self = Self(14i32);
    pub const Complex128: Self = Self(15i32);
}
impl ::core::marker::Copy for TensorKind {}
impl ::core::clone::Clone for TensorKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TensorKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TensorKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for TensorKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TensorKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TensorKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.AI.MachineLearning.TensorKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct TensorString(::windows::core::IUnknown);
impl TensorString {
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows::core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateReference)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IMemoryBufferReference>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn TensorKind(&self) -> ::windows::core::Result<TensorKind> {
        let this = &::windows::core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: TensorKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TensorKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TensorKind>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Shape)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i64>>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAsVectorView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Create() -> ::windows::core::Result<TensorString> {
        Self::ITensorStringStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TensorString>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0) -> ::windows::core::Result<TensorString> {
        Self::ITensorStringStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create2)(::core::mem::transmute_copy(this), shape.into_param().abi(), &mut result__).from_abi::<TensorString>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0, data: &[::windows::core::HSTRING]) -> ::windows::core::Result<TensorString> {
        Self::ITensorStringStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromArray)(::core::mem::transmute_copy(this), shape.into_param().abi(), data.len() as u32, ::core::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorString>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(shape: Param0, data: Param1) -> ::windows::core::Result<TensorString> {
        Self::ITensorStringStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromIterable)(::core::mem::transmute_copy(this), shape.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<TensorString>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[::windows::core::HSTRING]) -> ::windows::core::Result<TensorString> {
        Self::ITensorStringStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::core::mem::transmute_copy(this), shape.len() as u32, ::core::mem::transmute(shape.as_ptr()), data.len() as u32, ::core::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorString>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorStringStatics<R, F: FnOnce(&ITensorStringStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TensorString, ITensorStringStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ITensorStringStatics2<R, F: FnOnce(&ITensorStringStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TensorString, ITensorStringStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TensorString {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TensorString {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TensorString {}
impl ::core::fmt::Debug for TensorString {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TensorString").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TensorString {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorString;{582335c8-bdb1-4610-bc75-35e9cbf009b7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for TensorString {
    type Vtable = ITensorString_Vtbl;
    const IID: ::windows::core::GUID = <ITensorString as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TensorString {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorString";
}
impl ::core::convert::From<TensorString> for ::windows::core::IUnknown {
    fn from(value: TensorString) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TensorString> for ::windows::core::IUnknown {
    fn from(value: &TensorString) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TensorString {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TensorString {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TensorString> for ::windows::core::IInspectable {
    fn from(value: TensorString) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TensorString> for ::windows::core::IInspectable {
    fn from(value: &TensorString) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TensorString {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TensorString {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<TensorString> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorString) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&TensorString> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorString) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for TensorString {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &TensorString {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<TensorString> for ILearningModelFeatureValue {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorString) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TensorString> for ILearningModelFeatureValue {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorString) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureValue> for TensorString {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureValue> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureValue> for &TensorString {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureValue> {
        ::core::convert::TryInto::<ILearningModelFeatureValue>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<TensorString> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorString) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&TensorString> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorString) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for TensorString {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for &TensorString {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::core::convert::TryInto::<super::super::Foundation::IMemoryBuffer>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<TensorString> for ITensor {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorString) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TensorString> for ITensor {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorString) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITensor> for TensorString {
    fn into_param(self) -> ::windows::core::Param<'a, ITensor> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITensor> for &TensorString {
    fn into_param(self) -> ::windows::core::Param<'a, ITensor> {
        ::core::convert::TryInto::<ITensor>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for TensorString {}
unsafe impl ::core::marker::Sync for TensorString {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct TensorUInt16Bit(::windows::core::IUnknown);
impl TensorUInt16Bit {
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows::core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateReference)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IMemoryBufferReference>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn TensorKind(&self) -> ::windows::core::Result<TensorKind> {
        let this = &::windows::core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: TensorKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TensorKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TensorKind>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Shape)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i64>>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u16>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAsVectorView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u16>>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Create() -> ::windows::core::Result<TensorUInt16Bit> {
        Self::ITensorUInt16BitStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TensorUInt16Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0) -> ::windows::core::Result<TensorUInt16Bit> {
        Self::ITensorUInt16BitStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create2)(::core::mem::transmute_copy(this), shape.into_param().abi(), &mut result__).from_abi::<TensorUInt16Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0, data: &[u16]) -> ::windows::core::Result<TensorUInt16Bit> {
        Self::ITensorUInt16BitStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromArray)(::core::mem::transmute_copy(this), shape.into_param().abi(), data.len() as u32, ::core::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorUInt16Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<u16>>>(shape: Param0, data: Param1) -> ::windows::core::Result<TensorUInt16Bit> {
        Self::ITensorUInt16BitStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromIterable)(::core::mem::transmute_copy(this), shape.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<TensorUInt16Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[u16]) -> ::windows::core::Result<TensorUInt16Bit> {
        Self::ITensorUInt16BitStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::core::mem::transmute_copy(this), shape.len() as u32, ::core::mem::transmute(shape.as_ptr()), data.len() as u32, ::core::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorUInt16Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<'a, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(shape: &[i64], buffer: Param1) -> ::windows::core::Result<TensorUInt16Bit> {
        Self::ITensorUInt16BitStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromBuffer)(::core::mem::transmute_copy(this), shape.len() as u32, ::core::mem::transmute(shape.as_ptr()), buffer.into_param().abi(), &mut result__).from_abi::<TensorUInt16Bit>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorUInt16BitStatics<R, F: FnOnce(&ITensorUInt16BitStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TensorUInt16Bit, ITensorUInt16BitStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ITensorUInt16BitStatics2<R, F: FnOnce(&ITensorUInt16BitStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TensorUInt16Bit, ITensorUInt16BitStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TensorUInt16Bit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TensorUInt16Bit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TensorUInt16Bit {}
impl ::core::fmt::Debug for TensorUInt16Bit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TensorUInt16Bit").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TensorUInt16Bit {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorUInt16Bit;{68140f4b-23c0-42f3-81f6-a891c011bc3f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for TensorUInt16Bit {
    type Vtable = ITensorUInt16Bit_Vtbl;
    const IID: ::windows::core::GUID = <ITensorUInt16Bit as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TensorUInt16Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorUInt16Bit";
}
impl ::core::convert::From<TensorUInt16Bit> for ::windows::core::IUnknown {
    fn from(value: TensorUInt16Bit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TensorUInt16Bit> for ::windows::core::IUnknown {
    fn from(value: &TensorUInt16Bit) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TensorUInt16Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TensorUInt16Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TensorUInt16Bit> for ::windows::core::IInspectable {
    fn from(value: TensorUInt16Bit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TensorUInt16Bit> for ::windows::core::IInspectable {
    fn from(value: &TensorUInt16Bit) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TensorUInt16Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TensorUInt16Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<TensorUInt16Bit> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorUInt16Bit) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&TensorUInt16Bit> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorUInt16Bit) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for TensorUInt16Bit {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &TensorUInt16Bit {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<TensorUInt16Bit> for ILearningModelFeatureValue {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorUInt16Bit) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TensorUInt16Bit> for ILearningModelFeatureValue {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorUInt16Bit) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureValue> for TensorUInt16Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureValue> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureValue> for &TensorUInt16Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureValue> {
        ::core::convert::TryInto::<ILearningModelFeatureValue>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<TensorUInt16Bit> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorUInt16Bit) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&TensorUInt16Bit> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorUInt16Bit) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for TensorUInt16Bit {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for &TensorUInt16Bit {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::core::convert::TryInto::<super::super::Foundation::IMemoryBuffer>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<TensorUInt16Bit> for ITensor {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorUInt16Bit) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TensorUInt16Bit> for ITensor {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorUInt16Bit) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITensor> for TensorUInt16Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ITensor> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITensor> for &TensorUInt16Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ITensor> {
        ::core::convert::TryInto::<ITensor>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for TensorUInt16Bit {}
unsafe impl ::core::marker::Sync for TensorUInt16Bit {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct TensorUInt32Bit(::windows::core::IUnknown);
impl TensorUInt32Bit {
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows::core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateReference)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IMemoryBufferReference>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn TensorKind(&self) -> ::windows::core::Result<TensorKind> {
        let this = &::windows::core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: TensorKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TensorKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TensorKind>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Shape)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i64>>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAsVectorView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Create() -> ::windows::core::Result<TensorUInt32Bit> {
        Self::ITensorUInt32BitStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TensorUInt32Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0) -> ::windows::core::Result<TensorUInt32Bit> {
        Self::ITensorUInt32BitStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create2)(::core::mem::transmute_copy(this), shape.into_param().abi(), &mut result__).from_abi::<TensorUInt32Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0, data: &[u32]) -> ::windows::core::Result<TensorUInt32Bit> {
        Self::ITensorUInt32BitStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromArray)(::core::mem::transmute_copy(this), shape.into_param().abi(), data.len() as u32, ::core::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorUInt32Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<u32>>>(shape: Param0, data: Param1) -> ::windows::core::Result<TensorUInt32Bit> {
        Self::ITensorUInt32BitStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromIterable)(::core::mem::transmute_copy(this), shape.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<TensorUInt32Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[u32]) -> ::windows::core::Result<TensorUInt32Bit> {
        Self::ITensorUInt32BitStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::core::mem::transmute_copy(this), shape.len() as u32, ::core::mem::transmute(shape.as_ptr()), data.len() as u32, ::core::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorUInt32Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<'a, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(shape: &[i64], buffer: Param1) -> ::windows::core::Result<TensorUInt32Bit> {
        Self::ITensorUInt32BitStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromBuffer)(::core::mem::transmute_copy(this), shape.len() as u32, ::core::mem::transmute(shape.as_ptr()), buffer.into_param().abi(), &mut result__).from_abi::<TensorUInt32Bit>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorUInt32BitStatics<R, F: FnOnce(&ITensorUInt32BitStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TensorUInt32Bit, ITensorUInt32BitStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ITensorUInt32BitStatics2<R, F: FnOnce(&ITensorUInt32BitStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TensorUInt32Bit, ITensorUInt32BitStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TensorUInt32Bit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TensorUInt32Bit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TensorUInt32Bit {}
impl ::core::fmt::Debug for TensorUInt32Bit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TensorUInt32Bit").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TensorUInt32Bit {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorUInt32Bit;{d8c9c2ff-7511-45a3-bfac-c38f370d2237})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for TensorUInt32Bit {
    type Vtable = ITensorUInt32Bit_Vtbl;
    const IID: ::windows::core::GUID = <ITensorUInt32Bit as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TensorUInt32Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorUInt32Bit";
}
impl ::core::convert::From<TensorUInt32Bit> for ::windows::core::IUnknown {
    fn from(value: TensorUInt32Bit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TensorUInt32Bit> for ::windows::core::IUnknown {
    fn from(value: &TensorUInt32Bit) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TensorUInt32Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TensorUInt32Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TensorUInt32Bit> for ::windows::core::IInspectable {
    fn from(value: TensorUInt32Bit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TensorUInt32Bit> for ::windows::core::IInspectable {
    fn from(value: &TensorUInt32Bit) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TensorUInt32Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TensorUInt32Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<TensorUInt32Bit> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorUInt32Bit) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&TensorUInt32Bit> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorUInt32Bit) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for TensorUInt32Bit {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &TensorUInt32Bit {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<TensorUInt32Bit> for ILearningModelFeatureValue {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorUInt32Bit) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TensorUInt32Bit> for ILearningModelFeatureValue {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorUInt32Bit) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureValue> for TensorUInt32Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureValue> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureValue> for &TensorUInt32Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureValue> {
        ::core::convert::TryInto::<ILearningModelFeatureValue>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<TensorUInt32Bit> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorUInt32Bit) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&TensorUInt32Bit> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorUInt32Bit) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for TensorUInt32Bit {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for &TensorUInt32Bit {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::core::convert::TryInto::<super::super::Foundation::IMemoryBuffer>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<TensorUInt32Bit> for ITensor {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorUInt32Bit) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TensorUInt32Bit> for ITensor {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorUInt32Bit) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITensor> for TensorUInt32Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ITensor> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITensor> for &TensorUInt32Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ITensor> {
        ::core::convert::TryInto::<ITensor>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for TensorUInt32Bit {}
unsafe impl ::core::marker::Sync for TensorUInt32Bit {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct TensorUInt64Bit(::windows::core::IUnknown);
impl TensorUInt64Bit {
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows::core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateReference)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IMemoryBufferReference>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn TensorKind(&self) -> ::windows::core::Result<TensorKind> {
        let this = &::windows::core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: TensorKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TensorKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TensorKind>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Shape)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i64>>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAsVectorView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u64>>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Create() -> ::windows::core::Result<TensorUInt64Bit> {
        Self::ITensorUInt64BitStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TensorUInt64Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0) -> ::windows::core::Result<TensorUInt64Bit> {
        Self::ITensorUInt64BitStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create2)(::core::mem::transmute_copy(this), shape.into_param().abi(), &mut result__).from_abi::<TensorUInt64Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0, data: &[u64]) -> ::windows::core::Result<TensorUInt64Bit> {
        Self::ITensorUInt64BitStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromArray)(::core::mem::transmute_copy(this), shape.into_param().abi(), data.len() as u32, ::core::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorUInt64Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<u64>>>(shape: Param0, data: Param1) -> ::windows::core::Result<TensorUInt64Bit> {
        Self::ITensorUInt64BitStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromIterable)(::core::mem::transmute_copy(this), shape.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<TensorUInt64Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[u64]) -> ::windows::core::Result<TensorUInt64Bit> {
        Self::ITensorUInt64BitStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::core::mem::transmute_copy(this), shape.len() as u32, ::core::mem::transmute(shape.as_ptr()), data.len() as u32, ::core::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorUInt64Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<'a, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(shape: &[i64], buffer: Param1) -> ::windows::core::Result<TensorUInt64Bit> {
        Self::ITensorUInt64BitStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromBuffer)(::core::mem::transmute_copy(this), shape.len() as u32, ::core::mem::transmute(shape.as_ptr()), buffer.into_param().abi(), &mut result__).from_abi::<TensorUInt64Bit>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorUInt64BitStatics<R, F: FnOnce(&ITensorUInt64BitStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TensorUInt64Bit, ITensorUInt64BitStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ITensorUInt64BitStatics2<R, F: FnOnce(&ITensorUInt64BitStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TensorUInt64Bit, ITensorUInt64BitStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TensorUInt64Bit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TensorUInt64Bit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TensorUInt64Bit {}
impl ::core::fmt::Debug for TensorUInt64Bit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TensorUInt64Bit").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TensorUInt64Bit {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorUInt64Bit;{2e70ffad-04bf-4825-839a-82baef8c7886})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for TensorUInt64Bit {
    type Vtable = ITensorUInt64Bit_Vtbl;
    const IID: ::windows::core::GUID = <ITensorUInt64Bit as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TensorUInt64Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorUInt64Bit";
}
impl ::core::convert::From<TensorUInt64Bit> for ::windows::core::IUnknown {
    fn from(value: TensorUInt64Bit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TensorUInt64Bit> for ::windows::core::IUnknown {
    fn from(value: &TensorUInt64Bit) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TensorUInt64Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TensorUInt64Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TensorUInt64Bit> for ::windows::core::IInspectable {
    fn from(value: TensorUInt64Bit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TensorUInt64Bit> for ::windows::core::IInspectable {
    fn from(value: &TensorUInt64Bit) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TensorUInt64Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TensorUInt64Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<TensorUInt64Bit> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorUInt64Bit) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&TensorUInt64Bit> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorUInt64Bit) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for TensorUInt64Bit {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &TensorUInt64Bit {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<TensorUInt64Bit> for ILearningModelFeatureValue {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorUInt64Bit) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TensorUInt64Bit> for ILearningModelFeatureValue {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorUInt64Bit) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureValue> for TensorUInt64Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureValue> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureValue> for &TensorUInt64Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureValue> {
        ::core::convert::TryInto::<ILearningModelFeatureValue>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<TensorUInt64Bit> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorUInt64Bit) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&TensorUInt64Bit> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorUInt64Bit) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for TensorUInt64Bit {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for &TensorUInt64Bit {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::core::convert::TryInto::<super::super::Foundation::IMemoryBuffer>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<TensorUInt64Bit> for ITensor {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorUInt64Bit) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TensorUInt64Bit> for ITensor {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorUInt64Bit) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITensor> for TensorUInt64Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ITensor> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITensor> for &TensorUInt64Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ITensor> {
        ::core::convert::TryInto::<ITensor>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for TensorUInt64Bit {}
unsafe impl ::core::marker::Sync for TensorUInt64Bit {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct TensorUInt8Bit(::windows::core::IUnknown);
impl TensorUInt8Bit {
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__: LearningModelFeatureKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LearningModelFeatureKind>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows::core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateReference)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IMemoryBufferReference>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn TensorKind(&self) -> ::windows::core::Result<TensorKind> {
        let this = &::windows::core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: TensorKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TensorKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TensorKind>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Shape)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i64>>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAsVectorView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u8>>(result__)
        }
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn Create() -> ::windows::core::Result<TensorUInt8Bit> {
        Self::ITensorUInt8BitStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TensorUInt8Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0) -> ::windows::core::Result<TensorUInt8Bit> {
        Self::ITensorUInt8BitStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create2)(::core::mem::transmute_copy(this), shape.into_param().abi(), &mut result__).from_abi::<TensorUInt8Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>>(shape: Param0, data: &[u8]) -> ::windows::core::Result<TensorUInt8Bit> {
        Self::ITensorUInt8BitStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromArray)(::core::mem::transmute_copy(this), shape.into_param().abi(), data.len() as u32, ::core::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorUInt8Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<i64>>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<u8>>>(shape: Param0, data: Param1) -> ::windows::core::Result<TensorUInt8Bit> {
        Self::ITensorUInt8BitStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromIterable)(::core::mem::transmute_copy(this), shape.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<TensorUInt8Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`*"]
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[u8]) -> ::windows::core::Result<TensorUInt8Bit> {
        Self::ITensorUInt8BitStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::core::mem::transmute_copy(this), shape.len() as u32, ::core::mem::transmute(shape.as_ptr()), data.len() as u32, ::core::mem::transmute(data.as_ptr()), &mut result__).from_abi::<TensorUInt8Bit>(result__)
        })
    }
    #[doc = "*Required features: `\"AI_MachineLearning\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<'a, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(shape: &[i64], buffer: Param1) -> ::windows::core::Result<TensorUInt8Bit> {
        Self::ITensorUInt8BitStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromBuffer)(::core::mem::transmute_copy(this), shape.len() as u32, ::core::mem::transmute(shape.as_ptr()), buffer.into_param().abi(), &mut result__).from_abi::<TensorUInt8Bit>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorUInt8BitStatics<R, F: FnOnce(&ITensorUInt8BitStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TensorUInt8Bit, ITensorUInt8BitStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ITensorUInt8BitStatics2<R, F: FnOnce(&ITensorUInt8BitStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TensorUInt8Bit, ITensorUInt8BitStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TensorUInt8Bit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TensorUInt8Bit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TensorUInt8Bit {}
impl ::core::fmt::Debug for TensorUInt8Bit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TensorUInt8Bit").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TensorUInt8Bit {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorUInt8Bit;{58e1ae27-622b-48e3-be22-d867aed1daac})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for TensorUInt8Bit {
    type Vtable = ITensorUInt8Bit_Vtbl;
    const IID: ::windows::core::GUID = <ITensorUInt8Bit as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TensorUInt8Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorUInt8Bit";
}
impl ::core::convert::From<TensorUInt8Bit> for ::windows::core::IUnknown {
    fn from(value: TensorUInt8Bit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TensorUInt8Bit> for ::windows::core::IUnknown {
    fn from(value: &TensorUInt8Bit) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TensorUInt8Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TensorUInt8Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TensorUInt8Bit> for ::windows::core::IInspectable {
    fn from(value: TensorUInt8Bit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TensorUInt8Bit> for ::windows::core::IInspectable {
    fn from(value: &TensorUInt8Bit) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TensorUInt8Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TensorUInt8Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<TensorUInt8Bit> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorUInt8Bit) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&TensorUInt8Bit> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorUInt8Bit) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for TensorUInt8Bit {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &TensorUInt8Bit {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<TensorUInt8Bit> for ILearningModelFeatureValue {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorUInt8Bit) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TensorUInt8Bit> for ILearningModelFeatureValue {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorUInt8Bit) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureValue> for TensorUInt8Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureValue> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILearningModelFeatureValue> for &TensorUInt8Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ILearningModelFeatureValue> {
        ::core::convert::TryInto::<ILearningModelFeatureValue>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<TensorUInt8Bit> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorUInt8Bit) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&TensorUInt8Bit> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorUInt8Bit) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for TensorUInt8Bit {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for &TensorUInt8Bit {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::core::convert::TryInto::<super::super::Foundation::IMemoryBuffer>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<TensorUInt8Bit> for ITensor {
    type Error = ::windows::core::Error;
    fn try_from(value: TensorUInt8Bit) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TensorUInt8Bit> for ITensor {
    type Error = ::windows::core::Error;
    fn try_from(value: &TensorUInt8Bit) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITensor> for TensorUInt8Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ITensor> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITensor> for &TensorUInt8Bit {
    fn into_param(self) -> ::windows::core::Param<'a, ITensor> {
        ::core::convert::TryInto::<ITensor>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for TensorUInt8Bit {}
unsafe impl ::core::marker::Sync for TensorUInt8Bit {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
