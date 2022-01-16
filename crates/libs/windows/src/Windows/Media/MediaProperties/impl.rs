#[cfg(feature = "Foundation_Collections")]
pub trait IMediaEncodingProperties_Impl: Sized {
    fn Properties(&mut self) -> ::windows::core::Result<MediaPropertySet>;
    fn Type(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSubtype(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Subtype(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IMediaEncodingProperties {
    const NAME: &'static str = "Windows.Media.MediaProperties.IMediaEncodingProperties";
}
#[cfg(feature = "Foundation_Collections")]
impl IMediaEncodingProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaEncodingProperties_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaEncodingProperties_Vtbl {
        unsafe extern "system" fn Properties<Impl: IMediaEncodingProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IMediaEncodingProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubtype<Impl: IMediaEncodingProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSubtype(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Subtype<Impl: IMediaEncodingProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subtype() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaEncodingProperties, BASE_OFFSET>(),
            Properties: Properties::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
            SetSubtype: SetSubtype::<Impl, IMPL_OFFSET>,
            Subtype: Subtype::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaEncodingProperties as ::windows::core::Interface>::IID
    }
}
