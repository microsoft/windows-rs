pub trait IImageListImpl: Sized {
    fn Add();
    fn ReplaceIcon();
    fn SetOverlayImage();
    fn Replace();
    fn AddMasked();
    fn Draw();
    fn Remove();
    fn GetIcon();
    fn GetImageInfo();
    fn Copy();
    fn Merge();
    fn Clone();
    fn GetImageRect();
    fn GetIconSize();
    fn SetIconSize();
    fn GetImageCount();
    fn SetImageCount();
    fn SetBkColor();
    fn GetBkColor();
    fn BeginDrag();
    fn EndDrag();
    fn DragEnter();
    fn DragLeave();
    fn DragMove();
    fn SetDragCursorImage();
    fn DragShowNolock();
    fn GetDragImage();
    fn GetItemFlags();
    fn GetOverlayImage();
}
impl ::windows::core::RuntimeName for IImageList {
    const NAME: &'static str = "Windows.Win32.UI.Controls.IImageList";
}
impl IImageListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageListImpl, const OFFSET: isize>() -> IImageListVtbl {
        unsafe extern "system" fn Add<Impl: IImageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP, pi: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(&*(&hbmimage as *const <super::super::Graphics::Gdi::HBITMAP as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Gdi::HBITMAP as ::windows::core::DefaultType>::DefaultType), &*(&hbmmask as *const <super::super::Graphics::Gdi::HBITMAP as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Gdi::HBITMAP as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pi)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReplaceIcon<Impl: IImageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i: i32, hicon: super::WindowsAndMessaging::HICON, pi: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReplaceIcon(i, &*(&hicon as *const <super::WindowsAndMessaging::HICON as ::windows::core::Abi>::Abi as *const <super::WindowsAndMessaging::HICON as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pi)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOverlayImage<Impl: IImageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iimage: i32, ioverlay: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOverlayImage(iimage, ioverlay) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Replace<Impl: IImageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i: i32, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Replace(i, &*(&hbmimage as *const <super::super::Graphics::Gdi::HBITMAP as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Gdi::HBITMAP as ::windows::core::DefaultType>::DefaultType), &*(&hbmmask as *const <super::super::Graphics::Gdi::HBITMAP as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Gdi::HBITMAP as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddMasked<Impl: IImageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hbmimage: super::super::Graphics::Gdi::HBITMAP, crmask: u32, pi: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddMasked(&*(&hbmimage as *const <super::super::Graphics::Gdi::HBITMAP as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Gdi::HBITMAP as ::windows::core::DefaultType>::DefaultType), crmask, ::core::mem::transmute_copy(&pi)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Draw<Impl: IImageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimldp: *const IMAGELISTDRAWPARAMS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Draw(&*(&pimldp as *const <IMAGELISTDRAWPARAMS as ::windows::core::Abi>::Abi as *const <IMAGELISTDRAWPARAMS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: IImageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Remove(i) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIcon<Impl: IImageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i: i32, flags: u32, picon: *mut super::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIcon(i, flags, ::core::mem::transmute_copy(&picon)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImageInfo<Impl: IImageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i: i32, pimageinfo: *mut IMAGEINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetImageInfo(i, ::core::mem::transmute_copy(&pimageinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Copy<Impl: IImageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idst: i32, punksrc: *mut ::core::ffi::c_void, isrc: i32, uflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Copy(idst, &*(&punksrc as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), isrc, uflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Merge<Impl: IImageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i1: i32, punk2: *mut ::core::ffi::c_void, i2: i32, dx: i32, dy: i32, riid: &::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Merge(i1, &*(&punk2 as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), i2, dx, dy, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IImageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: &::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImageRect<Impl: IImageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i: i32, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetImageRect(i, ::core::mem::transmute_copy(&prc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIconSize<Impl: IImageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cx: *mut i32, cy: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIconSize(::core::mem::transmute_copy(&cx), ::core::mem::transmute_copy(&cy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIconSize<Impl: IImageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cx: i32, cy: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIconSize(cx, cy) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImageCount<Impl: IImageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pi: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetImageCount(::core::mem::transmute_copy(&pi)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImageCount<Impl: IImageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unewcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetImageCount(unewcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBkColor<Impl: IImageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clrbk: u32, pclr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBkColor(clrbk, ::core::mem::transmute_copy(&pclr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBkColor<Impl: IImageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBkColor(::core::mem::transmute_copy(&pclr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginDrag<Impl: IImageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itrack: i32, dxhotspot: i32, dyhotspot: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginDrag(itrack, dxhotspot, dyhotspot) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndDrag<Impl: IImageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndDrag() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DragEnter<Impl: IImageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndlock: super::super::Foundation::HWND, x: i32, y: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DragEnter(&*(&hwndlock as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), x, y) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DragLeave<Impl: IImageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndlock: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DragLeave(&*(&hwndlock as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DragMove<Impl: IImageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DragMove(x, y) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDragCursorImage<Impl: IImageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void, idrag: i32, dxhotspot: i32, dyhotspot: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDragCursorImage(&*(&punk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), idrag, dxhotspot, dyhotspot) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DragShowNolock<Impl: IImageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DragShowNolock(&*(&fshow as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDragImage<Impl: IImageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppt: *mut super::super::Foundation::POINT, ppthotspot: *mut super::super::Foundation::POINT, riid: &::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDragImage(::core::mem::transmute_copy(&ppt), ::core::mem::transmute_copy(&ppthotspot), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemFlags<Impl: IImageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i: i32, dwflags: *mut IMAGE_LIST_ITEM_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemFlags(i, ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOverlayImage<Impl: IImageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ioverlay: i32, piindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOverlayImage(ioverlay, ::core::mem::transmute_copy(&piindex)) {
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
            ::windows::core::GetRuntimeClassName::<IImageList>,
            ::windows::core::GetTrustLevel,
            Add::<Impl, OFFSET>,
            ReplaceIcon::<Impl, OFFSET>,
            SetOverlayImage::<Impl, OFFSET>,
            Replace::<Impl, OFFSET>,
            AddMasked::<Impl, OFFSET>,
            Draw::<Impl, OFFSET>,
            Remove::<Impl, OFFSET>,
            GetIcon::<Impl, OFFSET>,
            GetImageInfo::<Impl, OFFSET>,
            Copy::<Impl, OFFSET>,
            Merge::<Impl, OFFSET>,
            Clone::<Impl, OFFSET>,
            GetImageRect::<Impl, OFFSET>,
            GetIconSize::<Impl, OFFSET>,
            SetIconSize::<Impl, OFFSET>,
            GetImageCount::<Impl, OFFSET>,
            SetImageCount::<Impl, OFFSET>,
            SetBkColor::<Impl, OFFSET>,
            GetBkColor::<Impl, OFFSET>,
            BeginDrag::<Impl, OFFSET>,
            EndDrag::<Impl, OFFSET>,
            DragEnter::<Impl, OFFSET>,
            DragLeave::<Impl, OFFSET>,
            DragMove::<Impl, OFFSET>,
            SetDragCursorImage::<Impl, OFFSET>,
            DragShowNolock::<Impl, OFFSET>,
            GetDragImage::<Impl, OFFSET>,
            GetItemFlags::<Impl, OFFSET>,
            GetOverlayImage::<Impl, OFFSET>,
        )
    }
}
pub trait IImageList2Impl: Sized + IImageListImpl {
    fn Resize();
    fn GetOriginalSize();
    fn SetOriginalSize();
    fn SetCallback();
    fn GetCallback();
    fn ForceImagePresent();
    fn DiscardImages();
    fn PreloadImages();
    fn GetStatistics();
    fn Initialize();
    fn Replace2();
    fn ReplaceFromImageList();
}
impl ::windows::core::RuntimeName for IImageList2 {
    const NAME: &'static str = "Windows.Win32.UI.Controls.IImageList2";
}
impl IImageList2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageList2Impl, const OFFSET: isize>() -> IImageList2Vtbl {
        unsafe extern "system" fn Resize<Impl: IImageList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cxnewiconsize: i32, cynewiconsize: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Resize(cxnewiconsize, cynewiconsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOriginalSize<Impl: IImageList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iimage: i32, dwflags: u32, pcx: *mut i32, pcy: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOriginalSize(iimage, dwflags, ::core::mem::transmute_copy(&pcx), ::core::mem::transmute_copy(&pcy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOriginalSize<Impl: IImageList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iimage: i32, cx: i32, cy: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOriginalSize(iimage, cx, cy) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCallback<Impl: IImageList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCallback(&*(&punk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCallback<Impl: IImageList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: &::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCallback(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForceImagePresent<Impl: IImageList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iimage: i32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForceImagePresent(iimage, dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiscardImages<Impl: IImageList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ifirstimage: i32, ilastimage: i32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DiscardImages(ifirstimage, ilastimage, dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreloadImages<Impl: IImageList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimldp: *const IMAGELISTDRAWPARAMS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreloadImages(&*(&pimldp as *const <IMAGELISTDRAWPARAMS as ::windows::core::Abi>::Abi as *const <IMAGELISTDRAWPARAMS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatistics<Impl: IImageList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pils: *mut IMAGELISTSTATS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatistics(&*(&pils as *const <IMAGELISTSTATS as ::windows::core::Abi>::Abi as *const <IMAGELISTSTATS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IImageList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cx: i32, cy: i32, flags: IMAGELIST_CREATION_FLAGS, cinitial: i32, cgrow: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(cx, cy, flags, cinitial, cgrow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Replace2<Impl: IImageList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i: i32, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP, punk: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Replace2(
                i,
                &*(&hbmimage as *const <super::super::Graphics::Gdi::HBITMAP as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Gdi::HBITMAP as ::windows::core::DefaultType>::DefaultType),
                &*(&hbmmask as *const <super::super::Graphics::Gdi::HBITMAP as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Gdi::HBITMAP as ::windows::core::DefaultType>::DefaultType),
                &*(&punk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                dwflags,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReplaceFromImageList<Impl: IImageList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i: i32, pil: ::windows::core::RawPtr, isrc: i32, punk: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReplaceFromImageList(i, &*(&pil as *const <IImageList as ::windows::core::Abi>::Abi as *const <IImageList as ::windows::core::DefaultType>::DefaultType), isrc, &*(&punk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), dwflags) {
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
            ::windows::core::GetRuntimeClassName::<IImageList2>,
            ::windows::core::GetTrustLevel,
            Resize::<Impl, OFFSET>,
            GetOriginalSize::<Impl, OFFSET>,
            SetOriginalSize::<Impl, OFFSET>,
            SetCallback::<Impl, OFFSET>,
            GetCallback::<Impl, OFFSET>,
            ForceImagePresent::<Impl, OFFSET>,
            DiscardImages::<Impl, OFFSET>,
            PreloadImages::<Impl, OFFSET>,
            GetStatistics::<Impl, OFFSET>,
            Initialize::<Impl, OFFSET>,
            Replace2::<Impl, OFFSET>,
            ReplaceFromImageList::<Impl, OFFSET>,
        )
    }
}
