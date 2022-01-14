#[cfg(feature = "implement_exclusive")]
pub trait ICharacterGrouping_Impl: Sized {
    fn First(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Label(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICharacterGrouping {
    const NAME: &'static str = "Windows.Globalization.Collation.ICharacterGrouping";
}
#[cfg(feature = "implement_exclusive")]
impl ICharacterGrouping_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICharacterGrouping_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICharacterGrouping_Vtbl {
        unsafe extern "system" fn First<Impl: ICharacterGrouping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Label<Impl: ICharacterGrouping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait ICharacterGroupings_Impl: Sized + super::super::Foundation::Collections::IIterable_Impl<CharacterGrouping> + super::super::Foundation::Collections::IVectorView_Impl<CharacterGrouping> {
    fn Lookup(&mut self, text: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICharacterGroupings {
    const NAME: &'static str = "Windows.Globalization.Collation.ICharacterGroupings";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICharacterGroupings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICharacterGroupings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICharacterGroupings_Vtbl {
        unsafe extern "system" fn Lookup<Impl: ICharacterGroupings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait ICharacterGroupingsFactory_Impl: Sized {
    fn Create(&mut self, language: &::windows::core::HSTRING) -> ::windows::core::Result<CharacterGroupings>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICharacterGroupingsFactory {
    const NAME: &'static str = "Windows.Globalization.Collation.ICharacterGroupingsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICharacterGroupingsFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICharacterGroupingsFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICharacterGroupingsFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: ICharacterGroupingsFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
