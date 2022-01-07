#[cfg(feature = "implement_exclusive")]
pub trait ICharacterGroupingImpl: Sized {
    fn First(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICharacterGrouping {
    const NAME: &'static str = "Windows.Globalization.Collation.ICharacterGrouping";
}
#[cfg(feature = "implement_exclusive")]
impl ICharacterGroupingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICharacterGroupingImpl, const OFFSET: isize>() -> ICharacterGroupingVtbl {
        unsafe extern "system" fn First<Impl: ICharacterGroupingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).First() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Label<Impl: ICharacterGroupingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICharacterGrouping>, ::windows::core::GetTrustLevel, First::<Impl, OFFSET>, Label::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICharacterGroupingsImpl: Sized + IIterableImpl<CharacterGrouping> + IVectorViewImpl<CharacterGrouping> {
    fn Lookup(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICharacterGroupings {
    const NAME: &'static str = "Windows.Globalization.Collation.ICharacterGroupings";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICharacterGroupingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICharacterGroupingsImpl, const OFFSET: isize>() -> ICharacterGroupingsVtbl {
        unsafe extern "system" fn Lookup<Impl: ICharacterGroupingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Lookup(&*(&text as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICharacterGroupings>, ::windows::core::GetTrustLevel, Lookup::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICharacterGroupingsFactoryImpl: Sized {
    fn Create(&self, language: &::windows::core::HSTRING) -> ::windows::core::Result<CharacterGroupings>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICharacterGroupingsFactory {
    const NAME: &'static str = "Windows.Globalization.Collation.ICharacterGroupingsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICharacterGroupingsFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICharacterGroupingsFactoryImpl, const OFFSET: isize>() -> ICharacterGroupingsFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ICharacterGroupingsFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&language as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICharacterGroupingsFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
