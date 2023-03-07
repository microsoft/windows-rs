#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn BSTR_UserFree(param0: *const u32, param1: *const ::windows::core::BSTR) {
    ::windows::imp::link ! ( "oleaut32.dll""system" fn BSTR_UserFree ( param0 : *const u32 , param1 : *const ::std::mem::MaybeUninit <::windows::core::BSTR > ) -> ( ) );
    BSTR_UserFree(param0, ::core::mem::transmute(param1))
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn BSTR_UserFree64(param0: *const u32, param1: *const ::windows::core::BSTR) {
    ::windows::imp::link ! ( "oleaut32.dll""system" fn BSTR_UserFree64 ( param0 : *const u32 , param1 : *const ::std::mem::MaybeUninit <::windows::core::BSTR > ) -> ( ) );
    BSTR_UserFree64(param0, ::core::mem::transmute(param1))
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn BSTR_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const ::windows::core::BSTR) -> *mut u8 {
    ::windows::imp::link ! ( "oleaut32.dll""system" fn BSTR_UserMarshal ( param0 : *const u32 , param1 : *mut u8 , param2 : *const ::std::mem::MaybeUninit <::windows::core::BSTR > ) -> *mut u8 );
    BSTR_UserMarshal(param0, param1, ::core::mem::transmute(param2))
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn BSTR_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const ::windows::core::BSTR) -> *mut u8 {
    ::windows::imp::link ! ( "oleaut32.dll""system" fn BSTR_UserMarshal64 ( param0 : *const u32 , param1 : *mut u8 , param2 : *const ::std::mem::MaybeUninit <::windows::core::BSTR > ) -> *mut u8 );
    BSTR_UserMarshal64(param0, param1, ::core::mem::transmute(param2))
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn BSTR_UserSize(param0: *const u32, param1: u32, param2: *const ::windows::core::BSTR) -> u32 {
    ::windows::imp::link ! ( "oleaut32.dll""system" fn BSTR_UserSize ( param0 : *const u32 , param1 : u32 , param2 : *const ::std::mem::MaybeUninit <::windows::core::BSTR > ) -> u32 );
    BSTR_UserSize(param0, param1, ::core::mem::transmute(param2))
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn BSTR_UserSize64(param0: *const u32, param1: u32, param2: *const ::windows::core::BSTR) -> u32 {
    ::windows::imp::link ! ( "oleaut32.dll""system" fn BSTR_UserSize64 ( param0 : *const u32 , param1 : u32 , param2 : *const ::std::mem::MaybeUninit <::windows::core::BSTR > ) -> u32 );
    BSTR_UserSize64(param0, param1, ::core::mem::transmute(param2))
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn BSTR_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut ::windows::core::BSTR) -> *mut u8 {
    ::windows::imp::link ! ( "oleaut32.dll""system" fn BSTR_UserUnmarshal ( param0 : *const u32 , param1 : *const u8 , param2 : *mut ::std::mem::MaybeUninit <::windows::core::BSTR > ) -> *mut u8 );
    BSTR_UserUnmarshal(param0, param1, ::core::mem::transmute(param2))
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn BSTR_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut ::windows::core::BSTR) -> *mut u8 {
    ::windows::imp::link ! ( "oleaut32.dll""system" fn BSTR_UserUnmarshal64 ( param0 : *const u32 , param1 : *const u8 , param2 : *mut ::std::mem::MaybeUninit <::windows::core::BSTR > ) -> *mut u8 );
    BSTR_UserUnmarshal64(param0, param1, ::core::mem::transmute(param2))
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn CLIPFORMAT_UserFree(param0: *const u32, param1: *const u16) {
    ::windows::imp::link ! ( "ole32.dll""system" fn CLIPFORMAT_UserFree ( param0 : *const u32 , param1 : *const u16 ) -> ( ) );
    CLIPFORMAT_UserFree(param0, param1)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn CLIPFORMAT_UserFree64(param0: *const u32, param1: *const u16) {
    ::windows::imp::link ! ( "ole32.dll""system" fn CLIPFORMAT_UserFree64 ( param0 : *const u32 , param1 : *const u16 ) -> ( ) );
    CLIPFORMAT_UserFree64(param0, param1)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn CLIPFORMAT_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const u16) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn CLIPFORMAT_UserMarshal ( param0 : *const u32 , param1 : *mut u8 , param2 : *const u16 ) -> *mut u8 );
    CLIPFORMAT_UserMarshal(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn CLIPFORMAT_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const u16) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn CLIPFORMAT_UserMarshal64 ( param0 : *const u32 , param1 : *mut u8 , param2 : *const u16 ) -> *mut u8 );
    CLIPFORMAT_UserMarshal64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn CLIPFORMAT_UserSize(param0: *const u32, param1: u32, param2: *const u16) -> u32 {
    ::windows::imp::link ! ( "ole32.dll""system" fn CLIPFORMAT_UserSize ( param0 : *const u32 , param1 : u32 , param2 : *const u16 ) -> u32 );
    CLIPFORMAT_UserSize(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn CLIPFORMAT_UserSize64(param0: *const u32, param1: u32, param2: *const u16) -> u32 {
    ::windows::imp::link ! ( "ole32.dll""system" fn CLIPFORMAT_UserSize64 ( param0 : *const u32 , param1 : u32 , param2 : *const u16 ) -> u32 );
    CLIPFORMAT_UserSize64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn CLIPFORMAT_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut u16) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn CLIPFORMAT_UserUnmarshal ( param0 : *const u32 , param1 : *const u8 , param2 : *mut u16 ) -> *mut u8 );
    CLIPFORMAT_UserUnmarshal(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn CLIPFORMAT_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut u16) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn CLIPFORMAT_UserUnmarshal64 ( param0 : *const u32 , param1 : *const u8 , param2 : *mut u16 ) -> *mut u8 );
    CLIPFORMAT_UserUnmarshal64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn CoGetMarshalSizeMax<P0>(pulsize: *mut u32, riid: *const ::windows::core::GUID, punk: P0, dwdestcontext: u32, pvdestcontext: ::core::option::Option<*const ::core::ffi::c_void>, mshlflags: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn CoGetMarshalSizeMax ( pulsize : *mut u32 , riid : *const :: windows::core::GUID , punk : * mut::core::ffi::c_void , dwdestcontext : u32 , pvdestcontext : *const ::core::ffi::c_void , mshlflags : u32 ) -> :: windows::core::HRESULT );
    CoGetMarshalSizeMax(pulsize, riid, punk.into_param().abi(), dwdestcontext, ::core::mem::transmute(pvdestcontext.unwrap_or(::std::ptr::null())), mshlflags).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn CoGetStandardMarshal<P0>(riid: *const ::windows::core::GUID, punk: P0, dwdestcontext: u32, pvdestcontext: ::core::option::Option<*const ::core::ffi::c_void>, mshlflags: u32) -> ::windows::core::Result<IMarshal>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn CoGetStandardMarshal ( riid : *const :: windows::core::GUID , punk : * mut::core::ffi::c_void , dwdestcontext : u32 , pvdestcontext : *const ::core::ffi::c_void , mshlflags : u32 , ppmarshal : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IMarshal>();
    CoGetStandardMarshal(riid, punk.into_param().abi(), dwdestcontext, ::core::mem::transmute(pvdestcontext.unwrap_or(::std::ptr::null())), mshlflags, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn CoGetStdMarshalEx<P0>(punkouter: P0, smexflags: u32) -> ::windows::core::Result<::windows::core::IUnknown>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn CoGetStdMarshalEx ( punkouter : * mut::core::ffi::c_void , smexflags : u32 , ppunkinner : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
    CoGetStdMarshalEx(punkouter.into_param().abi(), smexflags, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn CoMarshalHresult<P0>(pstm: P0, hresult: ::windows::core::HRESULT) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::IStream>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn CoMarshalHresult ( pstm : * mut::core::ffi::c_void , hresult : :: windows::core::HRESULT ) -> :: windows::core::HRESULT );
    CoMarshalHresult(pstm.into_param().abi(), hresult).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn CoMarshalInterThreadInterfaceInStream<P0>(riid: *const ::windows::core::GUID, punk: P0) -> ::windows::core::Result<super::IStream>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn CoMarshalInterThreadInterfaceInStream ( riid : *const :: windows::core::GUID , punk : * mut::core::ffi::c_void , ppstm : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::IStream>();
    CoMarshalInterThreadInterfaceInStream(riid, punk.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn CoMarshalInterface<P0, P1>(pstm: P0, riid: *const ::windows::core::GUID, punk: P1, dwdestcontext: u32, pvdestcontext: ::core::option::Option<*const ::core::ffi::c_void>, mshlflags: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::IStream>,
    P1: ::windows::core::IntoParam<::windows::core::IUnknown>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn CoMarshalInterface ( pstm : * mut::core::ffi::c_void , riid : *const :: windows::core::GUID , punk : * mut::core::ffi::c_void , dwdestcontext : u32 , pvdestcontext : *const ::core::ffi::c_void , mshlflags : u32 ) -> :: windows::core::HRESULT );
    CoMarshalInterface(pstm.into_param().abi(), riid, punk.into_param().abi(), dwdestcontext, ::core::mem::transmute(pvdestcontext.unwrap_or(::std::ptr::null())), mshlflags).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn CoReleaseMarshalData<P0>(pstm: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::IStream>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn CoReleaseMarshalData ( pstm : * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    CoReleaseMarshalData(pstm.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn CoUnmarshalHresult<P0>(pstm: P0) -> ::windows::core::Result<::windows::core::HRESULT>
where
    P0: ::windows::core::IntoParam<super::IStream>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn CoUnmarshalHresult ( pstm : * mut::core::ffi::c_void , phresult : *mut :: windows::core::HRESULT ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::HRESULT>();
    CoUnmarshalHresult(pstm.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn CoUnmarshalInterface<P0, T>(pstm: P0) -> ::windows::core::Result<T>
where
    P0: ::windows::core::IntoParam<super::IStream>,
    T: ::windows::core::ComInterface,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn CoUnmarshalInterface ( pstm : * mut::core::ffi::c_void , riid : *const :: windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::std::ptr::null_mut();
    CoUnmarshalInterface(pstm.into_param().abi(), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserFree(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HACCEL) {
    ::windows::imp::link ! ( "ole32.dll""system" fn HACCEL_UserFree ( param0 : *const u32 , param1 : *const super::super::super::UI::WindowsAndMessaging:: HACCEL ) -> ( ) );
    HACCEL_UserFree(param0, param1)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserFree64(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HACCEL) {
    ::windows::imp::link ! ( "ole32.dll""system" fn HACCEL_UserFree64 ( param0 : *const u32 , param1 : *const super::super::super::UI::WindowsAndMessaging:: HACCEL ) -> ( ) );
    HACCEL_UserFree64(param0, param1)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HACCEL) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HACCEL_UserMarshal ( param0 : *const u32 , param1 : *mut u8 , param2 : *const super::super::super::UI::WindowsAndMessaging:: HACCEL ) -> *mut u8 );
    HACCEL_UserMarshal(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HACCEL) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HACCEL_UserMarshal64 ( param0 : *const u32 , param1 : *mut u8 , param2 : *const super::super::super::UI::WindowsAndMessaging:: HACCEL ) -> *mut u8 );
    HACCEL_UserMarshal64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HACCEL) -> u32 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HACCEL_UserSize ( param0 : *const u32 , param1 : u32 , param2 : *const super::super::super::UI::WindowsAndMessaging:: HACCEL ) -> u32 );
    HACCEL_UserSize(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HACCEL) -> u32 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HACCEL_UserSize64 ( param0 : *const u32 , param1 : u32 , param2 : *const super::super::super::UI::WindowsAndMessaging:: HACCEL ) -> u32 );
    HACCEL_UserSize64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HACCEL) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HACCEL_UserUnmarshal ( param0 : *const u32 , param1 : *const u8 , param2 : *mut super::super::super::UI::WindowsAndMessaging:: HACCEL ) -> *mut u8 );
    HACCEL_UserUnmarshal(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HACCEL) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HACCEL_UserUnmarshal64 ( param0 : *const u32 , param1 : *const u8 , param2 : *mut super::super::super::UI::WindowsAndMessaging:: HACCEL ) -> *mut u8 );
    HACCEL_UserUnmarshal64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserFree(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HBITMAP) {
    ::windows::imp::link ! ( "ole32.dll""system" fn HBITMAP_UserFree ( param0 : *const u32 , param1 : *const super::super::super::Graphics::Gdi:: HBITMAP ) -> ( ) );
    HBITMAP_UserFree(param0, param1)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserFree64(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HBITMAP) {
    ::windows::imp::link ! ( "ole32.dll""system" fn HBITMAP_UserFree64 ( param0 : *const u32 , param1 : *const super::super::super::Graphics::Gdi:: HBITMAP ) -> ( ) );
    HBITMAP_UserFree64(param0, param1)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HBITMAP) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HBITMAP_UserMarshal ( param0 : *const u32 , param1 : *mut u8 , param2 : *const super::super::super::Graphics::Gdi:: HBITMAP ) -> *mut u8 );
    HBITMAP_UserMarshal(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HBITMAP) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HBITMAP_UserMarshal64 ( param0 : *const u32 , param1 : *mut u8 , param2 : *const super::super::super::Graphics::Gdi:: HBITMAP ) -> *mut u8 );
    HBITMAP_UserMarshal64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HBITMAP) -> u32 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HBITMAP_UserSize ( param0 : *const u32 , param1 : u32 , param2 : *const super::super::super::Graphics::Gdi:: HBITMAP ) -> u32 );
    HBITMAP_UserSize(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HBITMAP) -> u32 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HBITMAP_UserSize64 ( param0 : *const u32 , param1 : u32 , param2 : *const super::super::super::Graphics::Gdi:: HBITMAP ) -> u32 );
    HBITMAP_UserSize64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HBITMAP) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HBITMAP_UserUnmarshal ( param0 : *const u32 , param1 : *const u8 , param2 : *mut super::super::super::Graphics::Gdi:: HBITMAP ) -> *mut u8 );
    HBITMAP_UserUnmarshal(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HBITMAP) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HBITMAP_UserUnmarshal64 ( param0 : *const u32 , param1 : *const u8 , param2 : *mut super::super::super::Graphics::Gdi:: HBITMAP ) -> *mut u8 );
    HBITMAP_UserUnmarshal64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserFree(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HDC) {
    ::windows::imp::link ! ( "ole32.dll""system" fn HDC_UserFree ( param0 : *const u32 , param1 : *const super::super::super::Graphics::Gdi:: HDC ) -> ( ) );
    HDC_UserFree(param0, param1)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserFree64(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HDC) {
    ::windows::imp::link ! ( "ole32.dll""system" fn HDC_UserFree64 ( param0 : *const u32 , param1 : *const super::super::super::Graphics::Gdi:: HDC ) -> ( ) );
    HDC_UserFree64(param0, param1)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HDC) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HDC_UserMarshal ( param0 : *const u32 , param1 : *mut u8 , param2 : *const super::super::super::Graphics::Gdi:: HDC ) -> *mut u8 );
    HDC_UserMarshal(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HDC) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HDC_UserMarshal64 ( param0 : *const u32 , param1 : *mut u8 , param2 : *const super::super::super::Graphics::Gdi:: HDC ) -> *mut u8 );
    HDC_UserMarshal64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HDC) -> u32 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HDC_UserSize ( param0 : *const u32 , param1 : u32 , param2 : *const super::super::super::Graphics::Gdi:: HDC ) -> u32 );
    HDC_UserSize(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HDC) -> u32 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HDC_UserSize64 ( param0 : *const u32 , param1 : u32 , param2 : *const super::super::super::Graphics::Gdi:: HDC ) -> u32 );
    HDC_UserSize64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HDC) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HDC_UserUnmarshal ( param0 : *const u32 , param1 : *const u8 , param2 : *mut super::super::super::Graphics::Gdi:: HDC ) -> *mut u8 );
    HDC_UserUnmarshal(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HDC) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HDC_UserUnmarshal64 ( param0 : *const u32 , param1 : *const u8 , param2 : *mut super::super::super::Graphics::Gdi:: HDC ) -> *mut u8 );
    HDC_UserUnmarshal64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HGLOBAL_UserFree(param0: *const u32, param1: *const super::super::super::Foundation::HGLOBAL) {
    ::windows::imp::link ! ( "ole32.dll""system" fn HGLOBAL_UserFree ( param0 : *const u32 , param1 : *const super::super::super::Foundation:: HGLOBAL ) -> ( ) );
    HGLOBAL_UserFree(param0, param1)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HGLOBAL_UserFree64(param0: *const u32, param1: *const super::super::super::Foundation::HGLOBAL) {
    ::windows::imp::link ! ( "ole32.dll""system" fn HGLOBAL_UserFree64 ( param0 : *const u32 , param1 : *const super::super::super::Foundation:: HGLOBAL ) -> ( ) );
    HGLOBAL_UserFree64(param0, param1)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HGLOBAL_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Foundation::HGLOBAL) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HGLOBAL_UserMarshal ( param0 : *const u32 , param1 : *mut u8 , param2 : *const super::super::super::Foundation:: HGLOBAL ) -> *mut u8 );
    HGLOBAL_UserMarshal(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HGLOBAL_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Foundation::HGLOBAL) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HGLOBAL_UserMarshal64 ( param0 : *const u32 , param1 : *mut u8 , param2 : *const super::super::super::Foundation:: HGLOBAL ) -> *mut u8 );
    HGLOBAL_UserMarshal64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HGLOBAL_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Foundation::HGLOBAL) -> u32 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HGLOBAL_UserSize ( param0 : *const u32 , param1 : u32 , param2 : *const super::super::super::Foundation:: HGLOBAL ) -> u32 );
    HGLOBAL_UserSize(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HGLOBAL_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Foundation::HGLOBAL) -> u32 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HGLOBAL_UserSize64 ( param0 : *const u32 , param1 : u32 , param2 : *const super::super::super::Foundation:: HGLOBAL ) -> u32 );
    HGLOBAL_UserSize64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HGLOBAL_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Foundation::HGLOBAL) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HGLOBAL_UserUnmarshal ( param0 : *const u32 , param1 : *const u8 , param2 : *mut super::super::super::Foundation:: HGLOBAL ) -> *mut u8 );
    HGLOBAL_UserUnmarshal(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HGLOBAL_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Foundation::HGLOBAL) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HGLOBAL_UserUnmarshal64 ( param0 : *const u32 , param1 : *const u8 , param2 : *mut super::super::super::Foundation:: HGLOBAL ) -> *mut u8 );
    HGLOBAL_UserUnmarshal64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserFree(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HICON) {
    ::windows::imp::link ! ( "ole32.dll""system" fn HICON_UserFree ( param0 : *const u32 , param1 : *const super::super::super::UI::WindowsAndMessaging:: HICON ) -> ( ) );
    HICON_UserFree(param0, param1)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserFree64(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HICON) {
    ::windows::imp::link ! ( "ole32.dll""system" fn HICON_UserFree64 ( param0 : *const u32 , param1 : *const super::super::super::UI::WindowsAndMessaging:: HICON ) -> ( ) );
    HICON_UserFree64(param0, param1)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HICON) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HICON_UserMarshal ( param0 : *const u32 , param1 : *mut u8 , param2 : *const super::super::super::UI::WindowsAndMessaging:: HICON ) -> *mut u8 );
    HICON_UserMarshal(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HICON) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HICON_UserMarshal64 ( param0 : *const u32 , param1 : *mut u8 , param2 : *const super::super::super::UI::WindowsAndMessaging:: HICON ) -> *mut u8 );
    HICON_UserMarshal64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HICON) -> u32 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HICON_UserSize ( param0 : *const u32 , param1 : u32 , param2 : *const super::super::super::UI::WindowsAndMessaging:: HICON ) -> u32 );
    HICON_UserSize(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HICON) -> u32 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HICON_UserSize64 ( param0 : *const u32 , param1 : u32 , param2 : *const super::super::super::UI::WindowsAndMessaging:: HICON ) -> u32 );
    HICON_UserSize64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HICON) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HICON_UserUnmarshal ( param0 : *const u32 , param1 : *const u8 , param2 : *mut super::super::super::UI::WindowsAndMessaging:: HICON ) -> *mut u8 );
    HICON_UserUnmarshal(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HICON) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HICON_UserUnmarshal64 ( param0 : *const u32 , param1 : *const u8 , param2 : *mut super::super::super::UI::WindowsAndMessaging:: HICON ) -> *mut u8 );
    HICON_UserUnmarshal64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserFree(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HMENU) {
    ::windows::imp::link ! ( "ole32.dll""system" fn HMENU_UserFree ( param0 : *const u32 , param1 : *const super::super::super::UI::WindowsAndMessaging:: HMENU ) -> ( ) );
    HMENU_UserFree(param0, param1)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserFree64(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HMENU) {
    ::windows::imp::link ! ( "ole32.dll""system" fn HMENU_UserFree64 ( param0 : *const u32 , param1 : *const super::super::super::UI::WindowsAndMessaging:: HMENU ) -> ( ) );
    HMENU_UserFree64(param0, param1)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HMENU) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HMENU_UserMarshal ( param0 : *const u32 , param1 : *mut u8 , param2 : *const super::super::super::UI::WindowsAndMessaging:: HMENU ) -> *mut u8 );
    HMENU_UserMarshal(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HMENU) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HMENU_UserMarshal64 ( param0 : *const u32 , param1 : *mut u8 , param2 : *const super::super::super::UI::WindowsAndMessaging:: HMENU ) -> *mut u8 );
    HMENU_UserMarshal64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HMENU) -> u32 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HMENU_UserSize ( param0 : *const u32 , param1 : u32 , param2 : *const super::super::super::UI::WindowsAndMessaging:: HMENU ) -> u32 );
    HMENU_UserSize(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HMENU) -> u32 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HMENU_UserSize64 ( param0 : *const u32 , param1 : u32 , param2 : *const super::super::super::UI::WindowsAndMessaging:: HMENU ) -> u32 );
    HMENU_UserSize64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HMENU) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HMENU_UserUnmarshal ( param0 : *const u32 , param1 : *const u8 , param2 : *mut super::super::super::UI::WindowsAndMessaging:: HMENU ) -> *mut u8 );
    HMENU_UserUnmarshal(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HMENU) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HMENU_UserUnmarshal64 ( param0 : *const u32 , param1 : *const u8 , param2 : *mut super::super::super::UI::WindowsAndMessaging:: HMENU ) -> *mut u8 );
    HMENU_UserUnmarshal64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserFree(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HPALETTE) {
    ::windows::imp::link ! ( "ole32.dll""system" fn HPALETTE_UserFree ( param0 : *const u32 , param1 : *const super::super::super::Graphics::Gdi:: HPALETTE ) -> ( ) );
    HPALETTE_UserFree(param0, param1)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserFree64(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HPALETTE) {
    ::windows::imp::link ! ( "ole32.dll""system" fn HPALETTE_UserFree64 ( param0 : *const u32 , param1 : *const super::super::super::Graphics::Gdi:: HPALETTE ) -> ( ) );
    HPALETTE_UserFree64(param0, param1)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HPALETTE) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HPALETTE_UserMarshal ( param0 : *const u32 , param1 : *mut u8 , param2 : *const super::super::super::Graphics::Gdi:: HPALETTE ) -> *mut u8 );
    HPALETTE_UserMarshal(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HPALETTE) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HPALETTE_UserMarshal64 ( param0 : *const u32 , param1 : *mut u8 , param2 : *const super::super::super::Graphics::Gdi:: HPALETTE ) -> *mut u8 );
    HPALETTE_UserMarshal64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HPALETTE) -> u32 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HPALETTE_UserSize ( param0 : *const u32 , param1 : u32 , param2 : *const super::super::super::Graphics::Gdi:: HPALETTE ) -> u32 );
    HPALETTE_UserSize(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HPALETTE) -> u32 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HPALETTE_UserSize64 ( param0 : *const u32 , param1 : u32 , param2 : *const super::super::super::Graphics::Gdi:: HPALETTE ) -> u32 );
    HPALETTE_UserSize64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HPALETTE) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HPALETTE_UserUnmarshal ( param0 : *const u32 , param1 : *const u8 , param2 : *mut super::super::super::Graphics::Gdi:: HPALETTE ) -> *mut u8 );
    HPALETTE_UserUnmarshal(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HPALETTE) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HPALETTE_UserUnmarshal64 ( param0 : *const u32 , param1 : *const u8 , param2 : *mut super::super::super::Graphics::Gdi:: HPALETTE ) -> *mut u8 );
    HPALETTE_UserUnmarshal64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HWND_UserFree(param0: *const u32, param1: *const super::super::super::Foundation::HWND) {
    ::windows::imp::link ! ( "ole32.dll""system" fn HWND_UserFree ( param0 : *const u32 , param1 : *const super::super::super::Foundation:: HWND ) -> ( ) );
    HWND_UserFree(param0, param1)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HWND_UserFree64(param0: *const u32, param1: *const super::super::super::Foundation::HWND) {
    ::windows::imp::link ! ( "ole32.dll""system" fn HWND_UserFree64 ( param0 : *const u32 , param1 : *const super::super::super::Foundation:: HWND ) -> ( ) );
    HWND_UserFree64(param0, param1)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HWND_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Foundation::HWND) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HWND_UserMarshal ( param0 : *const u32 , param1 : *mut u8 , param2 : *const super::super::super::Foundation:: HWND ) -> *mut u8 );
    HWND_UserMarshal(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HWND_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Foundation::HWND) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HWND_UserMarshal64 ( param0 : *const u32 , param1 : *mut u8 , param2 : *const super::super::super::Foundation:: HWND ) -> *mut u8 );
    HWND_UserMarshal64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HWND_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Foundation::HWND) -> u32 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HWND_UserSize ( param0 : *const u32 , param1 : u32 , param2 : *const super::super::super::Foundation:: HWND ) -> u32 );
    HWND_UserSize(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HWND_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Foundation::HWND) -> u32 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HWND_UserSize64 ( param0 : *const u32 , param1 : u32 , param2 : *const super::super::super::Foundation:: HWND ) -> u32 );
    HWND_UserSize64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HWND_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Foundation::HWND) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HWND_UserUnmarshal ( param0 : *const u32 , param1 : *const u8 , param2 : *mut super::super::super::Foundation:: HWND ) -> *mut u8 );
    HWND_UserUnmarshal(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HWND_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Foundation::HWND) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn HWND_UserUnmarshal64 ( param0 : *const u32 , param1 : *const u8 , param2 : *mut super::super::super::Foundation:: HWND ) -> *mut u8 );
    HWND_UserUnmarshal64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn LPSAFEARRAY_UserFree(param0: *const u32, param1: *const *const super::SAFEARRAY) {
    ::windows::imp::link ! ( "oleaut32.dll""system" fn LPSAFEARRAY_UserFree ( param0 : *const u32 , param1 : *const *const super:: SAFEARRAY ) -> ( ) );
    LPSAFEARRAY_UserFree(param0, param1)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn LPSAFEARRAY_UserFree64(param0: *const u32, param1: *const *const super::SAFEARRAY) {
    ::windows::imp::link ! ( "oleaut32.dll""system" fn LPSAFEARRAY_UserFree64 ( param0 : *const u32 , param1 : *const *const super:: SAFEARRAY ) -> ( ) );
    LPSAFEARRAY_UserFree64(param0, param1)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn LPSAFEARRAY_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const *const super::SAFEARRAY) -> *mut u8 {
    ::windows::imp::link ! ( "oleaut32.dll""system" fn LPSAFEARRAY_UserMarshal ( param0 : *const u32 , param1 : *mut u8 , param2 : *const *const super:: SAFEARRAY ) -> *mut u8 );
    LPSAFEARRAY_UserMarshal(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn LPSAFEARRAY_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const *const super::SAFEARRAY) -> *mut u8 {
    ::windows::imp::link ! ( "oleaut32.dll""system" fn LPSAFEARRAY_UserMarshal64 ( param0 : *const u32 , param1 : *mut u8 , param2 : *const *const super:: SAFEARRAY ) -> *mut u8 );
    LPSAFEARRAY_UserMarshal64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn LPSAFEARRAY_UserSize(param0: *const u32, param1: u32, param2: *const *const super::SAFEARRAY) -> u32 {
    ::windows::imp::link ! ( "oleaut32.dll""system" fn LPSAFEARRAY_UserSize ( param0 : *const u32 , param1 : u32 , param2 : *const *const super:: SAFEARRAY ) -> u32 );
    LPSAFEARRAY_UserSize(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn LPSAFEARRAY_UserSize64(param0: *const u32, param1: u32, param2: *const *const super::SAFEARRAY) -> u32 {
    ::windows::imp::link ! ( "oleaut32.dll""system" fn LPSAFEARRAY_UserSize64 ( param0 : *const u32 , param1 : u32 , param2 : *const *const super:: SAFEARRAY ) -> u32 );
    LPSAFEARRAY_UserSize64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn LPSAFEARRAY_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut *mut super::SAFEARRAY) -> *mut u8 {
    ::windows::imp::link ! ( "oleaut32.dll""system" fn LPSAFEARRAY_UserUnmarshal ( param0 : *const u32 , param1 : *const u8 , param2 : *mut *mut super:: SAFEARRAY ) -> *mut u8 );
    LPSAFEARRAY_UserUnmarshal(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn LPSAFEARRAY_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut *mut super::SAFEARRAY) -> *mut u8 {
    ::windows::imp::link ! ( "oleaut32.dll""system" fn LPSAFEARRAY_UserUnmarshal64 ( param0 : *const u32 , param1 : *const u8 , param2 : *mut *mut super:: SAFEARRAY ) -> *mut u8 );
    LPSAFEARRAY_UserUnmarshal64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn SNB_UserFree(param0: *const u32, param1: *const *const *const u16) {
    ::windows::imp::link ! ( "ole32.dll""system" fn SNB_UserFree ( param0 : *const u32 , param1 : *const *const *const u16 ) -> ( ) );
    SNB_UserFree(param0, param1)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn SNB_UserFree64(param0: *const u32, param1: *const *const *const u16) {
    ::windows::imp::link ! ( "ole32.dll""system" fn SNB_UserFree64 ( param0 : *const u32 , param1 : *const *const *const u16 ) -> ( ) );
    SNB_UserFree64(param0, param1)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn SNB_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const *const *const u16) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn SNB_UserMarshal ( param0 : *const u32 , param1 : *mut u8 , param2 : *const *const *const u16 ) -> *mut u8 );
    SNB_UserMarshal(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn SNB_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const *const *const u16) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn SNB_UserMarshal64 ( param0 : *const u32 , param1 : *mut u8 , param2 : *const *const *const u16 ) -> *mut u8 );
    SNB_UserMarshal64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn SNB_UserSize(param0: *const u32, param1: u32, param2: *const *const *const u16) -> u32 {
    ::windows::imp::link ! ( "ole32.dll""system" fn SNB_UserSize ( param0 : *const u32 , param1 : u32 , param2 : *const *const *const u16 ) -> u32 );
    SNB_UserSize(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn SNB_UserSize64(param0: *const u32, param1: u32, param2: *const *const *const u16) -> u32 {
    ::windows::imp::link ! ( "ole32.dll""system" fn SNB_UserSize64 ( param0 : *const u32 , param1 : u32 , param2 : *const *const *const u16 ) -> u32 );
    SNB_UserSize64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn SNB_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut *mut *mut u16) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn SNB_UserUnmarshal ( param0 : *const u32 , param1 : *const u8 , param2 : *mut *mut *mut u16 ) -> *mut u8 );
    SNB_UserUnmarshal(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn SNB_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut *mut *mut u16) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn SNB_UserUnmarshal64 ( param0 : *const u32 , param1 : *const u8 , param2 : *mut *mut *mut u16 ) -> *mut u8 );
    SNB_UserUnmarshal64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserFree(param0: *const u32, param1: *const super::STGMEDIUM) {
    ::windows::imp::link ! ( "ole32.dll""system" fn STGMEDIUM_UserFree ( param0 : *const u32 , param1 : *const super:: STGMEDIUM ) -> ( ) );
    STGMEDIUM_UserFree(param0, param1)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserFree64(param0: *const u32, param1: *const super::STGMEDIUM) {
    ::windows::imp::link ! ( "ole32.dll""system" fn STGMEDIUM_UserFree64 ( param0 : *const u32 , param1 : *const super:: STGMEDIUM ) -> ( ) );
    STGMEDIUM_UserFree64(param0, param1)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::STGMEDIUM) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn STGMEDIUM_UserMarshal ( param0 : *const u32 , param1 : *mut u8 , param2 : *const super:: STGMEDIUM ) -> *mut u8 );
    STGMEDIUM_UserMarshal(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::STGMEDIUM) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn STGMEDIUM_UserMarshal64 ( param0 : *const u32 , param1 : *mut u8 , param2 : *const super:: STGMEDIUM ) -> *mut u8 );
    STGMEDIUM_UserMarshal64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserSize(param0: *const u32, param1: u32, param2: *const super::STGMEDIUM) -> u32 {
    ::windows::imp::link ! ( "ole32.dll""system" fn STGMEDIUM_UserSize ( param0 : *const u32 , param1 : u32 , param2 : *const super:: STGMEDIUM ) -> u32 );
    STGMEDIUM_UserSize(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserSize64(param0: *const u32, param1: u32, param2: *const super::STGMEDIUM) -> u32 {
    ::windows::imp::link ! ( "ole32.dll""system" fn STGMEDIUM_UserSize64 ( param0 : *const u32 , param1 : u32 , param2 : *const super:: STGMEDIUM ) -> u32 );
    STGMEDIUM_UserSize64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::STGMEDIUM) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn STGMEDIUM_UserUnmarshal ( param0 : *const u32 , param1 : *const u8 , param2 : *mut super:: STGMEDIUM ) -> *mut u8 );
    STGMEDIUM_UserUnmarshal(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::STGMEDIUM) -> *mut u8 {
    ::windows::imp::link ! ( "ole32.dll""system" fn STGMEDIUM_UserUnmarshal64 ( param0 : *const u32 , param1 : *const u8 , param2 : *mut super:: STGMEDIUM ) -> *mut u8 );
    STGMEDIUM_UserUnmarshal64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserFree(param0: *const u32, param1: *const super::VARIANT) {
    ::windows::imp::link ! ( "oleaut32.dll""system" fn VARIANT_UserFree ( param0 : *const u32 , param1 : *const super:: VARIANT ) -> ( ) );
    VARIANT_UserFree(param0, param1)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserFree64(param0: *const u32, param1: *const super::VARIANT) {
    ::windows::imp::link ! ( "oleaut32.dll""system" fn VARIANT_UserFree64 ( param0 : *const u32 , param1 : *const super:: VARIANT ) -> ( ) );
    VARIANT_UserFree64(param0, param1)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::VARIANT) -> *mut u8 {
    ::windows::imp::link ! ( "oleaut32.dll""system" fn VARIANT_UserMarshal ( param0 : *const u32 , param1 : *mut u8 , param2 : *const super:: VARIANT ) -> *mut u8 );
    VARIANT_UserMarshal(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::VARIANT) -> *mut u8 {
    ::windows::imp::link ! ( "oleaut32.dll""system" fn VARIANT_UserMarshal64 ( param0 : *const u32 , param1 : *mut u8 , param2 : *const super:: VARIANT ) -> *mut u8 );
    VARIANT_UserMarshal64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserSize(param0: *const u32, param1: u32, param2: *const super::VARIANT) -> u32 {
    ::windows::imp::link ! ( "oleaut32.dll""system" fn VARIANT_UserSize ( param0 : *const u32 , param1 : u32 , param2 : *const super:: VARIANT ) -> u32 );
    VARIANT_UserSize(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserSize64(param0: *const u32, param1: u32, param2: *const super::VARIANT) -> u32 {
    ::windows::imp::link ! ( "oleaut32.dll""system" fn VARIANT_UserSize64 ( param0 : *const u32 , param1 : u32 , param2 : *const super:: VARIANT ) -> u32 );
    VARIANT_UserSize64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::VARIANT) -> *mut u8 {
    ::windows::imp::link ! ( "oleaut32.dll""system" fn VARIANT_UserUnmarshal ( param0 : *const u32 , param1 : *const u8 , param2 : *mut super:: VARIANT ) -> *mut u8 );
    VARIANT_UserUnmarshal(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::VARIANT) -> *mut u8 {
    ::windows::imp::link ! ( "oleaut32.dll""system" fn VARIANT_UserUnmarshal64 ( param0 : *const u32 , param1 : *const u8 , param2 : *mut super:: VARIANT ) -> *mut u8 );
    VARIANT_UserUnmarshal64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[repr(transparent)]
pub struct IMarshal(::windows::core::IUnknown);
impl IMarshal {
    pub unsafe fn GetUnmarshalClass(&self, riid: *const ::windows::core::GUID, pv: ::core::option::Option<*const ::core::ffi::c_void>, dwdestcontext: u32, pvdestcontext: ::core::option::Option<*const ::core::ffi::c_void>, mshlflags: u32) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetUnmarshalClass)(::windows::core::Interface::as_raw(self), riid, ::core::mem::transmute(pv.unwrap_or(::std::ptr::null())), dwdestcontext, ::core::mem::transmute(pvdestcontext.unwrap_or(::std::ptr::null())), mshlflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMarshalSizeMax(&self, riid: *const ::windows::core::GUID, pv: ::core::option::Option<*const ::core::ffi::c_void>, dwdestcontext: u32, pvdestcontext: ::core::option::Option<*const ::core::ffi::c_void>, mshlflags: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetMarshalSizeMax)(::windows::core::Interface::as_raw(self), riid, ::core::mem::transmute(pv.unwrap_or(::std::ptr::null())), dwdestcontext, ::core::mem::transmute(pvdestcontext.unwrap_or(::std::ptr::null())), mshlflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn MarshalInterface<P0>(&self, pstm: P0, riid: *const ::windows::core::GUID, pv: ::core::option::Option<*const ::core::ffi::c_void>, dwdestcontext: u32, pvdestcontext: ::core::option::Option<*const ::core::ffi::c_void>, mshlflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::IStream>,
    {
        (::windows::core::Interface::vtable(self).MarshalInterface)(::windows::core::Interface::as_raw(self), pstm.into_param().abi(), riid, ::core::mem::transmute(pv.unwrap_or(::std::ptr::null())), dwdestcontext, ::core::mem::transmute(pvdestcontext.unwrap_or(::std::ptr::null())), mshlflags).ok()
    }
    pub unsafe fn UnmarshalInterface<P0>(&self, pstm: P0, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::IStream>,
    {
        (::windows::core::Interface::vtable(self).UnmarshalInterface)(::windows::core::Interface::as_raw(self), pstm.into_param().abi(), riid, ppv).ok()
    }
    pub unsafe fn ReleaseMarshalData<P0>(&self, pstm: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::IStream>,
    {
        (::windows::core::Interface::vtable(self).ReleaseMarshalData)(::windows::core::Interface::as_raw(self), pstm.into_param().abi()).ok()
    }
    pub unsafe fn DisconnectObject(&self, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DisconnectObject)(::windows::core::Interface::as_raw(self), dwreserved).ok()
    }
}
::windows::imp::interface_hierarchy!(IMarshal, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMarshal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMarshal {}
impl ::core::fmt::Debug for IMarshal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMarshal").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMarshal {
    type Vtable = IMarshal_Vtbl;
}
impl ::core::clone::Clone for IMarshal {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMarshal {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000003_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarshal_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetUnmarshalClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, mshlflags: u32, pcid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetMarshalSizeMax: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, mshlflags: u32, psize: *mut u32) -> ::windows::core::HRESULT,
    pub MarshalInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstm: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, mshlflags: u32) -> ::windows::core::HRESULT,
    pub UnmarshalInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstm: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReleaseMarshalData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstm: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisconnectObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[repr(transparent)]
pub struct IMarshal2(::windows::core::IUnknown);
impl IMarshal2 {
    pub unsafe fn GetUnmarshalClass(&self, riid: *const ::windows::core::GUID, pv: ::core::option::Option<*const ::core::ffi::c_void>, dwdestcontext: u32, pvdestcontext: ::core::option::Option<*const ::core::ffi::c_void>, mshlflags: u32) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetUnmarshalClass)(::windows::core::Interface::as_raw(self), riid, ::core::mem::transmute(pv.unwrap_or(::std::ptr::null())), dwdestcontext, ::core::mem::transmute(pvdestcontext.unwrap_or(::std::ptr::null())), mshlflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMarshalSizeMax(&self, riid: *const ::windows::core::GUID, pv: ::core::option::Option<*const ::core::ffi::c_void>, dwdestcontext: u32, pvdestcontext: ::core::option::Option<*const ::core::ffi::c_void>, mshlflags: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetMarshalSizeMax)(::windows::core::Interface::as_raw(self), riid, ::core::mem::transmute(pv.unwrap_or(::std::ptr::null())), dwdestcontext, ::core::mem::transmute(pvdestcontext.unwrap_or(::std::ptr::null())), mshlflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn MarshalInterface<P0>(&self, pstm: P0, riid: *const ::windows::core::GUID, pv: ::core::option::Option<*const ::core::ffi::c_void>, dwdestcontext: u32, pvdestcontext: ::core::option::Option<*const ::core::ffi::c_void>, mshlflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::IStream>,
    {
        (::windows::core::Interface::vtable(self).base__.MarshalInterface)(::windows::core::Interface::as_raw(self), pstm.into_param().abi(), riid, ::core::mem::transmute(pv.unwrap_or(::std::ptr::null())), dwdestcontext, ::core::mem::transmute(pvdestcontext.unwrap_or(::std::ptr::null())), mshlflags).ok()
    }
    pub unsafe fn UnmarshalInterface<P0>(&self, pstm: P0, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::IStream>,
    {
        (::windows::core::Interface::vtable(self).base__.UnmarshalInterface)(::windows::core::Interface::as_raw(self), pstm.into_param().abi(), riid, ppv).ok()
    }
    pub unsafe fn ReleaseMarshalData<P0>(&self, pstm: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::IStream>,
    {
        (::windows::core::Interface::vtable(self).base__.ReleaseMarshalData)(::windows::core::Interface::as_raw(self), pstm.into_param().abi()).ok()
    }
    pub unsafe fn DisconnectObject(&self, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DisconnectObject)(::windows::core::Interface::as_raw(self), dwreserved).ok()
    }
}
::windows::imp::interface_hierarchy!(IMarshal2, ::windows::core::IUnknown, IMarshal);
impl ::core::cmp::PartialEq for IMarshal2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMarshal2 {}
impl ::core::fmt::Debug for IMarshal2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMarshal2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMarshal2 {
    type Vtable = IMarshal2_Vtbl;
}
impl ::core::clone::Clone for IMarshal2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMarshal2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x000001cf_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarshal2_Vtbl {
    pub base__: IMarshal_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[repr(transparent)]
pub struct IMarshalingStream(::windows::core::IUnknown);
impl IMarshalingStream {
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: ::core::option::Option<*mut u32>) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).base__.base__.Read)(::windows::core::Interface::as_raw(self), pv, cb, ::core::mem::transmute(pcbread.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: ::core::option::Option<*mut u32>) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).base__.base__.Write)(::windows::core::Interface::as_raw(self), pv, cb, ::core::mem::transmute(pcbwritten.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: super::STREAM_SEEK, plibnewposition: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Seek)(::windows::core::Interface::as_raw(self), dlibmove, dworigin, ::core::mem::transmute(plibnewposition.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetSize)(::windows::core::Interface::as_raw(self), libnewsize).ok()
    }
    pub unsafe fn CopyTo<P0>(&self, pstm: P0, cb: u64, pcbread: ::core::option::Option<*mut u64>, pcbwritten: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::IStream>,
    {
        (::windows::core::Interface::vtable(self).base__.CopyTo)(::windows::core::Interface::as_raw(self), pstm.into_param().abi(), cb, ::core::mem::transmute(pcbread.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcbwritten.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Commit(&self, grfcommitflags: super::STGC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Commit)(::windows::core::Interface::as_raw(self), grfcommitflags).ok()
    }
    pub unsafe fn Revert(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Revert)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: super::LOCKTYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.LockRegion)(::windows::core::Interface::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.UnlockRegion)(::windows::core::Interface::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Stat(&self, pstatstg: *mut super::STATSTG, grfstatflag: super::STATFLAG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Stat)(::windows::core::Interface::as_raw(self), pstatstg, grfstatflag).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<super::IStream> {
        let mut result__ = ::windows::core::zeroed::<super::IStream>();
        (::windows::core::Interface::vtable(self).base__.Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMarshalingContextAttribute(&self, attribute: super::CO_MARSHALING_CONTEXT_ATTRIBUTES) -> ::windows::core::Result<usize> {
        let mut result__ = ::windows::core::zeroed::<usize>();
        (::windows::core::Interface::vtable(self).GetMarshalingContextAttribute)(::windows::core::Interface::as_raw(self), attribute, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMarshalingStream, ::windows::core::IUnknown, super::ISequentialStream, super::IStream);
impl ::core::cmp::PartialEq for IMarshalingStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMarshalingStream {}
impl ::core::fmt::Debug for IMarshalingStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMarshalingStream").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMarshalingStream {
    type Vtable = IMarshalingStream_Vtbl;
}
impl ::core::clone::Clone for IMarshalingStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMarshalingStream {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8f2f5e6_6102_4863_9f26_389a4676efde);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarshalingStream_Vtbl {
    pub base__: super::IStream_Vtbl,
    pub GetMarshalingContextAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attribute: super::CO_MARSHALING_CONTEXT_ATTRIBUTES, pattributevalue: *mut usize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct STDMSHLFLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
pub const SMEXF_SERVER: STDMSHLFLAGS = STDMSHLFLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
pub const SMEXF_HANDLER: STDMSHLFLAGS = STDMSHLFLAGS(2i32);
impl ::core::marker::Copy for STDMSHLFLAGS {}
impl ::core::clone::Clone for STDMSHLFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STDMSHLFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for STDMSHLFLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for STDMSHLFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STDMSHLFLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
