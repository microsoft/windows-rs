#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDummyHICONIncluder(::windows::runtime::IUnknown);
impl IDummyHICONIncluder {
    #[cfg(all(
        feature = "Win32_Graphics_Gdi",
        feature = "Win32_UI_WindowsAndMessaging"
    ))]
    pub unsafe fn Dummy<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::UI::WindowsAndMessaging::HICON>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Gdi::HDC>,
    >(
        &self,
        h1: Param0,
        h2: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            h1.into_param().abi(),
            h2.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDummyHICONIncluder {
    type Vtable = IDummyHICONIncluder_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2490994910,
        52264,
        4562,
        [160, 247, 0, 128, 95, 133, 143, 177],
    );
}
impl ::std::convert::From<IDummyHICONIncluder> for ::windows::runtime::IUnknown {
    fn from(value: IDummyHICONIncluder) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDummyHICONIncluder> for ::windows::runtime::IUnknown {
    fn from(value: &IDummyHICONIncluder) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDummyHICONIncluder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDummyHICONIncluder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDummyHICONIncluder_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(
        feature = "Win32_Graphics_Gdi",
        feature = "Win32_UI_WindowsAndMessaging"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        h1: super::super::super::UI::WindowsAndMessaging::HICON,
        h2: super::super::super::Graphics::Gdi::HDC,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Graphics_Gdi",
        feature = "Win32_UI_WindowsAndMessaging"
    )))]
    usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IThumbnailExtractor(::windows::runtime::IUnknown);
impl IThumbnailExtractor {
    #[cfg(all(
        feature = "Win32_Graphics_Gdi",
        feature = "Win32_System_Com_StructuredStorage"
    ))]
    pub unsafe fn ExtractThumbnail<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::StructuredStorage::IStorage>,
    >(
        &self,
        pstg: Param0,
        ullength: u32,
        ulheight: u32,
        puloutputlength: *mut u32,
        puloutputheight: *mut u32,
        phoutputbitmap: *mut super::super::super::Graphics::Gdi::HBITMAP,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pstg.into_param().abi(),
            ::std::mem::transmute(ullength),
            ::std::mem::transmute(ulheight),
            ::std::mem::transmute(puloutputlength),
            ::std::mem::transmute(puloutputheight),
            ::std::mem::transmute(phoutputbitmap),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn OnFileUpdated<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::StructuredStorage::IStorage>,
    >(
        &self,
        pstg: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            pstg.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IThumbnailExtractor {
    type Vtable = IThumbnailExtractor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2526922504,
        23670,
        4561,
        [141, 134, 0, 0, 248, 4, 176, 87],
    );
}
impl ::std::convert::From<IThumbnailExtractor> for ::windows::runtime::IUnknown {
    fn from(value: IThumbnailExtractor) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IThumbnailExtractor> for ::windows::runtime::IUnknown {
    fn from(value: &IThumbnailExtractor) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IThumbnailExtractor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IThumbnailExtractor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IThumbnailExtractor_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(
        feature = "Win32_Graphics_Gdi",
        feature = "Win32_System_Com_StructuredStorage"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pstg: ::windows::runtime::RawPtr,
        ullength: u32,
        ulheight: u32,
        puloutputlength: *mut u32,
        puloutputheight: *mut u32,
        phoutputbitmap: *mut super::super::super::Graphics::Gdi::HBITMAP,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Graphics_Gdi",
        feature = "Win32_System_Com_StructuredStorage"
    )))]
    usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pstg: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))] usize,
);
