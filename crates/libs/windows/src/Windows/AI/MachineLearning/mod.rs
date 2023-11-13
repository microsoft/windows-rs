#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IImageFeatureDescriptor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IImageFeatureDescriptor {
    type Vtable = IImageFeatureDescriptor_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IImageFeatureDescriptor {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x365585a5_171a_4a2a_985f_265159d3895a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageFeatureDescriptor_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Imaging")]
    pub BitmapPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    BitmapPixelFormat: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub BitmapAlphaMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapAlphaMode) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    BitmapAlphaMode: usize,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IImageFeatureDescriptor2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IImageFeatureDescriptor2 {
    type Vtable = IImageFeatureDescriptor2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IImageFeatureDescriptor2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2b27cca7_d533_5862_bb98_1611b155b0e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageFeatureDescriptor2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PixelRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LearningModelPixelRange) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IImageFeatureValue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IImageFeatureValue {
    type Vtable = IImageFeatureValue_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IImageFeatureValue {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf0414fd9_c9aa_4405_b7fb_94f87c8a3037);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageFeatureValue_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media")]
    pub VideoFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media"))]
    VideoFrame: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IImageFeatureValueStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IImageFeatureValueStatics {
    type Vtable = IImageFeatureValueStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IImageFeatureValueStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1bc317fd_23cb_4610_b085_c8e1c87ebaa0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageFeatureValueStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media")]
    pub CreateFromVideoFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, image: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media"))]
    CreateFromVideoFrame: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILearningModel(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILearningModel {
    type Vtable = ILearningModel_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILearningModel {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5b8e4920_489f_4e86_9128_265a327b78fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModel_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Author: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Domain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Metadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Metadata: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub InputFeatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InputFeatures: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub OutputFeatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    OutputFeatures: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILearningModelBinding(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILearningModelBinding {
    type Vtable = ILearningModelBinding_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILearningModelBinding {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xea312f20_168f_4f8c_94fe_2e7ac31b4aa8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelBinding_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Bind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub BindWithProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: *mut ::core::ffi::c_void, props: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BindWithProperties: usize,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILearningModelBindingFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILearningModelBindingFactory {
    type Vtable = ILearningModelBindingFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILearningModelBindingFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc95f7a7a_e788_475e_8917_23aa381faf0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelBindingFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateFromSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, session: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILearningModelDevice(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILearningModelDevice {
    type Vtable = ILearningModelDevice_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILearningModelDevice {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf5c2c8fe_3f56_4a8c_ac5f_fdb92d8b8252);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelDevice_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics")]
    pub AdapterId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::DisplayAdapterId) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    AdapterId: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Direct3D11Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Direct3D11Device: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILearningModelDeviceFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILearningModelDeviceFactory {
    type Vtable = ILearningModelDeviceFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILearningModelDeviceFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9cffd74d_b1e5_4f20_80ad_0a56690db06b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelDeviceFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, devicekind: LearningModelDeviceKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILearningModelDeviceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILearningModelDeviceStatics {
    type Vtable = ILearningModelDeviceStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILearningModelDeviceStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x49f32107_a8bf_42bb_92c7_10b12dc5d21f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelDeviceStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CreateFromDirect3D11Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CreateFromDirect3D11Device: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILearningModelEvaluationResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILearningModelEvaluationResult {
    type Vtable = ILearningModelEvaluationResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILearningModelEvaluationResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb2f9bfcd_960e_49c0_8593_eb190ae3eee2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelEvaluationResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ErrorStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub Succeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Outputs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Outputs: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILearningModelFeatureDescriptor(::windows_core::IUnknown);
impl ILearningModelFeatureDescriptor {
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<LearningModelFeatureKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsRequired(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsRequired)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(ILearningModelFeatureDescriptor, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for ILearningModelFeatureDescriptor {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{bc08cf7c-6ed0-4004-97ba-b9a2eecd2b4f}");
}
unsafe impl ::windows_core::Interface for ILearningModelFeatureDescriptor {
    type Vtable = ILearningModelFeatureDescriptor_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILearningModelFeatureDescriptor {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc08cf7c_6ed0_4004_97ba_b9a2eecd2b4f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelFeatureDescriptor_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LearningModelFeatureKind) -> ::windows_core::HRESULT,
    pub IsRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILearningModelFeatureValue(::windows_core::IUnknown);
impl ILearningModelFeatureValue {
    pub fn Kind(&self) -> ::windows_core::Result<LearningModelFeatureKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(ILearningModelFeatureValue, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for ILearningModelFeatureValue {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{f51005db-4085-4dfe-9fed-95eb0c0cf75c}");
}
unsafe impl ::windows_core::Interface for ILearningModelFeatureValue {
    type Vtable = ILearningModelFeatureValue_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILearningModelFeatureValue {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf51005db_4085_4dfe_9fed_95eb0c0cf75c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelFeatureValue_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LearningModelFeatureKind) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILearningModelOperatorProvider(::windows_core::IUnknown);
impl ILearningModelOperatorProvider {}
::windows_core::imp::interface_hierarchy!(ILearningModelOperatorProvider, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for ILearningModelOperatorProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{2a222e5d-afb1-47ed-bfad-b5b3a459ec04}");
}
unsafe impl ::windows_core::Interface for ILearningModelOperatorProvider {
    type Vtable = ILearningModelOperatorProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILearningModelOperatorProvider {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2a222e5d_afb1_47ed_bfad_b5b3a459ec04);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelOperatorProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILearningModelSession(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILearningModelSession {
    type Vtable = ILearningModelSession_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILearningModelSession {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8e58f8f6_b787_4c11_90f0_7129aeca74a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSession_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Model: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub EvaluationProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EvaluationProperties: usize,
    #[cfg(feature = "Foundation")]
    pub EvaluateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bindings: *mut ::core::ffi::c_void, correlationid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EvaluateAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub EvaluateFeaturesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, features: *mut ::core::ffi::c_void, correlationid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EvaluateFeaturesAsync: usize,
    pub Evaluate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bindings: *mut ::core::ffi::c_void, correlationid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub EvaluateFeatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, features: *mut ::core::ffi::c_void, correlationid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EvaluateFeatures: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILearningModelSessionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILearningModelSessionFactory {
    type Vtable = ILearningModelSessionFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILearningModelSessionFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0f6b881d_1c9b_47b6_bfe0_f1cf62a67579);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSessionFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateFromModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, model: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateFromModelOnDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, model: *mut ::core::ffi::c_void, devicetorunon: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILearningModelSessionFactory2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILearningModelSessionFactory2 {
    type Vtable = ILearningModelSessionFactory2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILearningModelSessionFactory2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4e5c88bf_0a1f_5fec_ade0_2fd91e4ef29b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSessionFactory2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateFromModelOnDeviceWithSessionOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, model: *mut ::core::ffi::c_void, devicetorunon: *mut ::core::ffi::c_void, learningmodelsessionoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILearningModelSessionOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILearningModelSessionOptions {
    type Vtable = ILearningModelSessionOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILearningModelSessionOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb8f63fa1_134d_5133_8cff_3a5c3c263beb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSessionOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub BatchSizeOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetBatchSizeOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILearningModelSessionOptions2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILearningModelSessionOptions2 {
    type Vtable = ILearningModelSessionOptions2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILearningModelSessionOptions2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6fcd1dc4_175f_5bd2_8de5_2f2006a25adf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSessionOptions2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CloseModelOnSessionCreation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCloseModelOnSessionCreation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILearningModelSessionOptions3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILearningModelSessionOptions3 {
    type Vtable = ILearningModelSessionOptions3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILearningModelSessionOptions3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x58e15cee_d8c2_56fc_92e8_76d751081086);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSessionOptions3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OverrideNamedDimension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, dimension: u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILearningModelStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILearningModelStatics {
    type Vtable = ILearningModelStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILearningModelStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe3b977e8_6952_4e47_8ef4_1f7f07897c6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LoadFromStorageFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modelfile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LoadFromStorageFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub LoadFromStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modelstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    LoadFromStreamAsync: usize,
    pub LoadFromFilePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub LoadFromStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modelstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LoadFromStream: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LoadFromStorageFileWithOperatorProviderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modelfile: *mut ::core::ffi::c_void, operatorprovider: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LoadFromStorageFileWithOperatorProviderAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub LoadFromStreamWithOperatorProviderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modelstream: *mut ::core::ffi::c_void, operatorprovider: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    LoadFromStreamWithOperatorProviderAsync: usize,
    pub LoadFromFilePathWithOperatorProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::std::mem::MaybeUninit<::windows_core::HSTRING>, operatorprovider: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub LoadFromStreamWithOperatorProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modelstream: *mut ::core::ffi::c_void, operatorprovider: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LoadFromStreamWithOperatorProvider: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMapFeatureDescriptor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMapFeatureDescriptor {
    type Vtable = IMapFeatureDescriptor_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMapFeatureDescriptor {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x530424bd_a257_436d_9e60_c2981f7cc5c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapFeatureDescriptor_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub KeyKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TensorKind) -> ::windows_core::HRESULT,
    pub ValueDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISequenceFeatureDescriptor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISequenceFeatureDescriptor {
    type Vtable = ISequenceFeatureDescriptor_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISequenceFeatureDescriptor {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x84f6945a_562b_4d62_a851_739aced96668);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISequenceFeatureDescriptor_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ElementDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensor(::windows_core::IUnknown);
impl ITensor {
    pub fn TensorKind(&self) -> ::windows_core::Result<TensorKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TensorKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Shape)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<LearningModelFeatureKind> {
        let this = &::windows_core::ComInterface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(ITensor, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<ILearningModelFeatureValue> for ITensor {}
impl ::windows_core::RuntimeType for ITensor {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{05489593-a305-4a25-ad09-440119b4b7f6}");
}
unsafe impl ::windows_core::Interface for ITensor {
    type Vtable = ITensor_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensor {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x05489593_a305_4a25_ad09_440119b4b7f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensor_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TensorKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TensorKind) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Shape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Shape: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorBoolean(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorBoolean {
    type Vtable = ITensorBoolean_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorBoolean {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x50f311ed_29e9_4a5c_a44d_8fc512584eed);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorBoolean_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorBooleanStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorBooleanStatics {
    type Vtable = ITensorBooleanStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorBooleanStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2796862c_2357_49a7_b476_d0aa3dfe6866);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorBooleanStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorBooleanStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorBooleanStatics2 {
    type Vtable = ITensorBooleanStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorBooleanStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa3a4a501_6a2d_52d7_b04b_c435baee0115);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorBooleanStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorDouble(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorDouble {
    type Vtable = ITensorDouble_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorDouble {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x91e41252_7a8f_4f0e_a28f_9637ffc8a3d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorDouble_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorDoubleStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorDoubleStatics {
    type Vtable = ITensorDoubleStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorDoubleStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa86693c5_9538_44e7_a3ca_5df374a5a70c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorDoubleStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorDoubleStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorDoubleStatics2 {
    type Vtable = ITensorDoubleStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorDoubleStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x93a570de_5e9a_5094_85c8_592c655e68ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorDoubleStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorFeatureDescriptor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorFeatureDescriptor {
    type Vtable = ITensorFeatureDescriptor_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorFeatureDescriptor {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x74455c80_946a_4310_a19c_ee0af028fce4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorFeatureDescriptor_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TensorKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TensorKind) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Shape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Shape: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorFloat(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorFloat {
    type Vtable = ITensorFloat_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorFloat {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf2282d82_aa02_42c8_a0c8_df1efc9676e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorFloat_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorFloat16Bit(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorFloat16Bit {
    type Vtable = ITensorFloat16Bit_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorFloat16Bit {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0ab994fc_5b89_4c3c_b5e4_5282a5316c0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorFloat16Bit_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorFloat16BitStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorFloat16BitStatics {
    type Vtable = ITensorFloat16BitStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorFloat16BitStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa52db6f5_318a_44d4_820b_0cdc7054a84a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorFloat16BitStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorFloat16BitStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorFloat16BitStatics2 {
    type Vtable = ITensorFloat16BitStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorFloat16BitStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x68545726_2dc7_51bf_b470_0b344cc2a1bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorFloat16BitStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorFloatStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorFloatStatics {
    type Vtable = ITensorFloatStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorFloatStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdbcd395b_3ba3_452f_b10d_3c135e573fa9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorFloatStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorFloatStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorFloatStatics2 {
    type Vtable = ITensorFloatStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorFloatStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x24610bc1_5e44_5713_b281_8f4ad4d555e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorFloatStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorInt16Bit(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorInt16Bit {
    type Vtable = ITensorInt16Bit_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorInt16Bit {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x98a32d39_e6d6_44af_8afa_baebc44dc020);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt16Bit_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorInt16BitStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorInt16BitStatics {
    type Vtable = ITensorInt16BitStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorInt16BitStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x98646293_266e_4b1a_821f_e60d70898b91);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt16BitStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const i16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorInt16BitStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorInt16BitStatics2 {
    type Vtable = ITensorInt16BitStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorInt16BitStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0cd70cf4_696c_5e5f_95d8_5ebf9670148b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt16BitStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const i16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorInt32Bit(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorInt32Bit {
    type Vtable = ITensorInt32Bit_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorInt32Bit {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c0c28d3_207c_4486_a7d2_884522c5e589);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt32Bit_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorInt32BitStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorInt32BitStatics {
    type Vtable = ITensorInt32BitStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorInt32BitStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6539864b_52fa_4e35_907c_834cac417b50);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt32BitStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorInt32BitStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorInt32BitStatics2 {
    type Vtable = ITensorInt32BitStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorInt32BitStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7c4b079a_e956_5ce0_a3bd_157d9d79b5ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt32BitStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorInt64Bit(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorInt64Bit {
    type Vtable = ITensorInt64Bit_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorInt64Bit {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x499665ba_1fa2_45ad_af25_a0bd9bda4c87);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt64Bit_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorInt64BitStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorInt64BitStatics {
    type Vtable = ITensorInt64BitStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorInt64BitStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9648ad9d_1198_4d74_9517_783ab62b9cc2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt64BitStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const i64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorInt64BitStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorInt64BitStatics2 {
    type Vtable = ITensorInt64BitStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorInt64BitStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6d3d9dcb_ff40_5ec2_89fe_084e2b6bc6db);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt64BitStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const i64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorInt8Bit(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorInt8Bit {
    type Vtable = ITensorInt8Bit_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorInt8Bit {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcddd97c5_ffd8_4fef_aefb_30e1a485b2ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt8Bit_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorInt8BitStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorInt8BitStatics {
    type Vtable = ITensorInt8BitStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorInt8BitStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb1a12284_095c_4c76_a661_ac4cee1f3e8b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt8BitStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorInt8BitStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorInt8BitStatics2 {
    type Vtable = ITensorInt8BitStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorInt8BitStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc0d59637_c468_56fb_9535_c052bdb93dc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt8BitStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorString(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorString {
    type Vtable = ITensorString_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorString {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x582335c8_bdb1_4610_bc75_35e9cbf009b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorString_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorStringStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorStringStatics {
    type Vtable = ITensorStringStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorStringStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x83623324_cf26_4f17_a2d4_20ef8d097d53);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorStringStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorStringStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorStringStatics2 {
    type Vtable = ITensorStringStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorStringStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9e355ed0_c8e2_5254_9137_0193a3668fd8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorStringStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorUInt16Bit(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorUInt16Bit {
    type Vtable = ITensorUInt16Bit_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorUInt16Bit {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x68140f4b_23c0_42f3_81f6_a891c011bc3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt16Bit_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorUInt16BitStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorUInt16BitStatics {
    type Vtable = ITensorUInt16BitStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorUInt16BitStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5df745dd_028a_481a_a27c_c7e6435e52dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt16BitStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorUInt16BitStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorUInt16BitStatics2 {
    type Vtable = ITensorUInt16BitStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorUInt16BitStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8af40c64_d69f_5315_9348_490877bbd642);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt16BitStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorUInt32Bit(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorUInt32Bit {
    type Vtable = ITensorUInt32Bit_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorUInt32Bit {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd8c9c2ff_7511_45a3_bfac_c38f370d2237);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt32Bit_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorUInt32BitStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorUInt32BitStatics {
    type Vtable = ITensorUInt32BitStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorUInt32BitStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x417c3837_e773_4378_8e7f_0cc33dbea697);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt32BitStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorUInt32BitStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorUInt32BitStatics2 {
    type Vtable = ITensorUInt32BitStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorUInt32BitStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef1a1f1c_314e_569d_b496_5c8447d20cd2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt32BitStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorUInt64Bit(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorUInt64Bit {
    type Vtable = ITensorUInt64Bit_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorUInt64Bit {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2e70ffad_04bf_4825_839a_82baef8c7886);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt64Bit_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorUInt64BitStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorUInt64BitStatics {
    type Vtable = ITensorUInt64BitStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorUInt64BitStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7a7e20eb_242f_47cb_a9c6_f602ecfbfee4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt64BitStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorUInt64BitStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorUInt64BitStatics2 {
    type Vtable = ITensorUInt64BitStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorUInt64BitStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x085a687d_67e1_5b1e_b232_4fabe9ca20b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt64BitStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorUInt8Bit(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorUInt8Bit {
    type Vtable = ITensorUInt8Bit_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorUInt8Bit {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x58e1ae27_622b_48e3_be22_d867aed1daac);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt8Bit_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorUInt8BitStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorUInt8BitStatics {
    type Vtable = ITensorUInt8BitStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorUInt8BitStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x05f67583_bc24_4220_8a41_2dcd8c5ed33c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt8BitStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITensorUInt8BitStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITensorUInt8BitStatics2 {
    type Vtable = ITensorUInt8BitStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITensorUInt8BitStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2ba042d6_373e_5a3a_a2fc_a6c41bd52789);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt8BitStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ImageFeatureDescriptor(::windows_core::IUnknown);
impl ImageFeatureDescriptor {
    #[doc = "Required features: `\"Graphics_Imaging\"`"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn BitmapPixelFormat(&self) -> ::windows_core::Result<super::super::Graphics::Imaging::BitmapPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BitmapPixelFormat)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Graphics_Imaging\"`"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn BitmapAlphaMode(&self) -> ::windows_core::Result<super::super::Graphics::Imaging::BitmapAlphaMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BitmapAlphaMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Width(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Width)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Height(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Height)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PixelRange(&self) -> ::windows_core::Result<LearningModelPixelRange> {
        let this = &::windows_core::ComInterface::cast::<IImageFeatureDescriptor2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PixelRange)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<LearningModelFeatureKind> {
        let this = &::windows_core::ComInterface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsRequired(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsRequired)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ImageFeatureDescriptor {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.ImageFeatureDescriptor;{365585a5-171a-4a2a-985f-265159d3895a})");
}
unsafe impl ::windows_core::Interface for ImageFeatureDescriptor {
    type Vtable = IImageFeatureDescriptor_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ImageFeatureDescriptor {
    const IID: ::windows_core::GUID = <IImageFeatureDescriptor as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ImageFeatureDescriptor {
    const NAME: &'static str = "Windows.AI.MachineLearning.ImageFeatureDescriptor";
}
::windows_core::imp::interface_hierarchy!(ImageFeatureDescriptor, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<ILearningModelFeatureDescriptor> for ImageFeatureDescriptor {}
unsafe impl ::core::marker::Send for ImageFeatureDescriptor {}
unsafe impl ::core::marker::Sync for ImageFeatureDescriptor {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ImageFeatureValue(::windows_core::IUnknown);
impl ImageFeatureValue {
    #[doc = "Required features: `\"Media\"`"]
    #[cfg(feature = "Media")]
    pub fn VideoFrame(&self) -> ::windows_core::Result<super::super::Media::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFrame)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Media\"`"]
    #[cfg(feature = "Media")]
    pub fn CreateFromVideoFrame<P0>(image: P0) -> ::windows_core::Result<ImageFeatureValue>
    where
        P0: ::windows_core::IntoParam<super::super::Media::VideoFrame>,
    {
        Self::IImageFeatureValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromVideoFrame)(::windows_core::Interface::as_raw(this), image.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn Kind(&self) -> ::windows_core::Result<LearningModelFeatureKind> {
        let this = &::windows_core::ComInterface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IImageFeatureValueStatics<R, F: FnOnce(&IImageFeatureValueStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ImageFeatureValue, IImageFeatureValueStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for ImageFeatureValue {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.ImageFeatureValue;{f0414fd9-c9aa-4405-b7fb-94f87c8a3037})");
}
unsafe impl ::windows_core::Interface for ImageFeatureValue {
    type Vtable = IImageFeatureValue_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ImageFeatureValue {
    const IID: ::windows_core::GUID = <IImageFeatureValue as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ImageFeatureValue {
    const NAME: &'static str = "Windows.AI.MachineLearning.ImageFeatureValue";
}
::windows_core::imp::interface_hierarchy!(ImageFeatureValue, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<ILearningModelFeatureValue> for ImageFeatureValue {}
unsafe impl ::core::marker::Send for ImageFeatureValue {}
unsafe impl ::core::marker::Sync for ImageFeatureValue {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct LearningModel(::windows_core::IUnknown);
impl LearningModel {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Author(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Author)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Domain(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Domain)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Version(&self) -> ::windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Version)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Metadata(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Metadata)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InputFeatures(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<ILearningModelFeatureDescriptor>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InputFeatures)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutputFeatures(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<ILearningModelFeatureDescriptor>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OutputFeatures)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LoadFromStorageFileAsync<P0>(modelfile: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<LearningModel>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::IStorageFile>,
    {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LoadFromStorageFileAsync)(::windows_core::Interface::as_raw(this), modelfile.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn LoadFromStreamAsync<P0>(modelstream: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<LearningModel>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LoadFromStreamAsync)(::windows_core::Interface::as_raw(this), modelstream.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn LoadFromFilePath(filepath: &::windows_core::HSTRING) -> ::windows_core::Result<LearningModel> {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LoadFromFilePath)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(filepath), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadFromStream<P0>(modelstream: P0) -> ::windows_core::Result<LearningModel>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LoadFromStream)(::windows_core::Interface::as_raw(this), modelstream.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LoadFromStorageFileWithOperatorProviderAsync<P0, P1>(modelfile: P0, operatorprovider: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<LearningModel>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::IStorageFile>,
        P1: ::windows_core::TryIntoParam<ILearningModelOperatorProvider>,
    {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LoadFromStorageFileWithOperatorProviderAsync)(::windows_core::Interface::as_raw(this), modelfile.try_into_param()?.abi(), operatorprovider.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn LoadFromStreamWithOperatorProviderAsync<P0, P1>(modelstream: P0, operatorprovider: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<LearningModel>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStreamReference>,
        P1: ::windows_core::TryIntoParam<ILearningModelOperatorProvider>,
    {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LoadFromStreamWithOperatorProviderAsync)(::windows_core::Interface::as_raw(this), modelstream.try_into_param()?.abi(), operatorprovider.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn LoadFromFilePathWithOperatorProvider<P0>(filepath: &::windows_core::HSTRING, operatorprovider: P0) -> ::windows_core::Result<LearningModel>
    where
        P0: ::windows_core::TryIntoParam<ILearningModelOperatorProvider>,
    {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LoadFromFilePathWithOperatorProvider)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(filepath), operatorprovider.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadFromStreamWithOperatorProvider<P0, P1>(modelstream: P0, operatorprovider: P1) -> ::windows_core::Result<LearningModel>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStreamReference>,
        P1: ::windows_core::TryIntoParam<ILearningModelOperatorProvider>,
    {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LoadFromStreamWithOperatorProvider)(::windows_core::Interface::as_raw(this), modelstream.try_into_param()?.abi(), operatorprovider.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILearningModelStatics<R, F: FnOnce(&ILearningModelStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<LearningModel, ILearningModelStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for LearningModel {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.LearningModel;{5b8e4920-489f-4e86-9128-265a327b78fa})");
}
unsafe impl ::windows_core::Interface for LearningModel {
    type Vtable = ILearningModel_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LearningModel {
    const IID: ::windows_core::GUID = <ILearningModel as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LearningModel {
    const NAME: &'static str = "Windows.AI.MachineLearning.LearningModel";
}
::windows_core::imp::interface_hierarchy!(LearningModel, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for LearningModel {}
unsafe impl ::core::marker::Send for LearningModel {}
unsafe impl ::core::marker::Sync for LearningModel {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct LearningModelBinding(::windows_core::IUnknown);
impl LearningModelBinding {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Bind<P0>(&self, name: &::windows_core::HSTRING, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Bind)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.into_param().abi()).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn BindWithProperties<P0, P1>(&self, name: &::windows_core::HSTRING, value: P0, props: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IPropertySet>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).BindWithProperties)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.into_param().abi(), props.try_into_param()?.abi()).ok() }
    }
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CreateFromSession<P0>(session: P0) -> ::windows_core::Result<LearningModelBinding>
    where
        P0: ::windows_core::IntoParam<LearningModelSession>,
    {
        Self::ILearningModelBindingFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromSession)(::windows_core::Interface::as_raw(this), session.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Lookup)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasKey)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Split(&self, first: &mut ::core::option::Option<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>, second: &mut ::core::option::Option<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Split)(::windows_core::Interface::as_raw(this), first as *mut _ as _, second as *mut _ as _).ok() }
    }
    #[doc(hidden)]
    pub fn ILearningModelBindingFactory<R, F: FnOnce(&ILearningModelBindingFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<LearningModelBinding, ILearningModelBindingFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for LearningModelBinding {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.LearningModelBinding;{ea312f20-168f-4f8c-94fe-2e7ac31b4aa8})");
}
unsafe impl ::windows_core::Interface for LearningModelBinding {
    type Vtable = ILearningModelBinding_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LearningModelBinding {
    const IID: ::windows_core::GUID = <ILearningModelBinding as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LearningModelBinding {
    const NAME: &'static str = "Windows.AI.MachineLearning.LearningModelBinding";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for LearningModelBinding {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &LearningModelBinding {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
::windows_core::imp::interface_hierarchy!(LearningModelBinding, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>> for LearningModelBinding {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> for LearningModelBinding {}
unsafe impl ::core::marker::Send for LearningModelBinding {}
unsafe impl ::core::marker::Sync for LearningModelBinding {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct LearningModelDevice(::windows_core::IUnknown);
impl LearningModelDevice {
    #[doc = "Required features: `\"Graphics\"`"]
    #[cfg(feature = "Graphics")]
    pub fn AdapterId(&self) -> ::windows_core::Result<super::super::Graphics::DisplayAdapterId> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AdapterId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Graphics_DirectX_Direct3D11\"`"]
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Direct3D11Device(&self) -> ::windows_core::Result<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Direct3D11Device)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(devicekind: LearningModelDeviceKind) -> ::windows_core::Result<LearningModelDevice> {
        Self::ILearningModelDeviceFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), devicekind, &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Graphics_DirectX_Direct3D11\"`"]
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CreateFromDirect3D11Device<P0>(device: P0) -> ::windows_core::Result<LearningModelDevice>
    where
        P0: ::windows_core::TryIntoParam<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>,
    {
        Self::ILearningModelDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromDirect3D11Device)(::windows_core::Interface::as_raw(this), device.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILearningModelDeviceFactory<R, F: FnOnce(&ILearningModelDeviceFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<LearningModelDevice, ILearningModelDeviceFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ILearningModelDeviceStatics<R, F: FnOnce(&ILearningModelDeviceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<LearningModelDevice, ILearningModelDeviceStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for LearningModelDevice {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.LearningModelDevice;{f5c2c8fe-3f56-4a8c-ac5f-fdb92d8b8252})");
}
unsafe impl ::windows_core::Interface for LearningModelDevice {
    type Vtable = ILearningModelDevice_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LearningModelDevice {
    const IID: ::windows_core::GUID = <ILearningModelDevice as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LearningModelDevice {
    const NAME: &'static str = "Windows.AI.MachineLearning.LearningModelDevice";
}
::windows_core::imp::interface_hierarchy!(LearningModelDevice, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for LearningModelDevice {}
unsafe impl ::core::marker::Sync for LearningModelDevice {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct LearningModelEvaluationResult(::windows_core::IUnknown);
impl LearningModelEvaluationResult {
    pub fn CorrelationId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CorrelationId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ErrorStatus(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Succeeded(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Succeeded)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Outputs(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Outputs)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for LearningModelEvaluationResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.LearningModelEvaluationResult;{b2f9bfcd-960e-49c0-8593-eb190ae3eee2})");
}
unsafe impl ::windows_core::Interface for LearningModelEvaluationResult {
    type Vtable = ILearningModelEvaluationResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LearningModelEvaluationResult {
    const IID: ::windows_core::GUID = <ILearningModelEvaluationResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LearningModelEvaluationResult {
    const NAME: &'static str = "Windows.AI.MachineLearning.LearningModelEvaluationResult";
}
::windows_core::imp::interface_hierarchy!(LearningModelEvaluationResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for LearningModelEvaluationResult {}
unsafe impl ::core::marker::Sync for LearningModelEvaluationResult {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct LearningModelSession(::windows_core::IUnknown);
impl LearningModelSession {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Model(&self) -> ::windows_core::Result<LearningModel> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Model)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Device(&self) -> ::windows_core::Result<LearningModelDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Device)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn EvaluationProperties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EvaluationProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn EvaluateAsync<P0>(&self, bindings: P0, correlationid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<LearningModelEvaluationResult>>
    where
        P0: ::windows_core::IntoParam<LearningModelBinding>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EvaluateAsync)(::windows_core::Interface::as_raw(this), bindings.into_param().abi(), ::core::mem::transmute_copy(correlationid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn EvaluateFeaturesAsync<P0>(&self, features: P0, correlationid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<LearningModelEvaluationResult>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EvaluateFeaturesAsync)(::windows_core::Interface::as_raw(this), features.try_into_param()?.abi(), ::core::mem::transmute_copy(correlationid), &mut result__).from_abi(result__)
        }
    }
    pub fn Evaluate<P0>(&self, bindings: P0, correlationid: &::windows_core::HSTRING) -> ::windows_core::Result<LearningModelEvaluationResult>
    where
        P0: ::windows_core::IntoParam<LearningModelBinding>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Evaluate)(::windows_core::Interface::as_raw(this), bindings.into_param().abi(), ::core::mem::transmute_copy(correlationid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn EvaluateFeatures<P0>(&self, features: P0, correlationid: &::windows_core::HSTRING) -> ::windows_core::Result<LearningModelEvaluationResult>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EvaluateFeatures)(::windows_core::Interface::as_raw(this), features.try_into_param()?.abi(), ::core::mem::transmute_copy(correlationid), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateFromModel<P0>(model: P0) -> ::windows_core::Result<LearningModelSession>
    where
        P0: ::windows_core::IntoParam<LearningModel>,
    {
        Self::ILearningModelSessionFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromModel)(::windows_core::Interface::as_raw(this), model.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromModelOnDevice<P0, P1>(model: P0, devicetorunon: P1) -> ::windows_core::Result<LearningModelSession>
    where
        P0: ::windows_core::IntoParam<LearningModel>,
        P1: ::windows_core::IntoParam<LearningModelDevice>,
    {
        Self::ILearningModelSessionFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromModelOnDevice)(::windows_core::Interface::as_raw(this), model.into_param().abi(), devicetorunon.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromModelOnDeviceWithSessionOptions<P0, P1, P2>(model: P0, devicetorunon: P1, learningmodelsessionoptions: P2) -> ::windows_core::Result<LearningModelSession>
    where
        P0: ::windows_core::IntoParam<LearningModel>,
        P1: ::windows_core::IntoParam<LearningModelDevice>,
        P2: ::windows_core::IntoParam<LearningModelSessionOptions>,
    {
        Self::ILearningModelSessionFactory2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromModelOnDeviceWithSessionOptions)(::windows_core::Interface::as_raw(this), model.into_param().abi(), devicetorunon.into_param().abi(), learningmodelsessionoptions.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILearningModelSessionFactory<R, F: FnOnce(&ILearningModelSessionFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<LearningModelSession, ILearningModelSessionFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ILearningModelSessionFactory2<R, F: FnOnce(&ILearningModelSessionFactory2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<LearningModelSession, ILearningModelSessionFactory2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for LearningModelSession {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.LearningModelSession;{8e58f8f6-b787-4c11-90f0-7129aeca74a9})");
}
unsafe impl ::windows_core::Interface for LearningModelSession {
    type Vtable = ILearningModelSession_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LearningModelSession {
    const IID: ::windows_core::GUID = <ILearningModelSession as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LearningModelSession {
    const NAME: &'static str = "Windows.AI.MachineLearning.LearningModelSession";
}
::windows_core::imp::interface_hierarchy!(LearningModelSession, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for LearningModelSession {}
unsafe impl ::core::marker::Send for LearningModelSession {}
unsafe impl ::core::marker::Sync for LearningModelSession {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct LearningModelSessionOptions(::windows_core::IUnknown);
impl LearningModelSessionOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<LearningModelSessionOptions, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn BatchSizeOverride(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BatchSizeOverride)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBatchSizeOverride(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBatchSizeOverride)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CloseModelOnSessionCreation(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ILearningModelSessionOptions2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CloseModelOnSessionCreation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCloseModelOnSessionCreation(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ILearningModelSessionOptions2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCloseModelOnSessionCreation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OverrideNamedDimension(&self, name: &::windows_core::HSTRING, dimension: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ILearningModelSessionOptions3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).OverrideNamedDimension)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), dimension).ok() }
    }
}
impl ::windows_core::RuntimeType for LearningModelSessionOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.LearningModelSessionOptions;{b8f63fa1-134d-5133-8cff-3a5c3c263beb})");
}
unsafe impl ::windows_core::Interface for LearningModelSessionOptions {
    type Vtable = ILearningModelSessionOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LearningModelSessionOptions {
    const IID: ::windows_core::GUID = <ILearningModelSessionOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LearningModelSessionOptions {
    const NAME: &'static str = "Windows.AI.MachineLearning.LearningModelSessionOptions";
}
::windows_core::imp::interface_hierarchy!(LearningModelSessionOptions, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for LearningModelSessionOptions {}
unsafe impl ::core::marker::Sync for LearningModelSessionOptions {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MapFeatureDescriptor(::windows_core::IUnknown);
impl MapFeatureDescriptor {
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<LearningModelFeatureKind> {
        let this = &::windows_core::ComInterface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsRequired(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsRequired)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn KeyKind(&self) -> ::windows_core::Result<TensorKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ValueDescriptor(&self) -> ::windows_core::Result<ILearningModelFeatureDescriptor> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ValueDescriptor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MapFeatureDescriptor {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.MapFeatureDescriptor;{530424bd-a257-436d-9e60-c2981f7cc5c4})");
}
unsafe impl ::windows_core::Interface for MapFeatureDescriptor {
    type Vtable = IMapFeatureDescriptor_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MapFeatureDescriptor {
    const IID: ::windows_core::GUID = <IMapFeatureDescriptor as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MapFeatureDescriptor {
    const NAME: &'static str = "Windows.AI.MachineLearning.MapFeatureDescriptor";
}
::windows_core::imp::interface_hierarchy!(MapFeatureDescriptor, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<ILearningModelFeatureDescriptor> for MapFeatureDescriptor {}
unsafe impl ::core::marker::Send for MapFeatureDescriptor {}
unsafe impl ::core::marker::Sync for MapFeatureDescriptor {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SequenceFeatureDescriptor(::windows_core::IUnknown);
impl SequenceFeatureDescriptor {
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<LearningModelFeatureKind> {
        let this = &::windows_core::ComInterface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsRequired(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsRequired)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ElementDescriptor(&self) -> ::windows_core::Result<ILearningModelFeatureDescriptor> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementDescriptor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for SequenceFeatureDescriptor {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.SequenceFeatureDescriptor;{84f6945a-562b-4d62-a851-739aced96668})");
}
unsafe impl ::windows_core::Interface for SequenceFeatureDescriptor {
    type Vtable = ISequenceFeatureDescriptor_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SequenceFeatureDescriptor {
    const IID: ::windows_core::GUID = <ISequenceFeatureDescriptor as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SequenceFeatureDescriptor {
    const NAME: &'static str = "Windows.AI.MachineLearning.SequenceFeatureDescriptor";
}
::windows_core::imp::interface_hierarchy!(SequenceFeatureDescriptor, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<ILearningModelFeatureDescriptor> for SequenceFeatureDescriptor {}
unsafe impl ::core::marker::Send for SequenceFeatureDescriptor {}
unsafe impl ::core::marker::Sync for SequenceFeatureDescriptor {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TensorBoolean(::windows_core::IUnknown);
impl TensorBoolean {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> ::windows_core::Result<LearningModelFeatureKind> {
        let this = &::windows_core::ComInterface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateReference)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TensorKind(&self) -> ::windows_core::Result<TensorKind> {
        let this = &::windows_core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TensorKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows_core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Shape)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAsVectorView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create() -> ::windows_core::Result<TensorBoolean> {
        Self::ITensorBooleanStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> ::windows_core::Result<TensorBoolean>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorBooleanStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create2)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[bool]) -> ::windows_core::Result<TensorBoolean>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorBooleanStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromArray)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> ::windows_core::Result<TensorBoolean>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<bool>>,
    {
        Self::ITensorBooleanStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromIterable)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[bool]) -> ::windows_core::Result<TensorBoolean> {
        Self::ITensorBooleanStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(shape: &[i64], buffer: P0) -> ::windows_core::Result<TensorBoolean>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorBooleanStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromBuffer)(::windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorBooleanStatics<R, F: FnOnce(&ITensorBooleanStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TensorBoolean, ITensorBooleanStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorBooleanStatics2<R, F: FnOnce(&ITensorBooleanStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TensorBoolean, ITensorBooleanStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for TensorBoolean {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorBoolean;{50f311ed-29e9-4a5c-a44d-8fc512584eed})");
}
unsafe impl ::windows_core::Interface for TensorBoolean {
    type Vtable = ITensorBoolean_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TensorBoolean {
    const IID: ::windows_core::GUID = <ITensorBoolean as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TensorBoolean {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorBoolean";
}
::windows_core::imp::interface_hierarchy!(TensorBoolean, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for TensorBoolean {}
impl ::windows_core::CanTryInto<ILearningModelFeatureValue> for TensorBoolean {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IMemoryBuffer> for TensorBoolean {}
impl ::windows_core::CanTryInto<ITensor> for TensorBoolean {}
unsafe impl ::core::marker::Send for TensorBoolean {}
unsafe impl ::core::marker::Sync for TensorBoolean {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TensorDouble(::windows_core::IUnknown);
impl TensorDouble {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> ::windows_core::Result<LearningModelFeatureKind> {
        let this = &::windows_core::ComInterface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateReference)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TensorKind(&self) -> ::windows_core::Result<TensorKind> {
        let this = &::windows_core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TensorKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows_core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Shape)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAsVectorView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create() -> ::windows_core::Result<TensorDouble> {
        Self::ITensorDoubleStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> ::windows_core::Result<TensorDouble>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorDoubleStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create2)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[f64]) -> ::windows_core::Result<TensorDouble>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorDoubleStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromArray)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> ::windows_core::Result<TensorDouble>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<f64>>,
    {
        Self::ITensorDoubleStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromIterable)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[f64]) -> ::windows_core::Result<TensorDouble> {
        Self::ITensorDoubleStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(shape: &[i64], buffer: P0) -> ::windows_core::Result<TensorDouble>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorDoubleStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromBuffer)(::windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorDoubleStatics<R, F: FnOnce(&ITensorDoubleStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TensorDouble, ITensorDoubleStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorDoubleStatics2<R, F: FnOnce(&ITensorDoubleStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TensorDouble, ITensorDoubleStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for TensorDouble {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorDouble;{91e41252-7a8f-4f0e-a28f-9637ffc8a3d0})");
}
unsafe impl ::windows_core::Interface for TensorDouble {
    type Vtable = ITensorDouble_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TensorDouble {
    const IID: ::windows_core::GUID = <ITensorDouble as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TensorDouble {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorDouble";
}
::windows_core::imp::interface_hierarchy!(TensorDouble, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for TensorDouble {}
impl ::windows_core::CanTryInto<ILearningModelFeatureValue> for TensorDouble {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IMemoryBuffer> for TensorDouble {}
impl ::windows_core::CanTryInto<ITensor> for TensorDouble {}
unsafe impl ::core::marker::Send for TensorDouble {}
unsafe impl ::core::marker::Sync for TensorDouble {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TensorFeatureDescriptor(::windows_core::IUnknown);
impl TensorFeatureDescriptor {
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<LearningModelFeatureKind> {
        let this = &::windows_core::ComInterface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsRequired(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsRequired)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TensorKind(&self) -> ::windows_core::Result<TensorKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TensorKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Shape)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for TensorFeatureDescriptor {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorFeatureDescriptor;{74455c80-946a-4310-a19c-ee0af028fce4})");
}
unsafe impl ::windows_core::Interface for TensorFeatureDescriptor {
    type Vtable = ITensorFeatureDescriptor_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TensorFeatureDescriptor {
    const IID: ::windows_core::GUID = <ITensorFeatureDescriptor as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TensorFeatureDescriptor {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorFeatureDescriptor";
}
::windows_core::imp::interface_hierarchy!(TensorFeatureDescriptor, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<ILearningModelFeatureDescriptor> for TensorFeatureDescriptor {}
unsafe impl ::core::marker::Send for TensorFeatureDescriptor {}
unsafe impl ::core::marker::Sync for TensorFeatureDescriptor {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TensorFloat(::windows_core::IUnknown);
impl TensorFloat {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> ::windows_core::Result<LearningModelFeatureKind> {
        let this = &::windows_core::ComInterface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateReference)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TensorKind(&self) -> ::windows_core::Result<TensorKind> {
        let this = &::windows_core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TensorKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows_core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Shape)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<f32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAsVectorView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create() -> ::windows_core::Result<TensorFloat> {
        Self::ITensorFloatStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> ::windows_core::Result<TensorFloat>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorFloatStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create2)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[f32]) -> ::windows_core::Result<TensorFloat>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorFloatStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromArray)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> ::windows_core::Result<TensorFloat>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<f32>>,
    {
        Self::ITensorFloatStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromIterable)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[f32]) -> ::windows_core::Result<TensorFloat> {
        Self::ITensorFloatStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(shape: &[i64], buffer: P0) -> ::windows_core::Result<TensorFloat>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorFloatStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromBuffer)(::windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorFloatStatics<R, F: FnOnce(&ITensorFloatStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TensorFloat, ITensorFloatStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorFloatStatics2<R, F: FnOnce(&ITensorFloatStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TensorFloat, ITensorFloatStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for TensorFloat {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorFloat;{f2282d82-aa02-42c8-a0c8-df1efc9676e1})");
}
unsafe impl ::windows_core::Interface for TensorFloat {
    type Vtable = ITensorFloat_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TensorFloat {
    const IID: ::windows_core::GUID = <ITensorFloat as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TensorFloat {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorFloat";
}
::windows_core::imp::interface_hierarchy!(TensorFloat, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for TensorFloat {}
impl ::windows_core::CanTryInto<ILearningModelFeatureValue> for TensorFloat {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IMemoryBuffer> for TensorFloat {}
impl ::windows_core::CanTryInto<ITensor> for TensorFloat {}
unsafe impl ::core::marker::Send for TensorFloat {}
unsafe impl ::core::marker::Sync for TensorFloat {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TensorFloat16Bit(::windows_core::IUnknown);
impl TensorFloat16Bit {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> ::windows_core::Result<LearningModelFeatureKind> {
        let this = &::windows_core::ComInterface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateReference)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TensorKind(&self) -> ::windows_core::Result<TensorKind> {
        let this = &::windows_core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TensorKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows_core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Shape)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<f32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAsVectorView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create() -> ::windows_core::Result<TensorFloat16Bit> {
        Self::ITensorFloat16BitStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> ::windows_core::Result<TensorFloat16Bit>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorFloat16BitStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create2)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[f32]) -> ::windows_core::Result<TensorFloat16Bit>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorFloat16BitStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromArray)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> ::windows_core::Result<TensorFloat16Bit>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<f32>>,
    {
        Self::ITensorFloat16BitStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromIterable)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[f32]) -> ::windows_core::Result<TensorFloat16Bit> {
        Self::ITensorFloat16BitStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(shape: &[i64], buffer: P0) -> ::windows_core::Result<TensorFloat16Bit>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorFloat16BitStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromBuffer)(::windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorFloat16BitStatics<R, F: FnOnce(&ITensorFloat16BitStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TensorFloat16Bit, ITensorFloat16BitStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorFloat16BitStatics2<R, F: FnOnce(&ITensorFloat16BitStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TensorFloat16Bit, ITensorFloat16BitStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for TensorFloat16Bit {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorFloat16Bit;{0ab994fc-5b89-4c3c-b5e4-5282a5316c0a})");
}
unsafe impl ::windows_core::Interface for TensorFloat16Bit {
    type Vtable = ITensorFloat16Bit_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TensorFloat16Bit {
    const IID: ::windows_core::GUID = <ITensorFloat16Bit as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TensorFloat16Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorFloat16Bit";
}
::windows_core::imp::interface_hierarchy!(TensorFloat16Bit, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for TensorFloat16Bit {}
impl ::windows_core::CanTryInto<ILearningModelFeatureValue> for TensorFloat16Bit {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IMemoryBuffer> for TensorFloat16Bit {}
impl ::windows_core::CanTryInto<ITensor> for TensorFloat16Bit {}
unsafe impl ::core::marker::Send for TensorFloat16Bit {}
unsafe impl ::core::marker::Sync for TensorFloat16Bit {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TensorInt16Bit(::windows_core::IUnknown);
impl TensorInt16Bit {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> ::windows_core::Result<LearningModelFeatureKind> {
        let this = &::windows_core::ComInterface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateReference)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TensorKind(&self) -> ::windows_core::Result<TensorKind> {
        let this = &::windows_core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TensorKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows_core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Shape)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<i16>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAsVectorView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create() -> ::windows_core::Result<TensorInt16Bit> {
        Self::ITensorInt16BitStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> ::windows_core::Result<TensorInt16Bit>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorInt16BitStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create2)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[i16]) -> ::windows_core::Result<TensorInt16Bit>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorInt16BitStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromArray)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> ::windows_core::Result<TensorInt16Bit>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i16>>,
    {
        Self::ITensorInt16BitStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromIterable)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[i16]) -> ::windows_core::Result<TensorInt16Bit> {
        Self::ITensorInt16BitStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(shape: &[i64], buffer: P0) -> ::windows_core::Result<TensorInt16Bit>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorInt16BitStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromBuffer)(::windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorInt16BitStatics<R, F: FnOnce(&ITensorInt16BitStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TensorInt16Bit, ITensorInt16BitStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorInt16BitStatics2<R, F: FnOnce(&ITensorInt16BitStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TensorInt16Bit, ITensorInt16BitStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for TensorInt16Bit {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorInt16Bit;{98a32d39-e6d6-44af-8afa-baebc44dc020})");
}
unsafe impl ::windows_core::Interface for TensorInt16Bit {
    type Vtable = ITensorInt16Bit_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TensorInt16Bit {
    const IID: ::windows_core::GUID = <ITensorInt16Bit as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TensorInt16Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorInt16Bit";
}
::windows_core::imp::interface_hierarchy!(TensorInt16Bit, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for TensorInt16Bit {}
impl ::windows_core::CanTryInto<ILearningModelFeatureValue> for TensorInt16Bit {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IMemoryBuffer> for TensorInt16Bit {}
impl ::windows_core::CanTryInto<ITensor> for TensorInt16Bit {}
unsafe impl ::core::marker::Send for TensorInt16Bit {}
unsafe impl ::core::marker::Sync for TensorInt16Bit {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TensorInt32Bit(::windows_core::IUnknown);
impl TensorInt32Bit {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> ::windows_core::Result<LearningModelFeatureKind> {
        let this = &::windows_core::ComInterface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateReference)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TensorKind(&self) -> ::windows_core::Result<TensorKind> {
        let this = &::windows_core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TensorKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows_core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Shape)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAsVectorView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create() -> ::windows_core::Result<TensorInt32Bit> {
        Self::ITensorInt32BitStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> ::windows_core::Result<TensorInt32Bit>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorInt32BitStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create2)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[i32]) -> ::windows_core::Result<TensorInt32Bit>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorInt32BitStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromArray)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> ::windows_core::Result<TensorInt32Bit>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i32>>,
    {
        Self::ITensorInt32BitStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromIterable)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[i32]) -> ::windows_core::Result<TensorInt32Bit> {
        Self::ITensorInt32BitStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(shape: &[i64], buffer: P0) -> ::windows_core::Result<TensorInt32Bit>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorInt32BitStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromBuffer)(::windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorInt32BitStatics<R, F: FnOnce(&ITensorInt32BitStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TensorInt32Bit, ITensorInt32BitStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorInt32BitStatics2<R, F: FnOnce(&ITensorInt32BitStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TensorInt32Bit, ITensorInt32BitStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for TensorInt32Bit {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorInt32Bit;{2c0c28d3-207c-4486-a7d2-884522c5e589})");
}
unsafe impl ::windows_core::Interface for TensorInt32Bit {
    type Vtable = ITensorInt32Bit_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TensorInt32Bit {
    const IID: ::windows_core::GUID = <ITensorInt32Bit as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TensorInt32Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorInt32Bit";
}
::windows_core::imp::interface_hierarchy!(TensorInt32Bit, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for TensorInt32Bit {}
impl ::windows_core::CanTryInto<ILearningModelFeatureValue> for TensorInt32Bit {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IMemoryBuffer> for TensorInt32Bit {}
impl ::windows_core::CanTryInto<ITensor> for TensorInt32Bit {}
unsafe impl ::core::marker::Send for TensorInt32Bit {}
unsafe impl ::core::marker::Sync for TensorInt32Bit {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TensorInt64Bit(::windows_core::IUnknown);
impl TensorInt64Bit {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> ::windows_core::Result<LearningModelFeatureKind> {
        let this = &::windows_core::ComInterface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateReference)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TensorKind(&self) -> ::windows_core::Result<TensorKind> {
        let this = &::windows_core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TensorKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows_core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Shape)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAsVectorView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create() -> ::windows_core::Result<TensorInt64Bit> {
        Self::ITensorInt64BitStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> ::windows_core::Result<TensorInt64Bit>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorInt64BitStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create2)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[i64]) -> ::windows_core::Result<TensorInt64Bit>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorInt64BitStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromArray)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> ::windows_core::Result<TensorInt64Bit>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorInt64BitStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromIterable)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[i64]) -> ::windows_core::Result<TensorInt64Bit> {
        Self::ITensorInt64BitStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(shape: &[i64], buffer: P0) -> ::windows_core::Result<TensorInt64Bit>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorInt64BitStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromBuffer)(::windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorInt64BitStatics<R, F: FnOnce(&ITensorInt64BitStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TensorInt64Bit, ITensorInt64BitStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorInt64BitStatics2<R, F: FnOnce(&ITensorInt64BitStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TensorInt64Bit, ITensorInt64BitStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for TensorInt64Bit {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorInt64Bit;{499665ba-1fa2-45ad-af25-a0bd9bda4c87})");
}
unsafe impl ::windows_core::Interface for TensorInt64Bit {
    type Vtable = ITensorInt64Bit_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TensorInt64Bit {
    const IID: ::windows_core::GUID = <ITensorInt64Bit as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TensorInt64Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorInt64Bit";
}
::windows_core::imp::interface_hierarchy!(TensorInt64Bit, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for TensorInt64Bit {}
impl ::windows_core::CanTryInto<ILearningModelFeatureValue> for TensorInt64Bit {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IMemoryBuffer> for TensorInt64Bit {}
impl ::windows_core::CanTryInto<ITensor> for TensorInt64Bit {}
unsafe impl ::core::marker::Send for TensorInt64Bit {}
unsafe impl ::core::marker::Sync for TensorInt64Bit {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TensorInt8Bit(::windows_core::IUnknown);
impl TensorInt8Bit {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> ::windows_core::Result<LearningModelFeatureKind> {
        let this = &::windows_core::ComInterface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateReference)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TensorKind(&self) -> ::windows_core::Result<TensorKind> {
        let this = &::windows_core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TensorKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows_core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Shape)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAsVectorView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create() -> ::windows_core::Result<TensorInt8Bit> {
        Self::ITensorInt8BitStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> ::windows_core::Result<TensorInt8Bit>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorInt8BitStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create2)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[u8]) -> ::windows_core::Result<TensorInt8Bit>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorInt8BitStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromArray)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> ::windows_core::Result<TensorInt8Bit>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<u8>>,
    {
        Self::ITensorInt8BitStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromIterable)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[u8]) -> ::windows_core::Result<TensorInt8Bit> {
        Self::ITensorInt8BitStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(shape: &[i64], buffer: P0) -> ::windows_core::Result<TensorInt8Bit>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorInt8BitStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromBuffer)(::windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorInt8BitStatics<R, F: FnOnce(&ITensorInt8BitStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TensorInt8Bit, ITensorInt8BitStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorInt8BitStatics2<R, F: FnOnce(&ITensorInt8BitStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TensorInt8Bit, ITensorInt8BitStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for TensorInt8Bit {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorInt8Bit;{cddd97c5-ffd8-4fef-aefb-30e1a485b2ee})");
}
unsafe impl ::windows_core::Interface for TensorInt8Bit {
    type Vtable = ITensorInt8Bit_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TensorInt8Bit {
    const IID: ::windows_core::GUID = <ITensorInt8Bit as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TensorInt8Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorInt8Bit";
}
::windows_core::imp::interface_hierarchy!(TensorInt8Bit, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for TensorInt8Bit {}
impl ::windows_core::CanTryInto<ILearningModelFeatureValue> for TensorInt8Bit {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IMemoryBuffer> for TensorInt8Bit {}
impl ::windows_core::CanTryInto<ITensor> for TensorInt8Bit {}
unsafe impl ::core::marker::Send for TensorInt8Bit {}
unsafe impl ::core::marker::Sync for TensorInt8Bit {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TensorString(::windows_core::IUnknown);
impl TensorString {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> ::windows_core::Result<LearningModelFeatureKind> {
        let this = &::windows_core::ComInterface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateReference)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TensorKind(&self) -> ::windows_core::Result<TensorKind> {
        let this = &::windows_core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TensorKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows_core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Shape)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAsVectorView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create() -> ::windows_core::Result<TensorString> {
        Self::ITensorStringStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> ::windows_core::Result<TensorString>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorStringStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create2)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[::windows_core::HSTRING]) -> ::windows_core::Result<TensorString>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorStringStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromArray)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.len().try_into().unwrap(), ::core::mem::transmute(data.as_ptr()), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> ::windows_core::Result<TensorString>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::ITensorStringStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromIterable)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[::windows_core::HSTRING]) -> ::windows_core::Result<TensorString> {
        Self::ITensorStringStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), ::core::mem::transmute(data.as_ptr()), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorStringStatics<R, F: FnOnce(&ITensorStringStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TensorString, ITensorStringStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorStringStatics2<R, F: FnOnce(&ITensorStringStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TensorString, ITensorStringStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for TensorString {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorString;{582335c8-bdb1-4610-bc75-35e9cbf009b7})");
}
unsafe impl ::windows_core::Interface for TensorString {
    type Vtable = ITensorString_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TensorString {
    const IID: ::windows_core::GUID = <ITensorString as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TensorString {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorString";
}
::windows_core::imp::interface_hierarchy!(TensorString, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for TensorString {}
impl ::windows_core::CanTryInto<ILearningModelFeatureValue> for TensorString {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IMemoryBuffer> for TensorString {}
impl ::windows_core::CanTryInto<ITensor> for TensorString {}
unsafe impl ::core::marker::Send for TensorString {}
unsafe impl ::core::marker::Sync for TensorString {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TensorUInt16Bit(::windows_core::IUnknown);
impl TensorUInt16Bit {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> ::windows_core::Result<LearningModelFeatureKind> {
        let this = &::windows_core::ComInterface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateReference)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TensorKind(&self) -> ::windows_core::Result<TensorKind> {
        let this = &::windows_core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TensorKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows_core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Shape)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<u16>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAsVectorView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create() -> ::windows_core::Result<TensorUInt16Bit> {
        Self::ITensorUInt16BitStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> ::windows_core::Result<TensorUInt16Bit>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorUInt16BitStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create2)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[u16]) -> ::windows_core::Result<TensorUInt16Bit>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorUInt16BitStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromArray)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> ::windows_core::Result<TensorUInt16Bit>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<u16>>,
    {
        Self::ITensorUInt16BitStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromIterable)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[u16]) -> ::windows_core::Result<TensorUInt16Bit> {
        Self::ITensorUInt16BitStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(shape: &[i64], buffer: P0) -> ::windows_core::Result<TensorUInt16Bit>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorUInt16BitStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromBuffer)(::windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorUInt16BitStatics<R, F: FnOnce(&ITensorUInt16BitStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TensorUInt16Bit, ITensorUInt16BitStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorUInt16BitStatics2<R, F: FnOnce(&ITensorUInt16BitStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TensorUInt16Bit, ITensorUInt16BitStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for TensorUInt16Bit {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorUInt16Bit;{68140f4b-23c0-42f3-81f6-a891c011bc3f})");
}
unsafe impl ::windows_core::Interface for TensorUInt16Bit {
    type Vtable = ITensorUInt16Bit_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TensorUInt16Bit {
    const IID: ::windows_core::GUID = <ITensorUInt16Bit as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TensorUInt16Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorUInt16Bit";
}
::windows_core::imp::interface_hierarchy!(TensorUInt16Bit, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for TensorUInt16Bit {}
impl ::windows_core::CanTryInto<ILearningModelFeatureValue> for TensorUInt16Bit {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IMemoryBuffer> for TensorUInt16Bit {}
impl ::windows_core::CanTryInto<ITensor> for TensorUInt16Bit {}
unsafe impl ::core::marker::Send for TensorUInt16Bit {}
unsafe impl ::core::marker::Sync for TensorUInt16Bit {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TensorUInt32Bit(::windows_core::IUnknown);
impl TensorUInt32Bit {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> ::windows_core::Result<LearningModelFeatureKind> {
        let this = &::windows_core::ComInterface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateReference)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TensorKind(&self) -> ::windows_core::Result<TensorKind> {
        let this = &::windows_core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TensorKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows_core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Shape)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAsVectorView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create() -> ::windows_core::Result<TensorUInt32Bit> {
        Self::ITensorUInt32BitStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> ::windows_core::Result<TensorUInt32Bit>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorUInt32BitStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create2)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[u32]) -> ::windows_core::Result<TensorUInt32Bit>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorUInt32BitStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromArray)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> ::windows_core::Result<TensorUInt32Bit>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<u32>>,
    {
        Self::ITensorUInt32BitStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromIterable)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[u32]) -> ::windows_core::Result<TensorUInt32Bit> {
        Self::ITensorUInt32BitStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(shape: &[i64], buffer: P0) -> ::windows_core::Result<TensorUInt32Bit>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorUInt32BitStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromBuffer)(::windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorUInt32BitStatics<R, F: FnOnce(&ITensorUInt32BitStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TensorUInt32Bit, ITensorUInt32BitStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorUInt32BitStatics2<R, F: FnOnce(&ITensorUInt32BitStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TensorUInt32Bit, ITensorUInt32BitStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for TensorUInt32Bit {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorUInt32Bit;{d8c9c2ff-7511-45a3-bfac-c38f370d2237})");
}
unsafe impl ::windows_core::Interface for TensorUInt32Bit {
    type Vtable = ITensorUInt32Bit_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TensorUInt32Bit {
    const IID: ::windows_core::GUID = <ITensorUInt32Bit as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TensorUInt32Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorUInt32Bit";
}
::windows_core::imp::interface_hierarchy!(TensorUInt32Bit, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for TensorUInt32Bit {}
impl ::windows_core::CanTryInto<ILearningModelFeatureValue> for TensorUInt32Bit {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IMemoryBuffer> for TensorUInt32Bit {}
impl ::windows_core::CanTryInto<ITensor> for TensorUInt32Bit {}
unsafe impl ::core::marker::Send for TensorUInt32Bit {}
unsafe impl ::core::marker::Sync for TensorUInt32Bit {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TensorUInt64Bit(::windows_core::IUnknown);
impl TensorUInt64Bit {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> ::windows_core::Result<LearningModelFeatureKind> {
        let this = &::windows_core::ComInterface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateReference)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TensorKind(&self) -> ::windows_core::Result<TensorKind> {
        let this = &::windows_core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TensorKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows_core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Shape)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAsVectorView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create() -> ::windows_core::Result<TensorUInt64Bit> {
        Self::ITensorUInt64BitStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> ::windows_core::Result<TensorUInt64Bit>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorUInt64BitStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create2)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[u64]) -> ::windows_core::Result<TensorUInt64Bit>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorUInt64BitStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromArray)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> ::windows_core::Result<TensorUInt64Bit>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<u64>>,
    {
        Self::ITensorUInt64BitStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromIterable)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[u64]) -> ::windows_core::Result<TensorUInt64Bit> {
        Self::ITensorUInt64BitStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(shape: &[i64], buffer: P0) -> ::windows_core::Result<TensorUInt64Bit>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorUInt64BitStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromBuffer)(::windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorUInt64BitStatics<R, F: FnOnce(&ITensorUInt64BitStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TensorUInt64Bit, ITensorUInt64BitStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorUInt64BitStatics2<R, F: FnOnce(&ITensorUInt64BitStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TensorUInt64Bit, ITensorUInt64BitStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for TensorUInt64Bit {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorUInt64Bit;{2e70ffad-04bf-4825-839a-82baef8c7886})");
}
unsafe impl ::windows_core::Interface for TensorUInt64Bit {
    type Vtable = ITensorUInt64Bit_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TensorUInt64Bit {
    const IID: ::windows_core::GUID = <ITensorUInt64Bit as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TensorUInt64Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorUInt64Bit";
}
::windows_core::imp::interface_hierarchy!(TensorUInt64Bit, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for TensorUInt64Bit {}
impl ::windows_core::CanTryInto<ILearningModelFeatureValue> for TensorUInt64Bit {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IMemoryBuffer> for TensorUInt64Bit {}
impl ::windows_core::CanTryInto<ITensor> for TensorUInt64Bit {}
unsafe impl ::core::marker::Send for TensorUInt64Bit {}
unsafe impl ::core::marker::Sync for TensorUInt64Bit {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TensorUInt8Bit(::windows_core::IUnknown);
impl TensorUInt8Bit {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> ::windows_core::Result<LearningModelFeatureKind> {
        let this = &::windows_core::ComInterface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateReference)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TensorKind(&self) -> ::windows_core::Result<TensorKind> {
        let this = &::windows_core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TensorKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &::windows_core::ComInterface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Shape)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAsVectorView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create() -> ::windows_core::Result<TensorUInt8Bit> {
        Self::ITensorUInt8BitStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> ::windows_core::Result<TensorUInt8Bit>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorUInt8BitStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create2)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[u8]) -> ::windows_core::Result<TensorUInt8Bit>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorUInt8BitStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromArray)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> ::windows_core::Result<TensorUInt8Bit>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i64>>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<u8>>,
    {
        Self::ITensorUInt8BitStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromIterable)(::windows_core::Interface::as_raw(this), shape.try_into_param()?.abi(), data.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[u8]) -> ::windows_core::Result<TensorUInt8Bit> {
        Self::ITensorUInt8BitStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(::windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(shape: &[i64], buffer: P0) -> ::windows_core::Result<TensorUInt8Bit>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorUInt8BitStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromBuffer)(::windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITensorUInt8BitStatics<R, F: FnOnce(&ITensorUInt8BitStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TensorUInt8Bit, ITensorUInt8BitStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorUInt8BitStatics2<R, F: FnOnce(&ITensorUInt8BitStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TensorUInt8Bit, ITensorUInt8BitStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for TensorUInt8Bit {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.TensorUInt8Bit;{58e1ae27-622b-48e3-be22-d867aed1daac})");
}
unsafe impl ::windows_core::Interface for TensorUInt8Bit {
    type Vtable = ITensorUInt8Bit_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TensorUInt8Bit {
    const IID: ::windows_core::GUID = <ITensorUInt8Bit as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TensorUInt8Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorUInt8Bit";
}
::windows_core::imp::interface_hierarchy!(TensorUInt8Bit, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for TensorUInt8Bit {}
impl ::windows_core::CanTryInto<ILearningModelFeatureValue> for TensorUInt8Bit {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IMemoryBuffer> for TensorUInt8Bit {}
impl ::windows_core::CanTryInto<ITensor> for TensorUInt8Bit {}
unsafe impl ::core::marker::Send for TensorUInt8Bit {}
unsafe impl ::core::marker::Sync for TensorUInt8Bit {}
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
impl ::windows_core::TypeKind for LearningModelDeviceKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for LearningModelDeviceKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LearningModelDeviceKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LearningModelDeviceKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.AI.MachineLearning.LearningModelDeviceKind;i4)");
}
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
impl ::windows_core::TypeKind for LearningModelFeatureKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for LearningModelFeatureKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LearningModelFeatureKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LearningModelFeatureKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.AI.MachineLearning.LearningModelFeatureKind;i4)");
}
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
impl ::windows_core::TypeKind for LearningModelPixelRange {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for LearningModelPixelRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LearningModelPixelRange").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LearningModelPixelRange {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.AI.MachineLearning.LearningModelPixelRange;i4)");
}
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
impl ::windows_core::TypeKind for TensorKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TensorKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TensorKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TensorKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.AI.MachineLearning.TensorKind;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
