#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IPdfDocument(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPdfDocument {
    type Vtable = IPdfDocument_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac7ebedd_80fa_4089_846e_81b77ff5a86c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPdfDocument_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pageindex: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub PageCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub IsPasswordProtected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPdfDocumentStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPdfDocumentStatics {
    type Vtable = IPdfDocumentStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x433a0b5f_c007_4788_90f2_08143d922599);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPdfDocumentStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LoadFromFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LoadFromFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LoadFromFileWithPasswordAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LoadFromFileWithPasswordAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub LoadFromStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    LoadFromStreamAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub LoadFromStreamWithPasswordAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    LoadFromStreamWithPasswordAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPdfPage(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPdfPage {
    type Vtable = IPdfPage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9db4b0c8_5320_4cfc_ad76_493fdad0e594);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPdfPage_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub RenderToStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    RenderToStreamAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub RenderWithOptionsToStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    RenderWithOptionsToStreamAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PreparePageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PreparePageAsync: usize,
    pub Index: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Size: usize,
    pub Dimensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Rotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PdfPageRotation) -> ::windows::core::HRESULT,
    pub PreferredZoom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPdfPageDimensions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPdfPageDimensions {
    type Vtable = IPdfPageDimensions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22170471_313e_44e8_835d_63a3e7624a10);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPdfPageDimensions_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub MediaBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MediaBox: usize,
    #[cfg(feature = "Foundation")]
    pub CropBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CropBox: usize,
    #[cfg(feature = "Foundation")]
    pub BleedBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BleedBox: usize,
    #[cfg(feature = "Foundation")]
    pub TrimBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrimBox: usize,
    #[cfg(feature = "Foundation")]
    pub ArtBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ArtBox: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPdfPageRenderOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPdfPageRenderOptions {
    type Vtable = IPdfPageRenderOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c98056f_b7cf_4c29_9a04_52d90267f425);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPdfPageRenderOptions_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub SourceRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SourceRect: usize,
    #[cfg(feature = "Foundation")]
    pub SetSourceRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSourceRect: usize,
    pub DestinationWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetDestinationWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub DestinationHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetDestinationHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")]
    pub BackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    BackgroundColor: usize,
    #[cfg(feature = "UI")]
    pub SetBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetBackgroundColor: usize,
    pub IsIgnoringHighContrast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsIgnoringHighContrast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub BitmapEncoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetBitmapEncoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Data_Pdf\"`*"]
#[repr(transparent)]
pub struct PdfDocument(::windows::core::IUnknown);
impl PdfDocument {
    #[doc = "*Required features: `\"Data_Pdf\"`*"]
    pub fn GetPage(&self, pageindex: u32) -> ::windows::core::Result<PdfPage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetPage)(::core::mem::transmute_copy(this), pageindex, &mut result__).from_abi::<PdfPage>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Pdf\"`*"]
    pub fn PageCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PageCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Pdf\"`*"]
    pub fn IsPasswordProtected(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPasswordProtected)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Pdf\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LoadFromFileAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>>(file: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PdfDocument>> {
        Self::IPdfDocumentStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LoadFromFileAsync)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PdfDocument>>(result__)
        })
    }
    #[doc = "*Required features: `\"Data_Pdf\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LoadFromFileWithPasswordAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(file: Param0, password: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PdfDocument>> {
        Self::IPdfDocumentStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LoadFromFileWithPasswordAsync)(::core::mem::transmute_copy(this), file.into_param().abi(), password.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PdfDocument>>(result__)
        })
    }
    #[doc = "*Required features: `\"Data_Pdf\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn LoadFromStreamAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(inputstream: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PdfDocument>> {
        Self::IPdfDocumentStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LoadFromStreamAsync)(::core::mem::transmute_copy(this), inputstream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PdfDocument>>(result__)
        })
    }
    #[doc = "*Required features: `\"Data_Pdf\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn LoadFromStreamWithPasswordAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(inputstream: Param0, password: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PdfDocument>> {
        Self::IPdfDocumentStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LoadFromStreamWithPasswordAsync)(::core::mem::transmute_copy(this), inputstream.into_param().abi(), password.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PdfDocument>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPdfDocumentStatics<R, F: FnOnce(&IPdfDocumentStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PdfDocument, IPdfDocumentStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PdfDocument {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PdfDocument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PdfDocument {}
impl ::core::fmt::Debug for PdfDocument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PdfDocument").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PdfDocument {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Pdf.PdfDocument;{ac7ebedd-80fa-4089-846e-81b77ff5a86c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PdfDocument {
    type Vtable = IPdfDocument_Vtbl;
    const IID: ::windows::core::GUID = <IPdfDocument as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PdfDocument {
    const NAME: &'static str = "Windows.Data.Pdf.PdfDocument";
}
impl ::core::convert::From<PdfDocument> for ::windows::core::IUnknown {
    fn from(value: PdfDocument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PdfDocument> for ::windows::core::IUnknown {
    fn from(value: &PdfDocument) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PdfDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PdfDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PdfDocument> for ::windows::core::IInspectable {
    fn from(value: PdfDocument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PdfDocument> for ::windows::core::IInspectable {
    fn from(value: &PdfDocument) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PdfDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PdfDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PdfDocument {}
unsafe impl ::core::marker::Sync for PdfDocument {}
#[doc = "*Required features: `\"Data_Pdf\"`*"]
#[repr(transparent)]
pub struct PdfPage(::windows::core::IUnknown);
impl PdfPage {
    #[doc = "*Required features: `\"Data_Pdf\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Data_Pdf\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn RenderToStreamAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, outputstream: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RenderToStreamAsync)(::core::mem::transmute_copy(this), outputstream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Pdf\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn RenderWithOptionsToStreamAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>, Param1: ::windows::core::IntoParam<'a, PdfPageRenderOptions>>(&self, outputstream: Param0, options: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RenderWithOptionsToStreamAsync)(::core::mem::transmute_copy(this), outputstream.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Pdf\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PreparePageAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PreparePageAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Pdf\"`*"]
    pub fn Index(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Index)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Pdf\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Size(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Size)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Pdf\"`*"]
    pub fn Dimensions(&self) -> ::windows::core::Result<PdfPageDimensions> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Dimensions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PdfPageDimensions>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Pdf\"`*"]
    pub fn Rotation(&self) -> ::windows::core::Result<PdfPageRotation> {
        let this = self;
        unsafe {
            let mut result__: PdfPageRotation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Rotation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PdfPageRotation>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Pdf\"`*"]
    pub fn PreferredZoom(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PreferredZoom)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
}
impl ::core::clone::Clone for PdfPage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PdfPage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PdfPage {}
impl ::core::fmt::Debug for PdfPage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PdfPage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PdfPage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Pdf.PdfPage;{9db4b0c8-5320-4cfc-ad76-493fdad0e594})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PdfPage {
    type Vtable = IPdfPage_Vtbl;
    const IID: ::windows::core::GUID = <IPdfPage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PdfPage {
    const NAME: &'static str = "Windows.Data.Pdf.PdfPage";
}
impl ::core::convert::From<PdfPage> for ::windows::core::IUnknown {
    fn from(value: PdfPage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PdfPage> for ::windows::core::IUnknown {
    fn from(value: &PdfPage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PdfPage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PdfPage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PdfPage> for ::windows::core::IInspectable {
    fn from(value: PdfPage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PdfPage> for ::windows::core::IInspectable {
    fn from(value: &PdfPage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PdfPage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PdfPage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<PdfPage> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: PdfPage) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&PdfPage> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &PdfPage) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for PdfPage {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &PdfPage {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PdfPage {}
unsafe impl ::core::marker::Sync for PdfPage {}
#[doc = "*Required features: `\"Data_Pdf\"`*"]
#[repr(transparent)]
pub struct PdfPageDimensions(::windows::core::IUnknown);
impl PdfPageDimensions {
    #[doc = "*Required features: `\"Data_Pdf\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MediaBox(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MediaBox)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Pdf\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CropBox(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CropBox)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Pdf\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BleedBox(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BleedBox)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Pdf\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrimBox(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrimBox)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Pdf\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ArtBox(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ArtBox)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
}
impl ::core::clone::Clone for PdfPageDimensions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PdfPageDimensions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PdfPageDimensions {}
impl ::core::fmt::Debug for PdfPageDimensions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PdfPageDimensions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PdfPageDimensions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Pdf.PdfPageDimensions;{22170471-313e-44e8-835d-63a3e7624a10})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PdfPageDimensions {
    type Vtable = IPdfPageDimensions_Vtbl;
    const IID: ::windows::core::GUID = <IPdfPageDimensions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PdfPageDimensions {
    const NAME: &'static str = "Windows.Data.Pdf.PdfPageDimensions";
}
impl ::core::convert::From<PdfPageDimensions> for ::windows::core::IUnknown {
    fn from(value: PdfPageDimensions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PdfPageDimensions> for ::windows::core::IUnknown {
    fn from(value: &PdfPageDimensions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PdfPageDimensions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PdfPageDimensions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PdfPageDimensions> for ::windows::core::IInspectable {
    fn from(value: PdfPageDimensions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PdfPageDimensions> for ::windows::core::IInspectable {
    fn from(value: &PdfPageDimensions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PdfPageDimensions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PdfPageDimensions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PdfPageDimensions {}
unsafe impl ::core::marker::Sync for PdfPageDimensions {}
#[doc = "*Required features: `\"Data_Pdf\"`*"]
#[repr(transparent)]
pub struct PdfPageRenderOptions(::windows::core::IUnknown);
impl PdfPageRenderOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PdfPageRenderOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Data_Pdf\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SourceRect(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SourceRect)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Pdf\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSourceRect<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSourceRect)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Data_Pdf\"`*"]
    pub fn DestinationWidth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DestinationWidth)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Pdf\"`*"]
    pub fn SetDestinationWidth(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDestinationWidth)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Data_Pdf\"`*"]
    pub fn DestinationHeight(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DestinationHeight)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Pdf\"`*"]
    pub fn SetDestinationHeight(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDestinationHeight)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Data_Pdf\"`, `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn BackgroundColor(&self) -> ::windows::core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BackgroundColor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Pdf\"`, `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetBackgroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBackgroundColor)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Data_Pdf\"`*"]
    pub fn IsIgnoringHighContrast(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsIgnoringHighContrast)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Pdf\"`*"]
    pub fn SetIsIgnoringHighContrast(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsIgnoringHighContrast)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Data_Pdf\"`*"]
    pub fn BitmapEncoderId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BitmapEncoderId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Pdf\"`*"]
    pub fn SetBitmapEncoderId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBitmapEncoderId)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for PdfPageRenderOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PdfPageRenderOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PdfPageRenderOptions {}
impl ::core::fmt::Debug for PdfPageRenderOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PdfPageRenderOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PdfPageRenderOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Pdf.PdfPageRenderOptions;{3c98056f-b7cf-4c29-9a04-52d90267f425})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PdfPageRenderOptions {
    type Vtable = IPdfPageRenderOptions_Vtbl;
    const IID: ::windows::core::GUID = <IPdfPageRenderOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PdfPageRenderOptions {
    const NAME: &'static str = "Windows.Data.Pdf.PdfPageRenderOptions";
}
impl ::core::convert::From<PdfPageRenderOptions> for ::windows::core::IUnknown {
    fn from(value: PdfPageRenderOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PdfPageRenderOptions> for ::windows::core::IUnknown {
    fn from(value: &PdfPageRenderOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PdfPageRenderOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PdfPageRenderOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PdfPageRenderOptions> for ::windows::core::IInspectable {
    fn from(value: PdfPageRenderOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PdfPageRenderOptions> for ::windows::core::IInspectable {
    fn from(value: &PdfPageRenderOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PdfPageRenderOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PdfPageRenderOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PdfPageRenderOptions {}
unsafe impl ::core::marker::Sync for PdfPageRenderOptions {}
#[doc = "*Required features: `\"Data_Pdf\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PdfPageRotation(pub i32);
impl PdfPageRotation {
    pub const Normal: Self = Self(0i32);
    pub const Rotate90: Self = Self(1i32);
    pub const Rotate180: Self = Self(2i32);
    pub const Rotate270: Self = Self(3i32);
}
impl ::core::marker::Copy for PdfPageRotation {}
impl ::core::clone::Clone for PdfPageRotation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PdfPageRotation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PdfPageRotation {
    type Abi = Self;
}
impl ::core::fmt::Debug for PdfPageRotation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PdfPageRotation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PdfPageRotation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Data.Pdf.PdfPageRotation;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
