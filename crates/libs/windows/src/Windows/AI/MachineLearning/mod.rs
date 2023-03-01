#[doc(hidden)]
#[repr(transparent)]
pub struct IImageFeatureDescriptor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IImageFeatureDescriptor {
    type Vtable = IImageFeatureDescriptor_Vtbl;
}
impl ::core::clone::Clone for IImageFeatureDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IImageFeatureDescriptor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x365585a5_171a_4a2a_985f_265159d3895a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageFeatureDescriptor_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
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
}
impl ::core::clone::Clone for IImageFeatureDescriptor2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IImageFeatureDescriptor2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b27cca7_d533_5862_bb98_1611b155b0e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageFeatureDescriptor2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PixelRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LearningModelPixelRange) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IImageFeatureValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IImageFeatureValue {
    type Vtable = IImageFeatureValue_Vtbl;
}
impl ::core::clone::Clone for IImageFeatureValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IImageFeatureValue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0414fd9_c9aa_4405_b7fb_94f87c8a3037);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageFeatureValue_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Media")]
    pub VideoFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media"))]
    VideoFrame: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IImageFeatureValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IImageFeatureValueStatics {
    type Vtable = IImageFeatureValueStatics_Vtbl;
}
impl ::core::clone::Clone for IImageFeatureValueStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IImageFeatureValueStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bc317fd_23cb_4610_b085_c8e1c87ebaa0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageFeatureValueStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Media")]
    pub CreateFromVideoFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, image: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media"))]
    CreateFromVideoFrame: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILearningModel(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILearningModel {
    type Vtable = ILearningModel_Vtbl;
}
impl ::core::clone::Clone for ILearningModel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILearningModel {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b8e4920_489f_4e86_9128_265a327b78fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModel_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Author: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Domain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Metadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Metadata: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub InputFeatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InputFeatures: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub OutputFeatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    OutputFeatures: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILearningModelBinding(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILearningModelBinding {
    type Vtable = ILearningModelBinding_Vtbl;
}
impl ::core::clone::Clone for ILearningModelBinding {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILearningModelBinding {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea312f20_168f_4f8c_94fe_2e7ac31b4aa8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelBinding_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Bind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub BindWithProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: *mut ::core::ffi::c_void, props: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BindWithProperties: usize,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILearningModelBindingFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILearningModelBindingFactory {
    type Vtable = ILearningModelBindingFactory_Vtbl;
}
impl ::core::clone::Clone for ILearningModelBindingFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILearningModelBindingFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc95f7a7a_e788_475e_8917_23aa381faf0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelBindingFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFromSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, session: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILearningModelDevice(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILearningModelDevice {
    type Vtable = ILearningModelDevice_Vtbl;
}
impl ::core::clone::Clone for ILearningModelDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILearningModelDevice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5c2c8fe_3f56_4a8c_ac5f_fdb92d8b8252);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelDevice_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics")]
    pub AdapterId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::DisplayAdapterId) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    AdapterId: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Direct3D11Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Direct3D11Device: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILearningModelDeviceFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILearningModelDeviceFactory {
    type Vtable = ILearningModelDeviceFactory_Vtbl;
}
impl ::core::clone::Clone for ILearningModelDeviceFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILearningModelDeviceFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9cffd74d_b1e5_4f20_80ad_0a56690db06b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelDeviceFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, devicekind: LearningModelDeviceKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILearningModelDeviceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILearningModelDeviceStatics {
    type Vtable = ILearningModelDeviceStatics_Vtbl;
}
impl ::core::clone::Clone for ILearningModelDeviceStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILearningModelDeviceStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49f32107_a8bf_42bb_92c7_10b12dc5d21f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelDeviceStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CreateFromDirect3D11Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CreateFromDirect3D11Device: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILearningModelEvaluationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILearningModelEvaluationResult {
    type Vtable = ILearningModelEvaluationResult_Vtbl;
}
impl ::core::clone::Clone for ILearningModelEvaluationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILearningModelEvaluationResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2f9bfcd_960e_49c0_8593_eb190ae3eee2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelEvaluationResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ErrorStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub Succeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Outputs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Outputs: usize,
}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct ILearningModelFeatureDescriptor(::windows::core::IUnknown);
impl ILearningModelFeatureDescriptor {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LearningModelFeatureKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsRequired(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsRequired)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(ILearningModelFeatureDescriptor, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::windows::core::RuntimeType for ILearningModelFeatureDescriptor {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{bc08cf7c-6ed0-4004-97ba-b9a2eecd2b4f}");
}
unsafe impl ::windows::core::Interface for ILearningModelFeatureDescriptor {
    type Vtable = ILearningModelFeatureDescriptor_Vtbl;
}
impl ::core::clone::Clone for ILearningModelFeatureDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILearningModelFeatureDescriptor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc08cf7c_6ed0_4004_97ba_b9a2eecd2b4f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelFeatureDescriptor_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LearningModelFeatureKind) -> ::windows::core::HRESULT,
    pub IsRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct ILearningModelFeatureValue(::windows::core::IUnknown);
impl ILearningModelFeatureValue {
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LearningModelFeatureKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(ILearningModelFeatureValue, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::windows::core::RuntimeType for ILearningModelFeatureValue {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{f51005db-4085-4dfe-9fed-95eb0c0cf75c}");
}
unsafe impl ::windows::core::Interface for ILearningModelFeatureValue {
    type Vtable = ILearningModelFeatureValue_Vtbl;
}
impl ::core::clone::Clone for ILearningModelFeatureValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILearningModelFeatureValue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf51005db_4085_4dfe_9fed_95eb0c0cf75c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelFeatureValue_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LearningModelFeatureKind) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct ILearningModelOperatorProvider(::windows::core::IUnknown);
impl ILearningModelOperatorProvider {}
::windows::imp::interface_hierarchy!(ILearningModelOperatorProvider, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::windows::core::RuntimeType for ILearningModelOperatorProvider {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{2a222e5d-afb1-47ed-bfad-b5b3a459ec04}");
}
unsafe impl ::windows::core::Interface for ILearningModelOperatorProvider {
    type Vtable = ILearningModelOperatorProvider_Vtbl;
}
impl ::core::clone::Clone for ILearningModelOperatorProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILearningModelOperatorProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a222e5d_afb1_47ed_bfad_b5b3a459ec04);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelOperatorProvider_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILearningModelSession(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILearningModelSession {
    type Vtable = ILearningModelSession_Vtbl;
}
impl ::core::clone::Clone for ILearningModelSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILearningModelSession {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8e58f8f6_b787_4c11_90f0_7129aeca74a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSession_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Model: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub EvaluationProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EvaluationProperties: usize,
    #[cfg(feature = "Foundation")]
    pub EvaluateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bindings: *mut ::core::ffi::c_void, correlationid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EvaluateAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub EvaluateFeaturesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, features: *mut ::core::ffi::c_void, correlationid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EvaluateFeaturesAsync: usize,
    pub Evaluate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bindings: *mut ::core::ffi::c_void, correlationid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub EvaluateFeatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, features: *mut ::core::ffi::c_void, correlationid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EvaluateFeatures: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILearningModelSessionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILearningModelSessionFactory {
    type Vtable = ILearningModelSessionFactory_Vtbl;
}
impl ::core::clone::Clone for ILearningModelSessionFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILearningModelSessionFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f6b881d_1c9b_47b6_bfe0_f1cf62a67579);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSessionFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFromModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, model: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateFromModelOnDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, model: *mut ::core::ffi::c_void, devicetorunon: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILearningModelSessionFactory2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILearningModelSessionFactory2 {
    type Vtable = ILearningModelSessionFactory2_Vtbl;
}
impl ::core::clone::Clone for ILearningModelSessionFactory2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILearningModelSessionFactory2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e5c88bf_0a1f_5fec_ade0_2fd91e4ef29b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSessionFactory2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFromModelOnDeviceWithSessionOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, model: *mut ::core::ffi::c_void, devicetorunon: *mut ::core::ffi::c_void, learningmodelsessionoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILearningModelSessionOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILearningModelSessionOptions {
    type Vtable = ILearningModelSessionOptions_Vtbl;
}
impl ::core::clone::Clone for ILearningModelSessionOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILearningModelSessionOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8f63fa1_134d_5133_8cff_3a5c3c263beb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSessionOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub BatchSizeOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetBatchSizeOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILearningModelSessionOptions2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILearningModelSessionOptions2 {
    type Vtable = ILearningModelSessionOptions2_Vtbl;
}
impl ::core::clone::Clone for ILearningModelSessionOptions2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILearningModelSessionOptions2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fcd1dc4_175f_5bd2_8de5_2f2006a25adf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSessionOptions2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CloseModelOnSessionCreation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCloseModelOnSessionCreation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILearningModelSessionOptions3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILearningModelSessionOptions3 {
    type Vtable = ILearningModelSessionOptions3_Vtbl;
}
impl ::core::clone::Clone for ILearningModelSessionOptions3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILearningModelSessionOptions3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58e15cee_d8c2_56fc_92e8_76d751081086);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSessionOptions3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub OverrideNamedDimension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, dimension: u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILearningModelStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILearningModelStatics {
    type Vtable = ILearningModelStatics_Vtbl;
}
impl ::core::clone::Clone for ILearningModelStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILearningModelStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3b977e8_6952_4e47_8ef4_1f7f07897c6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LoadFromStorageFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modelfile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LoadFromStorageFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub LoadFromStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modelstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    LoadFromStreamAsync: usize,
    pub LoadFromFilePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub LoadFromStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modelstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LoadFromStream: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LoadFromStorageFileWithOperatorProviderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modelfile: *mut ::core::ffi::c_void, operatorprovider: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LoadFromStorageFileWithOperatorProviderAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub LoadFromStreamWithOperatorProviderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modelstream: *mut ::core::ffi::c_void, operatorprovider: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    LoadFromStreamWithOperatorProviderAsync: usize,
    pub LoadFromFilePathWithOperatorProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::std::mem::MaybeUninit<::windows::core::HSTRING>, operatorprovider: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub LoadFromStreamWithOperatorProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modelstream: *mut ::core::ffi::c_void, operatorprovider: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LoadFromStreamWithOperatorProvider: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMapFeatureDescriptor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMapFeatureDescriptor {
    type Vtable = IMapFeatureDescriptor_Vtbl;
}
impl ::core::clone::Clone for IMapFeatureDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMapFeatureDescriptor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x530424bd_a257_436d_9e60_c2981f7cc5c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapFeatureDescriptor_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub KeyKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TensorKind) -> ::windows::core::HRESULT,
    pub ValueDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISequenceFeatureDescriptor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISequenceFeatureDescriptor {
    type Vtable = ISequenceFeatureDescriptor_Vtbl;
}
impl ::core::clone::Clone for ISequenceFeatureDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISequenceFeatureDescriptor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84f6945a_562b_4d62_a851_739aced96668);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISequenceFeatureDescriptor_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ElementDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct ITensor(::windows::core::IUnknown);
impl ITensor {
    pub fn TensorKind(&self) -> ::windows::core::Result<TensorKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorKind>();
            (::windows::core::Interface::vtable(this).TensorKind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<i64>>();
            (::windows::core::Interface::vtable(this).Shape)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::ComInterface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LearningModelFeatureKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(ITensor, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<ILearningModelFeatureValue> for ITensor {}
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
impl ::windows::core::RuntimeType for ITensor {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{05489593-a305-4a25-ad09-440119b4b7f6}");
}
unsafe impl ::windows::core::Interface for ITensor {
    type Vtable = ITensor_Vtbl;
}
impl ::core::clone::Clone for ITensor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05489593_a305_4a25_ad09_440119b4b7f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensor_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TensorKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TensorKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Shape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Shape: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorBoolean(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorBoolean {
    type Vtable = ITensorBoolean_Vtbl;
}
impl ::core::clone::Clone for ITensorBoolean {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorBoolean {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50f311ed_29e9_4a5c_a44d_8fc512584eed);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorBoolean_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorBooleanStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorBooleanStatics {
    type Vtable = ITensorBooleanStatics_Vtbl;
}
impl ::core::clone::Clone for ITensorBooleanStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorBooleanStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2796862c_2357_49a7_b476_d0aa3dfe6866);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorBooleanStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorBooleanStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorBooleanStatics2 {
    type Vtable = ITensorBooleanStatics2_Vtbl;
}
impl ::core::clone::Clone for ITensorBooleanStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorBooleanStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3a4a501_6a2d_52d7_b04b_c435baee0115);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorBooleanStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorDouble(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorDouble {
    type Vtable = ITensorDouble_Vtbl;
}
impl ::core::clone::Clone for ITensorDouble {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorDouble {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91e41252_7a8f_4f0e_a28f_9637ffc8a3d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorDouble_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorDoubleStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorDoubleStatics {
    type Vtable = ITensorDoubleStatics_Vtbl;
}
impl ::core::clone::Clone for ITensorDoubleStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorDoubleStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa86693c5_9538_44e7_a3ca_5df374a5a70c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorDoubleStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorDoubleStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorDoubleStatics2 {
    type Vtable = ITensorDoubleStatics2_Vtbl;
}
impl ::core::clone::Clone for ITensorDoubleStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorDoubleStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93a570de_5e9a_5094_85c8_592c655e68ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorDoubleStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorFeatureDescriptor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorFeatureDescriptor {
    type Vtable = ITensorFeatureDescriptor_Vtbl;
}
impl ::core::clone::Clone for ITensorFeatureDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorFeatureDescriptor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74455c80_946a_4310_a19c_ee0af028fce4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorFeatureDescriptor_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TensorKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TensorKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Shape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Shape: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorFloat(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorFloat {
    type Vtable = ITensorFloat_Vtbl;
}
impl ::core::clone::Clone for ITensorFloat {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorFloat {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2282d82_aa02_42c8_a0c8_df1efc9676e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorFloat_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorFloat16Bit(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorFloat16Bit {
    type Vtable = ITensorFloat16Bit_Vtbl;
}
impl ::core::clone::Clone for ITensorFloat16Bit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorFloat16Bit {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ab994fc_5b89_4c3c_b5e4_5282a5316c0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorFloat16Bit_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorFloat16BitStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorFloat16BitStatics {
    type Vtable = ITensorFloat16BitStatics_Vtbl;
}
impl ::core::clone::Clone for ITensorFloat16BitStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorFloat16BitStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa52db6f5_318a_44d4_820b_0cdc7054a84a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorFloat16BitStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorFloat16BitStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorFloat16BitStatics2 {
    type Vtable = ITensorFloat16BitStatics2_Vtbl;
}
impl ::core::clone::Clone for ITensorFloat16BitStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorFloat16BitStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68545726_2dc7_51bf_b470_0b344cc2a1bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorFloat16BitStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorFloatStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorFloatStatics {
    type Vtable = ITensorFloatStatics_Vtbl;
}
impl ::core::clone::Clone for ITensorFloatStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorFloatStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdbcd395b_3ba3_452f_b10d_3c135e573fa9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorFloatStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorFloatStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorFloatStatics2 {
    type Vtable = ITensorFloatStatics2_Vtbl;
}
impl ::core::clone::Clone for ITensorFloatStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorFloatStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24610bc1_5e44_5713_b281_8f4ad4d555e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorFloatStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorInt16Bit(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorInt16Bit {
    type Vtable = ITensorInt16Bit_Vtbl;
}
impl ::core::clone::Clone for ITensorInt16Bit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorInt16Bit {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98a32d39_e6d6_44af_8afa_baebc44dc020);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt16Bit_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorInt16BitStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorInt16BitStatics {
    type Vtable = ITensorInt16BitStatics_Vtbl;
}
impl ::core::clone::Clone for ITensorInt16BitStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorInt16BitStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98646293_266e_4b1a_821f_e60d70898b91);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt16BitStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const i16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorInt16BitStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorInt16BitStatics2 {
    type Vtable = ITensorInt16BitStatics2_Vtbl;
}
impl ::core::clone::Clone for ITensorInt16BitStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorInt16BitStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cd70cf4_696c_5e5f_95d8_5ebf9670148b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt16BitStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const i16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorInt32Bit(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorInt32Bit {
    type Vtable = ITensorInt32Bit_Vtbl;
}
impl ::core::clone::Clone for ITensorInt32Bit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorInt32Bit {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c0c28d3_207c_4486_a7d2_884522c5e589);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt32Bit_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorInt32BitStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorInt32BitStatics {
    type Vtable = ITensorInt32BitStatics_Vtbl;
}
impl ::core::clone::Clone for ITensorInt32BitStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorInt32BitStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6539864b_52fa_4e35_907c_834cac417b50);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt32BitStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorInt32BitStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorInt32BitStatics2 {
    type Vtable = ITensorInt32BitStatics2_Vtbl;
}
impl ::core::clone::Clone for ITensorInt32BitStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorInt32BitStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c4b079a_e956_5ce0_a3bd_157d9d79b5ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt32BitStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorInt64Bit(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorInt64Bit {
    type Vtable = ITensorInt64Bit_Vtbl;
}
impl ::core::clone::Clone for ITensorInt64Bit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorInt64Bit {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x499665ba_1fa2_45ad_af25_a0bd9bda4c87);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt64Bit_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorInt64BitStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorInt64BitStatics {
    type Vtable = ITensorInt64BitStatics_Vtbl;
}
impl ::core::clone::Clone for ITensorInt64BitStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorInt64BitStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9648ad9d_1198_4d74_9517_783ab62b9cc2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt64BitStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const i64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorInt64BitStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorInt64BitStatics2 {
    type Vtable = ITensorInt64BitStatics2_Vtbl;
}
impl ::core::clone::Clone for ITensorInt64BitStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorInt64BitStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d3d9dcb_ff40_5ec2_89fe_084e2b6bc6db);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt64BitStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const i64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorInt8Bit(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorInt8Bit {
    type Vtable = ITensorInt8Bit_Vtbl;
}
impl ::core::clone::Clone for ITensorInt8Bit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorInt8Bit {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcddd97c5_ffd8_4fef_aefb_30e1a485b2ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt8Bit_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorInt8BitStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorInt8BitStatics {
    type Vtable = ITensorInt8BitStatics_Vtbl;
}
impl ::core::clone::Clone for ITensorInt8BitStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorInt8BitStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1a12284_095c_4c76_a661_ac4cee1f3e8b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt8BitStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorInt8BitStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorInt8BitStatics2 {
    type Vtable = ITensorInt8BitStatics2_Vtbl;
}
impl ::core::clone::Clone for ITensorInt8BitStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorInt8BitStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0d59637_c468_56fb_9535_c052bdb93dc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt8BitStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorString(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorString {
    type Vtable = ITensorString_Vtbl;
}
impl ::core::clone::Clone for ITensorString {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorString {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x582335c8_bdb1_4610_bc75_35e9cbf009b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorString_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorStringStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorStringStatics {
    type Vtable = ITensorStringStatics_Vtbl;
}
impl ::core::clone::Clone for ITensorStringStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorStringStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83623324_cf26_4f17_a2d4_20ef8d097d53);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorStringStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorStringStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorStringStatics2 {
    type Vtable = ITensorStringStatics2_Vtbl;
}
impl ::core::clone::Clone for ITensorStringStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorStringStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e355ed0_c8e2_5254_9137_0193a3668fd8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorStringStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorUInt16Bit(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorUInt16Bit {
    type Vtable = ITensorUInt16Bit_Vtbl;
}
impl ::core::clone::Clone for ITensorUInt16Bit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorUInt16Bit {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68140f4b_23c0_42f3_81f6_a891c011bc3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt16Bit_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorUInt16BitStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorUInt16BitStatics {
    type Vtable = ITensorUInt16BitStatics_Vtbl;
}
impl ::core::clone::Clone for ITensorUInt16BitStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorUInt16BitStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5df745dd_028a_481a_a27c_c7e6435e52dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt16BitStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorUInt16BitStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorUInt16BitStatics2 {
    type Vtable = ITensorUInt16BitStatics2_Vtbl;
}
impl ::core::clone::Clone for ITensorUInt16BitStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorUInt16BitStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8af40c64_d69f_5315_9348_490877bbd642);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt16BitStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorUInt32Bit(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorUInt32Bit {
    type Vtable = ITensorUInt32Bit_Vtbl;
}
impl ::core::clone::Clone for ITensorUInt32Bit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorUInt32Bit {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8c9c2ff_7511_45a3_bfac_c38f370d2237);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt32Bit_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorUInt32BitStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorUInt32BitStatics {
    type Vtable = ITensorUInt32BitStatics_Vtbl;
}
impl ::core::clone::Clone for ITensorUInt32BitStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorUInt32BitStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x417c3837_e773_4378_8e7f_0cc33dbea697);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt32BitStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorUInt32BitStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorUInt32BitStatics2 {
    type Vtable = ITensorUInt32BitStatics2_Vtbl;
}
impl ::core::clone::Clone for ITensorUInt32BitStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorUInt32BitStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef1a1f1c_314e_569d_b496_5c8447d20cd2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt32BitStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorUInt64Bit(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorUInt64Bit {
    type Vtable = ITensorUInt64Bit_Vtbl;
}
impl ::core::clone::Clone for ITensorUInt64Bit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorUInt64Bit {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e70ffad_04bf_4825_839a_82baef8c7886);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt64Bit_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorUInt64BitStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorUInt64BitStatics {
    type Vtable = ITensorUInt64BitStatics_Vtbl;
}
impl ::core::clone::Clone for ITensorUInt64BitStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorUInt64BitStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a7e20eb_242f_47cb_a9c6_f602ecfbfee4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt64BitStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorUInt64BitStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorUInt64BitStatics2 {
    type Vtable = ITensorUInt64BitStatics2_Vtbl;
}
impl ::core::clone::Clone for ITensorUInt64BitStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorUInt64BitStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x085a687d_67e1_5b1e_b232_4fabe9ca20b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt64BitStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorUInt8Bit(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorUInt8Bit {
    type Vtable = ITensorUInt8Bit_Vtbl;
}
impl ::core::clone::Clone for ITensorUInt8Bit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorUInt8Bit {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58e1ae27_622b_48e3_be22_d867aed1daac);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt8Bit_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorUInt8BitStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorUInt8BitStatics {
    type Vtable = ITensorUInt8BitStatics_Vtbl;
}
impl ::core::clone::Clone for ITensorUInt8BitStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorUInt8BitStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05f67583_bc24_4220_8a41_2dcd8c5ed33c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt8BitStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITensorUInt8BitStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITensorUInt8BitStatics2 {
    type Vtable = ITensorUInt8BitStatics2_Vtbl;
}
impl ::core::clone::Clone for ITensorUInt8BitStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITensorUInt8BitStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ba042d6_373e_5a3a_a2fc_a6c41bd52789);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt8BitStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct ImageFeatureDescriptor(::windows::core::IUnknown);
impl ImageFeatureDescriptor {
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn BitmapPixelFormat(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Graphics::Imaging::BitmapPixelFormat>();
            (::windows::core::Interface::vtable(this).BitmapPixelFormat)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn BitmapAlphaMode(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapAlphaMode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Graphics::Imaging::BitmapAlphaMode>();
            (::windows::core::Interface::vtable(this).BitmapAlphaMode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Width(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Width)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Height(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Height)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PixelRange(&self) -> ::windows::core::Result<LearningModelPixelRange> {
        let this = &::windows::core::ComInterface::cast::<IImageFeatureDescriptor2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LearningModelPixelRange>();
            (::windows::core::Interface::vtable(this).PixelRange)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::ComInterface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LearningModelFeatureKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsRequired(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsRequired)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for ImageFeatureDescriptor {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.ImageFeatureDescriptor;{365585a5-171a-4a2a-985f-265159d3895a})");
}
impl ::core::clone::Clone for ImageFeatureDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ImageFeatureDescriptor {
    type Vtable = IImageFeatureDescriptor_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ImageFeatureDescriptor {
    const IID: ::windows::core::GUID = <IImageFeatureDescriptor as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ImageFeatureDescriptor {
    const NAME: &'static str = "Windows.AI.MachineLearning.ImageFeatureDescriptor";
}
::windows::imp::interface_hierarchy!(ImageFeatureDescriptor, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<ILearningModelFeatureDescriptor> for ImageFeatureDescriptor {}
unsafe impl ::core::marker::Send for ImageFeatureDescriptor {}
unsafe impl ::core::marker::Sync for ImageFeatureDescriptor {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct ImageFeatureValue(::windows::core::IUnknown);
impl ImageFeatureValue {
    #[doc = "*Required features: `\"Media\"`*"]
    #[cfg(feature = "Media")]
    pub fn VideoFrame(&self) -> ::windows::core::Result<super::super::Media::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Media::VideoFrame>();
            (::windows::core::Interface::vtable(this).VideoFrame)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    #[cfg(feature = "Media")]
    pub fn CreateFromVideoFrame(image: &super::super::Media::VideoFrame) -> ::windows::core::Result<ImageFeatureValue> {
        Self::IImageFeatureValueStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<ImageFeatureValue>();
            (::windows::core::Interface::vtable(this).CreateFromVideoFrame)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(image), &mut result__).from_abi(result__)
        })
    }
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::ComInterface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LearningModelFeatureKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IImageFeatureValueStatics<R, F: FnOnce(&IImageFeatureValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<ImageFeatureValue, IImageFeatureValueStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for ImageFeatureValue {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.ImageFeatureValue;{f0414fd9-c9aa-4405-b7fb-94f87c8a3037})");
}
impl ::core::clone::Clone for ImageFeatureValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ImageFeatureValue {
    type Vtable = IImageFeatureValue_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ImageFeatureValue {
    const IID: ::windows::core::GUID = <IImageFeatureValue as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ImageFeatureValue {
    const NAME: &'static str = "Windows.AI.MachineLearning.ImageFeatureValue";
}
::windows::imp::interface_hierarchy!(ImageFeatureValue, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<ILearningModelFeatureValue> for ImageFeatureValue {}
unsafe impl ::core::marker::Send for ImageFeatureValue {}
unsafe impl ::core::marker::Sync for ImageFeatureValue {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct LearningModel(::windows::core::IUnknown);
impl LearningModel {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Author(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Author)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Domain(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Domain)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Version(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i64>();
            (::windows::core::Interface::vtable(this).Version)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Metadata(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).Metadata)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InputFeatures(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ILearningModelFeatureDescriptor>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<ILearningModelFeatureDescriptor>>();
            (::windows::core::Interface::vtable(this).InputFeatures)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutputFeatures(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ILearningModelFeatureDescriptor>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<ILearningModelFeatureDescriptor>>();
            (::windows::core::Interface::vtable(this).OutputFeatures)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LoadFromStorageFileAsync<P0>(modelfile: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LearningModel>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::IStorageFile>,
    {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<LearningModel>>();
            (::windows::core::Interface::vtable(this).LoadFromStorageFileAsync)(::windows::core::Interface::as_raw(this), modelfile.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn LoadFromStreamAsync<P0>(modelstream: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LearningModel>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<LearningModel>>();
            (::windows::core::Interface::vtable(this).LoadFromStreamAsync)(::windows::core::Interface::as_raw(this), modelstream.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn LoadFromFilePath(filepath: &::windows::core::HSTRING) -> ::windows::core::Result<LearningModel> {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<LearningModel>();
            (::windows::core::Interface::vtable(this).LoadFromFilePath)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(filepath), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadFromStream<P0>(modelstream: P0) -> ::windows::core::Result<LearningModel>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<LearningModel>();
            (::windows::core::Interface::vtable(this).LoadFromStream)(::windows::core::Interface::as_raw(this), modelstream.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LoadFromStorageFileWithOperatorProviderAsync<P0, P1>(modelfile: P0, operatorprovider: P1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LearningModel>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::IStorageFile>,
        P1: ::windows::core::TryIntoParam<ILearningModelOperatorProvider>,
    {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<LearningModel>>();
            (::windows::core::Interface::vtable(this).LoadFromStorageFileWithOperatorProviderAsync)(::windows::core::Interface::as_raw(this), modelfile.try_into_param()?.abi(), operatorprovider.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn LoadFromStreamWithOperatorProviderAsync<P0, P1>(modelstream: P0, operatorprovider: P1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LearningModel>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStreamReference>,
        P1: ::windows::core::TryIntoParam<ILearningModelOperatorProvider>,
    {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<LearningModel>>();
            (::windows::core::Interface::vtable(this).LoadFromStreamWithOperatorProviderAsync)(::windows::core::Interface::as_raw(this), modelstream.try_into_param()?.abi(), operatorprovider.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn LoadFromFilePathWithOperatorProvider<P0>(filepath: &::windows::core::HSTRING, operatorprovider: P0) -> ::windows::core::Result<LearningModel>
    where
        P0: ::windows::core::TryIntoParam<ILearningModelOperatorProvider>,
    {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<LearningModel>();
            (::windows::core::Interface::vtable(this).LoadFromFilePathWithOperatorProvider)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(filepath), operatorprovider.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadFromStreamWithOperatorProvider<P0, P1>(modelstream: P0, operatorprovider: P1) -> ::windows::core::Result<LearningModel>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStreamReference>,
        P1: ::windows::core::TryIntoParam<ILearningModelOperatorProvider>,
    {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<LearningModel>();
            (::windows::core::Interface::vtable(this).LoadFromStreamWithOperatorProvider)(::windows::core::Interface::as_raw(this), modelstream.try_into_param()?.abi(), operatorprovider.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILearningModelStatics<R, F: FnOnce(&ILearningModelStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<LearningModel, ILearningModelStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for LearningModel {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.LearningModel;{5b8e4920-489f-4e86-9128-265a327b78fa})");
}
impl ::core::clone::Clone for LearningModel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for LearningModel {
    type Vtable = ILearningModel_Vtbl;
}
unsafe impl ::windows::core::ComInterface for LearningModel {
    const IID: ::windows::core::GUID = <ILearningModel as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for LearningModel {
    const NAME: &'static str = "Windows.AI.MachineLearning.LearningModel";
}
::windows::imp::interface_hierarchy!(LearningModel, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for LearningModel {}
unsafe impl ::core::marker::Send for LearningModel {}
unsafe impl ::core::marker::Sync for LearningModel {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct LearningModelBinding(::windows::core::IUnknown);
impl LearningModelBinding {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>();
            (::windows::core::Interface::vtable(this).First)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Bind<P0>(&self, name: &::windows::core::HSTRING, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Bind)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn BindWithProperties<P0, P1>(&self, name: &::windows::core::HSTRING, value: P0, props: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
        P1: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IPropertySet>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).BindWithProperties)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.into_param().abi(), props.try_into_param()?.abi()).ok() }
    }
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Clear)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn CreateFromSession(session: &LearningModelSession) -> ::windows::core::Result<LearningModelBinding> {
        Self::ILearningModelBindingFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<LearningModelBinding>();
            (::windows::core::Interface::vtable(this).CreateFromSession)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(session), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Lookup)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(key), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Size)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).HasKey)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(key), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Split(&self, first: &mut ::core::option::Option<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>, second: &mut ::core::option::Option<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Split)(::windows::core::Interface::as_raw(this), first as *mut _ as _, second as *mut _ as _).ok() }
    }
    #[doc(hidden)]
    pub fn ILearningModelBindingFactory<R, F: FnOnce(&ILearningModelBindingFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<LearningModelBinding, ILearningModelBindingFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for LearningModelBinding {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.LearningModelBinding;{ea312f20-168f-4f8c-94fe-2e7ac31b4aa8})");
}
impl ::core::clone::Clone for LearningModelBinding {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for LearningModelBinding {
    type Vtable = ILearningModelBinding_Vtbl;
}
unsafe impl ::windows::core::ComInterface for LearningModelBinding {
    const IID: ::windows::core::GUID = <ILearningModelBinding as ::windows::core::ComInterface>::IID;
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
::windows::imp::interface_hierarchy!(LearningModelBinding, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::CanTryInto<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> for LearningModelBinding {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::CanTryInto<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> for LearningModelBinding {}
unsafe impl ::core::marker::Send for LearningModelBinding {}
unsafe impl ::core::marker::Sync for LearningModelBinding {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct LearningModelDevice(::windows::core::IUnknown);
impl LearningModelDevice {
    #[doc = "*Required features: `\"Graphics\"`*"]
    #[cfg(feature = "Graphics")]
    pub fn AdapterId(&self) -> ::windows::core::Result<super::super::Graphics::DisplayAdapterId> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Graphics::DisplayAdapterId>();
            (::windows::core::Interface::vtable(this).AdapterId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Direct3D11Device(&self) -> ::windows::core::Result<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>();
            (::windows::core::Interface::vtable(this).Direct3D11Device)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(devicekind: LearningModelDeviceKind) -> ::windows::core::Result<LearningModelDevice> {
        Self::ILearningModelDeviceFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<LearningModelDevice>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), devicekind, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CreateFromDirect3D11Device<P0>(device: P0) -> ::windows::core::Result<LearningModelDevice>
    where
        P0: ::windows::core::TryIntoParam<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>,
    {
        Self::ILearningModelDeviceStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<LearningModelDevice>();
            (::windows::core::Interface::vtable(this).CreateFromDirect3D11Device)(::windows::core::Interface::as_raw(this), device.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILearningModelDeviceFactory<R, F: FnOnce(&ILearningModelDeviceFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<LearningModelDevice, ILearningModelDeviceFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ILearningModelDeviceStatics<R, F: FnOnce(&ILearningModelDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<LearningModelDevice, ILearningModelDeviceStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for LearningModelDevice {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.LearningModelDevice;{f5c2c8fe-3f56-4a8c-ac5f-fdb92d8b8252})");
}
impl ::core::clone::Clone for LearningModelDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for LearningModelDevice {
    type Vtable = ILearningModelDevice_Vtbl;
}
unsafe impl ::windows::core::ComInterface for LearningModelDevice {
    const IID: ::windows::core::GUID = <ILearningModelDevice as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for LearningModelDevice {
    const NAME: &'static str = "Windows.AI.MachineLearning.LearningModelDevice";
}
::windows::imp::interface_hierarchy!(LearningModelDevice, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for LearningModelDevice {}
unsafe impl ::core::marker::Sync for LearningModelDevice {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct LearningModelEvaluationResult(::windows::core::IUnknown);
impl LearningModelEvaluationResult {
    pub fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).CorrelationId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ErrorStatus(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).ErrorStatus)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Succeeded(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Succeeded)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Outputs(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>();
            (::windows::core::Interface::vtable(this).Outputs)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for LearningModelEvaluationResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.LearningModelEvaluationResult;{b2f9bfcd-960e-49c0-8593-eb190ae3eee2})");
}
impl ::core::clone::Clone for LearningModelEvaluationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for LearningModelEvaluationResult {
    type Vtable = ILearningModelEvaluationResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for LearningModelEvaluationResult {
    const IID: ::windows::core::GUID = <ILearningModelEvaluationResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for LearningModelEvaluationResult {
    const NAME: &'static str = "Windows.AI.MachineLearning.LearningModelEvaluationResult";
}
::windows::imp::interface_hierarchy!(LearningModelEvaluationResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for LearningModelEvaluationResult {}
unsafe impl ::core::marker::Sync for LearningModelEvaluationResult {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct LearningModelSession(::windows::core::IUnknown);
impl LearningModelSession {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Model(&self) -> ::windows::core::Result<LearningModel> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LearningModel>();
            (::windows::core::Interface::vtable(this).Model)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Device(&self) -> ::windows::core::Result<LearningModelDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LearningModelDevice>();
            (::windows::core::Interface::vtable(this).Device)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn EvaluationProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IPropertySet>();
            (::windows::core::Interface::vtable(this).EvaluationProperties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EvaluateAsync(&self, bindings: &LearningModelBinding, correlationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LearningModelEvaluationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<LearningModelEvaluationResult>>();
            (::windows::core::Interface::vtable(this).EvaluateAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(bindings), ::core::mem::transmute_copy(correlationid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn EvaluateFeaturesAsync<P0>(&self, features: P0, correlationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LearningModelEvaluationResult>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<LearningModelEvaluationResult>>();
            (::windows::core::Interface::vtable(this).EvaluateFeaturesAsync)(::windows::core::Interface::as_raw(this), features.try_into_param()?.abi(), ::core::mem::transmute_copy(correlationid), &mut result__).from_abi(result__)
        }
    }
    pub fn Evaluate(&self, bindings: &LearningModelBinding, correlationid: &::windows::core::HSTRING) -> ::windows::core::Result<LearningModelEvaluationResult> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LearningModelEvaluationResult>();
            (::windows::core::Interface::vtable(this).Evaluate)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(bindings), ::core::mem::transmute_copy(correlationid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn EvaluateFeatures<P0>(&self, features: P0, correlationid: &::windows::core::HSTRING) -> ::windows::core::Result<LearningModelEvaluationResult>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LearningModelEvaluationResult>();
            (::windows::core::Interface::vtable(this).EvaluateFeatures)(::windows::core::Interface::as_raw(this), features.try_into_param()?.abi(), ::core::mem::transmute_copy(correlationid), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateFromModel(model: &LearningModel) -> ::windows::core::Result<LearningModelSession> {
        Self::ILearningModelSessionFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<LearningModelSession>();
            (::windows::core::Interface::vtable(this).CreateFromModel)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(model), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromModelOnDevice(model: &LearningModel, devicetorunon: &LearningModelDevice) -> ::windows::core::Result<LearningModelSession> {
        Self::ILearningModelSessionFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<LearningModelSession>();
            (::windows::core::Interface::vtable(this).CreateFromModelOnDevice)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(model), ::core::mem::transmute_copy(devicetorunon), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromModelOnDeviceWithSessionOptions(model: &LearningModel, devicetorunon: &LearningModelDevice, learningmodelsessionoptions: &LearningModelSessionOptions) -> ::windows::core::Result<LearningModelSession> {
        Self::ILearningModelSessionFactory2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<LearningModelSession>();
            (::windows::core::Interface::vtable(this).CreateFromModelOnDeviceWithSessionOptions)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(model), ::core::mem::transmute_copy(devicetorunon), ::core::mem::transmute_copy(learningmodelsessionoptions), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILearningModelSessionFactory<R, F: FnOnce(&ILearningModelSessionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<LearningModelSession, ILearningModelSessionFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ILearningModelSessionFactory2<R, F: FnOnce(&ILearningModelSessionFactory2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<LearningModelSession, ILearningModelSessionFactory2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for LearningModelSession {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.LearningModelSession;{8e58f8f6-b787-4c11-90f0-7129aeca74a9})");
}
impl ::core::clone::Clone for LearningModelSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for LearningModelSession {
    type Vtable = ILearningModelSession_Vtbl;
}
unsafe impl ::windows::core::ComInterface for LearningModelSession {
    const IID: ::windows::core::GUID = <ILearningModelSession as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for LearningModelSession {
    const NAME: &'static str = "Windows.AI.MachineLearning.LearningModelSession";
}
::windows::imp::interface_hierarchy!(LearningModelSession, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for LearningModelSession {}
unsafe impl ::core::marker::Send for LearningModelSession {}
unsafe impl ::core::marker::Sync for LearningModelSession {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct LearningModelSessionOptions(::windows::core::IUnknown);
impl LearningModelSessionOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<LearningModelSessionOptions, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn BatchSizeOverride(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).BatchSizeOverride)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBatchSizeOverride(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBatchSizeOverride)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn CloseModelOnSessionCreation(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<ILearningModelSessionOptions2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).CloseModelOnSessionCreation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCloseModelOnSessionCreation(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ILearningModelSessionOptions2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCloseModelOnSessionCreation)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn OverrideNamedDimension(&self, name: &::windows::core::HSTRING, dimension: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ILearningModelSessionOptions3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).OverrideNamedDimension)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), dimension).ok() }
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
impl ::windows::core::RuntimeType for LearningModelSessionOptions {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.LearningModelSessionOptions;{b8f63fa1-134d-5133-8cff-3a5c3c263beb})");
}
impl ::core::clone::Clone for LearningModelSessionOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for LearningModelSessionOptions {
    type Vtable = ILearningModelSessionOptions_Vtbl;
}
unsafe impl ::windows::core::ComInterface for LearningModelSessionOptions {
    const IID: ::windows::core::GUID = <ILearningModelSessionOptions as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for LearningModelSessionOptions {
    const NAME: &'static str = "Windows.AI.MachineLearning.LearningModelSessionOptions";
}
::windows::imp::interface_hierarchy!(LearningModelSessionOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for LearningModelSessionOptions {}
unsafe impl ::core::marker::Sync for LearningModelSessionOptions {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct MapFeatureDescriptor(::windows::core::IUnknown);
impl MapFeatureDescriptor {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::ComInterface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LearningModelFeatureKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsRequired(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsRequired)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn KeyKind(&self) -> ::windows::core::Result<TensorKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorKind>();
            (::windows::core::Interface::vtable(this).KeyKind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ValueDescriptor(&self) -> ::windows::core::Result<ILearningModelFeatureDescriptor> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ILearningModelFeatureDescriptor>();
            (::windows::core::Interface::vtable(this).ValueDescriptor)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for MapFeatureDescriptor {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.MapFeatureDescriptor;{530424bd-a257-436d-9e60-c2981f7cc5c4})");
}
impl ::core::clone::Clone for MapFeatureDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for MapFeatureDescriptor {
    type Vtable = IMapFeatureDescriptor_Vtbl;
}
unsafe impl ::windows::core::ComInterface for MapFeatureDescriptor {
    const IID: ::windows::core::GUID = <IMapFeatureDescriptor as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for MapFeatureDescriptor {
    const NAME: &'static str = "Windows.AI.MachineLearning.MapFeatureDescriptor";
}
::windows::imp::interface_hierarchy!(MapFeatureDescriptor, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<ILearningModelFeatureDescriptor> for MapFeatureDescriptor {}
unsafe impl ::core::marker::Send for MapFeatureDescriptor {}
unsafe impl ::core::marker::Sync for MapFeatureDescriptor {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct SequenceFeatureDescriptor(::windows::core::IUnknown);
impl SequenceFeatureDescriptor {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::ComInterface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LearningModelFeatureKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsRequired(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsRequired)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ElementDescriptor(&self) -> ::windows::core::Result<ILearningModelFeatureDescriptor> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ILearningModelFeatureDescriptor>();
            (::windows::core::Interface::vtable(this).ElementDescriptor)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for SequenceFeatureDescriptor {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.SequenceFeatureDescriptor;{84f6945a-562b-4d62-a851-739aced96668})");
}
impl ::core::clone::Clone for SequenceFeatureDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SequenceFeatureDescriptor {
    type Vtable = ISequenceFeatureDescriptor_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SequenceFeatureDescriptor {
    const IID: ::windows::core::GUID = <ISequenceFeatureDescriptor as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SequenceFeatureDescriptor {
    const NAME: &'static str = "Windows.AI.MachineLearning.SequenceFeatureDescriptor";
}
::windows::imp::interface_hierarchy!(SequenceFeatureDescriptor, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<ILearningModelFeatureDescriptor> for SequenceFeatureDescriptor {}
unsafe impl ::core::marker::Send for SequenceFeatureDescriptor {}
unsafe impl ::core::marker::Sync for SequenceFeatureDescriptor {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct TensorBoolean(::windows::core::IUnknown);
impl TensorBoolean {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::ComInterface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LearningModelFeatureKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows::core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IMemoryBufferReference>();
            (::windows::core::Interface::vtable(this).CreateReference)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TensorKind(&self) -> ::windows::core::Result<TensorKind> {
        let this = &::windows::core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorKind>();
            (::windows::core::Interface::vtable(this).TensorKind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<i64>>();
            (::windows::core::Interface::vtable(this).Shape)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<bool>>();
            (::windows::core::Interface::vtable(this).GetAsVectorView)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create() -> ::windows::core::Result<TensorBoolean> {
        Self::ITensorBooleanStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorBoolean>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> ::windows::core::Result<TensorBoolean>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorBooleanStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorBoolean>();
            (::windows::core::Interface::vtable(this).Create2)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[bool]) -> ::windows::core::Result<TensorBoolean>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorBooleanStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorBoolean>();
            (::windows::core::Interface::vtable(this).CreateFromArray)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.len() as u32, data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> ::windows::core::Result<TensorBoolean>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
        P1: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<bool>>,
    {
        Self::ITensorBooleanStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorBoolean>();
            (::windows::core::Interface::vtable(this).CreateFromIterable)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[bool]) -> ::windows::core::Result<TensorBoolean> {
        Self::ITensorBooleanStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorBoolean>();
            (::windows::core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::windows::core::Interface::as_raw(this), shape.len() as u32, shape.as_ptr(), data.len() as u32, data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(shape: &[i64], buffer: P0) -> ::windows::core::Result<TensorBoolean>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorBooleanStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorBoolean>();
            (::windows::core::Interface::vtable(this).CreateFromBuffer)(::windows::core::Interface::as_raw(this), shape.len() as u32, shape.as_ptr(), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorBooleanStatics<R, F: FnOnce(&ITensorBooleanStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<TensorBoolean, ITensorBooleanStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorBooleanStatics2<R, F: FnOnce(&ITensorBooleanStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<TensorBoolean, ITensorBooleanStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for TensorBoolean {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorBoolean;{50f311ed-29e9-4a5c-a44d-8fc512584eed})");
}
impl ::core::clone::Clone for TensorBoolean {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for TensorBoolean {
    type Vtable = ITensorBoolean_Vtbl;
}
unsafe impl ::windows::core::ComInterface for TensorBoolean {
    const IID: ::windows::core::GUID = <ITensorBoolean as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for TensorBoolean {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorBoolean";
}
::windows::imp::interface_hierarchy!(TensorBoolean, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for TensorBoolean {}
impl ::windows::core::CanTryInto<ILearningModelFeatureValue> for TensorBoolean {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IMemoryBuffer> for TensorBoolean {}
impl ::windows::core::CanTryInto<ITensor> for TensorBoolean {}
unsafe impl ::core::marker::Send for TensorBoolean {}
unsafe impl ::core::marker::Sync for TensorBoolean {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct TensorDouble(::windows::core::IUnknown);
impl TensorDouble {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::ComInterface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LearningModelFeatureKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows::core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IMemoryBufferReference>();
            (::windows::core::Interface::vtable(this).CreateReference)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TensorKind(&self) -> ::windows::core::Result<TensorKind> {
        let this = &::windows::core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorKind>();
            (::windows::core::Interface::vtable(this).TensorKind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<i64>>();
            (::windows::core::Interface::vtable(this).Shape)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<f64>>();
            (::windows::core::Interface::vtable(this).GetAsVectorView)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create() -> ::windows::core::Result<TensorDouble> {
        Self::ITensorDoubleStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorDouble>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> ::windows::core::Result<TensorDouble>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorDoubleStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorDouble>();
            (::windows::core::Interface::vtable(this).Create2)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[f64]) -> ::windows::core::Result<TensorDouble>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorDoubleStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorDouble>();
            (::windows::core::Interface::vtable(this).CreateFromArray)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.len() as u32, data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> ::windows::core::Result<TensorDouble>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
        P1: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<f64>>,
    {
        Self::ITensorDoubleStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorDouble>();
            (::windows::core::Interface::vtable(this).CreateFromIterable)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[f64]) -> ::windows::core::Result<TensorDouble> {
        Self::ITensorDoubleStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorDouble>();
            (::windows::core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::windows::core::Interface::as_raw(this), shape.len() as u32, shape.as_ptr(), data.len() as u32, data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(shape: &[i64], buffer: P0) -> ::windows::core::Result<TensorDouble>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorDoubleStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorDouble>();
            (::windows::core::Interface::vtable(this).CreateFromBuffer)(::windows::core::Interface::as_raw(this), shape.len() as u32, shape.as_ptr(), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorDoubleStatics<R, F: FnOnce(&ITensorDoubleStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<TensorDouble, ITensorDoubleStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorDoubleStatics2<R, F: FnOnce(&ITensorDoubleStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<TensorDouble, ITensorDoubleStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for TensorDouble {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorDouble;{91e41252-7a8f-4f0e-a28f-9637ffc8a3d0})");
}
impl ::core::clone::Clone for TensorDouble {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for TensorDouble {
    type Vtable = ITensorDouble_Vtbl;
}
unsafe impl ::windows::core::ComInterface for TensorDouble {
    const IID: ::windows::core::GUID = <ITensorDouble as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for TensorDouble {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorDouble";
}
::windows::imp::interface_hierarchy!(TensorDouble, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for TensorDouble {}
impl ::windows::core::CanTryInto<ILearningModelFeatureValue> for TensorDouble {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IMemoryBuffer> for TensorDouble {}
impl ::windows::core::CanTryInto<ITensor> for TensorDouble {}
unsafe impl ::core::marker::Send for TensorDouble {}
unsafe impl ::core::marker::Sync for TensorDouble {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct TensorFeatureDescriptor(::windows::core::IUnknown);
impl TensorFeatureDescriptor {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::ComInterface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LearningModelFeatureKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsRequired(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsRequired)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TensorKind(&self) -> ::windows::core::Result<TensorKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorKind>();
            (::windows::core::Interface::vtable(this).TensorKind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<i64>>();
            (::windows::core::Interface::vtable(this).Shape)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for TensorFeatureDescriptor {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorFeatureDescriptor;{74455c80-946a-4310-a19c-ee0af028fce4})");
}
impl ::core::clone::Clone for TensorFeatureDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for TensorFeatureDescriptor {
    type Vtable = ITensorFeatureDescriptor_Vtbl;
}
unsafe impl ::windows::core::ComInterface for TensorFeatureDescriptor {
    const IID: ::windows::core::GUID = <ITensorFeatureDescriptor as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for TensorFeatureDescriptor {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorFeatureDescriptor";
}
::windows::imp::interface_hierarchy!(TensorFeatureDescriptor, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<ILearningModelFeatureDescriptor> for TensorFeatureDescriptor {}
unsafe impl ::core::marker::Send for TensorFeatureDescriptor {}
unsafe impl ::core::marker::Sync for TensorFeatureDescriptor {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct TensorFloat(::windows::core::IUnknown);
impl TensorFloat {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::ComInterface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LearningModelFeatureKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows::core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IMemoryBufferReference>();
            (::windows::core::Interface::vtable(this).CreateReference)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TensorKind(&self) -> ::windows::core::Result<TensorKind> {
        let this = &::windows::core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorKind>();
            (::windows::core::Interface::vtable(this).TensorKind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<i64>>();
            (::windows::core::Interface::vtable(this).Shape)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<f32>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<f32>>();
            (::windows::core::Interface::vtable(this).GetAsVectorView)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create() -> ::windows::core::Result<TensorFloat> {
        Self::ITensorFloatStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorFloat>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> ::windows::core::Result<TensorFloat>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorFloatStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorFloat>();
            (::windows::core::Interface::vtable(this).Create2)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[f32]) -> ::windows::core::Result<TensorFloat>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorFloatStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorFloat>();
            (::windows::core::Interface::vtable(this).CreateFromArray)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.len() as u32, data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> ::windows::core::Result<TensorFloat>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
        P1: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<f32>>,
    {
        Self::ITensorFloatStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorFloat>();
            (::windows::core::Interface::vtable(this).CreateFromIterable)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[f32]) -> ::windows::core::Result<TensorFloat> {
        Self::ITensorFloatStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorFloat>();
            (::windows::core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::windows::core::Interface::as_raw(this), shape.len() as u32, shape.as_ptr(), data.len() as u32, data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(shape: &[i64], buffer: P0) -> ::windows::core::Result<TensorFloat>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorFloatStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorFloat>();
            (::windows::core::Interface::vtable(this).CreateFromBuffer)(::windows::core::Interface::as_raw(this), shape.len() as u32, shape.as_ptr(), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorFloatStatics<R, F: FnOnce(&ITensorFloatStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<TensorFloat, ITensorFloatStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorFloatStatics2<R, F: FnOnce(&ITensorFloatStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<TensorFloat, ITensorFloatStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for TensorFloat {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorFloat;{f2282d82-aa02-42c8-a0c8-df1efc9676e1})");
}
impl ::core::clone::Clone for TensorFloat {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for TensorFloat {
    type Vtable = ITensorFloat_Vtbl;
}
unsafe impl ::windows::core::ComInterface for TensorFloat {
    const IID: ::windows::core::GUID = <ITensorFloat as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for TensorFloat {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorFloat";
}
::windows::imp::interface_hierarchy!(TensorFloat, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for TensorFloat {}
impl ::windows::core::CanTryInto<ILearningModelFeatureValue> for TensorFloat {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IMemoryBuffer> for TensorFloat {}
impl ::windows::core::CanTryInto<ITensor> for TensorFloat {}
unsafe impl ::core::marker::Send for TensorFloat {}
unsafe impl ::core::marker::Sync for TensorFloat {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct TensorFloat16Bit(::windows::core::IUnknown);
impl TensorFloat16Bit {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::ComInterface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LearningModelFeatureKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows::core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IMemoryBufferReference>();
            (::windows::core::Interface::vtable(this).CreateReference)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TensorKind(&self) -> ::windows::core::Result<TensorKind> {
        let this = &::windows::core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorKind>();
            (::windows::core::Interface::vtable(this).TensorKind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<i64>>();
            (::windows::core::Interface::vtable(this).Shape)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<f32>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<f32>>();
            (::windows::core::Interface::vtable(this).GetAsVectorView)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create() -> ::windows::core::Result<TensorFloat16Bit> {
        Self::ITensorFloat16BitStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorFloat16Bit>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> ::windows::core::Result<TensorFloat16Bit>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorFloat16BitStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorFloat16Bit>();
            (::windows::core::Interface::vtable(this).Create2)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[f32]) -> ::windows::core::Result<TensorFloat16Bit>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorFloat16BitStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorFloat16Bit>();
            (::windows::core::Interface::vtable(this).CreateFromArray)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.len() as u32, data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> ::windows::core::Result<TensorFloat16Bit>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
        P1: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<f32>>,
    {
        Self::ITensorFloat16BitStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorFloat16Bit>();
            (::windows::core::Interface::vtable(this).CreateFromIterable)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[f32]) -> ::windows::core::Result<TensorFloat16Bit> {
        Self::ITensorFloat16BitStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorFloat16Bit>();
            (::windows::core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::windows::core::Interface::as_raw(this), shape.len() as u32, shape.as_ptr(), data.len() as u32, data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(shape: &[i64], buffer: P0) -> ::windows::core::Result<TensorFloat16Bit>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorFloat16BitStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorFloat16Bit>();
            (::windows::core::Interface::vtable(this).CreateFromBuffer)(::windows::core::Interface::as_raw(this), shape.len() as u32, shape.as_ptr(), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorFloat16BitStatics<R, F: FnOnce(&ITensorFloat16BitStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<TensorFloat16Bit, ITensorFloat16BitStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorFloat16BitStatics2<R, F: FnOnce(&ITensorFloat16BitStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<TensorFloat16Bit, ITensorFloat16BitStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for TensorFloat16Bit {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorFloat16Bit;{0ab994fc-5b89-4c3c-b5e4-5282a5316c0a})");
}
impl ::core::clone::Clone for TensorFloat16Bit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for TensorFloat16Bit {
    type Vtable = ITensorFloat16Bit_Vtbl;
}
unsafe impl ::windows::core::ComInterface for TensorFloat16Bit {
    const IID: ::windows::core::GUID = <ITensorFloat16Bit as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for TensorFloat16Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorFloat16Bit";
}
::windows::imp::interface_hierarchy!(TensorFloat16Bit, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for TensorFloat16Bit {}
impl ::windows::core::CanTryInto<ILearningModelFeatureValue> for TensorFloat16Bit {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IMemoryBuffer> for TensorFloat16Bit {}
impl ::windows::core::CanTryInto<ITensor> for TensorFloat16Bit {}
unsafe impl ::core::marker::Send for TensorFloat16Bit {}
unsafe impl ::core::marker::Sync for TensorFloat16Bit {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct TensorInt16Bit(::windows::core::IUnknown);
impl TensorInt16Bit {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::ComInterface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LearningModelFeatureKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows::core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IMemoryBufferReference>();
            (::windows::core::Interface::vtable(this).CreateReference)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TensorKind(&self) -> ::windows::core::Result<TensorKind> {
        let this = &::windows::core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorKind>();
            (::windows::core::Interface::vtable(this).TensorKind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<i64>>();
            (::windows::core::Interface::vtable(this).Shape)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i16>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<i16>>();
            (::windows::core::Interface::vtable(this).GetAsVectorView)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create() -> ::windows::core::Result<TensorInt16Bit> {
        Self::ITensorInt16BitStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorInt16Bit>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> ::windows::core::Result<TensorInt16Bit>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorInt16BitStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorInt16Bit>();
            (::windows::core::Interface::vtable(this).Create2)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[i16]) -> ::windows::core::Result<TensorInt16Bit>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorInt16BitStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorInt16Bit>();
            (::windows::core::Interface::vtable(this).CreateFromArray)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.len() as u32, data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> ::windows::core::Result<TensorInt16Bit>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
        P1: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i16>>,
    {
        Self::ITensorInt16BitStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorInt16Bit>();
            (::windows::core::Interface::vtable(this).CreateFromIterable)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[i16]) -> ::windows::core::Result<TensorInt16Bit> {
        Self::ITensorInt16BitStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorInt16Bit>();
            (::windows::core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::windows::core::Interface::as_raw(this), shape.len() as u32, shape.as_ptr(), data.len() as u32, data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(shape: &[i64], buffer: P0) -> ::windows::core::Result<TensorInt16Bit>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorInt16BitStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorInt16Bit>();
            (::windows::core::Interface::vtable(this).CreateFromBuffer)(::windows::core::Interface::as_raw(this), shape.len() as u32, shape.as_ptr(), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorInt16BitStatics<R, F: FnOnce(&ITensorInt16BitStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<TensorInt16Bit, ITensorInt16BitStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorInt16BitStatics2<R, F: FnOnce(&ITensorInt16BitStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<TensorInt16Bit, ITensorInt16BitStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for TensorInt16Bit {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorInt16Bit;{98a32d39-e6d6-44af-8afa-baebc44dc020})");
}
impl ::core::clone::Clone for TensorInt16Bit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for TensorInt16Bit {
    type Vtable = ITensorInt16Bit_Vtbl;
}
unsafe impl ::windows::core::ComInterface for TensorInt16Bit {
    const IID: ::windows::core::GUID = <ITensorInt16Bit as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for TensorInt16Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorInt16Bit";
}
::windows::imp::interface_hierarchy!(TensorInt16Bit, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for TensorInt16Bit {}
impl ::windows::core::CanTryInto<ILearningModelFeatureValue> for TensorInt16Bit {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IMemoryBuffer> for TensorInt16Bit {}
impl ::windows::core::CanTryInto<ITensor> for TensorInt16Bit {}
unsafe impl ::core::marker::Send for TensorInt16Bit {}
unsafe impl ::core::marker::Sync for TensorInt16Bit {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct TensorInt32Bit(::windows::core::IUnknown);
impl TensorInt32Bit {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::ComInterface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LearningModelFeatureKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows::core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IMemoryBufferReference>();
            (::windows::core::Interface::vtable(this).CreateReference)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TensorKind(&self) -> ::windows::core::Result<TensorKind> {
        let this = &::windows::core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorKind>();
            (::windows::core::Interface::vtable(this).TensorKind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<i64>>();
            (::windows::core::Interface::vtable(this).Shape)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<i32>>();
            (::windows::core::Interface::vtable(this).GetAsVectorView)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create() -> ::windows::core::Result<TensorInt32Bit> {
        Self::ITensorInt32BitStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorInt32Bit>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> ::windows::core::Result<TensorInt32Bit>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorInt32BitStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorInt32Bit>();
            (::windows::core::Interface::vtable(this).Create2)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[i32]) -> ::windows::core::Result<TensorInt32Bit>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorInt32BitStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorInt32Bit>();
            (::windows::core::Interface::vtable(this).CreateFromArray)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.len() as u32, data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> ::windows::core::Result<TensorInt32Bit>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
        P1: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i32>>,
    {
        Self::ITensorInt32BitStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorInt32Bit>();
            (::windows::core::Interface::vtable(this).CreateFromIterable)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[i32]) -> ::windows::core::Result<TensorInt32Bit> {
        Self::ITensorInt32BitStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorInt32Bit>();
            (::windows::core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::windows::core::Interface::as_raw(this), shape.len() as u32, shape.as_ptr(), data.len() as u32, data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(shape: &[i64], buffer: P0) -> ::windows::core::Result<TensorInt32Bit>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorInt32BitStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorInt32Bit>();
            (::windows::core::Interface::vtable(this).CreateFromBuffer)(::windows::core::Interface::as_raw(this), shape.len() as u32, shape.as_ptr(), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorInt32BitStatics<R, F: FnOnce(&ITensorInt32BitStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<TensorInt32Bit, ITensorInt32BitStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorInt32BitStatics2<R, F: FnOnce(&ITensorInt32BitStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<TensorInt32Bit, ITensorInt32BitStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for TensorInt32Bit {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorInt32Bit;{2c0c28d3-207c-4486-a7d2-884522c5e589})");
}
impl ::core::clone::Clone for TensorInt32Bit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for TensorInt32Bit {
    type Vtable = ITensorInt32Bit_Vtbl;
}
unsafe impl ::windows::core::ComInterface for TensorInt32Bit {
    const IID: ::windows::core::GUID = <ITensorInt32Bit as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for TensorInt32Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorInt32Bit";
}
::windows::imp::interface_hierarchy!(TensorInt32Bit, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for TensorInt32Bit {}
impl ::windows::core::CanTryInto<ILearningModelFeatureValue> for TensorInt32Bit {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IMemoryBuffer> for TensorInt32Bit {}
impl ::windows::core::CanTryInto<ITensor> for TensorInt32Bit {}
unsafe impl ::core::marker::Send for TensorInt32Bit {}
unsafe impl ::core::marker::Sync for TensorInt32Bit {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct TensorInt64Bit(::windows::core::IUnknown);
impl TensorInt64Bit {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::ComInterface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LearningModelFeatureKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows::core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IMemoryBufferReference>();
            (::windows::core::Interface::vtable(this).CreateReference)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TensorKind(&self) -> ::windows::core::Result<TensorKind> {
        let this = &::windows::core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorKind>();
            (::windows::core::Interface::vtable(this).TensorKind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<i64>>();
            (::windows::core::Interface::vtable(this).Shape)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<i64>>();
            (::windows::core::Interface::vtable(this).GetAsVectorView)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create() -> ::windows::core::Result<TensorInt64Bit> {
        Self::ITensorInt64BitStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorInt64Bit>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> ::windows::core::Result<TensorInt64Bit>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorInt64BitStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorInt64Bit>();
            (::windows::core::Interface::vtable(this).Create2)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[i64]) -> ::windows::core::Result<TensorInt64Bit>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorInt64BitStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorInt64Bit>();
            (::windows::core::Interface::vtable(this).CreateFromArray)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.len() as u32, data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> ::windows::core::Result<TensorInt64Bit>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
        P1: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorInt64BitStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorInt64Bit>();
            (::windows::core::Interface::vtable(this).CreateFromIterable)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[i64]) -> ::windows::core::Result<TensorInt64Bit> {
        Self::ITensorInt64BitStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorInt64Bit>();
            (::windows::core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::windows::core::Interface::as_raw(this), shape.len() as u32, shape.as_ptr(), data.len() as u32, data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(shape: &[i64], buffer: P0) -> ::windows::core::Result<TensorInt64Bit>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorInt64BitStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorInt64Bit>();
            (::windows::core::Interface::vtable(this).CreateFromBuffer)(::windows::core::Interface::as_raw(this), shape.len() as u32, shape.as_ptr(), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorInt64BitStatics<R, F: FnOnce(&ITensorInt64BitStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<TensorInt64Bit, ITensorInt64BitStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorInt64BitStatics2<R, F: FnOnce(&ITensorInt64BitStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<TensorInt64Bit, ITensorInt64BitStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for TensorInt64Bit {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorInt64Bit;{499665ba-1fa2-45ad-af25-a0bd9bda4c87})");
}
impl ::core::clone::Clone for TensorInt64Bit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for TensorInt64Bit {
    type Vtable = ITensorInt64Bit_Vtbl;
}
unsafe impl ::windows::core::ComInterface for TensorInt64Bit {
    const IID: ::windows::core::GUID = <ITensorInt64Bit as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for TensorInt64Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorInt64Bit";
}
::windows::imp::interface_hierarchy!(TensorInt64Bit, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for TensorInt64Bit {}
impl ::windows::core::CanTryInto<ILearningModelFeatureValue> for TensorInt64Bit {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IMemoryBuffer> for TensorInt64Bit {}
impl ::windows::core::CanTryInto<ITensor> for TensorInt64Bit {}
unsafe impl ::core::marker::Send for TensorInt64Bit {}
unsafe impl ::core::marker::Sync for TensorInt64Bit {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct TensorInt8Bit(::windows::core::IUnknown);
impl TensorInt8Bit {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::ComInterface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LearningModelFeatureKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows::core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IMemoryBufferReference>();
            (::windows::core::Interface::vtable(this).CreateReference)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TensorKind(&self) -> ::windows::core::Result<TensorKind> {
        let this = &::windows::core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorKind>();
            (::windows::core::Interface::vtable(this).TensorKind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<i64>>();
            (::windows::core::Interface::vtable(this).Shape)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<u8>>();
            (::windows::core::Interface::vtable(this).GetAsVectorView)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create() -> ::windows::core::Result<TensorInt8Bit> {
        Self::ITensorInt8BitStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorInt8Bit>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> ::windows::core::Result<TensorInt8Bit>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorInt8BitStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorInt8Bit>();
            (::windows::core::Interface::vtable(this).Create2)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[u8]) -> ::windows::core::Result<TensorInt8Bit>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorInt8BitStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorInt8Bit>();
            (::windows::core::Interface::vtable(this).CreateFromArray)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.len() as u32, data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> ::windows::core::Result<TensorInt8Bit>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
        P1: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<u8>>,
    {
        Self::ITensorInt8BitStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorInt8Bit>();
            (::windows::core::Interface::vtable(this).CreateFromIterable)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[u8]) -> ::windows::core::Result<TensorInt8Bit> {
        Self::ITensorInt8BitStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorInt8Bit>();
            (::windows::core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::windows::core::Interface::as_raw(this), shape.len() as u32, shape.as_ptr(), data.len() as u32, data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(shape: &[i64], buffer: P0) -> ::windows::core::Result<TensorInt8Bit>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorInt8BitStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorInt8Bit>();
            (::windows::core::Interface::vtable(this).CreateFromBuffer)(::windows::core::Interface::as_raw(this), shape.len() as u32, shape.as_ptr(), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorInt8BitStatics<R, F: FnOnce(&ITensorInt8BitStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<TensorInt8Bit, ITensorInt8BitStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorInt8BitStatics2<R, F: FnOnce(&ITensorInt8BitStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<TensorInt8Bit, ITensorInt8BitStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for TensorInt8Bit {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorInt8Bit;{cddd97c5-ffd8-4fef-aefb-30e1a485b2ee})");
}
impl ::core::clone::Clone for TensorInt8Bit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for TensorInt8Bit {
    type Vtable = ITensorInt8Bit_Vtbl;
}
unsafe impl ::windows::core::ComInterface for TensorInt8Bit {
    const IID: ::windows::core::GUID = <ITensorInt8Bit as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for TensorInt8Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorInt8Bit";
}
::windows::imp::interface_hierarchy!(TensorInt8Bit, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for TensorInt8Bit {}
impl ::windows::core::CanTryInto<ILearningModelFeatureValue> for TensorInt8Bit {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IMemoryBuffer> for TensorInt8Bit {}
impl ::windows::core::CanTryInto<ITensor> for TensorInt8Bit {}
unsafe impl ::core::marker::Send for TensorInt8Bit {}
unsafe impl ::core::marker::Sync for TensorInt8Bit {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct TensorString(::windows::core::IUnknown);
impl TensorString {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::ComInterface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LearningModelFeatureKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows::core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IMemoryBufferReference>();
            (::windows::core::Interface::vtable(this).CreateReference)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TensorKind(&self) -> ::windows::core::Result<TensorKind> {
        let this = &::windows::core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorKind>();
            (::windows::core::Interface::vtable(this).TensorKind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<i64>>();
            (::windows::core::Interface::vtable(this).Shape)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).GetAsVectorView)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create() -> ::windows::core::Result<TensorString> {
        Self::ITensorStringStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorString>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> ::windows::core::Result<TensorString>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorStringStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorString>();
            (::windows::core::Interface::vtable(this).Create2)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[::windows::core::HSTRING]) -> ::windows::core::Result<TensorString>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorStringStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorString>();
            (::windows::core::Interface::vtable(this).CreateFromArray)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.len() as u32, ::core::mem::transmute(data.as_ptr()), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> ::windows::core::Result<TensorString>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
        P1: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>,
    {
        Self::ITensorStringStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorString>();
            (::windows::core::Interface::vtable(this).CreateFromIterable)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[::windows::core::HSTRING]) -> ::windows::core::Result<TensorString> {
        Self::ITensorStringStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorString>();
            (::windows::core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::windows::core::Interface::as_raw(this), shape.len() as u32, shape.as_ptr(), data.len() as u32, ::core::mem::transmute(data.as_ptr()), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorStringStatics<R, F: FnOnce(&ITensorStringStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<TensorString, ITensorStringStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorStringStatics2<R, F: FnOnce(&ITensorStringStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<TensorString, ITensorStringStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for TensorString {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorString;{582335c8-bdb1-4610-bc75-35e9cbf009b7})");
}
impl ::core::clone::Clone for TensorString {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for TensorString {
    type Vtable = ITensorString_Vtbl;
}
unsafe impl ::windows::core::ComInterface for TensorString {
    const IID: ::windows::core::GUID = <ITensorString as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for TensorString {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorString";
}
::windows::imp::interface_hierarchy!(TensorString, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for TensorString {}
impl ::windows::core::CanTryInto<ILearningModelFeatureValue> for TensorString {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IMemoryBuffer> for TensorString {}
impl ::windows::core::CanTryInto<ITensor> for TensorString {}
unsafe impl ::core::marker::Send for TensorString {}
unsafe impl ::core::marker::Sync for TensorString {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct TensorUInt16Bit(::windows::core::IUnknown);
impl TensorUInt16Bit {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::ComInterface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LearningModelFeatureKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows::core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IMemoryBufferReference>();
            (::windows::core::Interface::vtable(this).CreateReference)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TensorKind(&self) -> ::windows::core::Result<TensorKind> {
        let this = &::windows::core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorKind>();
            (::windows::core::Interface::vtable(this).TensorKind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<i64>>();
            (::windows::core::Interface::vtable(this).Shape)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u16>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<u16>>();
            (::windows::core::Interface::vtable(this).GetAsVectorView)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create() -> ::windows::core::Result<TensorUInt16Bit> {
        Self::ITensorUInt16BitStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorUInt16Bit>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> ::windows::core::Result<TensorUInt16Bit>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorUInt16BitStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorUInt16Bit>();
            (::windows::core::Interface::vtable(this).Create2)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[u16]) -> ::windows::core::Result<TensorUInt16Bit>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorUInt16BitStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorUInt16Bit>();
            (::windows::core::Interface::vtable(this).CreateFromArray)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.len() as u32, data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> ::windows::core::Result<TensorUInt16Bit>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
        P1: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<u16>>,
    {
        Self::ITensorUInt16BitStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorUInt16Bit>();
            (::windows::core::Interface::vtable(this).CreateFromIterable)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[u16]) -> ::windows::core::Result<TensorUInt16Bit> {
        Self::ITensorUInt16BitStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorUInt16Bit>();
            (::windows::core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::windows::core::Interface::as_raw(this), shape.len() as u32, shape.as_ptr(), data.len() as u32, data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(shape: &[i64], buffer: P0) -> ::windows::core::Result<TensorUInt16Bit>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorUInt16BitStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorUInt16Bit>();
            (::windows::core::Interface::vtable(this).CreateFromBuffer)(::windows::core::Interface::as_raw(this), shape.len() as u32, shape.as_ptr(), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorUInt16BitStatics<R, F: FnOnce(&ITensorUInt16BitStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<TensorUInt16Bit, ITensorUInt16BitStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorUInt16BitStatics2<R, F: FnOnce(&ITensorUInt16BitStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<TensorUInt16Bit, ITensorUInt16BitStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for TensorUInt16Bit {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorUInt16Bit;{68140f4b-23c0-42f3-81f6-a891c011bc3f})");
}
impl ::core::clone::Clone for TensorUInt16Bit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for TensorUInt16Bit {
    type Vtable = ITensorUInt16Bit_Vtbl;
}
unsafe impl ::windows::core::ComInterface for TensorUInt16Bit {
    const IID: ::windows::core::GUID = <ITensorUInt16Bit as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for TensorUInt16Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorUInt16Bit";
}
::windows::imp::interface_hierarchy!(TensorUInt16Bit, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for TensorUInt16Bit {}
impl ::windows::core::CanTryInto<ILearningModelFeatureValue> for TensorUInt16Bit {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IMemoryBuffer> for TensorUInt16Bit {}
impl ::windows::core::CanTryInto<ITensor> for TensorUInt16Bit {}
unsafe impl ::core::marker::Send for TensorUInt16Bit {}
unsafe impl ::core::marker::Sync for TensorUInt16Bit {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct TensorUInt32Bit(::windows::core::IUnknown);
impl TensorUInt32Bit {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::ComInterface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LearningModelFeatureKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows::core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IMemoryBufferReference>();
            (::windows::core::Interface::vtable(this).CreateReference)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TensorKind(&self) -> ::windows::core::Result<TensorKind> {
        let this = &::windows::core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorKind>();
            (::windows::core::Interface::vtable(this).TensorKind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<i64>>();
            (::windows::core::Interface::vtable(this).Shape)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<u32>>();
            (::windows::core::Interface::vtable(this).GetAsVectorView)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create() -> ::windows::core::Result<TensorUInt32Bit> {
        Self::ITensorUInt32BitStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorUInt32Bit>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> ::windows::core::Result<TensorUInt32Bit>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorUInt32BitStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorUInt32Bit>();
            (::windows::core::Interface::vtable(this).Create2)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[u32]) -> ::windows::core::Result<TensorUInt32Bit>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorUInt32BitStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorUInt32Bit>();
            (::windows::core::Interface::vtable(this).CreateFromArray)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.len() as u32, data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> ::windows::core::Result<TensorUInt32Bit>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
        P1: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<u32>>,
    {
        Self::ITensorUInt32BitStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorUInt32Bit>();
            (::windows::core::Interface::vtable(this).CreateFromIterable)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[u32]) -> ::windows::core::Result<TensorUInt32Bit> {
        Self::ITensorUInt32BitStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorUInt32Bit>();
            (::windows::core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::windows::core::Interface::as_raw(this), shape.len() as u32, shape.as_ptr(), data.len() as u32, data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(shape: &[i64], buffer: P0) -> ::windows::core::Result<TensorUInt32Bit>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorUInt32BitStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorUInt32Bit>();
            (::windows::core::Interface::vtable(this).CreateFromBuffer)(::windows::core::Interface::as_raw(this), shape.len() as u32, shape.as_ptr(), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorUInt32BitStatics<R, F: FnOnce(&ITensorUInt32BitStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<TensorUInt32Bit, ITensorUInt32BitStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorUInt32BitStatics2<R, F: FnOnce(&ITensorUInt32BitStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<TensorUInt32Bit, ITensorUInt32BitStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for TensorUInt32Bit {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorUInt32Bit;{d8c9c2ff-7511-45a3-bfac-c38f370d2237})");
}
impl ::core::clone::Clone for TensorUInt32Bit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for TensorUInt32Bit {
    type Vtable = ITensorUInt32Bit_Vtbl;
}
unsafe impl ::windows::core::ComInterface for TensorUInt32Bit {
    const IID: ::windows::core::GUID = <ITensorUInt32Bit as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for TensorUInt32Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorUInt32Bit";
}
::windows::imp::interface_hierarchy!(TensorUInt32Bit, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for TensorUInt32Bit {}
impl ::windows::core::CanTryInto<ILearningModelFeatureValue> for TensorUInt32Bit {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IMemoryBuffer> for TensorUInt32Bit {}
impl ::windows::core::CanTryInto<ITensor> for TensorUInt32Bit {}
unsafe impl ::core::marker::Send for TensorUInt32Bit {}
unsafe impl ::core::marker::Sync for TensorUInt32Bit {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct TensorUInt64Bit(::windows::core::IUnknown);
impl TensorUInt64Bit {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::ComInterface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LearningModelFeatureKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows::core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IMemoryBufferReference>();
            (::windows::core::Interface::vtable(this).CreateReference)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TensorKind(&self) -> ::windows::core::Result<TensorKind> {
        let this = &::windows::core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorKind>();
            (::windows::core::Interface::vtable(this).TensorKind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<i64>>();
            (::windows::core::Interface::vtable(this).Shape)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<u64>>();
            (::windows::core::Interface::vtable(this).GetAsVectorView)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create() -> ::windows::core::Result<TensorUInt64Bit> {
        Self::ITensorUInt64BitStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorUInt64Bit>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> ::windows::core::Result<TensorUInt64Bit>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorUInt64BitStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorUInt64Bit>();
            (::windows::core::Interface::vtable(this).Create2)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[u64]) -> ::windows::core::Result<TensorUInt64Bit>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorUInt64BitStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorUInt64Bit>();
            (::windows::core::Interface::vtable(this).CreateFromArray)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.len() as u32, data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> ::windows::core::Result<TensorUInt64Bit>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
        P1: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<u64>>,
    {
        Self::ITensorUInt64BitStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorUInt64Bit>();
            (::windows::core::Interface::vtable(this).CreateFromIterable)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[u64]) -> ::windows::core::Result<TensorUInt64Bit> {
        Self::ITensorUInt64BitStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorUInt64Bit>();
            (::windows::core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::windows::core::Interface::as_raw(this), shape.len() as u32, shape.as_ptr(), data.len() as u32, data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(shape: &[i64], buffer: P0) -> ::windows::core::Result<TensorUInt64Bit>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorUInt64BitStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorUInt64Bit>();
            (::windows::core::Interface::vtable(this).CreateFromBuffer)(::windows::core::Interface::as_raw(this), shape.len() as u32, shape.as_ptr(), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorUInt64BitStatics<R, F: FnOnce(&ITensorUInt64BitStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<TensorUInt64Bit, ITensorUInt64BitStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorUInt64BitStatics2<R, F: FnOnce(&ITensorUInt64BitStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<TensorUInt64Bit, ITensorUInt64BitStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for TensorUInt64Bit {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorUInt64Bit;{2e70ffad-04bf-4825-839a-82baef8c7886})");
}
impl ::core::clone::Clone for TensorUInt64Bit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for TensorUInt64Bit {
    type Vtable = ITensorUInt64Bit_Vtbl;
}
unsafe impl ::windows::core::ComInterface for TensorUInt64Bit {
    const IID: ::windows::core::GUID = <ITensorUInt64Bit as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for TensorUInt64Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorUInt64Bit";
}
::windows::imp::interface_hierarchy!(TensorUInt64Bit, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for TensorUInt64Bit {}
impl ::windows::core::CanTryInto<ILearningModelFeatureValue> for TensorUInt64Bit {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IMemoryBuffer> for TensorUInt64Bit {}
impl ::windows::core::CanTryInto<ITensor> for TensorUInt64Bit {}
unsafe impl ::core::marker::Send for TensorUInt64Bit {}
unsafe impl ::core::marker::Sync for TensorUInt64Bit {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct TensorUInt8Bit(::windows::core::IUnknown);
impl TensorUInt8Bit {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind> {
        let this = &::windows::core::ComInterface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LearningModelFeatureKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows::core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IMemoryBufferReference>();
            (::windows::core::Interface::vtable(this).CreateReference)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TensorKind(&self) -> ::windows::core::Result<TensorKind> {
        let this = &::windows::core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorKind>();
            (::windows::core::Interface::vtable(this).TensorKind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows::core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<i64>>();
            (::windows::core::Interface::vtable(this).Shape)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<u8>>();
            (::windows::core::Interface::vtable(this).GetAsVectorView)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create() -> ::windows::core::Result<TensorUInt8Bit> {
        Self::ITensorUInt8BitStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorUInt8Bit>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> ::windows::core::Result<TensorUInt8Bit>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorUInt8BitStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorUInt8Bit>();
            (::windows::core::Interface::vtable(this).Create2)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[u8]) -> ::windows::core::Result<TensorUInt8Bit>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorUInt8BitStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorUInt8Bit>();
            (::windows::core::Interface::vtable(this).CreateFromArray)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.len() as u32, data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> ::windows::core::Result<TensorUInt8Bit>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
        P1: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<u8>>,
    {
        Self::ITensorUInt8BitStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorUInt8Bit>();
            (::windows::core::Interface::vtable(this).CreateFromIterable)(::windows::core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[u8]) -> ::windows::core::Result<TensorUInt8Bit> {
        Self::ITensorUInt8BitStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorUInt8Bit>();
            (::windows::core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::windows::core::Interface::as_raw(this), shape.len() as u32, shape.as_ptr(), data.len() as u32, data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(shape: &[i64], buffer: P0) -> ::windows::core::Result<TensorUInt8Bit>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorUInt8BitStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TensorUInt8Bit>();
            (::windows::core::Interface::vtable(this).CreateFromBuffer)(::windows::core::Interface::as_raw(this), shape.len() as u32, shape.as_ptr(), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorUInt8BitStatics<R, F: FnOnce(&ITensorUInt8BitStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<TensorUInt8Bit, ITensorUInt8BitStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorUInt8BitStatics2<R, F: FnOnce(&ITensorUInt8BitStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<TensorUInt8Bit, ITensorUInt8BitStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for TensorUInt8Bit {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorUInt8Bit;{58e1ae27-622b-48e3-be22-d867aed1daac})");
}
impl ::core::clone::Clone for TensorUInt8Bit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for TensorUInt8Bit {
    type Vtable = ITensorUInt8Bit_Vtbl;
}
unsafe impl ::windows::core::ComInterface for TensorUInt8Bit {
    const IID: ::windows::core::GUID = <ITensorUInt8Bit as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for TensorUInt8Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorUInt8Bit";
}
::windows::imp::interface_hierarchy!(TensorUInt8Bit, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for TensorUInt8Bit {}
impl ::windows::core::CanTryInto<ILearningModelFeatureValue> for TensorUInt8Bit {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IMemoryBuffer> for TensorUInt8Bit {}
impl ::windows::core::CanTryInto<ITensor> for TensorUInt8Bit {}
unsafe impl ::core::marker::Send for TensorUInt8Bit {}
unsafe impl ::core::marker::Sync for TensorUInt8Bit {}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for LearningModelDeviceKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for LearningModelDeviceKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LearningModelDeviceKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for LearningModelDeviceKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.AI.MachineLearning.LearningModelDeviceKind;i4)");
}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for LearningModelFeatureKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for LearningModelFeatureKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LearningModelFeatureKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for LearningModelFeatureKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.AI.MachineLearning.LearningModelFeatureKind;i4)");
}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for LearningModelPixelRange {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for LearningModelPixelRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LearningModelPixelRange").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for LearningModelPixelRange {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.AI.MachineLearning.LearningModelPixelRange;i4)");
}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for TensorKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TensorKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TensorKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for TensorKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.AI.MachineLearning.TensorKind;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
