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
    pub const fn new<Impl: ICharacterGroupingImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICharacterGroupingVtbl {
        unsafe extern "system" fn First<Impl: ICharacterGroupingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).First() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Label<Impl: ICharacterGroupingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICharacterGrouping>, base.5, First::<Impl, OFFSET>, Label::<Impl, OFFSET>)
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
    pub const fn new<Impl: ICharacterGroupingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICharacterGroupingsVtbl {
        unsafe extern "system" fn Lookup<Impl: ICharacterGroupingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Lookup(&*(&text as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICharacterGroupings>, base.5, Lookup::<Impl, OFFSET>)
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
    pub const fn new<Impl: ICharacterGroupingsFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICharacterGroupingsFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ICharacterGroupingsFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, language: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&language as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICharacterGroupingsFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
