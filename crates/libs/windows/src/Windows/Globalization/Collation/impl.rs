#[cfg(feature = "implement_exclusive")]
pub trait ICharacterGroupingImpl: Sized {
    fn First(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Label(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICharacterGrouping {
    const NAME: &'static str = "Windows.Globalization.Collation.ICharacterGrouping";
}
#[cfg(feature = "implement_exclusive")]
impl ICharacterGroupingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICharacterGroupingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICharacterGroupingVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICharacterGrouping, BASE_OFFSET>(),
            First: First::<Impl, IMPL_OFFSET>,
            Label: Label::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICharacterGrouping as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICharacterGroupingsImpl: Sized + IIterableImpl<CharacterGrouping> + IVectorViewImpl<CharacterGrouping> {
    fn Lookup(&mut self, text: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICharacterGroupings {
    const NAME: &'static str = "Windows.Globalization.Collation.ICharacterGroupings";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICharacterGroupingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICharacterGroupingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICharacterGroupingsVtbl {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICharacterGroupings, BASE_OFFSET>(), Lookup: Lookup::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICharacterGroupings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICharacterGroupingsFactoryImpl: Sized {
    fn Create(&mut self, language: &::windows::core::HSTRING) -> ::windows::core::Result<CharacterGroupings>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICharacterGroupingsFactory {
    const NAME: &'static str = "Windows.Globalization.Collation.ICharacterGroupingsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICharacterGroupingsFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICharacterGroupingsFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICharacterGroupingsFactoryVtbl {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICharacterGroupingsFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICharacterGroupingsFactory as ::windows::core::Interface>::IID
    }
}
