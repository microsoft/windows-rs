#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections"))]
pub trait IGameListEntry_Impl: Sized {
    fn DisplayInfo(&mut self) -> ::windows::core::Result<super::super::super::ApplicationModel::AppDisplayInfo>;
    fn LaunchAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
    fn Category(&mut self) -> ::windows::core::Result<GameListCategory>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
    fn SetCategoryAsync(&mut self, value: GameListCategory) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections"))]
impl ::windows::core::RuntimeName for IGameListEntry {
    const NAME: &'static str = "Windows.Gaming.Preview.GamesEnumeration.IGameListEntry";
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections"))]
impl IGameListEntry_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameListEntry_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameListEntry_Vtbl {
        unsafe extern "system" fn DisplayInfo<Impl: IGameListEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchAsync<Impl: IGameListEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Category<Impl: IGameListEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GameListCategory) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Category() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IGameListEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCategoryAsync<Impl: IGameListEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: GameListCategory, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCategoryAsync(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameListEntry, BASE_OFFSET>(),
            DisplayInfo: DisplayInfo::<Impl, IMPL_OFFSET>,
            LaunchAsync: LaunchAsync::<Impl, IMPL_OFFSET>,
            Category: Category::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            SetCategoryAsync: SetCategoryAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameListEntry as ::windows::core::Interface>::IID
    }
}
