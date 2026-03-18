windows_core::imp::define_interface!(IImageFeatureDescriptor, IImageFeatureDescriptor_Vtbl, 0x365585a5_171a_4a2a_985f_265159d3895a);
impl windows_core::RuntimeType for IImageFeatureDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageFeatureDescriptor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Imaging")]
    pub BitmapPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::Imaging::BitmapPixelFormat) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    BitmapPixelFormat: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub BitmapAlphaMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::Imaging::BitmapAlphaMode) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    BitmapAlphaMode: usize,
    pub Width: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IImageFeatureDescriptor2, IImageFeatureDescriptor2_Vtbl, 0xe7ff7690_b803_57de_b55b_cb7768691992);
impl windows_core::RuntimeType for IImageFeatureDescriptor2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageFeatureDescriptor2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PixelRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut LearningModelPixelRange) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IImageFeatureValue, IImageFeatureValue_Vtbl, 0xd980c30a_9179_5e26_9fd2_a0cad76274e6);
impl windows_core::RuntimeType for IImageFeatureValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageFeatureValue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media")]
    pub VideoFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media"))]
    VideoFrame: usize,
}
windows_core::imp::define_interface!(IImageFeatureValueStatics, IImageFeatureValueStatics_Vtbl, 0x1bc317fd_23cb_4610_b085_c8e1c87ebaa0);
impl windows_core::RuntimeType for IImageFeatureValueStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageFeatureValueStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media")]
    pub CreateFromVideoFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media"))]
    CreateFromVideoFrame: usize,
}
windows_core::imp::define_interface!(ILearningModel, ILearningModel_Vtbl, 0x5ee50f87_f1ca_5e6d_af5c_78b03f490235);
impl windows_core::RuntimeType for ILearningModel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModel_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Author: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Domain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub Metadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InputFeatures: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OutputFeatures: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILearningModelBinding, ILearningModelBinding_Vtbl, 0x89054d4e_e0f4_5c2e_9ec5_ff2d8a6f2ddd);
impl windows_core::RuntimeType for ILearningModelBinding {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelBinding_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Bind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub BindWithProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BindWithProperties: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILearningModelBindingFactory, ILearningModelBindingFactory_Vtbl, 0xc95f7a7a_e788_475e_8917_23aa381faf0b);
impl windows_core::RuntimeType for ILearningModelBindingFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelBindingFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILearningModelDevice, ILearningModelDevice_Vtbl, 0x2f78efe1_cb75_5317_90c0_c30e9f6497ad);
impl windows_core::RuntimeType for ILearningModelDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelDevice_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics")]
    pub AdapterId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::DisplayAdapterId) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    AdapterId: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Direct3D11Device: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Direct3D11Device: usize,
}
windows_core::imp::define_interface!(ILearningModelDeviceFactory, ILearningModelDeviceFactory_Vtbl, 0xc2313779_6ad2_5d13_9e28_27e2f7aa467d);
impl windows_core::RuntimeType for ILearningModelDeviceFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelDeviceFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, LearningModelDeviceKind, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILearningModelDeviceStatics, ILearningModelDeviceStatics_Vtbl, 0x6d6b1901_d584_586d_b825_c7cb21cbd3fd);
impl windows_core::RuntimeType for ILearningModelDeviceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelDeviceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CreateFromDirect3D11Device: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CreateFromDirect3D11Device: usize,
}
windows_core::imp::define_interface!(ILearningModelEvaluationResult, ILearningModelEvaluationResult_Vtbl, 0xc126cfb7_b7d2_5aa8_94c2_044ac4848d22);
impl windows_core::RuntimeType for ILearningModelEvaluationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelEvaluationResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CorrelationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ErrorStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Succeeded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Outputs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILearningModelFeatureDescriptor, ILearningModelFeatureDescriptor_Vtbl, 0x91edee9c_1d44_5d65_9d67_3c193ea910a4);
impl windows_core::RuntimeType for ILearningModelFeatureDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ILearningModelFeatureDescriptor, windows_core::IUnknown, windows_core::IInspectable);
impl ILearningModelFeatureDescriptor {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsRequired(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRequired)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for ILearningModelFeatureDescriptor {
    const NAME: &'static str = "Windows.AI.MachineLearning.ILearningModelFeatureDescriptor";
}
pub trait ILearningModelFeatureDescriptor_Impl: windows_core::IUnknownImpl {
    fn Name(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Description(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind>;
    fn IsRequired(&self) -> windows_core::Result<bool>;
}
impl ILearningModelFeatureDescriptor_Vtbl {
    pub const fn new<Identity: ILearningModelFeatureDescriptor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: ILearningModelFeatureDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILearningModelFeatureDescriptor_Impl::Name(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Description<Identity: ILearningModelFeatureDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILearningModelFeatureDescriptor_Impl::Description(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Kind<Identity: ILearningModelFeatureDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut LearningModelFeatureKind) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILearningModelFeatureDescriptor_Impl::Kind(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsRequired<Identity: ILearningModelFeatureDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILearningModelFeatureDescriptor_Impl::IsRequired(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ILearningModelFeatureDescriptor, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            Kind: Kind::<Identity, OFFSET>,
            IsRequired: IsRequired::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILearningModelFeatureDescriptor as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelFeatureDescriptor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut LearningModelFeatureKind) -> windows_core::HRESULT,
    pub IsRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILearningModelFeatureValue, ILearningModelFeatureValue_Vtbl, 0x2815fabb_b7c4_5c1a_9c8f_bcf1807efbd1);
impl windows_core::RuntimeType for ILearningModelFeatureValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ILearningModelFeatureValue, windows_core::IUnknown, windows_core::IInspectable);
impl ILearningModelFeatureValue {
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for ILearningModelFeatureValue {
    const NAME: &'static str = "Windows.AI.MachineLearning.ILearningModelFeatureValue";
}
pub trait ILearningModelFeatureValue_Impl: windows_core::IUnknownImpl {
    fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind>;
}
impl ILearningModelFeatureValue_Vtbl {
    pub const fn new<Identity: ILearningModelFeatureValue_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Kind<Identity: ILearningModelFeatureValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut LearningModelFeatureKind) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILearningModelFeatureValue_Impl::Kind(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, ILearningModelFeatureValue, OFFSET>(), Kind: Kind::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILearningModelFeatureValue as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelFeatureValue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut LearningModelFeatureKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILearningModelOperatorProvider, ILearningModelOperatorProvider_Vtbl, 0xbe154420_8028_5ee6_ae99_fcf676fef497);
impl windows_core::RuntimeType for ILearningModelOperatorProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ILearningModelOperatorProvider, windows_core::IUnknown, windows_core::IInspectable);
impl windows_core::RuntimeName for ILearningModelOperatorProvider {
    const NAME: &'static str = "Windows.AI.MachineLearning.ILearningModelOperatorProvider";
}
pub trait ILearningModelOperatorProvider_Impl: windows_core::IUnknownImpl {}
impl ILearningModelOperatorProvider_Vtbl {
    pub const fn new<Identity: ILearningModelOperatorProvider_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, ILearningModelOperatorProvider, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILearningModelOperatorProvider as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelOperatorProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ILearningModelSession, ILearningModelSession_Vtbl, 0x8e58f8f6_b787_4c11_90f0_7129aeca74a9);
impl windows_core::RuntimeType for ILearningModelSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSession_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Model: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Device: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub EvaluationProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EvaluationProperties: usize,
    pub EvaluateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EvaluateFeaturesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Evaluate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EvaluateFeatures: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILearningModelSessionFactory, ILearningModelSessionFactory_Vtbl, 0xa7dbc6fc_6935_5515_993b_5d7984eb0ded);
impl windows_core::RuntimeType for ILearningModelSessionFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSessionFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromModel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromModelOnDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILearningModelSessionFactory2, ILearningModelSessionFactory2_Vtbl, 0xa3f140f6_f90f_55d2_a6ed_dd10c12e68e2);
impl windows_core::RuntimeType for ILearningModelSessionFactory2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSessionFactory2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromModelOnDeviceWithSessionOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILearningModelSessionOptions, ILearningModelSessionOptions_Vtbl, 0x3c53bd28_253f_54f8_86d6_432c02c57342);
impl windows_core::RuntimeType for ILearningModelSessionOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSessionOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BatchSizeOverride: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetBatchSizeOverride: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILearningModelSessionOptions2, ILearningModelSessionOptions2_Vtbl, 0x2f837453_059b_53fb_b288_420bf41db29a);
impl windows_core::RuntimeType for ILearningModelSessionOptions2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSessionOptions2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CloseModelOnSessionCreation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCloseModelOnSessionCreation: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILearningModelSessionOptions3, ILearningModelSessionOptions3_Vtbl, 0x9a545393_7c76_5ee5_949b_8fb1885d84de);
impl windows_core::RuntimeType for ILearningModelSessionOptions3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSessionOptions3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OverrideNamedDimension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILearningModelStatics, ILearningModelStatics_Vtbl, 0xe3b977e8_6952_4e47_8ef4_1f7f07897c6d);
impl windows_core::RuntimeType for ILearningModelStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub LoadFromStorageFileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LoadFromStorageFileAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub LoadFromStreamAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LoadFromStreamAsync: usize,
    pub LoadFromFilePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub LoadFromStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LoadFromStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub LoadFromStorageFileWithOperatorProviderAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LoadFromStorageFileWithOperatorProviderAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub LoadFromStreamWithOperatorProviderAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LoadFromStreamWithOperatorProviderAsync: usize,
    pub LoadFromFilePathWithOperatorProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub LoadFromStreamWithOperatorProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LoadFromStreamWithOperatorProvider: usize,
}
windows_core::imp::define_interface!(IMapFeatureDescriptor, IMapFeatureDescriptor_Vtbl, 0xe5178975_54f9_5b10_a186_bacb2b6e701a);
impl windows_core::RuntimeType for IMapFeatureDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapFeatureDescriptor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub KeyKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TensorKind) -> windows_core::HRESULT,
    pub ValueDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISequenceFeatureDescriptor, ISequenceFeatureDescriptor_Vtbl, 0xecf5b3b4_7898_506b_bf91_8fe657ad534c);
impl windows_core::RuntimeType for ISequenceFeatureDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISequenceFeatureDescriptor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ElementDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITensor, ITensor_Vtbl, 0xbb059e7f_c08c_5401_ae2b_415e27fcb15f);
impl windows_core::RuntimeType for ITensor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ITensor, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ITensor, ILearningModelFeatureValue);
impl ITensor {
    pub fn TensorKind(&self) -> windows_core::Result<TensorKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TensorKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Shape(&self) -> windows_core::Result<windows_collections::IVectorView<i64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shape)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for ITensor {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensor";
}
pub trait ITensor_Impl: ILearningModelFeatureValue_Impl {
    fn TensorKind(&self) -> windows_core::Result<TensorKind>;
    fn Shape(&self) -> windows_core::Result<windows_collections::IVectorView<i64>>;
}
impl ITensor_Vtbl {
    pub const fn new<Identity: ITensor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TensorKind<Identity: ITensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut TensorKind) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITensor_Impl::TensorKind(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Shape<Identity: ITensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITensor_Impl::Shape(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ITensor, OFFSET>(),
            TensorKind: TensorKind::<Identity, OFFSET>,
            Shape: Shape::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITensor as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TensorKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TensorKind) -> windows_core::HRESULT,
    pub Shape: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITensorBoolean, ITensorBoolean_Vtbl, 0x3330e1a8_2fdb_5d56_8f2e_fc486d6d0b69);
impl windows_core::RuntimeType for ITensorBoolean {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorBoolean_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetAsVectorView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITensorBooleanStatics, ITensorBooleanStatics_Vtbl, 0x9d28d3ee_fe13_53ad_9b3f_3b342cc7e330);
impl windows_core::RuntimeType for ITensorBooleanStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorBooleanStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Create2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromIterable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITensorBooleanStatics2, ITensorBooleanStatics2_Vtbl, 0x9f0f2679_1c15_5a48_9275_3b1ccb9a92f8);
impl windows_core::RuntimeType for ITensorBooleanStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorBooleanStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, u32, *const bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
windows_core::imp::define_interface!(ITensorDouble, ITensorDouble_Vtbl, 0x6fd0e921_b341_5439_8e4a_c63be0e0fffc);
impl windows_core::RuntimeType for ITensorDouble {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorDouble_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetAsVectorView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITensorDoubleStatics, ITensorDoubleStatics_Vtbl, 0x22541397_f2ce_5998_bd1f_a0db9216dd44);
impl windows_core::RuntimeType for ITensorDoubleStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorDoubleStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Create2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromIterable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITensorDoubleStatics2, ITensorDoubleStatics2_Vtbl, 0x7856592d_5b65_5f79_8376_705505fde2dc);
impl windows_core::RuntimeType for ITensorDoubleStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorDoubleStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, u32, *const f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
windows_core::imp::define_interface!(ITensorFeatureDescriptor, ITensorFeatureDescriptor_Vtbl, 0xcb60fa8d_d27a_5933_a327_f7efcb980ca2);
impl windows_core::RuntimeType for ITensorFeatureDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorFeatureDescriptor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TensorKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TensorKind) -> windows_core::HRESULT,
    pub Shape: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITensorFloat, ITensorFloat_Vtbl, 0x02bb8c7a_edc3_51d3_a466_e6a5bf5c3a0c);
impl windows_core::RuntimeType for ITensorFloat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorFloat_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetAsVectorView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITensorFloat16Bit, ITensorFloat16Bit_Vtbl, 0x0ab994fc_5b89_4c3c_b5e4_5282a5316c0a);
impl windows_core::RuntimeType for ITensorFloat16Bit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorFloat16Bit_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetAsVectorView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITensorFloat16BitStatics, ITensorFloat16BitStatics_Vtbl, 0xa52db6f5_318a_44d4_820b_0cdc7054a84a);
impl windows_core::RuntimeType for ITensorFloat16BitStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorFloat16BitStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Create2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const f32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromIterable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITensorFloat16BitStatics2, ITensorFloat16BitStatics2_Vtbl, 0x62908a55_9747_5747_b6c7_9d3cca09935e);
impl windows_core::RuntimeType for ITensorFloat16BitStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorFloat16BitStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, u32, *const f32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
windows_core::imp::define_interface!(ITensorFloatStatics, ITensorFloatStatics_Vtbl, 0x6b0cb039_ffca_5044_9a67_d2fc0df455d2);
impl windows_core::RuntimeType for ITensorFloatStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorFloatStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Create2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const f32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromIterable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITensorFloatStatics2, ITensorFloatStatics2_Vtbl, 0x5c9ff7c8_07d3_5849_86fc_b1771501dc93);
impl windows_core::RuntimeType for ITensorFloatStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorFloatStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, u32, *const f32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
windows_core::imp::define_interface!(ITensorInt16Bit, ITensorInt16Bit_Vtbl, 0xfca2362c_ca54_540f_86c9_5391239a28a2);
impl windows_core::RuntimeType for ITensorInt16Bit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt16Bit_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetAsVectorView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITensorInt16BitStatics, ITensorInt16BitStatics_Vtbl, 0x98646293_266e_4b1a_821f_e60d70898b91);
impl windows_core::RuntimeType for ITensorInt16BitStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt16BitStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Create2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const i16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromIterable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITensorInt16BitStatics2, ITensorInt16BitStatics2_Vtbl, 0xd050e968_f2d5_52c8_9514_3a3b67ab2b6f);
impl windows_core::RuntimeType for ITensorInt16BitStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt16BitStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, u32, *const i16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
windows_core::imp::define_interface!(ITensorInt32Bit, ITensorInt32Bit_Vtbl, 0x2c0c28d3_207c_4486_a7d2_884522c5e589);
impl windows_core::RuntimeType for ITensorInt32Bit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt32Bit_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetAsVectorView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITensorInt32BitStatics, ITensorInt32BitStatics_Vtbl, 0xfb1e99f0_6c09_5125_849b_4383b48f0812);
impl windows_core::RuntimeType for ITensorInt32BitStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt32BitStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Create2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromIterable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITensorInt32BitStatics2, ITensorInt32BitStatics2_Vtbl, 0x7c4b079a_e956_5ce0_a3bd_157d9d79b5ec);
impl windows_core::RuntimeType for ITensorInt32BitStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt32BitStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, u32, *const i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
windows_core::imp::define_interface!(ITensorInt64Bit, ITensorInt64Bit_Vtbl, 0x90635af7_fd5b_555e_86ae_51b2d3e3e707);
impl windows_core::RuntimeType for ITensorInt64Bit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt64Bit_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetAsVectorView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITensorInt64BitStatics, ITensorInt64BitStatics_Vtbl, 0xb22d1537_be74_50be_bbab_954b708e7a8e);
impl windows_core::RuntimeType for ITensorInt64BitStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt64BitStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Create2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const i64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromIterable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITensorInt64BitStatics2, ITensorInt64BitStatics2_Vtbl, 0x7ccc278f_b7e8_5162_bd64_e5bb8d29b172);
impl windows_core::RuntimeType for ITensorInt64BitStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt64BitStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, u32, *const i64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
windows_core::imp::define_interface!(ITensorInt8Bit, ITensorInt8Bit_Vtbl, 0xcddd97c5_ffd8_4fef_aefb_30e1a485b2ee);
impl windows_core::RuntimeType for ITensorInt8Bit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt8Bit_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetAsVectorView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITensorInt8BitStatics, ITensorInt8BitStatics_Vtbl, 0xb1a12284_095c_4c76_a661_ac4cee1f3e8b);
impl windows_core::RuntimeType for ITensorInt8BitStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt8BitStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Create2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromIterable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITensorInt8BitStatics2, ITensorInt8BitStatics2_Vtbl, 0xc0d59637_c468_56fb_9535_c052bdb93dc0);
impl windows_core::RuntimeType for ITensorInt8BitStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorInt8BitStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
windows_core::imp::define_interface!(ITensorString, ITensorString_Vtbl, 0x582335c8_bdb1_4610_bc75_35e9cbf009b7);
impl windows_core::RuntimeType for ITensorString {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorString_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetAsVectorView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITensorStringStatics, ITensorStringStatics_Vtbl, 0x83623324_cf26_4f17_a2d4_20ef8d097d53);
impl windows_core::RuntimeType for ITensorStringStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorStringStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Create2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const windows_core::HSTRING, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromIterable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITensorStringStatics2, ITensorStringStatics2_Vtbl, 0xdf255218_77a6_5173_b655_443ac5bbe383);
impl windows_core::RuntimeType for ITensorStringStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorStringStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, u32, *const windows_core::HSTRING, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITensorUInt16Bit, ITensorUInt16Bit_Vtbl, 0x64399566_c0a8_53c5_bc86_640cbe9e4288);
impl windows_core::RuntimeType for ITensorUInt16Bit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt16Bit_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetAsVectorView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITensorUInt16BitStatics, ITensorUInt16BitStatics_Vtbl, 0x2dbd1562_770e_5935_9ef8_929b8573846c);
impl windows_core::RuntimeType for ITensorUInt16BitStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt16BitStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Create2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromIterable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITensorUInt16BitStatics2, ITensorUInt16BitStatics2_Vtbl, 0x72ce938b_11f3_50d6_b3fe_425519c0dc4c);
impl windows_core::RuntimeType for ITensorUInt16BitStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt16BitStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, u32, *const u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
windows_core::imp::define_interface!(ITensorUInt32Bit, ITensorUInt32Bit_Vtbl, 0xd8c9c2ff_7511_45a3_bfac_c38f370d2237);
impl windows_core::RuntimeType for ITensorUInt32Bit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt32Bit_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetAsVectorView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITensorUInt32BitStatics, ITensorUInt32BitStatics_Vtbl, 0xfe24f2c6_0b9d_53d7_85be_f321c4be6e19);
impl windows_core::RuntimeType for ITensorUInt32BitStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt32BitStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Create2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromIterable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITensorUInt32BitStatics2, ITensorUInt32BitStatics2_Vtbl, 0xef1a1f1c_314e_569d_b496_5c8447d20cd2);
impl windows_core::RuntimeType for ITensorUInt32BitStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt32BitStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, u32, *const u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
windows_core::imp::define_interface!(ITensorUInt64Bit, ITensorUInt64Bit_Vtbl, 0x6d97ad9c_1d86_5b16_b022_df7e13dc9f14);
impl windows_core::RuntimeType for ITensorUInt64Bit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt64Bit_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetAsVectorView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITensorUInt64BitStatics, ITensorUInt64BitStatics_Vtbl, 0xba5a41a5_9f3f_5c3a_8810_5044fb77320d);
impl windows_core::RuntimeType for ITensorUInt64BitStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt64BitStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Create2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromIterable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITensorUInt64BitStatics2, ITensorUInt64BitStatics2_Vtbl, 0x6ba8b843_d0cd_52a8_9846_43db7211fc52);
impl windows_core::RuntimeType for ITensorUInt64BitStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt64BitStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, u32, *const u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
windows_core::imp::define_interface!(ITensorUInt8Bit, ITensorUInt8Bit_Vtbl, 0xf30bebbf_05a4_5312_b6f2_2af919e6b92d);
impl windows_core::RuntimeType for ITensorUInt8Bit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt8Bit_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetAsVectorView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITensorUInt8BitStatics, ITensorUInt8BitStatics_Vtbl, 0x7d6fd83b_f01c_573b_8826_5739fdd41221);
impl windows_core::RuntimeType for ITensorUInt8BitStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt8BitStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Create2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromIterable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITensorUInt8BitStatics2, ITensorUInt8BitStatics2_Vtbl, 0x229637c0_d787_510d_b163_43e32d0acc70);
impl windows_core::RuntimeType for ITensorUInt8BitStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorUInt8BitStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageFeatureDescriptor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ImageFeatureDescriptor, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ImageFeatureDescriptor, ILearningModelFeatureDescriptor);
impl ImageFeatureDescriptor {
    #[cfg(feature = "Graphics_Imaging")]
    pub fn BitmapPixelFormat(&self) -> windows_core::Result<super::super::Graphics::Imaging::BitmapPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BitmapPixelFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn BitmapAlphaMode(&self) -> windows_core::Result<super::super::Graphics::Imaging::BitmapAlphaMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BitmapAlphaMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Width(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Height(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PixelRange(&self) -> windows_core::Result<LearningModelPixelRange> {
        let this = &windows_core::Interface::cast::<IImageFeatureDescriptor2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PixelRange)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsRequired(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRequired)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ImageFeatureDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IImageFeatureDescriptor>();
}
unsafe impl windows_core::Interface for ImageFeatureDescriptor {
    type Vtable = <IImageFeatureDescriptor as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IImageFeatureDescriptor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ImageFeatureDescriptor {
    const NAME: &'static str = "Windows.AI.MachineLearning.ImageFeatureDescriptor";
}
unsafe impl Send for ImageFeatureDescriptor {}
unsafe impl Sync for ImageFeatureDescriptor {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageFeatureValue(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ImageFeatureValue, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ImageFeatureValue, ILearningModelFeatureValue);
impl ImageFeatureValue {
    #[cfg(feature = "Media")]
    pub fn VideoFrame(&self) -> windows_core::Result<super::super::Media::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoFrame)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media")]
    pub fn CreateFromVideoFrame<P0>(image: P0) -> windows_core::Result<ImageFeatureValue>
    where
        P0: windows_core::Param<super::super::Media::VideoFrame>,
    {
        Self::IImageFeatureValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromVideoFrame)(windows_core::Interface::as_raw(this), image.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    fn IImageFeatureValueStatics<R, F: FnOnce(&IImageFeatureValueStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ImageFeatureValue, IImageFeatureValueStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ImageFeatureValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IImageFeatureValue>();
}
unsafe impl windows_core::Interface for ImageFeatureValue {
    type Vtable = <IImageFeatureValue as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IImageFeatureValue as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ImageFeatureValue {
    const NAME: &'static str = "Windows.AI.MachineLearning.ImageFeatureValue";
}
unsafe impl Send for ImageFeatureValue {}
unsafe impl Sync for ImageFeatureValue {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LearningModel(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LearningModel, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(LearningModel, super::super::Foundation::IClosable);
impl LearningModel {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Author(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Author)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Domain(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Domain)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Version(&self) -> windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Version)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Metadata(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Metadata)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InputFeatures(&self) -> windows_core::Result<windows_collections::IVectorView<ILearningModelFeatureDescriptor>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InputFeatures)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OutputFeatures(&self) -> windows_core::Result<windows_collections::IVectorView<ILearningModelFeatureDescriptor>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutputFeatures)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadFromStorageFileAsync<P0>(modelfile: P0) -> windows_core::Result<windows_future::IAsyncOperation<LearningModel>>
    where
        P0: windows_core::Param<super::super::Storage::IStorageFile>,
    {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadFromStorageFileAsync)(windows_core::Interface::as_raw(this), modelfile.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadFromStreamAsync<P0>(modelstream: P0) -> windows_core::Result<windows_future::IAsyncOperation<LearningModel>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadFromStreamAsync)(windows_core::Interface::as_raw(this), modelstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn LoadFromFilePath(filepath: &windows_core::HSTRING) -> windows_core::Result<LearningModel> {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadFromFilePath)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(filepath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadFromStream<P0>(modelstream: P0) -> windows_core::Result<LearningModel>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadFromStream)(windows_core::Interface::as_raw(this), modelstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadFromStorageFileWithOperatorProviderAsync<P0, P1>(modelfile: P0, operatorprovider: P1) -> windows_core::Result<windows_future::IAsyncOperation<LearningModel>>
    where
        P0: windows_core::Param<super::super::Storage::IStorageFile>,
        P1: windows_core::Param<ILearningModelOperatorProvider>,
    {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadFromStorageFileWithOperatorProviderAsync)(windows_core::Interface::as_raw(this), modelfile.param().abi(), operatorprovider.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadFromStreamWithOperatorProviderAsync<P0, P1>(modelstream: P0, operatorprovider: P1) -> windows_core::Result<windows_future::IAsyncOperation<LearningModel>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStreamReference>,
        P1: windows_core::Param<ILearningModelOperatorProvider>,
    {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadFromStreamWithOperatorProviderAsync)(windows_core::Interface::as_raw(this), modelstream.param().abi(), operatorprovider.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn LoadFromFilePathWithOperatorProvider<P1>(filepath: &windows_core::HSTRING, operatorprovider: P1) -> windows_core::Result<LearningModel>
    where
        P1: windows_core::Param<ILearningModelOperatorProvider>,
    {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadFromFilePathWithOperatorProvider)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(filepath), operatorprovider.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadFromStreamWithOperatorProvider<P0, P1>(modelstream: P0, operatorprovider: P1) -> windows_core::Result<LearningModel>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStreamReference>,
        P1: windows_core::Param<ILearningModelOperatorProvider>,
    {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadFromStreamWithOperatorProvider)(windows_core::Interface::as_raw(this), modelstream.param().abi(), operatorprovider.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ILearningModelStatics<R, F: FnOnce(&ILearningModelStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LearningModel, ILearningModelStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for LearningModel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILearningModel>();
}
unsafe impl windows_core::Interface for LearningModel {
    type Vtable = <ILearningModel as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ILearningModel as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LearningModel {
    const NAME: &'static str = "Windows.AI.MachineLearning.LearningModel";
}
unsafe impl Send for LearningModel {}
unsafe impl Sync for LearningModel {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LearningModelBinding(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LearningModelBinding, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy ! ( LearningModelBinding , windows_collections:: IIterable < windows_collections:: IKeyValuePair < windows_core::HSTRING , windows_core::IInspectable > > , windows_collections:: IMapView < windows_core::HSTRING , windows_core::IInspectable > );
impl LearningModelBinding {
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>>> {
        let this = &windows_core::Interface::cast::<windows_collections::IIterable<windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Bind<P1>(&self, name: &windows_core::HSTRING, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Bind)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn BindWithProperties<P1, P2>(&self, name: &windows_core::HSTRING, value: P1, props: P2) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IInspectable>,
        P2: windows_core::Param<super::super::Foundation::Collections::IPropertySet>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).BindWithProperties)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.param().abi(), props.param().abi()).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CreateFromSession<P0>(session: P0) -> windows_core::Result<LearningModelBinding>
    where
        P0: windows_core::Param<LearningModelSession>,
    {
        Self::ILearningModelBindingFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromSession)(windows_core::Interface::as_raw(this), session.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Lookup(&self, key: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lookup)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasKey(&self, key: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasKey)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).map(|| result__)
        }
    }
    pub fn Split(&self, first: &mut Option<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>>, second: &mut Option<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Split)(windows_core::Interface::as_raw(this), first as *mut _ as _, second as *mut _ as _).ok() }
    }
    fn ILearningModelBindingFactory<R, F: FnOnce(&ILearningModelBindingFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LearningModelBinding, ILearningModelBindingFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for LearningModelBinding {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILearningModelBinding>();
}
unsafe impl windows_core::Interface for LearningModelBinding {
    type Vtable = <ILearningModelBinding as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ILearningModelBinding as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LearningModelBinding {
    const NAME: &'static str = "Windows.AI.MachineLearning.LearningModelBinding";
}
unsafe impl Send for LearningModelBinding {}
unsafe impl Sync for LearningModelBinding {}
impl IntoIterator for LearningModelBinding {
    type Item = windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &LearningModelBinding {
    type Item = windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LearningModelDevice(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LearningModelDevice, windows_core::IUnknown, windows_core::IInspectable);
impl LearningModelDevice {
    #[cfg(feature = "Graphics")]
    pub fn AdapterId(&self) -> windows_core::Result<super::super::Graphics::DisplayAdapterId> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AdapterId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Direct3D11Device(&self) -> windows_core::Result<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Direct3D11Device)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create(devicekind: LearningModelDeviceKind) -> windows_core::Result<LearningModelDevice> {
        Self::ILearningModelDeviceFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), devicekind, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CreateFromDirect3D11Device<P0>(device: P0) -> windows_core::Result<LearningModelDevice>
    where
        P0: windows_core::Param<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>,
    {
        Self::ILearningModelDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromDirect3D11Device)(windows_core::Interface::as_raw(this), device.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ILearningModelDeviceFactory<R, F: FnOnce(&ILearningModelDeviceFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LearningModelDevice, ILearningModelDeviceFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn ILearningModelDeviceStatics<R, F: FnOnce(&ILearningModelDeviceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LearningModelDevice, ILearningModelDeviceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for LearningModelDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILearningModelDevice>();
}
unsafe impl windows_core::Interface for LearningModelDevice {
    type Vtable = <ILearningModelDevice as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ILearningModelDevice as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LearningModelDevice {
    const NAME: &'static str = "Windows.AI.MachineLearning.LearningModelDevice";
}
unsafe impl Send for LearningModelDevice {}
unsafe impl Sync for LearningModelDevice {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LearningModelDeviceKind(pub i32);
impl LearningModelDeviceKind {
    pub const Default: Self = Self(0i32);
    pub const Cpu: Self = Self(1i32);
    pub const DirectX: Self = Self(2i32);
    pub const DirectXHighPerformance: Self = Self(3i32);
    pub const DirectXMinPower: Self = Self(4i32);
}
impl windows_core::TypeKind for LearningModelDeviceKind {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for LearningModelDeviceKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.AI.MachineLearning.LearningModelDeviceKind;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LearningModelEvaluationResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LearningModelEvaluationResult, windows_core::IUnknown, windows_core::IInspectable);
impl LearningModelEvaluationResult {
    pub fn CorrelationId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CorrelationId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ErrorStatus(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Succeeded(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Succeeded)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Outputs(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Outputs)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for LearningModelEvaluationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILearningModelEvaluationResult>();
}
unsafe impl windows_core::Interface for LearningModelEvaluationResult {
    type Vtable = <ILearningModelEvaluationResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ILearningModelEvaluationResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LearningModelEvaluationResult {
    const NAME: &'static str = "Windows.AI.MachineLearning.LearningModelEvaluationResult";
}
unsafe impl Send for LearningModelEvaluationResult {}
unsafe impl Sync for LearningModelEvaluationResult {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LearningModelFeatureKind(pub i32);
impl LearningModelFeatureKind {
    pub const Tensor: Self = Self(0i32);
    pub const Sequence: Self = Self(1i32);
    pub const Map: Self = Self(2i32);
    pub const Image: Self = Self(3i32);
}
impl windows_core::TypeKind for LearningModelFeatureKind {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for LearningModelFeatureKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.AI.MachineLearning.LearningModelFeatureKind;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LearningModelPixelRange(pub i32);
impl LearningModelPixelRange {
    pub const ZeroTo255: Self = Self(0i32);
    pub const ZeroToOne: Self = Self(1i32);
    pub const MinusOneToOne: Self = Self(2i32);
}
impl windows_core::TypeKind for LearningModelPixelRange {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for LearningModelPixelRange {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.AI.MachineLearning.LearningModelPixelRange;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LearningModelSession(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LearningModelSession, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(LearningModelSession, super::super::Foundation::IClosable);
impl LearningModelSession {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Model(&self) -> windows_core::Result<LearningModel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Model)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Device(&self) -> windows_core::Result<LearningModelDevice> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Device)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn EvaluationProperties(&self) -> windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EvaluationProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EvaluateAsync<P0>(&self, bindings: P0, correlationid: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<LearningModelEvaluationResult>>
    where
        P0: windows_core::Param<LearningModelBinding>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EvaluateAsync)(windows_core::Interface::as_raw(this), bindings.param().abi(), core::mem::transmute_copy(correlationid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EvaluateFeaturesAsync<P0>(&self, features: P0, correlationid: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<LearningModelEvaluationResult>>
    where
        P0: windows_core::Param<windows_collections::IMap<windows_core::HSTRING, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EvaluateFeaturesAsync)(windows_core::Interface::as_raw(this), features.param().abi(), core::mem::transmute_copy(correlationid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Evaluate<P0>(&self, bindings: P0, correlationid: &windows_core::HSTRING) -> windows_core::Result<LearningModelEvaluationResult>
    where
        P0: windows_core::Param<LearningModelBinding>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Evaluate)(windows_core::Interface::as_raw(this), bindings.param().abi(), core::mem::transmute_copy(correlationid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EvaluateFeatures<P0>(&self, features: P0, correlationid: &windows_core::HSTRING) -> windows_core::Result<LearningModelEvaluationResult>
    where
        P0: windows_core::Param<windows_collections::IMap<windows_core::HSTRING, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EvaluateFeatures)(windows_core::Interface::as_raw(this), features.param().abi(), core::mem::transmute_copy(correlationid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateFromModel<P0>(model: P0) -> windows_core::Result<LearningModelSession>
    where
        P0: windows_core::Param<LearningModel>,
    {
        Self::ILearningModelSessionFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromModel)(windows_core::Interface::as_raw(this), model.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromModelOnDevice<P0, P1>(model: P0, devicetorunon: P1) -> windows_core::Result<LearningModelSession>
    where
        P0: windows_core::Param<LearningModel>,
        P1: windows_core::Param<LearningModelDevice>,
    {
        Self::ILearningModelSessionFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromModelOnDevice)(windows_core::Interface::as_raw(this), model.param().abi(), devicetorunon.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromModelOnDeviceWithSessionOptions<P0, P1, P2>(model: P0, devicetorunon: P1, learningmodelsessionoptions: P2) -> windows_core::Result<LearningModelSession>
    where
        P0: windows_core::Param<LearningModel>,
        P1: windows_core::Param<LearningModelDevice>,
        P2: windows_core::Param<LearningModelSessionOptions>,
    {
        Self::ILearningModelSessionFactory2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromModelOnDeviceWithSessionOptions)(windows_core::Interface::as_raw(this), model.param().abi(), devicetorunon.param().abi(), learningmodelsessionoptions.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ILearningModelSessionFactory<R, F: FnOnce(&ILearningModelSessionFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LearningModelSession, ILearningModelSessionFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn ILearningModelSessionFactory2<R, F: FnOnce(&ILearningModelSessionFactory2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LearningModelSession, ILearningModelSessionFactory2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for LearningModelSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILearningModelSession>();
}
unsafe impl windows_core::Interface for LearningModelSession {
    type Vtable = <ILearningModelSession as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ILearningModelSession as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LearningModelSession {
    const NAME: &'static str = "Windows.AI.MachineLearning.LearningModelSession";
}
unsafe impl Send for LearningModelSession {}
unsafe impl Sync for LearningModelSession {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LearningModelSessionOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LearningModelSessionOptions, windows_core::IUnknown, windows_core::IInspectable);
impl LearningModelSessionOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LearningModelSessionOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn BatchSizeOverride(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BatchSizeOverride)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBatchSizeOverride(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBatchSizeOverride)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CloseModelOnSessionCreation(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ILearningModelSessionOptions2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CloseModelOnSessionCreation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCloseModelOnSessionCreation(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ILearningModelSessionOptions2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCloseModelOnSessionCreation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OverrideNamedDimension(&self, name: &windows_core::HSTRING, dimension: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ILearningModelSessionOptions3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OverrideNamedDimension)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), dimension).ok() }
    }
}
impl windows_core::RuntimeType for LearningModelSessionOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILearningModelSessionOptions>();
}
unsafe impl windows_core::Interface for LearningModelSessionOptions {
    type Vtable = <ILearningModelSessionOptions as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ILearningModelSessionOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LearningModelSessionOptions {
    const NAME: &'static str = "Windows.AI.MachineLearning.LearningModelSessionOptions";
}
unsafe impl Send for LearningModelSessionOptions {}
unsafe impl Sync for LearningModelSessionOptions {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MapFeatureDescriptor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MapFeatureDescriptor, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MapFeatureDescriptor, ILearningModelFeatureDescriptor);
impl MapFeatureDescriptor {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsRequired(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRequired)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn KeyKind(&self) -> windows_core::Result<TensorKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ValueDescriptor(&self) -> windows_core::Result<ILearningModelFeatureDescriptor> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ValueDescriptor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MapFeatureDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMapFeatureDescriptor>();
}
unsafe impl windows_core::Interface for MapFeatureDescriptor {
    type Vtable = <IMapFeatureDescriptor as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMapFeatureDescriptor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MapFeatureDescriptor {
    const NAME: &'static str = "Windows.AI.MachineLearning.MapFeatureDescriptor";
}
unsafe impl Send for MapFeatureDescriptor {}
unsafe impl Sync for MapFeatureDescriptor {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SequenceFeatureDescriptor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SequenceFeatureDescriptor, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SequenceFeatureDescriptor, ILearningModelFeatureDescriptor);
impl SequenceFeatureDescriptor {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsRequired(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRequired)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ElementDescriptor(&self) -> windows_core::Result<ILearningModelFeatureDescriptor> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ElementDescriptor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SequenceFeatureDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISequenceFeatureDescriptor>();
}
unsafe impl windows_core::Interface for SequenceFeatureDescriptor {
    type Vtable = <ISequenceFeatureDescriptor as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISequenceFeatureDescriptor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SequenceFeatureDescriptor {
    const NAME: &'static str = "Windows.AI.MachineLearning.SequenceFeatureDescriptor";
}
unsafe impl Send for SequenceFeatureDescriptor {}
unsafe impl Sync for SequenceFeatureDescriptor {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TensorBoolean(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TensorBoolean, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(TensorBoolean, super::super::Foundation::IClosable, ILearningModelFeatureValue, super::super::Foundation::IMemoryBuffer, ITensor);
impl TensorBoolean {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateReference(&self) -> windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateReference)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TensorKind(&self) -> windows_core::Result<TensorKind> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TensorKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Shape(&self) -> windows_core::Result<windows_collections::IVectorView<i64>> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shape)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAsVectorView(&self) -> windows_core::Result<windows_collections::IVectorView<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAsVectorView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create() -> windows_core::Result<TensorBoolean> {
        Self::ITensorBooleanStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Create2<P0>(shape: P0) -> windows_core::Result<TensorBoolean>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
    {
        Self::ITensorBooleanStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create2)(windows_core::Interface::as_raw(this), shape.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromArray<P0>(shape: P0, data: &[bool]) -> windows_core::Result<TensorBoolean>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
    {
        Self::ITensorBooleanStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromArray)(windows_core::Interface::as_raw(this), shape.param().abi(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> windows_core::Result<TensorBoolean>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
        P1: windows_core::Param<windows_collections::IIterable<bool>>,
    {
        Self::ITensorBooleanStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromIterable)(windows_core::Interface::as_raw(this), shape.param().abi(), data.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[bool]) -> windows_core::Result<TensorBoolean> {
        Self::ITensorBooleanStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P1>(shape: &[i64], buffer: P1) -> windows_core::Result<TensorBoolean>
    where
        P1: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorBooleanStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromBuffer)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ITensorBooleanStatics<R, F: FnOnce(&ITensorBooleanStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorBoolean, ITensorBooleanStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn ITensorBooleanStatics2<R, F: FnOnce(&ITensorBooleanStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorBoolean, ITensorBooleanStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TensorBoolean {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITensorBoolean>();
}
unsafe impl windows_core::Interface for TensorBoolean {
    type Vtable = <ITensorBoolean as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITensorBoolean as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TensorBoolean {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorBoolean";
}
unsafe impl Send for TensorBoolean {}
unsafe impl Sync for TensorBoolean {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TensorDouble(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TensorDouble, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(TensorDouble, super::super::Foundation::IClosable, ILearningModelFeatureValue, super::super::Foundation::IMemoryBuffer, ITensor);
impl TensorDouble {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateReference(&self) -> windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateReference)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TensorKind(&self) -> windows_core::Result<TensorKind> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TensorKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Shape(&self) -> windows_core::Result<windows_collections::IVectorView<i64>> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shape)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAsVectorView(&self) -> windows_core::Result<windows_collections::IVectorView<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAsVectorView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create() -> windows_core::Result<TensorDouble> {
        Self::ITensorDoubleStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Create2<P0>(shape: P0) -> windows_core::Result<TensorDouble>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
    {
        Self::ITensorDoubleStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create2)(windows_core::Interface::as_raw(this), shape.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromArray<P0>(shape: P0, data: &[f64]) -> windows_core::Result<TensorDouble>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
    {
        Self::ITensorDoubleStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromArray)(windows_core::Interface::as_raw(this), shape.param().abi(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> windows_core::Result<TensorDouble>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
        P1: windows_core::Param<windows_collections::IIterable<f64>>,
    {
        Self::ITensorDoubleStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromIterable)(windows_core::Interface::as_raw(this), shape.param().abi(), data.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[f64]) -> windows_core::Result<TensorDouble> {
        Self::ITensorDoubleStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P1>(shape: &[i64], buffer: P1) -> windows_core::Result<TensorDouble>
    where
        P1: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorDoubleStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromBuffer)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ITensorDoubleStatics<R, F: FnOnce(&ITensorDoubleStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorDouble, ITensorDoubleStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn ITensorDoubleStatics2<R, F: FnOnce(&ITensorDoubleStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorDouble, ITensorDoubleStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TensorDouble {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITensorDouble>();
}
unsafe impl windows_core::Interface for TensorDouble {
    type Vtable = <ITensorDouble as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITensorDouble as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TensorDouble {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorDouble";
}
unsafe impl Send for TensorDouble {}
unsafe impl Sync for TensorDouble {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TensorFeatureDescriptor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TensorFeatureDescriptor, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(TensorFeatureDescriptor, ILearningModelFeatureDescriptor);
impl TensorFeatureDescriptor {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsRequired(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRequired)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TensorKind(&self) -> windows_core::Result<TensorKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TensorKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Shape(&self) -> windows_core::Result<windows_collections::IVectorView<i64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shape)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for TensorFeatureDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITensorFeatureDescriptor>();
}
unsafe impl windows_core::Interface for TensorFeatureDescriptor {
    type Vtable = <ITensorFeatureDescriptor as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITensorFeatureDescriptor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TensorFeatureDescriptor {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorFeatureDescriptor";
}
unsafe impl Send for TensorFeatureDescriptor {}
unsafe impl Sync for TensorFeatureDescriptor {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TensorFloat(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TensorFloat, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(TensorFloat, super::super::Foundation::IClosable, ILearningModelFeatureValue, super::super::Foundation::IMemoryBuffer, ITensor);
impl TensorFloat {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateReference(&self) -> windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateReference)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TensorKind(&self) -> windows_core::Result<TensorKind> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TensorKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Shape(&self) -> windows_core::Result<windows_collections::IVectorView<i64>> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shape)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAsVectorView(&self) -> windows_core::Result<windows_collections::IVectorView<f32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAsVectorView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create() -> windows_core::Result<TensorFloat> {
        Self::ITensorFloatStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Create2<P0>(shape: P0) -> windows_core::Result<TensorFloat>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
    {
        Self::ITensorFloatStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create2)(windows_core::Interface::as_raw(this), shape.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromArray<P0>(shape: P0, data: &[f32]) -> windows_core::Result<TensorFloat>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
    {
        Self::ITensorFloatStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromArray)(windows_core::Interface::as_raw(this), shape.param().abi(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> windows_core::Result<TensorFloat>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
        P1: windows_core::Param<windows_collections::IIterable<f32>>,
    {
        Self::ITensorFloatStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromIterable)(windows_core::Interface::as_raw(this), shape.param().abi(), data.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[f32]) -> windows_core::Result<TensorFloat> {
        Self::ITensorFloatStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P1>(shape: &[i64], buffer: P1) -> windows_core::Result<TensorFloat>
    where
        P1: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorFloatStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromBuffer)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ITensorFloatStatics<R, F: FnOnce(&ITensorFloatStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorFloat, ITensorFloatStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn ITensorFloatStatics2<R, F: FnOnce(&ITensorFloatStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorFloat, ITensorFloatStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TensorFloat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITensorFloat>();
}
unsafe impl windows_core::Interface for TensorFloat {
    type Vtable = <ITensorFloat as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITensorFloat as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TensorFloat {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorFloat";
}
unsafe impl Send for TensorFloat {}
unsafe impl Sync for TensorFloat {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TensorFloat16Bit(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TensorFloat16Bit, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(TensorFloat16Bit, super::super::Foundation::IClosable, ILearningModelFeatureValue, super::super::Foundation::IMemoryBuffer, ITensor);
impl TensorFloat16Bit {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateReference(&self) -> windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateReference)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TensorKind(&self) -> windows_core::Result<TensorKind> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TensorKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Shape(&self) -> windows_core::Result<windows_collections::IVectorView<i64>> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shape)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAsVectorView(&self) -> windows_core::Result<windows_collections::IVectorView<f32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAsVectorView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create() -> windows_core::Result<TensorFloat16Bit> {
        Self::ITensorFloat16BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Create2<P0>(shape: P0) -> windows_core::Result<TensorFloat16Bit>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
    {
        Self::ITensorFloat16BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create2)(windows_core::Interface::as_raw(this), shape.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromArray<P0>(shape: P0, data: &[f32]) -> windows_core::Result<TensorFloat16Bit>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
    {
        Self::ITensorFloat16BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromArray)(windows_core::Interface::as_raw(this), shape.param().abi(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> windows_core::Result<TensorFloat16Bit>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
        P1: windows_core::Param<windows_collections::IIterable<f32>>,
    {
        Self::ITensorFloat16BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromIterable)(windows_core::Interface::as_raw(this), shape.param().abi(), data.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[f32]) -> windows_core::Result<TensorFloat16Bit> {
        Self::ITensorFloat16BitStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P1>(shape: &[i64], buffer: P1) -> windows_core::Result<TensorFloat16Bit>
    where
        P1: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorFloat16BitStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromBuffer)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ITensorFloat16BitStatics<R, F: FnOnce(&ITensorFloat16BitStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorFloat16Bit, ITensorFloat16BitStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn ITensorFloat16BitStatics2<R, F: FnOnce(&ITensorFloat16BitStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorFloat16Bit, ITensorFloat16BitStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TensorFloat16Bit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITensorFloat16Bit>();
}
unsafe impl windows_core::Interface for TensorFloat16Bit {
    type Vtable = <ITensorFloat16Bit as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITensorFloat16Bit as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TensorFloat16Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorFloat16Bit";
}
unsafe impl Send for TensorFloat16Bit {}
unsafe impl Sync for TensorFloat16Bit {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TensorInt16Bit(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TensorInt16Bit, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(TensorInt16Bit, super::super::Foundation::IClosable, ILearningModelFeatureValue, super::super::Foundation::IMemoryBuffer, ITensor);
impl TensorInt16Bit {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateReference(&self) -> windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateReference)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TensorKind(&self) -> windows_core::Result<TensorKind> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TensorKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Shape(&self) -> windows_core::Result<windows_collections::IVectorView<i64>> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shape)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAsVectorView(&self) -> windows_core::Result<windows_collections::IVectorView<i16>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAsVectorView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create() -> windows_core::Result<TensorInt16Bit> {
        Self::ITensorInt16BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Create2<P0>(shape: P0) -> windows_core::Result<TensorInt16Bit>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
    {
        Self::ITensorInt16BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create2)(windows_core::Interface::as_raw(this), shape.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromArray<P0>(shape: P0, data: &[i16]) -> windows_core::Result<TensorInt16Bit>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
    {
        Self::ITensorInt16BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromArray)(windows_core::Interface::as_raw(this), shape.param().abi(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> windows_core::Result<TensorInt16Bit>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
        P1: windows_core::Param<windows_collections::IIterable<i16>>,
    {
        Self::ITensorInt16BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromIterable)(windows_core::Interface::as_raw(this), shape.param().abi(), data.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[i16]) -> windows_core::Result<TensorInt16Bit> {
        Self::ITensorInt16BitStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P1>(shape: &[i64], buffer: P1) -> windows_core::Result<TensorInt16Bit>
    where
        P1: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorInt16BitStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromBuffer)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ITensorInt16BitStatics<R, F: FnOnce(&ITensorInt16BitStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorInt16Bit, ITensorInt16BitStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn ITensorInt16BitStatics2<R, F: FnOnce(&ITensorInt16BitStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorInt16Bit, ITensorInt16BitStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TensorInt16Bit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITensorInt16Bit>();
}
unsafe impl windows_core::Interface for TensorInt16Bit {
    type Vtable = <ITensorInt16Bit as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITensorInt16Bit as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TensorInt16Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorInt16Bit";
}
unsafe impl Send for TensorInt16Bit {}
unsafe impl Sync for TensorInt16Bit {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TensorInt32Bit(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TensorInt32Bit, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(TensorInt32Bit, super::super::Foundation::IClosable, ILearningModelFeatureValue, super::super::Foundation::IMemoryBuffer, ITensor);
impl TensorInt32Bit {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateReference(&self) -> windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateReference)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TensorKind(&self) -> windows_core::Result<TensorKind> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TensorKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Shape(&self) -> windows_core::Result<windows_collections::IVectorView<i64>> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shape)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAsVectorView(&self) -> windows_core::Result<windows_collections::IVectorView<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAsVectorView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create() -> windows_core::Result<TensorInt32Bit> {
        Self::ITensorInt32BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Create2<P0>(shape: P0) -> windows_core::Result<TensorInt32Bit>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
    {
        Self::ITensorInt32BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create2)(windows_core::Interface::as_raw(this), shape.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromArray<P0>(shape: P0, data: &[i32]) -> windows_core::Result<TensorInt32Bit>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
    {
        Self::ITensorInt32BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromArray)(windows_core::Interface::as_raw(this), shape.param().abi(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> windows_core::Result<TensorInt32Bit>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
        P1: windows_core::Param<windows_collections::IIterable<i32>>,
    {
        Self::ITensorInt32BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromIterable)(windows_core::Interface::as_raw(this), shape.param().abi(), data.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[i32]) -> windows_core::Result<TensorInt32Bit> {
        Self::ITensorInt32BitStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P1>(shape: &[i64], buffer: P1) -> windows_core::Result<TensorInt32Bit>
    where
        P1: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorInt32BitStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromBuffer)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ITensorInt32BitStatics<R, F: FnOnce(&ITensorInt32BitStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorInt32Bit, ITensorInt32BitStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn ITensorInt32BitStatics2<R, F: FnOnce(&ITensorInt32BitStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorInt32Bit, ITensorInt32BitStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TensorInt32Bit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITensorInt32Bit>();
}
unsafe impl windows_core::Interface for TensorInt32Bit {
    type Vtable = <ITensorInt32Bit as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITensorInt32Bit as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TensorInt32Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorInt32Bit";
}
unsafe impl Send for TensorInt32Bit {}
unsafe impl Sync for TensorInt32Bit {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TensorInt64Bit(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TensorInt64Bit, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(TensorInt64Bit, super::super::Foundation::IClosable, ILearningModelFeatureValue, super::super::Foundation::IMemoryBuffer, ITensor);
impl TensorInt64Bit {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateReference(&self) -> windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateReference)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TensorKind(&self) -> windows_core::Result<TensorKind> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TensorKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Shape(&self) -> windows_core::Result<windows_collections::IVectorView<i64>> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shape)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAsVectorView(&self) -> windows_core::Result<windows_collections::IVectorView<i64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAsVectorView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create() -> windows_core::Result<TensorInt64Bit> {
        Self::ITensorInt64BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Create2<P0>(shape: P0) -> windows_core::Result<TensorInt64Bit>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
    {
        Self::ITensorInt64BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create2)(windows_core::Interface::as_raw(this), shape.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromArray<P0>(shape: P0, data: &[i64]) -> windows_core::Result<TensorInt64Bit>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
    {
        Self::ITensorInt64BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromArray)(windows_core::Interface::as_raw(this), shape.param().abi(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> windows_core::Result<TensorInt64Bit>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
        P1: windows_core::Param<windows_collections::IIterable<i64>>,
    {
        Self::ITensorInt64BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromIterable)(windows_core::Interface::as_raw(this), shape.param().abi(), data.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[i64]) -> windows_core::Result<TensorInt64Bit> {
        Self::ITensorInt64BitStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P1>(shape: &[i64], buffer: P1) -> windows_core::Result<TensorInt64Bit>
    where
        P1: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorInt64BitStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromBuffer)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ITensorInt64BitStatics<R, F: FnOnce(&ITensorInt64BitStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorInt64Bit, ITensorInt64BitStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn ITensorInt64BitStatics2<R, F: FnOnce(&ITensorInt64BitStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorInt64Bit, ITensorInt64BitStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TensorInt64Bit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITensorInt64Bit>();
}
unsafe impl windows_core::Interface for TensorInt64Bit {
    type Vtable = <ITensorInt64Bit as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITensorInt64Bit as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TensorInt64Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorInt64Bit";
}
unsafe impl Send for TensorInt64Bit {}
unsafe impl Sync for TensorInt64Bit {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TensorInt8Bit(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TensorInt8Bit, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(TensorInt8Bit, super::super::Foundation::IClosable, ILearningModelFeatureValue, super::super::Foundation::IMemoryBuffer, ITensor);
impl TensorInt8Bit {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateReference(&self) -> windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateReference)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TensorKind(&self) -> windows_core::Result<TensorKind> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TensorKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Shape(&self) -> windows_core::Result<windows_collections::IVectorView<i64>> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shape)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAsVectorView(&self) -> windows_core::Result<windows_collections::IVectorView<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAsVectorView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create() -> windows_core::Result<TensorInt8Bit> {
        Self::ITensorInt8BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Create2<P0>(shape: P0) -> windows_core::Result<TensorInt8Bit>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
    {
        Self::ITensorInt8BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create2)(windows_core::Interface::as_raw(this), shape.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromArray<P0>(shape: P0, data: &[u8]) -> windows_core::Result<TensorInt8Bit>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
    {
        Self::ITensorInt8BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromArray)(windows_core::Interface::as_raw(this), shape.param().abi(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> windows_core::Result<TensorInt8Bit>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
        P1: windows_core::Param<windows_collections::IIterable<u8>>,
    {
        Self::ITensorInt8BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromIterable)(windows_core::Interface::as_raw(this), shape.param().abi(), data.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[u8]) -> windows_core::Result<TensorInt8Bit> {
        Self::ITensorInt8BitStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P1>(shape: &[i64], buffer: P1) -> windows_core::Result<TensorInt8Bit>
    where
        P1: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorInt8BitStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromBuffer)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ITensorInt8BitStatics<R, F: FnOnce(&ITensorInt8BitStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorInt8Bit, ITensorInt8BitStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn ITensorInt8BitStatics2<R, F: FnOnce(&ITensorInt8BitStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorInt8Bit, ITensorInt8BitStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TensorInt8Bit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITensorInt8Bit>();
}
unsafe impl windows_core::Interface for TensorInt8Bit {
    type Vtable = <ITensorInt8Bit as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITensorInt8Bit as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TensorInt8Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorInt8Bit";
}
unsafe impl Send for TensorInt8Bit {}
unsafe impl Sync for TensorInt8Bit {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
impl windows_core::TypeKind for TensorKind {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TensorKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.AI.MachineLearning.TensorKind;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TensorString(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TensorString, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(TensorString, super::super::Foundation::IClosable, ILearningModelFeatureValue, super::super::Foundation::IMemoryBuffer, ITensor);
impl TensorString {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateReference(&self) -> windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateReference)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TensorKind(&self) -> windows_core::Result<TensorKind> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TensorKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Shape(&self) -> windows_core::Result<windows_collections::IVectorView<i64>> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shape)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAsVectorView(&self) -> windows_core::Result<windows_collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAsVectorView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create() -> windows_core::Result<TensorString> {
        Self::ITensorStringStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Create2<P0>(shape: P0) -> windows_core::Result<TensorString>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
    {
        Self::ITensorStringStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create2)(windows_core::Interface::as_raw(this), shape.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromArray<P0>(shape: P0, data: &[windows_core::HSTRING]) -> windows_core::Result<TensorString>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
    {
        Self::ITensorStringStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromArray)(windows_core::Interface::as_raw(this), shape.param().abi(), data.len().try_into().unwrap(), core::mem::transmute(data.as_ptr()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> windows_core::Result<TensorString>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
        P1: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        Self::ITensorStringStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromIterable)(windows_core::Interface::as_raw(this), shape.param().abi(), data.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[windows_core::HSTRING]) -> windows_core::Result<TensorString> {
        Self::ITensorStringStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), core::mem::transmute(data.as_ptr()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ITensorStringStatics<R, F: FnOnce(&ITensorStringStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorString, ITensorStringStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn ITensorStringStatics2<R, F: FnOnce(&ITensorStringStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorString, ITensorStringStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TensorString {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITensorString>();
}
unsafe impl windows_core::Interface for TensorString {
    type Vtable = <ITensorString as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITensorString as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TensorString {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorString";
}
unsafe impl Send for TensorString {}
unsafe impl Sync for TensorString {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TensorUInt16Bit(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TensorUInt16Bit, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(TensorUInt16Bit, super::super::Foundation::IClosable, ILearningModelFeatureValue, super::super::Foundation::IMemoryBuffer, ITensor);
impl TensorUInt16Bit {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateReference(&self) -> windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateReference)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TensorKind(&self) -> windows_core::Result<TensorKind> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TensorKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Shape(&self) -> windows_core::Result<windows_collections::IVectorView<i64>> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shape)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAsVectorView(&self) -> windows_core::Result<windows_collections::IVectorView<u16>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAsVectorView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create() -> windows_core::Result<TensorUInt16Bit> {
        Self::ITensorUInt16BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Create2<P0>(shape: P0) -> windows_core::Result<TensorUInt16Bit>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
    {
        Self::ITensorUInt16BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create2)(windows_core::Interface::as_raw(this), shape.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromArray<P0>(shape: P0, data: &[u16]) -> windows_core::Result<TensorUInt16Bit>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
    {
        Self::ITensorUInt16BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromArray)(windows_core::Interface::as_raw(this), shape.param().abi(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> windows_core::Result<TensorUInt16Bit>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
        P1: windows_core::Param<windows_collections::IIterable<u16>>,
    {
        Self::ITensorUInt16BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromIterable)(windows_core::Interface::as_raw(this), shape.param().abi(), data.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[u16]) -> windows_core::Result<TensorUInt16Bit> {
        Self::ITensorUInt16BitStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P1>(shape: &[i64], buffer: P1) -> windows_core::Result<TensorUInt16Bit>
    where
        P1: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorUInt16BitStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromBuffer)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ITensorUInt16BitStatics<R, F: FnOnce(&ITensorUInt16BitStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorUInt16Bit, ITensorUInt16BitStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn ITensorUInt16BitStatics2<R, F: FnOnce(&ITensorUInt16BitStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorUInt16Bit, ITensorUInt16BitStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TensorUInt16Bit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITensorUInt16Bit>();
}
unsafe impl windows_core::Interface for TensorUInt16Bit {
    type Vtable = <ITensorUInt16Bit as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITensorUInt16Bit as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TensorUInt16Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorUInt16Bit";
}
unsafe impl Send for TensorUInt16Bit {}
unsafe impl Sync for TensorUInt16Bit {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TensorUInt32Bit(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TensorUInt32Bit, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(TensorUInt32Bit, super::super::Foundation::IClosable, ILearningModelFeatureValue, super::super::Foundation::IMemoryBuffer, ITensor);
impl TensorUInt32Bit {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateReference(&self) -> windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateReference)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TensorKind(&self) -> windows_core::Result<TensorKind> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TensorKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Shape(&self) -> windows_core::Result<windows_collections::IVectorView<i64>> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shape)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAsVectorView(&self) -> windows_core::Result<windows_collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAsVectorView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create() -> windows_core::Result<TensorUInt32Bit> {
        Self::ITensorUInt32BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Create2<P0>(shape: P0) -> windows_core::Result<TensorUInt32Bit>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
    {
        Self::ITensorUInt32BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create2)(windows_core::Interface::as_raw(this), shape.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromArray<P0>(shape: P0, data: &[u32]) -> windows_core::Result<TensorUInt32Bit>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
    {
        Self::ITensorUInt32BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromArray)(windows_core::Interface::as_raw(this), shape.param().abi(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> windows_core::Result<TensorUInt32Bit>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
        P1: windows_core::Param<windows_collections::IIterable<u32>>,
    {
        Self::ITensorUInt32BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromIterable)(windows_core::Interface::as_raw(this), shape.param().abi(), data.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[u32]) -> windows_core::Result<TensorUInt32Bit> {
        Self::ITensorUInt32BitStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P1>(shape: &[i64], buffer: P1) -> windows_core::Result<TensorUInt32Bit>
    where
        P1: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorUInt32BitStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromBuffer)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ITensorUInt32BitStatics<R, F: FnOnce(&ITensorUInt32BitStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorUInt32Bit, ITensorUInt32BitStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn ITensorUInt32BitStatics2<R, F: FnOnce(&ITensorUInt32BitStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorUInt32Bit, ITensorUInt32BitStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TensorUInt32Bit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITensorUInt32Bit>();
}
unsafe impl windows_core::Interface for TensorUInt32Bit {
    type Vtable = <ITensorUInt32Bit as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITensorUInt32Bit as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TensorUInt32Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorUInt32Bit";
}
unsafe impl Send for TensorUInt32Bit {}
unsafe impl Sync for TensorUInt32Bit {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TensorUInt64Bit(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TensorUInt64Bit, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(TensorUInt64Bit, super::super::Foundation::IClosable, ILearningModelFeatureValue, super::super::Foundation::IMemoryBuffer, ITensor);
impl TensorUInt64Bit {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateReference(&self) -> windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateReference)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TensorKind(&self) -> windows_core::Result<TensorKind> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TensorKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Shape(&self) -> windows_core::Result<windows_collections::IVectorView<i64>> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shape)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAsVectorView(&self) -> windows_core::Result<windows_collections::IVectorView<u64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAsVectorView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create() -> windows_core::Result<TensorUInt64Bit> {
        Self::ITensorUInt64BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Create2<P0>(shape: P0) -> windows_core::Result<TensorUInt64Bit>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
    {
        Self::ITensorUInt64BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create2)(windows_core::Interface::as_raw(this), shape.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromArray<P0>(shape: P0, data: &[u64]) -> windows_core::Result<TensorUInt64Bit>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
    {
        Self::ITensorUInt64BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromArray)(windows_core::Interface::as_raw(this), shape.param().abi(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> windows_core::Result<TensorUInt64Bit>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
        P1: windows_core::Param<windows_collections::IIterable<u64>>,
    {
        Self::ITensorUInt64BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromIterable)(windows_core::Interface::as_raw(this), shape.param().abi(), data.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[u64]) -> windows_core::Result<TensorUInt64Bit> {
        Self::ITensorUInt64BitStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P1>(shape: &[i64], buffer: P1) -> windows_core::Result<TensorUInt64Bit>
    where
        P1: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorUInt64BitStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromBuffer)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ITensorUInt64BitStatics<R, F: FnOnce(&ITensorUInt64BitStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorUInt64Bit, ITensorUInt64BitStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn ITensorUInt64BitStatics2<R, F: FnOnce(&ITensorUInt64BitStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorUInt64Bit, ITensorUInt64BitStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TensorUInt64Bit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITensorUInt64Bit>();
}
unsafe impl windows_core::Interface for TensorUInt64Bit {
    type Vtable = <ITensorUInt64Bit as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITensorUInt64Bit as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TensorUInt64Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorUInt64Bit";
}
unsafe impl Send for TensorUInt64Bit {}
unsafe impl Sync for TensorUInt64Bit {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TensorUInt8Bit(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TensorUInt8Bit, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(TensorUInt8Bit, super::super::Foundation::IClosable, ILearningModelFeatureValue, super::super::Foundation::IMemoryBuffer, ITensor);
impl TensorUInt8Bit {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateReference(&self) -> windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateReference)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TensorKind(&self) -> windows_core::Result<TensorKind> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TensorKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Shape(&self) -> windows_core::Result<windows_collections::IVectorView<i64>> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shape)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAsVectorView(&self) -> windows_core::Result<windows_collections::IVectorView<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAsVectorView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create() -> windows_core::Result<TensorUInt8Bit> {
        Self::ITensorUInt8BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Create2<P0>(shape: P0) -> windows_core::Result<TensorUInt8Bit>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
    {
        Self::ITensorUInt8BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create2)(windows_core::Interface::as_raw(this), shape.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromArray<P0>(shape: P0, data: &[u8]) -> windows_core::Result<TensorUInt8Bit>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
    {
        Self::ITensorUInt8BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromArray)(windows_core::Interface::as_raw(this), shape.param().abi(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> windows_core::Result<TensorUInt8Bit>
    where
        P0: windows_core::Param<windows_collections::IIterable<i64>>,
        P1: windows_core::Param<windows_collections::IIterable<u8>>,
    {
        Self::ITensorUInt8BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromIterable)(windows_core::Interface::as_raw(this), shape.param().abi(), data.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[u8]) -> windows_core::Result<TensorUInt8Bit> {
        Self::ITensorUInt8BitStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P1>(shape: &[i64], buffer: P1) -> windows_core::Result<TensorUInt8Bit>
    where
        P1: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorUInt8BitStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromBuffer)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ITensorUInt8BitStatics<R, F: FnOnce(&ITensorUInt8BitStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorUInt8Bit, ITensorUInt8BitStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn ITensorUInt8BitStatics2<R, F: FnOnce(&ITensorUInt8BitStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorUInt8Bit, ITensorUInt8BitStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TensorUInt8Bit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITensorUInt8Bit>();
}
unsafe impl windows_core::Interface for TensorUInt8Bit {
    type Vtable = <ITensorUInt8Bit as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITensorUInt8Bit as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TensorUInt8Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorUInt8Bit";
}
unsafe impl Send for TensorUInt8Bit {}
unsafe impl Sync for TensorUInt8Bit {}
