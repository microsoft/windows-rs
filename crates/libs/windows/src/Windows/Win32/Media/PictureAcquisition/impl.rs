pub trait IPhotoAcquireImpl: Sized {
    fn CreatePhotoSource();
    fn Acquire();
    fn EnumResults();
}
impl ::windows::core::RuntimeName for IPhotoAcquire {
    const NAME: &'static str = "Windows.Win32.Media.PictureAcquisition.IPhotoAcquire";
}
impl IPhotoAcquireVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoAcquireImpl, const OFFSET: isize>() -> IPhotoAcquireVtbl {
        unsafe extern "system" fn CreatePhotoSource<Impl: IPhotoAcquireImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdevice: super::super::Foundation::PWSTR, ppphotoacquiresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePhotoSource(&*(&pszdevice as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppphotoacquiresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Acquire<Impl: IPhotoAcquireImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphotoacquiresource: ::windows::core::RawPtr, fshowprogress: super::super::Foundation::BOOL, hwndparent: super::super::Foundation::HWND, pszapplicationname: super::super::Foundation::PWSTR, pphotoacquireprogresscb: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Acquire(
                &*(&pphotoacquiresource as *const <IPhotoAcquireSource as ::windows::core::Abi>::Abi as *const <IPhotoAcquireSource as ::windows::core::DefaultType>::DefaultType),
                &*(&fshowprogress as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                &*(&pszapplicationname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pphotoacquireprogresscb as *const <IPhotoAcquireProgressCB as ::windows::core::Abi>::Abi as *const <IPhotoAcquireProgressCB as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumResults<Impl: IPhotoAcquireImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumfilepaths: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumResults(::core::mem::transmute_copy(&ppenumfilepaths)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPhotoAcquire>, ::windows::core::GetTrustLevel, CreatePhotoSource::<Impl, OFFSET>, Acquire::<Impl, OFFSET>, EnumResults::<Impl, OFFSET>)
    }
}
pub trait IPhotoAcquireDeviceSelectionDialogImpl: Sized {
    fn SetTitle();
    fn SetSubmitButtonText();
    fn DoModal();
}
impl ::windows::core::RuntimeName for IPhotoAcquireDeviceSelectionDialog {
    const NAME: &'static str = "Windows.Win32.Media.PictureAcquisition.IPhotoAcquireDeviceSelectionDialog";
}
impl IPhotoAcquireDeviceSelectionDialogVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoAcquireDeviceSelectionDialogImpl, const OFFSET: isize>() -> IPhotoAcquireDeviceSelectionDialogVtbl {
        unsafe extern "system" fn SetTitle<Impl: IPhotoAcquireDeviceSelectionDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztitle: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTitle(&*(&psztitle as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubmitButtonText<Impl: IPhotoAcquireDeviceSelectionDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubmitbuttontext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSubmitButtonText(&*(&pszsubmitbuttontext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoModal<Impl: IPhotoAcquireDeviceSelectionDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwdeviceflags: u32, pbstrdeviceid: *mut super::super::Foundation::BSTR, pndevicetype: *mut DEVICE_SELECTION_DEVICE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoModal(&*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), dwdeviceflags, &*(&pbstrdeviceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), pndevicetype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPhotoAcquireDeviceSelectionDialog>, ::windows::core::GetTrustLevel, SetTitle::<Impl, OFFSET>, SetSubmitButtonText::<Impl, OFFSET>, DoModal::<Impl, OFFSET>)
    }
}
pub trait IPhotoAcquireItemImpl: Sized {
    fn GetItemName();
    fn GetThumbnail();
    fn GetProperty();
    fn SetProperty();
    fn GetStream();
    fn CanDelete();
    fn Delete();
    fn GetSubItemCount();
    fn GetSubItemAt();
}
impl ::windows::core::RuntimeName for IPhotoAcquireItem {
    const NAME: &'static str = "Windows.Win32.Media.PictureAcquisition.IPhotoAcquireItem";
}
impl IPhotoAcquireItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoAcquireItemImpl, const OFFSET: isize>() -> IPhotoAcquireItemVtbl {
        unsafe extern "system" fn GetItemName<Impl: IPhotoAcquireItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstritemname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemName(::core::mem::transmute_copy(&pbstritemname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThumbnail<Impl: IPhotoAcquireItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sizethumbnail: super::super::Foundation::SIZE, phbmpthumbnail: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetThumbnail(&*(&sizethumbnail as *const <super::super::Foundation::SIZE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::SIZE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&phbmpthumbnail)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IPhotoAcquireItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IPhotoAcquireItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProperty(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), &*(&pv as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Impl: IPhotoAcquireItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStream(::core::mem::transmute_copy(&ppstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanDelete<Impl: IPhotoAcquireItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfcandelete: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanDelete(::core::mem::transmute_copy(&pfcandelete)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IPhotoAcquireItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delete() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubItemCount<Impl: IPhotoAcquireItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubItemCount(::core::mem::transmute_copy(&pncount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubItemAt<Impl: IPhotoAcquireItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nitemindex: u32, ppphotoacquireitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubItemAt(nitemindex, ::core::mem::transmute_copy(&ppphotoacquireitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPhotoAcquireItem>,
            ::windows::core::GetTrustLevel,
            GetItemName::<Impl, OFFSET>,
            GetThumbnail::<Impl, OFFSET>,
            GetProperty::<Impl, OFFSET>,
            SetProperty::<Impl, OFFSET>,
            GetStream::<Impl, OFFSET>,
            CanDelete::<Impl, OFFSET>,
            Delete::<Impl, OFFSET>,
            GetSubItemCount::<Impl, OFFSET>,
            GetSubItemAt::<Impl, OFFSET>,
        )
    }
}
pub trait IPhotoAcquireOptionsDialogImpl: Sized {
    fn Initialize();
    fn Create();
    fn Destroy();
    fn DoModal();
    fn SaveData();
}
impl ::windows::core::RuntimeName for IPhotoAcquireOptionsDialog {
    const NAME: &'static str = "Windows.Win32.Media.PictureAcquisition.IPhotoAcquireOptionsDialog";
}
impl IPhotoAcquireOptionsDialogVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoAcquireOptionsDialogImpl, const OFFSET: isize>() -> IPhotoAcquireOptionsDialogVtbl {
        unsafe extern "system" fn Initialize<Impl: IPhotoAcquireOptionsDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszregistryroot: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&pszregistryroot as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Impl: IPhotoAcquireOptionsDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, phwnddialog: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&phwnddialog)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Destroy<Impl: IPhotoAcquireOptionsDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Destroy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoModal<Impl: IPhotoAcquireOptionsDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ppnreturncode: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoModal(&*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), ppnreturncode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveData<Impl: IPhotoAcquireOptionsDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPhotoAcquireOptionsDialog>, ::windows::core::GetTrustLevel, Initialize::<Impl, OFFSET>, Create::<Impl, OFFSET>, Destroy::<Impl, OFFSET>, DoModal::<Impl, OFFSET>, SaveData::<Impl, OFFSET>)
    }
}
pub trait IPhotoAcquirePluginImpl: Sized {
    fn Initialize();
    fn ProcessItem();
    fn TransferComplete();
    fn DisplayConfigureDialog();
}
impl ::windows::core::RuntimeName for IPhotoAcquirePlugin {
    const NAME: &'static str = "Windows.Win32.Media.PictureAcquisition.IPhotoAcquirePlugin";
}
impl IPhotoAcquirePluginVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoAcquirePluginImpl, const OFFSET: isize>() -> IPhotoAcquirePluginVtbl {
        unsafe extern "system" fn Initialize<Impl: IPhotoAcquirePluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphotoacquiresource: ::windows::core::RawPtr, pphotoacquireprogresscb: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&pphotoacquiresource as *const <IPhotoAcquireSource as ::windows::core::Abi>::Abi as *const <IPhotoAcquireSource as ::windows::core::DefaultType>::DefaultType), &*(&pphotoacquireprogresscb as *const <IPhotoAcquireProgressCB as ::windows::core::Abi>::Abi as *const <IPhotoAcquireProgressCB as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessItem<Impl: IPhotoAcquirePluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwacquirestage: u32, pphotoacquireitem: ::windows::core::RawPtr, poriginalitemstream: ::windows::core::RawPtr, pszfinalfilename: super::super::Foundation::PWSTR, ppropertystore: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessItem(
                dwacquirestage,
                &*(&pphotoacquireitem as *const <IPhotoAcquireItem as ::windows::core::Abi>::Abi as *const <IPhotoAcquireItem as ::windows::core::DefaultType>::DefaultType),
                &*(&poriginalitemstream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&pszfinalfilename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&ppropertystore as *const <super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransferComplete<Impl: IPhotoAcquirePluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransferComplete(hr) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayConfigureDialog<Impl: IPhotoAcquirePluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayConfigureDialog(&*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPhotoAcquirePlugin>, ::windows::core::GetTrustLevel, Initialize::<Impl, OFFSET>, ProcessItem::<Impl, OFFSET>, TransferComplete::<Impl, OFFSET>, DisplayConfigureDialog::<Impl, OFFSET>)
    }
}
pub trait IPhotoAcquireProgressCBImpl: Sized {
    fn Cancelled();
    fn StartEnumeration();
    fn FoundItem();
    fn EndEnumeration();
    fn StartTransfer();
    fn StartItemTransfer();
    fn DirectoryCreated();
    fn UpdateTransferPercent();
    fn EndItemTransfer();
    fn EndTransfer();
    fn StartDelete();
    fn StartItemDelete();
    fn UpdateDeletePercent();
    fn EndItemDelete();
    fn EndDelete();
    fn EndSession();
    fn GetDeleteAfterAcquire();
    fn ErrorAdvise();
    fn GetUserInput();
}
impl ::windows::core::RuntimeName for IPhotoAcquireProgressCB {
    const NAME: &'static str = "Windows.Win32.Media.PictureAcquisition.IPhotoAcquireProgressCB";
}
impl IPhotoAcquireProgressCBVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>() -> IPhotoAcquireProgressCBVtbl {
        unsafe extern "system" fn Cancelled<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfcancelled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cancelled(::core::mem::transmute_copy(&pfcancelled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartEnumeration<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphotoacquiresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartEnumeration(&*(&pphotoacquiresource as *const <IPhotoAcquireSource as ::windows::core::Abi>::Abi as *const <IPhotoAcquireSource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FoundItem<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphotoacquireitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FoundItem(&*(&pphotoacquireitem as *const <IPhotoAcquireItem as ::windows::core::Abi>::Abi as *const <IPhotoAcquireItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndEnumeration<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndEnumeration(hr) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartTransfer<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphotoacquiresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartTransfer(&*(&pphotoacquiresource as *const <IPhotoAcquireSource as ::windows::core::Abi>::Abi as *const <IPhotoAcquireSource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartItemTransfer<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nitemindex: u32, pphotoacquireitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartItemTransfer(nitemindex, &*(&pphotoacquireitem as *const <IPhotoAcquireItem as ::windows::core::Abi>::Abi as *const <IPhotoAcquireItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DirectoryCreated<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdirectory: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DirectoryCreated(&*(&pszdirectory as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateTransferPercent<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foverall: super::super::Foundation::BOOL, npercent: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateTransferPercent(&*(&foverall as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), npercent) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndItemTransfer<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nitemindex: u32, pphotoacquireitem: ::windows::core::RawPtr, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndItemTransfer(nitemindex, &*(&pphotoacquireitem as *const <IPhotoAcquireItem as ::windows::core::Abi>::Abi as *const <IPhotoAcquireItem as ::windows::core::DefaultType>::DefaultType), hr) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndTransfer<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndTransfer(hr) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartDelete<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphotoacquiresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartDelete(&*(&pphotoacquiresource as *const <IPhotoAcquireSource as ::windows::core::Abi>::Abi as *const <IPhotoAcquireSource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartItemDelete<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nitemindex: u32, pphotoacquireitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartItemDelete(nitemindex, &*(&pphotoacquireitem as *const <IPhotoAcquireItem as ::windows::core::Abi>::Abi as *const <IPhotoAcquireItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateDeletePercent<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, npercent: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateDeletePercent(npercent) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndItemDelete<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nitemindex: u32, pphotoacquireitem: ::windows::core::RawPtr, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndItemDelete(nitemindex, &*(&pphotoacquireitem as *const <IPhotoAcquireItem as ::windows::core::Abi>::Abi as *const <IPhotoAcquireItem as ::windows::core::DefaultType>::DefaultType), hr) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndDelete<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndDelete(hr) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndSession<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndSession(hr) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeleteAfterAcquire<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfdeleteafteracquire: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeleteAfterAcquire(::core::mem::transmute_copy(&pfdeleteafteracquire)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorAdvise<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, pszerrormessage: super::super::Foundation::PWSTR, nmessagetype: ERROR_ADVISE_MESSAGE_TYPE, pnerroradviseresult: *mut ERROR_ADVISE_RESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorAdvise(hr, &*(&pszerrormessage as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), nmessagetype, ::core::mem::transmute_copy(&pnerroradviseresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserInput<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riidtype: *const ::windows::core::GUID, punknown: *mut ::core::ffi::c_void, ppropvarresult: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvardefault: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUserInput(
                &*(&riidtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&punknown as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppropvarresult),
                &*(&ppropvardefault as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPhotoAcquireProgressCB>,
            ::windows::core::GetTrustLevel,
            Cancelled::<Impl, OFFSET>,
            StartEnumeration::<Impl, OFFSET>,
            FoundItem::<Impl, OFFSET>,
            EndEnumeration::<Impl, OFFSET>,
            StartTransfer::<Impl, OFFSET>,
            StartItemTransfer::<Impl, OFFSET>,
            DirectoryCreated::<Impl, OFFSET>,
            UpdateTransferPercent::<Impl, OFFSET>,
            EndItemTransfer::<Impl, OFFSET>,
            EndTransfer::<Impl, OFFSET>,
            StartDelete::<Impl, OFFSET>,
            StartItemDelete::<Impl, OFFSET>,
            UpdateDeletePercent::<Impl, OFFSET>,
            EndItemDelete::<Impl, OFFSET>,
            EndDelete::<Impl, OFFSET>,
            EndSession::<Impl, OFFSET>,
            GetDeleteAfterAcquire::<Impl, OFFSET>,
            ErrorAdvise::<Impl, OFFSET>,
            GetUserInput::<Impl, OFFSET>,
        )
    }
}
pub trait IPhotoAcquireSettingsImpl: Sized {
    fn InitializeFromRegistry();
    fn SetFlags();
    fn SetOutputFilenameTemplate();
    fn SetSequencePaddingWidth();
    fn SetSequenceZeroPadding();
    fn SetGroupTag();
    fn SetAcquisitionTime();
    fn GetFlags();
    fn GetOutputFilenameTemplate();
    fn GetSequencePaddingWidth();
    fn GetSequenceZeroPadding();
    fn GetGroupTag();
    fn GetAcquisitionTime();
}
impl ::windows::core::RuntimeName for IPhotoAcquireSettings {
    const NAME: &'static str = "Windows.Win32.Media.PictureAcquisition.IPhotoAcquireSettings";
}
impl IPhotoAcquireSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>() -> IPhotoAcquireSettingsVtbl {
        unsafe extern "system" fn InitializeFromRegistry<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszregistrykey: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitializeFromRegistry(&*(&pszregistrykey as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlags<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwphotoacquireflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFlags(dwphotoacquireflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputFilenameTemplate<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztemplate: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOutputFilenameTemplate(&*(&psztemplate as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSequencePaddingWidth<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwwidth: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSequencePaddingWidth(dwwidth) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSequenceZeroPadding<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fzeropad: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSequenceZeroPadding(&*(&fzeropad as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroupTag<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszgrouptag: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGroupTag(&*(&pszgrouptag as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAcquisitionTime<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pftacquisitiontime: *const super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAcquisitionTime(&*(&pftacquisitiontime as *const <super::super::Foundation::FILETIME as ::windows::core::Abi>::Abi as *const <super::super::Foundation::FILETIME as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlags<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwphotoacquireflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFlags(::core::mem::transmute_copy(&pdwphotoacquireflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputFilenameTemplate<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtemplate: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputFilenameTemplate(::core::mem::transmute_copy(&pbstrtemplate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSequencePaddingWidth<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwwidth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSequencePaddingWidth(::core::mem::transmute_copy(&pdwwidth)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSequenceZeroPadding<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfzeropad: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSequenceZeroPadding(::core::mem::transmute_copy(&pfzeropad)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGroupTag<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrgrouptag: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGroupTag(::core::mem::transmute_copy(&pbstrgrouptag)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAcquisitionTime<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pftacquisitiontime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAcquisitionTime(::core::mem::transmute_copy(&pftacquisitiontime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPhotoAcquireSettings>,
            ::windows::core::GetTrustLevel,
            InitializeFromRegistry::<Impl, OFFSET>,
            SetFlags::<Impl, OFFSET>,
            SetOutputFilenameTemplate::<Impl, OFFSET>,
            SetSequencePaddingWidth::<Impl, OFFSET>,
            SetSequenceZeroPadding::<Impl, OFFSET>,
            SetGroupTag::<Impl, OFFSET>,
            SetAcquisitionTime::<Impl, OFFSET>,
            GetFlags::<Impl, OFFSET>,
            GetOutputFilenameTemplate::<Impl, OFFSET>,
            GetSequencePaddingWidth::<Impl, OFFSET>,
            GetSequenceZeroPadding::<Impl, OFFSET>,
            GetGroupTag::<Impl, OFFSET>,
            GetAcquisitionTime::<Impl, OFFSET>,
        )
    }
}
pub trait IPhotoAcquireSourceImpl: Sized {
    fn GetFriendlyName();
    fn GetDeviceIcons();
    fn InitializeItemList();
    fn GetItemCount();
    fn GetItemAt();
    fn GetPhotoAcquireSettings();
    fn GetDeviceId();
    fn BindToObject();
}
impl ::windows::core::RuntimeName for IPhotoAcquireSource {
    const NAME: &'static str = "Windows.Win32.Media.PictureAcquisition.IPhotoAcquireSource";
}
impl IPhotoAcquireSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoAcquireSourceImpl, const OFFSET: isize>() -> IPhotoAcquireSourceVtbl {
        unsafe extern "system" fn GetFriendlyName<Impl: IPhotoAcquireSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfriendlyname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFriendlyName(::core::mem::transmute_copy(&pbstrfriendlyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceIcons<Impl: IPhotoAcquireSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nsize: u32, phlargeicon: *mut super::super::UI::WindowsAndMessaging::HICON, phsmallicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceIcons(nsize, &*(&phlargeicon as *const <super::super::UI::WindowsAndMessaging::HICON as ::windows::core::Abi>::Abi as *const <super::super::UI::WindowsAndMessaging::HICON as ::windows::core::DefaultType>::DefaultType), &*(&phsmallicon as *const <super::super::UI::WindowsAndMessaging::HICON as ::windows::core::Abi>::Abi as *const <super::super::UI::WindowsAndMessaging::HICON as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeItemList<Impl: IPhotoAcquireSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fforceenumeration: super::super::Foundation::BOOL, pphotoacquireprogresscb: ::windows::core::RawPtr, pnitemcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitializeItemList(&*(&fforceenumeration as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), &*(&pphotoacquireprogresscb as *const <IPhotoAcquireProgressCB as ::windows::core::Abi>::Abi as *const <IPhotoAcquireProgressCB as ::windows::core::DefaultType>::DefaultType), pnitemcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemCount<Impl: IPhotoAcquireSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnitemcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemCount(::core::mem::transmute_copy(&pnitemcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemAt<Impl: IPhotoAcquireSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, ppphotoacquireitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemAt(nindex, ::core::mem::transmute_copy(&ppphotoacquireitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPhotoAcquireSettings<Impl: IPhotoAcquireSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppphotoacquiresettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPhotoAcquireSettings(::core::mem::transmute_copy(&ppphotoacquiresettings)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceId<Impl: IPhotoAcquireSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdeviceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceId(::core::mem::transmute_copy(&pbstrdeviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindToObject<Impl: IPhotoAcquireSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BindToObject(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPhotoAcquireSource>,
            ::windows::core::GetTrustLevel,
            GetFriendlyName::<Impl, OFFSET>,
            GetDeviceIcons::<Impl, OFFSET>,
            InitializeItemList::<Impl, OFFSET>,
            GetItemCount::<Impl, OFFSET>,
            GetItemAt::<Impl, OFFSET>,
            GetPhotoAcquireSettings::<Impl, OFFSET>,
            GetDeviceId::<Impl, OFFSET>,
            BindToObject::<Impl, OFFSET>,
        )
    }
}
pub trait IPhotoProgressActionCBImpl: Sized {
    fn DoAction();
}
impl ::windows::core::RuntimeName for IPhotoProgressActionCB {
    const NAME: &'static str = "Windows.Win32.Media.PictureAcquisition.IPhotoProgressActionCB";
}
impl IPhotoProgressActionCBVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoProgressActionCBImpl, const OFFSET: isize>() -> IPhotoProgressActionCBVtbl {
        unsafe extern "system" fn DoAction<Impl: IPhotoProgressActionCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoAction(&*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPhotoProgressActionCB>, ::windows::core::GetTrustLevel, DoAction::<Impl, OFFSET>)
    }
}
pub trait IPhotoProgressDialogImpl: Sized {
    fn Create();
    fn GetWindow();
    fn Destroy();
    fn SetTitle();
    fn ShowCheckbox();
    fn SetCheckboxText();
    fn SetCheckboxCheck();
    fn SetCheckboxTooltip();
    fn IsCheckboxChecked();
    fn SetCaption();
    fn SetImage();
    fn SetPercentComplete();
    fn SetProgressText();
    fn SetActionLinkCallback();
    fn SetActionLinkText();
    fn ShowActionLink();
    fn IsCancelled();
    fn GetUserInput();
}
impl ::windows::core::RuntimeName for IPhotoProgressDialog {
    const NAME: &'static str = "Windows.Win32.Media.PictureAcquisition.IPhotoProgressDialog";
}
impl IPhotoProgressDialogVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoProgressDialogImpl, const OFFSET: isize>() -> IPhotoProgressDialogVtbl {
        unsafe extern "system" fn Create<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWindow<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwndprogressdialog: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWindow(::core::mem::transmute_copy(&phwndprogressdialog)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Destroy<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Destroy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTitle<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztitle: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTitle(&*(&psztitle as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowCheckbox<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowCheckbox(ncheckboxid, &*(&fshow as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckboxText<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCheckboxText(ncheckboxid, &*(&pszcheckboxtext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckboxCheck<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fchecked: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCheckboxCheck(ncheckboxid, &*(&fchecked as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckboxTooltip<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtooltiptext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCheckboxTooltip(ncheckboxid, &*(&pszcheckboxtooltiptext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCheckboxChecked<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pfchecked: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCheckboxChecked(ncheckboxid, ::core::mem::transmute_copy(&pfchecked)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCaption<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztitle: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCaption(&*(&psztitle as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImage<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nimagetype: PROGRESS_DIALOG_IMAGE_TYPE, hicon: super::super::UI::WindowsAndMessaging::HICON, hbitmap: super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetImage(nimagetype, &*(&hicon as *const <super::super::UI::WindowsAndMessaging::HICON as ::windows::core::Abi>::Abi as *const <super::super::UI::WindowsAndMessaging::HICON as ::windows::core::DefaultType>::DefaultType), &*(&hbitmap as *const <super::super::Graphics::Gdi::HBITMAP as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Gdi::HBITMAP as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPercentComplete<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, npercent: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPercentComplete(npercent) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProgressText<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszprogresstext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProgressText(&*(&pszprogresstext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActionLinkCallback<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphotoprogressactioncb: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetActionLinkCallback(&*(&pphotoprogressactioncb as *const <IPhotoProgressActionCB as ::windows::core::Abi>::Abi as *const <IPhotoProgressActionCB as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActionLinkText<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcaption: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetActionLinkText(&*(&pszcaption as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowActionLink<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowActionLink(&*(&fshow as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCancelled<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfcancelled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCancelled(::core::mem::transmute_copy(&pfcancelled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserInput<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riidtype: *const ::windows::core::GUID, punknown: *mut ::core::ffi::c_void, ppropvarresult: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvardefault: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUserInput(
                &*(&riidtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&punknown as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppropvarresult),
                &*(&ppropvardefault as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPhotoProgressDialog>,
            ::windows::core::GetTrustLevel,
            Create::<Impl, OFFSET>,
            GetWindow::<Impl, OFFSET>,
            Destroy::<Impl, OFFSET>,
            SetTitle::<Impl, OFFSET>,
            ShowCheckbox::<Impl, OFFSET>,
            SetCheckboxText::<Impl, OFFSET>,
            SetCheckboxCheck::<Impl, OFFSET>,
            SetCheckboxTooltip::<Impl, OFFSET>,
            IsCheckboxChecked::<Impl, OFFSET>,
            SetCaption::<Impl, OFFSET>,
            SetImage::<Impl, OFFSET>,
            SetPercentComplete::<Impl, OFFSET>,
            SetProgressText::<Impl, OFFSET>,
            SetActionLinkCallback::<Impl, OFFSET>,
            SetActionLinkText::<Impl, OFFSET>,
            ShowActionLink::<Impl, OFFSET>,
            IsCancelled::<Impl, OFFSET>,
            GetUserInput::<Impl, OFFSET>,
        )
    }
}
pub trait IUserInputStringImpl: Sized {
    fn GetSubmitButtonText();
    fn GetPrompt();
    fn GetStringId();
    fn GetStringType();
    fn GetTooltipText();
    fn GetMaxLength();
    fn GetDefault();
    fn GetMruCount();
    fn GetMruEntryAt();
    fn GetImage();
}
impl ::windows::core::RuntimeName for IUserInputString {
    const NAME: &'static str = "Windows.Win32.Media.PictureAcquisition.IUserInputString";
}
impl IUserInputStringVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserInputStringImpl, const OFFSET: isize>() -> IUserInputStringVtbl {
        unsafe extern "system" fn GetSubmitButtonText<Impl: IUserInputStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsubmitbuttontext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubmitButtonText(::core::mem::transmute_copy(&pbstrsubmitbuttontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrompt<Impl: IUserInputStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprompttitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrompt(::core::mem::transmute_copy(&pbstrprompttitle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringId<Impl: IUserInputStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstringid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStringId(::core::mem::transmute_copy(&pbstrstringid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringType<Impl: IUserInputStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnstringtype: *mut USER_INPUT_STRING_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStringType(::core::mem::transmute_copy(&pnstringtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTooltipText<Impl: IUserInputStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtooltiptext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTooltipText(::core::mem::transmute_copy(&pbstrtooltiptext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxLength<Impl: IUserInputStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchmaxlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxLength(::core::mem::transmute_copy(&pcchmaxlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefault<Impl: IUserInputStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdefault: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault(::core::mem::transmute_copy(&pbstrdefault)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMruCount<Impl: IUserInputStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnmrucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMruCount(::core::mem::transmute_copy(&pnmrucount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMruEntryAt<Impl: IUserInputStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, pbstrmruentry: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMruEntryAt(nindex, ::core::mem::transmute_copy(&pbstrmruentry)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImage<Impl: IUserInputStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nsize: u32, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP, phicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetImage(nsize, &*(&phbitmap as *const <super::super::Graphics::Gdi::HBITMAP as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Gdi::HBITMAP as ::windows::core::DefaultType>::DefaultType), &*(&phicon as *const <super::super::UI::WindowsAndMessaging::HICON as ::windows::core::Abi>::Abi as *const <super::super::UI::WindowsAndMessaging::HICON as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUserInputString>,
            ::windows::core::GetTrustLevel,
            GetSubmitButtonText::<Impl, OFFSET>,
            GetPrompt::<Impl, OFFSET>,
            GetStringId::<Impl, OFFSET>,
            GetStringType::<Impl, OFFSET>,
            GetTooltipText::<Impl, OFFSET>,
            GetMaxLength::<Impl, OFFSET>,
            GetDefault::<Impl, OFFSET>,
            GetMruCount::<Impl, OFFSET>,
            GetMruEntryAt::<Impl, OFFSET>,
            GetImage::<Impl, OFFSET>,
        )
    }
}
