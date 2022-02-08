#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
pub trait IStorageItemInformation_Impl: Sized {
    fn MusicProperties(&self) -> ::windows::core::Result<super::FileProperties::MusicProperties>;
    fn VideoProperties(&self) -> ::windows::core::Result<super::FileProperties::VideoProperties>;
    fn ImageProperties(&self) -> ::windows::core::Result<super::FileProperties::ImageProperties>;
    fn DocumentProperties(&self) -> ::windows::core::Result<super::FileProperties::DocumentProperties>;
    fn BasicProperties(&self) -> ::windows::core::Result<super::FileProperties::BasicProperties>;
    fn Thumbnail(&self) -> ::windows::core::Result<super::FileProperties::StorageItemThumbnail>;
    fn ThumbnailUpdated(&self, changedhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveThumbnailUpdated(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PropertiesUpdated(&self, changedhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePropertiesUpdated(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl ::windows::core::RuntimeName for IStorageItemInformation {
    const NAME: &'static str = "Windows.Storage.BulkAccess.IStorageItemInformation";
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl IStorageItemInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemInformation_Impl, const OFFSET: isize>() -> IStorageItemInformation_Vtbl {
        unsafe extern "system" fn MusicProperties<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MusicProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoProperties<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).VideoProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImageProperties<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ImageProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentProperties<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DocumentProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BasicProperties<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BasicProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Thumbnail<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Thumbnail() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ThumbnailUpdated<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changedhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ThumbnailUpdated(::core::mem::transmute(&changedhandler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveThumbnailUpdated<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveThumbnailUpdated(::core::mem::transmute(&eventcookie)).into()
        }
        unsafe extern "system" fn PropertiesUpdated<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changedhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PropertiesUpdated(::core::mem::transmute(&changedhandler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePropertiesUpdated<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemovePropertiesUpdated(::core::mem::transmute(&eventcookie)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageItemInformation, OFFSET>(),
            MusicProperties: MusicProperties::<Identity, Impl, OFFSET>,
            VideoProperties: VideoProperties::<Identity, Impl, OFFSET>,
            ImageProperties: ImageProperties::<Identity, Impl, OFFSET>,
            DocumentProperties: DocumentProperties::<Identity, Impl, OFFSET>,
            BasicProperties: BasicProperties::<Identity, Impl, OFFSET>,
            Thumbnail: Thumbnail::<Identity, Impl, OFFSET>,
            ThumbnailUpdated: ThumbnailUpdated::<Identity, Impl, OFFSET>,
            RemoveThumbnailUpdated: RemoveThumbnailUpdated::<Identity, Impl, OFFSET>,
            PropertiesUpdated: PropertiesUpdated::<Identity, Impl, OFFSET>,
            RemovePropertiesUpdated: RemovePropertiesUpdated::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageItemInformation as ::windows::core::Interface>::IID
    }
}
