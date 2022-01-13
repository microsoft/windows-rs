#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IPhotoAcquireImpl: Sized {
    fn CreatePhotoSource(&mut self, pszdevice: super::super::Foundation::PWSTR) -> ::windows::core::Result<IPhotoAcquireSource>;
    fn Acquire(&mut self, pphotoacquiresource: ::core::option::Option<IPhotoAcquireSource>, fshowprogress: super::super::Foundation::BOOL, hwndparent: super::super::Foundation::HWND, pszapplicationname: super::super::Foundation::PWSTR, pphotoacquireprogresscb: ::core::option::Option<IPhotoAcquireProgressCB>) -> ::windows::core::Result<()>;
    fn EnumResults(&mut self) -> ::windows::core::Result<super::super::System::Com::IEnumString>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IPhotoAcquireVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoAcquireImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoAcquireVtbl {
        unsafe extern "system" fn CreatePhotoSource<Impl: IPhotoAcquireImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdevice: super::super::Foundation::PWSTR, ppphotoacquiresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePhotoSource(::core::mem::transmute_copy(&pszdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppphotoacquiresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Acquire<Impl: IPhotoAcquireImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphotoacquiresource: ::windows::core::RawPtr, fshowprogress: super::super::Foundation::BOOL, hwndparent: super::super::Foundation::HWND, pszapplicationname: super::super::Foundation::PWSTR, pphotoacquireprogresscb: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Acquire(::core::mem::transmute(&pphotoacquiresource), ::core::mem::transmute_copy(&fshowprogress), ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&pszapplicationname), ::core::mem::transmute(&pphotoacquireprogresscb)).into()
        }
        unsafe extern "system" fn EnumResults<Impl: IPhotoAcquireImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumfilepaths: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumResults() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumfilepaths = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreatePhotoSource: CreatePhotoSource::<Impl, IMPL_OFFSET>,
            Acquire: Acquire::<Impl, IMPL_OFFSET>,
            EnumResults: EnumResults::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoAcquire as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPhotoAcquireDeviceSelectionDialogImpl: Sized {
    fn SetTitle(&mut self, psztitle: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetSubmitButtonText(&mut self, pszsubmitbuttontext: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn DoModal(&mut self, hwndparent: super::super::Foundation::HWND, dwdeviceflags: u32, pbstrdeviceid: *mut super::super::Foundation::BSTR, pndevicetype: *mut DEVICE_SELECTION_DEVICE_TYPE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPhotoAcquireDeviceSelectionDialogVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoAcquireDeviceSelectionDialogImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoAcquireDeviceSelectionDialogVtbl {
        unsafe extern "system" fn SetTitle<Impl: IPhotoAcquireDeviceSelectionDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztitle: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTitle(::core::mem::transmute_copy(&psztitle)).into()
        }
        unsafe extern "system" fn SetSubmitButtonText<Impl: IPhotoAcquireDeviceSelectionDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubmitbuttontext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSubmitButtonText(::core::mem::transmute_copy(&pszsubmitbuttontext)).into()
        }
        unsafe extern "system" fn DoModal<Impl: IPhotoAcquireDeviceSelectionDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwdeviceflags: u32, pbstrdeviceid: *mut super::super::Foundation::BSTR, pndevicetype: *mut DEVICE_SELECTION_DEVICE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DoModal(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwdeviceflags), ::core::mem::transmute_copy(&pbstrdeviceid), ::core::mem::transmute_copy(&pndevicetype)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetTitle: SetTitle::<Impl, IMPL_OFFSET>,
            SetSubmitButtonText: SetSubmitButtonText::<Impl, IMPL_OFFSET>,
            DoModal: DoModal::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoAcquireDeviceSelectionDialog as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IPhotoAcquireItemImpl: Sized {
    fn GetItemName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetThumbnail(&mut self, sizethumbnail: super::super::Foundation::SIZE) -> ::windows::core::Result<super::super::Graphics::Gdi::HBITMAP>;
    fn GetProperty(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn SetProperty(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn GetStream(&mut self) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn CanDelete(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
    fn GetSubItemCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetSubItemAt(&mut self, nitemindex: u32) -> ::windows::core::Result<IPhotoAcquireItem>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IPhotoAcquireItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoAcquireItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoAcquireItemVtbl {
        unsafe extern "system" fn GetItemName<Impl: IPhotoAcquireItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstritemname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstritemname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThumbnail<Impl: IPhotoAcquireItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sizethumbnail: super::super::Foundation::SIZE, phbmpthumbnail: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetThumbnail(::core::mem::transmute_copy(&sizethumbnail)) {
                ::core::result::Result::Ok(ok__) => {
                    *phbmpthumbnail = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IPhotoAcquireItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *pv = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IPhotoAcquireItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&pv)).into()
        }
        unsafe extern "system" fn GetStream<Impl: IPhotoAcquireItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStream() {
                ::core::result::Result::Ok(ok__) => {
                    *ppstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanDelete<Impl: IPhotoAcquireItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfcandelete: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanDelete() {
                ::core::result::Result::Ok(ok__) => {
                    *pfcandelete = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IPhotoAcquireItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn GetSubItemCount<Impl: IPhotoAcquireItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pncount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubItemAt<Impl: IPhotoAcquireItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nitemindex: u32, ppphotoacquireitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubItemAt(::core::mem::transmute_copy(&nitemindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppphotoacquireitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetItemName: GetItemName::<Impl, IMPL_OFFSET>,
            GetThumbnail: GetThumbnail::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            GetStream: GetStream::<Impl, IMPL_OFFSET>,
            CanDelete: CanDelete::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            GetSubItemCount: GetSubItemCount::<Impl, IMPL_OFFSET>,
            GetSubItemAt: GetSubItemAt::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoAcquireItem as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPhotoAcquireOptionsDialogImpl: Sized {
    fn Initialize(&mut self, pszregistryroot: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Create(&mut self, hwndparent: super::super::Foundation::HWND) -> ::windows::core::Result<super::super::Foundation::HWND>;
    fn Destroy(&mut self) -> ::windows::core::Result<()>;
    fn DoModal(&mut self, hwndparent: super::super::Foundation::HWND, ppnreturncode: *mut isize) -> ::windows::core::Result<()>;
    fn SaveData(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPhotoAcquireOptionsDialogVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoAcquireOptionsDialogImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoAcquireOptionsDialogVtbl {
        unsafe extern "system" fn Initialize<Impl: IPhotoAcquireOptionsDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszregistryroot: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&pszregistryroot)).into()
        }
        unsafe extern "system" fn Create<Impl: IPhotoAcquireOptionsDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, phwnddialog: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(::core::mem::transmute_copy(&hwndparent)) {
                ::core::result::Result::Ok(ok__) => {
                    *phwnddialog = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Destroy<Impl: IPhotoAcquireOptionsDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Destroy().into()
        }
        unsafe extern "system" fn DoModal<Impl: IPhotoAcquireOptionsDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ppnreturncode: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DoModal(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&ppnreturncode)).into()
        }
        unsafe extern "system" fn SaveData<Impl: IPhotoAcquireOptionsDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SaveData().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Create: Create::<Impl, IMPL_OFFSET>,
            Destroy: Destroy::<Impl, IMPL_OFFSET>,
            DoModal: DoModal::<Impl, IMPL_OFFSET>,
            SaveData: SaveData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoAcquireOptionsDialog as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IPhotoAcquirePluginImpl: Sized {
    fn Initialize(&mut self, pphotoacquiresource: ::core::option::Option<IPhotoAcquireSource>, pphotoacquireprogresscb: ::core::option::Option<IPhotoAcquireProgressCB>) -> ::windows::core::Result<()>;
    fn ProcessItem(&mut self, dwacquirestage: u32, pphotoacquireitem: ::core::option::Option<IPhotoAcquireItem>, poriginalitemstream: ::core::option::Option<super::super::System::Com::IStream>, pszfinalfilename: super::super::Foundation::PWSTR, ppropertystore: ::core::option::Option<super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows::core::Result<()>;
    fn TransferComplete(&mut self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn DisplayConfigureDialog(&mut self, hwndparent: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IPhotoAcquirePluginVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoAcquirePluginImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoAcquirePluginVtbl {
        unsafe extern "system" fn Initialize<Impl: IPhotoAcquirePluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphotoacquiresource: ::windows::core::RawPtr, pphotoacquireprogresscb: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pphotoacquiresource), ::core::mem::transmute(&pphotoacquireprogresscb)).into()
        }
        unsafe extern "system" fn ProcessItem<Impl: IPhotoAcquirePluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwacquirestage: u32, pphotoacquireitem: ::windows::core::RawPtr, poriginalitemstream: ::windows::core::RawPtr, pszfinalfilename: super::super::Foundation::PWSTR, ppropertystore: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessItem(::core::mem::transmute_copy(&dwacquirestage), ::core::mem::transmute(&pphotoacquireitem), ::core::mem::transmute(&poriginalitemstream), ::core::mem::transmute_copy(&pszfinalfilename), ::core::mem::transmute(&ppropertystore)).into()
        }
        unsafe extern "system" fn TransferComplete<Impl: IPhotoAcquirePluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TransferComplete(::core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn DisplayConfigureDialog<Impl: IPhotoAcquirePluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisplayConfigureDialog(::core::mem::transmute_copy(&hwndparent)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            ProcessItem: ProcessItem::<Impl, IMPL_OFFSET>,
            TransferComplete: TransferComplete::<Impl, IMPL_OFFSET>,
            DisplayConfigureDialog: DisplayConfigureDialog::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoAcquirePlugin as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IPhotoAcquireProgressCBImpl: Sized {
    fn Cancelled(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn StartEnumeration(&mut self, pphotoacquiresource: ::core::option::Option<IPhotoAcquireSource>) -> ::windows::core::Result<()>;
    fn FoundItem(&mut self, pphotoacquireitem: ::core::option::Option<IPhotoAcquireItem>) -> ::windows::core::Result<()>;
    fn EndEnumeration(&mut self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn StartTransfer(&mut self, pphotoacquiresource: ::core::option::Option<IPhotoAcquireSource>) -> ::windows::core::Result<()>;
    fn StartItemTransfer(&mut self, nitemindex: u32, pphotoacquireitem: ::core::option::Option<IPhotoAcquireItem>) -> ::windows::core::Result<()>;
    fn DirectoryCreated(&mut self, pszdirectory: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn UpdateTransferPercent(&mut self, foverall: super::super::Foundation::BOOL, npercent: u32) -> ::windows::core::Result<()>;
    fn EndItemTransfer(&mut self, nitemindex: u32, pphotoacquireitem: ::core::option::Option<IPhotoAcquireItem>, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn EndTransfer(&mut self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn StartDelete(&mut self, pphotoacquiresource: ::core::option::Option<IPhotoAcquireSource>) -> ::windows::core::Result<()>;
    fn StartItemDelete(&mut self, nitemindex: u32, pphotoacquireitem: ::core::option::Option<IPhotoAcquireItem>) -> ::windows::core::Result<()>;
    fn UpdateDeletePercent(&mut self, npercent: u32) -> ::windows::core::Result<()>;
    fn EndItemDelete(&mut self, nitemindex: u32, pphotoacquireitem: ::core::option::Option<IPhotoAcquireItem>, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn EndDelete(&mut self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn EndSession(&mut self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn GetDeleteAfterAcquire(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn ErrorAdvise(&mut self, hr: ::windows::core::HRESULT, pszerrormessage: super::super::Foundation::PWSTR, nmessagetype: ERROR_ADVISE_MESSAGE_TYPE) -> ::windows::core::Result<ERROR_ADVISE_RESULT>;
    fn GetUserInput(&mut self, riidtype: *const ::windows::core::GUID, punknown: ::core::option::Option<::windows::core::IUnknown>, ppropvarresult: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvardefault: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IPhotoAcquireProgressCBVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoAcquireProgressCBImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoAcquireProgressCBVtbl {
        unsafe extern "system" fn Cancelled<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfcancelled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cancelled() {
                ::core::result::Result::Ok(ok__) => {
                    *pfcancelled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartEnumeration<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphotoacquiresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartEnumeration(::core::mem::transmute(&pphotoacquiresource)).into()
        }
        unsafe extern "system" fn FoundItem<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphotoacquireitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FoundItem(::core::mem::transmute(&pphotoacquireitem)).into()
        }
        unsafe extern "system" fn EndEnumeration<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndEnumeration(::core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn StartTransfer<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphotoacquiresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartTransfer(::core::mem::transmute(&pphotoacquiresource)).into()
        }
        unsafe extern "system" fn StartItemTransfer<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nitemindex: u32, pphotoacquireitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartItemTransfer(::core::mem::transmute_copy(&nitemindex), ::core::mem::transmute(&pphotoacquireitem)).into()
        }
        unsafe extern "system" fn DirectoryCreated<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdirectory: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DirectoryCreated(::core::mem::transmute_copy(&pszdirectory)).into()
        }
        unsafe extern "system" fn UpdateTransferPercent<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foverall: super::super::Foundation::BOOL, npercent: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateTransferPercent(::core::mem::transmute_copy(&foverall), ::core::mem::transmute_copy(&npercent)).into()
        }
        unsafe extern "system" fn EndItemTransfer<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nitemindex: u32, pphotoacquireitem: ::windows::core::RawPtr, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndItemTransfer(::core::mem::transmute_copy(&nitemindex), ::core::mem::transmute(&pphotoacquireitem), ::core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn EndTransfer<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndTransfer(::core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn StartDelete<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphotoacquiresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartDelete(::core::mem::transmute(&pphotoacquiresource)).into()
        }
        unsafe extern "system" fn StartItemDelete<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nitemindex: u32, pphotoacquireitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartItemDelete(::core::mem::transmute_copy(&nitemindex), ::core::mem::transmute(&pphotoacquireitem)).into()
        }
        unsafe extern "system" fn UpdateDeletePercent<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, npercent: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateDeletePercent(::core::mem::transmute_copy(&npercent)).into()
        }
        unsafe extern "system" fn EndItemDelete<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nitemindex: u32, pphotoacquireitem: ::windows::core::RawPtr, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndItemDelete(::core::mem::transmute_copy(&nitemindex), ::core::mem::transmute(&pphotoacquireitem), ::core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn EndDelete<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndDelete(::core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn EndSession<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndSession(::core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn GetDeleteAfterAcquire<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfdeleteafteracquire: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeleteAfterAcquire() {
                ::core::result::Result::Ok(ok__) => {
                    *pfdeleteafteracquire = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorAdvise<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, pszerrormessage: super::super::Foundation::PWSTR, nmessagetype: ERROR_ADVISE_MESSAGE_TYPE, pnerroradviseresult: *mut ERROR_ADVISE_RESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorAdvise(::core::mem::transmute_copy(&hr), ::core::mem::transmute_copy(&pszerrormessage), ::core::mem::transmute_copy(&nmessagetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pnerroradviseresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserInput<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riidtype: *const ::windows::core::GUID, punknown: *mut ::core::ffi::c_void, ppropvarresult: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvardefault: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetUserInput(::core::mem::transmute_copy(&riidtype), ::core::mem::transmute(&punknown), ::core::mem::transmute_copy(&ppropvarresult), ::core::mem::transmute_copy(&ppropvardefault)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Cancelled: Cancelled::<Impl, IMPL_OFFSET>,
            StartEnumeration: StartEnumeration::<Impl, IMPL_OFFSET>,
            FoundItem: FoundItem::<Impl, IMPL_OFFSET>,
            EndEnumeration: EndEnumeration::<Impl, IMPL_OFFSET>,
            StartTransfer: StartTransfer::<Impl, IMPL_OFFSET>,
            StartItemTransfer: StartItemTransfer::<Impl, IMPL_OFFSET>,
            DirectoryCreated: DirectoryCreated::<Impl, IMPL_OFFSET>,
            UpdateTransferPercent: UpdateTransferPercent::<Impl, IMPL_OFFSET>,
            EndItemTransfer: EndItemTransfer::<Impl, IMPL_OFFSET>,
            EndTransfer: EndTransfer::<Impl, IMPL_OFFSET>,
            StartDelete: StartDelete::<Impl, IMPL_OFFSET>,
            StartItemDelete: StartItemDelete::<Impl, IMPL_OFFSET>,
            UpdateDeletePercent: UpdateDeletePercent::<Impl, IMPL_OFFSET>,
            EndItemDelete: EndItemDelete::<Impl, IMPL_OFFSET>,
            EndDelete: EndDelete::<Impl, IMPL_OFFSET>,
            EndSession: EndSession::<Impl, IMPL_OFFSET>,
            GetDeleteAfterAcquire: GetDeleteAfterAcquire::<Impl, IMPL_OFFSET>,
            ErrorAdvise: ErrorAdvise::<Impl, IMPL_OFFSET>,
            GetUserInput: GetUserInput::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoAcquireProgressCB as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPhotoAcquireSettingsImpl: Sized {
    fn InitializeFromRegistry(&mut self, pszregistrykey: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetFlags(&mut self, dwphotoacquireflags: u32) -> ::windows::core::Result<()>;
    fn SetOutputFilenameTemplate(&mut self, psztemplate: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetSequencePaddingWidth(&mut self, dwwidth: u32) -> ::windows::core::Result<()>;
    fn SetSequenceZeroPadding(&mut self, fzeropad: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetGroupTag(&mut self, pszgrouptag: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetAcquisitionTime(&mut self, pftacquisitiontime: *const super::super::Foundation::FILETIME) -> ::windows::core::Result<()>;
    fn GetFlags(&mut self) -> ::windows::core::Result<u32>;
    fn GetOutputFilenameTemplate(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetSequencePaddingWidth(&mut self) -> ::windows::core::Result<u32>;
    fn GetSequenceZeroPadding(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetGroupTag(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetAcquisitionTime(&mut self) -> ::windows::core::Result<super::super::Foundation::FILETIME>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPhotoAcquireSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoAcquireSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoAcquireSettingsVtbl {
        unsafe extern "system" fn InitializeFromRegistry<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszregistrykey: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeFromRegistry(::core::mem::transmute_copy(&pszregistrykey)).into()
        }
        unsafe extern "system" fn SetFlags<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwphotoacquireflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFlags(::core::mem::transmute_copy(&dwphotoacquireflags)).into()
        }
        unsafe extern "system" fn SetOutputFilenameTemplate<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztemplate: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutputFilenameTemplate(::core::mem::transmute_copy(&psztemplate)).into()
        }
        unsafe extern "system" fn SetSequencePaddingWidth<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwwidth: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSequencePaddingWidth(::core::mem::transmute_copy(&dwwidth)).into()
        }
        unsafe extern "system" fn SetSequenceZeroPadding<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fzeropad: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSequenceZeroPadding(::core::mem::transmute_copy(&fzeropad)).into()
        }
        unsafe extern "system" fn SetGroupTag<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszgrouptag: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGroupTag(::core::mem::transmute_copy(&pszgrouptag)).into()
        }
        unsafe extern "system" fn SetAcquisitionTime<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pftacquisitiontime: *const super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAcquisitionTime(::core::mem::transmute_copy(&pftacquisitiontime)).into()
        }
        unsafe extern "system" fn GetFlags<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwphotoacquireflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwphotoacquireflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputFilenameTemplate<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtemplate: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputFilenameTemplate() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrtemplate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSequencePaddingWidth<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwwidth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSequencePaddingWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwwidth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSequenceZeroPadding<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfzeropad: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSequenceZeroPadding() {
                ::core::result::Result::Ok(ok__) => {
                    *pfzeropad = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGroupTag<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrgrouptag: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGroupTag() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrgrouptag = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAcquisitionTime<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pftacquisitiontime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAcquisitionTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pftacquisitiontime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            InitializeFromRegistry: InitializeFromRegistry::<Impl, IMPL_OFFSET>,
            SetFlags: SetFlags::<Impl, IMPL_OFFSET>,
            SetOutputFilenameTemplate: SetOutputFilenameTemplate::<Impl, IMPL_OFFSET>,
            SetSequencePaddingWidth: SetSequencePaddingWidth::<Impl, IMPL_OFFSET>,
            SetSequenceZeroPadding: SetSequenceZeroPadding::<Impl, IMPL_OFFSET>,
            SetGroupTag: SetGroupTag::<Impl, IMPL_OFFSET>,
            SetAcquisitionTime: SetAcquisitionTime::<Impl, IMPL_OFFSET>,
            GetFlags: GetFlags::<Impl, IMPL_OFFSET>,
            GetOutputFilenameTemplate: GetOutputFilenameTemplate::<Impl, IMPL_OFFSET>,
            GetSequencePaddingWidth: GetSequencePaddingWidth::<Impl, IMPL_OFFSET>,
            GetSequenceZeroPadding: GetSequenceZeroPadding::<Impl, IMPL_OFFSET>,
            GetGroupTag: GetGroupTag::<Impl, IMPL_OFFSET>,
            GetAcquisitionTime: GetAcquisitionTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoAcquireSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IPhotoAcquireSourceImpl: Sized {
    fn GetFriendlyName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetDeviceIcons(&mut self, nsize: u32, phlargeicon: *mut super::super::UI::WindowsAndMessaging::HICON, phsmallicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows::core::Result<()>;
    fn InitializeItemList(&mut self, fforceenumeration: super::super::Foundation::BOOL, pphotoacquireprogresscb: ::core::option::Option<IPhotoAcquireProgressCB>, pnitemcount: *mut u32) -> ::windows::core::Result<()>;
    fn GetItemCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetItemAt(&mut self, nindex: u32) -> ::windows::core::Result<IPhotoAcquireItem>;
    fn GetPhotoAcquireSettings(&mut self) -> ::windows::core::Result<IPhotoAcquireSettings>;
    fn GetDeviceId(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn BindToObject(&mut self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IPhotoAcquireSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoAcquireSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoAcquireSourceVtbl {
        unsafe extern "system" fn GetFriendlyName<Impl: IPhotoAcquireSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfriendlyname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrfriendlyname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceIcons<Impl: IPhotoAcquireSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nsize: u32, phlargeicon: *mut super::super::UI::WindowsAndMessaging::HICON, phsmallicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceIcons(::core::mem::transmute_copy(&nsize), ::core::mem::transmute_copy(&phlargeicon), ::core::mem::transmute_copy(&phsmallicon)).into()
        }
        unsafe extern "system" fn InitializeItemList<Impl: IPhotoAcquireSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fforceenumeration: super::super::Foundation::BOOL, pphotoacquireprogresscb: ::windows::core::RawPtr, pnitemcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeItemList(::core::mem::transmute_copy(&fforceenumeration), ::core::mem::transmute(&pphotoacquireprogresscb), ::core::mem::transmute_copy(&pnitemcount)).into()
        }
        unsafe extern "system" fn GetItemCount<Impl: IPhotoAcquireSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnitemcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pnitemcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemAt<Impl: IPhotoAcquireSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, ppphotoacquireitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemAt(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppphotoacquireitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPhotoAcquireSettings<Impl: IPhotoAcquireSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppphotoacquiresettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPhotoAcquireSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *ppphotoacquiresettings = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceId<Impl: IPhotoAcquireSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdeviceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdeviceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindToObject<Impl: IPhotoAcquireSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BindToObject(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetFriendlyName: GetFriendlyName::<Impl, IMPL_OFFSET>,
            GetDeviceIcons: GetDeviceIcons::<Impl, IMPL_OFFSET>,
            InitializeItemList: InitializeItemList::<Impl, IMPL_OFFSET>,
            GetItemCount: GetItemCount::<Impl, IMPL_OFFSET>,
            GetItemAt: GetItemAt::<Impl, IMPL_OFFSET>,
            GetPhotoAcquireSettings: GetPhotoAcquireSettings::<Impl, IMPL_OFFSET>,
            GetDeviceId: GetDeviceId::<Impl, IMPL_OFFSET>,
            BindToObject: BindToObject::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoAcquireSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPhotoProgressActionCBImpl: Sized {
    fn DoAction(&mut self, hwndparent: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPhotoProgressActionCBVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoProgressActionCBImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoProgressActionCBVtbl {
        unsafe extern "system" fn DoAction<Impl: IPhotoProgressActionCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DoAction(::core::mem::transmute_copy(&hwndparent)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), DoAction: DoAction::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoProgressActionCB as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IPhotoProgressDialogImpl: Sized {
    fn Create(&mut self, hwndparent: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn GetWindow(&mut self) -> ::windows::core::Result<super::super::Foundation::HWND>;
    fn Destroy(&mut self) -> ::windows::core::Result<()>;
    fn SetTitle(&mut self, psztitle: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn ShowCheckbox(&mut self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fshow: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetCheckboxText(&mut self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtext: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetCheckboxCheck(&mut self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fchecked: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetCheckboxTooltip(&mut self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtooltiptext: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn IsCheckboxChecked(&mut self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetCaption(&mut self, psztitle: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetImage(&mut self, nimagetype: PROGRESS_DIALOG_IMAGE_TYPE, hicon: super::super::UI::WindowsAndMessaging::HICON, hbitmap: super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::Result<()>;
    fn SetPercentComplete(&mut self, npercent: i32) -> ::windows::core::Result<()>;
    fn SetProgressText(&mut self, pszprogresstext: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetActionLinkCallback(&mut self, pphotoprogressactioncb: ::core::option::Option<IPhotoProgressActionCB>) -> ::windows::core::Result<()>;
    fn SetActionLinkText(&mut self, pszcaption: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn ShowActionLink(&mut self, fshow: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn IsCancelled(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetUserInput(&mut self, riidtype: *const ::windows::core::GUID, punknown: ::core::option::Option<::windows::core::IUnknown>, ppropvarresult: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvardefault: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_WindowsAndMessaging"))]
impl IPhotoProgressDialogVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoProgressDialogImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoProgressDialogVtbl {
        unsafe extern "system" fn Create<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Create(::core::mem::transmute_copy(&hwndparent)).into()
        }
        unsafe extern "system" fn GetWindow<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwndprogressdialog: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWindow() {
                ::core::result::Result::Ok(ok__) => {
                    *phwndprogressdialog = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Destroy<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Destroy().into()
        }
        unsafe extern "system" fn SetTitle<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztitle: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTitle(::core::mem::transmute_copy(&psztitle)).into()
        }
        unsafe extern "system" fn ShowCheckbox<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowCheckbox(::core::mem::transmute_copy(&ncheckboxid), ::core::mem::transmute_copy(&fshow)).into()
        }
        unsafe extern "system" fn SetCheckboxText<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckboxText(::core::mem::transmute_copy(&ncheckboxid), ::core::mem::transmute_copy(&pszcheckboxtext)).into()
        }
        unsafe extern "system" fn SetCheckboxCheck<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fchecked: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckboxCheck(::core::mem::transmute_copy(&ncheckboxid), ::core::mem::transmute_copy(&fchecked)).into()
        }
        unsafe extern "system" fn SetCheckboxTooltip<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtooltiptext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckboxTooltip(::core::mem::transmute_copy(&ncheckboxid), ::core::mem::transmute_copy(&pszcheckboxtooltiptext)).into()
        }
        unsafe extern "system" fn IsCheckboxChecked<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pfchecked: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCheckboxChecked(::core::mem::transmute_copy(&ncheckboxid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfchecked = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCaption<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztitle: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCaption(::core::mem::transmute_copy(&psztitle)).into()
        }
        unsafe extern "system" fn SetImage<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nimagetype: PROGRESS_DIALOG_IMAGE_TYPE, hicon: super::super::UI::WindowsAndMessaging::HICON, hbitmap: super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetImage(::core::mem::transmute_copy(&nimagetype), ::core::mem::transmute_copy(&hicon), ::core::mem::transmute_copy(&hbitmap)).into()
        }
        unsafe extern "system" fn SetPercentComplete<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, npercent: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPercentComplete(::core::mem::transmute_copy(&npercent)).into()
        }
        unsafe extern "system" fn SetProgressText<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszprogresstext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProgressText(::core::mem::transmute_copy(&pszprogresstext)).into()
        }
        unsafe extern "system" fn SetActionLinkCallback<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphotoprogressactioncb: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetActionLinkCallback(::core::mem::transmute(&pphotoprogressactioncb)).into()
        }
        unsafe extern "system" fn SetActionLinkText<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcaption: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetActionLinkText(::core::mem::transmute_copy(&pszcaption)).into()
        }
        unsafe extern "system" fn ShowActionLink<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowActionLink(::core::mem::transmute_copy(&fshow)).into()
        }
        unsafe extern "system" fn IsCancelled<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfcancelled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCancelled() {
                ::core::result::Result::Ok(ok__) => {
                    *pfcancelled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserInput<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riidtype: *const ::windows::core::GUID, punknown: *mut ::core::ffi::c_void, ppropvarresult: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvardefault: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetUserInput(::core::mem::transmute_copy(&riidtype), ::core::mem::transmute(&punknown), ::core::mem::transmute_copy(&ppropvarresult), ::core::mem::transmute_copy(&ppropvardefault)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            GetWindow: GetWindow::<Impl, IMPL_OFFSET>,
            Destroy: Destroy::<Impl, IMPL_OFFSET>,
            SetTitle: SetTitle::<Impl, IMPL_OFFSET>,
            ShowCheckbox: ShowCheckbox::<Impl, IMPL_OFFSET>,
            SetCheckboxText: SetCheckboxText::<Impl, IMPL_OFFSET>,
            SetCheckboxCheck: SetCheckboxCheck::<Impl, IMPL_OFFSET>,
            SetCheckboxTooltip: SetCheckboxTooltip::<Impl, IMPL_OFFSET>,
            IsCheckboxChecked: IsCheckboxChecked::<Impl, IMPL_OFFSET>,
            SetCaption: SetCaption::<Impl, IMPL_OFFSET>,
            SetImage: SetImage::<Impl, IMPL_OFFSET>,
            SetPercentComplete: SetPercentComplete::<Impl, IMPL_OFFSET>,
            SetProgressText: SetProgressText::<Impl, IMPL_OFFSET>,
            SetActionLinkCallback: SetActionLinkCallback::<Impl, IMPL_OFFSET>,
            SetActionLinkText: SetActionLinkText::<Impl, IMPL_OFFSET>,
            ShowActionLink: ShowActionLink::<Impl, IMPL_OFFSET>,
            IsCancelled: IsCancelled::<Impl, IMPL_OFFSET>,
            GetUserInput: GetUserInput::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoProgressDialog as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IUserInputStringImpl: Sized {
    fn GetSubmitButtonText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetPrompt(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetStringId(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetStringType(&mut self) -> ::windows::core::Result<USER_INPUT_STRING_TYPE>;
    fn GetTooltipText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetMaxLength(&mut self) -> ::windows::core::Result<u32>;
    fn GetDefault(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetMruCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetMruEntryAt(&mut self, nindex: u32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetImage(&mut self, nsize: u32, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP, phicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IUserInputStringVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserInputStringImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserInputStringVtbl {
        unsafe extern "system" fn GetSubmitButtonText<Impl: IUserInputStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsubmitbuttontext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubmitButtonText() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrsubmitbuttontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrompt<Impl: IUserInputStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprompttitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrompt() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprompttitle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringId<Impl: IUserInputStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstringid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStringId() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrstringid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringType<Impl: IUserInputStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnstringtype: *mut USER_INPUT_STRING_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStringType() {
                ::core::result::Result::Ok(ok__) => {
                    *pnstringtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTooltipText<Impl: IUserInputStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtooltiptext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTooltipText() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrtooltiptext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxLength<Impl: IUserInputStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchmaxlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxLength() {
                ::core::result::Result::Ok(ok__) => {
                    *pcchmaxlength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefault<Impl: IUserInputStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdefault: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdefault = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMruCount<Impl: IUserInputStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnmrucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMruCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pnmrucount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMruEntryAt<Impl: IUserInputStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, pbstrmruentry: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMruEntryAt(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrmruentry = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImage<Impl: IUserInputStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nsize: u32, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP, phicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetImage(::core::mem::transmute_copy(&nsize), ::core::mem::transmute_copy(&phbitmap), ::core::mem::transmute_copy(&phicon)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSubmitButtonText: GetSubmitButtonText::<Impl, IMPL_OFFSET>,
            GetPrompt: GetPrompt::<Impl, IMPL_OFFSET>,
            GetStringId: GetStringId::<Impl, IMPL_OFFSET>,
            GetStringType: GetStringType::<Impl, IMPL_OFFSET>,
            GetTooltipText: GetTooltipText::<Impl, IMPL_OFFSET>,
            GetMaxLength: GetMaxLength::<Impl, IMPL_OFFSET>,
            GetDefault: GetDefault::<Impl, IMPL_OFFSET>,
            GetMruCount: GetMruCount::<Impl, IMPL_OFFSET>,
            GetMruEntryAt: GetMruEntryAt::<Impl, IMPL_OFFSET>,
            GetImage: GetImage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserInputString as ::windows::core::Interface>::IID
    }
}
