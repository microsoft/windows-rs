#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IPhotoAcquireImpl: Sized {
    fn CreatePhotoSource();
    fn Acquire();
    fn EnumResults();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IPhotoAcquireVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoAcquireImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoAcquireVtbl {
        unsafe extern "system" fn CreatePhotoSource<Impl: IPhotoAcquireImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdevice: super::super::Foundation::PWSTR, ppphotoacquiresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Acquire<Impl: IPhotoAcquireImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphotoacquiresource: ::windows::core::RawPtr, fshowprogress: super::super::Foundation::BOOL, hwndparent: super::super::Foundation::HWND, pszapplicationname: super::super::Foundation::PWSTR, pphotoacquireprogresscb: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumResults<Impl: IPhotoAcquireImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumfilepaths: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreatePhotoSource::<Impl, IMPL_OFFSET>, Acquire::<Impl, IMPL_OFFSET>, EnumResults::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoAcquire as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPhotoAcquireDeviceSelectionDialogImpl: Sized {
    fn SetTitle();
    fn SetSubmitButtonText();
    fn DoModal();
}
#[cfg(feature = "Win32_Foundation")]
impl IPhotoAcquireDeviceSelectionDialogVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoAcquireDeviceSelectionDialogImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoAcquireDeviceSelectionDialogVtbl {
        unsafe extern "system" fn SetTitle<Impl: IPhotoAcquireDeviceSelectionDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztitle: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSubmitButtonText<Impl: IPhotoAcquireDeviceSelectionDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubmitbuttontext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DoModal<Impl: IPhotoAcquireDeviceSelectionDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwdeviceflags: u32, pbstrdeviceid: *mut super::super::Foundation::BSTR, pndevicetype: *mut DEVICE_SELECTION_DEVICE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetTitle::<Impl, IMPL_OFFSET>, SetSubmitButtonText::<Impl, IMPL_OFFSET>, DoModal::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoAcquireDeviceSelectionDialog as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IPhotoAcquireItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoAcquireItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoAcquireItemVtbl {
        unsafe extern "system" fn GetItemName<Impl: IPhotoAcquireItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstritemname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetThumbnail<Impl: IPhotoAcquireItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sizethumbnail: super::super::Foundation::SIZE, phbmpthumbnail: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperty<Impl: IPhotoAcquireItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProperty<Impl: IPhotoAcquireItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStream<Impl: IPhotoAcquireItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CanDelete<Impl: IPhotoAcquireItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfcandelete: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IPhotoAcquireItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSubItemCount<Impl: IPhotoAcquireItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSubItemAt<Impl: IPhotoAcquireItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nitemindex: u32, ppphotoacquireitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetItemName::<Impl, IMPL_OFFSET>, GetThumbnail::<Impl, IMPL_OFFSET>, GetProperty::<Impl, IMPL_OFFSET>, SetProperty::<Impl, IMPL_OFFSET>, GetStream::<Impl, IMPL_OFFSET>, CanDelete::<Impl, IMPL_OFFSET>, Delete::<Impl, IMPL_OFFSET>, GetSubItemCount::<Impl, IMPL_OFFSET>, GetSubItemAt::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoAcquireItem as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPhotoAcquireOptionsDialogImpl: Sized {
    fn Initialize();
    fn Create();
    fn Destroy();
    fn DoModal();
    fn SaveData();
}
#[cfg(feature = "Win32_Foundation")]
impl IPhotoAcquireOptionsDialogVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoAcquireOptionsDialogImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoAcquireOptionsDialogVtbl {
        unsafe extern "system" fn Initialize<Impl: IPhotoAcquireOptionsDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszregistryroot: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Create<Impl: IPhotoAcquireOptionsDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, phwnddialog: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Destroy<Impl: IPhotoAcquireOptionsDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DoModal<Impl: IPhotoAcquireOptionsDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ppnreturncode: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SaveData<Impl: IPhotoAcquireOptionsDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, Create::<Impl, IMPL_OFFSET>, Destroy::<Impl, IMPL_OFFSET>, DoModal::<Impl, IMPL_OFFSET>, SaveData::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoAcquireOptionsDialog as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IPhotoAcquirePluginImpl: Sized {
    fn Initialize();
    fn ProcessItem();
    fn TransferComplete();
    fn DisplayConfigureDialog();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IPhotoAcquirePluginVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoAcquirePluginImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoAcquirePluginVtbl {
        unsafe extern "system" fn Initialize<Impl: IPhotoAcquirePluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphotoacquiresource: ::windows::core::RawPtr, pphotoacquireprogresscb: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProcessItem<Impl: IPhotoAcquirePluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwacquirestage: u32, pphotoacquireitem: ::windows::core::RawPtr, poriginalitemstream: ::windows::core::RawPtr, pszfinalfilename: super::super::Foundation::PWSTR, ppropertystore: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TransferComplete<Impl: IPhotoAcquirePluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisplayConfigureDialog<Impl: IPhotoAcquirePluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, ProcessItem::<Impl, IMPL_OFFSET>, TransferComplete::<Impl, IMPL_OFFSET>, DisplayConfigureDialog::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoAcquirePlugin as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IPhotoAcquireProgressCBVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoAcquireProgressCBImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoAcquireProgressCBVtbl {
        unsafe extern "system" fn Cancelled<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfcancelled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartEnumeration<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphotoacquiresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FoundItem<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphotoacquireitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndEnumeration<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartTransfer<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphotoacquiresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartItemTransfer<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nitemindex: u32, pphotoacquireitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DirectoryCreated<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdirectory: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateTransferPercent<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foverall: super::super::Foundation::BOOL, npercent: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndItemTransfer<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nitemindex: u32, pphotoacquireitem: ::windows::core::RawPtr, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndTransfer<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartDelete<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphotoacquiresource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartItemDelete<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nitemindex: u32, pphotoacquireitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateDeletePercent<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, npercent: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndItemDelete<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nitemindex: u32, pphotoacquireitem: ::windows::core::RawPtr, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndDelete<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndSession<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeleteAfterAcquire<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfdeleteafteracquire: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ErrorAdvise<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, pszerrormessage: super::super::Foundation::PWSTR, nmessagetype: ERROR_ADVISE_MESSAGE_TYPE, pnerroradviseresult: *mut ERROR_ADVISE_RESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUserInput<Impl: IPhotoAcquireProgressCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riidtype: *const ::windows::core::GUID, punknown: *mut ::core::ffi::c_void, ppropvarresult: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvardefault: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            Cancelled::<Impl, IMPL_OFFSET>,
            StartEnumeration::<Impl, IMPL_OFFSET>,
            FoundItem::<Impl, IMPL_OFFSET>,
            EndEnumeration::<Impl, IMPL_OFFSET>,
            StartTransfer::<Impl, IMPL_OFFSET>,
            StartItemTransfer::<Impl, IMPL_OFFSET>,
            DirectoryCreated::<Impl, IMPL_OFFSET>,
            UpdateTransferPercent::<Impl, IMPL_OFFSET>,
            EndItemTransfer::<Impl, IMPL_OFFSET>,
            EndTransfer::<Impl, IMPL_OFFSET>,
            StartDelete::<Impl, IMPL_OFFSET>,
            StartItemDelete::<Impl, IMPL_OFFSET>,
            UpdateDeletePercent::<Impl, IMPL_OFFSET>,
            EndItemDelete::<Impl, IMPL_OFFSET>,
            EndDelete::<Impl, IMPL_OFFSET>,
            EndSession::<Impl, IMPL_OFFSET>,
            GetDeleteAfterAcquire::<Impl, IMPL_OFFSET>,
            ErrorAdvise::<Impl, IMPL_OFFSET>,
            GetUserInput::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoAcquireProgressCB as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IPhotoAcquireSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoAcquireSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoAcquireSettingsVtbl {
        unsafe extern "system" fn InitializeFromRegistry<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszregistrykey: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFlags<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwphotoacquireflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOutputFilenameTemplate<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztemplate: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSequencePaddingWidth<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwwidth: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSequenceZeroPadding<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fzeropad: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGroupTag<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszgrouptag: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAcquisitionTime<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pftacquisitiontime: *const super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFlags<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwphotoacquireflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputFilenameTemplate<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtemplate: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSequencePaddingWidth<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwwidth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSequenceZeroPadding<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfzeropad: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGroupTag<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrgrouptag: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAcquisitionTime<Impl: IPhotoAcquireSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pftacquisitiontime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            InitializeFromRegistry::<Impl, IMPL_OFFSET>,
            SetFlags::<Impl, IMPL_OFFSET>,
            SetOutputFilenameTemplate::<Impl, IMPL_OFFSET>,
            SetSequencePaddingWidth::<Impl, IMPL_OFFSET>,
            SetSequenceZeroPadding::<Impl, IMPL_OFFSET>,
            SetGroupTag::<Impl, IMPL_OFFSET>,
            SetAcquisitionTime::<Impl, IMPL_OFFSET>,
            GetFlags::<Impl, IMPL_OFFSET>,
            GetOutputFilenameTemplate::<Impl, IMPL_OFFSET>,
            GetSequencePaddingWidth::<Impl, IMPL_OFFSET>,
            GetSequenceZeroPadding::<Impl, IMPL_OFFSET>,
            GetGroupTag::<Impl, IMPL_OFFSET>,
            GetAcquisitionTime::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoAcquireSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IPhotoAcquireSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoAcquireSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoAcquireSourceVtbl {
        unsafe extern "system" fn GetFriendlyName<Impl: IPhotoAcquireSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfriendlyname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceIcons<Impl: IPhotoAcquireSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nsize: u32, phlargeicon: *mut super::super::UI::WindowsAndMessaging::HICON, phsmallicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeItemList<Impl: IPhotoAcquireSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fforceenumeration: super::super::Foundation::BOOL, pphotoacquireprogresscb: ::windows::core::RawPtr, pnitemcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetItemCount<Impl: IPhotoAcquireSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnitemcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetItemAt<Impl: IPhotoAcquireSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, ppphotoacquireitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPhotoAcquireSettings<Impl: IPhotoAcquireSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppphotoacquiresettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceId<Impl: IPhotoAcquireSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdeviceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BindToObject<Impl: IPhotoAcquireSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFriendlyName::<Impl, IMPL_OFFSET>, GetDeviceIcons::<Impl, IMPL_OFFSET>, InitializeItemList::<Impl, IMPL_OFFSET>, GetItemCount::<Impl, IMPL_OFFSET>, GetItemAt::<Impl, IMPL_OFFSET>, GetPhotoAcquireSettings::<Impl, IMPL_OFFSET>, GetDeviceId::<Impl, IMPL_OFFSET>, BindToObject::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoAcquireSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPhotoProgressActionCBImpl: Sized {
    fn DoAction();
}
#[cfg(feature = "Win32_Foundation")]
impl IPhotoProgressActionCBVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoProgressActionCBImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoProgressActionCBVtbl {
        unsafe extern "system" fn DoAction<Impl: IPhotoProgressActionCBImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, DoAction::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoProgressActionCB as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_WindowsAndMessaging"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_WindowsAndMessaging"))]
impl IPhotoProgressDialogVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhotoProgressDialogImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhotoProgressDialogVtbl {
        unsafe extern "system" fn Create<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWindow<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwndprogressdialog: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Destroy<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTitle<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztitle: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ShowCheckbox<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCheckboxText<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCheckboxCheck<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fchecked: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCheckboxTooltip<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtooltiptext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsCheckboxChecked<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pfchecked: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCaption<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztitle: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetImage<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nimagetype: PROGRESS_DIALOG_IMAGE_TYPE, hicon: super::super::UI::WindowsAndMessaging::HICON, hbitmap: super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPercentComplete<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, npercent: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProgressText<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszprogresstext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetActionLinkCallback<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphotoprogressactioncb: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetActionLinkText<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcaption: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ShowActionLink<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsCancelled<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfcancelled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUserInput<Impl: IPhotoProgressDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riidtype: *const ::windows::core::GUID, punknown: *mut ::core::ffi::c_void, ppropvarresult: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvardefault: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            Create::<Impl, IMPL_OFFSET>,
            GetWindow::<Impl, IMPL_OFFSET>,
            Destroy::<Impl, IMPL_OFFSET>,
            SetTitle::<Impl, IMPL_OFFSET>,
            ShowCheckbox::<Impl, IMPL_OFFSET>,
            SetCheckboxText::<Impl, IMPL_OFFSET>,
            SetCheckboxCheck::<Impl, IMPL_OFFSET>,
            SetCheckboxTooltip::<Impl, IMPL_OFFSET>,
            IsCheckboxChecked::<Impl, IMPL_OFFSET>,
            SetCaption::<Impl, IMPL_OFFSET>,
            SetImage::<Impl, IMPL_OFFSET>,
            SetPercentComplete::<Impl, IMPL_OFFSET>,
            SetProgressText::<Impl, IMPL_OFFSET>,
            SetActionLinkCallback::<Impl, IMPL_OFFSET>,
            SetActionLinkText::<Impl, IMPL_OFFSET>,
            ShowActionLink::<Impl, IMPL_OFFSET>,
            IsCancelled::<Impl, IMPL_OFFSET>,
            GetUserInput::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoProgressDialog as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IUserInputStringVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserInputStringImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserInputStringVtbl {
        unsafe extern "system" fn GetSubmitButtonText<Impl: IUserInputStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsubmitbuttontext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPrompt<Impl: IUserInputStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprompttitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStringId<Impl: IUserInputStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstringid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStringType<Impl: IUserInputStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnstringtype: *mut USER_INPUT_STRING_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTooltipText<Impl: IUserInputStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtooltiptext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaxLength<Impl: IUserInputStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchmaxlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDefault<Impl: IUserInputStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdefault: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMruCount<Impl: IUserInputStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnmrucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMruEntryAt<Impl: IUserInputStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, pbstrmruentry: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetImage<Impl: IUserInputStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nsize: u32, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP, phicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetSubmitButtonText::<Impl, IMPL_OFFSET>,
            GetPrompt::<Impl, IMPL_OFFSET>,
            GetStringId::<Impl, IMPL_OFFSET>,
            GetStringType::<Impl, IMPL_OFFSET>,
            GetTooltipText::<Impl, IMPL_OFFSET>,
            GetMaxLength::<Impl, IMPL_OFFSET>,
            GetDefault::<Impl, IMPL_OFFSET>,
            GetMruCount::<Impl, IMPL_OFFSET>,
            GetMruEntryAt::<Impl, IMPL_OFFSET>,
            GetImage::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserInputString as ::windows::core::Interface>::IID
    }
}
