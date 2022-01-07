#[cfg(feature = "implement_exclusive")]
pub trait IPrintBindingOptionDetailsImpl: Sized {
    fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintBindingOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintBindingOptionDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintBindingOptionDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintBindingOptionDetailsImpl, const OFFSET: isize>() -> IPrintBindingOptionDetailsVtbl {
        unsafe extern "system" fn SetWarningText<Impl: IPrintBindingOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWarningText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WarningText<Impl: IPrintBindingOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WarningText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IPrintBindingOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Description<Impl: IPrintBindingOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintBindingOptionDetails>, ::windows::core::GetTrustLevel, SetWarningText::<Impl, OFFSET>, WarningText::<Impl, OFFSET>, SetDescription::<Impl, OFFSET>, Description::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintBorderingOptionDetailsImpl: Sized {
    fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintBorderingOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintBorderingOptionDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintBorderingOptionDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintBorderingOptionDetailsImpl, const OFFSET: isize>() -> IPrintBorderingOptionDetailsVtbl {
        unsafe extern "system" fn SetWarningText<Impl: IPrintBorderingOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWarningText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WarningText<Impl: IPrintBorderingOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WarningText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IPrintBorderingOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Description<Impl: IPrintBorderingOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintBorderingOptionDetails>, ::windows::core::GetTrustLevel, SetWarningText::<Impl, OFFSET>, WarningText::<Impl, OFFSET>, SetDescription::<Impl, OFFSET>, Description::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintCollationOptionDetailsImpl: Sized {
    fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintCollationOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintCollationOptionDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintCollationOptionDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCollationOptionDetailsImpl, const OFFSET: isize>() -> IPrintCollationOptionDetailsVtbl {
        unsafe extern "system" fn SetWarningText<Impl: IPrintCollationOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWarningText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WarningText<Impl: IPrintCollationOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WarningText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IPrintCollationOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Description<Impl: IPrintCollationOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintCollationOptionDetails>, ::windows::core::GetTrustLevel, SetWarningText::<Impl, OFFSET>, WarningText::<Impl, OFFSET>, SetDescription::<Impl, OFFSET>, Description::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintColorModeOptionDetailsImpl: Sized {
    fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintColorModeOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintColorModeOptionDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintColorModeOptionDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintColorModeOptionDetailsImpl, const OFFSET: isize>() -> IPrintColorModeOptionDetailsVtbl {
        unsafe extern "system" fn SetWarningText<Impl: IPrintColorModeOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWarningText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WarningText<Impl: IPrintColorModeOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WarningText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IPrintColorModeOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Description<Impl: IPrintColorModeOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintColorModeOptionDetails>, ::windows::core::GetTrustLevel, SetWarningText::<Impl, OFFSET>, WarningText::<Impl, OFFSET>, SetDescription::<Impl, OFFSET>, Description::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintCopiesOptionDetailsImpl: Sized {
    fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintCopiesOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintCopiesOptionDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintCopiesOptionDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCopiesOptionDetailsImpl, const OFFSET: isize>() -> IPrintCopiesOptionDetailsVtbl {
        unsafe extern "system" fn SetWarningText<Impl: IPrintCopiesOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWarningText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WarningText<Impl: IPrintCopiesOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WarningText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IPrintCopiesOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Description<Impl: IPrintCopiesOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintCopiesOptionDetails>, ::windows::core::GetTrustLevel, SetWarningText::<Impl, OFFSET>, WarningText::<Impl, OFFSET>, SetDescription::<Impl, OFFSET>, Description::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintCustomItemDetailsImpl: Sized {
    fn ItemId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetItemDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ItemDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintCustomItemDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintCustomItemDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintCustomItemDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCustomItemDetailsImpl, const OFFSET: isize>() -> IPrintCustomItemDetailsVtbl {
        unsafe extern "system" fn ItemId<Impl: IPrintCustomItemDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItemDisplayName<Impl: IPrintCustomItemDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetItemDisplayName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ItemDisplayName<Impl: IPrintCustomItemDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintCustomItemDetails>, ::windows::core::GetTrustLevel, ItemId::<Impl, OFFSET>, SetItemDisplayName::<Impl, OFFSET>, ItemDisplayName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintCustomItemListOptionDetailsImpl: Sized + IPrintCustomOptionDetailsImpl + IPrintItemListOptionDetailsImpl + IPrintOptionDetailsImpl {
    fn AddItem(&self, itemid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintCustomItemListOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintCustomItemListOptionDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintCustomItemListOptionDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCustomItemListOptionDetailsImpl, const OFFSET: isize>() -> IPrintCustomItemListOptionDetailsVtbl {
        unsafe extern "system" fn AddItem<Impl: IPrintCustomItemListOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddItem(&*(&itemid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&displayname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintCustomItemListOptionDetails>, ::windows::core::GetTrustLevel, AddItem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintCustomItemListOptionDetails2Impl: Sized {
    fn AddItem(&self, itemid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING, description: &::windows::core::HSTRING, icon: &::core::option::Option<super::super::super::Storage::Streams::IRandomAccessStreamWithContentType>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintCustomItemListOptionDetails2 {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintCustomItemListOptionDetails2";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintCustomItemListOptionDetails2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCustomItemListOptionDetails2Impl, const OFFSET: isize>() -> IPrintCustomItemListOptionDetails2Vtbl {
        unsafe extern "system" fn AddItem<Impl: IPrintCustomItemListOptionDetails2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, description: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, icon: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .AddItem(
                    &*(&itemid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&displayname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&description as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&icon as *const <super::super::super::Storage::Streams::IRandomAccessStreamWithContentType as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IRandomAccessStreamWithContentType as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintCustomItemListOptionDetails2>, ::windows::core::GetTrustLevel, AddItem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintCustomItemListOptionDetails3Impl: Sized {
    fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintCustomItemListOptionDetails3 {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintCustomItemListOptionDetails3";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintCustomItemListOptionDetails3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCustomItemListOptionDetails3Impl, const OFFSET: isize>() -> IPrintCustomItemListOptionDetails3Vtbl {
        unsafe extern "system" fn SetWarningText<Impl: IPrintCustomItemListOptionDetails3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWarningText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WarningText<Impl: IPrintCustomItemListOptionDetails3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WarningText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IPrintCustomItemListOptionDetails3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Description<Impl: IPrintCustomItemListOptionDetails3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintCustomItemListOptionDetails3>, ::windows::core::GetTrustLevel, SetWarningText::<Impl, OFFSET>, WarningText::<Impl, OFFSET>, SetDescription::<Impl, OFFSET>, Description::<Impl, OFFSET>)
    }
}
pub trait IPrintCustomOptionDetailsImpl: Sized + IPrintOptionDetailsImpl {
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IPrintCustomOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintCustomOptionDetails";
}
impl IPrintCustomOptionDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCustomOptionDetailsImpl, const OFFSET: isize>() -> IPrintCustomOptionDetailsVtbl {
        unsafe extern "system" fn SetDisplayName<Impl: IPrintCustomOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisplayName<Impl: IPrintCustomOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintCustomOptionDetails>, ::windows::core::GetTrustLevel, SetDisplayName::<Impl, OFFSET>, DisplayName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintCustomTextOptionDetailsImpl: Sized + IPrintCustomOptionDetailsImpl + IPrintOptionDetailsImpl {
    fn SetMaxCharacters(&self, value: u32) -> ::windows::core::Result<()>;
    fn MaxCharacters(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintCustomTextOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintCustomTextOptionDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintCustomTextOptionDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCustomTextOptionDetailsImpl, const OFFSET: isize>() -> IPrintCustomTextOptionDetailsVtbl {
        unsafe extern "system" fn SetMaxCharacters<Impl: IPrintCustomTextOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxCharacters(value).into()
        }
        unsafe extern "system" fn MaxCharacters<Impl: IPrintCustomTextOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxCharacters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintCustomTextOptionDetails>, ::windows::core::GetTrustLevel, SetMaxCharacters::<Impl, OFFSET>, MaxCharacters::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintCustomTextOptionDetails2Impl: Sized {
    fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintCustomTextOptionDetails2 {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintCustomTextOptionDetails2";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintCustomTextOptionDetails2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCustomTextOptionDetails2Impl, const OFFSET: isize>() -> IPrintCustomTextOptionDetails2Vtbl {
        unsafe extern "system" fn SetWarningText<Impl: IPrintCustomTextOptionDetails2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWarningText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WarningText<Impl: IPrintCustomTextOptionDetails2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WarningText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IPrintCustomTextOptionDetails2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Description<Impl: IPrintCustomTextOptionDetails2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintCustomTextOptionDetails2>, ::windows::core::GetTrustLevel, SetWarningText::<Impl, OFFSET>, WarningText::<Impl, OFFSET>, SetDescription::<Impl, OFFSET>, Description::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintCustomToggleOptionDetailsImpl: Sized {
    fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintCustomToggleOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintCustomToggleOptionDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintCustomToggleOptionDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintCustomToggleOptionDetailsImpl, const OFFSET: isize>() -> IPrintCustomToggleOptionDetailsVtbl {
        unsafe extern "system" fn SetWarningText<Impl: IPrintCustomToggleOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWarningText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WarningText<Impl: IPrintCustomToggleOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WarningText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IPrintCustomToggleOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Description<Impl: IPrintCustomToggleOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintCustomToggleOptionDetails>, ::windows::core::GetTrustLevel, SetWarningText::<Impl, OFFSET>, WarningText::<Impl, OFFSET>, SetDescription::<Impl, OFFSET>, Description::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintDuplexOptionDetailsImpl: Sized {
    fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintDuplexOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintDuplexOptionDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintDuplexOptionDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintDuplexOptionDetailsImpl, const OFFSET: isize>() -> IPrintDuplexOptionDetailsVtbl {
        unsafe extern "system" fn SetWarningText<Impl: IPrintDuplexOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWarningText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WarningText<Impl: IPrintDuplexOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WarningText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IPrintDuplexOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Description<Impl: IPrintDuplexOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintDuplexOptionDetails>, ::windows::core::GetTrustLevel, SetWarningText::<Impl, OFFSET>, WarningText::<Impl, OFFSET>, SetDescription::<Impl, OFFSET>, Description::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintHolePunchOptionDetailsImpl: Sized {
    fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintHolePunchOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintHolePunchOptionDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintHolePunchOptionDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintHolePunchOptionDetailsImpl, const OFFSET: isize>() -> IPrintHolePunchOptionDetailsVtbl {
        unsafe extern "system" fn SetWarningText<Impl: IPrintHolePunchOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWarningText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WarningText<Impl: IPrintHolePunchOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WarningText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IPrintHolePunchOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Description<Impl: IPrintHolePunchOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintHolePunchOptionDetails>, ::windows::core::GetTrustLevel, SetWarningText::<Impl, OFFSET>, WarningText::<Impl, OFFSET>, SetDescription::<Impl, OFFSET>, Description::<Impl, OFFSET>)
    }
}
pub trait IPrintItemListOptionDetailsImpl: Sized + IPrintOptionDetailsImpl {
    fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>;
}
impl ::windows::core::RuntimeName for IPrintItemListOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintItemListOptionDetails";
}
impl IPrintItemListOptionDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintItemListOptionDetailsImpl, const OFFSET: isize>() -> IPrintItemListOptionDetailsVtbl {
        unsafe extern "system" fn Items<Impl: IPrintItemListOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Items() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintItemListOptionDetails>, ::windows::core::GetTrustLevel, Items::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintMediaSizeOptionDetailsImpl: Sized {
    fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintMediaSizeOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintMediaSizeOptionDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintMediaSizeOptionDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintMediaSizeOptionDetailsImpl, const OFFSET: isize>() -> IPrintMediaSizeOptionDetailsVtbl {
        unsafe extern "system" fn SetWarningText<Impl: IPrintMediaSizeOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWarningText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WarningText<Impl: IPrintMediaSizeOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WarningText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IPrintMediaSizeOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Description<Impl: IPrintMediaSizeOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintMediaSizeOptionDetails>, ::windows::core::GetTrustLevel, SetWarningText::<Impl, OFFSET>, WarningText::<Impl, OFFSET>, SetDescription::<Impl, OFFSET>, Description::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintMediaTypeOptionDetailsImpl: Sized {
    fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintMediaTypeOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintMediaTypeOptionDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintMediaTypeOptionDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintMediaTypeOptionDetailsImpl, const OFFSET: isize>() -> IPrintMediaTypeOptionDetailsVtbl {
        unsafe extern "system" fn SetWarningText<Impl: IPrintMediaTypeOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWarningText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WarningText<Impl: IPrintMediaTypeOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WarningText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IPrintMediaTypeOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Description<Impl: IPrintMediaTypeOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintMediaTypeOptionDetails>, ::windows::core::GetTrustLevel, SetWarningText::<Impl, OFFSET>, WarningText::<Impl, OFFSET>, SetDescription::<Impl, OFFSET>, Description::<Impl, OFFSET>)
    }
}
pub trait IPrintNumberOptionDetailsImpl: Sized + IPrintOptionDetailsImpl {
    fn MinValue(&self) -> ::windows::core::Result<u32>;
    fn MaxValue(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IPrintNumberOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintNumberOptionDetails";
}
impl IPrintNumberOptionDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintNumberOptionDetailsImpl, const OFFSET: isize>() -> IPrintNumberOptionDetailsVtbl {
        unsafe extern "system" fn MinValue<Impl: IPrintNumberOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxValue<Impl: IPrintNumberOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintNumberOptionDetails>, ::windows::core::GetTrustLevel, MinValue::<Impl, OFFSET>, MaxValue::<Impl, OFFSET>)
    }
}
pub trait IPrintOptionDetailsImpl: Sized {
    fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OptionType(&self) -> ::windows::core::Result<PrintOptionType>;
    fn SetErrorText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()>;
    fn State(&self) -> ::windows::core::Result<PrintOptionStates>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn TrySetValue(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IPrintOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails";
}
impl IPrintOptionDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOptionDetailsImpl, const OFFSET: isize>() -> IPrintOptionDetailsVtbl {
        unsafe extern "system" fn OptionId<Impl: IPrintOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OptionId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OptionType<Impl: IPrintOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintOptionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OptionType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetErrorText<Impl: IPrintOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetErrorText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ErrorText<Impl: IPrintOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Impl: IPrintOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintOptionStates) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetState(value).into()
        }
        unsafe extern "system" fn State<Impl: IPrintOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintOptionStates) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IPrintOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetValue<Impl: IPrintOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrySetValue(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintOptionDetails>, ::windows::core::GetTrustLevel, OptionId::<Impl, OFFSET>, OptionType::<Impl, OFFSET>, SetErrorText::<Impl, OFFSET>, ErrorText::<Impl, OFFSET>, SetState::<Impl, OFFSET>, State::<Impl, OFFSET>, Value::<Impl, OFFSET>, TrySetValue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintOrientationOptionDetailsImpl: Sized {
    fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintOrientationOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintOrientationOptionDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintOrientationOptionDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintOrientationOptionDetailsImpl, const OFFSET: isize>() -> IPrintOrientationOptionDetailsVtbl {
        unsafe extern "system" fn SetWarningText<Impl: IPrintOrientationOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWarningText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WarningText<Impl: IPrintOrientationOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WarningText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IPrintOrientationOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Description<Impl: IPrintOrientationOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintOrientationOptionDetails>, ::windows::core::GetTrustLevel, SetWarningText::<Impl, OFFSET>, WarningText::<Impl, OFFSET>, SetDescription::<Impl, OFFSET>, Description::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintPageRangeOptionDetailsImpl: Sized {
    fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintPageRangeOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintPageRangeOptionDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintPageRangeOptionDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintPageRangeOptionDetailsImpl, const OFFSET: isize>() -> IPrintPageRangeOptionDetailsVtbl {
        unsafe extern "system" fn SetWarningText<Impl: IPrintPageRangeOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWarningText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WarningText<Impl: IPrintPageRangeOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WarningText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IPrintPageRangeOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Description<Impl: IPrintPageRangeOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintPageRangeOptionDetails>, ::windows::core::GetTrustLevel, SetWarningText::<Impl, OFFSET>, WarningText::<Impl, OFFSET>, SetDescription::<Impl, OFFSET>, Description::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintQualityOptionDetailsImpl: Sized {
    fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintQualityOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintQualityOptionDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintQualityOptionDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintQualityOptionDetailsImpl, const OFFSET: isize>() -> IPrintQualityOptionDetailsVtbl {
        unsafe extern "system" fn SetWarningText<Impl: IPrintQualityOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWarningText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WarningText<Impl: IPrintQualityOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WarningText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IPrintQualityOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Description<Impl: IPrintQualityOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintQualityOptionDetails>, ::windows::core::GetTrustLevel, SetWarningText::<Impl, OFFSET>, WarningText::<Impl, OFFSET>, SetDescription::<Impl, OFFSET>, Description::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintStapleOptionDetailsImpl: Sized {
    fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintStapleOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintStapleOptionDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintStapleOptionDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintStapleOptionDetailsImpl, const OFFSET: isize>() -> IPrintStapleOptionDetailsVtbl {
        unsafe extern "system" fn SetWarningText<Impl: IPrintStapleOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWarningText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WarningText<Impl: IPrintStapleOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WarningText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IPrintStapleOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Description<Impl: IPrintStapleOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintStapleOptionDetails>, ::windows::core::GetTrustLevel, SetWarningText::<Impl, OFFSET>, WarningText::<Impl, OFFSET>, SetDescription::<Impl, OFFSET>, Description::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTaskOptionChangedEventArgsImpl: Sized {
    fn OptionId(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintTaskOptionChangedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintTaskOptionChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintTaskOptionChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTaskOptionChangedEventArgsImpl, const OFFSET: isize>() -> IPrintTaskOptionChangedEventArgsVtbl {
        unsafe extern "system" fn OptionId<Impl: IPrintTaskOptionChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OptionId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintTaskOptionChangedEventArgs>, ::windows::core::GetTrustLevel, OptionId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTaskOptionDetailsImpl: Sized {
    fn Options(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, IPrintOptionDetails>>;
    fn CreateItemListOption(&self, optionid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<PrintCustomItemListOptionDetails>;
    fn CreateTextOption(&self, optionid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<PrintCustomTextOptionDetails>;
    fn OptionChanged(&self, eventhandler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PrintTaskOptionDetails, PrintTaskOptionChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveOptionChanged(&self, eventcookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BeginValidation(&self, eventhandler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PrintTaskOptionDetails, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveBeginValidation(&self, eventcookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintTaskOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintTaskOptionDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintTaskOptionDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTaskOptionDetailsImpl, const OFFSET: isize>() -> IPrintTaskOptionDetailsVtbl {
        unsafe extern "system" fn Options<Impl: IPrintTaskOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Options() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItemListOption<Impl: IPrintTaskOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateItemListOption(&*(&optionid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&displayname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTextOption<Impl: IPrintTaskOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTextOption(&*(&optionid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&displayname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OptionChanged<Impl: IPrintTaskOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OptionChanged(&*(&eventhandler as *const <super::super::super::Foundation::TypedEventHandler<PrintTaskOptionDetails, PrintTaskOptionChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<PrintTaskOptionDetails, PrintTaskOptionChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveOptionChanged<Impl: IPrintTaskOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveOptionChanged(&*(&eventcookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BeginValidation<Impl: IPrintTaskOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginValidation(&*(&eventhandler as *const <super::super::super::Foundation::TypedEventHandler<PrintTaskOptionDetails, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<PrintTaskOptionDetails, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBeginValidation<Impl: IPrintTaskOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBeginValidation(&*(&eventcookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPrintTaskOptionDetails>,
            ::windows::core::GetTrustLevel,
            Options::<Impl, OFFSET>,
            CreateItemListOption::<Impl, OFFSET>,
            CreateTextOption::<Impl, OFFSET>,
            OptionChanged::<Impl, OFFSET>,
            RemoveOptionChanged::<Impl, OFFSET>,
            BeginValidation::<Impl, OFFSET>,
            RemoveBeginValidation::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTaskOptionDetails2Impl: Sized {
    fn CreateToggleOption(&self, optionid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<PrintCustomToggleOptionDetails>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintTaskOptionDetails2 {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintTaskOptionDetails2";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintTaskOptionDetails2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTaskOptionDetails2Impl, const OFFSET: isize>() -> IPrintTaskOptionDetails2Vtbl {
        unsafe extern "system" fn CreateToggleOption<Impl: IPrintTaskOptionDetails2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateToggleOption(&*(&optionid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&displayname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintTaskOptionDetails2>, ::windows::core::GetTrustLevel, CreateToggleOption::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTaskOptionDetailsStaticImpl: Sized {
    fn GetFromPrintTaskOptions(&self, printtaskoptions: &::core::option::Option<super::PrintTaskOptions>) -> ::windows::core::Result<PrintTaskOptionDetails>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintTaskOptionDetailsStatic {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintTaskOptionDetailsStatic";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintTaskOptionDetailsStaticVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTaskOptionDetailsStaticImpl, const OFFSET: isize>() -> IPrintTaskOptionDetailsStaticVtbl {
        unsafe extern "system" fn GetFromPrintTaskOptions<Impl: IPrintTaskOptionDetailsStaticImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printtaskoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFromPrintTaskOptions(&*(&printtaskoptions as *const <super::PrintTaskOptions as ::windows::core::Abi>::Abi as *const <super::PrintTaskOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintTaskOptionDetailsStatic>, ::windows::core::GetTrustLevel, GetFromPrintTaskOptions::<Impl, OFFSET>)
    }
}
pub trait IPrintTextOptionDetailsImpl: Sized + IPrintOptionDetailsImpl {
    fn MaxCharacters(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IPrintTextOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintTextOptionDetails";
}
impl IPrintTextOptionDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTextOptionDetailsImpl, const OFFSET: isize>() -> IPrintTextOptionDetailsVtbl {
        unsafe extern "system" fn MaxCharacters<Impl: IPrintTextOptionDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxCharacters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintTextOptionDetails>, ::windows::core::GetTrustLevel, MaxCharacters::<Impl, OFFSET>)
    }
}
