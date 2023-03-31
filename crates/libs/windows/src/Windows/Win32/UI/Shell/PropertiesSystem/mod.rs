#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn ClearPropVariantArray(rgpropvar: &mut [super::super::super::System::Com::StructuredStorage::PROPVARIANT]) {
    ::windows_targets::link ! ( "propsys.dll""system" fn ClearPropVariantArray ( rgpropvar : *mut super::super::super::System::Com::StructuredStorage:: PROPVARIANT , cvars : u32 ) -> ( ) );
    ClearPropVariantArray(::core::mem::transmute(rgpropvar.as_ptr()), rgpropvar.len() as _)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn ClearVariantArray(pvars: &mut [super::super::super::System::Com::VARIANT]) {
    ::windows_targets::link ! ( "propsys.dll""system" fn ClearVariantArray ( pvars : *mut super::super::super::System::Com:: VARIANT , cvars : u32 ) -> ( ) );
    ClearVariantArray(::core::mem::transmute(pvars.as_ptr()), pvars.len() as _)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromBooleanVector(prgf: ::core::option::Option<&[super::super::super::Foundation::BOOL]>) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    ::windows_targets::link ! ( "propsys.dll""system" fn InitPropVariantFromBooleanVector ( prgf : *const super::super::super::Foundation:: BOOL , celems : u32 , ppropvar : *mut super::super::super::System::Com::StructuredStorage:: PROPVARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>();
    InitPropVariantFromBooleanVector(::core::mem::transmute(prgf.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), prgf.as_deref().map_or(0, |slice| slice.len() as _), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromBuffer(pv: *const ::core::ffi::c_void, cb: u32) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    ::windows_targets::link ! ( "propsys.dll""system" fn InitPropVariantFromBuffer ( pv : *const ::core::ffi::c_void , cb : u32 , ppropvar : *mut super::super::super::System::Com::StructuredStorage:: PROPVARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>();
    InitPropVariantFromBuffer(pv, cb, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromCLSID(clsid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    ::windows_targets::link ! ( "propsys.dll""system" fn InitPropVariantFromCLSID ( clsid : *const ::windows::core::GUID , ppropvar : *mut super::super::super::System::Com::StructuredStorage:: PROPVARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>();
    InitPropVariantFromCLSID(clsid, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromDoubleVector(prgn: ::core::option::Option<&[f64]>) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    ::windows_targets::link ! ( "propsys.dll""system" fn InitPropVariantFromDoubleVector ( prgn : *const f64 , celems : u32 , ppropvar : *mut super::super::super::System::Com::StructuredStorage:: PROPVARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>();
    InitPropVariantFromDoubleVector(::core::mem::transmute(prgn.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), prgn.as_deref().map_or(0, |slice| slice.len() as _), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromFileTime(pftin: *const super::super::super::Foundation::FILETIME) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    ::windows_targets::link ! ( "propsys.dll""system" fn InitPropVariantFromFileTime ( pftin : *const super::super::super::Foundation:: FILETIME , ppropvar : *mut super::super::super::System::Com::StructuredStorage:: PROPVARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>();
    InitPropVariantFromFileTime(pftin, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromFileTimeVector(prgft: ::core::option::Option<&[super::super::super::Foundation::FILETIME]>) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    ::windows_targets::link ! ( "propsys.dll""system" fn InitPropVariantFromFileTimeVector ( prgft : *const super::super::super::Foundation:: FILETIME , celems : u32 , ppropvar : *mut super::super::super::System::Com::StructuredStorage:: PROPVARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>();
    InitPropVariantFromFileTimeVector(::core::mem::transmute(prgft.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), prgft.as_deref().map_or(0, |slice| slice.len() as _), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromGUIDAsString(guid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    ::windows_targets::link ! ( "propsys.dll""system" fn InitPropVariantFromGUIDAsString ( guid : *const ::windows::core::GUID , ppropvar : *mut super::super::super::System::Com::StructuredStorage:: PROPVARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>();
    InitPropVariantFromGUIDAsString(guid, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromInt16Vector(prgn: ::core::option::Option<&[i16]>) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    ::windows_targets::link ! ( "propsys.dll""system" fn InitPropVariantFromInt16Vector ( prgn : *const i16 , celems : u32 , ppropvar : *mut super::super::super::System::Com::StructuredStorage:: PROPVARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>();
    InitPropVariantFromInt16Vector(::core::mem::transmute(prgn.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), prgn.as_deref().map_or(0, |slice| slice.len() as _), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromInt32Vector(prgn: ::core::option::Option<&[i32]>) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    ::windows_targets::link ! ( "propsys.dll""system" fn InitPropVariantFromInt32Vector ( prgn : *const i32 , celems : u32 , ppropvar : *mut super::super::super::System::Com::StructuredStorage:: PROPVARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>();
    InitPropVariantFromInt32Vector(::core::mem::transmute(prgn.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), prgn.as_deref().map_or(0, |slice| slice.len() as _), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromInt64Vector(prgn: ::core::option::Option<&[i64]>) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    ::windows_targets::link ! ( "propsys.dll""system" fn InitPropVariantFromInt64Vector ( prgn : *const i64 , celems : u32 , ppropvar : *mut super::super::super::System::Com::StructuredStorage:: PROPVARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>();
    InitPropVariantFromInt64Vector(::core::mem::transmute(prgn.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), prgn.as_deref().map_or(0, |slice| slice.len() as _), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromPropVariantVectorElem(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    ::windows_targets::link ! ( "propsys.dll""system" fn InitPropVariantFromPropVariantVectorElem ( propvarin : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , ielem : u32 , ppropvar : *mut super::super::super::System::Com::StructuredStorage:: PROPVARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>();
    InitPropVariantFromPropVariantVectorElem(propvarin, ielem, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromResource<P0>(hinst: P0, id: u32) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT>
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HMODULE>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn InitPropVariantFromResource ( hinst : super::super::super::Foundation:: HMODULE , id : u32 , ppropvar : *mut super::super::super::System::Com::StructuredStorage:: PROPVARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>();
    InitPropVariantFromResource(hinst.into_param().abi(), id, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_Common"))]
#[inline]
pub unsafe fn InitPropVariantFromStrRet(pstrret: *mut super::Common::STRRET, pidl: ::core::option::Option<*const super::Common::ITEMIDLIST>, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn InitPropVariantFromStrRet ( pstrret : *mut super::Common:: STRRET , pidl : *const super::Common:: ITEMIDLIST , ppropvar : *mut super::super::super::System::Com::StructuredStorage:: PROPVARIANT ) -> ::windows::core::HRESULT );
    InitPropVariantFromStrRet(pstrret, ::core::mem::transmute(pidl.unwrap_or(::std::ptr::null())), ppropvar).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromStringAsVector<P0>(psz: P0) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn InitPropVariantFromStringAsVector ( psz : ::windows::core::PCWSTR , ppropvar : *mut super::super::super::System::Com::StructuredStorage:: PROPVARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>();
    InitPropVariantFromStringAsVector(psz.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromStringVector(prgsz: ::core::option::Option<&[::windows::core::PCWSTR]>) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    ::windows_targets::link ! ( "propsys.dll""system" fn InitPropVariantFromStringVector ( prgsz : *const ::windows::core::PCWSTR , celems : u32 , ppropvar : *mut super::super::super::System::Com::StructuredStorage:: PROPVARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>();
    InitPropVariantFromStringVector(::core::mem::transmute(prgsz.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), prgsz.as_deref().map_or(0, |slice| slice.len() as _), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromUInt16Vector(prgn: ::core::option::Option<&[u16]>) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    ::windows_targets::link ! ( "propsys.dll""system" fn InitPropVariantFromUInt16Vector ( prgn : *const u16 , celems : u32 , ppropvar : *mut super::super::super::System::Com::StructuredStorage:: PROPVARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>();
    InitPropVariantFromUInt16Vector(::core::mem::transmute(prgn.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), prgn.as_deref().map_or(0, |slice| slice.len() as _), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromUInt32Vector(prgn: ::core::option::Option<&[u32]>) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    ::windows_targets::link ! ( "propsys.dll""system" fn InitPropVariantFromUInt32Vector ( prgn : *const u32 , celems : u32 , ppropvar : *mut super::super::super::System::Com::StructuredStorage:: PROPVARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>();
    InitPropVariantFromUInt32Vector(::core::mem::transmute(prgn.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), prgn.as_deref().map_or(0, |slice| slice.len() as _), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantFromUInt64Vector(prgn: ::core::option::Option<&[u64]>) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    ::windows_targets::link ! ( "propsys.dll""system" fn InitPropVariantFromUInt64Vector ( prgn : *const u64 , celems : u32 , ppropvar : *mut super::super::super::System::Com::StructuredStorage:: PROPVARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>();
    InitPropVariantFromUInt64Vector(::core::mem::transmute(prgn.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), prgn.as_deref().map_or(0, |slice| slice.len() as _), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn InitPropVariantVectorFromPropVariant(propvarsingle: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    ::windows_targets::link ! ( "propsys.dll""system" fn InitPropVariantVectorFromPropVariant ( propvarsingle : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , ppropvarvector : *mut super::super::super::System::Com::StructuredStorage:: PROPVARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>();
    InitPropVariantVectorFromPropVariant(propvarsingle, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromBooleanArray(prgf: &[super::super::super::Foundation::BOOL]) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
    ::windows_targets::link ! ( "propsys.dll""system" fn InitVariantFromBooleanArray ( prgf : *const super::super::super::Foundation:: BOOL , celems : u32 , pvar : *mut super::super::super::System::Com:: VARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
    InitVariantFromBooleanArray(::core::mem::transmute(prgf.as_ptr()), prgf.len() as _, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromBuffer(pv: *const ::core::ffi::c_void, cb: u32) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
    ::windows_targets::link ! ( "propsys.dll""system" fn InitVariantFromBuffer ( pv : *const ::core::ffi::c_void , cb : u32 , pvar : *mut super::super::super::System::Com:: VARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
    InitVariantFromBuffer(pv, cb, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromDoubleArray(prgn: &[f64]) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
    ::windows_targets::link ! ( "propsys.dll""system" fn InitVariantFromDoubleArray ( prgn : *const f64 , celems : u32 , pvar : *mut super::super::super::System::Com:: VARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
    InitVariantFromDoubleArray(::core::mem::transmute(prgn.as_ptr()), prgn.len() as _, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromFileTime(pft: *const super::super::super::Foundation::FILETIME) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
    ::windows_targets::link ! ( "propsys.dll""system" fn InitVariantFromFileTime ( pft : *const super::super::super::Foundation:: FILETIME , pvar : *mut super::super::super::System::Com:: VARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
    InitVariantFromFileTime(pft, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromFileTimeArray(prgft: ::core::option::Option<&[super::super::super::Foundation::FILETIME]>) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
    ::windows_targets::link ! ( "propsys.dll""system" fn InitVariantFromFileTimeArray ( prgft : *const super::super::super::Foundation:: FILETIME , celems : u32 , pvar : *mut super::super::super::System::Com:: VARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
    InitVariantFromFileTimeArray(::core::mem::transmute(prgft.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), prgft.as_deref().map_or(0, |slice| slice.len() as _), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromGUIDAsString(guid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
    ::windows_targets::link ! ( "propsys.dll""system" fn InitVariantFromGUIDAsString ( guid : *const ::windows::core::GUID , pvar : *mut super::super::super::System::Com:: VARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
    InitVariantFromGUIDAsString(guid, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromInt16Array(prgn: &[i16]) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
    ::windows_targets::link ! ( "propsys.dll""system" fn InitVariantFromInt16Array ( prgn : *const i16 , celems : u32 , pvar : *mut super::super::super::System::Com:: VARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
    InitVariantFromInt16Array(::core::mem::transmute(prgn.as_ptr()), prgn.len() as _, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromInt32Array(prgn: &[i32]) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
    ::windows_targets::link ! ( "propsys.dll""system" fn InitVariantFromInt32Array ( prgn : *const i32 , celems : u32 , pvar : *mut super::super::super::System::Com:: VARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
    InitVariantFromInt32Array(::core::mem::transmute(prgn.as_ptr()), prgn.len() as _, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromInt64Array(prgn: &[i64]) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
    ::windows_targets::link ! ( "propsys.dll""system" fn InitVariantFromInt64Array ( prgn : *const i64 , celems : u32 , pvar : *mut super::super::super::System::Com:: VARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
    InitVariantFromInt64Array(::core::mem::transmute(prgn.as_ptr()), prgn.len() as _, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromResource<P0>(hinst: P0, id: u32) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HMODULE>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn InitVariantFromResource ( hinst : super::super::super::Foundation:: HMODULE , id : u32 , pvar : *mut super::super::super::System::Com:: VARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
    InitVariantFromResource(hinst.into_param().abi(), id, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_UI_Shell_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
#[inline]
pub unsafe fn InitVariantFromStrRet(pstrret: *const super::Common::STRRET, pidl: *const super::Common::ITEMIDLIST) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
    ::windows_targets::link ! ( "propsys.dll""system" fn InitVariantFromStrRet ( pstrret : *const super::Common:: STRRET , pidl : *const super::Common:: ITEMIDLIST , pvar : *mut super::super::super::System::Com:: VARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
    InitVariantFromStrRet(pstrret, pidl, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromStringArray(prgsz: &[::windows::core::PCWSTR]) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
    ::windows_targets::link ! ( "propsys.dll""system" fn InitVariantFromStringArray ( prgsz : *const ::windows::core::PCWSTR , celems : u32 , pvar : *mut super::super::super::System::Com:: VARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
    InitVariantFromStringArray(::core::mem::transmute(prgsz.as_ptr()), prgsz.len() as _, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromUInt16Array(prgn: &[u16]) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
    ::windows_targets::link ! ( "propsys.dll""system" fn InitVariantFromUInt16Array ( prgn : *const u16 , celems : u32 , pvar : *mut super::super::super::System::Com:: VARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
    InitVariantFromUInt16Array(::core::mem::transmute(prgn.as_ptr()), prgn.len() as _, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromUInt32Array(prgn: &[u32]) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
    ::windows_targets::link ! ( "propsys.dll""system" fn InitVariantFromUInt32Array ( prgn : *const u32 , celems : u32 , pvar : *mut super::super::super::System::Com:: VARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
    InitVariantFromUInt32Array(::core::mem::transmute(prgn.as_ptr()), prgn.len() as _, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromUInt64Array(prgn: &[u64]) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
    ::windows_targets::link ! ( "propsys.dll""system" fn InitVariantFromUInt64Array ( prgn : *const u64 , celems : u32 , pvar : *mut super::super::super::System::Com:: VARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
    InitVariantFromUInt64Array(::core::mem::transmute(prgn.as_ptr()), prgn.len() as _, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromVariantArrayElem(varin: *const super::super::super::System::Com::VARIANT, ielem: u32) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
    ::windows_targets::link ! ( "propsys.dll""system" fn InitVariantFromVariantArrayElem ( varin : *const super::super::super::System::Com:: VARIANT , ielem : u32 , pvar : *mut super::super::super::System::Com:: VARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
    InitVariantFromVariantArrayElem(varin, ielem, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSCoerceToCanonicalValue(key: *const PROPERTYKEY, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PSCoerceToCanonicalValue ( key : *const PROPERTYKEY , ppropvar : *mut super::super::super::System::Com::StructuredStorage:: PROPVARIANT ) -> ::windows::core::HRESULT );
    PSCoerceToCanonicalValue(key, ppropvar).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSCreateAdapterFromPropertyStore<P0>(pps: P0, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<IPropertyStore>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSCreateAdapterFromPropertyStore ( pps : * mut::core::ffi::c_void , riid : *const ::windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    PSCreateAdapterFromPropertyStore(pps.into_param().abi(), riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSCreateDelayedMultiplexPropertyStore<P0>(flags: GETPROPERTYSTOREFLAGS, pdpsf: P0, rgstoreids: &[u32], riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<IDelayedPropertyStoreFactory>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSCreateDelayedMultiplexPropertyStore ( flags : GETPROPERTYSTOREFLAGS , pdpsf : * mut::core::ffi::c_void , rgstoreids : *const u32 , cstores : u32 , riid : *const ::windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    PSCreateDelayedMultiplexPropertyStore(flags, pdpsf.into_param().abi(), ::core::mem::transmute(rgstoreids.as_ptr()), rgstoreids.len() as _, riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSCreateMemoryPropertyStore(riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PSCreateMemoryPropertyStore ( riid : *const ::windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    PSCreateMemoryPropertyStore(riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSCreateMultiplexPropertyStore(prgpunkstores: &[::core::option::Option<::windows::core::IUnknown>], riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PSCreateMultiplexPropertyStore ( prgpunkstores : *const * mut::core::ffi::c_void , cstores : u32 , riid : *const ::windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    PSCreateMultiplexPropertyStore(::core::mem::transmute(prgpunkstores.as_ptr()), prgpunkstores.len() as _, riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSCreatePropertyChangeArray(rgpropkey: ::core::option::Option<*const PROPERTYKEY>, rgflags: ::core::option::Option<*const PKA_FLAGS>, rgpropvar: ::core::option::Option<*const super::super::super::System::Com::StructuredStorage::PROPVARIANT>, cchanges: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PSCreatePropertyChangeArray ( rgpropkey : *const PROPERTYKEY , rgflags : *const PKA_FLAGS , rgpropvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , cchanges : u32 , riid : *const ::windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    PSCreatePropertyChangeArray(::core::mem::transmute(rgpropkey.unwrap_or(::std::ptr::null())), ::core::mem::transmute(rgflags.unwrap_or(::std::ptr::null())), ::core::mem::transmute(rgpropvar.unwrap_or(::std::ptr::null())), cchanges, riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSCreatePropertyStoreFromObject<P0>(punk: P0, grfmode: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSCreatePropertyStoreFromObject ( punk : * mut::core::ffi::c_void , grfmode : u32 , riid : *const ::windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    PSCreatePropertyStoreFromObject(punk.into_param().abi(), grfmode, riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSCreatePropertyStoreFromPropertySetStorage<P0>(ppss: P0, grfmode: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertySetStorage>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSCreatePropertyStoreFromPropertySetStorage ( ppss : * mut::core::ffi::c_void , grfmode : u32 , riid : *const ::windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    PSCreatePropertyStoreFromPropertySetStorage(ppss.into_param().abi(), grfmode, riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSCreateSimplePropertyChange(flags: PKA_FLAGS, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PSCreateSimplePropertyChange ( flags : PKA_FLAGS , key : *const PROPERTYKEY , propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , riid : *const ::windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    PSCreateSimplePropertyChange(flags, key, propvar, riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSEnumeratePropertyDescriptions(filteron: PROPDESC_ENUMFILTER, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PSEnumeratePropertyDescriptions ( filteron : PROPDESC_ENUMFILTER , riid : *const ::windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    PSEnumeratePropertyDescriptions(filteron, riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSFormatForDisplay(propkey: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS, pwsztext: &mut [u16]) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PSFormatForDisplay ( propkey : *const PROPERTYKEY , propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , pdfflags : PROPDESC_FORMAT_FLAGS , pwsztext : ::windows::core::PWSTR , cchtext : u32 ) -> ::windows::core::HRESULT );
    PSFormatForDisplay(propkey, propvar, pdfflags, ::core::mem::transmute(pwsztext.as_ptr()), pwsztext.len() as _).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSFormatForDisplayAlloc(key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS) -> ::windows::core::Result<::windows::core::PWSTR> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PSFormatForDisplayAlloc ( key : *const PROPERTYKEY , propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , pdff : PROPDESC_FORMAT_FLAGS , ppszdisplay : *mut ::windows::core::PWSTR ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
    PSFormatForDisplayAlloc(key, propvar, pdff, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSFormatPropertyValue<P0, P1>(pps: P0, ppd: P1, pdff: PROPDESC_FORMAT_FLAGS) -> ::windows::core::Result<::windows::core::PWSTR>
where
    P0: ::windows::core::IntoParam<IPropertyStore>,
    P1: ::windows::core::IntoParam<IPropertyDescription>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSFormatPropertyValue ( pps : * mut::core::ffi::c_void , ppd : * mut::core::ffi::c_void , pdff : PROPDESC_FORMAT_FLAGS , ppszdisplay : *mut ::windows::core::PWSTR ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
    PSFormatPropertyValue(pps.into_param().abi(), ppd.into_param().abi(), pdff, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSGetImageReferenceForValue(propkey: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<::windows::core::PWSTR> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PSGetImageReferenceForValue ( propkey : *const PROPERTYKEY , propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , ppszimageres : *mut ::windows::core::PWSTR ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
    PSGetImageReferenceForValue(propkey, propvar, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PSGetItemPropertyHandler<P0, P1>(punkitem: P0, freadwrite: P1, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    P1: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSGetItemPropertyHandler ( punkitem : * mut::core::ffi::c_void , freadwrite : super::super::super::Foundation:: BOOL , riid : *const ::windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    PSGetItemPropertyHandler(punkitem.into_param().abi(), freadwrite.into_param().abi(), riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PSGetItemPropertyHandlerWithCreateObject<P0, P1, P2>(punkitem: P0, freadwrite: P1, punkcreateobject: P2, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    P1: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    P2: ::windows::core::IntoParam<::windows::core::IUnknown>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSGetItemPropertyHandlerWithCreateObject ( punkitem : * mut::core::ffi::c_void , freadwrite : super::super::super::Foundation:: BOOL , punkcreateobject : * mut::core::ffi::c_void , riid : *const ::windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    PSGetItemPropertyHandlerWithCreateObject(punkitem.into_param().abi(), freadwrite.into_param().abi(), punkcreateobject.into_param().abi(), riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSGetNameFromPropertyKey(propkey: *const PROPERTYKEY) -> ::windows::core::Result<::windows::core::PWSTR> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PSGetNameFromPropertyKey ( propkey : *const PROPERTYKEY , ppszcanonicalname : *mut ::windows::core::PWSTR ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
    PSGetNameFromPropertyKey(propkey, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSGetNamedPropertyFromPropertyStorage<P0>(psps: *const SERIALIZEDPROPSTORAGE, cb: u32, pszname: P0) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSGetNamedPropertyFromPropertyStorage ( psps : *const SERIALIZEDPROPSTORAGE , cb : u32 , pszname : ::windows::core::PCWSTR , ppropvar : *mut super::super::super::System::Com::StructuredStorage:: PROPVARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>();
    PSGetNamedPropertyFromPropertyStorage(psps, cb, pszname.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSGetPropertyDescription(propkey: *const PROPERTYKEY, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PSGetPropertyDescription ( propkey : *const PROPERTYKEY , riid : *const ::windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    PSGetPropertyDescription(propkey, riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSGetPropertyDescriptionByName<P0>(pszcanonicalname: P0, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSGetPropertyDescriptionByName ( pszcanonicalname : ::windows::core::PCWSTR , riid : *const ::windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    PSGetPropertyDescriptionByName(pszcanonicalname.into_param().abi(), riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSGetPropertyDescriptionListFromString<P0>(pszproplist: P0, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSGetPropertyDescriptionListFromString ( pszproplist : ::windows::core::PCWSTR , riid : *const ::windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    PSGetPropertyDescriptionListFromString(pszproplist.into_param().abi(), riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSGetPropertyFromPropertyStorage(psps: *const SERIALIZEDPROPSTORAGE, cb: u32, rpkey: *const PROPERTYKEY) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PSGetPropertyFromPropertyStorage ( psps : *const SERIALIZEDPROPSTORAGE , cb : u32 , rpkey : *const PROPERTYKEY , ppropvar : *mut super::super::super::System::Com::StructuredStorage:: PROPVARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>();
    PSGetPropertyFromPropertyStorage(psps, cb, rpkey, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSGetPropertyKeyFromName<P0>(pszname: P0, ppropkey: *mut PROPERTYKEY) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSGetPropertyKeyFromName ( pszname : ::windows::core::PCWSTR , ppropkey : *mut PROPERTYKEY ) -> ::windows::core::HRESULT );
    PSGetPropertyKeyFromName(pszname.into_param().abi(), ppropkey).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSGetPropertySystem(riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PSGetPropertySystem ( riid : *const ::windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    PSGetPropertySystem(riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSGetPropertyValue<P0, P1>(pps: P0, ppd: P1) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT>
where
    P0: ::windows::core::IntoParam<IPropertyStore>,
    P1: ::windows::core::IntoParam<IPropertyDescription>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSGetPropertyValue ( pps : * mut::core::ffi::c_void , ppd : * mut::core::ffi::c_void , ppropvar : *mut super::super::super::System::Com::StructuredStorage:: PROPVARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>();
    PSGetPropertyValue(pps.into_param().abi(), ppd.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSLookupPropertyHandlerCLSID<P0>(pszfilepath: P0) -> ::windows::core::Result<::windows::core::GUID>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSLookupPropertyHandlerCLSID ( pszfilepath : ::windows::core::PCWSTR , pclsid : *mut ::windows::core::GUID ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
    PSLookupPropertyHandlerCLSID(pszfilepath.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_Delete<P0, P1>(propbag: P0, propname: P1) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSPropertyBag_Delete ( propbag : * mut::core::ffi::c_void , propname : ::windows::core::PCWSTR ) -> ::windows::core::HRESULT );
    PSPropertyBag_Delete(propbag.into_param().abi(), propname.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadBOOL<P0, P1>(propbag: P0, propname: P1) -> ::windows::core::Result<super::super::super::Foundation::BOOL>
where
    P0: ::windows::core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSPropertyBag_ReadBOOL ( propbag : * mut::core::ffi::c_void , propname : ::windows::core::PCWSTR , value : *mut super::super::super::Foundation:: BOOL ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::BOOL>();
    PSPropertyBag_ReadBOOL(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadBSTR<P0, P1>(propbag: P0, propname: P1) -> ::windows::core::Result<::windows::core::BSTR>
where
    P0: ::windows::core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSPropertyBag_ReadBSTR ( propbag : * mut::core::ffi::c_void , propname : ::windows::core::PCWSTR , value : *mut ::std::mem::MaybeUninit <::windows::core::BSTR > ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
    PSPropertyBag_ReadBSTR(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadDWORD<P0, P1>(propbag: P0, propname: P1) -> ::windows::core::Result<u32>
where
    P0: ::windows::core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSPropertyBag_ReadDWORD ( propbag : * mut::core::ffi::c_void , propname : ::windows::core::PCWSTR , value : *mut u32 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<u32>();
    PSPropertyBag_ReadDWORD(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadGUID<P0, P1>(propbag: P0, propname: P1) -> ::windows::core::Result<::windows::core::GUID>
where
    P0: ::windows::core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSPropertyBag_ReadGUID ( propbag : * mut::core::ffi::c_void , propname : ::windows::core::PCWSTR , value : *mut ::windows::core::GUID ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
    PSPropertyBag_ReadGUID(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadInt<P0, P1>(propbag: P0, propname: P1) -> ::windows::core::Result<i32>
where
    P0: ::windows::core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSPropertyBag_ReadInt ( propbag : * mut::core::ffi::c_void , propname : ::windows::core::PCWSTR , value : *mut i32 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<i32>();
    PSPropertyBag_ReadInt(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadLONG<P0, P1>(propbag: P0, propname: P1) -> ::windows::core::Result<i32>
where
    P0: ::windows::core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSPropertyBag_ReadLONG ( propbag : * mut::core::ffi::c_void , propname : ::windows::core::PCWSTR , value : *mut i32 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<i32>();
    PSPropertyBag_ReadLONG(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadPOINTL<P0, P1>(propbag: P0, propname: P1) -> ::windows::core::Result<super::super::super::Foundation::POINTL>
where
    P0: ::windows::core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSPropertyBag_ReadPOINTL ( propbag : * mut::core::ffi::c_void , propname : ::windows::core::PCWSTR , value : *mut super::super::super::Foundation:: POINTL ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::POINTL>();
    PSPropertyBag_ReadPOINTL(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadPOINTS<P0, P1>(propbag: P0, propname: P1) -> ::windows::core::Result<super::super::super::Foundation::POINTS>
where
    P0: ::windows::core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSPropertyBag_ReadPOINTS ( propbag : * mut::core::ffi::c_void , propname : ::windows::core::PCWSTR , value : *mut super::super::super::Foundation:: POINTS ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::POINTS>();
    PSPropertyBag_ReadPOINTS(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadPropertyKey<P0, P1>(propbag: P0, propname: P1, value: *mut PROPERTYKEY) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSPropertyBag_ReadPropertyKey ( propbag : * mut::core::ffi::c_void , propname : ::windows::core::PCWSTR , value : *mut PROPERTYKEY ) -> ::windows::core::HRESULT );
    PSPropertyBag_ReadPropertyKey(propbag.into_param().abi(), propname.into_param().abi(), value).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadRECTL<P0, P1>(propbag: P0, propname: P1) -> ::windows::core::Result<super::super::super::Foundation::RECTL>
where
    P0: ::windows::core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSPropertyBag_ReadRECTL ( propbag : * mut::core::ffi::c_void , propname : ::windows::core::PCWSTR , value : *mut super::super::super::Foundation:: RECTL ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::RECTL>();
    PSPropertyBag_ReadRECTL(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadSHORT<P0, P1>(propbag: P0, propname: P1) -> ::windows::core::Result<i16>
where
    P0: ::windows::core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSPropertyBag_ReadSHORT ( propbag : * mut::core::ffi::c_void , propname : ::windows::core::PCWSTR , value : *mut i16 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<i16>();
    PSPropertyBag_ReadSHORT(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadStr<P0, P1>(propbag: P0, propname: P1, value: &mut [u16]) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSPropertyBag_ReadStr ( propbag : * mut::core::ffi::c_void , propname : ::windows::core::PCWSTR , value : ::windows::core::PWSTR , charactercount : i32 ) -> ::windows::core::HRESULT );
    PSPropertyBag_ReadStr(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(value.as_ptr()), value.len() as _).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadStrAlloc<P0, P1>(propbag: P0, propname: P1) -> ::windows::core::Result<::windows::core::PWSTR>
where
    P0: ::windows::core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSPropertyBag_ReadStrAlloc ( propbag : * mut::core::ffi::c_void , propname : ::windows::core::PCWSTR , value : *mut ::windows::core::PWSTR ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
    PSPropertyBag_ReadStrAlloc(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadStream<P0, P1>(propbag: P0, propname: P1) -> ::windows::core::Result<super::super::super::System::Com::IStream>
where
    P0: ::windows::core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSPropertyBag_ReadStream ( propbag : * mut::core::ffi::c_void , propname : ::windows::core::PCWSTR , value : *mut * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::IStream>();
    PSPropertyBag_ReadStream(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadType<P0, P1>(propbag: P0, propname: P1, var: *mut super::super::super::System::Com::VARIANT, r#type: super::super::super::System::Com::VARENUM) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSPropertyBag_ReadType ( propbag : * mut::core::ffi::c_void , propname : ::windows::core::PCWSTR , var : *mut super::super::super::System::Com:: VARIANT , r#type : super::super::super::System::Com:: VARENUM ) -> ::windows::core::HRESULT );
    PSPropertyBag_ReadType(propbag.into_param().abi(), propname.into_param().abi(), var, r#type).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadULONGLONG<P0, P1>(propbag: P0, propname: P1) -> ::windows::core::Result<u64>
where
    P0: ::windows::core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSPropertyBag_ReadULONGLONG ( propbag : * mut::core::ffi::c_void , propname : ::windows::core::PCWSTR , value : *mut u64 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<u64>();
    PSPropertyBag_ReadULONGLONG(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadUnknown<P0, P1>(propbag: P0, propname: P1, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSPropertyBag_ReadUnknown ( propbag : * mut::core::ffi::c_void , propname : ::windows::core::PCWSTR , riid : *const ::windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    PSPropertyBag_ReadUnknown(propbag.into_param().abi(), propname.into_param().abi(), riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_WriteBOOL<P0, P1, P2>(propbag: P0, propname: P1, value: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSPropertyBag_WriteBOOL ( propbag : * mut::core::ffi::c_void , propname : ::windows::core::PCWSTR , value : super::super::super::Foundation:: BOOL ) -> ::windows::core::HRESULT );
    PSPropertyBag_WriteBOOL(propbag.into_param().abi(), propname.into_param().abi(), value.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteBSTR<P0, P1, P2>(propbag: P0, propname: P1, value: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::BSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSPropertyBag_WriteBSTR ( propbag : * mut::core::ffi::c_void , propname : ::windows::core::PCWSTR , value : ::std::mem::MaybeUninit <::windows::core::BSTR > ) -> ::windows::core::HRESULT );
    PSPropertyBag_WriteBSTR(propbag.into_param().abi(), propname.into_param().abi(), value.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteDWORD<P0, P1>(propbag: P0, propname: P1, value: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSPropertyBag_WriteDWORD ( propbag : * mut::core::ffi::c_void , propname : ::windows::core::PCWSTR , value : u32 ) -> ::windows::core::HRESULT );
    PSPropertyBag_WriteDWORD(propbag.into_param().abi(), propname.into_param().abi(), value).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteGUID<P0, P1>(propbag: P0, propname: P1, value: *const ::windows::core::GUID) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSPropertyBag_WriteGUID ( propbag : * mut::core::ffi::c_void , propname : ::windows::core::PCWSTR , value : *const ::windows::core::GUID ) -> ::windows::core::HRESULT );
    PSPropertyBag_WriteGUID(propbag.into_param().abi(), propname.into_param().abi(), value).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteInt<P0, P1>(propbag: P0, propname: P1, value: i32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSPropertyBag_WriteInt ( propbag : * mut::core::ffi::c_void , propname : ::windows::core::PCWSTR , value : i32 ) -> ::windows::core::HRESULT );
    PSPropertyBag_WriteInt(propbag.into_param().abi(), propname.into_param().abi(), value).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteLONG<P0, P1>(propbag: P0, propname: P1, value: i32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSPropertyBag_WriteLONG ( propbag : * mut::core::ffi::c_void , propname : ::windows::core::PCWSTR , value : i32 ) -> ::windows::core::HRESULT );
    PSPropertyBag_WriteLONG(propbag.into_param().abi(), propname.into_param().abi(), value).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_WritePOINTL<P0, P1>(propbag: P0, propname: P1, value: *const super::super::super::Foundation::POINTL) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSPropertyBag_WritePOINTL ( propbag : * mut::core::ffi::c_void , propname : ::windows::core::PCWSTR , value : *const super::super::super::Foundation:: POINTL ) -> ::windows::core::HRESULT );
    PSPropertyBag_WritePOINTL(propbag.into_param().abi(), propname.into_param().abi(), value).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_WritePOINTS<P0, P1>(propbag: P0, propname: P1, value: *const super::super::super::Foundation::POINTS) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSPropertyBag_WritePOINTS ( propbag : * mut::core::ffi::c_void , propname : ::windows::core::PCWSTR , value : *const super::super::super::Foundation:: POINTS ) -> ::windows::core::HRESULT );
    PSPropertyBag_WritePOINTS(propbag.into_param().abi(), propname.into_param().abi(), value).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WritePropertyKey<P0, P1>(propbag: P0, propname: P1, value: *const PROPERTYKEY) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSPropertyBag_WritePropertyKey ( propbag : * mut::core::ffi::c_void , propname : ::windows::core::PCWSTR , value : *const PROPERTYKEY ) -> ::windows::core::HRESULT );
    PSPropertyBag_WritePropertyKey(propbag.into_param().abi(), propname.into_param().abi(), value).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_WriteRECTL<P0, P1>(propbag: P0, propname: P1, value: *const super::super::super::Foundation::RECTL) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSPropertyBag_WriteRECTL ( propbag : * mut::core::ffi::c_void , propname : ::windows::core::PCWSTR , value : *const super::super::super::Foundation:: RECTL ) -> ::windows::core::HRESULT );
    PSPropertyBag_WriteRECTL(propbag.into_param().abi(), propname.into_param().abi(), value).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteSHORT<P0, P1>(propbag: P0, propname: P1, value: i16) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSPropertyBag_WriteSHORT ( propbag : * mut::core::ffi::c_void , propname : ::windows::core::PCWSTR , value : i16 ) -> ::windows::core::HRESULT );
    PSPropertyBag_WriteSHORT(propbag.into_param().abi(), propname.into_param().abi(), value).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteStr<P0, P1, P2>(propbag: P0, propname: P1, value: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSPropertyBag_WriteStr ( propbag : * mut::core::ffi::c_void , propname : ::windows::core::PCWSTR , value : ::windows::core::PCWSTR ) -> ::windows::core::HRESULT );
    PSPropertyBag_WriteStr(propbag.into_param().abi(), propname.into_param().abi(), value.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteStream<P0, P1, P2>(propbag: P0, propname: P1, value: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::super::super::System::Com::IStream>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSPropertyBag_WriteStream ( propbag : * mut::core::ffi::c_void , propname : ::windows::core::PCWSTR , value : * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    PSPropertyBag_WriteStream(propbag.into_param().abi(), propname.into_param().abi(), value.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteULONGLONG<P0, P1>(propbag: P0, propname: P1, value: u64) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSPropertyBag_WriteULONGLONG ( propbag : * mut::core::ffi::c_void , propname : ::windows::core::PCWSTR , value : u64 ) -> ::windows::core::HRESULT );
    PSPropertyBag_WriteULONGLONG(propbag.into_param().abi(), propname.into_param().abi(), value).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteUnknown<P0, P1, P2>(propbag: P0, propname: P1, punk: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::IUnknown>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSPropertyBag_WriteUnknown ( propbag : * mut::core::ffi::c_void , propname : ::windows::core::PCWSTR , punk : * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    PSPropertyBag_WriteUnknown(propbag.into_param().abi(), propname.into_param().abi(), punk.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSPropertyKeyFromString<P0>(pszstring: P0, pkey: *mut PROPERTYKEY) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSPropertyKeyFromString ( pszstring : ::windows::core::PCWSTR , pkey : *mut PROPERTYKEY ) -> ::windows::core::HRESULT );
    PSPropertyKeyFromString(pszstring.into_param().abi(), pkey).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSRefreshPropertySchema() -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PSRefreshPropertySchema ( ) -> ::windows::core::HRESULT );
    PSRefreshPropertySchema().ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSRegisterPropertySchema<P0>(pszpath: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSRegisterPropertySchema ( pszpath : ::windows::core::PCWSTR ) -> ::windows::core::HRESULT );
    PSRegisterPropertySchema(pszpath.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSSetPropertyValue<P0, P1>(pps: P0, ppd: P1, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<IPropertyStore>,
    P1: ::windows::core::IntoParam<IPropertyDescription>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSSetPropertyValue ( pps : * mut::core::ffi::c_void , ppd : * mut::core::ffi::c_void , propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT ) -> ::windows::core::HRESULT );
    PSSetPropertyValue(pps.into_param().abi(), ppd.into_param().abi(), propvar).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSStringFromPropertyKey(pkey: *const PROPERTYKEY, psz: &mut [u16]) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PSStringFromPropertyKey ( pkey : *const PROPERTYKEY , psz : ::windows::core::PWSTR , cch : u32 ) -> ::windows::core::HRESULT );
    PSStringFromPropertyKey(pkey, ::core::mem::transmute(psz.as_ptr()), psz.len() as _).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSUnregisterPropertySchema<P0>(pszpath: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PSUnregisterPropertySchema ( pszpath : ::windows::core::PCWSTR ) -> ::windows::core::HRESULT );
    PSUnregisterPropertySchema(pszpath.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PifMgr_CloseProperties<P0>(hprops: P0, flopt: u32) -> super::super::super::Foundation::HANDLE
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "shell32.dll""system" fn PifMgr_CloseProperties ( hprops : super::super::super::Foundation:: HANDLE , flopt : u32 ) -> super::super::super::Foundation:: HANDLE );
    PifMgr_CloseProperties(hprops.into_param().abi(), flopt)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PifMgr_GetProperties<P0, P1>(hprops: P0, pszgroup: P1, lpprops: ::core::option::Option<*mut ::core::ffi::c_void>, cbprops: i32, flopt: u32) -> i32
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "shell32.dll""system" fn PifMgr_GetProperties ( hprops : super::super::super::Foundation:: HANDLE , pszgroup : ::windows::core::PCSTR , lpprops : *mut ::core::ffi::c_void , cbprops : i32 , flopt : u32 ) -> i32 );
    PifMgr_GetProperties(hprops.into_param().abi(), pszgroup.into_param().abi(), ::core::mem::transmute(lpprops.unwrap_or(::std::ptr::null_mut())), cbprops, flopt)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PifMgr_OpenProperties<P0, P1>(pszapp: P0, pszpif: P1, hinf: u32, flopt: u32) -> super::super::super::Foundation::HANDLE
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "shell32.dll""system" fn PifMgr_OpenProperties ( pszapp : ::windows::core::PCWSTR , pszpif : ::windows::core::PCWSTR , hinf : u32 , flopt : u32 ) -> super::super::super::Foundation:: HANDLE );
    PifMgr_OpenProperties(pszapp.into_param().abi(), pszpif.into_param().abi(), hinf, flopt)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PifMgr_SetProperties<P0, P1>(hprops: P0, pszgroup: P1, lpprops: *const ::core::ffi::c_void, cbprops: i32, flopt: u32) -> i32
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "shell32.dll""system" fn PifMgr_SetProperties ( hprops : super::super::super::Foundation:: HANDLE , pszgroup : ::windows::core::PCSTR , lpprops : *const ::core::ffi::c_void , cbprops : i32 , flopt : u32 ) -> i32 );
    PifMgr_SetProperties(hprops.into_param().abi(), pszgroup.into_param().abi(), lpprops, cbprops, flopt)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantChangeType(ppropvardest: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvarsrc: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, flags: PROPVAR_CHANGE_FLAGS, vt: super::super::super::System::Com::VARENUM) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantChangeType ( ppropvardest : *mut super::super::super::System::Com::StructuredStorage:: PROPVARIANT , propvarsrc : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , flags : PROPVAR_CHANGE_FLAGS , vt : super::super::super::System::Com:: VARENUM ) -> ::windows::core::HRESULT );
    PropVariantChangeType(ppropvardest, propvarsrc, flags, vt).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantCompareEx(propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, unit: PROPVAR_COMPARE_UNIT, flags: PROPVAR_COMPARE_FLAGS) -> i32 {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantCompareEx ( propvar1 : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , propvar2 : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , unit : PROPVAR_COMPARE_UNIT , flags : PROPVAR_COMPARE_FLAGS ) -> i32 );
    PropVariantCompareEx(propvar1, propvar2, unit, flags)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantGetBooleanElem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantGetBooleanElem ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , ielem : u32 , pfval : *mut super::super::super::Foundation:: BOOL ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::BOOL>();
    PropVariantGetBooleanElem(propvar, ielem, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantGetDoubleElem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows::core::Result<f64> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantGetDoubleElem ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , ielem : u32 , pnval : *mut f64 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<f64>();
    PropVariantGetDoubleElem(propvar, ielem, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantGetElementCount(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> u32 {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantGetElementCount ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT ) -> u32 );
    PropVariantGetElementCount(propvar)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantGetFileTimeElem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows::core::Result<super::super::super::Foundation::FILETIME> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantGetFileTimeElem ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , ielem : u32 , pftval : *mut super::super::super::Foundation:: FILETIME ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::FILETIME>();
    PropVariantGetFileTimeElem(propvar, ielem, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantGetInt16Elem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows::core::Result<i16> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantGetInt16Elem ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , ielem : u32 , pnval : *mut i16 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<i16>();
    PropVariantGetInt16Elem(propvar, ielem, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantGetInt32Elem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows::core::Result<i32> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantGetInt32Elem ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , ielem : u32 , pnval : *mut i32 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<i32>();
    PropVariantGetInt32Elem(propvar, ielem, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantGetInt64Elem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows::core::Result<i64> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantGetInt64Elem ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , ielem : u32 , pnval : *mut i64 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<i64>();
    PropVariantGetInt64Elem(propvar, ielem, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantGetStringElem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows::core::Result<::windows::core::PWSTR> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantGetStringElem ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , ielem : u32 , ppszval : *mut ::windows::core::PWSTR ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
    PropVariantGetStringElem(propvar, ielem, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantGetUInt16Elem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows::core::Result<u16> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantGetUInt16Elem ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , ielem : u32 , pnval : *mut u16 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<u16>();
    PropVariantGetUInt16Elem(propvar, ielem, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantGetUInt32Elem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows::core::Result<u32> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantGetUInt32Elem ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , ielem : u32 , pnval : *mut u32 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<u32>();
    PropVariantGetUInt32Elem(propvar, ielem, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantGetUInt64Elem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32) -> ::windows::core::Result<u64> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantGetUInt64Elem ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , ielem : u32 , pnval : *mut u64 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<u64>();
    PropVariantGetUInt64Elem(propvar, ielem, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToBSTR(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<::windows::core::BSTR> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToBSTR ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , pbstrout : *mut ::std::mem::MaybeUninit <::windows::core::BSTR > ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
    PropVariantToBSTR(propvar, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToBoolean(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToBoolean ( propvarin : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , pfret : *mut super::super::super::Foundation:: BOOL ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::BOOL>();
    PropVariantToBoolean(propvarin, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToBooleanVector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgf: &mut [super::super::super::Foundation::BOOL], pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToBooleanVector ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , prgf : *mut super::super::super::Foundation:: BOOL , crgf : u32 , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    PropVariantToBooleanVector(propvar, ::core::mem::transmute(prgf.as_ptr()), prgf.len() as _, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToBooleanVectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgf: *mut *mut super::super::super::Foundation::BOOL, pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToBooleanVectorAlloc ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , pprgf : *mut *mut super::super::super::Foundation:: BOOL , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    PropVariantToBooleanVectorAlloc(propvar, pprgf, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToBooleanWithDefault<P0>(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, fdefault: P0) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToBooleanWithDefault ( propvarin : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , fdefault : super::super::super::Foundation:: BOOL ) -> super::super::super::Foundation:: BOOL );
    PropVariantToBooleanWithDefault(propvarin, fdefault.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToBuffer(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pv: *mut ::core::ffi::c_void, cb: u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToBuffer ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , pv : *mut ::core::ffi::c_void , cb : u32 ) -> ::windows::core::HRESULT );
    PropVariantToBuffer(propvar, pv, cb).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToDouble(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<f64> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToDouble ( propvarin : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , pdblret : *mut f64 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<f64>();
    PropVariantToDouble(propvarin, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToDoubleVector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: &mut [f64], pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToDoubleVector ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , prgn : *mut f64 , crgn : u32 , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    PropVariantToDoubleVector(propvar, ::core::mem::transmute(prgn.as_ptr()), prgn.len() as _, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToDoubleVectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut f64, pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToDoubleVectorAlloc ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , pprgn : *mut *mut f64 , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    PropVariantToDoubleVectorAlloc(propvar, pprgn, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToDoubleWithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, dbldefault: f64) -> f64 {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToDoubleWithDefault ( propvarin : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , dbldefault : f64 ) -> f64 );
    PropVariantToDoubleWithDefault(propvarin, dbldefault)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToFileTime(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pstfout: PSTIME_FLAGS) -> ::windows::core::Result<super::super::super::Foundation::FILETIME> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToFileTime ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , pstfout : PSTIME_FLAGS , pftout : *mut super::super::super::Foundation:: FILETIME ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::FILETIME>();
    PropVariantToFileTime(propvar, pstfout, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToFileTimeVector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgft: &mut [super::super::super::Foundation::FILETIME], pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToFileTimeVector ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , prgft : *mut super::super::super::Foundation:: FILETIME , crgft : u32 , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    PropVariantToFileTimeVector(propvar, ::core::mem::transmute(prgft.as_ptr()), prgft.len() as _, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToFileTimeVectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgft: *mut *mut super::super::super::Foundation::FILETIME, pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToFileTimeVectorAlloc ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , pprgft : *mut *mut super::super::super::Foundation:: FILETIME , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    PropVariantToFileTimeVectorAlloc(propvar, pprgft, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToGUID(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<::windows::core::GUID> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToGUID ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , pguid : *mut ::windows::core::GUID ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
    PropVariantToGUID(propvar, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToInt16(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<i16> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToInt16 ( propvarin : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , piret : *mut i16 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<i16>();
    PropVariantToInt16(propvarin, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToInt16Vector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: &mut [i16], pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToInt16Vector ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , prgn : *mut i16 , crgn : u32 , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    PropVariantToInt16Vector(propvar, ::core::mem::transmute(prgn.as_ptr()), prgn.len() as _, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToInt16VectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut i16, pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToInt16VectorAlloc ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , pprgn : *mut *mut i16 , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    PropVariantToInt16VectorAlloc(propvar, pprgn, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToInt16WithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, idefault: i16) -> i16 {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToInt16WithDefault ( propvarin : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , idefault : i16 ) -> i16 );
    PropVariantToInt16WithDefault(propvarin, idefault)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToInt32(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<i32> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToInt32 ( propvarin : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , plret : *mut i32 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<i32>();
    PropVariantToInt32(propvarin, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToInt32Vector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: &mut [i32], pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToInt32Vector ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , prgn : *mut i32 , crgn : u32 , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    PropVariantToInt32Vector(propvar, ::core::mem::transmute(prgn.as_ptr()), prgn.len() as _, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToInt32VectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut i32, pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToInt32VectorAlloc ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , pprgn : *mut *mut i32 , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    PropVariantToInt32VectorAlloc(propvar, pprgn, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToInt32WithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ldefault: i32) -> i32 {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToInt32WithDefault ( propvarin : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , ldefault : i32 ) -> i32 );
    PropVariantToInt32WithDefault(propvarin, ldefault)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToInt64(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<i64> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToInt64 ( propvarin : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , pllret : *mut i64 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<i64>();
    PropVariantToInt64(propvarin, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToInt64Vector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: &mut [i64], pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToInt64Vector ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , prgn : *mut i64 , crgn : u32 , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    PropVariantToInt64Vector(propvar, ::core::mem::transmute(prgn.as_ptr()), prgn.len() as _, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToInt64VectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut i64, pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToInt64VectorAlloc ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , pprgn : *mut *mut i64 , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    PropVariantToInt64VectorAlloc(propvar, pprgn, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToInt64WithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, lldefault: i64) -> i64 {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToInt64WithDefault ( propvarin : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , lldefault : i64 ) -> i64 );
    PropVariantToInt64WithDefault(propvarin, lldefault)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_Common"))]
#[inline]
pub unsafe fn PropVariantToStrRet(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pstrret: *mut super::Common::STRRET) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToStrRet ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , pstrret : *mut super::Common:: STRRET ) -> ::windows::core::HRESULT );
    PropVariantToStrRet(propvar, pstrret).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToString(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, psz: &mut [u16]) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToString ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , psz : ::windows::core::PWSTR , cch : u32 ) -> ::windows::core::HRESULT );
    PropVariantToString(propvar, ::core::mem::transmute(psz.as_ptr()), psz.len() as _).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToStringAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<::windows::core::PWSTR> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToStringAlloc ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , ppszout : *mut ::windows::core::PWSTR ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
    PropVariantToStringAlloc(propvar, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToStringVector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgsz: &mut [::windows::core::PWSTR], pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToStringVector ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , prgsz : *mut ::windows::core::PWSTR , crgsz : u32 , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    PropVariantToStringVector(propvar, ::core::mem::transmute(prgsz.as_ptr()), prgsz.len() as _, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToStringVectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgsz: *mut *mut ::windows::core::PWSTR, pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToStringVectorAlloc ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , pprgsz : *mut *mut ::windows::core::PWSTR , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    PropVariantToStringVectorAlloc(propvar, pprgsz, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToStringWithDefault<P0>(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pszdefault: P0) -> ::windows::core::PCWSTR
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToStringWithDefault ( propvarin : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , pszdefault : ::windows::core::PCWSTR ) -> ::windows::core::PCWSTR );
    PropVariantToStringWithDefault(propvarin, pszdefault.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToUInt16(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<u16> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToUInt16 ( propvarin : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , puiret : *mut u16 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<u16>();
    PropVariantToUInt16(propvarin, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToUInt16Vector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: &mut [u16], pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToUInt16Vector ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , prgn : *mut u16 , crgn : u32 , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    PropVariantToUInt16Vector(propvar, ::core::mem::transmute(prgn.as_ptr()), prgn.len() as _, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToUInt16VectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut u16, pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToUInt16VectorAlloc ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , pprgn : *mut *mut u16 , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    PropVariantToUInt16VectorAlloc(propvar, pprgn, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToUInt16WithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, uidefault: u16) -> u16 {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToUInt16WithDefault ( propvarin : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , uidefault : u16 ) -> u16 );
    PropVariantToUInt16WithDefault(propvarin, uidefault)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToUInt32(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<u32> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToUInt32 ( propvarin : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , pulret : *mut u32 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<u32>();
    PropVariantToUInt32(propvarin, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToUInt32Vector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: &mut [u32], pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToUInt32Vector ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , prgn : *mut u32 , crgn : u32 , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    PropVariantToUInt32Vector(propvar, ::core::mem::transmute(prgn.as_ptr()), prgn.len() as _, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToUInt32VectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut u32, pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToUInt32VectorAlloc ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , pprgn : *mut *mut u32 , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    PropVariantToUInt32VectorAlloc(propvar, pprgn, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToUInt32WithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, uldefault: u32) -> u32 {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToUInt32WithDefault ( propvarin : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , uldefault : u32 ) -> u32 );
    PropVariantToUInt32WithDefault(propvarin, uldefault)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToUInt64(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<u64> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToUInt64 ( propvarin : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , pullret : *mut u64 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<u64>();
    PropVariantToUInt64(propvarin, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToUInt64Vector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: &mut [u64], pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToUInt64Vector ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , prgn : *mut u64 , crgn : u32 , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    PropVariantToUInt64Vector(propvar, ::core::mem::transmute(prgn.as_ptr()), prgn.len() as _, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToUInt64VectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut u64, pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToUInt64VectorAlloc ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , pprgn : *mut *mut u64 , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    PropVariantToUInt64VectorAlloc(propvar, pprgn, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToUInt64WithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ulldefault: u64) -> u64 {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToUInt64WithDefault ( propvarin : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , ulldefault : u64 ) -> u64 );
    PropVariantToUInt64WithDefault(propvarin, ulldefault)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn PropVariantToVariant(ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToVariant ( ppropvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , pvar : *mut super::super::super::System::Com:: VARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::VARIANT>();
    PropVariantToVariant(ppropvar, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PropVariantToWinRTPropertyValue<T>(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<T>
where
    T: ::windows::core::ComInterface,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn PropVariantToWinRTPropertyValue ( propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT , riid : *const ::windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    let mut result__ = ::std::ptr::null_mut();
    PropVariantToWinRTPropertyValue(propvar, &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn SHAddDefaultPropertiesByExt<P0, P1>(pszext: P0, ppropstore: P1) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<IPropertyStore>,
{
    ::windows_targets::link ! ( "shell32.dll""system" fn SHAddDefaultPropertiesByExt ( pszext : ::windows::core::PCWSTR , ppropstore : * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    SHAddDefaultPropertiesByExt(pszext.into_param().abi(), ppropstore.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SHGetPropertyStoreForWindow<P0>(hwnd: P0, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "shell32.dll""system" fn SHGetPropertyStoreForWindow ( hwnd : super::super::super::Foundation:: HWND , riid : *const ::windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    SHGetPropertyStoreForWindow(hwnd.into_param().abi(), riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_UI_Shell_Common\"`*"]
#[cfg(feature = "Win32_UI_Shell_Common")]
#[inline]
pub unsafe fn SHGetPropertyStoreFromIDList(pidl: *const super::Common::ITEMIDLIST, flags: GETPROPERTYSTOREFLAGS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "shell32.dll""system" fn SHGetPropertyStoreFromIDList ( pidl : *const super::Common:: ITEMIDLIST , flags : GETPROPERTYSTOREFLAGS , riid : *const ::windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    SHGetPropertyStoreFromIDList(pidl, flags, riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SHGetPropertyStoreFromParsingName<P0, P1, T>(pszpath: P0, pbc: P1, flags: GETPROPERTYSTOREFLAGS) -> ::windows::core::Result<T>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::super::System::Com::IBindCtx>,
    T: ::windows::core::ComInterface,
{
    ::windows_targets::link ! ( "shell32.dll""system" fn SHGetPropertyStoreFromParsingName ( pszpath : ::windows::core::PCWSTR , pbc : * mut::core::ffi::c_void , flags : GETPROPERTYSTOREFLAGS , riid : *const ::windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    let mut result__ = ::std::ptr::null_mut();
    SHGetPropertyStoreFromParsingName(pszpath.into_param().abi(), pbc.into_param().abi(), flags, &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn SHPropStgCreate<P0>(psstg: P0, fmtid: *const ::windows::core::GUID, pclsid: ::core::option::Option<*const ::windows::core::GUID>, grfflags: u32, grfmode: u32, dwdisposition: u32, ppstg: *mut ::core::option::Option<super::super::super::System::Com::StructuredStorage::IPropertyStorage>, pucodepage: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertySetStorage>,
{
    ::windows_targets::link ! ( "shell32.dll""system" fn SHPropStgCreate ( psstg : * mut::core::ffi::c_void , fmtid : *const ::windows::core::GUID , pclsid : *const ::windows::core::GUID , grfflags : u32 , grfmode : u32 , dwdisposition : u32 , ppstg : *mut * mut::core::ffi::c_void , pucodepage : *mut u32 ) -> ::windows::core::HRESULT );
    SHPropStgCreate(psstg.into_param().abi(), fmtid, ::core::mem::transmute(pclsid.unwrap_or(::std::ptr::null())), grfflags, grfmode, dwdisposition, ::core::mem::transmute(ppstg), ::core::mem::transmute(pucodepage.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn SHPropStgReadMultiple<P0>(pps: P0, ucodepage: u32, cpspec: u32, rgpspec: *const super::super::super::System::Com::StructuredStorage::PROPSPEC, rgvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyStorage>,
{
    ::windows_targets::link ! ( "shell32.dll""system" fn SHPropStgReadMultiple ( pps : * mut::core::ffi::c_void , ucodepage : u32 , cpspec : u32 , rgpspec : *const super::super::super::System::Com::StructuredStorage:: PROPSPEC , rgvar : *mut super::super::super::System::Com::StructuredStorage:: PROPVARIANT ) -> ::windows::core::HRESULT );
    SHPropStgReadMultiple(pps.into_param().abi(), ucodepage, cpspec, rgpspec, rgvar).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn SHPropStgWriteMultiple<P0>(pps: P0, pucodepage: ::core::option::Option<*mut u32>, cpspec: u32, rgpspec: *const super::super::super::System::Com::StructuredStorage::PROPSPEC, rgvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT, propidnamefirst: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyStorage>,
{
    ::windows_targets::link ! ( "shell32.dll""system" fn SHPropStgWriteMultiple ( pps : * mut::core::ffi::c_void , pucodepage : *mut u32 , cpspec : u32 , rgpspec : *const super::super::super::System::Com::StructuredStorage:: PROPSPEC , rgvar : *mut super::super::super::System::Com::StructuredStorage:: PROPVARIANT , propidnamefirst : u32 ) -> ::windows::core::HRESULT );
    SHPropStgWriteMultiple(pps.into_param().abi(), ::core::mem::transmute(pucodepage.unwrap_or(::std::ptr::null_mut())), cpspec, rgpspec, rgvar, propidnamefirst).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantCompare(var1: *const super::super::super::System::Com::VARIANT, var2: *const super::super::super::System::Com::VARIANT) -> i32 {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantCompare ( var1 : *const super::super::super::System::Com:: VARIANT , var2 : *const super::super::super::System::Com:: VARIANT ) -> i32 );
    VariantCompare(var1, var2)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetBooleanElem(var: *const super::super::super::System::Com::VARIANT, ielem: u32) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantGetBooleanElem ( var : *const super::super::super::System::Com:: VARIANT , ielem : u32 , pfval : *mut super::super::super::Foundation:: BOOL ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::BOOL>();
    VariantGetBooleanElem(var, ielem, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetDoubleElem(var: *const super::super::super::System::Com::VARIANT, ielem: u32) -> ::windows::core::Result<f64> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantGetDoubleElem ( var : *const super::super::super::System::Com:: VARIANT , ielem : u32 , pnval : *mut f64 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<f64>();
    VariantGetDoubleElem(var, ielem, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetElementCount(varin: *const super::super::super::System::Com::VARIANT) -> u32 {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantGetElementCount ( varin : *const super::super::super::System::Com:: VARIANT ) -> u32 );
    VariantGetElementCount(varin)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetInt16Elem(var: *const super::super::super::System::Com::VARIANT, ielem: u32) -> ::windows::core::Result<i16> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantGetInt16Elem ( var : *const super::super::super::System::Com:: VARIANT , ielem : u32 , pnval : *mut i16 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<i16>();
    VariantGetInt16Elem(var, ielem, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetInt32Elem(var: *const super::super::super::System::Com::VARIANT, ielem: u32) -> ::windows::core::Result<i32> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantGetInt32Elem ( var : *const super::super::super::System::Com:: VARIANT , ielem : u32 , pnval : *mut i32 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<i32>();
    VariantGetInt32Elem(var, ielem, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetInt64Elem(var: *const super::super::super::System::Com::VARIANT, ielem: u32) -> ::windows::core::Result<i64> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantGetInt64Elem ( var : *const super::super::super::System::Com:: VARIANT , ielem : u32 , pnval : *mut i64 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<i64>();
    VariantGetInt64Elem(var, ielem, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetStringElem(var: *const super::super::super::System::Com::VARIANT, ielem: u32) -> ::windows::core::Result<::windows::core::PWSTR> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantGetStringElem ( var : *const super::super::super::System::Com:: VARIANT , ielem : u32 , ppszval : *mut ::windows::core::PWSTR ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
    VariantGetStringElem(var, ielem, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetUInt16Elem(var: *const super::super::super::System::Com::VARIANT, ielem: u32) -> ::windows::core::Result<u16> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantGetUInt16Elem ( var : *const super::super::super::System::Com:: VARIANT , ielem : u32 , pnval : *mut u16 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<u16>();
    VariantGetUInt16Elem(var, ielem, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetUInt32Elem(var: *const super::super::super::System::Com::VARIANT, ielem: u32) -> ::windows::core::Result<u32> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantGetUInt32Elem ( var : *const super::super::super::System::Com:: VARIANT , ielem : u32 , pnval : *mut u32 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<u32>();
    VariantGetUInt32Elem(var, ielem, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetUInt64Elem(var: *const super::super::super::System::Com::VARIANT, ielem: u32) -> ::windows::core::Result<u64> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantGetUInt64Elem ( var : *const super::super::super::System::Com:: VARIANT , ielem : u32 , pnval : *mut u64 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<u64>();
    VariantGetUInt64Elem(var, ielem, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToBoolean(varin: *const super::super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToBoolean ( varin : *const super::super::super::System::Com:: VARIANT , pfret : *mut super::super::super::Foundation:: BOOL ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::BOOL>();
    VariantToBoolean(varin, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToBooleanArray(var: *const super::super::super::System::Com::VARIANT, prgf: &mut [super::super::super::Foundation::BOOL], pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToBooleanArray ( var : *const super::super::super::System::Com:: VARIANT , prgf : *mut super::super::super::Foundation:: BOOL , crgn : u32 , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    VariantToBooleanArray(var, ::core::mem::transmute(prgf.as_ptr()), prgf.len() as _, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToBooleanArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgf: *mut *mut super::super::super::Foundation::BOOL, pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToBooleanArrayAlloc ( var : *const super::super::super::System::Com:: VARIANT , pprgf : *mut *mut super::super::super::Foundation:: BOOL , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    VariantToBooleanArrayAlloc(var, pprgf, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToBooleanWithDefault<P0>(varin: *const super::super::super::System::Com::VARIANT, fdefault: P0) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToBooleanWithDefault ( varin : *const super::super::super::System::Com:: VARIANT , fdefault : super::super::super::Foundation:: BOOL ) -> super::super::super::Foundation:: BOOL );
    VariantToBooleanWithDefault(varin, fdefault.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToBuffer(varin: *const super::super::super::System::Com::VARIANT, pv: *mut ::core::ffi::c_void, cb: u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToBuffer ( varin : *const super::super::super::System::Com:: VARIANT , pv : *mut ::core::ffi::c_void , cb : u32 ) -> ::windows::core::HRESULT );
    VariantToBuffer(varin, pv, cb).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToDosDateTime(varin: *const super::super::super::System::Com::VARIANT, pwdate: *mut u16, pwtime: *mut u16) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToDosDateTime ( varin : *const super::super::super::System::Com:: VARIANT , pwdate : *mut u16 , pwtime : *mut u16 ) -> ::windows::core::HRESULT );
    VariantToDosDateTime(varin, pwdate, pwtime).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToDouble(varin: *const super::super::super::System::Com::VARIANT) -> ::windows::core::Result<f64> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToDouble ( varin : *const super::super::super::System::Com:: VARIANT , pdblret : *mut f64 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<f64>();
    VariantToDouble(varin, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToDoubleArray(var: *const super::super::super::System::Com::VARIANT, prgn: &mut [f64], pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToDoubleArray ( var : *const super::super::super::System::Com:: VARIANT , prgn : *mut f64 , crgn : u32 , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    VariantToDoubleArray(var, ::core::mem::transmute(prgn.as_ptr()), prgn.len() as _, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToDoubleArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut f64, pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToDoubleArrayAlloc ( var : *const super::super::super::System::Com:: VARIANT , pprgn : *mut *mut f64 , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    VariantToDoubleArrayAlloc(var, pprgn, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToDoubleWithDefault(varin: *const super::super::super::System::Com::VARIANT, dbldefault: f64) -> f64 {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToDoubleWithDefault ( varin : *const super::super::super::System::Com:: VARIANT , dbldefault : f64 ) -> f64 );
    VariantToDoubleWithDefault(varin, dbldefault)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToFileTime(varin: *const super::super::super::System::Com::VARIANT, stfout: PSTIME_FLAGS) -> ::windows::core::Result<super::super::super::Foundation::FILETIME> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToFileTime ( varin : *const super::super::super::System::Com:: VARIANT , stfout : PSTIME_FLAGS , pftout : *mut super::super::super::Foundation:: FILETIME ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::FILETIME>();
    VariantToFileTime(varin, stfout, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToGUID(varin: *const super::super::super::System::Com::VARIANT) -> ::windows::core::Result<::windows::core::GUID> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToGUID ( varin : *const super::super::super::System::Com:: VARIANT , pguid : *mut ::windows::core::GUID ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
    VariantToGUID(varin, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt16(varin: *const super::super::super::System::Com::VARIANT) -> ::windows::core::Result<i16> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToInt16 ( varin : *const super::super::super::System::Com:: VARIANT , piret : *mut i16 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<i16>();
    VariantToInt16(varin, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt16Array(var: *const super::super::super::System::Com::VARIANT, prgn: &mut [i16], pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToInt16Array ( var : *const super::super::super::System::Com:: VARIANT , prgn : *mut i16 , crgn : u32 , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    VariantToInt16Array(var, ::core::mem::transmute(prgn.as_ptr()), prgn.len() as _, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt16ArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut i16, pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToInt16ArrayAlloc ( var : *const super::super::super::System::Com:: VARIANT , pprgn : *mut *mut i16 , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    VariantToInt16ArrayAlloc(var, pprgn, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt16WithDefault(varin: *const super::super::super::System::Com::VARIANT, idefault: i16) -> i16 {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToInt16WithDefault ( varin : *const super::super::super::System::Com:: VARIANT , idefault : i16 ) -> i16 );
    VariantToInt16WithDefault(varin, idefault)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt32(varin: *const super::super::super::System::Com::VARIANT) -> ::windows::core::Result<i32> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToInt32 ( varin : *const super::super::super::System::Com:: VARIANT , plret : *mut i32 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<i32>();
    VariantToInt32(varin, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt32Array(var: *const super::super::super::System::Com::VARIANT, prgn: &mut [i32], pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToInt32Array ( var : *const super::super::super::System::Com:: VARIANT , prgn : *mut i32 , crgn : u32 , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    VariantToInt32Array(var, ::core::mem::transmute(prgn.as_ptr()), prgn.len() as _, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt32ArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut i32, pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToInt32ArrayAlloc ( var : *const super::super::super::System::Com:: VARIANT , pprgn : *mut *mut i32 , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    VariantToInt32ArrayAlloc(var, pprgn, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt32WithDefault(varin: *const super::super::super::System::Com::VARIANT, ldefault: i32) -> i32 {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToInt32WithDefault ( varin : *const super::super::super::System::Com:: VARIANT , ldefault : i32 ) -> i32 );
    VariantToInt32WithDefault(varin, ldefault)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt64(varin: *const super::super::super::System::Com::VARIANT) -> ::windows::core::Result<i64> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToInt64 ( varin : *const super::super::super::System::Com:: VARIANT , pllret : *mut i64 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<i64>();
    VariantToInt64(varin, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt64Array(var: *const super::super::super::System::Com::VARIANT, prgn: &mut [i64], pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToInt64Array ( var : *const super::super::super::System::Com:: VARIANT , prgn : *mut i64 , crgn : u32 , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    VariantToInt64Array(var, ::core::mem::transmute(prgn.as_ptr()), prgn.len() as _, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt64ArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut i64, pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToInt64ArrayAlloc ( var : *const super::super::super::System::Com:: VARIANT , pprgn : *mut *mut i64 , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    VariantToInt64ArrayAlloc(var, pprgn, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt64WithDefault(varin: *const super::super::super::System::Com::VARIANT, lldefault: i64) -> i64 {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToInt64WithDefault ( varin : *const super::super::super::System::Com:: VARIANT , lldefault : i64 ) -> i64 );
    VariantToInt64WithDefault(varin, lldefault)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToPropVariant(pvar: *const super::super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToPropVariant ( pvar : *const super::super::super::System::Com:: VARIANT , ppropvar : *mut super::super::super::System::Com::StructuredStorage:: PROPVARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>();
    VariantToPropVariant(pvar, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_UI_Shell_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
#[inline]
pub unsafe fn VariantToStrRet(varin: *const super::super::super::System::Com::VARIANT, pstrret: *mut super::Common::STRRET) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToStrRet ( varin : *const super::super::super::System::Com:: VARIANT , pstrret : *mut super::Common:: STRRET ) -> ::windows::core::HRESULT );
    VariantToStrRet(varin, pstrret).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToString(varin: *const super::super::super::System::Com::VARIANT, pszbuf: &mut [u16]) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToString ( varin : *const super::super::super::System::Com:: VARIANT , pszbuf : ::windows::core::PWSTR , cchbuf : u32 ) -> ::windows::core::HRESULT );
    VariantToString(varin, ::core::mem::transmute(pszbuf.as_ptr()), pszbuf.len() as _).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToStringAlloc(varin: *const super::super::super::System::Com::VARIANT) -> ::windows::core::Result<::windows::core::PWSTR> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToStringAlloc ( varin : *const super::super::super::System::Com:: VARIANT , ppszbuf : *mut ::windows::core::PWSTR ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
    VariantToStringAlloc(varin, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToStringArray(var: *const super::super::super::System::Com::VARIANT, prgsz: &mut [::windows::core::PWSTR], pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToStringArray ( var : *const super::super::super::System::Com:: VARIANT , prgsz : *mut ::windows::core::PWSTR , crgsz : u32 , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    VariantToStringArray(var, ::core::mem::transmute(prgsz.as_ptr()), prgsz.len() as _, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToStringArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgsz: *mut *mut ::windows::core::PWSTR, pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToStringArrayAlloc ( var : *const super::super::super::System::Com:: VARIANT , pprgsz : *mut *mut ::windows::core::PWSTR , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    VariantToStringArrayAlloc(var, pprgsz, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToStringWithDefault<P0>(varin: *const super::super::super::System::Com::VARIANT, pszdefault: P0) -> ::windows::core::PCWSTR
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToStringWithDefault ( varin : *const super::super::super::System::Com:: VARIANT , pszdefault : ::windows::core::PCWSTR ) -> ::windows::core::PCWSTR );
    VariantToStringWithDefault(varin, pszdefault.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt16(varin: *const super::super::super::System::Com::VARIANT) -> ::windows::core::Result<u16> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToUInt16 ( varin : *const super::super::super::System::Com:: VARIANT , puiret : *mut u16 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<u16>();
    VariantToUInt16(varin, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt16Array(var: *const super::super::super::System::Com::VARIANT, prgn: &mut [u16], pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToUInt16Array ( var : *const super::super::super::System::Com:: VARIANT , prgn : *mut u16 , crgn : u32 , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    VariantToUInt16Array(var, ::core::mem::transmute(prgn.as_ptr()), prgn.len() as _, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt16ArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut u16, pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToUInt16ArrayAlloc ( var : *const super::super::super::System::Com:: VARIANT , pprgn : *mut *mut u16 , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    VariantToUInt16ArrayAlloc(var, pprgn, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt16WithDefault(varin: *const super::super::super::System::Com::VARIANT, uidefault: u16) -> u16 {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToUInt16WithDefault ( varin : *const super::super::super::System::Com:: VARIANT , uidefault : u16 ) -> u16 );
    VariantToUInt16WithDefault(varin, uidefault)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt32(varin: *const super::super::super::System::Com::VARIANT) -> ::windows::core::Result<u32> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToUInt32 ( varin : *const super::super::super::System::Com:: VARIANT , pulret : *mut u32 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<u32>();
    VariantToUInt32(varin, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt32Array(var: *const super::super::super::System::Com::VARIANT, prgn: &mut [u32], pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToUInt32Array ( var : *const super::super::super::System::Com:: VARIANT , prgn : *mut u32 , crgn : u32 , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    VariantToUInt32Array(var, ::core::mem::transmute(prgn.as_ptr()), prgn.len() as _, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt32ArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut u32, pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToUInt32ArrayAlloc ( var : *const super::super::super::System::Com:: VARIANT , pprgn : *mut *mut u32 , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    VariantToUInt32ArrayAlloc(var, pprgn, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt32WithDefault(varin: *const super::super::super::System::Com::VARIANT, uldefault: u32) -> u32 {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToUInt32WithDefault ( varin : *const super::super::super::System::Com:: VARIANT , uldefault : u32 ) -> u32 );
    VariantToUInt32WithDefault(varin, uldefault)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt64(varin: *const super::super::super::System::Com::VARIANT) -> ::windows::core::Result<u64> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToUInt64 ( varin : *const super::super::super::System::Com:: VARIANT , pullret : *mut u64 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<u64>();
    VariantToUInt64(varin, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt64Array(var: *const super::super::super::System::Com::VARIANT, prgn: &mut [u64], pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToUInt64Array ( var : *const super::super::super::System::Com:: VARIANT , prgn : *mut u64 , crgn : u32 , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    VariantToUInt64Array(var, ::core::mem::transmute(prgn.as_ptr()), prgn.len() as _, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt64ArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut u64, pcelem: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToUInt64ArrayAlloc ( var : *const super::super::super::System::Com:: VARIANT , pprgn : *mut *mut u64 , pcelem : *mut u32 ) -> ::windows::core::HRESULT );
    VariantToUInt64ArrayAlloc(var, pprgn, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt64WithDefault(varin: *const super::super::super::System::Com::VARIANT, ulldefault: u64) -> u64 {
    ::windows_targets::link ! ( "propsys.dll""system" fn VariantToUInt64WithDefault ( varin : *const super::super::super::System::Com:: VARIANT , ulldefault : u64 ) -> u64 );
    VariantToUInt64WithDefault(varin, ulldefault)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn WinRTPropertyValueToPropVariant<P0>(punkpropertyvalue: P0) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
{
    ::windows_targets::link ! ( "propsys.dll""system" fn WinRTPropertyValueToPropVariant ( punkpropertyvalue : * mut::core::ffi::c_void , ppropvar : *mut super::super::super::System::Com::StructuredStorage:: PROPVARIANT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>();
    WinRTPropertyValueToPropVariant(punkpropertyvalue.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct ICreateObject(::windows::core::IUnknown);
impl ICreateObject {
    pub unsafe fn CreateObject<P0, T>(&self, clsid: *const ::windows::core::GUID, punkouter: P0) -> ::windows::core::Result<T>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).CreateObject)(::windows::core::Interface::as_raw(self), clsid, punkouter.into_param().abi(), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ICreateObject, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICreateObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICreateObject {}
impl ::core::fmt::Debug for ICreateObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICreateObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICreateObject {
    type Vtable = ICreateObject_Vtbl;
}
impl ::core::clone::Clone for ICreateObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICreateObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75121952_e0d0_43e5_9380_1d80483acf72);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateObject_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IDelayedPropertyStoreFactory(::windows::core::IUnknown);
impl IDelayedPropertyStoreFactory {
    pub unsafe fn GetPropertyStore<P0, T>(&self, flags: GETPROPERTYSTOREFLAGS, punkfactory: P0) -> ::windows::core::Result<T>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.GetPropertyStore)(::windows::core::Interface::as_raw(self), flags, punkfactory.into_param().abi(), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPropertyStoreForKeys<T>(&self, rgkeys: *const PROPERTYKEY, ckeys: u32, flags: GETPROPERTYSTOREFLAGS) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.GetPropertyStoreForKeys)(::windows::core::Interface::as_raw(self), rgkeys, ckeys, flags, &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDelayedPropertyStore<T>(&self, flags: GETPROPERTYSTOREFLAGS, dwstoreid: u32) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).GetDelayedPropertyStore)(::windows::core::Interface::as_raw(self), flags, dwstoreid, &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDelayedPropertyStoreFactory, ::windows::core::IUnknown, IPropertyStoreFactory);
impl ::core::cmp::PartialEq for IDelayedPropertyStoreFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDelayedPropertyStoreFactory {}
impl ::core::fmt::Debug for IDelayedPropertyStoreFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDelayedPropertyStoreFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDelayedPropertyStoreFactory {
    type Vtable = IDelayedPropertyStoreFactory_Vtbl;
}
impl ::core::clone::Clone for IDelayedPropertyStoreFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDelayedPropertyStoreFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40d4577f_e237_4bdb_bd69_58f089431b6a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDelayedPropertyStoreFactory_Vtbl {
    pub base__: IPropertyStoreFactory_Vtbl,
    pub GetDelayedPropertyStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: GETPROPERTYSTOREFLAGS, dwstoreid: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IInitializeWithFile(::windows::core::IUnknown);
impl IInitializeWithFile {
    pub unsafe fn Initialize<P0>(&self, pszfilepath: P0, grfmode: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), pszfilepath.into_param().abi(), grfmode).ok()
    }
}
::windows::imp::interface_hierarchy!(IInitializeWithFile, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IInitializeWithFile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInitializeWithFile {}
impl ::core::fmt::Debug for IInitializeWithFile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInitializeWithFile").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInitializeWithFile {
    type Vtable = IInitializeWithFile_Vtbl;
}
impl ::core::clone::Clone for IInitializeWithFile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInitializeWithFile {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7d14566_0509_4cce_a71f_0a554233bd9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInitializeWithFile_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfilepath: ::windows::core::PCWSTR, grfmode: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IInitializeWithStream(::windows::core::IUnknown);
impl IInitializeWithStream {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0>(&self, pstream: P0, grfmode: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::System::Com::IStream>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), pstream.into_param().abi(), grfmode).ok()
    }
}
::windows::imp::interface_hierarchy!(IInitializeWithStream, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IInitializeWithStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInitializeWithStream {}
impl ::core::fmt::Debug for IInitializeWithStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInitializeWithStream").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInitializeWithStream {
    type Vtable = IInitializeWithStream_Vtbl;
}
impl ::core::clone::Clone for IInitializeWithStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInitializeWithStream {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb824b49d_22ac_4161_ac8a_9916e8fa3f7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInitializeWithStream_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, grfmode: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct INamedPropertyStore(::windows::core::IUnknown);
impl INamedPropertyStore {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetNamedValue<P0>(&self, pszname: P0) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>();
        (::windows::core::Interface::vtable(self).GetNamedValue)(::windows::core::Interface::as_raw(self), pszname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetNamedValue<P0>(&self, pszname: P0, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetNamedValue)(::windows::core::Interface::as_raw(self), pszname.into_param().abi(), propvar).ok()
    }
    pub unsafe fn GetNameCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetNameCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetNameAt(&self, iprop: u32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetNameAt)(::windows::core::Interface::as_raw(self), iprop, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(INamedPropertyStore, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for INamedPropertyStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INamedPropertyStore {}
impl ::core::fmt::Debug for INamedPropertyStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INamedPropertyStore").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INamedPropertyStore {
    type Vtable = INamedPropertyStore_Vtbl;
}
impl ::core::clone::Clone for INamedPropertyStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INamedPropertyStore {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71604b0f_97b0_4764_8577_2f13e98a1422);
}
#[repr(C)]
#[doc(hidden)]
pub struct INamedPropertyStore_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetNamedValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetNamedValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub SetNamedValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    SetNamedValue: usize,
    pub GetNameCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetNameAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iprop: u32, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IObjectWithPropertyKey(::windows::core::IUnknown);
impl IObjectWithPropertyKey {
    pub unsafe fn SetPropertyKey(&self, key: *const PROPERTYKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPropertyKey)(::windows::core::Interface::as_raw(self), key).ok()
    }
    pub unsafe fn GetPropertyKey(&self, pkey: *mut PROPERTYKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPropertyKey)(::windows::core::Interface::as_raw(self), pkey).ok()
    }
}
::windows::imp::interface_hierarchy!(IObjectWithPropertyKey, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IObjectWithPropertyKey {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectWithPropertyKey {}
impl ::core::fmt::Debug for IObjectWithPropertyKey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectWithPropertyKey").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IObjectWithPropertyKey {
    type Vtable = IObjectWithPropertyKey_Vtbl;
}
impl ::core::clone::Clone for IObjectWithPropertyKey {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IObjectWithPropertyKey {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc0ca0a7_c316_4fd2_9031_3e628e6d4f23);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectWithPropertyKey_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetPropertyKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY) -> ::windows::core::HRESULT,
    pub GetPropertyKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pkey: *mut PROPERTYKEY) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPersistSerializedPropStorage(::windows::core::IUnknown);
impl IPersistSerializedPropStorage {
    pub unsafe fn SetFlags(&self, flags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFlags)(::windows::core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn SetPropertyStorage(&self, psps: *const SERIALIZEDPROPSTORAGE, cb: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPropertyStorage)(::windows::core::Interface::as_raw(self), psps, cb).ok()
    }
    pub unsafe fn GetPropertyStorage(&self, ppsps: *mut *mut SERIALIZEDPROPSTORAGE, pcb: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPropertyStorage)(::windows::core::Interface::as_raw(self), ppsps, pcb).ok()
    }
}
::windows::imp::interface_hierarchy!(IPersistSerializedPropStorage, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPersistSerializedPropStorage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPersistSerializedPropStorage {}
impl ::core::fmt::Debug for IPersistSerializedPropStorage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistSerializedPropStorage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPersistSerializedPropStorage {
    type Vtable = IPersistSerializedPropStorage_Vtbl;
}
impl ::core::clone::Clone for IPersistSerializedPropStorage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPersistSerializedPropStorage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe318ad57_0aa0_450f_aca5_6fab7103d917);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistSerializedPropStorage_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT,
    pub SetPropertyStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psps: *const SERIALIZEDPROPSTORAGE, cb: u32) -> ::windows::core::HRESULT,
    pub GetPropertyStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsps: *mut *mut SERIALIZEDPROPSTORAGE, pcb: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPersistSerializedPropStorage2(::windows::core::IUnknown);
impl IPersistSerializedPropStorage2 {
    pub unsafe fn SetFlags(&self, flags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetFlags)(::windows::core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn SetPropertyStorage(&self, psps: *const SERIALIZEDPROPSTORAGE, cb: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetPropertyStorage)(::windows::core::Interface::as_raw(self), psps, cb).ok()
    }
    pub unsafe fn GetPropertyStorage(&self, ppsps: *mut *mut SERIALIZEDPROPSTORAGE, pcb: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPropertyStorage)(::windows::core::Interface::as_raw(self), ppsps, pcb).ok()
    }
    pub unsafe fn GetPropertyStorageSize(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetPropertyStorageSize)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPropertyStorageBuffer(&self, psps: *mut SERIALIZEDPROPSTORAGE, cb: u32, pcbwritten: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPropertyStorageBuffer)(::windows::core::Interface::as_raw(self), psps, cb, pcbwritten).ok()
    }
}
::windows::imp::interface_hierarchy!(IPersistSerializedPropStorage2, ::windows::core::IUnknown, IPersistSerializedPropStorage);
impl ::core::cmp::PartialEq for IPersistSerializedPropStorage2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPersistSerializedPropStorage2 {}
impl ::core::fmt::Debug for IPersistSerializedPropStorage2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistSerializedPropStorage2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPersistSerializedPropStorage2 {
    type Vtable = IPersistSerializedPropStorage2_Vtbl;
}
impl ::core::clone::Clone for IPersistSerializedPropStorage2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPersistSerializedPropStorage2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77effa68_4f98_4366_ba72_573b3d880571);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistSerializedPropStorage2_Vtbl {
    pub base__: IPersistSerializedPropStorage_Vtbl,
    pub GetPropertyStorageSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcb: *mut u32) -> ::windows::core::HRESULT,
    pub GetPropertyStorageBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psps: *mut SERIALIZEDPROPSTORAGE, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPropertyChange(::windows::core::IUnknown);
impl IPropertyChange {
    pub unsafe fn SetPropertyKey(&self, key: *const PROPERTYKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetPropertyKey)(::windows::core::Interface::as_raw(self), key).ok()
    }
    pub unsafe fn GetPropertyKey(&self, pkey: *mut PROPERTYKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPropertyKey)(::windows::core::Interface::as_raw(self), pkey).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn ApplyToPropVariant(&self, propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>();
        (::windows::core::Interface::vtable(self).ApplyToPropVariant)(::windows::core::Interface::as_raw(self), propvarin, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IPropertyChange, ::windows::core::IUnknown, IObjectWithPropertyKey);
impl ::core::cmp::PartialEq for IPropertyChange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyChange {}
impl ::core::fmt::Debug for IPropertyChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyChange").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPropertyChange {
    type Vtable = IPropertyChange_Vtbl;
}
impl ::core::clone::Clone for IPropertyChange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPropertyChange {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf917bc8a_1bba_4478_a245_1bde03eb9431);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyChange_Vtbl {
    pub base__: IObjectWithPropertyKey_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub ApplyToPropVariant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvarout: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    ApplyToPropVariant: usize,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPropertyChangeArray(::windows::core::IUnknown);
impl IPropertyChangeArray {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAt<T>(&self, iindex: u32) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).GetAt)(::windows::core::Interface::as_raw(self), iindex, &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn InsertAt<P0>(&self, iindex: u32, ppropchange: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPropertyChange>,
    {
        (::windows::core::Interface::vtable(self).InsertAt)(::windows::core::Interface::as_raw(self), iindex, ppropchange.into_param().abi()).ok()
    }
    pub unsafe fn Append<P0>(&self, ppropchange: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPropertyChange>,
    {
        (::windows::core::Interface::vtable(self).Append)(::windows::core::Interface::as_raw(self), ppropchange.into_param().abi()).ok()
    }
    pub unsafe fn AppendOrReplace<P0>(&self, ppropchange: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPropertyChange>,
    {
        (::windows::core::Interface::vtable(self).AppendOrReplace)(::windows::core::Interface::as_raw(self), ppropchange.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, iindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAt)(::windows::core::Interface::as_raw(self), iindex).ok()
    }
    pub unsafe fn IsKeyInArray(&self, key: *const PROPERTYKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsKeyInArray)(::windows::core::Interface::as_raw(self), key).ok()
    }
}
::windows::imp::interface_hierarchy!(IPropertyChangeArray, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPropertyChangeArray {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyChangeArray {}
impl ::core::fmt::Debug for IPropertyChangeArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyChangeArray").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPropertyChangeArray {
    type Vtable = IPropertyChangeArray_Vtbl;
}
impl ::core::clone::Clone for IPropertyChangeArray {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPropertyChangeArray {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x380f5cad_1b5e_42f2_805d_637fd392d31e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyChangeArray_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcoperations: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iindex: u32, ppropchange: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropchange: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AppendOrReplace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropchange: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iindex: u32) -> ::windows::core::HRESULT,
    pub IsKeyInArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPropertyDescription(::windows::core::IUnknown);
impl IPropertyDescription {
    pub unsafe fn GetPropertyKey(&self, pkey: *mut PROPERTYKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPropertyKey)(::windows::core::Interface::as_raw(self), pkey).ok()
    }
    pub unsafe fn GetCanonicalName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetCanonicalName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPropertyType(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).GetPropertyType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetDisplayName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetEditInvitation(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetEditInvitation)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTypeFlags(&self, mask: PROPDESC_TYPE_FLAGS) -> ::windows::core::Result<PROPDESC_TYPE_FLAGS> {
        let mut result__ = ::windows::core::zeroed::<PROPDESC_TYPE_FLAGS>();
        (::windows::core::Interface::vtable(self).GetTypeFlags)(::windows::core::Interface::as_raw(self), mask, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetViewFlags(&self) -> ::windows::core::Result<PROPDESC_VIEW_FLAGS> {
        let mut result__ = ::windows::core::zeroed::<PROPDESC_VIEW_FLAGS>();
        (::windows::core::Interface::vtable(self).GetViewFlags)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDefaultColumnWidth(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetDefaultColumnWidth)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDisplayType(&self) -> ::windows::core::Result<PROPDESC_DISPLAYTYPE> {
        let mut result__ = ::windows::core::zeroed::<PROPDESC_DISPLAYTYPE>();
        (::windows::core::Interface::vtable(self).GetDisplayType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetColumnState(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetColumnState)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetGroupingRange(&self) -> ::windows::core::Result<PROPDESC_GROUPING_RANGE> {
        let mut result__ = ::windows::core::zeroed::<PROPDESC_GROUPING_RANGE>();
        (::windows::core::Interface::vtable(self).GetGroupingRange)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRelativeDescriptionType(&self) -> ::windows::core::Result<PROPDESC_RELATIVEDESCRIPTION_TYPE> {
        let mut result__ = ::windows::core::zeroed::<PROPDESC_RELATIVEDESCRIPTION_TYPE>();
        (::windows::core::Interface::vtable(self).GetRelativeDescriptionType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetRelativeDescription(&self, propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszdesc1: *mut ::windows::core::PWSTR, ppszdesc2: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRelativeDescription)(::windows::core::Interface::as_raw(self), propvar1, propvar2, ppszdesc1, ppszdesc2).ok()
    }
    pub unsafe fn GetSortDescription(&self) -> ::windows::core::Result<PROPDESC_SORTDESCRIPTION> {
        let mut result__ = ::windows::core::zeroed::<PROPDESC_SORTDESCRIPTION>();
        (::windows::core::Interface::vtable(self).GetSortDescription)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSortDescriptionLabel<P0>(&self, fdescending: P0) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetSortDescriptionLabel)(::windows::core::Interface::as_raw(self), fdescending.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAggregationType(&self) -> ::windows::core::Result<PROPDESC_AGGREGATION_TYPE> {
        let mut result__ = ::windows::core::zeroed::<PROPDESC_AGGREGATION_TYPE>();
        (::windows::core::Interface::vtable(self).GetAggregationType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Search_Common\"`*"]
    #[cfg(feature = "Win32_System_Search_Common")]
    pub unsafe fn GetConditionType(&self, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetConditionType)(::windows::core::Interface::as_raw(self), pcontype, popdefault).ok()
    }
    pub unsafe fn GetEnumTypeList<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).GetEnumTypeList)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn CoerceToCanonicalValue(&self, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CoerceToCanonicalValue)(::windows::core::Interface::as_raw(self), ppropvar).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn FormatForDisplay(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).FormatForDisplay)(::windows::core::Interface::as_raw(self), propvar, pdfflags, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn IsValueCanonical(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsValueCanonical)(::windows::core::Interface::as_raw(self), propvar).ok()
    }
}
::windows::imp::interface_hierarchy!(IPropertyDescription, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPropertyDescription {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyDescription {}
impl ::core::fmt::Debug for IPropertyDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyDescription").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPropertyDescription {
    type Vtable = IPropertyDescription_Vtbl;
}
impl ::core::clone::Clone for IPropertyDescription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPropertyDescription {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f79d558_3e96_4549_a1d1_7d75d2288814);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyDescription_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetPropertyKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pkey: *mut PROPERTYKEY) -> ::windows::core::HRESULT,
    pub GetCanonicalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetPropertyType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvartype: *mut u16) -> ::windows::core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetEditInvitation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszinvite: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetTypeFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mask: PROPDESC_TYPE_FLAGS, ppdtflags: *mut PROPDESC_TYPE_FLAGS) -> ::windows::core::HRESULT,
    pub GetViewFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdvflags: *mut PROPDESC_VIEW_FLAGS) -> ::windows::core::HRESULT,
    pub GetDefaultColumnWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcxchars: *mut u32) -> ::windows::core::HRESULT,
    pub GetDisplayType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdisplaytype: *mut PROPDESC_DISPLAYTYPE) -> ::windows::core::HRESULT,
    pub GetColumnState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcsflags: *mut u32) -> ::windows::core::HRESULT,
    pub GetGroupingRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgr: *mut PROPDESC_GROUPING_RANGE) -> ::windows::core::HRESULT,
    pub GetRelativeDescriptionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prdt: *mut PROPDESC_RELATIVEDESCRIPTION_TYPE) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetRelativeDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszdesc1: *mut ::windows::core::PWSTR, ppszdesc2: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetRelativeDescription: usize,
    pub GetSortDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psd: *mut PROPDESC_SORTDESCRIPTION) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSortDescriptionLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fdescending: super::super::super::Foundation::BOOL, ppszdescription: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSortDescriptionLabel: usize,
    pub GetAggregationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paggtype: *mut PROPDESC_AGGREGATION_TYPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Search_Common")]
    pub GetConditionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Search_Common"))]
    GetConditionType: usize,
    pub GetEnumTypeList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub CoerceToCanonicalValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    CoerceToCanonicalValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub FormatForDisplay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    FormatForDisplay: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub IsValueCanonical: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    IsValueCanonical: usize,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPropertyDescription2(::windows::core::IUnknown);
impl IPropertyDescription2 {
    pub unsafe fn GetPropertyKey(&self, pkey: *mut PROPERTYKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPropertyKey)(::windows::core::Interface::as_raw(self), pkey).ok()
    }
    pub unsafe fn GetCanonicalName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.GetCanonicalName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPropertyType(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).base__.GetPropertyType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.GetDisplayName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetEditInvitation(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.GetEditInvitation)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTypeFlags(&self, mask: PROPDESC_TYPE_FLAGS) -> ::windows::core::Result<PROPDESC_TYPE_FLAGS> {
        let mut result__ = ::windows::core::zeroed::<PROPDESC_TYPE_FLAGS>();
        (::windows::core::Interface::vtable(self).base__.GetTypeFlags)(::windows::core::Interface::as_raw(self), mask, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetViewFlags(&self) -> ::windows::core::Result<PROPDESC_VIEW_FLAGS> {
        let mut result__ = ::windows::core::zeroed::<PROPDESC_VIEW_FLAGS>();
        (::windows::core::Interface::vtable(self).base__.GetViewFlags)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDefaultColumnWidth(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetDefaultColumnWidth)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDisplayType(&self) -> ::windows::core::Result<PROPDESC_DISPLAYTYPE> {
        let mut result__ = ::windows::core::zeroed::<PROPDESC_DISPLAYTYPE>();
        (::windows::core::Interface::vtable(self).base__.GetDisplayType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetColumnState(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetColumnState)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetGroupingRange(&self) -> ::windows::core::Result<PROPDESC_GROUPING_RANGE> {
        let mut result__ = ::windows::core::zeroed::<PROPDESC_GROUPING_RANGE>();
        (::windows::core::Interface::vtable(self).base__.GetGroupingRange)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRelativeDescriptionType(&self) -> ::windows::core::Result<PROPDESC_RELATIVEDESCRIPTION_TYPE> {
        let mut result__ = ::windows::core::zeroed::<PROPDESC_RELATIVEDESCRIPTION_TYPE>();
        (::windows::core::Interface::vtable(self).base__.GetRelativeDescriptionType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetRelativeDescription(&self, propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszdesc1: *mut ::windows::core::PWSTR, ppszdesc2: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRelativeDescription)(::windows::core::Interface::as_raw(self), propvar1, propvar2, ppszdesc1, ppszdesc2).ok()
    }
    pub unsafe fn GetSortDescription(&self) -> ::windows::core::Result<PROPDESC_SORTDESCRIPTION> {
        let mut result__ = ::windows::core::zeroed::<PROPDESC_SORTDESCRIPTION>();
        (::windows::core::Interface::vtable(self).base__.GetSortDescription)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSortDescriptionLabel<P0>(&self, fdescending: P0) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.GetSortDescriptionLabel)(::windows::core::Interface::as_raw(self), fdescending.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAggregationType(&self) -> ::windows::core::Result<PROPDESC_AGGREGATION_TYPE> {
        let mut result__ = ::windows::core::zeroed::<PROPDESC_AGGREGATION_TYPE>();
        (::windows::core::Interface::vtable(self).base__.GetAggregationType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Search_Common\"`*"]
    #[cfg(feature = "Win32_System_Search_Common")]
    pub unsafe fn GetConditionType(&self, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetConditionType)(::windows::core::Interface::as_raw(self), pcontype, popdefault).ok()
    }
    pub unsafe fn GetEnumTypeList<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.GetEnumTypeList)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn CoerceToCanonicalValue(&self, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.CoerceToCanonicalValue)(::windows::core::Interface::as_raw(self), ppropvar).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn FormatForDisplay(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.FormatForDisplay)(::windows::core::Interface::as_raw(self), propvar, pdfflags, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn IsValueCanonical(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.IsValueCanonical)(::windows::core::Interface::as_raw(self), propvar).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetImageReferenceForValue(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetImageReferenceForValue)(::windows::core::Interface::as_raw(self), propvar, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IPropertyDescription2, ::windows::core::IUnknown, IPropertyDescription);
impl ::core::cmp::PartialEq for IPropertyDescription2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyDescription2 {}
impl ::core::fmt::Debug for IPropertyDescription2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyDescription2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPropertyDescription2 {
    type Vtable = IPropertyDescription2_Vtbl;
}
impl ::core::clone::Clone for IPropertyDescription2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPropertyDescription2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57d2eded_5062_400e_b107_5dae79fe57a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyDescription2_Vtbl {
    pub base__: IPropertyDescription_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetImageReferenceForValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszimageres: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetImageReferenceForValue: usize,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPropertyDescriptionAliasInfo(::windows::core::IUnknown);
impl IPropertyDescriptionAliasInfo {
    pub unsafe fn GetPropertyKey(&self, pkey: *mut PROPERTYKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPropertyKey)(::windows::core::Interface::as_raw(self), pkey).ok()
    }
    pub unsafe fn GetCanonicalName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.GetCanonicalName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPropertyType(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).base__.GetPropertyType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.GetDisplayName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetEditInvitation(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.GetEditInvitation)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTypeFlags(&self, mask: PROPDESC_TYPE_FLAGS) -> ::windows::core::Result<PROPDESC_TYPE_FLAGS> {
        let mut result__ = ::windows::core::zeroed::<PROPDESC_TYPE_FLAGS>();
        (::windows::core::Interface::vtable(self).base__.GetTypeFlags)(::windows::core::Interface::as_raw(self), mask, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetViewFlags(&self) -> ::windows::core::Result<PROPDESC_VIEW_FLAGS> {
        let mut result__ = ::windows::core::zeroed::<PROPDESC_VIEW_FLAGS>();
        (::windows::core::Interface::vtable(self).base__.GetViewFlags)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDefaultColumnWidth(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetDefaultColumnWidth)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDisplayType(&self) -> ::windows::core::Result<PROPDESC_DISPLAYTYPE> {
        let mut result__ = ::windows::core::zeroed::<PROPDESC_DISPLAYTYPE>();
        (::windows::core::Interface::vtable(self).base__.GetDisplayType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetColumnState(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetColumnState)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetGroupingRange(&self) -> ::windows::core::Result<PROPDESC_GROUPING_RANGE> {
        let mut result__ = ::windows::core::zeroed::<PROPDESC_GROUPING_RANGE>();
        (::windows::core::Interface::vtable(self).base__.GetGroupingRange)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRelativeDescriptionType(&self) -> ::windows::core::Result<PROPDESC_RELATIVEDESCRIPTION_TYPE> {
        let mut result__ = ::windows::core::zeroed::<PROPDESC_RELATIVEDESCRIPTION_TYPE>();
        (::windows::core::Interface::vtable(self).base__.GetRelativeDescriptionType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetRelativeDescription(&self, propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszdesc1: *mut ::windows::core::PWSTR, ppszdesc2: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRelativeDescription)(::windows::core::Interface::as_raw(self), propvar1, propvar2, ppszdesc1, ppszdesc2).ok()
    }
    pub unsafe fn GetSortDescription(&self) -> ::windows::core::Result<PROPDESC_SORTDESCRIPTION> {
        let mut result__ = ::windows::core::zeroed::<PROPDESC_SORTDESCRIPTION>();
        (::windows::core::Interface::vtable(self).base__.GetSortDescription)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSortDescriptionLabel<P0>(&self, fdescending: P0) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.GetSortDescriptionLabel)(::windows::core::Interface::as_raw(self), fdescending.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAggregationType(&self) -> ::windows::core::Result<PROPDESC_AGGREGATION_TYPE> {
        let mut result__ = ::windows::core::zeroed::<PROPDESC_AGGREGATION_TYPE>();
        (::windows::core::Interface::vtable(self).base__.GetAggregationType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Search_Common\"`*"]
    #[cfg(feature = "Win32_System_Search_Common")]
    pub unsafe fn GetConditionType(&self, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetConditionType)(::windows::core::Interface::as_raw(self), pcontype, popdefault).ok()
    }
    pub unsafe fn GetEnumTypeList<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.GetEnumTypeList)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn CoerceToCanonicalValue(&self, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.CoerceToCanonicalValue)(::windows::core::Interface::as_raw(self), ppropvar).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn FormatForDisplay(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.FormatForDisplay)(::windows::core::Interface::as_raw(self), propvar, pdfflags, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn IsValueCanonical(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.IsValueCanonical)(::windows::core::Interface::as_raw(self), propvar).ok()
    }
    pub unsafe fn GetSortByAlias<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).GetSortByAlias)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAdditionalSortByAliases<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).GetAdditionalSortByAliases)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IPropertyDescriptionAliasInfo, ::windows::core::IUnknown, IPropertyDescription);
impl ::core::cmp::PartialEq for IPropertyDescriptionAliasInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyDescriptionAliasInfo {}
impl ::core::fmt::Debug for IPropertyDescriptionAliasInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyDescriptionAliasInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPropertyDescriptionAliasInfo {
    type Vtable = IPropertyDescriptionAliasInfo_Vtbl;
}
impl ::core::clone::Clone for IPropertyDescriptionAliasInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPropertyDescriptionAliasInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf67104fc_2af9_46fd_b32d_243c1404f3d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyDescriptionAliasInfo_Vtbl {
    pub base__: IPropertyDescription_Vtbl,
    pub GetSortByAlias: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetAdditionalSortByAliases: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPropertyDescriptionList(::windows::core::IUnknown);
impl IPropertyDescriptionList {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAt<T>(&self, ielem: u32) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).GetAt)(::windows::core::Interface::as_raw(self), ielem, &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IPropertyDescriptionList, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPropertyDescriptionList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyDescriptionList {}
impl ::core::fmt::Debug for IPropertyDescriptionList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyDescriptionList").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPropertyDescriptionList {
    type Vtable = IPropertyDescriptionList_Vtbl;
}
impl ::core::clone::Clone for IPropertyDescriptionList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPropertyDescriptionList {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f9fc1d0_c39b_4b26_817f_011967d3440e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyDescriptionList_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelem: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ielem: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPropertyDescriptionRelatedPropertyInfo(::windows::core::IUnknown);
impl IPropertyDescriptionRelatedPropertyInfo {
    pub unsafe fn GetPropertyKey(&self, pkey: *mut PROPERTYKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPropertyKey)(::windows::core::Interface::as_raw(self), pkey).ok()
    }
    pub unsafe fn GetCanonicalName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.GetCanonicalName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPropertyType(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).base__.GetPropertyType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.GetDisplayName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetEditInvitation(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.GetEditInvitation)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTypeFlags(&self, mask: PROPDESC_TYPE_FLAGS) -> ::windows::core::Result<PROPDESC_TYPE_FLAGS> {
        let mut result__ = ::windows::core::zeroed::<PROPDESC_TYPE_FLAGS>();
        (::windows::core::Interface::vtable(self).base__.GetTypeFlags)(::windows::core::Interface::as_raw(self), mask, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetViewFlags(&self) -> ::windows::core::Result<PROPDESC_VIEW_FLAGS> {
        let mut result__ = ::windows::core::zeroed::<PROPDESC_VIEW_FLAGS>();
        (::windows::core::Interface::vtable(self).base__.GetViewFlags)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDefaultColumnWidth(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetDefaultColumnWidth)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDisplayType(&self) -> ::windows::core::Result<PROPDESC_DISPLAYTYPE> {
        let mut result__ = ::windows::core::zeroed::<PROPDESC_DISPLAYTYPE>();
        (::windows::core::Interface::vtable(self).base__.GetDisplayType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetColumnState(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetColumnState)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetGroupingRange(&self) -> ::windows::core::Result<PROPDESC_GROUPING_RANGE> {
        let mut result__ = ::windows::core::zeroed::<PROPDESC_GROUPING_RANGE>();
        (::windows::core::Interface::vtable(self).base__.GetGroupingRange)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRelativeDescriptionType(&self) -> ::windows::core::Result<PROPDESC_RELATIVEDESCRIPTION_TYPE> {
        let mut result__ = ::windows::core::zeroed::<PROPDESC_RELATIVEDESCRIPTION_TYPE>();
        (::windows::core::Interface::vtable(self).base__.GetRelativeDescriptionType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetRelativeDescription(&self, propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszdesc1: *mut ::windows::core::PWSTR, ppszdesc2: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRelativeDescription)(::windows::core::Interface::as_raw(self), propvar1, propvar2, ppszdesc1, ppszdesc2).ok()
    }
    pub unsafe fn GetSortDescription(&self) -> ::windows::core::Result<PROPDESC_SORTDESCRIPTION> {
        let mut result__ = ::windows::core::zeroed::<PROPDESC_SORTDESCRIPTION>();
        (::windows::core::Interface::vtable(self).base__.GetSortDescription)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSortDescriptionLabel<P0>(&self, fdescending: P0) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.GetSortDescriptionLabel)(::windows::core::Interface::as_raw(self), fdescending.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAggregationType(&self) -> ::windows::core::Result<PROPDESC_AGGREGATION_TYPE> {
        let mut result__ = ::windows::core::zeroed::<PROPDESC_AGGREGATION_TYPE>();
        (::windows::core::Interface::vtable(self).base__.GetAggregationType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Search_Common\"`*"]
    #[cfg(feature = "Win32_System_Search_Common")]
    pub unsafe fn GetConditionType(&self, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetConditionType)(::windows::core::Interface::as_raw(self), pcontype, popdefault).ok()
    }
    pub unsafe fn GetEnumTypeList<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.GetEnumTypeList)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn CoerceToCanonicalValue(&self, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.CoerceToCanonicalValue)(::windows::core::Interface::as_raw(self), ppropvar).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn FormatForDisplay(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.FormatForDisplay)(::windows::core::Interface::as_raw(self), propvar, pdfflags, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn IsValueCanonical(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.IsValueCanonical)(::windows::core::Interface::as_raw(self), propvar).ok()
    }
    pub unsafe fn GetRelatedProperty<P0, T>(&self, pszrelationshipname: P0) -> ::windows::core::Result<T>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).GetRelatedProperty)(::windows::core::Interface::as_raw(self), pszrelationshipname.into_param().abi(), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IPropertyDescriptionRelatedPropertyInfo, ::windows::core::IUnknown, IPropertyDescription);
impl ::core::cmp::PartialEq for IPropertyDescriptionRelatedPropertyInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyDescriptionRelatedPropertyInfo {}
impl ::core::fmt::Debug for IPropertyDescriptionRelatedPropertyInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyDescriptionRelatedPropertyInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPropertyDescriptionRelatedPropertyInfo {
    type Vtable = IPropertyDescriptionRelatedPropertyInfo_Vtbl;
}
impl ::core::clone::Clone for IPropertyDescriptionRelatedPropertyInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPropertyDescriptionRelatedPropertyInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x507393f4_2a3d_4a60_b59e_d9c75716c2dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyDescriptionRelatedPropertyInfo_Vtbl {
    pub base__: IPropertyDescription_Vtbl,
    pub GetRelatedProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszrelationshipname: ::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPropertyDescriptionSearchInfo(::windows::core::IUnknown);
impl IPropertyDescriptionSearchInfo {
    pub unsafe fn GetPropertyKey(&self, pkey: *mut PROPERTYKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPropertyKey)(::windows::core::Interface::as_raw(self), pkey).ok()
    }
    pub unsafe fn GetCanonicalName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.GetCanonicalName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPropertyType(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).base__.GetPropertyType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.GetDisplayName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetEditInvitation(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.GetEditInvitation)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTypeFlags(&self, mask: PROPDESC_TYPE_FLAGS) -> ::windows::core::Result<PROPDESC_TYPE_FLAGS> {
        let mut result__ = ::windows::core::zeroed::<PROPDESC_TYPE_FLAGS>();
        (::windows::core::Interface::vtable(self).base__.GetTypeFlags)(::windows::core::Interface::as_raw(self), mask, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetViewFlags(&self) -> ::windows::core::Result<PROPDESC_VIEW_FLAGS> {
        let mut result__ = ::windows::core::zeroed::<PROPDESC_VIEW_FLAGS>();
        (::windows::core::Interface::vtable(self).base__.GetViewFlags)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDefaultColumnWidth(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetDefaultColumnWidth)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDisplayType(&self) -> ::windows::core::Result<PROPDESC_DISPLAYTYPE> {
        let mut result__ = ::windows::core::zeroed::<PROPDESC_DISPLAYTYPE>();
        (::windows::core::Interface::vtable(self).base__.GetDisplayType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetColumnState(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetColumnState)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetGroupingRange(&self) -> ::windows::core::Result<PROPDESC_GROUPING_RANGE> {
        let mut result__ = ::windows::core::zeroed::<PROPDESC_GROUPING_RANGE>();
        (::windows::core::Interface::vtable(self).base__.GetGroupingRange)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRelativeDescriptionType(&self) -> ::windows::core::Result<PROPDESC_RELATIVEDESCRIPTION_TYPE> {
        let mut result__ = ::windows::core::zeroed::<PROPDESC_RELATIVEDESCRIPTION_TYPE>();
        (::windows::core::Interface::vtable(self).base__.GetRelativeDescriptionType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetRelativeDescription(&self, propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszdesc1: *mut ::windows::core::PWSTR, ppszdesc2: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRelativeDescription)(::windows::core::Interface::as_raw(self), propvar1, propvar2, ppszdesc1, ppszdesc2).ok()
    }
    pub unsafe fn GetSortDescription(&self) -> ::windows::core::Result<PROPDESC_SORTDESCRIPTION> {
        let mut result__ = ::windows::core::zeroed::<PROPDESC_SORTDESCRIPTION>();
        (::windows::core::Interface::vtable(self).base__.GetSortDescription)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSortDescriptionLabel<P0>(&self, fdescending: P0) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.GetSortDescriptionLabel)(::windows::core::Interface::as_raw(self), fdescending.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAggregationType(&self) -> ::windows::core::Result<PROPDESC_AGGREGATION_TYPE> {
        let mut result__ = ::windows::core::zeroed::<PROPDESC_AGGREGATION_TYPE>();
        (::windows::core::Interface::vtable(self).base__.GetAggregationType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Search_Common\"`*"]
    #[cfg(feature = "Win32_System_Search_Common")]
    pub unsafe fn GetConditionType(&self, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetConditionType)(::windows::core::Interface::as_raw(self), pcontype, popdefault).ok()
    }
    pub unsafe fn GetEnumTypeList<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.GetEnumTypeList)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn CoerceToCanonicalValue(&self, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.CoerceToCanonicalValue)(::windows::core::Interface::as_raw(self), ppropvar).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn FormatForDisplay(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.FormatForDisplay)(::windows::core::Interface::as_raw(self), propvar, pdfflags, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn IsValueCanonical(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.IsValueCanonical)(::windows::core::Interface::as_raw(self), propvar).ok()
    }
    pub unsafe fn GetSearchInfoFlags(&self) -> ::windows::core::Result<PROPDESC_SEARCHINFO_FLAGS> {
        let mut result__ = ::windows::core::zeroed::<PROPDESC_SEARCHINFO_FLAGS>();
        (::windows::core::Interface::vtable(self).GetSearchInfoFlags)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetColumnIndexType(&self) -> ::windows::core::Result<PROPDESC_COLUMNINDEX_TYPE> {
        let mut result__ = ::windows::core::zeroed::<PROPDESC_COLUMNINDEX_TYPE>();
        (::windows::core::Interface::vtable(self).GetColumnIndexType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetProjectionString(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetProjectionString)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMaxSize(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetMaxSize)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IPropertyDescriptionSearchInfo, ::windows::core::IUnknown, IPropertyDescription);
impl ::core::cmp::PartialEq for IPropertyDescriptionSearchInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyDescriptionSearchInfo {}
impl ::core::fmt::Debug for IPropertyDescriptionSearchInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyDescriptionSearchInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPropertyDescriptionSearchInfo {
    type Vtable = IPropertyDescriptionSearchInfo_Vtbl;
}
impl ::core::clone::Clone for IPropertyDescriptionSearchInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPropertyDescriptionSearchInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x078f91bd_29a2_440f_924e_46a291524520);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyDescriptionSearchInfo_Vtbl {
    pub base__: IPropertyDescription_Vtbl,
    pub GetSearchInfoFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdsiflags: *mut PROPDESC_SEARCHINFO_FLAGS) -> ::windows::core::HRESULT,
    pub GetColumnIndexType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdcitype: *mut PROPDESC_COLUMNINDEX_TYPE) -> ::windows::core::HRESULT,
    pub GetProjectionString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszprojection: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetMaxSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbmaxsize: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPropertyEnumType(::windows::core::IUnknown);
impl IPropertyEnumType {
    pub unsafe fn GetEnumType(&self) -> ::windows::core::Result<PROPENUMTYPE> {
        let mut result__ = ::windows::core::zeroed::<PROPENUMTYPE>();
        (::windows::core::Interface::vtable(self).GetEnumType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetValue(&self) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>();
        (::windows::core::Interface::vtable(self).GetValue)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetRangeMinValue(&self) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>();
        (::windows::core::Interface::vtable(self).GetRangeMinValue)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetRangeSetValue(&self) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>();
        (::windows::core::Interface::vtable(self).GetRangeSetValue)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDisplayText(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetDisplayText)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IPropertyEnumType, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPropertyEnumType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyEnumType {}
impl ::core::fmt::Debug for IPropertyEnumType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyEnumType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPropertyEnumType {
    type Vtable = IPropertyEnumType_Vtbl;
}
impl ::core::clone::Clone for IPropertyEnumType {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPropertyEnumType {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11e1fbf9_2d56_4a6b_8db3_7cd193a471f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyEnumType_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetEnumType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penumtype: *mut PROPENUMTYPE) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetRangeMinValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropvarmin: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetRangeMinValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetRangeSetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropvarset: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetRangeSetValue: usize,
    pub GetDisplayText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszdisplay: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPropertyEnumType2(::windows::core::IUnknown);
impl IPropertyEnumType2 {
    pub unsafe fn GetEnumType(&self) -> ::windows::core::Result<PROPENUMTYPE> {
        let mut result__ = ::windows::core::zeroed::<PROPENUMTYPE>();
        (::windows::core::Interface::vtable(self).base__.GetEnumType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetValue(&self) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>();
        (::windows::core::Interface::vtable(self).base__.GetValue)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetRangeMinValue(&self) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>();
        (::windows::core::Interface::vtable(self).base__.GetRangeMinValue)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetRangeSetValue(&self) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>();
        (::windows::core::Interface::vtable(self).base__.GetRangeSetValue)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDisplayText(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.GetDisplayText)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetImageReference(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetImageReference)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IPropertyEnumType2, ::windows::core::IUnknown, IPropertyEnumType);
impl ::core::cmp::PartialEq for IPropertyEnumType2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyEnumType2 {}
impl ::core::fmt::Debug for IPropertyEnumType2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyEnumType2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPropertyEnumType2 {
    type Vtable = IPropertyEnumType2_Vtbl;
}
impl ::core::clone::Clone for IPropertyEnumType2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPropertyEnumType2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b6e051c_5ddd_4321_9070_fe2acb55e794);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyEnumType2_Vtbl {
    pub base__: IPropertyEnumType_Vtbl,
    pub GetImageReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszimageres: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPropertyEnumTypeList(::windows::core::IUnknown);
impl IPropertyEnumTypeList {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAt<T>(&self, itype: u32) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).GetAt)(::windows::core::Interface::as_raw(self), itype, &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetConditionAt<T>(&self, nindex: u32) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).GetConditionAt)(::windows::core::Interface::as_raw(self), nindex, &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn FindMatchingIndex(&self, propvarcmp: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).FindMatchingIndex)(::windows::core::Interface::as_raw(self), propvarcmp, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IPropertyEnumTypeList, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPropertyEnumTypeList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyEnumTypeList {}
impl ::core::fmt::Debug for IPropertyEnumTypeList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyEnumTypeList").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPropertyEnumTypeList {
    type Vtable = IPropertyEnumTypeList_Vtbl;
}
impl ::core::clone::Clone for IPropertyEnumTypeList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPropertyEnumTypeList {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa99400f4_3d84_4557_94ba_1242fb2cc9a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyEnumTypeList_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctypes: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itype: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetConditionAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub FindMatchingIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propvarcmp: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pnindex: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    FindMatchingIndex: usize,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPropertyStore(::windows::core::IUnknown);
impl IPropertyStore {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, iprop: u32, pkey: *mut PROPERTYKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAt)(::windows::core::Interface::as_raw(self), iprop, pkey).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetValue(&self, key: *const PROPERTYKEY) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>();
        (::windows::core::Interface::vtable(self).GetValue)(::windows::core::Interface::as_raw(self), key, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetValue(&self, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetValue)(::windows::core::Interface::as_raw(self), key, propvar).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Commit)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IPropertyStore, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPropertyStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyStore {}
impl ::core::fmt::Debug for IPropertyStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyStore").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPropertyStore {
    type Vtable = IPropertyStore_Vtbl;
}
impl ::core::clone::Clone for IPropertyStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPropertyStore {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x886d8eeb_8cf2_4446_8d02_cdba1dbdcf99);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStore_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cprops: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iprop: u32, pkey: *mut PROPERTYKEY) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, pv: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    SetValue: usize,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPropertyStoreCache(::windows::core::IUnknown);
impl IPropertyStoreCache {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, iprop: u32, pkey: *mut PROPERTYKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetAt)(::windows::core::Interface::as_raw(self), iprop, pkey).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetValue(&self, key: *const PROPERTYKEY) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::StructuredStorage::PROPVARIANT>();
        (::windows::core::Interface::vtable(self).base__.GetValue)(::windows::core::Interface::as_raw(self), key, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetValue(&self, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetValue)(::windows::core::Interface::as_raw(self), key, propvar).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Commit)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetState(&self, key: *const PROPERTYKEY) -> ::windows::core::Result<PSC_STATE> {
        let mut result__ = ::windows::core::zeroed::<PSC_STATE>();
        (::windows::core::Interface::vtable(self).GetState)(::windows::core::Interface::as_raw(self), key, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetValueAndState(&self, key: *const PROPERTYKEY, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT, pstate: *mut PSC_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetValueAndState)(::windows::core::Interface::as_raw(self), key, ppropvar, pstate).ok()
    }
    pub unsafe fn SetState(&self, key: *const PROPERTYKEY, state: PSC_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetState)(::windows::core::Interface::as_raw(self), key, state).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetValueAndState(&self, key: *const PROPERTYKEY, ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, state: PSC_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetValueAndState)(::windows::core::Interface::as_raw(self), key, ppropvar, state).ok()
    }
}
::windows::imp::interface_hierarchy!(IPropertyStoreCache, ::windows::core::IUnknown, IPropertyStore);
impl ::core::cmp::PartialEq for IPropertyStoreCache {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyStoreCache {}
impl ::core::fmt::Debug for IPropertyStoreCache {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyStoreCache").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPropertyStoreCache {
    type Vtable = IPropertyStoreCache_Vtbl;
}
impl ::core::clone::Clone for IPropertyStoreCache {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPropertyStoreCache {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3017056d_9a91_4e90_937d_746c72abbf4f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStoreCache_Vtbl {
    pub base__: IPropertyStore_Vtbl,
    pub GetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, pstate: *mut PSC_STATE) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetValueAndState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT, pstate: *mut PSC_STATE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetValueAndState: usize,
    pub SetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, state: PSC_STATE) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub SetValueAndState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, state: PSC_STATE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    SetValueAndState: usize,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPropertyStoreCapabilities(::windows::core::IUnknown);
impl IPropertyStoreCapabilities {
    pub unsafe fn IsPropertyWritable(&self, key: *const PROPERTYKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsPropertyWritable)(::windows::core::Interface::as_raw(self), key).ok()
    }
}
::windows::imp::interface_hierarchy!(IPropertyStoreCapabilities, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPropertyStoreCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyStoreCapabilities {}
impl ::core::fmt::Debug for IPropertyStoreCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyStoreCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPropertyStoreCapabilities {
    type Vtable = IPropertyStoreCapabilities_Vtbl;
}
impl ::core::clone::Clone for IPropertyStoreCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPropertyStoreCapabilities {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8e2d566_186e_4d49_bf41_6909ead56acc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStoreCapabilities_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub IsPropertyWritable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPropertyStoreFactory(::windows::core::IUnknown);
impl IPropertyStoreFactory {
    pub unsafe fn GetPropertyStore<P0, T>(&self, flags: GETPROPERTYSTOREFLAGS, punkfactory: P0) -> ::windows::core::Result<T>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).GetPropertyStore)(::windows::core::Interface::as_raw(self), flags, punkfactory.into_param().abi(), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPropertyStoreForKeys<T>(&self, rgkeys: *const PROPERTYKEY, ckeys: u32, flags: GETPROPERTYSTOREFLAGS) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).GetPropertyStoreForKeys)(::windows::core::Interface::as_raw(self), rgkeys, ckeys, flags, &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IPropertyStoreFactory, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPropertyStoreFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyStoreFactory {}
impl ::core::fmt::Debug for IPropertyStoreFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyStoreFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPropertyStoreFactory {
    type Vtable = IPropertyStoreFactory_Vtbl;
}
impl ::core::clone::Clone for IPropertyStoreFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPropertyStoreFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc110b6d_57e8_4148_a9c6_91015ab2f3a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStoreFactory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetPropertyStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: GETPROPERTYSTOREFLAGS, punkfactory: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPropertyStoreForKeys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rgkeys: *const PROPERTYKEY, ckeys: u32, flags: GETPROPERTYSTOREFLAGS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPropertySystem(::windows::core::IUnknown);
impl IPropertySystem {
    pub unsafe fn GetPropertyDescription<T>(&self, propkey: *const PROPERTYKEY) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).GetPropertyDescription)(::windows::core::Interface::as_raw(self), propkey, &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPropertyDescriptionByName<P0, T>(&self, pszcanonicalname: P0) -> ::windows::core::Result<T>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).GetPropertyDescriptionByName)(::windows::core::Interface::as_raw(self), pszcanonicalname.into_param().abi(), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPropertyDescriptionListFromString<P0, T>(&self, pszproplist: P0) -> ::windows::core::Result<T>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).GetPropertyDescriptionListFromString)(::windows::core::Interface::as_raw(self), pszproplist.into_param().abi(), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumeratePropertyDescriptions<T>(&self, filteron: PROPDESC_ENUMFILTER) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).EnumeratePropertyDescriptions)(::windows::core::Interface::as_raw(self), filteron, &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn FormatForDisplay(&self, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS, psztext: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FormatForDisplay)(::windows::core::Interface::as_raw(self), key, propvar, pdff, ::core::mem::transmute(psztext.as_ptr()), psztext.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn FormatForDisplayAlloc(&self, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).FormatForDisplayAlloc)(::windows::core::Interface::as_raw(self), key, propvar, pdff, &mut result__).from_abi(result__)
    }
    pub unsafe fn RegisterPropertySchema<P0>(&self, pszpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).RegisterPropertySchema)(::windows::core::Interface::as_raw(self), pszpath.into_param().abi()).ok()
    }
    pub unsafe fn UnregisterPropertySchema<P0>(&self, pszpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).UnregisterPropertySchema)(::windows::core::Interface::as_raw(self), pszpath.into_param().abi()).ok()
    }
    pub unsafe fn RefreshPropertySchema(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RefreshPropertySchema)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IPropertySystem, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPropertySystem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertySystem {}
impl ::core::fmt::Debug for IPropertySystem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertySystem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPropertySystem {
    type Vtable = IPropertySystem_Vtbl;
}
impl ::core::clone::Clone for IPropertySystem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPropertySystem {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca724e8a_c3e6_442b_88a4_6fb0db8035a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertySystem_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetPropertyDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propkey: *const PROPERTYKEY, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPropertyDescriptionByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcanonicalname: ::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPropertyDescriptionListFromString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszproplist: ::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumeratePropertyDescriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filteron: PROPDESC_ENUMFILTER, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub FormatForDisplay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS, psztext: ::windows::core::PWSTR, cchtext: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    FormatForDisplay: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub FormatForDisplayAlloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    FormatForDisplayAlloc: usize,
    pub RegisterPropertySchema: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub UnregisterPropertySchema: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub RefreshPropertySchema: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPropertySystemChangeNotify(::windows::core::IUnknown);
impl IPropertySystemChangeNotify {
    pub unsafe fn SchemaRefreshed(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SchemaRefreshed)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IPropertySystemChangeNotify, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPropertySystemChangeNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertySystemChangeNotify {}
impl ::core::fmt::Debug for IPropertySystemChangeNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertySystemChangeNotify").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPropertySystemChangeNotify {
    type Vtable = IPropertySystemChangeNotify_Vtbl;
}
impl ::core::clone::Clone for IPropertySystemChangeNotify {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPropertySystemChangeNotify {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa955fd9_38be_4879_a6ce_824cf52d609f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertySystemChangeNotify_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SchemaRefreshed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPropertyUI(::windows::core::IUnknown);
impl IPropertyUI {
    pub unsafe fn ParsePropertyName<P0>(&self, pszname: P0, pfmtid: *mut ::windows::core::GUID, ppid: *mut u32, pcheaten: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).ParsePropertyName)(::windows::core::Interface::as_raw(self), pszname.into_param().abi(), pfmtid, ppid, pcheaten).ok()
    }
    pub unsafe fn GetCannonicalName(&self, fmtid: *const ::windows::core::GUID, pid: u32, pwsztext: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCannonicalName)(::windows::core::Interface::as_raw(self), fmtid, pid, ::core::mem::transmute(pwsztext.as_ptr()), pwsztext.len() as _).ok()
    }
    pub unsafe fn GetDisplayName(&self, fmtid: *const ::windows::core::GUID, pid: u32, flags: PROPERTYUI_NAME_FLAGS, pwsztext: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDisplayName)(::windows::core::Interface::as_raw(self), fmtid, pid, flags, ::core::mem::transmute(pwsztext.as_ptr()), pwsztext.len() as _).ok()
    }
    pub unsafe fn GetPropertyDescription(&self, fmtid: *const ::windows::core::GUID, pid: u32, pwsztext: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPropertyDescription)(::windows::core::Interface::as_raw(self), fmtid, pid, ::core::mem::transmute(pwsztext.as_ptr()), pwsztext.len() as _).ok()
    }
    pub unsafe fn GetDefaultWidth(&self, fmtid: *const ::windows::core::GUID, pid: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetDefaultWidth)(::windows::core::Interface::as_raw(self), fmtid, pid, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFlags(&self, fmtid: *const ::windows::core::GUID, pid: u32) -> ::windows::core::Result<PROPERTYUI_FLAGS> {
        let mut result__ = ::windows::core::zeroed::<PROPERTYUI_FLAGS>();
        (::windows::core::Interface::vtable(self).GetFlags)(::windows::core::Interface::as_raw(self), fmtid, pid, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn FormatForDisplay(&self, fmtid: *const ::windows::core::GUID, pid: u32, ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, puiff: PROPERTYUI_FORMAT_FLAGS, pwsztext: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FormatForDisplay)(::windows::core::Interface::as_raw(self), fmtid, pid, ppropvar, puiff, ::core::mem::transmute(pwsztext.as_ptr()), pwsztext.len() as _).ok()
    }
    pub unsafe fn GetHelpInfo(&self, fmtid: *const ::windows::core::GUID, pid: u32, pwszhelpfile: &mut [u16], puhelpid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetHelpInfo)(::windows::core::Interface::as_raw(self), fmtid, pid, ::core::mem::transmute(pwszhelpfile.as_ptr()), pwszhelpfile.len() as _, puhelpid).ok()
    }
}
::windows::imp::interface_hierarchy!(IPropertyUI, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPropertyUI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyUI {}
impl ::core::fmt::Debug for IPropertyUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyUI").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPropertyUI {
    type Vtable = IPropertyUI_Vtbl;
}
impl ::core::clone::Clone for IPropertyUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPropertyUI {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x757a7d9f_919a_4118_99d7_dbb208c8cc66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyUI_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ParsePropertyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, pfmtid: *mut ::windows::core::GUID, ppid: *mut u32, pcheaten: *mut u32) -> ::windows::core::HRESULT,
    pub GetCannonicalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmtid: *const ::windows::core::GUID, pid: u32, pwsztext: ::windows::core::PWSTR, cchtext: u32) -> ::windows::core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmtid: *const ::windows::core::GUID, pid: u32, flags: PROPERTYUI_NAME_FLAGS, pwsztext: ::windows::core::PWSTR, cchtext: u32) -> ::windows::core::HRESULT,
    pub GetPropertyDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmtid: *const ::windows::core::GUID, pid: u32, pwsztext: ::windows::core::PWSTR, cchtext: u32) -> ::windows::core::HRESULT,
    pub GetDefaultWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmtid: *const ::windows::core::GUID, pid: u32, pcxchars: *mut u32) -> ::windows::core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmtid: *const ::windows::core::GUID, pid: u32, pflags: *mut PROPERTYUI_FLAGS) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub FormatForDisplay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmtid: *const ::windows::core::GUID, pid: u32, ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, puiff: PROPERTYUI_FORMAT_FLAGS, pwsztext: ::windows::core::PWSTR, cchtext: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    FormatForDisplay: usize,
    pub GetHelpInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmtid: *const ::windows::core::GUID, pid: u32, pwszhelpfile: ::windows::core::PWSTR, cch: u32, puhelpid: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const InMemoryPropertyStore: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a02e012_6303_4e1e_b9a1_630f802592c5);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const InMemoryPropertyStoreMarshalByValue: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4ca0e2d_6da7_4b75_a97c_5f306f0eaedc);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PKEY_PIDSTR_MAX: u32 = 10u32;
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PropertySystem: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8967f85_58ae_4f46_9fb2_5d7904798f4b);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DRAWPROGRESSFLAGS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const DPF_NONE: DRAWPROGRESSFLAGS = DRAWPROGRESSFLAGS(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const DPF_MARQUEE: DRAWPROGRESSFLAGS = DRAWPROGRESSFLAGS(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const DPF_MARQUEE_COMPLETE: DRAWPROGRESSFLAGS = DRAWPROGRESSFLAGS(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const DPF_ERROR: DRAWPROGRESSFLAGS = DRAWPROGRESSFLAGS(4i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const DPF_WARNING: DRAWPROGRESSFLAGS = DRAWPROGRESSFLAGS(8i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const DPF_STOPPED: DRAWPROGRESSFLAGS = DRAWPROGRESSFLAGS(16i32);
impl ::core::marker::Copy for DRAWPROGRESSFLAGS {}
impl ::core::clone::Clone for DRAWPROGRESSFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRAWPROGRESSFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DRAWPROGRESSFLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DRAWPROGRESSFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRAWPROGRESSFLAGS").field(&self.0).finish()
    }
}
impl DRAWPROGRESSFLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for DRAWPROGRESSFLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DRAWPROGRESSFLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DRAWPROGRESSFLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DRAWPROGRESSFLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DRAWPROGRESSFLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GETPROPERTYSTOREFLAGS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const GPS_DEFAULT: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const GPS_HANDLERPROPERTIESONLY: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const GPS_READWRITE: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const GPS_TEMPORARY: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(4i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const GPS_FASTPROPERTIESONLY: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(8i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const GPS_OPENSLOWITEM: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(16i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const GPS_DELAYCREATION: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(32i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const GPS_BESTEFFORT: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(64i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const GPS_NO_OPLOCK: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(128i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const GPS_PREFERQUERYPROPERTIES: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(256i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const GPS_EXTRINSICPROPERTIES: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(512i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const GPS_EXTRINSICPROPERTIESONLY: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(1024i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const GPS_VOLATILEPROPERTIES: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(2048i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const GPS_VOLATILEPROPERTIESONLY: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(4096i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const GPS_MASK_VALID: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(8191i32);
impl ::core::marker::Copy for GETPROPERTYSTOREFLAGS {}
impl ::core::clone::Clone for GETPROPERTYSTOREFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GETPROPERTYSTOREFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for GETPROPERTYSTOREFLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for GETPROPERTYSTOREFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GETPROPERTYSTOREFLAGS").field(&self.0).finish()
    }
}
impl GETPROPERTYSTOREFLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for GETPROPERTYSTOREFLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GETPROPERTYSTOREFLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GETPROPERTYSTOREFLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GETPROPERTYSTOREFLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GETPROPERTYSTOREFLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PDOPSTATUS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDOPS_RUNNING: PDOPSTATUS = PDOPSTATUS(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDOPS_PAUSED: PDOPSTATUS = PDOPSTATUS(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDOPS_CANCELLED: PDOPSTATUS = PDOPSTATUS(3i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDOPS_STOPPED: PDOPSTATUS = PDOPSTATUS(4i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDOPS_ERRORS: PDOPSTATUS = PDOPSTATUS(5i32);
impl ::core::marker::Copy for PDOPSTATUS {}
impl ::core::clone::Clone for PDOPSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PDOPSTATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PDOPSTATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PDOPSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PDOPSTATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PKA_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PKA_SET: PKA_FLAGS = PKA_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PKA_APPEND: PKA_FLAGS = PKA_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PKA_DELETE: PKA_FLAGS = PKA_FLAGS(2i32);
impl ::core::marker::Copy for PKA_FLAGS {}
impl ::core::clone::Clone for PKA_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PKA_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PKA_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PKA_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PKA_FLAGS").field(&self.0).finish()
    }
}
impl PKA_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PKA_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PKA_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PKA_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PKA_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PKA_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PLACEHOLDER_STATES(pub i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PS_NONE: PLACEHOLDER_STATES = PLACEHOLDER_STATES(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PS_MARKED_FOR_OFFLINE_AVAILABILITY: PLACEHOLDER_STATES = PLACEHOLDER_STATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PS_FULL_PRIMARY_STREAM_AVAILABLE: PLACEHOLDER_STATES = PLACEHOLDER_STATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PS_CREATE_FILE_ACCESSIBLE: PLACEHOLDER_STATES = PLACEHOLDER_STATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PS_CLOUDFILE_PLACEHOLDER: PLACEHOLDER_STATES = PLACEHOLDER_STATES(8i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PS_DEFAULT: PLACEHOLDER_STATES = PLACEHOLDER_STATES(7i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PS_ALL: PLACEHOLDER_STATES = PLACEHOLDER_STATES(15i32);
impl ::core::marker::Copy for PLACEHOLDER_STATES {}
impl ::core::clone::Clone for PLACEHOLDER_STATES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PLACEHOLDER_STATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PLACEHOLDER_STATES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PLACEHOLDER_STATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PLACEHOLDER_STATES").field(&self.0).finish()
    }
}
impl PLACEHOLDER_STATES {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PLACEHOLDER_STATES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PLACEHOLDER_STATES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PLACEHOLDER_STATES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PLACEHOLDER_STATES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PLACEHOLDER_STATES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPDESC_AGGREGATION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDAT_DEFAULT: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDAT_FIRST: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDAT_SUM: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDAT_AVERAGE: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(3i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDAT_DATERANGE: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(4i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDAT_UNION: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(5i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDAT_MAX: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(6i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDAT_MIN: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(7i32);
impl ::core::marker::Copy for PROPDESC_AGGREGATION_TYPE {}
impl ::core::clone::Clone for PROPDESC_AGGREGATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPDESC_AGGREGATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROPDESC_AGGREGATION_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROPDESC_AGGREGATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPDESC_AGGREGATION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPDESC_COLUMNINDEX_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDCIT_NONE: PROPDESC_COLUMNINDEX_TYPE = PROPDESC_COLUMNINDEX_TYPE(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDCIT_ONDISK: PROPDESC_COLUMNINDEX_TYPE = PROPDESC_COLUMNINDEX_TYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDCIT_INMEMORY: PROPDESC_COLUMNINDEX_TYPE = PROPDESC_COLUMNINDEX_TYPE(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDCIT_ONDEMAND: PROPDESC_COLUMNINDEX_TYPE = PROPDESC_COLUMNINDEX_TYPE(3i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDCIT_ONDISKALL: PROPDESC_COLUMNINDEX_TYPE = PROPDESC_COLUMNINDEX_TYPE(4i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDCIT_ONDISKVECTOR: PROPDESC_COLUMNINDEX_TYPE = PROPDESC_COLUMNINDEX_TYPE(5i32);
impl ::core::marker::Copy for PROPDESC_COLUMNINDEX_TYPE {}
impl ::core::clone::Clone for PROPDESC_COLUMNINDEX_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPDESC_COLUMNINDEX_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROPDESC_COLUMNINDEX_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROPDESC_COLUMNINDEX_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPDESC_COLUMNINDEX_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPDESC_CONDITION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDCOT_NONE: PROPDESC_CONDITION_TYPE = PROPDESC_CONDITION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDCOT_STRING: PROPDESC_CONDITION_TYPE = PROPDESC_CONDITION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDCOT_SIZE: PROPDESC_CONDITION_TYPE = PROPDESC_CONDITION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDCOT_DATETIME: PROPDESC_CONDITION_TYPE = PROPDESC_CONDITION_TYPE(3i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDCOT_BOOLEAN: PROPDESC_CONDITION_TYPE = PROPDESC_CONDITION_TYPE(4i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDCOT_NUMBER: PROPDESC_CONDITION_TYPE = PROPDESC_CONDITION_TYPE(5i32);
impl ::core::marker::Copy for PROPDESC_CONDITION_TYPE {}
impl ::core::clone::Clone for PROPDESC_CONDITION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPDESC_CONDITION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROPDESC_CONDITION_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROPDESC_CONDITION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPDESC_CONDITION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPDESC_DISPLAYTYPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDDT_STRING: PROPDESC_DISPLAYTYPE = PROPDESC_DISPLAYTYPE(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDDT_NUMBER: PROPDESC_DISPLAYTYPE = PROPDESC_DISPLAYTYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDDT_BOOLEAN: PROPDESC_DISPLAYTYPE = PROPDESC_DISPLAYTYPE(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDDT_DATETIME: PROPDESC_DISPLAYTYPE = PROPDESC_DISPLAYTYPE(3i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDDT_ENUMERATED: PROPDESC_DISPLAYTYPE = PROPDESC_DISPLAYTYPE(4i32);
impl ::core::marker::Copy for PROPDESC_DISPLAYTYPE {}
impl ::core::clone::Clone for PROPDESC_DISPLAYTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPDESC_DISPLAYTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROPDESC_DISPLAYTYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROPDESC_DISPLAYTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPDESC_DISPLAYTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPDESC_ENUMFILTER(pub i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDEF_ALL: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDEF_SYSTEM: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDEF_NONSYSTEM: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDEF_VIEWABLE: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(3i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDEF_QUERYABLE: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(4i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDEF_INFULLTEXTQUERY: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(5i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDEF_COLUMN: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(6i32);
impl ::core::marker::Copy for PROPDESC_ENUMFILTER {}
impl ::core::clone::Clone for PROPDESC_ENUMFILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPDESC_ENUMFILTER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROPDESC_ENUMFILTER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROPDESC_ENUMFILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPDESC_ENUMFILTER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPDESC_FORMAT_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDFF_DEFAULT: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDFF_PREFIXNAME: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDFF_FILENAME: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDFF_ALWAYSKB: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDFF_RESERVED_RIGHTTOLEFT: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDFF_SHORTTIME: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDFF_LONGTIME: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(32i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDFF_HIDETIME: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(64i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDFF_SHORTDATE: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(128i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDFF_LONGDATE: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(256i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDFF_HIDEDATE: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(512i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDFF_RELATIVEDATE: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(1024i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDFF_USEEDITINVITATION: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(2048i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDFF_READONLY: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(4096i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDFF_NOAUTOREADINGORDER: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(8192i32);
impl ::core::marker::Copy for PROPDESC_FORMAT_FLAGS {}
impl ::core::clone::Clone for PROPDESC_FORMAT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPDESC_FORMAT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROPDESC_FORMAT_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROPDESC_FORMAT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPDESC_FORMAT_FLAGS").field(&self.0).finish()
    }
}
impl PROPDESC_FORMAT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PROPDESC_FORMAT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROPDESC_FORMAT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROPDESC_FORMAT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROPDESC_FORMAT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROPDESC_FORMAT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPDESC_GROUPING_RANGE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDGR_DISCRETE: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDGR_ALPHANUMERIC: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDGR_SIZE: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDGR_DYNAMIC: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(3i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDGR_DATE: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(4i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDGR_PERCENT: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(5i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDGR_ENUMERATED: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(6i32);
impl ::core::marker::Copy for PROPDESC_GROUPING_RANGE {}
impl ::core::clone::Clone for PROPDESC_GROUPING_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPDESC_GROUPING_RANGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROPDESC_GROUPING_RANGE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROPDESC_GROUPING_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPDESC_GROUPING_RANGE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPDESC_RELATIVEDESCRIPTION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDRDT_GENERAL: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDRDT_DATE: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDRDT_SIZE: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDRDT_COUNT: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(3i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDRDT_REVISION: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(4i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDRDT_LENGTH: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(5i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDRDT_DURATION: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(6i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDRDT_SPEED: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(7i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDRDT_RATE: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(8i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDRDT_RATING: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(9i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDRDT_PRIORITY: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(10i32);
impl ::core::marker::Copy for PROPDESC_RELATIVEDESCRIPTION_TYPE {}
impl ::core::clone::Clone for PROPDESC_RELATIVEDESCRIPTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPDESC_RELATIVEDESCRIPTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROPDESC_RELATIVEDESCRIPTION_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROPDESC_RELATIVEDESCRIPTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPDESC_RELATIVEDESCRIPTION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPDESC_SEARCHINFO_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDSIF_DEFAULT: PROPDESC_SEARCHINFO_FLAGS = PROPDESC_SEARCHINFO_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDSIF_ININVERTEDINDEX: PROPDESC_SEARCHINFO_FLAGS = PROPDESC_SEARCHINFO_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDSIF_ISCOLUMN: PROPDESC_SEARCHINFO_FLAGS = PROPDESC_SEARCHINFO_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDSIF_ISCOLUMNSPARSE: PROPDESC_SEARCHINFO_FLAGS = PROPDESC_SEARCHINFO_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDSIF_ALWAYSINCLUDE: PROPDESC_SEARCHINFO_FLAGS = PROPDESC_SEARCHINFO_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDSIF_USEFORTYPEAHEAD: PROPDESC_SEARCHINFO_FLAGS = PROPDESC_SEARCHINFO_FLAGS(16i32);
impl ::core::marker::Copy for PROPDESC_SEARCHINFO_FLAGS {}
impl ::core::clone::Clone for PROPDESC_SEARCHINFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPDESC_SEARCHINFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROPDESC_SEARCHINFO_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROPDESC_SEARCHINFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPDESC_SEARCHINFO_FLAGS").field(&self.0).finish()
    }
}
impl PROPDESC_SEARCHINFO_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PROPDESC_SEARCHINFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROPDESC_SEARCHINFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROPDESC_SEARCHINFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROPDESC_SEARCHINFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROPDESC_SEARCHINFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPDESC_SORTDESCRIPTION(pub i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDSD_GENERAL: PROPDESC_SORTDESCRIPTION = PROPDESC_SORTDESCRIPTION(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDSD_A_Z: PROPDESC_SORTDESCRIPTION = PROPDESC_SORTDESCRIPTION(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDSD_LOWEST_HIGHEST: PROPDESC_SORTDESCRIPTION = PROPDESC_SORTDESCRIPTION(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDSD_SMALLEST_BIGGEST: PROPDESC_SORTDESCRIPTION = PROPDESC_SORTDESCRIPTION(3i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDSD_OLDEST_NEWEST: PROPDESC_SORTDESCRIPTION = PROPDESC_SORTDESCRIPTION(4i32);
impl ::core::marker::Copy for PROPDESC_SORTDESCRIPTION {}
impl ::core::clone::Clone for PROPDESC_SORTDESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPDESC_SORTDESCRIPTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROPDESC_SORTDESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROPDESC_SORTDESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPDESC_SORTDESCRIPTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPDESC_TYPE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDTF_DEFAULT: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDTF_MULTIPLEVALUES: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDTF_ISINNATE: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDTF_ISGROUP: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDTF_CANGROUPBY: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDTF_CANSTACKBY: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDTF_ISTREEPROPERTY: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDTF_INCLUDEINFULLTEXTQUERY: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDTF_ISVIEWABLE: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDTF_ISQUERYABLE: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDTF_CANBEPURGED: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDTF_SEARCHRAWVALUE: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDTF_DONTCOERCEEMPTYSTRINGS: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDTF_ALWAYSINSUPPLEMENTALSTORE: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDTF_ISSYSTEMPROPERTY: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(2147483648u32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDTF_MASK_ALL: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(2147491839u32);
impl ::core::marker::Copy for PROPDESC_TYPE_FLAGS {}
impl ::core::clone::Clone for PROPDESC_TYPE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPDESC_TYPE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROPDESC_TYPE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROPDESC_TYPE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPDESC_TYPE_FLAGS").field(&self.0).finish()
    }
}
impl PROPDESC_TYPE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PROPDESC_TYPE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROPDESC_TYPE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROPDESC_TYPE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROPDESC_TYPE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROPDESC_TYPE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPDESC_VIEW_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDVF_DEFAULT: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDVF_CENTERALIGN: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDVF_RIGHTALIGN: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDVF_BEGINNEWGROUP: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDVF_FILLAREA: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDVF_SORTDESCENDING: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDVF_SHOWONLYIFPRESENT: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(32i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDVF_SHOWBYDEFAULT: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(64i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDVF_SHOWINPRIMARYLIST: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(128i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDVF_SHOWINSECONDARYLIST: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(256i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDVF_HIDELABEL: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(512i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDVF_HIDDEN: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(2048i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDVF_CANWRAP: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(4096i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDVF_MASK_ALL: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(7167i32);
impl ::core::marker::Copy for PROPDESC_VIEW_FLAGS {}
impl ::core::clone::Clone for PROPDESC_VIEW_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPDESC_VIEW_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROPDESC_VIEW_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROPDESC_VIEW_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPDESC_VIEW_FLAGS").field(&self.0).finish()
    }
}
impl PROPDESC_VIEW_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PROPDESC_VIEW_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROPDESC_VIEW_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROPDESC_VIEW_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROPDESC_VIEW_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROPDESC_VIEW_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPENUMTYPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PET_DISCRETEVALUE: PROPENUMTYPE = PROPENUMTYPE(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PET_RANGEDVALUE: PROPENUMTYPE = PROPENUMTYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PET_DEFAULTVALUE: PROPENUMTYPE = PROPENUMTYPE(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PET_ENDRANGE: PROPENUMTYPE = PROPENUMTYPE(3i32);
impl ::core::marker::Copy for PROPENUMTYPE {}
impl ::core::clone::Clone for PROPENUMTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPENUMTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROPENUMTYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROPENUMTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPENUMTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPERTYUI_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PUIF_DEFAULT: PROPERTYUI_FLAGS = PROPERTYUI_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PUIF_RIGHTALIGN: PROPERTYUI_FLAGS = PROPERTYUI_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PUIF_NOLABELININFOTIP: PROPERTYUI_FLAGS = PROPERTYUI_FLAGS(2i32);
impl ::core::marker::Copy for PROPERTYUI_FLAGS {}
impl ::core::clone::Clone for PROPERTYUI_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPERTYUI_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROPERTYUI_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROPERTYUI_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPERTYUI_FLAGS").field(&self.0).finish()
    }
}
impl PROPERTYUI_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PROPERTYUI_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROPERTYUI_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROPERTYUI_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROPERTYUI_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROPERTYUI_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPERTYUI_FORMAT_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PUIFFDF_DEFAULT: PROPERTYUI_FORMAT_FLAGS = PROPERTYUI_FORMAT_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PUIFFDF_RIGHTTOLEFT: PROPERTYUI_FORMAT_FLAGS = PROPERTYUI_FORMAT_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PUIFFDF_SHORTFORMAT: PROPERTYUI_FORMAT_FLAGS = PROPERTYUI_FORMAT_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PUIFFDF_NOTIME: PROPERTYUI_FORMAT_FLAGS = PROPERTYUI_FORMAT_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PUIFFDF_FRIENDLYDATE: PROPERTYUI_FORMAT_FLAGS = PROPERTYUI_FORMAT_FLAGS(8i32);
impl ::core::marker::Copy for PROPERTYUI_FORMAT_FLAGS {}
impl ::core::clone::Clone for PROPERTYUI_FORMAT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPERTYUI_FORMAT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROPERTYUI_FORMAT_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROPERTYUI_FORMAT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPERTYUI_FORMAT_FLAGS").field(&self.0).finish()
    }
}
impl PROPERTYUI_FORMAT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PROPERTYUI_FORMAT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROPERTYUI_FORMAT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROPERTYUI_FORMAT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROPERTYUI_FORMAT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROPERTYUI_FORMAT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPERTYUI_NAME_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PUIFNF_DEFAULT: PROPERTYUI_NAME_FLAGS = PROPERTYUI_NAME_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PUIFNF_MNEMONIC: PROPERTYUI_NAME_FLAGS = PROPERTYUI_NAME_FLAGS(1i32);
impl ::core::marker::Copy for PROPERTYUI_NAME_FLAGS {}
impl ::core::clone::Clone for PROPERTYUI_NAME_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPERTYUI_NAME_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROPERTYUI_NAME_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROPERTYUI_NAME_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPERTYUI_NAME_FLAGS").field(&self.0).finish()
    }
}
impl PROPERTYUI_NAME_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PROPERTYUI_NAME_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROPERTYUI_NAME_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROPERTYUI_NAME_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROPERTYUI_NAME_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROPERTYUI_NAME_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPVAR_CHANGE_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PVCHF_DEFAULT: PROPVAR_CHANGE_FLAGS = PROPVAR_CHANGE_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PVCHF_NOVALUEPROP: PROPVAR_CHANGE_FLAGS = PROPVAR_CHANGE_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PVCHF_ALPHABOOL: PROPVAR_CHANGE_FLAGS = PROPVAR_CHANGE_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PVCHF_NOUSEROVERRIDE: PROPVAR_CHANGE_FLAGS = PROPVAR_CHANGE_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PVCHF_LOCALBOOL: PROPVAR_CHANGE_FLAGS = PROPVAR_CHANGE_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PVCHF_NOHEXSTRING: PROPVAR_CHANGE_FLAGS = PROPVAR_CHANGE_FLAGS(16i32);
impl ::core::marker::Copy for PROPVAR_CHANGE_FLAGS {}
impl ::core::clone::Clone for PROPVAR_CHANGE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPVAR_CHANGE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROPVAR_CHANGE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROPVAR_CHANGE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPVAR_CHANGE_FLAGS").field(&self.0).finish()
    }
}
impl PROPVAR_CHANGE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PROPVAR_CHANGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROPVAR_CHANGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROPVAR_CHANGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROPVAR_CHANGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROPVAR_CHANGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPVAR_COMPARE_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PVCF_DEFAULT: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PVCF_TREATEMPTYASGREATERTHAN: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PVCF_USESTRCMP: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PVCF_USESTRCMPC: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PVCF_USESTRCMPI: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PVCF_USESTRCMPIC: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PVCF_DIGITSASNUMBERS_CASESENSITIVE: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(32i32);
impl ::core::marker::Copy for PROPVAR_COMPARE_FLAGS {}
impl ::core::clone::Clone for PROPVAR_COMPARE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPVAR_COMPARE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROPVAR_COMPARE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROPVAR_COMPARE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPVAR_COMPARE_FLAGS").field(&self.0).finish()
    }
}
impl PROPVAR_COMPARE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PROPVAR_COMPARE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROPVAR_COMPARE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROPVAR_COMPARE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROPVAR_COMPARE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROPVAR_COMPARE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPVAR_COMPARE_UNIT(pub i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PVCU_DEFAULT: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PVCU_SECOND: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PVCU_MINUTE: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PVCU_HOUR: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(3i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PVCU_DAY: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(4i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PVCU_MONTH: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(5i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PVCU_YEAR: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(6i32);
impl ::core::marker::Copy for PROPVAR_COMPARE_UNIT {}
impl ::core::clone::Clone for PROPVAR_COMPARE_UNIT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPVAR_COMPARE_UNIT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROPVAR_COMPARE_UNIT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROPVAR_COMPARE_UNIT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPVAR_COMPARE_UNIT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PSC_STATE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PSC_NORMAL: PSC_STATE = PSC_STATE(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PSC_NOTINSOURCE: PSC_STATE = PSC_STATE(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PSC_DIRTY: PSC_STATE = PSC_STATE(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PSC_READONLY: PSC_STATE = PSC_STATE(3i32);
impl ::core::marker::Copy for PSC_STATE {}
impl ::core::clone::Clone for PSC_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PSC_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PSC_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PSC_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PSC_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PSTIME_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PSTF_UTC: PSTIME_FLAGS = PSTIME_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PSTF_LOCAL: PSTIME_FLAGS = PSTIME_FLAGS(1i32);
impl ::core::marker::Copy for PSTIME_FLAGS {}
impl ::core::clone::Clone for PSTIME_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PSTIME_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PSTIME_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PSTIME_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PSTIME_FLAGS").field(&self.0).finish()
    }
}
impl PSTIME_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PSTIME_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PSTIME_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PSTIME_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PSTIME_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PSTIME_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SYNC_ENGINE_STATE_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const SESF_NONE: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const SESF_SERVICE_QUOTA_NEARING_LIMIT: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const SESF_SERVICE_QUOTA_EXCEEDED_LIMIT: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const SESF_AUTHENTICATION_ERROR: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const SESF_PAUSED_DUE_TO_METERED_NETWORK: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const SESF_PAUSED_DUE_TO_DISK_SPACE_FULL: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const SESF_PAUSED_DUE_TO_CLIENT_POLICY: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(32i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const SESF_PAUSED_DUE_TO_SERVICE_POLICY: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(64i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const SESF_SERVICE_UNAVAILABLE: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(128i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const SESF_PAUSED_DUE_TO_USER_REQUEST: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(256i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const SESF_ALL_FLAGS: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(511i32);
impl ::core::marker::Copy for SYNC_ENGINE_STATE_FLAGS {}
impl ::core::clone::Clone for SYNC_ENGINE_STATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYNC_ENGINE_STATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SYNC_ENGINE_STATE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SYNC_ENGINE_STATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNC_ENGINE_STATE_FLAGS").field(&self.0).finish()
    }
}
impl SYNC_ENGINE_STATE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for SYNC_ENGINE_STATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SYNC_ENGINE_STATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SYNC_ENGINE_STATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SYNC_ENGINE_STATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SYNC_ENGINE_STATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SYNC_TRANSFER_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const STS_NONE: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const STS_NEEDSUPLOAD: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const STS_NEEDSDOWNLOAD: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const STS_TRANSFERRING: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(4i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const STS_PAUSED: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(8i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const STS_HASERROR: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(16i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const STS_FETCHING_METADATA: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(32i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const STS_USER_REQUESTED_REFRESH: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(64i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const STS_HASWARNING: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(128i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const STS_EXCLUDED: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(256i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const STS_INCOMPLETE: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(512i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const STS_PLACEHOLDER_IFEMPTY: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(1024i32);
impl ::core::marker::Copy for SYNC_TRANSFER_STATUS {}
impl ::core::clone::Clone for SYNC_TRANSFER_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYNC_TRANSFER_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SYNC_TRANSFER_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SYNC_TRANSFER_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNC_TRANSFER_STATUS").field(&self.0).finish()
    }
}
impl SYNC_TRANSFER_STATUS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for SYNC_TRANSFER_STATUS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SYNC_TRANSFER_STATUS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SYNC_TRANSFER_STATUS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SYNC_TRANSFER_STATUS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SYNC_TRANSFER_STATUS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _PERSIST_SPROPSTORE_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const FPSPS_DEFAULT: _PERSIST_SPROPSTORE_FLAGS = _PERSIST_SPROPSTORE_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const FPSPS_READONLY: _PERSIST_SPROPSTORE_FLAGS = _PERSIST_SPROPSTORE_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const FPSPS_TREAT_NEW_VALUES_AS_DIRTY: _PERSIST_SPROPSTORE_FLAGS = _PERSIST_SPROPSTORE_FLAGS(2i32);
impl ::core::marker::Copy for _PERSIST_SPROPSTORE_FLAGS {}
impl ::core::clone::Clone for _PERSIST_SPROPSTORE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _PERSIST_SPROPSTORE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for _PERSIST_SPROPSTORE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for _PERSIST_SPROPSTORE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_PERSIST_SPROPSTORE_FLAGS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub struct PROPERTYKEY {
    pub fmtid: ::windows::core::GUID,
    pub pid: u32,
}
impl ::core::marker::Copy for PROPERTYKEY {}
impl ::core::clone::Clone for PROPERTYKEY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROPERTYKEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROPERTYKEY").field("fmtid", &self.fmtid).field("pid", &self.pid).finish()
    }
}
impl ::windows::core::TypeKind for PROPERTYKEY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PROPERTYKEY {
    fn eq(&self, other: &Self) -> bool {
        self.fmtid == other.fmtid && self.pid == other.pid
    }
}
impl ::core::cmp::Eq for PROPERTYKEY {}
impl ::core::default::Default for PROPERTYKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub struct PROPPRG {
    pub flPrg: u16,
    pub flPrgInit: u16,
    pub achTitle: [u8; 30],
    pub achCmdLine: [u8; 128],
    pub achWorkDir: [u8; 64],
    pub wHotKey: u16,
    pub achIconFile: [u8; 80],
    pub wIconIndex: u16,
    pub dwEnhModeFlags: u32,
    pub dwRealModeFlags: u32,
    pub achOtherFile: [u8; 80],
    pub achPIFFile: [u8; 260],
}
impl ::core::marker::Copy for PROPPRG {}
impl ::core::clone::Clone for PROPPRG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for PROPPRG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for PROPPRG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SERIALIZEDPROPSTORAGE(pub u8);
#[cfg(feature = "implement")]
::core::include!("impl.rs");
