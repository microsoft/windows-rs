#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IPhotoAcquire_Impl: Sized {
    fn CreatePhotoSource(&self, pszdevice: &::windows::core::PCWSTR) -> ::windows::core::Result<IPhotoAcquireSource>;
    fn Acquire(&self, pphotoacquiresource: ::core::option::Option<&IPhotoAcquireSource>, fshowprogress: super::super::Foundation::BOOL, hwndparent: super::super::Foundation::HWND, pszapplicationname: &::windows::core::PCWSTR, pphotoacquireprogresscb: ::core::option::Option<&IPhotoAcquireProgressCB>) -> ::windows::core::Result<()>;
    fn EnumResults(&self) -> ::windows::core::Result<super::super::System::Com::IEnumString>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IPhotoAcquire {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IPhotoAcquire_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquire_Impl, const OFFSET: isize>() -> IPhotoAcquire_Vtbl {
        unsafe extern "system" fn CreatePhotoSource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquire_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdevice: ::windows::core::PCWSTR, ppphotoacquiresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePhotoSource(::core::mem::transmute(&pszdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppphotoacquiresource, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Acquire<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquire_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphotoacquiresource: *mut ::core::ffi::c_void, fshowprogress: super::super::Foundation::BOOL, hwndparent: super::super::Foundation::HWND, pszapplicationname: ::windows::core::PCWSTR, pphotoacquireprogresscb: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Acquire(::windows::core::from_raw_borrowed(&pphotoacquiresource), ::core::mem::transmute_copy(&fshowprogress), ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute(&pszapplicationname), ::windows::core::from_raw_borrowed(&pphotoacquireprogresscb)).into()
        }
        unsafe extern "system" fn EnumResults<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquire_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumfilepaths: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumResults() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumfilepaths, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreatePhotoSource: CreatePhotoSource::<Identity, Impl, OFFSET>,
            Acquire: Acquire::<Identity, Impl, OFFSET>,
            EnumResults: EnumResults::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoAcquire as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPhotoAcquireDeviceSelectionDialog_Impl: Sized {
    fn SetTitle(&self, psztitle: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn SetSubmitButtonText(&self, pszsubmitbuttontext: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn DoModal(&self, hwndparent: super::super::Foundation::HWND, dwdeviceflags: u32, pbstrdeviceid: *mut ::windows::core::BSTR, pndevicetype: *mut DEVICE_SELECTION_DEVICE_TYPE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IPhotoAcquireDeviceSelectionDialog {}
#[cfg(feature = "Win32_Foundation")]
impl IPhotoAcquireDeviceSelectionDialog_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireDeviceSelectionDialog_Impl, const OFFSET: isize>() -> IPhotoAcquireDeviceSelectionDialog_Vtbl {
        unsafe extern "system" fn SetTitle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireDeviceSelectionDialog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztitle: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTitle(::core::mem::transmute(&psztitle)).into()
        }
        unsafe extern "system" fn SetSubmitButtonText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireDeviceSelectionDialog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubmitbuttontext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSubmitButtonText(::core::mem::transmute(&pszsubmitbuttontext)).into()
        }
        unsafe extern "system" fn DoModal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireDeviceSelectionDialog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwdeviceflags: u32, pbstrdeviceid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pndevicetype: *mut DEVICE_SELECTION_DEVICE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DoModal(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwdeviceflags), ::core::mem::transmute_copy(&pbstrdeviceid), ::core::mem::transmute_copy(&pndevicetype)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetTitle: SetTitle::<Identity, Impl, OFFSET>,
            SetSubmitButtonText: SetSubmitButtonText::<Identity, Impl, OFFSET>,
            DoModal: DoModal::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoAcquireDeviceSelectionDialog as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IPhotoAcquireItem_Impl: Sized {
    fn GetItemName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn GetThumbnail(&self, sizethumbnail: &super::super::Foundation::SIZE) -> ::windows::core::Result<super::super::Graphics::Gdi::HBITMAP>;
    fn GetProperty(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn SetProperty(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn GetStream(&self) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn CanDelete(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn Delete(&self) -> ::windows::core::Result<()>;
    fn GetSubItemCount(&self) -> ::windows::core::Result<u32>;
    fn GetSubItemAt(&self, nitemindex: u32) -> ::windows::core::Result<IPhotoAcquireItem>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows::core::RuntimeName for IPhotoAcquireItem {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IPhotoAcquireItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireItem_Impl, const OFFSET: isize>() -> IPhotoAcquireItem_Vtbl {
        unsafe extern "system" fn GetItemName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstritemname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItemName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstritemname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThumbnail<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sizethumbnail: super::super::Foundation::SIZE, phbmpthumbnail: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetThumbnail(::core::mem::transmute(&sizethumbnail)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phbmpthumbnail, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperty(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pv, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&pv)).into()
        }
        unsafe extern "system" fn GetStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstream, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanDelete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfcandelete: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanDelete() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcandelete, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete().into()
        }
        unsafe extern "system" fn GetSubItemCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSubItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pncount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubItemAt<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nitemindex: u32, ppphotoacquireitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSubItemAt(::core::mem::transmute_copy(&nitemindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppphotoacquireitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetItemName: GetItemName::<Identity, Impl, OFFSET>,
            GetThumbnail: GetThumbnail::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            GetStream: GetStream::<Identity, Impl, OFFSET>,
            CanDelete: CanDelete::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            GetSubItemCount: GetSubItemCount::<Identity, Impl, OFFSET>,
            GetSubItemAt: GetSubItemAt::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoAcquireItem as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPhotoAcquireOptionsDialog_Impl: Sized {
    fn Initialize(&self, pszregistryroot: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn Create(&self, hwndparent: super::super::Foundation::HWND) -> ::windows::core::Result<super::super::Foundation::HWND>;
    fn Destroy(&self) -> ::windows::core::Result<()>;
    fn DoModal(&self, hwndparent: super::super::Foundation::HWND, ppnreturncode: *mut isize) -> ::windows::core::Result<()>;
    fn SaveData(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IPhotoAcquireOptionsDialog {}
#[cfg(feature = "Win32_Foundation")]
impl IPhotoAcquireOptionsDialog_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireOptionsDialog_Impl, const OFFSET: isize>() -> IPhotoAcquireOptionsDialog_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireOptionsDialog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszregistryroot: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute(&pszregistryroot)).into()
        }
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireOptionsDialog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, phwnddialog: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Create(::core::mem::transmute_copy(&hwndparent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phwnddialog, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Destroy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireOptionsDialog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Destroy().into()
        }
        unsafe extern "system" fn DoModal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireOptionsDialog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ppnreturncode: *mut isize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DoModal(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&ppnreturncode)).into()
        }
        unsafe extern "system" fn SaveData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireOptionsDialog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SaveData().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Create: Create::<Identity, Impl, OFFSET>,
            Destroy: Destroy::<Identity, Impl, OFFSET>,
            DoModal: DoModal::<Identity, Impl, OFFSET>,
            SaveData: SaveData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoAcquireOptionsDialog as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_UI_Shell_PropertiesSystem\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IPhotoAcquirePlugin_Impl: Sized {
    fn Initialize(&self, pphotoacquiresource: ::core::option::Option<&IPhotoAcquireSource>, pphotoacquireprogresscb: ::core::option::Option<&IPhotoAcquireProgressCB>) -> ::windows::core::Result<()>;
    fn ProcessItem(&self, dwacquirestage: u32, pphotoacquireitem: ::core::option::Option<&IPhotoAcquireItem>, poriginalitemstream: ::core::option::Option<&super::super::System::Com::IStream>, pszfinalfilename: &::windows::core::PCWSTR, ppropertystore: ::core::option::Option<&super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows::core::Result<()>;
    fn TransferComplete(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn DisplayConfigureDialog(&self, hwndparent: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows::core::RuntimeName for IPhotoAcquirePlugin {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IPhotoAcquirePlugin_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquirePlugin_Impl, const OFFSET: isize>() -> IPhotoAcquirePlugin_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquirePlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphotoacquiresource: *mut ::core::ffi::c_void, pphotoacquireprogresscb: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows::core::from_raw_borrowed(&pphotoacquiresource), ::windows::core::from_raw_borrowed(&pphotoacquireprogresscb)).into()
        }
        unsafe extern "system" fn ProcessItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquirePlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwacquirestage: u32, pphotoacquireitem: *mut ::core::ffi::c_void, poriginalitemstream: *mut ::core::ffi::c_void, pszfinalfilename: ::windows::core::PCWSTR, ppropertystore: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ProcessItem(::core::mem::transmute_copy(&dwacquirestage), ::windows::core::from_raw_borrowed(&pphotoacquireitem), ::windows::core::from_raw_borrowed(&poriginalitemstream), ::core::mem::transmute(&pszfinalfilename), ::windows::core::from_raw_borrowed(&ppropertystore)).into()
        }
        unsafe extern "system" fn TransferComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquirePlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TransferComplete(::core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn DisplayConfigureDialog<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquirePlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisplayConfigureDialog(::core::mem::transmute_copy(&hwndparent)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            ProcessItem: ProcessItem::<Identity, Impl, OFFSET>,
            TransferComplete: TransferComplete::<Identity, Impl, OFFSET>,
            DisplayConfigureDialog: DisplayConfigureDialog::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoAcquirePlugin as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IPhotoAcquireProgressCB_Impl: Sized {
    fn Cancelled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn StartEnumeration(&self, pphotoacquiresource: ::core::option::Option<&IPhotoAcquireSource>) -> ::windows::core::Result<()>;
    fn FoundItem(&self, pphotoacquireitem: ::core::option::Option<&IPhotoAcquireItem>) -> ::windows::core::Result<()>;
    fn EndEnumeration(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn StartTransfer(&self, pphotoacquiresource: ::core::option::Option<&IPhotoAcquireSource>) -> ::windows::core::Result<()>;
    fn StartItemTransfer(&self, nitemindex: u32, pphotoacquireitem: ::core::option::Option<&IPhotoAcquireItem>) -> ::windows::core::Result<()>;
    fn DirectoryCreated(&self, pszdirectory: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn UpdateTransferPercent(&self, foverall: super::super::Foundation::BOOL, npercent: u32) -> ::windows::core::Result<()>;
    fn EndItemTransfer(&self, nitemindex: u32, pphotoacquireitem: ::core::option::Option<&IPhotoAcquireItem>, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn EndTransfer(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn StartDelete(&self, pphotoacquiresource: ::core::option::Option<&IPhotoAcquireSource>) -> ::windows::core::Result<()>;
    fn StartItemDelete(&self, nitemindex: u32, pphotoacquireitem: ::core::option::Option<&IPhotoAcquireItem>) -> ::windows::core::Result<()>;
    fn UpdateDeletePercent(&self, npercent: u32) -> ::windows::core::Result<()>;
    fn EndItemDelete(&self, nitemindex: u32, pphotoacquireitem: ::core::option::Option<&IPhotoAcquireItem>, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn EndDelete(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn EndSession(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn GetDeleteAfterAcquire(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn ErrorAdvise(&self, hr: ::windows::core::HRESULT, pszerrormessage: &::windows::core::PCWSTR, nmessagetype: ERROR_ADVISE_MESSAGE_TYPE) -> ::windows::core::Result<ERROR_ADVISE_RESULT>;
    fn GetUserInput(&self, riidtype: *const ::windows::core::GUID, punknown: ::core::option::Option<&::windows::core::IUnknown>, ppropvarresult: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvardefault: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::RuntimeName for IPhotoAcquireProgressCB {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl IPhotoAcquireProgressCB_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>() -> IPhotoAcquireProgressCB_Vtbl {
        unsafe extern "system" fn Cancelled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfcancelled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Cancelled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcancelled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartEnumeration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphotoacquiresource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartEnumeration(::windows::core::from_raw_borrowed(&pphotoacquiresource)).into()
        }
        unsafe extern "system" fn FoundItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphotoacquireitem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FoundItem(::windows::core::from_raw_borrowed(&pphotoacquireitem)).into()
        }
        unsafe extern "system" fn EndEnumeration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndEnumeration(::core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn StartTransfer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphotoacquiresource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartTransfer(::windows::core::from_raw_borrowed(&pphotoacquiresource)).into()
        }
        unsafe extern "system" fn StartItemTransfer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nitemindex: u32, pphotoacquireitem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartItemTransfer(::core::mem::transmute_copy(&nitemindex), ::windows::core::from_raw_borrowed(&pphotoacquireitem)).into()
        }
        unsafe extern "system" fn DirectoryCreated<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdirectory: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DirectoryCreated(::core::mem::transmute(&pszdirectory)).into()
        }
        unsafe extern "system" fn UpdateTransferPercent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foverall: super::super::Foundation::BOOL, npercent: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateTransferPercent(::core::mem::transmute_copy(&foverall), ::core::mem::transmute_copy(&npercent)).into()
        }
        unsafe extern "system" fn EndItemTransfer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nitemindex: u32, pphotoacquireitem: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndItemTransfer(::core::mem::transmute_copy(&nitemindex), ::windows::core::from_raw_borrowed(&pphotoacquireitem), ::core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn EndTransfer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndTransfer(::core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn StartDelete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphotoacquiresource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartDelete(::windows::core::from_raw_borrowed(&pphotoacquiresource)).into()
        }
        unsafe extern "system" fn StartItemDelete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nitemindex: u32, pphotoacquireitem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartItemDelete(::core::mem::transmute_copy(&nitemindex), ::windows::core::from_raw_borrowed(&pphotoacquireitem)).into()
        }
        unsafe extern "system" fn UpdateDeletePercent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, npercent: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateDeletePercent(::core::mem::transmute_copy(&npercent)).into()
        }
        unsafe extern "system" fn EndItemDelete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nitemindex: u32, pphotoacquireitem: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndItemDelete(::core::mem::transmute_copy(&nitemindex), ::windows::core::from_raw_borrowed(&pphotoacquireitem), ::core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn EndDelete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndDelete(::core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn EndSession<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndSession(::core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn GetDeleteAfterAcquire<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfdeleteafteracquire: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDeleteAfterAcquire() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfdeleteafteracquire, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorAdvise<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, pszerrormessage: ::windows::core::PCWSTR, nmessagetype: ERROR_ADVISE_MESSAGE_TYPE, pnerroradviseresult: *mut ERROR_ADVISE_RESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ErrorAdvise(::core::mem::transmute_copy(&hr), ::core::mem::transmute(&pszerrormessage), ::core::mem::transmute_copy(&nmessagetype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnerroradviseresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserInput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireProgressCB_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riidtype: *const ::windows::core::GUID, punknown: *mut ::core::ffi::c_void, ppropvarresult: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvardefault: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetUserInput(::core::mem::transmute_copy(&riidtype), ::windows::core::from_raw_borrowed(&punknown), ::core::mem::transmute_copy(&ppropvarresult), ::core::mem::transmute_copy(&ppropvardefault)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Cancelled: Cancelled::<Identity, Impl, OFFSET>,
            StartEnumeration: StartEnumeration::<Identity, Impl, OFFSET>,
            FoundItem: FoundItem::<Identity, Impl, OFFSET>,
            EndEnumeration: EndEnumeration::<Identity, Impl, OFFSET>,
            StartTransfer: StartTransfer::<Identity, Impl, OFFSET>,
            StartItemTransfer: StartItemTransfer::<Identity, Impl, OFFSET>,
            DirectoryCreated: DirectoryCreated::<Identity, Impl, OFFSET>,
            UpdateTransferPercent: UpdateTransferPercent::<Identity, Impl, OFFSET>,
            EndItemTransfer: EndItemTransfer::<Identity, Impl, OFFSET>,
            EndTransfer: EndTransfer::<Identity, Impl, OFFSET>,
            StartDelete: StartDelete::<Identity, Impl, OFFSET>,
            StartItemDelete: StartItemDelete::<Identity, Impl, OFFSET>,
            UpdateDeletePercent: UpdateDeletePercent::<Identity, Impl, OFFSET>,
            EndItemDelete: EndItemDelete::<Identity, Impl, OFFSET>,
            EndDelete: EndDelete::<Identity, Impl, OFFSET>,
            EndSession: EndSession::<Identity, Impl, OFFSET>,
            GetDeleteAfterAcquire: GetDeleteAfterAcquire::<Identity, Impl, OFFSET>,
            ErrorAdvise: ErrorAdvise::<Identity, Impl, OFFSET>,
            GetUserInput: GetUserInput::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoAcquireProgressCB as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPhotoAcquireSettings_Impl: Sized {
    fn InitializeFromRegistry(&self, pszregistrykey: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn SetFlags(&self, dwphotoacquireflags: u32) -> ::windows::core::Result<()>;
    fn SetOutputFilenameTemplate(&self, psztemplate: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn SetSequencePaddingWidth(&self, dwwidth: u32) -> ::windows::core::Result<()>;
    fn SetSequenceZeroPadding(&self, fzeropad: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetGroupTag(&self, pszgrouptag: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn SetAcquisitionTime(&self, pftacquisitiontime: *const super::super::Foundation::FILETIME) -> ::windows::core::Result<()>;
    fn GetFlags(&self) -> ::windows::core::Result<u32>;
    fn GetOutputFilenameTemplate(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn GetSequencePaddingWidth(&self) -> ::windows::core::Result<u32>;
    fn GetSequenceZeroPadding(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetGroupTag(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn GetAcquisitionTime(&self) -> ::windows::core::Result<super::super::Foundation::FILETIME>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IPhotoAcquireSettings {}
#[cfg(feature = "Win32_Foundation")]
impl IPhotoAcquireSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: isize>() -> IPhotoAcquireSettings_Vtbl {
        unsafe extern "system" fn InitializeFromRegistry<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszregistrykey: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializeFromRegistry(::core::mem::transmute(&pszregistrykey)).into()
        }
        unsafe extern "system" fn SetFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwphotoacquireflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFlags(::core::mem::transmute_copy(&dwphotoacquireflags)).into()
        }
        unsafe extern "system" fn SetOutputFilenameTemplate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztemplate: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOutputFilenameTemplate(::core::mem::transmute(&psztemplate)).into()
        }
        unsafe extern "system" fn SetSequencePaddingWidth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwwidth: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSequencePaddingWidth(::core::mem::transmute_copy(&dwwidth)).into()
        }
        unsafe extern "system" fn SetSequenceZeroPadding<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fzeropad: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSequenceZeroPadding(::core::mem::transmute_copy(&fzeropad)).into()
        }
        unsafe extern "system" fn SetGroupTag<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszgrouptag: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGroupTag(::core::mem::transmute(&pszgrouptag)).into()
        }
        unsafe extern "system" fn SetAcquisitionTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pftacquisitiontime: *const super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAcquisitionTime(::core::mem::transmute_copy(&pftacquisitiontime)).into()
        }
        unsafe extern "system" fn GetFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwphotoacquireflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFlags() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwphotoacquireflags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputFilenameTemplate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtemplate: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOutputFilenameTemplate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtemplate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSequencePaddingWidth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwwidth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSequencePaddingWidth() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwwidth, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSequenceZeroPadding<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfzeropad: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSequenceZeroPadding() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfzeropad, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGroupTag<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrgrouptag: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGroupTag() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrgrouptag, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAcquisitionTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pftacquisitiontime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAcquisitionTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pftacquisitiontime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitializeFromRegistry: InitializeFromRegistry::<Identity, Impl, OFFSET>,
            SetFlags: SetFlags::<Identity, Impl, OFFSET>,
            SetOutputFilenameTemplate: SetOutputFilenameTemplate::<Identity, Impl, OFFSET>,
            SetSequencePaddingWidth: SetSequencePaddingWidth::<Identity, Impl, OFFSET>,
            SetSequenceZeroPadding: SetSequenceZeroPadding::<Identity, Impl, OFFSET>,
            SetGroupTag: SetGroupTag::<Identity, Impl, OFFSET>,
            SetAcquisitionTime: SetAcquisitionTime::<Identity, Impl, OFFSET>,
            GetFlags: GetFlags::<Identity, Impl, OFFSET>,
            GetOutputFilenameTemplate: GetOutputFilenameTemplate::<Identity, Impl, OFFSET>,
            GetSequencePaddingWidth: GetSequencePaddingWidth::<Identity, Impl, OFFSET>,
            GetSequenceZeroPadding: GetSequenceZeroPadding::<Identity, Impl, OFFSET>,
            GetGroupTag: GetGroupTag::<Identity, Impl, OFFSET>,
            GetAcquisitionTime: GetAcquisitionTime::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoAcquireSettings as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IPhotoAcquireSource_Impl: Sized {
    fn GetFriendlyName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn GetDeviceIcons(&self, nsize: u32, phlargeicon: *mut super::super::UI::WindowsAndMessaging::HICON, phsmallicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows::core::Result<()>;
    fn InitializeItemList(&self, fforceenumeration: super::super::Foundation::BOOL, pphotoacquireprogresscb: ::core::option::Option<&IPhotoAcquireProgressCB>, pnitemcount: *mut u32) -> ::windows::core::Result<()>;
    fn GetItemCount(&self) -> ::windows::core::Result<u32>;
    fn GetItemAt(&self, nindex: u32) -> ::windows::core::Result<IPhotoAcquireItem>;
    fn GetPhotoAcquireSettings(&self) -> ::windows::core::Result<IPhotoAcquireSettings>;
    fn GetDeviceId(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn BindToObject(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows::core::RuntimeName for IPhotoAcquireSource {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IPhotoAcquireSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSource_Impl, const OFFSET: isize>() -> IPhotoAcquireSource_Vtbl {
        unsafe extern "system" fn GetFriendlyName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfriendlyname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrfriendlyname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceIcons<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nsize: u32, phlargeicon: *mut super::super::UI::WindowsAndMessaging::HICON, phsmallicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeviceIcons(::core::mem::transmute_copy(&nsize), ::core::mem::transmute_copy(&phlargeicon), ::core::mem::transmute_copy(&phsmallicon)).into()
        }
        unsafe extern "system" fn InitializeItemList<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fforceenumeration: super::super::Foundation::BOOL, pphotoacquireprogresscb: *mut ::core::ffi::c_void, pnitemcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializeItemList(::core::mem::transmute_copy(&fforceenumeration), ::windows::core::from_raw_borrowed(&pphotoacquireprogresscb), ::core::mem::transmute_copy(&pnitemcount)).into()
        }
        unsafe extern "system" fn GetItemCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnitemcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnitemcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemAt<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, ppphotoacquireitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItemAt(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppphotoacquireitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPhotoAcquireSettings<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppphotoacquiresettings: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPhotoAcquireSettings() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppphotoacquiresettings, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdeviceid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdeviceid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindToObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoAcquireSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BindToObject(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFriendlyName: GetFriendlyName::<Identity, Impl, OFFSET>,
            GetDeviceIcons: GetDeviceIcons::<Identity, Impl, OFFSET>,
            InitializeItemList: InitializeItemList::<Identity, Impl, OFFSET>,
            GetItemCount: GetItemCount::<Identity, Impl, OFFSET>,
            GetItemAt: GetItemAt::<Identity, Impl, OFFSET>,
            GetPhotoAcquireSettings: GetPhotoAcquireSettings::<Identity, Impl, OFFSET>,
            GetDeviceId: GetDeviceId::<Identity, Impl, OFFSET>,
            BindToObject: BindToObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoAcquireSource as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPhotoProgressActionCB_Impl: Sized {
    fn DoAction(&self, hwndparent: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IPhotoProgressActionCB {}
#[cfg(feature = "Win32_Foundation")]
impl IPhotoProgressActionCB_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressActionCB_Impl, const OFFSET: isize>() -> IPhotoProgressActionCB_Vtbl {
        unsafe extern "system" fn DoAction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressActionCB_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DoAction(::core::mem::transmute_copy(&hwndparent)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), DoAction: DoAction::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoProgressActionCB as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_WindowsAndMessaging\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IPhotoProgressDialog_Impl: Sized {
    fn Create(&self, hwndparent: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn GetWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND>;
    fn Destroy(&self) -> ::windows::core::Result<()>;
    fn SetTitle(&self, psztitle: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn ShowCheckbox(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fshow: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetCheckboxText(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtext: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn SetCheckboxCheck(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fchecked: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetCheckboxTooltip(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtooltiptext: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn IsCheckboxChecked(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetCaption(&self, psztitle: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn SetImage(&self, nimagetype: PROGRESS_DIALOG_IMAGE_TYPE, hicon: super::super::UI::WindowsAndMessaging::HICON, hbitmap: super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::Result<()>;
    fn SetPercentComplete(&self, npercent: i32) -> ::windows::core::Result<()>;
    fn SetProgressText(&self, pszprogresstext: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn SetActionLinkCallback(&self, pphotoprogressactioncb: ::core::option::Option<&IPhotoProgressActionCB>) -> ::windows::core::Result<()>;
    fn SetActionLinkText(&self, pszcaption: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn ShowActionLink(&self, fshow: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn IsCancelled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetUserInput(&self, riidtype: *const ::windows::core::GUID, punknown: ::core::option::Option<&::windows::core::IUnknown>, ppropvarresult: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvardefault: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows::core::RuntimeName for IPhotoProgressDialog {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_WindowsAndMessaging"))]
impl IPhotoProgressDialog_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>() -> IPhotoProgressDialog_Vtbl {
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Create(::core::mem::transmute_copy(&hwndparent)).into()
        }
        unsafe extern "system" fn GetWindow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwndprogressdialog: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWindow() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phwndprogressdialog, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Destroy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Destroy().into()
        }
        unsafe extern "system" fn SetTitle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztitle: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTitle(::core::mem::transmute(&psztitle)).into()
        }
        unsafe extern "system" fn ShowCheckbox<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShowCheckbox(::core::mem::transmute_copy(&ncheckboxid), ::core::mem::transmute_copy(&fshow)).into()
        }
        unsafe extern "system" fn SetCheckboxText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCheckboxText(::core::mem::transmute_copy(&ncheckboxid), ::core::mem::transmute(&pszcheckboxtext)).into()
        }
        unsafe extern "system" fn SetCheckboxCheck<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fchecked: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCheckboxCheck(::core::mem::transmute_copy(&ncheckboxid), ::core::mem::transmute_copy(&fchecked)).into()
        }
        unsafe extern "system" fn SetCheckboxTooltip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtooltiptext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCheckboxTooltip(::core::mem::transmute_copy(&ncheckboxid), ::core::mem::transmute(&pszcheckboxtooltiptext)).into()
        }
        unsafe extern "system" fn IsCheckboxChecked<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pfchecked: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsCheckboxChecked(::core::mem::transmute_copy(&ncheckboxid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfchecked, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCaption<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztitle: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCaption(::core::mem::transmute(&psztitle)).into()
        }
        unsafe extern "system" fn SetImage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nimagetype: PROGRESS_DIALOG_IMAGE_TYPE, hicon: super::super::UI::WindowsAndMessaging::HICON, hbitmap: super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetImage(::core::mem::transmute_copy(&nimagetype), ::core::mem::transmute_copy(&hicon), ::core::mem::transmute_copy(&hbitmap)).into()
        }
        unsafe extern "system" fn SetPercentComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, npercent: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPercentComplete(::core::mem::transmute_copy(&npercent)).into()
        }
        unsafe extern "system" fn SetProgressText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszprogresstext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProgressText(::core::mem::transmute(&pszprogresstext)).into()
        }
        unsafe extern "system" fn SetActionLinkCallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphotoprogressactioncb: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetActionLinkCallback(::windows::core::from_raw_borrowed(&pphotoprogressactioncb)).into()
        }
        unsafe extern "system" fn SetActionLinkText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcaption: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetActionLinkText(::core::mem::transmute(&pszcaption)).into()
        }
        unsafe extern "system" fn ShowActionLink<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShowActionLink(::core::mem::transmute_copy(&fshow)).into()
        }
        unsafe extern "system" fn IsCancelled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfcancelled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsCancelled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcancelled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserInput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhotoProgressDialog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riidtype: *const ::windows::core::GUID, punknown: *mut ::core::ffi::c_void, ppropvarresult: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvardefault: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetUserInput(::core::mem::transmute_copy(&riidtype), ::windows::core::from_raw_borrowed(&punknown), ::core::mem::transmute_copy(&ppropvarresult), ::core::mem::transmute_copy(&ppropvardefault)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Create: Create::<Identity, Impl, OFFSET>,
            GetWindow: GetWindow::<Identity, Impl, OFFSET>,
            Destroy: Destroy::<Identity, Impl, OFFSET>,
            SetTitle: SetTitle::<Identity, Impl, OFFSET>,
            ShowCheckbox: ShowCheckbox::<Identity, Impl, OFFSET>,
            SetCheckboxText: SetCheckboxText::<Identity, Impl, OFFSET>,
            SetCheckboxCheck: SetCheckboxCheck::<Identity, Impl, OFFSET>,
            SetCheckboxTooltip: SetCheckboxTooltip::<Identity, Impl, OFFSET>,
            IsCheckboxChecked: IsCheckboxChecked::<Identity, Impl, OFFSET>,
            SetCaption: SetCaption::<Identity, Impl, OFFSET>,
            SetImage: SetImage::<Identity, Impl, OFFSET>,
            SetPercentComplete: SetPercentComplete::<Identity, Impl, OFFSET>,
            SetProgressText: SetProgressText::<Identity, Impl, OFFSET>,
            SetActionLinkCallback: SetActionLinkCallback::<Identity, Impl, OFFSET>,
            SetActionLinkText: SetActionLinkText::<Identity, Impl, OFFSET>,
            ShowActionLink: ShowActionLink::<Identity, Impl, OFFSET>,
            IsCancelled: IsCancelled::<Identity, Impl, OFFSET>,
            GetUserInput: GetUserInput::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhotoProgressDialog as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IUserInputString_Impl: Sized {
    fn GetSubmitButtonText(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn GetPrompt(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn GetStringId(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn GetStringType(&self) -> ::windows::core::Result<USER_INPUT_STRING_TYPE>;
    fn GetTooltipText(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn GetMaxLength(&self) -> ::windows::core::Result<u32>;
    fn GetDefault(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn GetMruCount(&self) -> ::windows::core::Result<u32>;
    fn GetMruEntryAt(&self, nindex: u32) -> ::windows::core::Result<::windows::core::BSTR>;
    fn GetImage(&self, nsize: u32, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP, phicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows::core::RuntimeName for IUserInputString {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IUserInputString_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUserInputString_Impl, const OFFSET: isize>() -> IUserInputString_Vtbl {
        unsafe extern "system" fn GetSubmitButtonText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUserInputString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsubmitbuttontext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSubmitButtonText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsubmitbuttontext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrompt<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUserInputString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprompttitle: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPrompt() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprompttitle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUserInputString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstringid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStringId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrstringid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUserInputString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnstringtype: *mut USER_INPUT_STRING_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStringType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnstringtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTooltipText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUserInputString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtooltiptext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTooltipText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtooltiptext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUserInputString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchmaxlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaxLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcchmaxlength, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefault<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUserInputString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdefault: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdefault, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMruCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUserInputString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnmrucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMruCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnmrucount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMruEntryAt<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUserInputString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, pbstrmruentry: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMruEntryAt(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrmruentry, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUserInputString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nsize: u32, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP, phicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetImage(::core::mem::transmute_copy(&nsize), ::core::mem::transmute_copy(&phbitmap), ::core::mem::transmute_copy(&phicon)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSubmitButtonText: GetSubmitButtonText::<Identity, Impl, OFFSET>,
            GetPrompt: GetPrompt::<Identity, Impl, OFFSET>,
            GetStringId: GetStringId::<Identity, Impl, OFFSET>,
            GetStringType: GetStringType::<Identity, Impl, OFFSET>,
            GetTooltipText: GetTooltipText::<Identity, Impl, OFFSET>,
            GetMaxLength: GetMaxLength::<Identity, Impl, OFFSET>,
            GetDefault: GetDefault::<Identity, Impl, OFFSET>,
            GetMruCount: GetMruCount::<Identity, Impl, OFFSET>,
            GetMruEntryAt: GetMruEntryAt::<Identity, Impl, OFFSET>,
            GetImage: GetImage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserInputString as ::windows::core::ComInterface>::IID
    }
}
