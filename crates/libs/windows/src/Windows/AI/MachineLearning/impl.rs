pub trait ILearningModelFeatureDescriptor_Impl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind>;
    fn IsRequired(&self) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for ILearningModelFeatureDescriptor {
    const NAME: &'static str = "Windows.AI.MachineLearning.ILearningModelFeatureDescriptor";
}
impl ILearningModelFeatureDescriptor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILearningModelFeatureDescriptor_Impl, const OFFSET: isize>() -> ILearningModelFeatureDescriptor_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILearningModelFeatureDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILearningModelFeatureDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Kind<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILearningModelFeatureDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LearningModelFeatureKind) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Kind() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRequired<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILearningModelFeatureDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsRequired() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, ILearningModelFeatureDescriptor, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            Kind: Kind::<Identity, Impl, OFFSET>,
            IsRequired: IsRequired::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILearningModelFeatureDescriptor as ::windows::core::Interface>::IID
    }
}
pub trait ILearningModelFeatureValue_Impl: Sized {
    fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind>;
}
impl ::windows::core::RuntimeName for ILearningModelFeatureValue {
    const NAME: &'static str = "Windows.AI.MachineLearning.ILearningModelFeatureValue";
}
impl ILearningModelFeatureValue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILearningModelFeatureValue_Impl, const OFFSET: isize>() -> ILearningModelFeatureValue_Vtbl {
        unsafe extern "system" fn Kind<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILearningModelFeatureValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LearningModelFeatureKind) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Kind() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IInspectableVtbl::new::<Identity, ILearningModelFeatureValue, OFFSET>(), Kind: Kind::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILearningModelFeatureValue as ::windows::core::Interface>::IID
    }
}
pub trait ILearningModelOperatorProvider_Impl: Sized {}
impl ::windows::core::RuntimeName for ILearningModelOperatorProvider {
    const NAME: &'static str = "Windows.AI.MachineLearning.ILearningModelOperatorProvider";
}
impl ILearningModelOperatorProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILearningModelOperatorProvider_Impl, const OFFSET: isize>() -> ILearningModelOperatorProvider_Vtbl {
        Self { base__: ::windows::core::IInspectableVtbl::new::<Identity, ILearningModelOperatorProvider, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILearningModelOperatorProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait ITensor_Impl: Sized + ILearningModelFeatureValue_Impl {
    fn TensorKind(&self) -> ::windows::core::Result<TensorKind>;
    fn Shape(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for ITensor {
    const NAME: &'static str = "Windows.AI.MachineLearning.ITensor";
}
#[cfg(feature = "Foundation_Collections")]
impl ITensor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITensor_Impl, const OFFSET: isize>() -> ITensor_Vtbl {
        unsafe extern "system" fn TensorKind<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TensorKind) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TensorKind() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shape<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Shape() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, ITensor, OFFSET>(),
            TensorKind: TensorKind::<Identity, Impl, OFFSET>,
            Shape: Shape::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensor as ::windows::core::Interface>::IID
    }
}
