pub trait IGraphicsEffectImpl: Sized + IGraphicsEffectSourceImpl {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IGraphicsEffect {
    const NAME: &'static str = "Windows.Graphics.Effects.IGraphicsEffect";
}
impl IGraphicsEffectVtbl {
    pub const fn new<Impl: IGraphicsEffectImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGraphicsEffectVtbl {
        unsafe extern "system" fn Name<Impl: IGraphicsEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IGraphicsEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetName(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGraphicsEffect>, base.5, Name::<Impl, OFFSET>, SetName::<Impl, OFFSET>)
    }
}
pub trait IGraphicsEffectSourceImpl: Sized {}
impl ::windows::core::RuntimeName for IGraphicsEffectSource {
    const NAME: &'static str = "Windows.Graphics.Effects.IGraphicsEffectSource";
}
impl IGraphicsEffectSourceVtbl {
    pub const fn new<Impl: IGraphicsEffectSourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGraphicsEffectSourceVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGraphicsEffectSource>, base.5)
    }
}
