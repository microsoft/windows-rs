#[cfg(feature = "Foundation_Collections")]
pub trait IMediaEncodingProperties_Impl: Sized {
    fn Properties(&self) -> ::windows::core::Result<MediaPropertySet>;
    fn Type(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSubtype(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Subtype(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IMediaEncodingProperties {
    const NAME: &'static str = "Windows.Media.MediaProperties.IMediaEncodingProperties";
}
#[cfg(feature = "Foundation_Collections")]
impl IMediaEncodingProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaEncodingProperties_Impl, const OFFSET: isize>() -> IMediaEncodingProperties_Vtbl {
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl, Impl: IMediaEncodingProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl, Impl: IMediaEncodingProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubtype<Identity: ::windows::core::IUnknownImpl, Impl: IMediaEncodingProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSubtype(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Subtype<Identity: ::windows::core::IUnknownImpl, Impl: IMediaEncodingProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaEncodingProperties, OFFSET>(),
            Properties: Properties::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            SetSubtype: SetSubtype::<Identity, Impl, OFFSET>,
            Subtype: Subtype::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaEncodingProperties as ::windows::core::Interface>::IID
    }
}
