#[cfg(all(feature = "Graphics_Imaging", feature = "implement_exclusive"))]
pub trait IImageFeatureDescriptorImpl: Sized {
    fn BitmapPixelFormat(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapPixelFormat>;
    fn BitmapAlphaMode(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapAlphaMode>;
    fn Width(&self) -> ::windows::core::Result<u32>;
    fn Height(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IImageFeatureDescriptor {
    const NAME: &'static str = "Windows.AI.MachineLearning.IImageFeatureDescriptor";
}
#[cfg(all(feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl IImageFeatureDescriptorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageFeatureDescriptorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImageFeatureDescriptorVtbl {
        unsafe extern "system" fn BitmapPixelFormat<Impl: IImageFeatureDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BitmapPixelFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BitmapAlphaMode<Impl: IImageFeatureDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapAlphaMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BitmapAlphaMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Width<Impl: IImageFeatureDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Width() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Impl: IImageFeatureDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Height() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IImageFeatureDescriptor, BASE_OFFSET>(),
            BitmapPixelFormat: BitmapPixelFormat::<Impl, IMPL_OFFSET>,
            BitmapAlphaMode: BitmapAlphaMode::<Impl, IMPL_OFFSET>,
            Width: Width::<Impl, IMPL_OFFSET>,
            Height: Height::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImageFeatureDescriptor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageFeatureDescriptor2Impl: Sized {
    fn PixelRange(&self) -> ::windows::core::Result<LearningModelPixelRange>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IImageFeatureDescriptor2 {
    const NAME: &'static str = "Windows.AI.MachineLearning.IImageFeatureDescriptor2";
}
#[cfg(feature = "implement_exclusive")]
impl IImageFeatureDescriptor2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageFeatureDescriptor2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImageFeatureDescriptor2Vtbl {
        unsafe extern "system" fn PixelRange<Impl: IImageFeatureDescriptor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LearningModelPixelRange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PixelRange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IImageFeatureDescriptor2, BASE_OFFSET>(), PixelRange: PixelRange::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImageFeatureDescriptor2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media", feature = "implement_exclusive"))]
pub trait IImageFeatureValueImpl: Sized {
    fn VideoFrame(&self) -> ::windows::core::Result<super::super::Media::VideoFrame>;
}
#[cfg(all(feature = "Media", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IImageFeatureValue {
    const NAME: &'static str = "Windows.AI.MachineLearning.IImageFeatureValue";
}
#[cfg(all(feature = "Media", feature = "implement_exclusive"))]
impl IImageFeatureValueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageFeatureValueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImageFeatureValueVtbl {
        unsafe extern "system" fn VideoFrame<Impl: IImageFeatureValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IImageFeatureValue, BASE_OFFSET>(), VideoFrame: VideoFrame::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImageFeatureValue as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media", feature = "implement_exclusive"))]
pub trait IImageFeatureValueStaticsImpl: Sized {
    fn CreateFromVideoFrame(&self, image: &::core::option::Option<super::super::Media::VideoFrame>) -> ::windows::core::Result<ImageFeatureValue>;
}
#[cfg(all(feature = "Media", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IImageFeatureValueStatics {
    const NAME: &'static str = "Windows.AI.MachineLearning.IImageFeatureValueStatics";
}
#[cfg(all(feature = "Media", feature = "implement_exclusive"))]
impl IImageFeatureValueStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageFeatureValueStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImageFeatureValueStaticsVtbl {
        unsafe extern "system" fn CreateFromVideoFrame<Impl: IImageFeatureValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromVideoFrame(&*(&image as *const <super::super::Media::VideoFrame as ::windows::core::Abi>::Abi as *const <super::super::Media::VideoFrame as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IImageFeatureValueStatics, BASE_OFFSET>(),
            CreateFromVideoFrame: CreateFromVideoFrame::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImageFeatureValueStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ILearningModelImpl: Sized {
    fn Author(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Domain(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Version(&self) -> ::windows::core::Result<i64>;
    fn Metadata(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn InputFeatures(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ILearningModelFeatureDescriptor>>;
    fn OutputFeatures(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ILearningModelFeatureDescriptor>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILearningModel {
    const NAME: &'static str = "Windows.AI.MachineLearning.ILearningModel";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ILearningModelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILearningModelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILearningModelVtbl {
        unsafe extern "system" fn Author<Impl: ILearningModelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Author() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ILearningModelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Domain<Impl: ILearningModelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Domain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: ILearningModelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Version<Impl: ILearningModelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Version() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Metadata<Impl: ILearningModelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Metadata() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputFeatures<Impl: ILearningModelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputFeatures() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutputFeatures<Impl: ILearningModelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutputFeatures() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILearningModel, BASE_OFFSET>(),
            Author: Author::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            Domain: Domain::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            Version: Version::<Impl, IMPL_OFFSET>,
            Metadata: Metadata::<Impl, IMPL_OFFSET>,
            InputFeatures: InputFeatures::<Impl, IMPL_OFFSET>,
            OutputFeatures: OutputFeatures::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILearningModel as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ILearningModelBindingImpl: Sized {
    fn Bind(&self, name: &::windows::core::HSTRING, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn BindWithProperties(&self, name: &::windows::core::HSTRING, value: &::core::option::Option<::windows::core::IInspectable>, props: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILearningModelBinding {
    const NAME: &'static str = "Windows.AI.MachineLearning.ILearningModelBinding";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ILearningModelBindingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILearningModelBindingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILearningModelBindingVtbl {
        unsafe extern "system" fn Bind<Impl: ILearningModelBindingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Bind(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BindWithProperties<Impl: ILearningModelBindingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: *mut ::core::ffi::c_void, props: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .BindWithProperties(
                    &*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                    &*(&props as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn Clear<Impl: ILearningModelBindingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILearningModelBinding, BASE_OFFSET>(),
            Bind: Bind::<Impl, IMPL_OFFSET>,
            BindWithProperties: BindWithProperties::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILearningModelBinding as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILearningModelBindingFactoryImpl: Sized {
    fn CreateFromSession(&self, session: &::core::option::Option<LearningModelSession>) -> ::windows::core::Result<LearningModelBinding>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILearningModelBindingFactory {
    const NAME: &'static str = "Windows.AI.MachineLearning.ILearningModelBindingFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ILearningModelBindingFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILearningModelBindingFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILearningModelBindingFactoryVtbl {
        unsafe extern "system" fn CreateFromSession<Impl: ILearningModelBindingFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, session: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromSession(&*(&session as *const <LearningModelSession as ::windows::core::Abi>::Abi as *const <LearningModelSession as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILearningModelBindingFactory, BASE_OFFSET>(),
            CreateFromSession: CreateFromSession::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILearningModelBindingFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
pub trait ILearningModelDeviceImpl: Sized {
    fn AdapterId(&self) -> ::windows::core::Result<super::super::Graphics::DisplayAdapterId>;
    fn Direct3D11Device(&self) -> ::windows::core::Result<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>;
}
#[cfg(all(feature = "Graphics", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILearningModelDevice {
    const NAME: &'static str = "Windows.AI.MachineLearning.ILearningModelDevice";
}
#[cfg(all(feature = "Graphics", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl ILearningModelDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILearningModelDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILearningModelDeviceVtbl {
        unsafe extern "system" fn AdapterId<Impl: ILearningModelDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::DisplayAdapterId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdapterId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direct3D11Device<Impl: ILearningModelDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Direct3D11Device() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILearningModelDevice, BASE_OFFSET>(),
            AdapterId: AdapterId::<Impl, IMPL_OFFSET>,
            Direct3D11Device: Direct3D11Device::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILearningModelDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILearningModelDeviceFactoryImpl: Sized {
    fn Create(&self, devicekind: LearningModelDeviceKind) -> ::windows::core::Result<LearningModelDevice>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILearningModelDeviceFactory {
    const NAME: &'static str = "Windows.AI.MachineLearning.ILearningModelDeviceFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ILearningModelDeviceFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILearningModelDeviceFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILearningModelDeviceFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ILearningModelDeviceFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, devicekind: LearningModelDeviceKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(devicekind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILearningModelDeviceFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILearningModelDeviceFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
pub trait ILearningModelDeviceStaticsImpl: Sized {
    fn CreateFromDirect3D11Device(&self, device: &::core::option::Option<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>) -> ::windows::core::Result<LearningModelDevice>;
}
#[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILearningModelDeviceStatics {
    const NAME: &'static str = "Windows.AI.MachineLearning.ILearningModelDeviceStatics";
}
#[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl ILearningModelDeviceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILearningModelDeviceStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILearningModelDeviceStaticsVtbl {
        unsafe extern "system" fn CreateFromDirect3D11Device<Impl: ILearningModelDeviceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, device: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromDirect3D11Device(&*(&device as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice as ::windows::core::Abi>::Abi as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILearningModelDeviceStatics, BASE_OFFSET>(),
            CreateFromDirect3D11Device: CreateFromDirect3D11Device::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILearningModelDeviceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ILearningModelEvaluationResultImpl: Sized {
    fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ErrorStatus(&self) -> ::windows::core::Result<i32>;
    fn Succeeded(&self) -> ::windows::core::Result<bool>;
    fn Outputs(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILearningModelEvaluationResult {
    const NAME: &'static str = "Windows.AI.MachineLearning.ILearningModelEvaluationResult";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ILearningModelEvaluationResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILearningModelEvaluationResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILearningModelEvaluationResultVtbl {
        unsafe extern "system" fn CorrelationId<Impl: ILearningModelEvaluationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CorrelationId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorStatus<Impl: ILearningModelEvaluationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Succeeded<Impl: ILearningModelEvaluationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Succeeded() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Outputs<Impl: ILearningModelEvaluationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Outputs() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILearningModelEvaluationResult, BASE_OFFSET>(),
            CorrelationId: CorrelationId::<Impl, IMPL_OFFSET>,
            ErrorStatus: ErrorStatus::<Impl, IMPL_OFFSET>,
            Succeeded: Succeeded::<Impl, IMPL_OFFSET>,
            Outputs: Outputs::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILearningModelEvaluationResult as ::windows::core::Interface>::IID
    }
}
pub trait ILearningModelFeatureDescriptorImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind>;
    fn IsRequired(&self) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for ILearningModelFeatureDescriptor {
    const NAME: &'static str = "Windows.AI.MachineLearning.ILearningModelFeatureDescriptor";
}
impl ILearningModelFeatureDescriptorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILearningModelFeatureDescriptorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILearningModelFeatureDescriptorVtbl {
        unsafe extern "system" fn Name<Impl: ILearningModelFeatureDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: ILearningModelFeatureDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Kind<Impl: ILearningModelFeatureDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LearningModelFeatureKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRequired<Impl: ILearningModelFeatureDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRequired() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILearningModelFeatureDescriptor, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            Kind: Kind::<Impl, IMPL_OFFSET>,
            IsRequired: IsRequired::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILearningModelFeatureDescriptor as ::windows::core::Interface>::IID
    }
}
pub trait ILearningModelFeatureValueImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind>;
}
impl ::windows::core::RuntimeName for ILearningModelFeatureValue {
    const NAME: &'static str = "Windows.AI.MachineLearning.ILearningModelFeatureValue";
}
impl ILearningModelFeatureValueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILearningModelFeatureValueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILearningModelFeatureValueVtbl {
        unsafe extern "system" fn Kind<Impl: ILearningModelFeatureValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LearningModelFeatureKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILearningModelFeatureValue, BASE_OFFSET>(), Kind: Kind::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILearningModelFeatureValue as ::windows::core::Interface>::IID
    }
}
pub trait ILearningModelOperatorProviderImpl: Sized {}
impl ::windows::core::RuntimeName for ILearningModelOperatorProvider {
    const NAME: &'static str = "Windows.AI.MachineLearning.ILearningModelOperatorProvider";
}
impl ILearningModelOperatorProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILearningModelOperatorProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILearningModelOperatorProviderVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILearningModelOperatorProvider, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILearningModelOperatorProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ILearningModelSessionImpl: Sized {
    fn Model(&self) -> ::windows::core::Result<LearningModel>;
    fn Device(&self) -> ::windows::core::Result<LearningModelDevice>;
    fn EvaluationProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
    fn EvaluateAsync(&self, bindings: &::core::option::Option<LearningModelBinding>, correlationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LearningModelEvaluationResult>>;
    fn EvaluateFeaturesAsync(&self, features: &::core::option::Option<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>, correlationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LearningModelEvaluationResult>>;
    fn Evaluate(&self, bindings: &::core::option::Option<LearningModelBinding>, correlationid: &::windows::core::HSTRING) -> ::windows::core::Result<LearningModelEvaluationResult>;
    fn EvaluateFeatures(&self, features: &::core::option::Option<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>, correlationid: &::windows::core::HSTRING) -> ::windows::core::Result<LearningModelEvaluationResult>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILearningModelSession {
    const NAME: &'static str = "Windows.AI.MachineLearning.ILearningModelSession";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ILearningModelSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILearningModelSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILearningModelSessionVtbl {
        unsafe extern "system" fn Model<Impl: ILearningModelSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Model() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Device<Impl: ILearningModelSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Device() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EvaluationProperties<Impl: ILearningModelSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EvaluationProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EvaluateAsync<Impl: ILearningModelSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bindings: ::windows::core::RawPtr, correlationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EvaluateAsync(&*(&bindings as *const <LearningModelBinding as ::windows::core::Abi>::Abi as *const <LearningModelBinding as ::windows::core::DefaultType>::DefaultType), &*(&correlationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EvaluateFeaturesAsync<Impl: ILearningModelSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, features: ::windows::core::RawPtr, correlationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EvaluateFeaturesAsync(
                &*(&features as *const <super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType),
                &*(&correlationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Evaluate<Impl: ILearningModelSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bindings: ::windows::core::RawPtr, correlationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Evaluate(&*(&bindings as *const <LearningModelBinding as ::windows::core::Abi>::Abi as *const <LearningModelBinding as ::windows::core::DefaultType>::DefaultType), &*(&correlationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EvaluateFeatures<Impl: ILearningModelSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, features: ::windows::core::RawPtr, correlationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EvaluateFeatures(
                &*(&features as *const <super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType),
                &*(&correlationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILearningModelSession, BASE_OFFSET>(),
            Model: Model::<Impl, IMPL_OFFSET>,
            Device: Device::<Impl, IMPL_OFFSET>,
            EvaluationProperties: EvaluationProperties::<Impl, IMPL_OFFSET>,
            EvaluateAsync: EvaluateAsync::<Impl, IMPL_OFFSET>,
            EvaluateFeaturesAsync: EvaluateFeaturesAsync::<Impl, IMPL_OFFSET>,
            Evaluate: Evaluate::<Impl, IMPL_OFFSET>,
            EvaluateFeatures: EvaluateFeatures::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILearningModelSession as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILearningModelSessionFactoryImpl: Sized {
    fn CreateFromModel(&self, model: &::core::option::Option<LearningModel>) -> ::windows::core::Result<LearningModelSession>;
    fn CreateFromModelOnDevice(&self, model: &::core::option::Option<LearningModel>, devicetorunon: &::core::option::Option<LearningModelDevice>) -> ::windows::core::Result<LearningModelSession>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILearningModelSessionFactory {
    const NAME: &'static str = "Windows.AI.MachineLearning.ILearningModelSessionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ILearningModelSessionFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILearningModelSessionFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILearningModelSessionFactoryVtbl {
        unsafe extern "system" fn CreateFromModel<Impl: ILearningModelSessionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, model: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromModel(&*(&model as *const <LearningModel as ::windows::core::Abi>::Abi as *const <LearningModel as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromModelOnDevice<Impl: ILearningModelSessionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, model: ::windows::core::RawPtr, devicetorunon: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromModelOnDevice(&*(&model as *const <LearningModel as ::windows::core::Abi>::Abi as *const <LearningModel as ::windows::core::DefaultType>::DefaultType), &*(&devicetorunon as *const <LearningModelDevice as ::windows::core::Abi>::Abi as *const <LearningModelDevice as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILearningModelSessionFactory, BASE_OFFSET>(),
            CreateFromModel: CreateFromModel::<Impl, IMPL_OFFSET>,
            CreateFromModelOnDevice: CreateFromModelOnDevice::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILearningModelSessionFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILearningModelSessionFactory2Impl: Sized {
    fn CreateFromModelOnDeviceWithSessionOptions(&self, model: &::core::option::Option<LearningModel>, devicetorunon: &::core::option::Option<LearningModelDevice>, learningmodelsessionoptions: &::core::option::Option<LearningModelSessionOptions>) -> ::windows::core::Result<LearningModelSession>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILearningModelSessionFactory2 {
    const NAME: &'static str = "Windows.AI.MachineLearning.ILearningModelSessionFactory2";
}
#[cfg(feature = "implement_exclusive")]
impl ILearningModelSessionFactory2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILearningModelSessionFactory2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILearningModelSessionFactory2Vtbl {
        unsafe extern "system" fn CreateFromModelOnDeviceWithSessionOptions<Impl: ILearningModelSessionFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, model: ::windows::core::RawPtr, devicetorunon: ::windows::core::RawPtr, learningmodelsessionoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromModelOnDeviceWithSessionOptions(
                &*(&model as *const <LearningModel as ::windows::core::Abi>::Abi as *const <LearningModel as ::windows::core::DefaultType>::DefaultType),
                &*(&devicetorunon as *const <LearningModelDevice as ::windows::core::Abi>::Abi as *const <LearningModelDevice as ::windows::core::DefaultType>::DefaultType),
                &*(&learningmodelsessionoptions as *const <LearningModelSessionOptions as ::windows::core::Abi>::Abi as *const <LearningModelSessionOptions as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILearningModelSessionFactory2, BASE_OFFSET>(),
            CreateFromModelOnDeviceWithSessionOptions: CreateFromModelOnDeviceWithSessionOptions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILearningModelSessionFactory2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILearningModelSessionOptionsImpl: Sized {
    fn BatchSizeOverride(&self) -> ::windows::core::Result<u32>;
    fn SetBatchSizeOverride(&self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILearningModelSessionOptions {
    const NAME: &'static str = "Windows.AI.MachineLearning.ILearningModelSessionOptions";
}
#[cfg(feature = "implement_exclusive")]
impl ILearningModelSessionOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILearningModelSessionOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILearningModelSessionOptionsVtbl {
        unsafe extern "system" fn BatchSizeOverride<Impl: ILearningModelSessionOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BatchSizeOverride() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBatchSizeOverride<Impl: ILearningModelSessionOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBatchSizeOverride(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILearningModelSessionOptions, BASE_OFFSET>(),
            BatchSizeOverride: BatchSizeOverride::<Impl, IMPL_OFFSET>,
            SetBatchSizeOverride: SetBatchSizeOverride::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILearningModelSessionOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILearningModelSessionOptions2Impl: Sized {
    fn CloseModelOnSessionCreation(&self) -> ::windows::core::Result<bool>;
    fn SetCloseModelOnSessionCreation(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILearningModelSessionOptions2 {
    const NAME: &'static str = "Windows.AI.MachineLearning.ILearningModelSessionOptions2";
}
#[cfg(feature = "implement_exclusive")]
impl ILearningModelSessionOptions2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILearningModelSessionOptions2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILearningModelSessionOptions2Vtbl {
        unsafe extern "system" fn CloseModelOnSessionCreation<Impl: ILearningModelSessionOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CloseModelOnSessionCreation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCloseModelOnSessionCreation<Impl: ILearningModelSessionOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCloseModelOnSessionCreation(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILearningModelSessionOptions2, BASE_OFFSET>(),
            CloseModelOnSessionCreation: CloseModelOnSessionCreation::<Impl, IMPL_OFFSET>,
            SetCloseModelOnSessionCreation: SetCloseModelOnSessionCreation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILearningModelSessionOptions2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILearningModelSessionOptions3Impl: Sized {
    fn OverrideNamedDimension(&self, name: &::windows::core::HSTRING, dimension: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILearningModelSessionOptions3 {
    const NAME: &'static str = "Windows.AI.MachineLearning.ILearningModelSessionOptions3";
}
#[cfg(feature = "implement_exclusive")]
impl ILearningModelSessionOptions3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILearningModelSessionOptions3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILearningModelSessionOptions3Vtbl {
        unsafe extern "system" fn OverrideNamedDimension<Impl: ILearningModelSessionOptions3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, dimension: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OverrideNamedDimension(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), dimension).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILearningModelSessionOptions3, BASE_OFFSET>(),
            OverrideNamedDimension: OverrideNamedDimension::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILearningModelSessionOptions3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ILearningModelStaticsImpl: Sized {
    fn LoadFromStorageFileAsync(&self, modelfile: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LearningModel>>;
    fn LoadFromStreamAsync(&self, modelstream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LearningModel>>;
    fn LoadFromFilePath(&self, filepath: &::windows::core::HSTRING) -> ::windows::core::Result<LearningModel>;
    fn LoadFromStream(&self, modelstream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<LearningModel>;
    fn LoadFromStorageFileWithOperatorProviderAsync(&self, modelfile: &::core::option::Option<super::super::Storage::IStorageFile>, operatorprovider: &::core::option::Option<ILearningModelOperatorProvider>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LearningModel>>;
    fn LoadFromStreamWithOperatorProviderAsync(&self, modelstream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>, operatorprovider: &::core::option::Option<ILearningModelOperatorProvider>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LearningModel>>;
    fn LoadFromFilePathWithOperatorProvider(&self, filepath: &::windows::core::HSTRING, operatorprovider: &::core::option::Option<ILearningModelOperatorProvider>) -> ::windows::core::Result<LearningModel>;
    fn LoadFromStreamWithOperatorProvider(&self, modelstream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>, operatorprovider: &::core::option::Option<ILearningModelOperatorProvider>) -> ::windows::core::Result<LearningModel>;
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILearningModelStatics {
    const NAME: &'static str = "Windows.AI.MachineLearning.ILearningModelStatics";
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ILearningModelStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILearningModelStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILearningModelStaticsVtbl {
        unsafe extern "system" fn LoadFromStorageFileAsync<Impl: ILearningModelStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modelfile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadFromStorageFileAsync(&*(&modelfile as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadFromStreamAsync<Impl: ILearningModelStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modelstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadFromStreamAsync(&*(&modelstream as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadFromFilePath<Impl: ILearningModelStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadFromFilePath(&*(&filepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadFromStream<Impl: ILearningModelStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modelstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadFromStream(&*(&modelstream as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadFromStorageFileWithOperatorProviderAsync<Impl: ILearningModelStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modelfile: ::windows::core::RawPtr, operatorprovider: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadFromStorageFileWithOperatorProviderAsync(&*(&modelfile as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType), &*(&operatorprovider as *const <ILearningModelOperatorProvider as ::windows::core::Abi>::Abi as *const <ILearningModelOperatorProvider as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadFromStreamWithOperatorProviderAsync<Impl: ILearningModelStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modelstream: ::windows::core::RawPtr, operatorprovider: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadFromStreamWithOperatorProviderAsync(&*(&modelstream as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType), &*(&operatorprovider as *const <ILearningModelOperatorProvider as ::windows::core::Abi>::Abi as *const <ILearningModelOperatorProvider as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadFromFilePathWithOperatorProvider<Impl: ILearningModelStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, operatorprovider: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadFromFilePathWithOperatorProvider(&*(&filepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&operatorprovider as *const <ILearningModelOperatorProvider as ::windows::core::Abi>::Abi as *const <ILearningModelOperatorProvider as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadFromStreamWithOperatorProvider<Impl: ILearningModelStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modelstream: ::windows::core::RawPtr, operatorprovider: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadFromStreamWithOperatorProvider(&*(&modelstream as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType), &*(&operatorprovider as *const <ILearningModelOperatorProvider as ::windows::core::Abi>::Abi as *const <ILearningModelOperatorProvider as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILearningModelStatics, BASE_OFFSET>(),
            LoadFromStorageFileAsync: LoadFromStorageFileAsync::<Impl, IMPL_OFFSET>,
            LoadFromStreamAsync: LoadFromStreamAsync::<Impl, IMPL_OFFSET>,
            LoadFromFilePath: LoadFromFilePath::<Impl, IMPL_OFFSET>,
            LoadFromStream: LoadFromStream::<Impl, IMPL_OFFSET>,
            LoadFromStorageFileWithOperatorProviderAsync: LoadFromStorageFileWithOperatorProviderAsync::<Impl, IMPL_OFFSET>,
            LoadFromStreamWithOperatorProviderAsync: LoadFromStreamWithOperatorProviderAsync::<Impl, IMPL_OFFSET>,
            LoadFromFilePathWithOperatorProvider: LoadFromFilePathWithOperatorProvider::<Impl, IMPL_OFFSET>,
            LoadFromStreamWithOperatorProvider: LoadFromStreamWithOperatorProvider::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILearningModelStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapFeatureDescriptorImpl: Sized {
    fn KeyKind(&self) -> ::windows::core::Result<TensorKind>;
    fn ValueDescriptor(&self) -> ::windows::core::Result<ILearningModelFeatureDescriptor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapFeatureDescriptor {
    const NAME: &'static str = "Windows.AI.MachineLearning.IMapFeatureDescriptor";
}
#[cfg(feature = "implement_exclusive")]
impl IMapFeatureDescriptorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapFeatureDescriptorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapFeatureDescriptorVtbl {
        unsafe extern "system" fn KeyKind<Impl: IMapFeatureDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TensorKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValueDescriptor<Impl: IMapFeatureDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValueDescriptor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapFeatureDescriptor, BASE_OFFSET>(),
            KeyKind: KeyKind::<Impl, IMPL_OFFSET>,
            ValueDescriptor: ValueDescriptor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapFeatureDescriptor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISequenceFeatureDescriptorImpl: Sized {
    fn ElementDescriptor(&self) -> ::windows::core::Result<ILearningModelFeatureDescriptor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISequenceFeatureDescriptor {
    const NAME: &'static str = "Windows.AI.MachineLearning.ISequenceFeatureDescriptor";
}
#[cfg(feature = "implement_exclusive")]
impl ISequenceFeatureDescriptorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISequenceFeatureDescriptorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISequenceFeatureDescriptorVtbl {
        unsafe extern "system" fn ElementDescriptor<Impl: ISequenceFeatureDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElementDescriptor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISequenceFeatureDescriptor, BASE_OFFSET>(),
            ElementDescriptor: ElementDescriptor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISequenceFeatureDescriptor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait ITensorImpl: Sized + ILearningModelFeatureValueImpl {
    fn TensorKind(&self) -> ::windows::core::Result<TensorKind>;
    fn Shape(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for ITensor {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensor";
}
#[cfg(feature = "Foundation_Collections")]
impl ITensorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorVtbl {
        unsafe extern "system" fn TensorKind<Impl: ITensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TensorKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TensorKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shape<Impl: ITensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Shape() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITensor, BASE_OFFSET>(),
            TensorKind: TensorKind::<Impl, IMPL_OFFSET>,
            Shape: Shape::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITensorBooleanImpl: Sized {
    fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<bool>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorBoolean {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorBoolean";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITensorBooleanVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorBooleanImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorBooleanVtbl {
        unsafe extern "system" fn GetAsVectorView<Impl: ITensorBooleanImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAsVectorView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorBoolean, BASE_OFFSET>(), GetAsVectorView: GetAsVectorView::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorBoolean as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITensorBooleanStaticsImpl: Sized {
    fn Create(&self) -> ::windows::core::Result<TensorBoolean>;
    fn Create2(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>) -> ::windows::core::Result<TensorBoolean>;
    fn CreateFromArray(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &[<bool as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorBoolean>;
    fn CreateFromIterable(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &::core::option::Option<super::super::Foundation::Collections::IIterable<bool>>) -> ::windows::core::Result<TensorBoolean>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorBooleanStatics {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorBooleanStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITensorBooleanStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorBooleanStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorBooleanStaticsVtbl {
        unsafe extern "system" fn Create<Impl: ITensorBooleanStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create2<Impl: ITensorBooleanStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create2(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromArray<Impl: ITensorBooleanStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data_array_size: u32, data: *const bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromArray(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&data), data_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromIterable<Impl: ITensorBooleanStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromIterable(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType), &*(&data as *const <super::super::Foundation::Collections::IIterable<bool> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<bool> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorBooleanStatics, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            Create2: Create2::<Impl, IMPL_OFFSET>,
            CreateFromArray: CreateFromArray::<Impl, IMPL_OFFSET>,
            CreateFromIterable: CreateFromIterable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorBooleanStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ITensorBooleanStatics2Impl: Sized {
    fn CreateFromShapeArrayAndDataArray(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], data: &[<bool as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorBoolean>;
    fn CreateFromBuffer(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<TensorBoolean>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorBooleanStatics2 {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorBooleanStatics2";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ITensorBooleanStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorBooleanStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorBooleanStatics2Vtbl {
        unsafe extern "system" fn CreateFromShapeArrayAndDataArray<Impl: ITensorBooleanStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromShapeArrayAndDataArray(::core::slice::from_raw_parts(::core::mem::transmute_copy(&shape), shape_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&data), data_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromBuffer<Impl: ITensorBooleanStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromBuffer(::core::slice::from_raw_parts(::core::mem::transmute_copy(&shape), shape_array_size as _), &*(&buffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorBooleanStatics2, BASE_OFFSET>(),
            CreateFromShapeArrayAndDataArray: CreateFromShapeArrayAndDataArray::<Impl, IMPL_OFFSET>,
            CreateFromBuffer: CreateFromBuffer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorBooleanStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITensorDoubleImpl: Sized {
    fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<f64>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorDouble {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorDouble";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITensorDoubleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorDoubleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorDoubleVtbl {
        unsafe extern "system" fn GetAsVectorView<Impl: ITensorDoubleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAsVectorView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorDouble, BASE_OFFSET>(), GetAsVectorView: GetAsVectorView::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorDouble as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITensorDoubleStaticsImpl: Sized {
    fn Create(&self) -> ::windows::core::Result<TensorDouble>;
    fn Create2(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>) -> ::windows::core::Result<TensorDouble>;
    fn CreateFromArray(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &[<f64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorDouble>;
    fn CreateFromIterable(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &::core::option::Option<super::super::Foundation::Collections::IIterable<f64>>) -> ::windows::core::Result<TensorDouble>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorDoubleStatics {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorDoubleStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITensorDoubleStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorDoubleStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorDoubleStaticsVtbl {
        unsafe extern "system" fn Create<Impl: ITensorDoubleStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create2<Impl: ITensorDoubleStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create2(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromArray<Impl: ITensorDoubleStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data_array_size: u32, data: *const f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromArray(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&data), data_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromIterable<Impl: ITensorDoubleStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromIterable(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType), &*(&data as *const <super::super::Foundation::Collections::IIterable<f64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<f64> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorDoubleStatics, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            Create2: Create2::<Impl, IMPL_OFFSET>,
            CreateFromArray: CreateFromArray::<Impl, IMPL_OFFSET>,
            CreateFromIterable: CreateFromIterable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorDoubleStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ITensorDoubleStatics2Impl: Sized {
    fn CreateFromShapeArrayAndDataArray(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], data: &[<f64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorDouble>;
    fn CreateFromBuffer(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<TensorDouble>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorDoubleStatics2 {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorDoubleStatics2";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ITensorDoubleStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorDoubleStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorDoubleStatics2Vtbl {
        unsafe extern "system" fn CreateFromShapeArrayAndDataArray<Impl: ITensorDoubleStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromShapeArrayAndDataArray(::core::slice::from_raw_parts(::core::mem::transmute_copy(&shape), shape_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&data), data_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromBuffer<Impl: ITensorDoubleStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromBuffer(::core::slice::from_raw_parts(::core::mem::transmute_copy(&shape), shape_array_size as _), &*(&buffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorDoubleStatics2, BASE_OFFSET>(),
            CreateFromShapeArrayAndDataArray: CreateFromShapeArrayAndDataArray::<Impl, IMPL_OFFSET>,
            CreateFromBuffer: CreateFromBuffer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorDoubleStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITensorFeatureDescriptorImpl: Sized {
    fn TensorKind(&self) -> ::windows::core::Result<TensorKind>;
    fn Shape(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorFeatureDescriptor {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorFeatureDescriptor";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITensorFeatureDescriptorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorFeatureDescriptorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorFeatureDescriptorVtbl {
        unsafe extern "system" fn TensorKind<Impl: ITensorFeatureDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TensorKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TensorKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shape<Impl: ITensorFeatureDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Shape() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorFeatureDescriptor, BASE_OFFSET>(),
            TensorKind: TensorKind::<Impl, IMPL_OFFSET>,
            Shape: Shape::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorFeatureDescriptor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITensorFloatImpl: Sized {
    fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<f32>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorFloat {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorFloat";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITensorFloatVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorFloatImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorFloatVtbl {
        unsafe extern "system" fn GetAsVectorView<Impl: ITensorFloatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAsVectorView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorFloat, BASE_OFFSET>(), GetAsVectorView: GetAsVectorView::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorFloat as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITensorFloat16BitImpl: Sized {
    fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<f32>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorFloat16Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorFloat16Bit";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITensorFloat16BitVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorFloat16BitImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorFloat16BitVtbl {
        unsafe extern "system" fn GetAsVectorView<Impl: ITensorFloat16BitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAsVectorView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorFloat16Bit, BASE_OFFSET>(),
            GetAsVectorView: GetAsVectorView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorFloat16Bit as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITensorFloat16BitStaticsImpl: Sized {
    fn Create(&self) -> ::windows::core::Result<TensorFloat16Bit>;
    fn Create2(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>) -> ::windows::core::Result<TensorFloat16Bit>;
    fn CreateFromArray(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &[<f32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorFloat16Bit>;
    fn CreateFromIterable(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &::core::option::Option<super::super::Foundation::Collections::IIterable<f32>>) -> ::windows::core::Result<TensorFloat16Bit>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorFloat16BitStatics {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorFloat16BitStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITensorFloat16BitStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorFloat16BitStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorFloat16BitStaticsVtbl {
        unsafe extern "system" fn Create<Impl: ITensorFloat16BitStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create2<Impl: ITensorFloat16BitStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create2(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromArray<Impl: ITensorFloat16BitStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data_array_size: u32, data: *const f32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromArray(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&data), data_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromIterable<Impl: ITensorFloat16BitStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromIterable(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType), &*(&data as *const <super::super::Foundation::Collections::IIterable<f32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<f32> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorFloat16BitStatics, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            Create2: Create2::<Impl, IMPL_OFFSET>,
            CreateFromArray: CreateFromArray::<Impl, IMPL_OFFSET>,
            CreateFromIterable: CreateFromIterable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorFloat16BitStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ITensorFloat16BitStatics2Impl: Sized {
    fn CreateFromShapeArrayAndDataArray(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], data: &[<f32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorFloat16Bit>;
    fn CreateFromBuffer(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<TensorFloat16Bit>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorFloat16BitStatics2 {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorFloat16BitStatics2";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ITensorFloat16BitStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorFloat16BitStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorFloat16BitStatics2Vtbl {
        unsafe extern "system" fn CreateFromShapeArrayAndDataArray<Impl: ITensorFloat16BitStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const f32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromShapeArrayAndDataArray(::core::slice::from_raw_parts(::core::mem::transmute_copy(&shape), shape_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&data), data_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromBuffer<Impl: ITensorFloat16BitStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromBuffer(::core::slice::from_raw_parts(::core::mem::transmute_copy(&shape), shape_array_size as _), &*(&buffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorFloat16BitStatics2, BASE_OFFSET>(),
            CreateFromShapeArrayAndDataArray: CreateFromShapeArrayAndDataArray::<Impl, IMPL_OFFSET>,
            CreateFromBuffer: CreateFromBuffer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorFloat16BitStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITensorFloatStaticsImpl: Sized {
    fn Create(&self) -> ::windows::core::Result<TensorFloat>;
    fn Create2(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>) -> ::windows::core::Result<TensorFloat>;
    fn CreateFromArray(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &[<f32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorFloat>;
    fn CreateFromIterable(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &::core::option::Option<super::super::Foundation::Collections::IIterable<f32>>) -> ::windows::core::Result<TensorFloat>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorFloatStatics {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorFloatStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITensorFloatStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorFloatStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorFloatStaticsVtbl {
        unsafe extern "system" fn Create<Impl: ITensorFloatStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create2<Impl: ITensorFloatStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create2(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromArray<Impl: ITensorFloatStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data_array_size: u32, data: *const f32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromArray(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&data), data_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromIterable<Impl: ITensorFloatStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromIterable(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType), &*(&data as *const <super::super::Foundation::Collections::IIterable<f32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<f32> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorFloatStatics, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            Create2: Create2::<Impl, IMPL_OFFSET>,
            CreateFromArray: CreateFromArray::<Impl, IMPL_OFFSET>,
            CreateFromIterable: CreateFromIterable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorFloatStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ITensorFloatStatics2Impl: Sized {
    fn CreateFromShapeArrayAndDataArray(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], data: &[<f32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorFloat>;
    fn CreateFromBuffer(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<TensorFloat>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorFloatStatics2 {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorFloatStatics2";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ITensorFloatStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorFloatStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorFloatStatics2Vtbl {
        unsafe extern "system" fn CreateFromShapeArrayAndDataArray<Impl: ITensorFloatStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const f32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromShapeArrayAndDataArray(::core::slice::from_raw_parts(::core::mem::transmute_copy(&shape), shape_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&data), data_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromBuffer<Impl: ITensorFloatStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromBuffer(::core::slice::from_raw_parts(::core::mem::transmute_copy(&shape), shape_array_size as _), &*(&buffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorFloatStatics2, BASE_OFFSET>(),
            CreateFromShapeArrayAndDataArray: CreateFromShapeArrayAndDataArray::<Impl, IMPL_OFFSET>,
            CreateFromBuffer: CreateFromBuffer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorFloatStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITensorInt16BitImpl: Sized {
    fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i16>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorInt16Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorInt16Bit";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITensorInt16BitVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorInt16BitImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorInt16BitVtbl {
        unsafe extern "system" fn GetAsVectorView<Impl: ITensorInt16BitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAsVectorView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorInt16Bit, BASE_OFFSET>(), GetAsVectorView: GetAsVectorView::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorInt16Bit as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITensorInt16BitStaticsImpl: Sized {
    fn Create(&self) -> ::windows::core::Result<TensorInt16Bit>;
    fn Create2(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>) -> ::windows::core::Result<TensorInt16Bit>;
    fn CreateFromArray(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &[<i16 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorInt16Bit>;
    fn CreateFromIterable(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &::core::option::Option<super::super::Foundation::Collections::IIterable<i16>>) -> ::windows::core::Result<TensorInt16Bit>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorInt16BitStatics {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorInt16BitStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITensorInt16BitStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorInt16BitStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorInt16BitStaticsVtbl {
        unsafe extern "system" fn Create<Impl: ITensorInt16BitStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create2<Impl: ITensorInt16BitStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create2(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromArray<Impl: ITensorInt16BitStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data_array_size: u32, data: *const i16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromArray(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&data), data_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromIterable<Impl: ITensorInt16BitStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromIterable(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType), &*(&data as *const <super::super::Foundation::Collections::IIterable<i16> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i16> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorInt16BitStatics, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            Create2: Create2::<Impl, IMPL_OFFSET>,
            CreateFromArray: CreateFromArray::<Impl, IMPL_OFFSET>,
            CreateFromIterable: CreateFromIterable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorInt16BitStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ITensorInt16BitStatics2Impl: Sized {
    fn CreateFromShapeArrayAndDataArray(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], data: &[<i16 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorInt16Bit>;
    fn CreateFromBuffer(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<TensorInt16Bit>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorInt16BitStatics2 {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorInt16BitStatics2";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ITensorInt16BitStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorInt16BitStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorInt16BitStatics2Vtbl {
        unsafe extern "system" fn CreateFromShapeArrayAndDataArray<Impl: ITensorInt16BitStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const i16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromShapeArrayAndDataArray(::core::slice::from_raw_parts(::core::mem::transmute_copy(&shape), shape_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&data), data_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromBuffer<Impl: ITensorInt16BitStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromBuffer(::core::slice::from_raw_parts(::core::mem::transmute_copy(&shape), shape_array_size as _), &*(&buffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorInt16BitStatics2, BASE_OFFSET>(),
            CreateFromShapeArrayAndDataArray: CreateFromShapeArrayAndDataArray::<Impl, IMPL_OFFSET>,
            CreateFromBuffer: CreateFromBuffer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorInt16BitStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITensorInt32BitImpl: Sized {
    fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i32>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorInt32Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorInt32Bit";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITensorInt32BitVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorInt32BitImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorInt32BitVtbl {
        unsafe extern "system" fn GetAsVectorView<Impl: ITensorInt32BitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAsVectorView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorInt32Bit, BASE_OFFSET>(), GetAsVectorView: GetAsVectorView::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorInt32Bit as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITensorInt32BitStaticsImpl: Sized {
    fn Create(&self) -> ::windows::core::Result<TensorInt32Bit>;
    fn Create2(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>) -> ::windows::core::Result<TensorInt32Bit>;
    fn CreateFromArray(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorInt32Bit>;
    fn CreateFromIterable(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &::core::option::Option<super::super::Foundation::Collections::IIterable<i32>>) -> ::windows::core::Result<TensorInt32Bit>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorInt32BitStatics {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorInt32BitStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITensorInt32BitStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorInt32BitStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorInt32BitStaticsVtbl {
        unsafe extern "system" fn Create<Impl: ITensorInt32BitStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create2<Impl: ITensorInt32BitStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create2(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromArray<Impl: ITensorInt32BitStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data_array_size: u32, data: *const i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromArray(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&data), data_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromIterable<Impl: ITensorInt32BitStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromIterable(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType), &*(&data as *const <super::super::Foundation::Collections::IIterable<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i32> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorInt32BitStatics, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            Create2: Create2::<Impl, IMPL_OFFSET>,
            CreateFromArray: CreateFromArray::<Impl, IMPL_OFFSET>,
            CreateFromIterable: CreateFromIterable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorInt32BitStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ITensorInt32BitStatics2Impl: Sized {
    fn CreateFromShapeArrayAndDataArray(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], data: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorInt32Bit>;
    fn CreateFromBuffer(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<TensorInt32Bit>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorInt32BitStatics2 {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorInt32BitStatics2";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ITensorInt32BitStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorInt32BitStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorInt32BitStatics2Vtbl {
        unsafe extern "system" fn CreateFromShapeArrayAndDataArray<Impl: ITensorInt32BitStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromShapeArrayAndDataArray(::core::slice::from_raw_parts(::core::mem::transmute_copy(&shape), shape_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&data), data_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromBuffer<Impl: ITensorInt32BitStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromBuffer(::core::slice::from_raw_parts(::core::mem::transmute_copy(&shape), shape_array_size as _), &*(&buffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorInt32BitStatics2, BASE_OFFSET>(),
            CreateFromShapeArrayAndDataArray: CreateFromShapeArrayAndDataArray::<Impl, IMPL_OFFSET>,
            CreateFromBuffer: CreateFromBuffer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorInt32BitStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITensorInt64BitImpl: Sized {
    fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorInt64Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorInt64Bit";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITensorInt64BitVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorInt64BitImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorInt64BitVtbl {
        unsafe extern "system" fn GetAsVectorView<Impl: ITensorInt64BitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAsVectorView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorInt64Bit, BASE_OFFSET>(), GetAsVectorView: GetAsVectorView::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorInt64Bit as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITensorInt64BitStaticsImpl: Sized {
    fn Create(&self) -> ::windows::core::Result<TensorInt64Bit>;
    fn Create2(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>) -> ::windows::core::Result<TensorInt64Bit>;
    fn CreateFromArray(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &[<i64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorInt64Bit>;
    fn CreateFromIterable(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>) -> ::windows::core::Result<TensorInt64Bit>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorInt64BitStatics {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorInt64BitStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITensorInt64BitStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorInt64BitStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorInt64BitStaticsVtbl {
        unsafe extern "system" fn Create<Impl: ITensorInt64BitStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create2<Impl: ITensorInt64BitStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create2(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromArray<Impl: ITensorInt64BitStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data_array_size: u32, data: *const i64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromArray(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&data), data_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromIterable<Impl: ITensorInt64BitStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromIterable(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType), &*(&data as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorInt64BitStatics, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            Create2: Create2::<Impl, IMPL_OFFSET>,
            CreateFromArray: CreateFromArray::<Impl, IMPL_OFFSET>,
            CreateFromIterable: CreateFromIterable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorInt64BitStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ITensorInt64BitStatics2Impl: Sized {
    fn CreateFromShapeArrayAndDataArray(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], data: &[<i64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorInt64Bit>;
    fn CreateFromBuffer(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<TensorInt64Bit>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorInt64BitStatics2 {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorInt64BitStatics2";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ITensorInt64BitStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorInt64BitStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorInt64BitStatics2Vtbl {
        unsafe extern "system" fn CreateFromShapeArrayAndDataArray<Impl: ITensorInt64BitStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const i64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromShapeArrayAndDataArray(::core::slice::from_raw_parts(::core::mem::transmute_copy(&shape), shape_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&data), data_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromBuffer<Impl: ITensorInt64BitStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromBuffer(::core::slice::from_raw_parts(::core::mem::transmute_copy(&shape), shape_array_size as _), &*(&buffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorInt64BitStatics2, BASE_OFFSET>(),
            CreateFromShapeArrayAndDataArray: CreateFromShapeArrayAndDataArray::<Impl, IMPL_OFFSET>,
            CreateFromBuffer: CreateFromBuffer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorInt64BitStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITensorInt8BitImpl: Sized {
    fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u8>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorInt8Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorInt8Bit";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITensorInt8BitVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorInt8BitImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorInt8BitVtbl {
        unsafe extern "system" fn GetAsVectorView<Impl: ITensorInt8BitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAsVectorView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorInt8Bit, BASE_OFFSET>(), GetAsVectorView: GetAsVectorView::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorInt8Bit as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITensorInt8BitStaticsImpl: Sized {
    fn Create(&self) -> ::windows::core::Result<TensorInt8Bit>;
    fn Create2(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>) -> ::windows::core::Result<TensorInt8Bit>;
    fn CreateFromArray(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorInt8Bit>;
    fn CreateFromIterable(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &::core::option::Option<super::super::Foundation::Collections::IIterable<u8>>) -> ::windows::core::Result<TensorInt8Bit>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorInt8BitStatics {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorInt8BitStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITensorInt8BitStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorInt8BitStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorInt8BitStaticsVtbl {
        unsafe extern "system" fn Create<Impl: ITensorInt8BitStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create2<Impl: ITensorInt8BitStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create2(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromArray<Impl: ITensorInt8BitStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data_array_size: u32, data: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromArray(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&data), data_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromIterable<Impl: ITensorInt8BitStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromIterable(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType), &*(&data as *const <super::super::Foundation::Collections::IIterable<u8> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<u8> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorInt8BitStatics, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            Create2: Create2::<Impl, IMPL_OFFSET>,
            CreateFromArray: CreateFromArray::<Impl, IMPL_OFFSET>,
            CreateFromIterable: CreateFromIterable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorInt8BitStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ITensorInt8BitStatics2Impl: Sized {
    fn CreateFromShapeArrayAndDataArray(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], data: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorInt8Bit>;
    fn CreateFromBuffer(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<TensorInt8Bit>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorInt8BitStatics2 {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorInt8BitStatics2";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ITensorInt8BitStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorInt8BitStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorInt8BitStatics2Vtbl {
        unsafe extern "system" fn CreateFromShapeArrayAndDataArray<Impl: ITensorInt8BitStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromShapeArrayAndDataArray(::core::slice::from_raw_parts(::core::mem::transmute_copy(&shape), shape_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&data), data_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromBuffer<Impl: ITensorInt8BitStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromBuffer(::core::slice::from_raw_parts(::core::mem::transmute_copy(&shape), shape_array_size as _), &*(&buffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorInt8BitStatics2, BASE_OFFSET>(),
            CreateFromShapeArrayAndDataArray: CreateFromShapeArrayAndDataArray::<Impl, IMPL_OFFSET>,
            CreateFromBuffer: CreateFromBuffer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorInt8BitStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITensorStringImpl: Sized {
    fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorString {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorString";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITensorStringVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorStringImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorStringVtbl {
        unsafe extern "system" fn GetAsVectorView<Impl: ITensorStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAsVectorView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorString, BASE_OFFSET>(), GetAsVectorView: GetAsVectorView::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorString as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITensorStringStaticsImpl: Sized {
    fn Create(&self) -> ::windows::core::Result<TensorString>;
    fn Create2(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>) -> ::windows::core::Result<TensorString>;
    fn CreateFromArray(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &[<::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorString>;
    fn CreateFromIterable(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<TensorString>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorStringStatics {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorStringStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITensorStringStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorStringStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorStringStaticsVtbl {
        unsafe extern "system" fn Create<Impl: ITensorStringStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create2<Impl: ITensorStringStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create2(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromArray<Impl: ITensorStringStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data_array_size: u32, data: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromArray(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&data), data_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromIterable<Impl: ITensorStringStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromIterable(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType), &*(&data as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorStringStatics, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            Create2: Create2::<Impl, IMPL_OFFSET>,
            CreateFromArray: CreateFromArray::<Impl, IMPL_OFFSET>,
            CreateFromIterable: CreateFromIterable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorStringStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorStringStatics2Impl: Sized {
    fn CreateFromShapeArrayAndDataArray(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], data: &[<::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorString>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITensorStringStatics2 {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorStringStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl ITensorStringStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorStringStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorStringStatics2Vtbl {
        unsafe extern "system" fn CreateFromShapeArrayAndDataArray<Impl: ITensorStringStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromShapeArrayAndDataArray(::core::slice::from_raw_parts(::core::mem::transmute_copy(&shape), shape_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&data), data_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorStringStatics2, BASE_OFFSET>(),
            CreateFromShapeArrayAndDataArray: CreateFromShapeArrayAndDataArray::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorStringStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITensorUInt16BitImpl: Sized {
    fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u16>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorUInt16Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorUInt16Bit";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITensorUInt16BitVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorUInt16BitImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorUInt16BitVtbl {
        unsafe extern "system" fn GetAsVectorView<Impl: ITensorUInt16BitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAsVectorView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorUInt16Bit, BASE_OFFSET>(), GetAsVectorView: GetAsVectorView::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorUInt16Bit as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITensorUInt16BitStaticsImpl: Sized {
    fn Create(&self) -> ::windows::core::Result<TensorUInt16Bit>;
    fn Create2(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>) -> ::windows::core::Result<TensorUInt16Bit>;
    fn CreateFromArray(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &[<u16 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorUInt16Bit>;
    fn CreateFromIterable(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &::core::option::Option<super::super::Foundation::Collections::IIterable<u16>>) -> ::windows::core::Result<TensorUInt16Bit>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorUInt16BitStatics {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorUInt16BitStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITensorUInt16BitStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorUInt16BitStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorUInt16BitStaticsVtbl {
        unsafe extern "system" fn Create<Impl: ITensorUInt16BitStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create2<Impl: ITensorUInt16BitStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create2(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromArray<Impl: ITensorUInt16BitStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data_array_size: u32, data: *const u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromArray(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&data), data_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromIterable<Impl: ITensorUInt16BitStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromIterable(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType), &*(&data as *const <super::super::Foundation::Collections::IIterable<u16> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<u16> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorUInt16BitStatics, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            Create2: Create2::<Impl, IMPL_OFFSET>,
            CreateFromArray: CreateFromArray::<Impl, IMPL_OFFSET>,
            CreateFromIterable: CreateFromIterable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorUInt16BitStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ITensorUInt16BitStatics2Impl: Sized {
    fn CreateFromShapeArrayAndDataArray(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], data: &[<u16 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorUInt16Bit>;
    fn CreateFromBuffer(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<TensorUInt16Bit>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorUInt16BitStatics2 {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorUInt16BitStatics2";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ITensorUInt16BitStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorUInt16BitStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorUInt16BitStatics2Vtbl {
        unsafe extern "system" fn CreateFromShapeArrayAndDataArray<Impl: ITensorUInt16BitStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromShapeArrayAndDataArray(::core::slice::from_raw_parts(::core::mem::transmute_copy(&shape), shape_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&data), data_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromBuffer<Impl: ITensorUInt16BitStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromBuffer(::core::slice::from_raw_parts(::core::mem::transmute_copy(&shape), shape_array_size as _), &*(&buffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorUInt16BitStatics2, BASE_OFFSET>(),
            CreateFromShapeArrayAndDataArray: CreateFromShapeArrayAndDataArray::<Impl, IMPL_OFFSET>,
            CreateFromBuffer: CreateFromBuffer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorUInt16BitStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITensorUInt32BitImpl: Sized {
    fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorUInt32Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorUInt32Bit";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITensorUInt32BitVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorUInt32BitImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorUInt32BitVtbl {
        unsafe extern "system" fn GetAsVectorView<Impl: ITensorUInt32BitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAsVectorView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorUInt32Bit, BASE_OFFSET>(), GetAsVectorView: GetAsVectorView::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorUInt32Bit as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITensorUInt32BitStaticsImpl: Sized {
    fn Create(&self) -> ::windows::core::Result<TensorUInt32Bit>;
    fn Create2(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>) -> ::windows::core::Result<TensorUInt32Bit>;
    fn CreateFromArray(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &[<u32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorUInt32Bit>;
    fn CreateFromIterable(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &::core::option::Option<super::super::Foundation::Collections::IIterable<u32>>) -> ::windows::core::Result<TensorUInt32Bit>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorUInt32BitStatics {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorUInt32BitStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITensorUInt32BitStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorUInt32BitStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorUInt32BitStaticsVtbl {
        unsafe extern "system" fn Create<Impl: ITensorUInt32BitStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create2<Impl: ITensorUInt32BitStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create2(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromArray<Impl: ITensorUInt32BitStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data_array_size: u32, data: *const u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromArray(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&data), data_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromIterable<Impl: ITensorUInt32BitStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromIterable(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType), &*(&data as *const <super::super::Foundation::Collections::IIterable<u32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<u32> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorUInt32BitStatics, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            Create2: Create2::<Impl, IMPL_OFFSET>,
            CreateFromArray: CreateFromArray::<Impl, IMPL_OFFSET>,
            CreateFromIterable: CreateFromIterable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorUInt32BitStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ITensorUInt32BitStatics2Impl: Sized {
    fn CreateFromShapeArrayAndDataArray(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], data: &[<u32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorUInt32Bit>;
    fn CreateFromBuffer(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<TensorUInt32Bit>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorUInt32BitStatics2 {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorUInt32BitStatics2";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ITensorUInt32BitStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorUInt32BitStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorUInt32BitStatics2Vtbl {
        unsafe extern "system" fn CreateFromShapeArrayAndDataArray<Impl: ITensorUInt32BitStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromShapeArrayAndDataArray(::core::slice::from_raw_parts(::core::mem::transmute_copy(&shape), shape_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&data), data_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromBuffer<Impl: ITensorUInt32BitStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromBuffer(::core::slice::from_raw_parts(::core::mem::transmute_copy(&shape), shape_array_size as _), &*(&buffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorUInt32BitStatics2, BASE_OFFSET>(),
            CreateFromShapeArrayAndDataArray: CreateFromShapeArrayAndDataArray::<Impl, IMPL_OFFSET>,
            CreateFromBuffer: CreateFromBuffer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorUInt32BitStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITensorUInt64BitImpl: Sized {
    fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u64>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorUInt64Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorUInt64Bit";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITensorUInt64BitVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorUInt64BitImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorUInt64BitVtbl {
        unsafe extern "system" fn GetAsVectorView<Impl: ITensorUInt64BitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAsVectorView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorUInt64Bit, BASE_OFFSET>(), GetAsVectorView: GetAsVectorView::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorUInt64Bit as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITensorUInt64BitStaticsImpl: Sized {
    fn Create(&self) -> ::windows::core::Result<TensorUInt64Bit>;
    fn Create2(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>) -> ::windows::core::Result<TensorUInt64Bit>;
    fn CreateFromArray(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &[<u64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorUInt64Bit>;
    fn CreateFromIterable(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &::core::option::Option<super::super::Foundation::Collections::IIterable<u64>>) -> ::windows::core::Result<TensorUInt64Bit>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorUInt64BitStatics {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorUInt64BitStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITensorUInt64BitStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorUInt64BitStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorUInt64BitStaticsVtbl {
        unsafe extern "system" fn Create<Impl: ITensorUInt64BitStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create2<Impl: ITensorUInt64BitStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create2(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromArray<Impl: ITensorUInt64BitStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data_array_size: u32, data: *const u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromArray(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&data), data_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromIterable<Impl: ITensorUInt64BitStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromIterable(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType), &*(&data as *const <super::super::Foundation::Collections::IIterable<u64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<u64> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorUInt64BitStatics, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            Create2: Create2::<Impl, IMPL_OFFSET>,
            CreateFromArray: CreateFromArray::<Impl, IMPL_OFFSET>,
            CreateFromIterable: CreateFromIterable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorUInt64BitStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ITensorUInt64BitStatics2Impl: Sized {
    fn CreateFromShapeArrayAndDataArray(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], data: &[<u64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorUInt64Bit>;
    fn CreateFromBuffer(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<TensorUInt64Bit>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorUInt64BitStatics2 {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorUInt64BitStatics2";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ITensorUInt64BitStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorUInt64BitStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorUInt64BitStatics2Vtbl {
        unsafe extern "system" fn CreateFromShapeArrayAndDataArray<Impl: ITensorUInt64BitStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromShapeArrayAndDataArray(::core::slice::from_raw_parts(::core::mem::transmute_copy(&shape), shape_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&data), data_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromBuffer<Impl: ITensorUInt64BitStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromBuffer(::core::slice::from_raw_parts(::core::mem::transmute_copy(&shape), shape_array_size as _), &*(&buffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorUInt64BitStatics2, BASE_OFFSET>(),
            CreateFromShapeArrayAndDataArray: CreateFromShapeArrayAndDataArray::<Impl, IMPL_OFFSET>,
            CreateFromBuffer: CreateFromBuffer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorUInt64BitStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITensorUInt8BitImpl: Sized {
    fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u8>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorUInt8Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorUInt8Bit";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITensorUInt8BitVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorUInt8BitImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorUInt8BitVtbl {
        unsafe extern "system" fn GetAsVectorView<Impl: ITensorUInt8BitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAsVectorView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorUInt8Bit, BASE_OFFSET>(), GetAsVectorView: GetAsVectorView::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorUInt8Bit as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITensorUInt8BitStaticsImpl: Sized {
    fn Create(&self) -> ::windows::core::Result<TensorUInt8Bit>;
    fn Create2(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>) -> ::windows::core::Result<TensorUInt8Bit>;
    fn CreateFromArray(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorUInt8Bit>;
    fn CreateFromIterable(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &::core::option::Option<super::super::Foundation::Collections::IIterable<u8>>) -> ::windows::core::Result<TensorUInt8Bit>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorUInt8BitStatics {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorUInt8BitStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITensorUInt8BitStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorUInt8BitStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorUInt8BitStaticsVtbl {
        unsafe extern "system" fn Create<Impl: ITensorUInt8BitStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create2<Impl: ITensorUInt8BitStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create2(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromArray<Impl: ITensorUInt8BitStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data_array_size: u32, data: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromArray(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&data), data_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromIterable<Impl: ITensorUInt8BitStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromIterable(&*(&shape as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<i64> as ::windows::core::DefaultType>::DefaultType), &*(&data as *const <super::super::Foundation::Collections::IIterable<u8> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<u8> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorUInt8BitStatics, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            Create2: Create2::<Impl, IMPL_OFFSET>,
            CreateFromArray: CreateFromArray::<Impl, IMPL_OFFSET>,
            CreateFromIterable: CreateFromIterable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorUInt8BitStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ITensorUInt8BitStatics2Impl: Sized {
    fn CreateFromShapeArrayAndDataArray(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], data: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorUInt8Bit>;
    fn CreateFromBuffer(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<TensorUInt8Bit>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorUInt8BitStatics2 {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensorUInt8BitStatics2";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ITensorUInt8BitStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorUInt8BitStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorUInt8BitStatics2Vtbl {
        unsafe extern "system" fn CreateFromShapeArrayAndDataArray<Impl: ITensorUInt8BitStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromShapeArrayAndDataArray(::core::slice::from_raw_parts(::core::mem::transmute_copy(&shape), shape_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&data), data_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromBuffer<Impl: ITensorUInt8BitStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shape_array_size: u32, shape: *const i64, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromBuffer(::core::slice::from_raw_parts(::core::mem::transmute_copy(&shape), shape_array_size as _), &*(&buffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorUInt8BitStatics2, BASE_OFFSET>(),
            CreateFromShapeArrayAndDataArray: CreateFromShapeArrayAndDataArray::<Impl, IMPL_OFFSET>,
            CreateFromBuffer: CreateFromBuffer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorUInt8BitStatics2 as ::windows::core::Interface>::IID
    }
}
