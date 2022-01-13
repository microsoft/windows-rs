pub trait IGraphicsEffectImpl: Sized + IGraphicsEffectSourceImpl {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IGraphicsEffect {
    const NAME: &'static str = "Windows.Graphics.Effects.IGraphicsEffect";
}
impl IGraphicsEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGraphicsEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGraphicsEffectVtbl {
        unsafe extern "system" fn Name<Impl: IGraphicsEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetName<Impl: IGraphicsEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGraphicsEffect, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGraphicsEffect as ::windows::core::Interface>::IID
    }
}
pub trait IGraphicsEffectSourceImpl: Sized {}
impl ::windows::core::RuntimeName for IGraphicsEffectSource {
    const NAME: &'static str = "Windows.Graphics.Effects.IGraphicsEffectSource";
}
impl IGraphicsEffectSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGraphicsEffectSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGraphicsEffectSourceVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGraphicsEffectSource, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGraphicsEffectSource as ::windows::core::Interface>::IID
    }
}
