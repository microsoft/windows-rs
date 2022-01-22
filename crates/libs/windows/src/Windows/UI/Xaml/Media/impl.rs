#[cfg(feature = "UI_Composition")]
pub trait IBrushOverrides2_Impl: Sized {
    fn PopulatePropertyInfoOverride(&mut self, propertyname: &::windows::core::HSTRING, animationpropertyinfo: &::core::option::Option<super::super::Composition::AnimationPropertyInfo>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
#[cfg(feature = "UI_Composition")]
impl ::windows::core::RuntimeName for IBrushOverrides2 {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IBrushOverrides2";
}
#[cfg(feature = "UI_Composition")]
impl IBrushOverrides2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBrushOverrides2_Impl, const OFFSET: isize>() -> IBrushOverrides2_Vtbl {
        unsafe extern "system" fn PopulatePropertyInfoOverride<Identity: ::windows::core::IUnknownImpl, Impl: IBrushOverrides2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, animationpropertyinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PopulatePropertyInfoOverride(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&animationpropertyinfo as *const <super::super::Composition::AnimationPropertyInfo as ::windows::core::Abi>::Abi as *const <super::super::Composition::AnimationPropertyInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBrushOverrides2, OFFSET>(),
            PopulatePropertyInfoOverride: PopulatePropertyInfoOverride::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBrushOverrides2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IGeneralTransformOverrides_Impl: Sized {
    fn InverseCore(&mut self) -> ::windows::core::Result<GeneralTransform>;
    fn TryTransformCore(&mut self, inpoint: &super::super::super::Foundation::Point, outpoint: &mut super::super::super::Foundation::Point) -> ::windows::core::Result<bool>;
    fn TransformBoundsCore(&mut self, rect: &super::super::super::Foundation::Rect) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IGeneralTransformOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IGeneralTransformOverrides";
}
#[cfg(feature = "Foundation")]
impl IGeneralTransformOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeneralTransformOverrides_Impl, const OFFSET: isize>() -> IGeneralTransformOverrides_Vtbl {
        unsafe extern "system" fn InverseCore<Identity: ::windows::core::IUnknownImpl, Impl: IGeneralTransformOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InverseCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryTransformCore<Identity: ::windows::core::IUnknownImpl, Impl: IGeneralTransformOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inpoint: super::super::super::Foundation::Point, outpoint: *mut super::super::super::Foundation::Point, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TryTransformCore(&*(&inpoint as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&outpoint)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransformBoundsCore<Identity: ::windows::core::IUnknownImpl, Impl: IGeneralTransformOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rect: super::super::super::Foundation::Rect, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TransformBoundsCore(&*(&rect as *const <super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGeneralTransformOverrides, OFFSET>(),
            InverseCore: InverseCore::<Identity, Impl, OFFSET>,
            TryTransformCore: TryTransformCore::<Identity, Impl, OFFSET>,
            TransformBoundsCore: TransformBoundsCore::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeneralTransformOverrides as ::windows::core::Interface>::IID
    }
}
pub trait IXamlCompositionBrushBaseOverrides_Impl: Sized {
    fn OnConnected(&mut self) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnDisconnected(&mut self) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
impl ::windows::core::RuntimeName for IXamlCompositionBrushBaseOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IXamlCompositionBrushBaseOverrides";
}
impl IXamlCompositionBrushBaseOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlCompositionBrushBaseOverrides_Impl, const OFFSET: isize>() -> IXamlCompositionBrushBaseOverrides_Vtbl {
        unsafe extern "system" fn OnConnected<Identity: ::windows::core::IUnknownImpl, Impl: IXamlCompositionBrushBaseOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnConnected().into()
        }
        unsafe extern "system" fn OnDisconnected<Identity: ::windows::core::IUnknownImpl, Impl: IXamlCompositionBrushBaseOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnDisconnected().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlCompositionBrushBaseOverrides, OFFSET>(),
            OnConnected: OnConnected::<Identity, Impl, OFFSET>,
            OnDisconnected: OnDisconnected::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlCompositionBrushBaseOverrides as ::windows::core::Interface>::IID
    }
}
pub trait IXamlLightOverrides_Impl: Sized {
    fn GetId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OnConnected(&mut self, newelement: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnDisconnected(&mut self, oldelement: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
impl ::windows::core::RuntimeName for IXamlLightOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IXamlLightOverrides";
}
impl IXamlLightOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlLightOverrides_Impl, const OFFSET: isize>() -> IXamlLightOverrides_Vtbl {
        unsafe extern "system" fn GetId<Identity: ::windows::core::IUnknownImpl, Impl: IXamlLightOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnConnected<Identity: ::windows::core::IUnknownImpl, Impl: IXamlLightOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newelement: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnConnected(&*(&newelement as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnDisconnected<Identity: ::windows::core::IUnknownImpl, Impl: IXamlLightOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldelement: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnDisconnected(&*(&oldelement as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlLightOverrides, OFFSET>(),
            GetId: GetId::<Identity, Impl, OFFSET>,
            OnConnected: OnConnected::<Identity, Impl, OFFSET>,
            OnDisconnected: OnDisconnected::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlLightOverrides as ::windows::core::Interface>::IID
    }
}
