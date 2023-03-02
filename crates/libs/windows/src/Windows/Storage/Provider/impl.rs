#[doc = "*Required features: `\"Storage_Provider\"`, `\"Foundation_Collections\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation_Collections")]
pub trait IStorageProviderItemPropertySource_Impl: Sized {
    fn GetItemProperties(&self, itempath: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<StorageProviderItemProperty>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IStorageProviderItemPropertySource {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderItemPropertySource";
}
#[cfg(feature = "Foundation_Collections")]
impl IStorageProviderItemPropertySource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageProviderItemPropertySource_Impl, const OFFSET: isize>() -> IStorageProviderItemPropertySource_Vtbl {
        unsafe extern "system" fn GetItemProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageProviderItemPropertySource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itempath: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItemProperties(::core::mem::transmute(&itempath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IStorageProviderItemPropertySource, OFFSET>(),
            GetItemProperties: GetItemProperties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderItemPropertySource as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`, `\"implement\"`*"]
pub trait IStorageProviderPropertyCapabilities_Impl: Sized {
    fn IsPropertySupported(&self, propertycanonicalname: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IStorageProviderPropertyCapabilities {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderPropertyCapabilities";
}
impl IStorageProviderPropertyCapabilities_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageProviderPropertyCapabilities_Impl, const OFFSET: isize>() -> IStorageProviderPropertyCapabilities_Vtbl {
        unsafe extern "system" fn IsPropertySupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageProviderPropertyCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertycanonicalname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsPropertySupported(::core::mem::transmute(&propertycanonicalname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IStorageProviderPropertyCapabilities, OFFSET>(),
            IsPropertySupported: IsPropertySupported::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderPropertyCapabilities as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`, `\"Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation")]
pub trait IStorageProviderStatusUISource_Impl: Sized {
    fn GetStatusUI(&self) -> ::windows::core::Result<StorageProviderStatusUI>;
    fn StatusUIChanged(&self, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IStorageProviderStatusUISource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusUIChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IStorageProviderStatusUISource {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderStatusUISource";
}
#[cfg(feature = "Foundation")]
impl IStorageProviderStatusUISource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageProviderStatusUISource_Impl, const OFFSET: isize>() -> IStorageProviderStatusUISource_Vtbl {
        unsafe extern "system" fn GetStatusUI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageProviderStatusUISource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStatusUI() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusUIChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageProviderStatusUISource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StatusUIChanged(::windows::core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStatusUIChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageProviderStatusUISource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveStatusUIChanged(::core::mem::transmute(&token)).into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IStorageProviderStatusUISource, OFFSET>(),
            GetStatusUI: GetStatusUI::<Identity, Impl, OFFSET>,
            StatusUIChanged: StatusUIChanged::<Identity, Impl, OFFSET>,
            RemoveStatusUIChanged: RemoveStatusUIChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderStatusUISource as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`, `\"implement\"`*"]
pub trait IStorageProviderStatusUISourceFactory_Impl: Sized {
    fn GetStatusUISource(&self, syncrootid: &::windows::core::HSTRING) -> ::windows::core::Result<IStorageProviderStatusUISource>;
}
impl ::windows::core::RuntimeName for IStorageProviderStatusUISourceFactory {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderStatusUISourceFactory";
}
impl IStorageProviderStatusUISourceFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageProviderStatusUISourceFactory_Impl, const OFFSET: isize>() -> IStorageProviderStatusUISourceFactory_Vtbl {
        unsafe extern "system" fn GetStatusUISource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageProviderStatusUISourceFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, syncrootid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStatusUISource(::core::mem::transmute(&syncrootid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IStorageProviderStatusUISourceFactory, OFFSET>(),
            GetStatusUISource: GetStatusUISource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderStatusUISourceFactory as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`, `\"Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation")]
pub trait IStorageProviderUICommand_Impl: Sized {
    fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Icon(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn State(&self) -> ::windows::core::Result<StorageProviderUICommandState>;
    fn Invoke(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IStorageProviderUICommand {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderUICommand";
}
#[cfg(feature = "Foundation")]
impl IStorageProviderUICommand_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageProviderUICommand_Impl, const OFFSET: isize>() -> IStorageProviderUICommand_Vtbl {
        unsafe extern "system" fn Label<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageProviderUICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Label() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageProviderUICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Icon<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageProviderUICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Icon() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageProviderUICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderUICommandState) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.State() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Invoke<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageProviderUICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Invoke().into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IStorageProviderUICommand, OFFSET>(),
            Label: Label::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            Icon: Icon::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            Invoke: Invoke::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderUICommand as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`, `\"implement\"`*"]
pub trait IStorageProviderUriSource_Impl: Sized {
    fn GetPathForContentUri(&self, contenturi: &::windows::core::HSTRING, result: ::core::option::Option<&StorageProviderGetPathForContentUriResult>) -> ::windows::core::Result<()>;
    fn GetContentInfoForPath(&self, path: &::windows::core::HSTRING, result: ::core::option::Option<&StorageProviderGetContentInfoForPathResult>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IStorageProviderUriSource {
    const NAME: &'static str = "Windows.Storage.Provider.IStorageProviderUriSource";
}
impl IStorageProviderUriSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageProviderUriSource_Impl, const OFFSET: isize>() -> IStorageProviderUriSource_Vtbl {
        unsafe extern "system" fn GetPathForContentUri<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageProviderUriSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenturi: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPathForContentUri(::core::mem::transmute(&contenturi), ::windows::core::from_raw_borrowed(&result)).into()
        }
        unsafe extern "system" fn GetContentInfoForPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStorageProviderUriSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetContentInfoForPath(::core::mem::transmute(&path), ::windows::core::from_raw_borrowed(&result)).into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IStorageProviderUriSource, OFFSET>(),
            GetPathForContentUri: GetPathForContentUri::<Identity, Impl, OFFSET>,
            GetContentInfoForPath: GetContentInfoForPath::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProviderUriSource as ::windows::core::ComInterface>::IID
    }
}
