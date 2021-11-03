#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DEVICE_SELECTION_DEVICE_TYPE(pub i32);
pub const DST_UNKNOWN_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = DEVICE_SELECTION_DEVICE_TYPE(0i32);
pub const DST_WPD_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = DEVICE_SELECTION_DEVICE_TYPE(1i32);
pub const DST_WIA_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = DEVICE_SELECTION_DEVICE_TYPE(2i32);
pub const DST_STI_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = DEVICE_SELECTION_DEVICE_TYPE(3i32);
pub const DSF_TWAIN_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = DEVICE_SELECTION_DEVICE_TYPE(4i32);
pub const DST_FS_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = DEVICE_SELECTION_DEVICE_TYPE(5i32);
pub const DST_DV_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = DEVICE_SELECTION_DEVICE_TYPE(6i32);
impl ::std::convert::From<i32> for DEVICE_SELECTION_DEVICE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DEVICE_SELECTION_DEVICE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const DSF_ALL_DEVICES: u32 = 65535u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const DSF_CPL_MODE: u32 = 65536u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const DSF_DV_DEVICES: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const DSF_FS_DEVICES: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const DSF_SHOW_OFFLINE: u32 = 131072u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const DSF_STI_DEVICES: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const DSF_TWAIN_DEVICES: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const DSF_WIA_CAMERAS: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const DSF_WIA_SCANNERS: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const DSF_WPD_DEVICES: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ERROR_ADVISE_MESSAGE_TYPE(pub i32);
pub const PHOTOACQUIRE_ERROR_SKIPRETRYCANCEL: ERROR_ADVISE_MESSAGE_TYPE = ERROR_ADVISE_MESSAGE_TYPE(0i32);
pub const PHOTOACQUIRE_ERROR_RETRYCANCEL: ERROR_ADVISE_MESSAGE_TYPE = ERROR_ADVISE_MESSAGE_TYPE(1i32);
pub const PHOTOACQUIRE_ERROR_YESNO: ERROR_ADVISE_MESSAGE_TYPE = ERROR_ADVISE_MESSAGE_TYPE(2i32);
pub const PHOTOACQUIRE_ERROR_OK: ERROR_ADVISE_MESSAGE_TYPE = ERROR_ADVISE_MESSAGE_TYPE(3i32);
impl ::std::convert::From<i32> for ERROR_ADVISE_MESSAGE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ERROR_ADVISE_MESSAGE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ERROR_ADVISE_RESULT(pub i32);
pub const PHOTOACQUIRE_RESULT_YES: ERROR_ADVISE_RESULT = ERROR_ADVISE_RESULT(0i32);
pub const PHOTOACQUIRE_RESULT_NO: ERROR_ADVISE_RESULT = ERROR_ADVISE_RESULT(1i32);
pub const PHOTOACQUIRE_RESULT_OK: ERROR_ADVISE_RESULT = ERROR_ADVISE_RESULT(2i32);
pub const PHOTOACQUIRE_RESULT_SKIP: ERROR_ADVISE_RESULT = ERROR_ADVISE_RESULT(3i32);
pub const PHOTOACQUIRE_RESULT_SKIP_ALL: ERROR_ADVISE_RESULT = ERROR_ADVISE_RESULT(4i32);
pub const PHOTOACQUIRE_RESULT_RETRY: ERROR_ADVISE_RESULT = ERROR_ADVISE_RESULT(5i32);
pub const PHOTOACQUIRE_RESULT_ABORT: ERROR_ADVISE_RESULT = ERROR_ADVISE_RESULT(6i32);
impl ::std::convert::From<i32> for ERROR_ADVISE_RESULT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ERROR_ADVISE_RESULT {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IPhotoAcquire(::windows::runtime::IUnknown);
impl IPhotoAcquire {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn CreatePhotoSource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszdevice: Param0) -> ::windows::runtime::Result<IPhotoAcquireSource> {
        let mut result__: <IPhotoAcquireSource as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pszdevice.into_param().abi(), &mut result__).from_abi::<IPhotoAcquireSource>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn Acquire<'a, Param0: ::windows::runtime::IntoParam<'a, IPhotoAcquireSource>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, IPhotoAcquireProgressCB>>(
        &self,
        pphotoacquiresource: Param0,
        fshowprogress: Param1,
        hwndparent: Param2,
        pszapplicationname: Param3,
        pphotoacquireprogresscb: Param4,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pphotoacquiresource.into_param().abi(), fshowprogress.into_param().abi(), hwndparent.into_param().abi(), pszapplicationname.into_param().abi(), pphotoacquireprogresscb.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_System_Com`*"]
    pub unsafe fn EnumResults(&self) -> ::windows::runtime::Result<super::super::System::Com::IEnumString> {
        let mut result__: <super::super::System::Com::IEnumString as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::IEnumString>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPhotoAcquire {
    type Vtable = IPhotoAcquire_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(15872851, 58139, 18773, [168, 173, 202, 94, 191, 49, 226, 206]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoAcquire_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszdevice: super::super::Foundation::PWSTR, ppphotoacquiresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pphotoacquiresource: ::windows::runtime::RawPtr, fshowprogress: super::super::Foundation::BOOL, hwndparent: super::super::Foundation::HWND, pszapplicationname: super::super::Foundation::PWSTR, pphotoacquireprogresscb: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumfilepaths: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IPhotoAcquireDeviceSelectionDialog(::windows::runtime::IUnknown);
impl IPhotoAcquireDeviceSelectionDialog {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn SetTitle<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, psztitle: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), psztitle.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn SetSubmitButtonText<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszsubmitbuttontext: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pszsubmitbuttontext.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn DoModal<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndparent: Param0, dwdeviceflags: u32, pbstrdeviceid: *mut super::super::Foundation::BSTR, pndevicetype: *mut DEVICE_SELECTION_DEVICE_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), hwndparent.into_param().abi(), ::std::mem::transmute(dwdeviceflags), ::std::mem::transmute(pbstrdeviceid), ::std::mem::transmute(pndevicetype)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPhotoAcquireDeviceSelectionDialog {
    type Vtable = IPhotoAcquireDeviceSelectionDialog_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(15894583, 21981, 20279, [170, 245, 104, 85, 169, 100, 4, 103]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoAcquireDeviceSelectionDialog_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psztitle: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszsubmitbuttontext: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: super::super::Foundation::HWND, dwdeviceflags: u32, pbstrdeviceid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pndevicetype: *mut DEVICE_SELECTION_DEVICE_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IPhotoAcquireItem(::windows::runtime::IUnknown);
impl IPhotoAcquireItem {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn GetItemName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn GetThumbnail<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::SIZE>>(&self, sizethumbnail: Param0) -> ::windows::runtime::Result<super::super::Graphics::Gdi::HBITMAP> {
        let mut result__: <super::super::Graphics::Gdi::HBITMAP as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), sizethumbnail.into_param().abi(), &mut result__).from_abi::<super::super::Graphics::Gdi::HBITMAP>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetProperty(&self, key: *const super::super::System::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(key), &mut result__).from_abi::<super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn SetProperty(&self, key: *const super::super::System::PropertiesSystem::PROPERTYKEY, pv: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(key), ::std::mem::transmute(pv)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_System_Com`*"]
    pub unsafe fn GetStream(&self) -> ::windows::runtime::Result<super::super::System::Com::IStream> {
        let mut result__: <super::super::System::Com::IStream as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn CanDelete(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
    pub unsafe fn GetSubItemCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
    pub unsafe fn GetSubItemAt(&self, nitemindex: u32) -> ::windows::runtime::Result<IPhotoAcquireItem> {
        let mut result__: <IPhotoAcquireItem as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(nitemindex), &mut result__).from_abi::<IPhotoAcquireItem>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPhotoAcquireItem {
    type Vtable = IPhotoAcquireItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(15867031, 10431, 19458, [184, 66, 94, 78, 144, 19, 154, 48]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoAcquireItem_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstritemname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sizethumbnail: super::super::Foundation::SIZE, phbmpthumbnail: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::System::PropertiesSystem::PROPERTYKEY, pv: *mut ::std::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::System::PropertiesSystem::PROPERTYKEY, pv: *const ::std::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppstream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfcandelete: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pncount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nitemindex: u32, ppphotoacquireitem: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IPhotoAcquireOptionsDialog(::windows::runtime::IUnknown);
impl IPhotoAcquireOptionsDialog {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszregistryroot: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pszregistryroot.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndparent: Param0) -> ::windows::runtime::Result<super::super::Foundation::HWND> {
        let mut result__: <super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), hwndparent.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
    pub unsafe fn Destroy(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn DoModal<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndparent: Param0, ppnreturncode: *mut isize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), hwndparent.into_param().abi(), ::std::mem::transmute(ppnreturncode)).ok()
    }
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
    pub unsafe fn SaveData(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPhotoAcquireOptionsDialog {
    type Vtable = IPhotoAcquireOptionsDialog_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(15905774, 48996, 18414, [137, 244, 77, 237, 215, 150, 67, 242]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoAcquireOptionsDialog_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszregistryroot: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: super::super::Foundation::HWND, phwnddialog: *mut super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: super::super::Foundation::HWND, ppnreturncode: *mut isize) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IPhotoAcquirePlugin(::windows::runtime::IUnknown);
impl IPhotoAcquirePlugin {
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, IPhotoAcquireSource>, Param1: ::windows::runtime::IntoParam<'a, IPhotoAcquireProgressCB>>(&self, pphotoacquiresource: Param0, pphotoacquireprogresscb: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pphotoacquiresource.into_param().abi(), pphotoacquireprogresscb.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn ProcessItem<'a, Param1: ::windows::runtime::IntoParam<'a, IPhotoAcquireItem>, Param2: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::System::PropertiesSystem::IPropertyStore>>(
        &self,
        dwacquirestage: u32,
        pphotoacquireitem: Param1,
        poriginalitemstream: Param2,
        pszfinalfilename: Param3,
        ppropertystore: Param4,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwacquirestage), pphotoacquireitem.into_param().abi(), poriginalitemstream.into_param().abi(), pszfinalfilename.into_param().abi(), ppropertystore.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
    pub unsafe fn TransferComplete(&self, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(hr)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn DisplayConfigureDialog<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndparent: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), hwndparent.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPhotoAcquirePlugin {
    type Vtable = IPhotoAcquirePlugin_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(15916267, 60600, 20343, [142, 71, 231, 169, 135, 200, 61, 208]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoAcquirePlugin_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pphotoacquiresource: ::windows::runtime::RawPtr, pphotoacquireprogresscb: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_PropertiesSystem"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwacquirestage: u32, pphotoacquireitem: ::windows::runtime::RawPtr, poriginalitemstream: ::windows::runtime::RawPtr, pszfinalfilename: super::super::Foundation::PWSTR, ppropertystore: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_PropertiesSystem")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IPhotoAcquireProgressCB(::windows::runtime::IUnknown);
impl IPhotoAcquireProgressCB {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn Cancelled(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
    pub unsafe fn StartEnumeration<'a, Param0: ::windows::runtime::IntoParam<'a, IPhotoAcquireSource>>(&self, pphotoacquiresource: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pphotoacquiresource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
    pub unsafe fn FoundItem<'a, Param0: ::windows::runtime::IntoParam<'a, IPhotoAcquireItem>>(&self, pphotoacquireitem: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pphotoacquireitem.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
    pub unsafe fn EndEnumeration(&self, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(hr)).ok()
    }
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
    pub unsafe fn StartTransfer<'a, Param0: ::windows::runtime::IntoParam<'a, IPhotoAcquireSource>>(&self, pphotoacquiresource: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pphotoacquiresource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
    pub unsafe fn StartItemTransfer<'a, Param1: ::windows::runtime::IntoParam<'a, IPhotoAcquireItem>>(&self, nitemindex: u32, pphotoacquireitem: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(nitemindex), pphotoacquireitem.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn DirectoryCreated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszdirectory: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pszdirectory.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn UpdateTransferPercent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, foverall: Param0, npercent: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), foverall.into_param().abi(), ::std::mem::transmute(npercent)).ok()
    }
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
    pub unsafe fn EndItemTransfer<'a, Param1: ::windows::runtime::IntoParam<'a, IPhotoAcquireItem>>(&self, nitemindex: u32, pphotoacquireitem: Param1, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(nitemindex), pphotoacquireitem.into_param().abi(), ::std::mem::transmute(hr)).ok()
    }
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
    pub unsafe fn EndTransfer(&self, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(hr)).ok()
    }
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
    pub unsafe fn StartDelete<'a, Param0: ::windows::runtime::IntoParam<'a, IPhotoAcquireSource>>(&self, pphotoacquiresource: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), pphotoacquiresource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
    pub unsafe fn StartItemDelete<'a, Param1: ::windows::runtime::IntoParam<'a, IPhotoAcquireItem>>(&self, nitemindex: u32, pphotoacquireitem: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(nitemindex), pphotoacquireitem.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
    pub unsafe fn UpdateDeletePercent(&self, npercent: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(npercent)).ok()
    }
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
    pub unsafe fn EndItemDelete<'a, Param1: ::windows::runtime::IntoParam<'a, IPhotoAcquireItem>>(&self, nitemindex: u32, pphotoacquireitem: Param1, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(nitemindex), pphotoacquireitem.into_param().abi(), ::std::mem::transmute(hr)).ok()
    }
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
    pub unsafe fn EndDelete(&self, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(hr)).ok()
    }
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
    pub unsafe fn EndSession(&self, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(hr)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn GetDeleteAfterAcquire(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn ErrorAdvise<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, hr: ::windows::runtime::HRESULT, pszerrormessage: Param1, nmessagetype: ERROR_ADVISE_MESSAGE_TYPE) -> ::windows::runtime::Result<ERROR_ADVISE_RESULT> {
        let mut result__: <ERROR_ADVISE_RESULT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(hr), pszerrormessage.into_param().abi(), ::std::mem::transmute(nmessagetype), &mut result__).from_abi::<ERROR_ADVISE_RESULT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetUserInput<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, riidtype: *const ::windows::runtime::GUID, punknown: Param1, ppropvarresult: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvardefault: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(riidtype), punknown.into_param().abi(), ::std::mem::transmute(ppropvarresult), ::std::mem::transmute(ppropvardefault)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPhotoAcquireProgressCB {
    type Vtable = IPhotoAcquireProgressCB_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(15912478, 37726, 16968, [137, 44, 19, 15, 50, 196, 92, 180]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoAcquireProgressCB_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfcancelled: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pphotoacquiresource: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pphotoacquireitem: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pphotoacquiresource: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nitemindex: u32, pphotoacquireitem: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszdirectory: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, foverall: super::super::Foundation::BOOL, npercent: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nitemindex: u32, pphotoacquireitem: ::windows::runtime::RawPtr, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pphotoacquiresource: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nitemindex: u32, pphotoacquireitem: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, npercent: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nitemindex: u32, pphotoacquireitem: ::windows::runtime::RawPtr, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hr: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfdeleteafteracquire: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hr: ::windows::runtime::HRESULT, pszerrormessage: super::super::Foundation::PWSTR, nmessagetype: ERROR_ADVISE_MESSAGE_TYPE, pnerroradviseresult: *mut ERROR_ADVISE_RESULT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riidtype: *const ::windows::runtime::GUID, punknown: ::windows::runtime::RawPtr, ppropvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>, ppropvardefault: *const ::std::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
);
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IPhotoAcquireSettings(::windows::runtime::IUnknown);
impl IPhotoAcquireSettings {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn InitializeFromRegistry<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszregistrykey: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pszregistrykey.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
    pub unsafe fn SetFlags(&self, dwphotoacquireflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwphotoacquireflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn SetOutputFilenameTemplate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, psztemplate: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), psztemplate.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
    pub unsafe fn SetSequencePaddingWidth(&self, dwwidth: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwwidth)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn SetSequenceZeroPadding<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fzeropad: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), fzeropad.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn SetGroupTag<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszgrouptag: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), pszgrouptag.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn SetAcquisitionTime(&self, pftacquisitiontime: *const super::super::Foundation::FILETIME) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(pftacquisitiontime)).ok()
    }
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
    pub unsafe fn GetFlags(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn GetOutputFilenameTemplate(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
    pub unsafe fn GetSequencePaddingWidth(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn GetSequenceZeroPadding(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn GetGroupTag(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn GetAcquisitionTime(&self) -> ::windows::runtime::Result<super::super::Foundation::FILETIME> {
        let mut result__: <super::super::Foundation::FILETIME as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPhotoAcquireSettings {
    type Vtable = IPhotoAcquireSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(15906920, 56679, 18556, [149, 83, 4, 146, 64, 118, 126, 145]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoAcquireSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszregistrykey: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwphotoacquireflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psztemplate: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwwidth: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fzeropad: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszgrouptag: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pftacquisitiontime: *const super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwphotoacquireflags: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrtemplate: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwwidth: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfzeropad: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrgrouptag: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pftacquisitiontime: *mut super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IPhotoAcquireSource(::windows::runtime::IUnknown);
impl IPhotoAcquireSource {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn GetFriendlyName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn GetDeviceIcons(&self, nsize: u32, phlargeicon: *mut super::super::UI::WindowsAndMessaging::HICON, phsmallicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(nsize), ::std::mem::transmute(phlargeicon), ::std::mem::transmute(phsmallicon)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn InitializeItemList<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::runtime::IntoParam<'a, IPhotoAcquireProgressCB>>(&self, fforceenumeration: Param0, pphotoacquireprogresscb: Param1, pnitemcount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), fforceenumeration.into_param().abi(), pphotoacquireprogresscb.into_param().abi(), ::std::mem::transmute(pnitemcount)).ok()
    }
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
    pub unsafe fn GetItemCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
    pub unsafe fn GetItemAt(&self, nindex: u32) -> ::windows::runtime::Result<IPhotoAcquireItem> {
        let mut result__: <IPhotoAcquireItem as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(nindex), &mut result__).from_abi::<IPhotoAcquireItem>(result__)
    }
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
    pub unsafe fn GetPhotoAcquireSettings(&self) -> ::windows::runtime::Result<IPhotoAcquireSettings> {
        let mut result__: <IPhotoAcquireSettings as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IPhotoAcquireSettings>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn GetDeviceId(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
    pub unsafe fn BindToObject(&self, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPhotoAcquireSource {
    type Vtable = IPhotoAcquireSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(15910659, 34323, 17026, [165, 59, 110, 197, 156, 88, 131, 172]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoAcquireSource_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrfriendlyname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nsize: u32, phlargeicon: *mut super::super::UI::WindowsAndMessaging::HICON, phsmallicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fforceenumeration: super::super::Foundation::BOOL, pphotoacquireprogresscb: ::windows::runtime::RawPtr, pnitemcount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnitemcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nindex: u32, ppphotoacquireitem: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppphotoacquiresettings: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdeviceid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IPhotoProgressActionCB(::windows::runtime::IUnknown);
impl IPhotoProgressActionCB {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn DoAction<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndparent: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), hwndparent.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPhotoProgressActionCB {
    type Vtable = IPhotoProgressActionCB_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(15876816, 45574, 20093, [180, 193, 71, 85, 188, 187, 156, 159]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoProgressActionCB_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IPhotoProgressDialog(::windows::runtime::IUnknown);
impl IPhotoProgressDialog {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndparent: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), hwndparent.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn GetWindow(&self) -> ::windows::runtime::Result<super::super::Foundation::HWND> {
        let mut result__: <super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
    pub unsafe fn Destroy(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn SetTitle<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, psztitle: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), psztitle.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn ShowCheckbox<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fshow: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(ncheckboxid), fshow.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn SetCheckboxText<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtext: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(ncheckboxid), pszcheckboxtext.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn SetCheckboxCheck<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fchecked: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(ncheckboxid), fchecked.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn SetCheckboxTooltip<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtooltiptext: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(ncheckboxid), pszcheckboxtooltiptext.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn IsCheckboxChecked(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(ncheckboxid), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn SetCaption<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, psztitle: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), psztitle.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn SetImage<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::UI::WindowsAndMessaging::HICON>, Param2: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HBITMAP>>(&self, nimagetype: PROGRESS_DIALOG_IMAGE_TYPE, hicon: Param1, hbitmap: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(nimagetype), hicon.into_param().abi(), hbitmap.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
    pub unsafe fn SetPercentComplete(&self, npercent: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(npercent)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn SetProgressText<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszprogresstext: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), pszprogresstext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
    pub unsafe fn SetActionLinkCallback<'a, Param0: ::windows::runtime::IntoParam<'a, IPhotoProgressActionCB>>(&self, pphotoprogressactioncb: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), pphotoprogressactioncb.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn SetActionLinkText<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszcaption: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), pszcaption.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn ShowActionLink<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fshow: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), fshow.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn IsCancelled(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetUserInput<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, riidtype: *const ::windows::runtime::GUID, punknown: Param1, ppropvarresult: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvardefault: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(riidtype), punknown.into_param().abi(), ::std::mem::transmute(ppropvarresult), ::std::mem::transmute(ppropvardefault)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPhotoProgressDialog {
    type Vtable = IPhotoProgressDialog_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(15877881, 1872, 20232, [147, 129, 44, 216, 233, 6, 164, 174]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoProgressDialog_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phwndprogressdialog: *mut super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psztitle: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fshow: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtext: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fchecked: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtooltiptext: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pfchecked: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psztitle: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nimagetype: PROGRESS_DIALOG_IMAGE_TYPE, hicon: super::super::UI::WindowsAndMessaging::HICON, hbitmap: super::super::Graphics::Gdi::HBITMAP) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, npercent: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszprogresstext: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pphotoprogressactioncb: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszcaption: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fshow: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfcancelled: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riidtype: *const ::windows::runtime::GUID, punknown: ::windows::runtime::RawPtr, ppropvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>, ppropvardefault: *const ::std::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
);
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IUserInputString(::windows::runtime::IUnknown);
impl IUserInputString {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn GetSubmitButtonText(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn GetPrompt(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn GetStringId(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
    pub unsafe fn GetStringType(&self) -> ::windows::runtime::Result<USER_INPUT_STRING_TYPE> {
        let mut result__: <USER_INPUT_STRING_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<USER_INPUT_STRING_TYPE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn GetTooltipText(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
    pub unsafe fn GetMaxLength(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn GetDefault(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
    pub unsafe fn GetMruCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Foundation`*"]
    pub unsafe fn GetMruEntryAt(&self, nindex: u32) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(nindex), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_Media_PictureAcquisition`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn GetImage(&self, nsize: u32, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP, phicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(nsize), ::std::mem::transmute(phbitmap), ::std::mem::transmute(phicon)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUserInputString {
    type Vtable = IUserInputString_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(15877025, 8283, 17850, [174, 38, 171, 188, 83, 170, 122, 111]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserInputString_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrsubmitbuttontext: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrprompttitle: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrstringid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnstringtype: *mut USER_INPUT_STRING_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrtooltiptext: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcchmaxlength: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdefault: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnmrucount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nindex: u32, pbstrmruentry: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nsize: u32, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP, phicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging")))] usize,
);
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PAPS_CLEANUP: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PAPS_POSTSAVE: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PAPS_PRESAVE: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PHOTOACQ_ABORT_ON_SETTINGS_UPDATE: u32 = 2048u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PHOTOACQ_DELETE_AFTER_ACQUIRE: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PHOTOACQ_DISABLE_AUTO_ROTATE: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PHOTOACQ_DISABLE_DB_INTEGRATION: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PHOTOACQ_DISABLE_DUPLICATE_DETECTION: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PHOTOACQ_DISABLE_GROUP_TAG_PROMPT: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PHOTOACQ_DISABLE_METADATA_WRITE: u32 = 256u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PHOTOACQ_DISABLE_PLUGINS: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PHOTOACQ_DISABLE_SETTINGS_LINK: u32 = 1024u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PHOTOACQ_DISABLE_THUMBNAIL_PROGRESS: u32 = 512u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PHOTOACQ_ENABLE_THUMBNAIL_CACHING: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PHOTOACQ_ERROR_RESTART_REQUIRED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147180543i32 as _);
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PHOTOACQ_IMPORT_VIDEO_AS_MULTIPLE_FILES: u32 = 4096u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PHOTOACQ_NO_GALLERY_LAUNCH: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PHOTOACQ_RUN_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROGRESS_DIALOG_CHECKBOX_ID(pub i32);
pub const PROGRESS_DIALOG_CHECKBOX_ID_DEFAULT: PROGRESS_DIALOG_CHECKBOX_ID = PROGRESS_DIALOG_CHECKBOX_ID(0i32);
impl ::std::convert::From<i32> for PROGRESS_DIALOG_CHECKBOX_ID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROGRESS_DIALOG_CHECKBOX_ID {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROGRESS_DIALOG_IMAGE_TYPE(pub i32);
pub const PROGRESS_DIALOG_ICON_SMALL: PROGRESS_DIALOG_IMAGE_TYPE = PROGRESS_DIALOG_IMAGE_TYPE(0i32);
pub const PROGRESS_DIALOG_ICON_LARGE: PROGRESS_DIALOG_IMAGE_TYPE = PROGRESS_DIALOG_IMAGE_TYPE(1i32);
pub const PROGRESS_DIALOG_ICON_THUMBNAIL: PROGRESS_DIALOG_IMAGE_TYPE = PROGRESS_DIALOG_IMAGE_TYPE(2i32);
pub const PROGRESS_DIALOG_BITMAP_THUMBNAIL: PROGRESS_DIALOG_IMAGE_TYPE = PROGRESS_DIALOG_IMAGE_TYPE(3i32);
impl ::std::convert::From<i32> for PROGRESS_DIALOG_IMAGE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROGRESS_DIALOG_IMAGE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
pub const PROGRESS_INDETERMINATE: i32 = -1i32;
pub const PhotoAcquire: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(15887874, 59890, 19103, [159, 221, 90, 150, 47, 178, 106, 152]);
pub const PhotoAcquireAutoPlayDropTarget: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(15863477, 36822, 19869, [183, 94, 54, 128, 23, 102, 200, 241]);
pub const PhotoAcquireAutoPlayHWEventHandler: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(15905843, 17636, 19848, [178, 176, 38, 152, 160, 169, 29, 186]);
pub const PhotoAcquireDeviceSelectionDialog: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(15899188, 47265, 18476, [188, 248, 58, 199, 176, 254, 143, 98]);
pub const PhotoAcquireOptionsDialog: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(15863969, 25328, 17291, [159, 126, 150, 24, 215, 42, 24, 49]);
pub const PhotoProgressDialog: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(15879328, 29839, 20106, [137, 79, 14, 3, 87, 198, 121, 159]);
#[doc = "*Required features: `Win32_Media_PictureAcquisition`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct USER_INPUT_STRING_TYPE(pub i32);
pub const USER_INPUT_DEFAULT: USER_INPUT_STRING_TYPE = USER_INPUT_STRING_TYPE(0i32);
pub const USER_INPUT_PATH_ELEMENT: USER_INPUT_STRING_TYPE = USER_INPUT_STRING_TYPE(1i32);
impl ::std::convert::From<i32> for USER_INPUT_STRING_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for USER_INPUT_STRING_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
