pub trait IGraphicsEffect_Impl: Sized + IGraphicsEffectSource_Impl {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IGraphicsEffect {
    const NAME: &'static str = "Windows.Graphics.Effects.IGraphicsEffect";
}
impl IGraphicsEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGraphicsEffect_Impl, const OFFSET: isize>() -> IGraphicsEffect_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IGraphicsEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl, Impl: IGraphicsEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetName(::core::mem::transmute(&name)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGraphicsEffect, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGraphicsEffect as ::windows::core::Interface>::IID
    }
}
pub trait IGraphicsEffectSource_Impl: Sized {}
impl ::windows::core::RuntimeName for IGraphicsEffectSource {
    const NAME: &'static str = "Windows.Graphics.Effects.IGraphicsEffectSource";
}
impl IGraphicsEffectSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGraphicsEffectSource_Impl, const OFFSET: isize>() -> IGraphicsEffectSource_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGraphicsEffectSource, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGraphicsEffectSource as ::windows::core::Interface>::IID
    }
}
