pub trait IGraphicsEffectImpl: Sized + IGraphicsEffectSourceImpl {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IGraphicsEffect {
    const NAME: &'static str = "Windows.Graphics.Effects.IGraphicsEffect";
}
impl IGraphicsEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGraphicsEffectImpl, const OFFSET: isize>() -> IGraphicsEffectVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGraphicsEffect>, ::windows::core::GetTrustLevel, Name::<Impl, OFFSET>, SetName::<Impl, OFFSET>)
    }
}
pub trait IGraphicsEffectSourceImpl: Sized {}
impl ::windows::core::RuntimeName for IGraphicsEffectSource {
    const NAME: &'static str = "Windows.Graphics.Effects.IGraphicsEffectSource";
}
impl IGraphicsEffectSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGraphicsEffectSourceImpl, const OFFSET: isize>() -> IGraphicsEffectSourceVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGraphicsEffectSource>, ::windows::core::GetTrustLevel)
    }
}
