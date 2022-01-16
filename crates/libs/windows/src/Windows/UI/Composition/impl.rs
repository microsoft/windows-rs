pub trait IAnimationObject_Impl: Sized {
    fn PopulatePropertyInfo(&mut self, propertyname: &::windows::core::HSTRING, propertyinfo: &::core::option::Option<AnimationPropertyInfo>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAnimationObject {
    const NAME: &'static str = "Windows.UI.Composition.IAnimationObject";
}
impl IAnimationObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAnimationObject_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAnimationObject_Vtbl {
        unsafe extern "system" fn PopulatePropertyInfo<Impl: IAnimationObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertyinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PopulatePropertyInfo(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&propertyinfo as *const <AnimationPropertyInfo as ::windows::core::Abi>::Abi as *const <AnimationPropertyInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAnimationObject, BASE_OFFSET>(),
            PopulatePropertyInfo: PopulatePropertyInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAnimationObject as ::windows::core::Interface>::IID
    }
}
pub trait ICompositionAnimationBase_Impl: Sized {}
impl ::windows::core::RuntimeName for ICompositionAnimationBase {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionAnimationBase";
}
impl ICompositionAnimationBase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionAnimationBase_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionAnimationBase_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionAnimationBase, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionAnimationBase as ::windows::core::Interface>::IID
    }
}
pub trait ICompositionSupportsSystemBackdrop_Impl: Sized {
    fn SystemBackdrop(&mut self) -> ::windows::core::Result<CompositionBrush>;
    fn SetSystemBackdrop(&mut self, value: &::core::option::Option<CompositionBrush>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICompositionSupportsSystemBackdrop {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionSupportsSystemBackdrop";
}
impl ICompositionSupportsSystemBackdrop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionSupportsSystemBackdrop_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionSupportsSystemBackdrop_Vtbl {
        unsafe extern "system" fn SystemBackdrop<Impl: ICompositionSupportsSystemBackdrop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemBackdrop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSystemBackdrop<Impl: ICompositionSupportsSystemBackdrop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSystemBackdrop(&*(&value as *const <CompositionBrush as ::windows::core::Abi>::Abi as *const <CompositionBrush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionSupportsSystemBackdrop, BASE_OFFSET>(),
            SystemBackdrop: SystemBackdrop::<Impl, IMPL_OFFSET>,
            SetSystemBackdrop: SetSystemBackdrop::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionSupportsSystemBackdrop as ::windows::core::Interface>::IID
    }
}
pub trait ICompositionSurface_Impl: Sized {}
impl ::windows::core::RuntimeName for ICompositionSurface {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionSurface";
}
impl ICompositionSurface_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionSurface_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionSurface_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionSurface, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionSurface as ::windows::core::Interface>::IID
    }
}
pub trait ICompositionSurfaceFacade_Impl: Sized {
    fn GetRealSurface(&mut self) -> ::windows::core::Result<ICompositionSurface>;
}
impl ::windows::core::RuntimeName for ICompositionSurfaceFacade {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionSurfaceFacade";
}
impl ICompositionSurfaceFacade_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionSurfaceFacade_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionSurfaceFacade_Vtbl {
        unsafe extern "system" fn GetRealSurface<Impl: ICompositionSurfaceFacade_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRealSurface() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionSurfaceFacade, BASE_OFFSET>(),
            GetRealSurface: GetRealSurface::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionSurfaceFacade as ::windows::core::Interface>::IID
    }
}
pub trait IVisualElement_Impl: Sized {}
impl ::windows::core::RuntimeName for IVisualElement {
    const NAME: &'static str = "Windows.UI.Composition.IVisualElement";
}
impl IVisualElement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualElement_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualElement_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualElement, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualElement as ::windows::core::Interface>::IID
    }
}
pub trait IVisualElement2_Impl: Sized {
    fn GetVisualInternal(&mut self) -> ::windows::core::Result<Visual>;
}
impl ::windows::core::RuntimeName for IVisualElement2 {
    const NAME: &'static str = "Windows.UI.Composition.IVisualElement2";
}
impl IVisualElement2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualElement2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualElement2_Vtbl {
        unsafe extern "system" fn GetVisualInternal<Impl: IVisualElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVisualInternal() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualElement2, BASE_OFFSET>(),
            GetVisualInternal: GetVisualInternal::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualElement2 as ::windows::core::Interface>::IID
    }
}
