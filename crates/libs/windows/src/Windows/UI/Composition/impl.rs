pub trait IAnimationObject_Impl: Sized {
    fn PopulatePropertyInfo(&self, propertyname: &::windows::core::HSTRING, propertyinfo: &::core::option::Option<AnimationPropertyInfo>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAnimationObject {
    const NAME: &'static str = "Windows.UI.Composition.IAnimationObject";
}
impl IAnimationObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAnimationObject_Impl, const OFFSET: isize>() -> IAnimationObject_Vtbl {
        unsafe extern "system" fn PopulatePropertyInfo<Identity: ::windows::core::IUnknownImpl, Impl: IAnimationObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertyinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PopulatePropertyInfo(::core::mem::transmute(&propertyname), ::core::mem::transmute(&propertyinfo)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAnimationObject, OFFSET>(),
            PopulatePropertyInfo: PopulatePropertyInfo::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionAnimationBase_Impl, const OFFSET: isize>() -> ICompositionAnimationBase_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionAnimationBase, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionAnimationBase as ::windows::core::Interface>::IID
    }
}
pub trait ICompositionSupportsSystemBackdrop_Impl: Sized {
    fn SystemBackdrop(&self) -> ::windows::core::Result<CompositionBrush>;
    fn SetSystemBackdrop(&self, value: &::core::option::Option<CompositionBrush>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICompositionSupportsSystemBackdrop {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionSupportsSystemBackdrop";
}
impl ICompositionSupportsSystemBackdrop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionSupportsSystemBackdrop_Impl, const OFFSET: isize>() -> ICompositionSupportsSystemBackdrop_Vtbl {
        unsafe extern "system" fn SystemBackdrop<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionSupportsSystemBackdrop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SystemBackdrop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSystemBackdrop<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionSupportsSystemBackdrop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSystemBackdrop(::core::mem::transmute(&value)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionSupportsSystemBackdrop, OFFSET>(),
            SystemBackdrop: SystemBackdrop::<Identity, Impl, OFFSET>,
            SetSystemBackdrop: SetSystemBackdrop::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionSurface_Impl, const OFFSET: isize>() -> ICompositionSurface_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionSurface, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionSurface as ::windows::core::Interface>::IID
    }
}
pub trait ICompositionSurfaceFacade_Impl: Sized {
    fn GetRealSurface(&self) -> ::windows::core::Result<ICompositionSurface>;
}
impl ::windows::core::RuntimeName for ICompositionSurfaceFacade {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionSurfaceFacade";
}
impl ICompositionSurfaceFacade_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionSurfaceFacade_Impl, const OFFSET: isize>() -> ICompositionSurfaceFacade_Vtbl {
        unsafe extern "system" fn GetRealSurface<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionSurfaceFacade_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionSurfaceFacade, OFFSET>(),
            GetRealSurface: GetRealSurface::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualElement_Impl, const OFFSET: isize>() -> IVisualElement_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualElement, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualElement as ::windows::core::Interface>::IID
    }
}
pub trait IVisualElement2_Impl: Sized {
    fn GetVisualInternal(&self) -> ::windows::core::Result<Visual>;
}
impl ::windows::core::RuntimeName for IVisualElement2 {
    const NAME: &'static str = "Windows.UI.Composition.IVisualElement2";
}
impl IVisualElement2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualElement2_Impl, const OFFSET: isize>() -> IVisualElement2_Vtbl {
        unsafe extern "system" fn GetVisualInternal<Identity: ::windows::core::IUnknownImpl, Impl: IVisualElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualElement2, OFFSET>(),
            GetVisualInternal: GetVisualInternal::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualElement2 as ::windows::core::Interface>::IID
    }
}
