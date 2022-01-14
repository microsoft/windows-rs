#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IImageList_Impl: Sized {
    fn Add(&mut self, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::Result<i32>;
    fn ReplaceIcon(&mut self, i: i32, hicon: super::WindowsAndMessaging::HICON) -> ::windows::core::Result<i32>;
    fn SetOverlayImage(&mut self, iimage: i32, ioverlay: i32) -> ::windows::core::Result<()>;
    fn Replace(&mut self, i: i32, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::Result<()>;
    fn AddMasked(&mut self, hbmimage: super::super::Graphics::Gdi::HBITMAP, crmask: u32) -> ::windows::core::Result<i32>;
    fn Draw(&mut self, pimldp: *const IMAGELISTDRAWPARAMS) -> ::windows::core::Result<()>;
    fn Remove(&mut self, i: i32) -> ::windows::core::Result<()>;
    fn GetIcon(&mut self, i: i32, flags: u32) -> ::windows::core::Result<super::WindowsAndMessaging::HICON>;
    fn GetImageInfo(&mut self, i: i32) -> ::windows::core::Result<IMAGEINFO>;
    fn Copy(&mut self, idst: i32, punksrc: &::core::option::Option<::windows::core::IUnknown>, isrc: i32, uflags: u32) -> ::windows::core::Result<()>;
    fn Merge(&mut self, i1: i32, punk2: &::core::option::Option<::windows::core::IUnknown>, i2: i32, dx: i32, dy: i32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Clone(&mut self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetImageRect(&mut self, i: i32) -> ::windows::core::Result<super::super::Foundation::RECT>;
    fn GetIconSize(&mut self, cx: *mut i32, cy: *mut i32) -> ::windows::core::Result<()>;
    fn SetIconSize(&mut self, cx: i32, cy: i32) -> ::windows::core::Result<()>;
    fn GetImageCount(&mut self) -> ::windows::core::Result<i32>;
    fn SetImageCount(&mut self, unewcount: u32) -> ::windows::core::Result<()>;
    fn SetBkColor(&mut self, clrbk: u32) -> ::windows::core::Result<u32>;
    fn GetBkColor(&mut self) -> ::windows::core::Result<u32>;
    fn BeginDrag(&mut self, itrack: i32, dxhotspot: i32, dyhotspot: i32) -> ::windows::core::Result<()>;
    fn EndDrag(&mut self) -> ::windows::core::Result<()>;
    fn DragEnter(&mut self, hwndlock: super::super::Foundation::HWND, x: i32, y: i32) -> ::windows::core::Result<()>;
    fn DragLeave(&mut self, hwndlock: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn DragMove(&mut self, x: i32, y: i32) -> ::windows::core::Result<()>;
    fn SetDragCursorImage(&mut self, punk: &::core::option::Option<::windows::core::IUnknown>, idrag: i32, dxhotspot: i32, dyhotspot: i32) -> ::windows::core::Result<()>;
    fn DragShowNolock(&mut self, fshow: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetDragImage(&mut self, ppt: *mut super::super::Foundation::POINT, ppthotspot: *mut super::super::Foundation::POINT, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetItemFlags(&mut self, i: i32) -> ::windows::core::Result<IMAGE_LIST_ITEM_FLAGS>;
    fn GetOverlayImage(&mut self, ioverlay: i32) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IImageList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageList_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImageList_Vtbl {
        unsafe extern "system" fn Add<Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP, pi: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(::core::mem::transmute_copy(&hbmimage), ::core::mem::transmute_copy(&hbmmask)) {
                ::core::result::Result::Ok(ok__) => {
                    *pi = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReplaceIcon<Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i: i32, hicon: super::WindowsAndMessaging::HICON, pi: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReplaceIcon(::core::mem::transmute_copy(&i), ::core::mem::transmute_copy(&hicon)) {
                ::core::result::Result::Ok(ok__) => {
                    *pi = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOverlayImage<Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iimage: i32, ioverlay: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOverlayImage(::core::mem::transmute_copy(&iimage), ::core::mem::transmute_copy(&ioverlay)).into()
        }
        unsafe extern "system" fn Replace<Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i: i32, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Replace(::core::mem::transmute_copy(&i), ::core::mem::transmute_copy(&hbmimage), ::core::mem::transmute_copy(&hbmmask)).into()
        }
        unsafe extern "system" fn AddMasked<Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hbmimage: super::super::Graphics::Gdi::HBITMAP, crmask: u32, pi: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddMasked(::core::mem::transmute_copy(&hbmimage), ::core::mem::transmute_copy(&crmask)) {
                ::core::result::Result::Ok(ok__) => {
                    *pi = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Draw<Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimldp: *const IMAGELISTDRAWPARAMS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Draw(::core::mem::transmute_copy(&pimldp)).into()
        }
        unsafe extern "system" fn Remove<Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&i)).into()
        }
        unsafe extern "system" fn GetIcon<Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i: i32, flags: u32, picon: *mut super::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIcon(::core::mem::transmute_copy(&i), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *picon = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImageInfo<Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i: i32, pimageinfo: *mut IMAGEINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetImageInfo(::core::mem::transmute_copy(&i)) {
                ::core::result::Result::Ok(ok__) => {
                    *pimageinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Copy<Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idst: i32, punksrc: *mut ::core::ffi::c_void, isrc: i32, uflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Copy(::core::mem::transmute_copy(&idst), ::core::mem::transmute(&punksrc), ::core::mem::transmute_copy(&isrc), ::core::mem::transmute_copy(&uflags)).into()
        }
        unsafe extern "system" fn Merge<Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i1: i32, punk2: *mut ::core::ffi::c_void, i2: i32, dx: i32, dy: i32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Merge(::core::mem::transmute_copy(&i1), ::core::mem::transmute(&punk2), ::core::mem::transmute_copy(&i2), ::core::mem::transmute_copy(&dx), ::core::mem::transmute_copy(&dy), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn Clone<Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clone(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetImageRect<Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i: i32, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetImageRect(::core::mem::transmute_copy(&i)) {
                ::core::result::Result::Ok(ok__) => {
                    *prc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIconSize<Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cx: *mut i32, cy: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetIconSize(::core::mem::transmute_copy(&cx), ::core::mem::transmute_copy(&cy)).into()
        }
        unsafe extern "system" fn SetIconSize<Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cx: i32, cy: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIconSize(::core::mem::transmute_copy(&cx), ::core::mem::transmute_copy(&cy)).into()
        }
        unsafe extern "system" fn GetImageCount<Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pi: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetImageCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pi = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImageCount<Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unewcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetImageCount(::core::mem::transmute_copy(&unewcount)).into()
        }
        unsafe extern "system" fn SetBkColor<Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clrbk: u32, pclr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBkColor(::core::mem::transmute_copy(&clrbk)) {
                ::core::result::Result::Ok(ok__) => {
                    *pclr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBkColor<Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBkColor() {
                ::core::result::Result::Ok(ok__) => {
                    *pclr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginDrag<Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itrack: i32, dxhotspot: i32, dyhotspot: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginDrag(::core::mem::transmute_copy(&itrack), ::core::mem::transmute_copy(&dxhotspot), ::core::mem::transmute_copy(&dyhotspot)).into()
        }
        unsafe extern "system" fn EndDrag<Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndDrag().into()
        }
        unsafe extern "system" fn DragEnter<Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndlock: super::super::Foundation::HWND, x: i32, y: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DragEnter(::core::mem::transmute_copy(&hwndlock), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn DragLeave<Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndlock: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DragLeave(::core::mem::transmute_copy(&hwndlock)).into()
        }
        unsafe extern "system" fn DragMove<Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DragMove(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn SetDragCursorImage<Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void, idrag: i32, dxhotspot: i32, dyhotspot: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDragCursorImage(::core::mem::transmute(&punk), ::core::mem::transmute_copy(&idrag), ::core::mem::transmute_copy(&dxhotspot), ::core::mem::transmute_copy(&dyhotspot)).into()
        }
        unsafe extern "system" fn DragShowNolock<Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DragShowNolock(::core::mem::transmute_copy(&fshow)).into()
        }
        unsafe extern "system" fn GetDragImage<Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppt: *mut super::super::Foundation::POINT, ppthotspot: *mut super::super::Foundation::POINT, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDragImage(::core::mem::transmute_copy(&ppt), ::core::mem::transmute_copy(&ppthotspot), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetItemFlags<Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i: i32, dwflags: *mut IMAGE_LIST_ITEM_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemFlags(::core::mem::transmute_copy(&i)) {
                ::core::result::Result::Ok(ok__) => {
                    *dwflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOverlayImage<Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ioverlay: i32, piindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOverlayImage(::core::mem::transmute_copy(&ioverlay)) {
                ::core::result::Result::Ok(ok__) => {
                    *piindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Add: Add::<Impl, IMPL_OFFSET>,
            ReplaceIcon: ReplaceIcon::<Impl, IMPL_OFFSET>,
            SetOverlayImage: SetOverlayImage::<Impl, IMPL_OFFSET>,
            Replace: Replace::<Impl, IMPL_OFFSET>,
            AddMasked: AddMasked::<Impl, IMPL_OFFSET>,
            Draw: Draw::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            GetIcon: GetIcon::<Impl, IMPL_OFFSET>,
            GetImageInfo: GetImageInfo::<Impl, IMPL_OFFSET>,
            Copy: Copy::<Impl, IMPL_OFFSET>,
            Merge: Merge::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
            GetImageRect: GetImageRect::<Impl, IMPL_OFFSET>,
            GetIconSize: GetIconSize::<Impl, IMPL_OFFSET>,
            SetIconSize: SetIconSize::<Impl, IMPL_OFFSET>,
            GetImageCount: GetImageCount::<Impl, IMPL_OFFSET>,
            SetImageCount: SetImageCount::<Impl, IMPL_OFFSET>,
            SetBkColor: SetBkColor::<Impl, IMPL_OFFSET>,
            GetBkColor: GetBkColor::<Impl, IMPL_OFFSET>,
            BeginDrag: BeginDrag::<Impl, IMPL_OFFSET>,
            EndDrag: EndDrag::<Impl, IMPL_OFFSET>,
            DragEnter: DragEnter::<Impl, IMPL_OFFSET>,
            DragLeave: DragLeave::<Impl, IMPL_OFFSET>,
            DragMove: DragMove::<Impl, IMPL_OFFSET>,
            SetDragCursorImage: SetDragCursorImage::<Impl, IMPL_OFFSET>,
            DragShowNolock: DragShowNolock::<Impl, IMPL_OFFSET>,
            GetDragImage: GetDragImage::<Impl, IMPL_OFFSET>,
            GetItemFlags: GetItemFlags::<Impl, IMPL_OFFSET>,
            GetOverlayImage: GetOverlayImage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImageList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IImageList2_Impl: Sized + IImageList_Impl {
    fn Resize(&mut self, cxnewiconsize: i32, cynewiconsize: i32) -> ::windows::core::Result<()>;
    fn GetOriginalSize(&mut self, iimage: i32, dwflags: u32, pcx: *mut i32, pcy: *mut i32) -> ::windows::core::Result<()>;
    fn SetOriginalSize(&mut self, iimage: i32, cx: i32, cy: i32) -> ::windows::core::Result<()>;
    fn SetCallback(&mut self, punk: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetCallback(&mut self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn ForceImagePresent(&mut self, iimage: i32, dwflags: u32) -> ::windows::core::Result<()>;
    fn DiscardImages(&mut self, ifirstimage: i32, ilastimage: i32, dwflags: u32) -> ::windows::core::Result<()>;
    fn PreloadImages(&mut self, pimldp: *const IMAGELISTDRAWPARAMS) -> ::windows::core::Result<()>;
    fn GetStatistics(&mut self, pils: *mut IMAGELISTSTATS) -> ::windows::core::Result<()>;
    fn Initialize(&mut self, cx: i32, cy: i32, flags: IMAGELIST_CREATION_FLAGS, cinitial: i32, cgrow: i32) -> ::windows::core::Result<()>;
    fn Replace2(&mut self, i: i32, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP, punk: &::core::option::Option<::windows::core::IUnknown>, dwflags: u32) -> ::windows::core::Result<()>;
    fn ReplaceFromImageList(&mut self, i: i32, pil: &::core::option::Option<IImageList>, isrc: i32, punk: &::core::option::Option<::windows::core::IUnknown>, dwflags: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IImageList2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageList2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImageList2_Vtbl {
        unsafe extern "system" fn Resize<Impl: IImageList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cxnewiconsize: i32, cynewiconsize: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resize(::core::mem::transmute_copy(&cxnewiconsize), ::core::mem::transmute_copy(&cynewiconsize)).into()
        }
        unsafe extern "system" fn GetOriginalSize<Impl: IImageList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iimage: i32, dwflags: u32, pcx: *mut i32, pcy: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOriginalSize(::core::mem::transmute_copy(&iimage), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pcx), ::core::mem::transmute_copy(&pcy)).into()
        }
        unsafe extern "system" fn SetOriginalSize<Impl: IImageList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iimage: i32, cx: i32, cy: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOriginalSize(::core::mem::transmute_copy(&iimage), ::core::mem::transmute_copy(&cx), ::core::mem::transmute_copy(&cy)).into()
        }
        unsafe extern "system" fn SetCallback<Impl: IImageList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCallback(::core::mem::transmute(&punk)).into()
        }
        unsafe extern "system" fn GetCallback<Impl: IImageList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCallback(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn ForceImagePresent<Impl: IImageList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iimage: i32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ForceImagePresent(::core::mem::transmute_copy(&iimage), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn DiscardImages<Impl: IImageList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ifirstimage: i32, ilastimage: i32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DiscardImages(::core::mem::transmute_copy(&ifirstimage), ::core::mem::transmute_copy(&ilastimage), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn PreloadImages<Impl: IImageList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimldp: *const IMAGELISTDRAWPARAMS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PreloadImages(::core::mem::transmute_copy(&pimldp)).into()
        }
        unsafe extern "system" fn GetStatistics<Impl: IImageList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pils: *mut IMAGELISTSTATS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStatistics(::core::mem::transmute_copy(&pils)).into()
        }
        unsafe extern "system" fn Initialize<Impl: IImageList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cx: i32, cy: i32, flags: IMAGELIST_CREATION_FLAGS, cinitial: i32, cgrow: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&cx), ::core::mem::transmute_copy(&cy), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&cinitial), ::core::mem::transmute_copy(&cgrow)).into()
        }
        unsafe extern "system" fn Replace2<Impl: IImageList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i: i32, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP, punk: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Replace2(::core::mem::transmute_copy(&i), ::core::mem::transmute_copy(&hbmimage), ::core::mem::transmute_copy(&hbmmask), ::core::mem::transmute(&punk), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn ReplaceFromImageList<Impl: IImageList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i: i32, pil: ::windows::core::RawPtr, isrc: i32, punk: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReplaceFromImageList(::core::mem::transmute_copy(&i), ::core::mem::transmute(&pil), ::core::mem::transmute_copy(&isrc), ::core::mem::transmute(&punk), ::core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base: IImageList_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Resize: Resize::<Impl, IMPL_OFFSET>,
            GetOriginalSize: GetOriginalSize::<Impl, IMPL_OFFSET>,
            SetOriginalSize: SetOriginalSize::<Impl, IMPL_OFFSET>,
            SetCallback: SetCallback::<Impl, IMPL_OFFSET>,
            GetCallback: GetCallback::<Impl, IMPL_OFFSET>,
            ForceImagePresent: ForceImagePresent::<Impl, IMPL_OFFSET>,
            DiscardImages: DiscardImages::<Impl, IMPL_OFFSET>,
            PreloadImages: PreloadImages::<Impl, IMPL_OFFSET>,
            GetStatistics: GetStatistics::<Impl, IMPL_OFFSET>,
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Replace2: Replace2::<Impl, IMPL_OFFSET>,
            ReplaceFromImageList: ReplaceFromImageList::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImageList2 as ::windows::core::Interface>::IID
    }
}
