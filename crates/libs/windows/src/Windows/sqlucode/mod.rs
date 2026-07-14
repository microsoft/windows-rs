#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLBrowseConnectA(hdbc: super::sqltypes::SQLHDBC, szconnstrin: &[super::sqltypes::SQLCHAR], szconnstrout: Option<&mut [super::sqltypes::SQLCHAR]>, pcbconnstrout: Option<*mut super::sqltypes::SQLSMALLINT>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLBrowseConnectA(hdbc : super::sqltypes::SQLHDBC, szconnstrin : *const super::sqltypes::SQLCHAR, cbconnstrin : super::sqltypes::SQLSMALLINT, szconnstrout : *mut super::sqltypes::SQLCHAR, cbconnstroutmax : super::sqltypes::SQLSMALLINT, pcbconnstrout : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe { SQLBrowseConnectA(hdbc, szconnstrin.as_ptr(), super::sqltypes::SQLSMALLINT(szconnstrin.len().try_into().unwrap()), szconnstrout.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), szconnstrout.as_deref().map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())), pcbconnstrout.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLBrowseConnectW(hdbc: super::sqltypes::SQLHDBC, szconnstrin: &[super::sqltypes::SQLWCHAR], szconnstrout: Option<&mut [super::sqltypes::SQLWCHAR]>, pcchconnstrout: Option<*mut super::sqltypes::SQLSMALLINT>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLBrowseConnectW(hdbc : super::sqltypes::SQLHDBC, szconnstrin : *const super::sqltypes::SQLWCHAR, cchconnstrin : super::sqltypes::SQLSMALLINT, szconnstrout : *mut super::sqltypes::SQLWCHAR, cchconnstroutmax : super::sqltypes::SQLSMALLINT, pcchconnstrout : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe { SQLBrowseConnectW(hdbc, szconnstrin.as_ptr(), super::sqltypes::SQLSMALLINT(szconnstrin.len().try_into().unwrap()), szconnstrout.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), szconnstrout.as_deref().map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())), pcchconnstrout.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLColAttributeA(hstmt: super::sqltypes::SQLHSTMT, icol: super::sqltypes::SQLSMALLINT, ifield: super::sqltypes::SQLSMALLINT, pcharattr: Option<super::sqltypes::SQLPOINTER>, cbcharattrmax: super::sqltypes::SQLSMALLINT, pcbcharattr: Option<*mut super::sqltypes::SQLSMALLINT>, pnumattr: Option<super::sqltypes::SQLPOINTER>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLColAttributeA(hstmt : super::sqltypes::SQLHSTMT, icol : super::sqltypes::SQLSMALLINT, ifield : super::sqltypes::SQLSMALLINT, pcharattr : super::sqltypes::SQLPOINTER, cbcharattrmax : super::sqltypes::SQLSMALLINT, pcbcharattr : *mut super::sqltypes::SQLSMALLINT, pnumattr : super::sqltypes::SQLPOINTER) -> super::sqltypes::SQLRETURN);
    unsafe { SQLColAttributeA(hstmt, icol, ifield, pcharattr.unwrap_or(core::mem::zeroed()) as _, cbcharattrmax, pcbcharattr.unwrap_or(core::mem::zeroed()) as _, pnumattr.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLColAttributeA(hstmt: super::sqltypes::SQLHSTMT, icol: super::sqltypes::SQLSMALLINT, ifield: super::sqltypes::SQLSMALLINT, pcharattr: Option<super::sqltypes::SQLPOINTER>, cbcharattrmax: super::sqltypes::SQLSMALLINT, pcbcharattr: Option<*mut super::sqltypes::SQLSMALLINT>, pnumattr: Option<*mut super::sqltypes::SQLLEN>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLColAttributeA(hstmt : super::sqltypes::SQLHSTMT, icol : super::sqltypes::SQLSMALLINT, ifield : super::sqltypes::SQLSMALLINT, pcharattr : super::sqltypes::SQLPOINTER, cbcharattrmax : super::sqltypes::SQLSMALLINT, pcbcharattr : *mut super::sqltypes::SQLSMALLINT, pnumattr : *mut super::sqltypes::SQLLEN) -> super::sqltypes::SQLRETURN);
    unsafe { SQLColAttributeA(hstmt, icol, ifield, pcharattr.unwrap_or(core::mem::zeroed()) as _, cbcharattrmax, pcbcharattr.unwrap_or(core::mem::zeroed()) as _, pnumattr.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLColAttributeW(hstmt: super::sqltypes::SQLHSTMT, icol: super::sqltypes::SQLUSMALLINT, ifield: super::sqltypes::SQLUSMALLINT, pcharattr: Option<super::sqltypes::SQLPOINTER>, cbdescmax: super::sqltypes::SQLSMALLINT, pcbcharattr: Option<*mut super::sqltypes::SQLSMALLINT>, pnumattr: Option<super::sqltypes::SQLPOINTER>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLColAttributeW(hstmt : super::sqltypes::SQLHSTMT, icol : super::sqltypes::SQLUSMALLINT, ifield : super::sqltypes::SQLUSMALLINT, pcharattr : super::sqltypes::SQLPOINTER, cbdescmax : super::sqltypes::SQLSMALLINT, pcbcharattr : *mut super::sqltypes::SQLSMALLINT, pnumattr : super::sqltypes::SQLPOINTER) -> super::sqltypes::SQLRETURN);
    unsafe { SQLColAttributeW(hstmt, icol, ifield, pcharattr.unwrap_or(core::mem::zeroed()) as _, cbdescmax, pcbcharattr.unwrap_or(core::mem::zeroed()) as _, pnumattr.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLColAttributeW(hstmt: super::sqltypes::SQLHSTMT, icol: super::sqltypes::SQLUSMALLINT, ifield: super::sqltypes::SQLUSMALLINT, pcharattr: Option<super::sqltypes::SQLPOINTER>, cbdescmax: super::sqltypes::SQLSMALLINT, pcbcharattr: Option<*mut super::sqltypes::SQLSMALLINT>, pnumattr: Option<*mut super::sqltypes::SQLLEN>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLColAttributeW(hstmt : super::sqltypes::SQLHSTMT, icol : super::sqltypes::SQLUSMALLINT, ifield : super::sqltypes::SQLUSMALLINT, pcharattr : super::sqltypes::SQLPOINTER, cbdescmax : super::sqltypes::SQLSMALLINT, pcbcharattr : *mut super::sqltypes::SQLSMALLINT, pnumattr : *mut super::sqltypes::SQLLEN) -> super::sqltypes::SQLRETURN);
    unsafe { SQLColAttributeW(hstmt, icol, ifield, pcharattr.unwrap_or(core::mem::zeroed()) as _, cbdescmax, pcbcharattr.unwrap_or(core::mem::zeroed()) as _, pnumattr.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLColAttributesA(hstmt: super::sqltypes::SQLHSTMT, icol: super::sqltypes::SQLUSMALLINT, fdesctype: super::sqltypes::SQLUSMALLINT, rgbdesc: Option<super::sqltypes::SQLPOINTER>, cbdescmax: super::sqltypes::SQLSMALLINT, pcbdesc: Option<*mut super::sqltypes::SQLSMALLINT>, pfdesc: Option<*mut super::sqltypes::SQLINTEGER>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLColAttributesA(hstmt : super::sqltypes::SQLHSTMT, icol : super::sqltypes::SQLUSMALLINT, fdesctype : super::sqltypes::SQLUSMALLINT, rgbdesc : super::sqltypes::SQLPOINTER, cbdescmax : super::sqltypes::SQLSMALLINT, pcbdesc : *mut super::sqltypes::SQLSMALLINT, pfdesc : *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
    unsafe { SQLColAttributesA(hstmt, icol, fdesctype, rgbdesc.unwrap_or(core::mem::zeroed()) as _, cbdescmax, pcbdesc.unwrap_or(core::mem::zeroed()) as _, pfdesc.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLColAttributesA(hstmt: super::sqltypes::SQLHSTMT, icol: super::sqltypes::SQLUSMALLINT, fdesctype: super::sqltypes::SQLUSMALLINT, rgbdesc: Option<super::sqltypes::SQLPOINTER>, cbdescmax: super::sqltypes::SQLSMALLINT, pcbdesc: Option<*mut super::sqltypes::SQLSMALLINT>, pfdesc: Option<*mut super::sqltypes::SQLLEN>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLColAttributesA(hstmt : super::sqltypes::SQLHSTMT, icol : super::sqltypes::SQLUSMALLINT, fdesctype : super::sqltypes::SQLUSMALLINT, rgbdesc : super::sqltypes::SQLPOINTER, cbdescmax : super::sqltypes::SQLSMALLINT, pcbdesc : *mut super::sqltypes::SQLSMALLINT, pfdesc : *mut super::sqltypes::SQLLEN) -> super::sqltypes::SQLRETURN);
    unsafe { SQLColAttributesA(hstmt, icol, fdesctype, rgbdesc.unwrap_or(core::mem::zeroed()) as _, cbdescmax, pcbdesc.unwrap_or(core::mem::zeroed()) as _, pfdesc.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLColAttributesW(hstmt: super::sqltypes::SQLHSTMT, icol: super::sqltypes::SQLUSMALLINT, fdesctype: super::sqltypes::SQLUSMALLINT, rgbdesc: Option<super::sqltypes::SQLPOINTER>, cbdescmax: super::sqltypes::SQLSMALLINT, pcbdesc: Option<*mut super::sqltypes::SQLSMALLINT>, pfdesc: Option<*mut super::sqltypes::SQLINTEGER>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLColAttributesW(hstmt : super::sqltypes::SQLHSTMT, icol : super::sqltypes::SQLUSMALLINT, fdesctype : super::sqltypes::SQLUSMALLINT, rgbdesc : super::sqltypes::SQLPOINTER, cbdescmax : super::sqltypes::SQLSMALLINT, pcbdesc : *mut super::sqltypes::SQLSMALLINT, pfdesc : *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
    unsafe { SQLColAttributesW(hstmt, icol, fdesctype, rgbdesc.unwrap_or(core::mem::zeroed()) as _, cbdescmax, pcbdesc.unwrap_or(core::mem::zeroed()) as _, pfdesc.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLColAttributesW(hstmt: super::sqltypes::SQLHSTMT, icol: super::sqltypes::SQLUSMALLINT, fdesctype: super::sqltypes::SQLUSMALLINT, rgbdesc: Option<super::sqltypes::SQLPOINTER>, cbdescmax: super::sqltypes::SQLSMALLINT, pcbdesc: Option<*mut super::sqltypes::SQLSMALLINT>, pfdesc: Option<*mut super::sqltypes::SQLLEN>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLColAttributesW(hstmt : super::sqltypes::SQLHSTMT, icol : super::sqltypes::SQLUSMALLINT, fdesctype : super::sqltypes::SQLUSMALLINT, rgbdesc : super::sqltypes::SQLPOINTER, cbdescmax : super::sqltypes::SQLSMALLINT, pcbdesc : *mut super::sqltypes::SQLSMALLINT, pfdesc : *mut super::sqltypes::SQLLEN) -> super::sqltypes::SQLRETURN);
    unsafe { SQLColAttributesW(hstmt, icol, fdesctype, rgbdesc.unwrap_or(core::mem::zeroed()) as _, cbdescmax, pcbdesc.unwrap_or(core::mem::zeroed()) as _, pfdesc.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLColumnPrivilegesA(hstmt: super::sqltypes::SQLHSTMT, szcatalogname: Option<&[super::sqltypes::SQLCHAR]>, szschemaname: Option<&[super::sqltypes::SQLCHAR]>, sztablename: Option<&[super::sqltypes::SQLCHAR]>, szcolumnname: Option<&[super::sqltypes::SQLCHAR]>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLColumnPrivilegesA(hstmt : super::sqltypes::SQLHSTMT, szcatalogname : *const super::sqltypes::SQLCHAR, cbcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLCHAR, cbschemaname : super::sqltypes::SQLSMALLINT, sztablename : *const super::sqltypes::SQLCHAR, cbtablename : super::sqltypes::SQLSMALLINT, szcolumnname : *const super::sqltypes::SQLCHAR, cbcolumnname : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe {
        SQLColumnPrivilegesA(
            hstmt,
            szcatalogname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szcatalogname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szschemaname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szschemaname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            sztablename.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            sztablename.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szcolumnname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szcolumnname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
        )
    }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLColumnPrivilegesW(hstmt: super::sqltypes::SQLHSTMT, szcatalogname: Option<&[super::sqltypes::SQLWCHAR]>, szschemaname: Option<&[super::sqltypes::SQLWCHAR]>, sztablename: Option<&[super::sqltypes::SQLWCHAR]>, szcolumnname: Option<&[super::sqltypes::SQLWCHAR]>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLColumnPrivilegesW(hstmt : super::sqltypes::SQLHSTMT, szcatalogname : *const super::sqltypes::SQLWCHAR, cchcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLWCHAR, cchschemaname : super::sqltypes::SQLSMALLINT, sztablename : *const super::sqltypes::SQLWCHAR, cchtablename : super::sqltypes::SQLSMALLINT, szcolumnname : *const super::sqltypes::SQLWCHAR, cchcolumnname : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe {
        SQLColumnPrivilegesW(
            hstmt,
            szcatalogname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szcatalogname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szschemaname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szschemaname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            sztablename.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            sztablename.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szcolumnname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szcolumnname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
        )
    }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLColumnsA(hstmt: super::sqltypes::SQLHSTMT, szcatalogname: Option<&[super::sqltypes::SQLCHAR]>, szschemaname: Option<&[super::sqltypes::SQLCHAR]>, sztablename: Option<&[super::sqltypes::SQLCHAR]>, szcolumnname: Option<&[super::sqltypes::SQLCHAR]>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLColumnsA(hstmt : super::sqltypes::SQLHSTMT, szcatalogname : *const super::sqltypes::SQLCHAR, cbcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLCHAR, cbschemaname : super::sqltypes::SQLSMALLINT, sztablename : *const super::sqltypes::SQLCHAR, cbtablename : super::sqltypes::SQLSMALLINT, szcolumnname : *const super::sqltypes::SQLCHAR, cbcolumnname : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe {
        SQLColumnsA(
            hstmt,
            szcatalogname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szcatalogname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szschemaname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szschemaname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            sztablename.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            sztablename.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szcolumnname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szcolumnname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
        )
    }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLColumnsW(hstmt: super::sqltypes::SQLHSTMT, szcatalogname: Option<&[super::sqltypes::SQLWCHAR]>, szschemaname: Option<&[super::sqltypes::SQLWCHAR]>, sztablename: Option<&[super::sqltypes::SQLWCHAR]>, szcolumnname: Option<&[super::sqltypes::SQLWCHAR]>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLColumnsW(hstmt : super::sqltypes::SQLHSTMT, szcatalogname : *const super::sqltypes::SQLWCHAR, cchcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLWCHAR, cchschemaname : super::sqltypes::SQLSMALLINT, sztablename : *const super::sqltypes::SQLWCHAR, cchtablename : super::sqltypes::SQLSMALLINT, szcolumnname : *const super::sqltypes::SQLWCHAR, cchcolumnname : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe {
        SQLColumnsW(
            hstmt,
            szcatalogname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szcatalogname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szschemaname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szschemaname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            sztablename.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            sztablename.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szcolumnname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szcolumnname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
        )
    }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLConnectA(hdbc: super::sqltypes::SQLHDBC, szdsn: &[super::sqltypes::SQLCHAR], szuid: &[super::sqltypes::SQLCHAR], szauthstr: &[super::sqltypes::SQLCHAR]) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLConnectA(hdbc : super::sqltypes::SQLHDBC, szdsn : *const super::sqltypes::SQLCHAR, cbdsn : super::sqltypes::SQLSMALLINT, szuid : *const super::sqltypes::SQLCHAR, cbuid : super::sqltypes::SQLSMALLINT, szauthstr : *const super::sqltypes::SQLCHAR, cbauthstr : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe { SQLConnectA(hdbc, szdsn.as_ptr(), super::sqltypes::SQLSMALLINT(szdsn.len().try_into().unwrap()), szuid.as_ptr(), super::sqltypes::SQLSMALLINT(szuid.len().try_into().unwrap()), szauthstr.as_ptr(), super::sqltypes::SQLSMALLINT(szauthstr.len().try_into().unwrap())) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLConnectW(hdbc: super::sqltypes::SQLHDBC, szdsn: &[super::sqltypes::SQLWCHAR], szuid: &[super::sqltypes::SQLWCHAR], szauthstr: &[super::sqltypes::SQLWCHAR]) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLConnectW(hdbc : super::sqltypes::SQLHDBC, szdsn : *const super::sqltypes::SQLWCHAR, cchdsn : super::sqltypes::SQLSMALLINT, szuid : *const super::sqltypes::SQLWCHAR, cchuid : super::sqltypes::SQLSMALLINT, szauthstr : *const super::sqltypes::SQLWCHAR, cchauthstr : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe { SQLConnectW(hdbc, szdsn.as_ptr(), super::sqltypes::SQLSMALLINT(szdsn.len().try_into().unwrap()), szuid.as_ptr(), super::sqltypes::SQLSMALLINT(szuid.len().try_into().unwrap()), szauthstr.as_ptr(), super::sqltypes::SQLSMALLINT(szauthstr.len().try_into().unwrap())) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLDataSourcesA(henv: super::sqltypes::SQLHENV, fdirection: super::sqltypes::SQLUSMALLINT, szdsn: Option<&mut [super::sqltypes::SQLCHAR]>, pcbdsn: *mut super::sqltypes::SQLSMALLINT, szdescription: Option<&mut [super::sqltypes::SQLCHAR]>, pcbdescription: *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLDataSourcesA(henv : super::sqltypes::SQLHENV, fdirection : super::sqltypes::SQLUSMALLINT, szdsn : *mut super::sqltypes::SQLCHAR, cbdsnmax : super::sqltypes::SQLSMALLINT, pcbdsn : *mut super::sqltypes::SQLSMALLINT, szdescription : *mut super::sqltypes::SQLCHAR, cbdescriptionmax : super::sqltypes::SQLSMALLINT, pcbdescription : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe {
        SQLDataSourcesA(
            henv,
            fdirection,
            szdsn.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()),
            szdsn.as_deref().map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            pcbdsn as _,
            szdescription.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()),
            szdescription.as_deref().map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            pcbdescription as _,
        )
    }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLDataSourcesW(henv: super::sqltypes::SQLHENV, fdirection: super::sqltypes::SQLUSMALLINT, szdsn: Option<&mut [super::sqltypes::SQLWCHAR]>, pcchdsn: Option<*mut super::sqltypes::SQLSMALLINT>, wszdescription: Option<&mut [super::sqltypes::SQLWCHAR]>, pcchdescription: Option<*mut super::sqltypes::SQLSMALLINT>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLDataSourcesW(henv : super::sqltypes::SQLHENV, fdirection : super::sqltypes::SQLUSMALLINT, szdsn : *mut super::sqltypes::SQLWCHAR, cchdsnmax : super::sqltypes::SQLSMALLINT, pcchdsn : *mut super::sqltypes::SQLSMALLINT, wszdescription : *mut super::sqltypes::SQLWCHAR, cchdescriptionmax : super::sqltypes::SQLSMALLINT, pcchdescription : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe {
        SQLDataSourcesW(
            henv,
            fdirection,
            szdsn.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()),
            szdsn.as_deref().map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            pcchdsn.unwrap_or(core::mem::zeroed()) as _,
            wszdescription.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()),
            wszdescription.as_deref().map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            pcchdescription.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLDescribeColA(hstmt: super::sqltypes::SQLHSTMT, icol: super::sqltypes::SQLUSMALLINT, szcolname: Option<&mut [super::sqltypes::SQLCHAR]>, pcbcolname: Option<*mut super::sqltypes::SQLSMALLINT>, pfsqltype: Option<*mut super::sqltypes::SQLSMALLINT>, pcbcoldef: Option<*mut super::sqltypes::SQLUINTEGER>, pibscale: Option<*mut super::sqltypes::SQLSMALLINT>, pfnullable: Option<*mut super::sqltypes::SQLSMALLINT>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLDescribeColA(hstmt : super::sqltypes::SQLHSTMT, icol : super::sqltypes::SQLUSMALLINT, szcolname : *mut super::sqltypes::SQLCHAR, cbcolnamemax : super::sqltypes::SQLSMALLINT, pcbcolname : *mut super::sqltypes::SQLSMALLINT, pfsqltype : *mut super::sqltypes::SQLSMALLINT, pcbcoldef : *mut super::sqltypes::SQLUINTEGER, pibscale : *mut super::sqltypes::SQLSMALLINT, pfnullable : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe { SQLDescribeColA(hstmt, icol, szcolname.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), szcolname.as_deref().map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())), pcbcolname.unwrap_or(core::mem::zeroed()) as _, pfsqltype.unwrap_or(core::mem::zeroed()) as _, pcbcoldef.unwrap_or(core::mem::zeroed()) as _, pibscale.unwrap_or(core::mem::zeroed()) as _, pfnullable.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLDescribeColA(hstmt: super::sqltypes::SQLHSTMT, icol: super::sqltypes::SQLUSMALLINT, szcolname: Option<&mut [super::sqltypes::SQLCHAR]>, pcbcolname: Option<*mut super::sqltypes::SQLSMALLINT>, pfsqltype: Option<*mut super::sqltypes::SQLSMALLINT>, pcbcoldef: Option<*mut super::sqltypes::SQLULEN>, pibscale: Option<*mut super::sqltypes::SQLSMALLINT>, pfnullable: Option<*mut super::sqltypes::SQLSMALLINT>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLDescribeColA(hstmt : super::sqltypes::SQLHSTMT, icol : super::sqltypes::SQLUSMALLINT, szcolname : *mut super::sqltypes::SQLCHAR, cbcolnamemax : super::sqltypes::SQLSMALLINT, pcbcolname : *mut super::sqltypes::SQLSMALLINT, pfsqltype : *mut super::sqltypes::SQLSMALLINT, pcbcoldef : *mut super::sqltypes::SQLULEN, pibscale : *mut super::sqltypes::SQLSMALLINT, pfnullable : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe { SQLDescribeColA(hstmt, icol, szcolname.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), szcolname.as_deref().map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())), pcbcolname.unwrap_or(core::mem::zeroed()) as _, pfsqltype.unwrap_or(core::mem::zeroed()) as _, pcbcoldef.unwrap_or(core::mem::zeroed()) as _, pibscale.unwrap_or(core::mem::zeroed()) as _, pfnullable.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLDescribeColW(hstmt: super::sqltypes::SQLHSTMT, icol: super::sqltypes::SQLUSMALLINT, szcolname: Option<&mut [super::sqltypes::SQLWCHAR]>, pcchcolname: Option<*mut super::sqltypes::SQLSMALLINT>, pfsqltype: Option<*mut super::sqltypes::SQLSMALLINT>, pcbcoldef: Option<*mut super::sqltypes::SQLUINTEGER>, pibscale: Option<*mut super::sqltypes::SQLSMALLINT>, pfnullable: Option<*mut super::sqltypes::SQLSMALLINT>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLDescribeColW(hstmt : super::sqltypes::SQLHSTMT, icol : super::sqltypes::SQLUSMALLINT, szcolname : *mut super::sqltypes::SQLWCHAR, cchcolnamemax : super::sqltypes::SQLSMALLINT, pcchcolname : *mut super::sqltypes::SQLSMALLINT, pfsqltype : *mut super::sqltypes::SQLSMALLINT, pcbcoldef : *mut super::sqltypes::SQLUINTEGER, pibscale : *mut super::sqltypes::SQLSMALLINT, pfnullable : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe { SQLDescribeColW(hstmt, icol, szcolname.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), szcolname.as_deref().map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())), pcchcolname.unwrap_or(core::mem::zeroed()) as _, pfsqltype.unwrap_or(core::mem::zeroed()) as _, pcbcoldef.unwrap_or(core::mem::zeroed()) as _, pibscale.unwrap_or(core::mem::zeroed()) as _, pfnullable.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLDescribeColW(hstmt: super::sqltypes::SQLHSTMT, icol: super::sqltypes::SQLUSMALLINT, szcolname: Option<&mut [super::sqltypes::SQLWCHAR]>, pcchcolname: Option<*mut super::sqltypes::SQLSMALLINT>, pfsqltype: Option<*mut super::sqltypes::SQLSMALLINT>, pcbcoldef: Option<*mut super::sqltypes::SQLULEN>, pibscale: Option<*mut super::sqltypes::SQLSMALLINT>, pfnullable: Option<*mut super::sqltypes::SQLSMALLINT>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLDescribeColW(hstmt : super::sqltypes::SQLHSTMT, icol : super::sqltypes::SQLUSMALLINT, szcolname : *mut super::sqltypes::SQLWCHAR, cchcolnamemax : super::sqltypes::SQLSMALLINT, pcchcolname : *mut super::sqltypes::SQLSMALLINT, pfsqltype : *mut super::sqltypes::SQLSMALLINT, pcbcoldef : *mut super::sqltypes::SQLULEN, pibscale : *mut super::sqltypes::SQLSMALLINT, pfnullable : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe { SQLDescribeColW(hstmt, icol, szcolname.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), szcolname.as_deref().map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())), pcchcolname.unwrap_or(core::mem::zeroed()) as _, pfsqltype.unwrap_or(core::mem::zeroed()) as _, pcbcoldef.unwrap_or(core::mem::zeroed()) as _, pibscale.unwrap_or(core::mem::zeroed()) as _, pfnullable.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "sqltypes", feature = "windef"))]
#[inline]
pub unsafe fn SQLDriverConnectA(hdbc: super::sqltypes::SQLHDBC, hwnd: super::sqltypes::SQLHWND, szconnstrin: &[super::sqltypes::SQLCHAR], szconnstrout: Option<&mut [super::sqltypes::SQLCHAR]>, pcbconnstrout: Option<*mut super::sqltypes::SQLSMALLINT>, fdrivercompletion: super::sqltypes::SQLUSMALLINT) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLDriverConnectA(hdbc : super::sqltypes::SQLHDBC, hwnd : super::sqltypes::SQLHWND, szconnstrin : *const super::sqltypes::SQLCHAR, cbconnstrin : super::sqltypes::SQLSMALLINT, szconnstrout : *mut super::sqltypes::SQLCHAR, cbconnstroutmax : super::sqltypes::SQLSMALLINT, pcbconnstrout : *mut super::sqltypes::SQLSMALLINT, fdrivercompletion : super::sqltypes::SQLUSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe { SQLDriverConnectA(hdbc, hwnd, szconnstrin.as_ptr(), super::sqltypes::SQLSMALLINT(szconnstrin.len().try_into().unwrap()), szconnstrout.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), szconnstrout.as_deref().map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())), pcbconnstrout.unwrap_or(core::mem::zeroed()) as _, fdrivercompletion) }
}
#[cfg(all(feature = "sqltypes", feature = "windef"))]
#[inline]
pub unsafe fn SQLDriverConnectW(hdbc: super::sqltypes::SQLHDBC, hwnd: super::sqltypes::SQLHWND, szconnstrin: &[super::sqltypes::SQLWCHAR], szconnstrout: Option<&mut [super::sqltypes::SQLWCHAR]>, pcchconnstrout: Option<*mut super::sqltypes::SQLSMALLINT>, fdrivercompletion: super::sqltypes::SQLUSMALLINT) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLDriverConnectW(hdbc : super::sqltypes::SQLHDBC, hwnd : super::sqltypes::SQLHWND, szconnstrin : *const super::sqltypes::SQLWCHAR, cchconnstrin : super::sqltypes::SQLSMALLINT, szconnstrout : *mut super::sqltypes::SQLWCHAR, cchconnstroutmax : super::sqltypes::SQLSMALLINT, pcchconnstrout : *mut super::sqltypes::SQLSMALLINT, fdrivercompletion : super::sqltypes::SQLUSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe { SQLDriverConnectW(hdbc, hwnd, szconnstrin.as_ptr(), super::sqltypes::SQLSMALLINT(szconnstrin.len().try_into().unwrap()), szconnstrout.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), szconnstrout.as_deref().map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())), pcchconnstrout.unwrap_or(core::mem::zeroed()) as _, fdrivercompletion) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLDriversA(henv: super::sqltypes::SQLHENV, fdirection: super::sqltypes::SQLUSMALLINT, szdriverdesc: Option<&mut [super::sqltypes::SQLCHAR]>, pcbdriverdesc: Option<*mut super::sqltypes::SQLSMALLINT>, szdriverattributes: Option<&mut [super::sqltypes::SQLCHAR]>, pcbdrvrattr: Option<*mut super::sqltypes::SQLSMALLINT>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLDriversA(henv : super::sqltypes::SQLHENV, fdirection : super::sqltypes::SQLUSMALLINT, szdriverdesc : *mut super::sqltypes::SQLCHAR, cbdriverdescmax : super::sqltypes::SQLSMALLINT, pcbdriverdesc : *mut super::sqltypes::SQLSMALLINT, szdriverattributes : *mut super::sqltypes::SQLCHAR, cbdrvrattrmax : super::sqltypes::SQLSMALLINT, pcbdrvrattr : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe {
        SQLDriversA(
            henv,
            fdirection,
            szdriverdesc.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()),
            szdriverdesc.as_deref().map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            pcbdriverdesc.unwrap_or(core::mem::zeroed()) as _,
            szdriverattributes.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()),
            szdriverattributes.as_deref().map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            pcbdrvrattr.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLDriversW(henv: super::sqltypes::SQLHENV, fdirection: super::sqltypes::SQLUSMALLINT, szdriverdesc: Option<&mut [super::sqltypes::SQLWCHAR]>, pcchdriverdesc: Option<*mut super::sqltypes::SQLSMALLINT>, szdriverattributes: Option<&mut [super::sqltypes::SQLWCHAR]>, pcchdrvrattr: Option<*mut super::sqltypes::SQLSMALLINT>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLDriversW(henv : super::sqltypes::SQLHENV, fdirection : super::sqltypes::SQLUSMALLINT, szdriverdesc : *mut super::sqltypes::SQLWCHAR, cchdriverdescmax : super::sqltypes::SQLSMALLINT, pcchdriverdesc : *mut super::sqltypes::SQLSMALLINT, szdriverattributes : *mut super::sqltypes::SQLWCHAR, cchdrvrattrmax : super::sqltypes::SQLSMALLINT, pcchdrvrattr : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe {
        SQLDriversW(
            henv,
            fdirection,
            szdriverdesc.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()),
            szdriverdesc.as_deref().map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            pcchdriverdesc.unwrap_or(core::mem::zeroed()) as _,
            szdriverattributes.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()),
            szdriverattributes.as_deref().map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            pcchdrvrattr.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLErrorA(henv: super::sqltypes::SQLHENV, hdbc: super::sqltypes::SQLHDBC, hstmt: super::sqltypes::SQLHSTMT, szsqlstate: *mut super::sqltypes::SQLCHAR, pfnativeerror: Option<*mut super::sqltypes::SQLINTEGER>, szerrormsg: Option<&mut [super::sqltypes::SQLCHAR]>, pcberrormsg: Option<*mut super::sqltypes::SQLSMALLINT>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLErrorA(henv : super::sqltypes::SQLHENV, hdbc : super::sqltypes::SQLHDBC, hstmt : super::sqltypes::SQLHSTMT, szsqlstate : *mut super::sqltypes::SQLCHAR, pfnativeerror : *mut super::sqltypes::SQLINTEGER, szerrormsg : *mut super::sqltypes::SQLCHAR, cberrormsgmax : super::sqltypes::SQLSMALLINT, pcberrormsg : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe { SQLErrorA(henv, hdbc, hstmt, szsqlstate as _, pfnativeerror.unwrap_or(core::mem::zeroed()) as _, szerrormsg.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), szerrormsg.as_deref().map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())), pcberrormsg.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLErrorW(henv: super::sqltypes::SQLHENV, hdbc: super::sqltypes::SQLHDBC, hstmt: super::sqltypes::SQLHSTMT, wszsqlstate: &mut [super::sqltypes::SQLWCHAR; 6], pfnativeerror: Option<*mut super::sqltypes::SQLINTEGER>, wszerrormsg: Option<&mut [super::sqltypes::SQLWCHAR]>, pccherrormsg: Option<*mut super::sqltypes::SQLSMALLINT>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLErrorW(henv : super::sqltypes::SQLHENV, hdbc : super::sqltypes::SQLHDBC, hstmt : super::sqltypes::SQLHSTMT, wszsqlstate : *mut super::sqltypes::SQLWCHAR, pfnativeerror : *mut super::sqltypes::SQLINTEGER, wszerrormsg : *mut super::sqltypes::SQLWCHAR, ccherrormsgmax : super::sqltypes::SQLSMALLINT, pccherrormsg : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe { SQLErrorW(henv, hdbc, hstmt, wszsqlstate.as_mut_ptr(), pfnativeerror.unwrap_or(core::mem::zeroed()) as _, wszerrormsg.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), wszerrormsg.as_deref().map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())), pccherrormsg.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLExecDirectA(hstmt: super::sqltypes::SQLHSTMT, szsqlstr: Option<&[super::sqltypes::SQLCHAR]>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLExecDirectA(hstmt : super::sqltypes::SQLHSTMT, szsqlstr : *const super::sqltypes::SQLCHAR, cbsqlstr : super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
    unsafe { SQLExecDirectA(hstmt, szsqlstr.map_or(core::ptr::null(), |slice| slice.as_ptr()), szsqlstr.map_or(super::sqltypes::SQLINTEGER(0), |slice| super::sqltypes::SQLINTEGER(slice.len().try_into().unwrap()))) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLExecDirectW(hstmt: super::sqltypes::SQLHSTMT, szsqlstr: Option<&[super::sqltypes::SQLWCHAR]>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLExecDirectW(hstmt : super::sqltypes::SQLHSTMT, szsqlstr : *const super::sqltypes::SQLWCHAR, textlength : super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
    unsafe { SQLExecDirectW(hstmt, szsqlstr.map_or(core::ptr::null(), |slice| slice.as_ptr()), szsqlstr.map_or(super::sqltypes::SQLINTEGER(0), |slice| super::sqltypes::SQLINTEGER(slice.len().try_into().unwrap()))) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLForeignKeysA(hstmt: super::sqltypes::SQLHSTMT, szpkcatalogname: Option<&[super::sqltypes::SQLCHAR]>, szpkschemaname: Option<&[super::sqltypes::SQLCHAR]>, szpktablename: Option<&[super::sqltypes::SQLCHAR]>, szfkcatalogname: Option<&[super::sqltypes::SQLCHAR]>, szfkschemaname: Option<&[super::sqltypes::SQLCHAR]>, szfktablename: Option<&[super::sqltypes::SQLCHAR]>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLForeignKeysA(hstmt : super::sqltypes::SQLHSTMT, szpkcatalogname : *const super::sqltypes::SQLCHAR, cbpkcatalogname : super::sqltypes::SQLSMALLINT, szpkschemaname : *const super::sqltypes::SQLCHAR, cbpkschemaname : super::sqltypes::SQLSMALLINT, szpktablename : *const super::sqltypes::SQLCHAR, cbpktablename : super::sqltypes::SQLSMALLINT, szfkcatalogname : *const super::sqltypes::SQLCHAR, cbfkcatalogname : super::sqltypes::SQLSMALLINT, szfkschemaname : *const super::sqltypes::SQLCHAR, cbfkschemaname : super::sqltypes::SQLSMALLINT, szfktablename : *const super::sqltypes::SQLCHAR, cbfktablename : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe {
        SQLForeignKeysA(
            hstmt,
            szpkcatalogname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szpkcatalogname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szpkschemaname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szpkschemaname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szpktablename.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szpktablename.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szfkcatalogname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szfkcatalogname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szfkschemaname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szfkschemaname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szfktablename.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szfktablename.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
        )
    }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLForeignKeysW(hstmt: super::sqltypes::SQLHSTMT, szpkcatalogname: Option<&[super::sqltypes::SQLWCHAR]>, szpkschemaname: Option<&[super::sqltypes::SQLWCHAR]>, szpktablename: Option<&[super::sqltypes::SQLWCHAR]>, szfkcatalogname: Option<&[super::sqltypes::SQLWCHAR]>, szfkschemaname: Option<&[super::sqltypes::SQLWCHAR]>, szfktablename: Option<&[super::sqltypes::SQLWCHAR]>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLForeignKeysW(hstmt : super::sqltypes::SQLHSTMT, szpkcatalogname : *const super::sqltypes::SQLWCHAR, cchpkcatalogname : super::sqltypes::SQLSMALLINT, szpkschemaname : *const super::sqltypes::SQLWCHAR, cchpkschemaname : super::sqltypes::SQLSMALLINT, szpktablename : *const super::sqltypes::SQLWCHAR, cchpktablename : super::sqltypes::SQLSMALLINT, szfkcatalogname : *const super::sqltypes::SQLWCHAR, cchfkcatalogname : super::sqltypes::SQLSMALLINT, szfkschemaname : *const super::sqltypes::SQLWCHAR, cchfkschemaname : super::sqltypes::SQLSMALLINT, szfktablename : *const super::sqltypes::SQLWCHAR, cchfktablename : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe {
        SQLForeignKeysW(
            hstmt,
            szpkcatalogname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szpkcatalogname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szpkschemaname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szpkschemaname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szpktablename.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szpktablename.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szfkcatalogname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szfkcatalogname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szfkschemaname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szfkschemaname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szfktablename.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szfktablename.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
        )
    }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetConnectAttrA(hdbc: super::sqltypes::SQLHDBC, fattribute: super::sqltypes::SQLINTEGER, rgbvalue: Option<super::sqltypes::SQLPOINTER>, cbvaluemax: super::sqltypes::SQLINTEGER, pcbvalue: Option<*mut super::sqltypes::SQLINTEGER>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetConnectAttrA(hdbc : super::sqltypes::SQLHDBC, fattribute : super::sqltypes::SQLINTEGER, rgbvalue : super::sqltypes::SQLPOINTER, cbvaluemax : super::sqltypes::SQLINTEGER, pcbvalue : *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
    unsafe { SQLGetConnectAttrA(hdbc, fattribute, rgbvalue.unwrap_or(core::mem::zeroed()) as _, cbvaluemax, pcbvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetConnectAttrW(hdbc: super::sqltypes::SQLHDBC, fattribute: super::sqltypes::SQLINTEGER, rgbvalue: Option<super::sqltypes::SQLPOINTER>, cbvaluemax: super::sqltypes::SQLINTEGER, pcbvalue: Option<*mut super::sqltypes::SQLINTEGER>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetConnectAttrW(hdbc : super::sqltypes::SQLHDBC, fattribute : super::sqltypes::SQLINTEGER, rgbvalue : super::sqltypes::SQLPOINTER, cbvaluemax : super::sqltypes::SQLINTEGER, pcbvalue : *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
    unsafe { SQLGetConnectAttrW(hdbc, fattribute, rgbvalue.unwrap_or(core::mem::zeroed()) as _, cbvaluemax, pcbvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetConnectOptionA(hdbc: super::sqltypes::SQLHDBC, foption: super::sqltypes::SQLUSMALLINT, pvparam: super::sqltypes::SQLPOINTER) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetConnectOptionA(hdbc : super::sqltypes::SQLHDBC, foption : super::sqltypes::SQLUSMALLINT, pvparam : super::sqltypes::SQLPOINTER) -> super::sqltypes::SQLRETURN);
    unsafe { SQLGetConnectOptionA(hdbc, foption, pvparam) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetConnectOptionW(hdbc: super::sqltypes::SQLHDBC, foption: super::sqltypes::SQLUSMALLINT, pvparam: super::sqltypes::SQLPOINTER) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetConnectOptionW(hdbc : super::sqltypes::SQLHDBC, foption : super::sqltypes::SQLUSMALLINT, pvparam : super::sqltypes::SQLPOINTER) -> super::sqltypes::SQLRETURN);
    unsafe { SQLGetConnectOptionW(hdbc, foption, pvparam) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetCursorNameA(hstmt: super::sqltypes::SQLHSTMT, szcursor: Option<&mut [super::sqltypes::SQLCHAR]>, pcbcursor: Option<*mut super::sqltypes::SQLSMALLINT>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetCursorNameA(hstmt : super::sqltypes::SQLHSTMT, szcursor : *mut super::sqltypes::SQLCHAR, cbcursormax : super::sqltypes::SQLSMALLINT, pcbcursor : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe { SQLGetCursorNameA(hstmt, szcursor.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), szcursor.as_deref().map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())), pcbcursor.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetCursorNameW(hstmt: super::sqltypes::SQLHSTMT, szcursor: Option<&mut [super::sqltypes::SQLWCHAR]>, pcchcursor: Option<*mut super::sqltypes::SQLSMALLINT>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetCursorNameW(hstmt : super::sqltypes::SQLHSTMT, szcursor : *mut super::sqltypes::SQLWCHAR, cchcursormax : super::sqltypes::SQLSMALLINT, pcchcursor : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe { SQLGetCursorNameW(hstmt, szcursor.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), szcursor.as_deref().map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())), pcchcursor.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetDescFieldA(hdesc: super::sqltypes::SQLHDESC, irecord: super::sqltypes::SQLSMALLINT, ifield: super::sqltypes::SQLSMALLINT, rgbvalue: Option<super::sqltypes::SQLPOINTER>, cbbufferlength: super::sqltypes::SQLINTEGER, stringlength: Option<*mut super::sqltypes::SQLINTEGER>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetDescFieldA(hdesc : super::sqltypes::SQLHDESC, irecord : super::sqltypes::SQLSMALLINT, ifield : super::sqltypes::SQLSMALLINT, rgbvalue : super::sqltypes::SQLPOINTER, cbbufferlength : super::sqltypes::SQLINTEGER, stringlength : *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
    unsafe { SQLGetDescFieldA(hdesc, irecord, ifield, rgbvalue.unwrap_or(core::mem::zeroed()) as _, cbbufferlength, stringlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetDescFieldW(hdesc: super::sqltypes::SQLHDESC, irecord: super::sqltypes::SQLSMALLINT, ifield: super::sqltypes::SQLSMALLINT, rgbvalue: Option<super::sqltypes::SQLPOINTER>, cbbufferlength: super::sqltypes::SQLINTEGER, stringlength: Option<*mut super::sqltypes::SQLINTEGER>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetDescFieldW(hdesc : super::sqltypes::SQLHDESC, irecord : super::sqltypes::SQLSMALLINT, ifield : super::sqltypes::SQLSMALLINT, rgbvalue : super::sqltypes::SQLPOINTER, cbbufferlength : super::sqltypes::SQLINTEGER, stringlength : *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
    unsafe { SQLGetDescFieldW(hdesc, irecord, ifield, rgbvalue.unwrap_or(core::mem::zeroed()) as _, cbbufferlength, stringlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetDescRecA(hdesc: super::sqltypes::SQLHDESC, irecord: super::sqltypes::SQLSMALLINT, szname: Option<&mut [super::sqltypes::SQLCHAR]>, pcbname: Option<*mut super::sqltypes::SQLSMALLINT>, pftype: Option<*mut super::sqltypes::SQLSMALLINT>, pfsubtype: Option<*mut super::sqltypes::SQLSMALLINT>, plength: Option<*mut super::sqltypes::SQLINTEGER>, pprecision: Option<*mut super::sqltypes::SQLSMALLINT>, pscale: Option<*mut super::sqltypes::SQLSMALLINT>, pnullable: Option<*mut super::sqltypes::SQLSMALLINT>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetDescRecA(hdesc : super::sqltypes::SQLHDESC, irecord : super::sqltypes::SQLSMALLINT, szname : *mut super::sqltypes::SQLCHAR, cbnamemax : super::sqltypes::SQLSMALLINT, pcbname : *mut super::sqltypes::SQLSMALLINT, pftype : *mut super::sqltypes::SQLSMALLINT, pfsubtype : *mut super::sqltypes::SQLSMALLINT, plength : *mut super::sqltypes::SQLINTEGER, pprecision : *mut super::sqltypes::SQLSMALLINT, pscale : *mut super::sqltypes::SQLSMALLINT, pnullable : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe {
        SQLGetDescRecA(
            hdesc,
            irecord,
            szname.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()),
            szname.as_deref().map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            pcbname.unwrap_or(core::mem::zeroed()) as _,
            pftype.unwrap_or(core::mem::zeroed()) as _,
            pfsubtype.unwrap_or(core::mem::zeroed()) as _,
            plength.unwrap_or(core::mem::zeroed()) as _,
            pprecision.unwrap_or(core::mem::zeroed()) as _,
            pscale.unwrap_or(core::mem::zeroed()) as _,
            pnullable.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetDescRecA(hdesc: super::sqltypes::SQLHDESC, irecord: super::sqltypes::SQLSMALLINT, szname: Option<&mut [super::sqltypes::SQLCHAR]>, pcbname: Option<*mut super::sqltypes::SQLSMALLINT>, pftype: Option<*mut super::sqltypes::SQLSMALLINT>, pfsubtype: Option<*mut super::sqltypes::SQLSMALLINT>, plength: Option<*mut super::sqltypes::SQLLEN>, pprecision: Option<*mut super::sqltypes::SQLSMALLINT>, pscale: Option<*mut super::sqltypes::SQLSMALLINT>, pnullable: Option<*mut super::sqltypes::SQLSMALLINT>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetDescRecA(hdesc : super::sqltypes::SQLHDESC, irecord : super::sqltypes::SQLSMALLINT, szname : *mut super::sqltypes::SQLCHAR, cbnamemax : super::sqltypes::SQLSMALLINT, pcbname : *mut super::sqltypes::SQLSMALLINT, pftype : *mut super::sqltypes::SQLSMALLINT, pfsubtype : *mut super::sqltypes::SQLSMALLINT, plength : *mut super::sqltypes::SQLLEN, pprecision : *mut super::sqltypes::SQLSMALLINT, pscale : *mut super::sqltypes::SQLSMALLINT, pnullable : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe {
        SQLGetDescRecA(
            hdesc,
            irecord,
            szname.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()),
            szname.as_deref().map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            pcbname.unwrap_or(core::mem::zeroed()) as _,
            pftype.unwrap_or(core::mem::zeroed()) as _,
            pfsubtype.unwrap_or(core::mem::zeroed()) as _,
            plength.unwrap_or(core::mem::zeroed()) as _,
            pprecision.unwrap_or(core::mem::zeroed()) as _,
            pscale.unwrap_or(core::mem::zeroed()) as _,
            pnullable.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetDescRecW(hdesc: super::sqltypes::SQLHDESC, irecord: super::sqltypes::SQLSMALLINT, szname: Option<&mut [super::sqltypes::SQLWCHAR]>, pcchname: Option<*mut super::sqltypes::SQLSMALLINT>, pftype: Option<*mut super::sqltypes::SQLSMALLINT>, pfsubtype: Option<*mut super::sqltypes::SQLSMALLINT>, plength: Option<*mut super::sqltypes::SQLINTEGER>, pprecision: Option<*mut super::sqltypes::SQLSMALLINT>, pscale: Option<*mut super::sqltypes::SQLSMALLINT>, pnullable: Option<*mut super::sqltypes::SQLSMALLINT>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetDescRecW(hdesc : super::sqltypes::SQLHDESC, irecord : super::sqltypes::SQLSMALLINT, szname : *mut super::sqltypes::SQLWCHAR, cchnamemax : super::sqltypes::SQLSMALLINT, pcchname : *mut super::sqltypes::SQLSMALLINT, pftype : *mut super::sqltypes::SQLSMALLINT, pfsubtype : *mut super::sqltypes::SQLSMALLINT, plength : *mut super::sqltypes::SQLINTEGER, pprecision : *mut super::sqltypes::SQLSMALLINT, pscale : *mut super::sqltypes::SQLSMALLINT, pnullable : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe {
        SQLGetDescRecW(
            hdesc,
            irecord,
            szname.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()),
            szname.as_deref().map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            pcchname.unwrap_or(core::mem::zeroed()) as _,
            pftype.unwrap_or(core::mem::zeroed()) as _,
            pfsubtype.unwrap_or(core::mem::zeroed()) as _,
            plength.unwrap_or(core::mem::zeroed()) as _,
            pprecision.unwrap_or(core::mem::zeroed()) as _,
            pscale.unwrap_or(core::mem::zeroed()) as _,
            pnullable.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetDescRecW(hdesc: super::sqltypes::SQLHDESC, irecord: super::sqltypes::SQLSMALLINT, szname: Option<&mut [super::sqltypes::SQLWCHAR]>, pcchname: Option<*mut super::sqltypes::SQLSMALLINT>, pftype: Option<*mut super::sqltypes::SQLSMALLINT>, pfsubtype: Option<*mut super::sqltypes::SQLSMALLINT>, plength: Option<*mut super::sqltypes::SQLLEN>, pprecision: Option<*mut super::sqltypes::SQLSMALLINT>, pscale: Option<*mut super::sqltypes::SQLSMALLINT>, pnullable: Option<*mut super::sqltypes::SQLSMALLINT>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetDescRecW(hdesc : super::sqltypes::SQLHDESC, irecord : super::sqltypes::SQLSMALLINT, szname : *mut super::sqltypes::SQLWCHAR, cchnamemax : super::sqltypes::SQLSMALLINT, pcchname : *mut super::sqltypes::SQLSMALLINT, pftype : *mut super::sqltypes::SQLSMALLINT, pfsubtype : *mut super::sqltypes::SQLSMALLINT, plength : *mut super::sqltypes::SQLLEN, pprecision : *mut super::sqltypes::SQLSMALLINT, pscale : *mut super::sqltypes::SQLSMALLINT, pnullable : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe {
        SQLGetDescRecW(
            hdesc,
            irecord,
            szname.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()),
            szname.as_deref().map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            pcchname.unwrap_or(core::mem::zeroed()) as _,
            pftype.unwrap_or(core::mem::zeroed()) as _,
            pfsubtype.unwrap_or(core::mem::zeroed()) as _,
            plength.unwrap_or(core::mem::zeroed()) as _,
            pprecision.unwrap_or(core::mem::zeroed()) as _,
            pscale.unwrap_or(core::mem::zeroed()) as _,
            pnullable.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetDiagFieldA(fhandletype: super::sqltypes::SQLSMALLINT, handle: super::sqltypes::SQLHANDLE, irecord: super::sqltypes::SQLSMALLINT, fdiagfield: super::sqltypes::SQLSMALLINT, rgbdiaginfo: Option<super::sqltypes::SQLPOINTER>, cbdiaginfomax: super::sqltypes::SQLSMALLINT, pcbdiaginfo: Option<*mut super::sqltypes::SQLSMALLINT>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetDiagFieldA(fhandletype : super::sqltypes::SQLSMALLINT, handle : super::sqltypes::SQLHANDLE, irecord : super::sqltypes::SQLSMALLINT, fdiagfield : super::sqltypes::SQLSMALLINT, rgbdiaginfo : super::sqltypes::SQLPOINTER, cbdiaginfomax : super::sqltypes::SQLSMALLINT, pcbdiaginfo : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe { SQLGetDiagFieldA(fhandletype, handle, irecord, fdiagfield, rgbdiaginfo.unwrap_or(core::mem::zeroed()) as _, cbdiaginfomax, pcbdiaginfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetDiagFieldW(fhandletype: super::sqltypes::SQLSMALLINT, handle: super::sqltypes::SQLHANDLE, irecord: super::sqltypes::SQLSMALLINT, fdiagfield: super::sqltypes::SQLSMALLINT, rgbdiaginfo: Option<super::sqltypes::SQLPOINTER>, cbbufferlength: super::sqltypes::SQLSMALLINT, pcbstringlength: Option<*mut super::sqltypes::SQLSMALLINT>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetDiagFieldW(fhandletype : super::sqltypes::SQLSMALLINT, handle : super::sqltypes::SQLHANDLE, irecord : super::sqltypes::SQLSMALLINT, fdiagfield : super::sqltypes::SQLSMALLINT, rgbdiaginfo : super::sqltypes::SQLPOINTER, cbbufferlength : super::sqltypes::SQLSMALLINT, pcbstringlength : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe { SQLGetDiagFieldW(fhandletype, handle, irecord, fdiagfield, rgbdiaginfo.unwrap_or(core::mem::zeroed()) as _, cbbufferlength, pcbstringlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetDiagRecA(fhandletype: super::sqltypes::SQLSMALLINT, handle: super::sqltypes::SQLHANDLE, irecord: super::sqltypes::SQLSMALLINT, szsqlstate: Option<&mut [super::sqltypes::SQLCHAR; 6]>, pfnativeerror: *mut super::sqltypes::SQLINTEGER, szerrormsg: Option<&mut [super::sqltypes::SQLCHAR]>, pcberrormsg: *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetDiagRecA(fhandletype : super::sqltypes::SQLSMALLINT, handle : super::sqltypes::SQLHANDLE, irecord : super::sqltypes::SQLSMALLINT, szsqlstate : *mut super::sqltypes::SQLCHAR, pfnativeerror : *mut super::sqltypes::SQLINTEGER, szerrormsg : *mut super::sqltypes::SQLCHAR, cberrormsgmax : super::sqltypes::SQLSMALLINT, pcberrormsg : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe { SQLGetDiagRecA(fhandletype, handle, irecord, szsqlstate.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), pfnativeerror as _, szerrormsg.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), szerrormsg.as_deref().map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())), pcberrormsg as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetDiagRecW(fhandletype: super::sqltypes::SQLSMALLINT, handle: super::sqltypes::SQLHANDLE, irecord: super::sqltypes::SQLSMALLINT, szsqlstate: Option<&mut [super::sqltypes::SQLWCHAR; 6]>, pfnativeerror: *mut super::sqltypes::SQLINTEGER, szerrormsg: Option<&mut [super::sqltypes::SQLWCHAR]>, pccherrormsg: *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetDiagRecW(fhandletype : super::sqltypes::SQLSMALLINT, handle : super::sqltypes::SQLHANDLE, irecord : super::sqltypes::SQLSMALLINT, szsqlstate : *mut super::sqltypes::SQLWCHAR, pfnativeerror : *mut super::sqltypes::SQLINTEGER, szerrormsg : *mut super::sqltypes::SQLWCHAR, ccherrormsgmax : super::sqltypes::SQLSMALLINT, pccherrormsg : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe { SQLGetDiagRecW(fhandletype, handle, irecord, szsqlstate.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), pfnativeerror as _, szerrormsg.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), szerrormsg.as_deref().map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())), pccherrormsg as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetInfoA(hdbc: super::sqltypes::SQLHDBC, finfotype: super::sqltypes::SQLUSMALLINT, rgbinfovalue: Option<super::sqltypes::SQLPOINTER>, cbinfovaluemax: super::sqltypes::SQLSMALLINT, pcbinfovalue: Option<*mut super::sqltypes::SQLSMALLINT>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetInfoA(hdbc : super::sqltypes::SQLHDBC, finfotype : super::sqltypes::SQLUSMALLINT, rgbinfovalue : super::sqltypes::SQLPOINTER, cbinfovaluemax : super::sqltypes::SQLSMALLINT, pcbinfovalue : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe { SQLGetInfoA(hdbc, finfotype, rgbinfovalue.unwrap_or(core::mem::zeroed()) as _, cbinfovaluemax, pcbinfovalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetInfoW(hdbc: super::sqltypes::SQLHDBC, finfotype: super::sqltypes::SQLUSMALLINT, rgbinfovalue: Option<super::sqltypes::SQLPOINTER>, cbinfovaluemax: super::sqltypes::SQLSMALLINT, pcbinfovalue: Option<*mut super::sqltypes::SQLSMALLINT>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetInfoW(hdbc : super::sqltypes::SQLHDBC, finfotype : super::sqltypes::SQLUSMALLINT, rgbinfovalue : super::sqltypes::SQLPOINTER, cbinfovaluemax : super::sqltypes::SQLSMALLINT, pcbinfovalue : *mut super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe { SQLGetInfoW(hdbc, finfotype, rgbinfovalue.unwrap_or(core::mem::zeroed()) as _, cbinfovaluemax, pcbinfovalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetStmtAttrA(hstmt: super::sqltypes::SQLHSTMT, fattribute: super::sqltypes::SQLINTEGER, rgbvalue: super::sqltypes::SQLPOINTER, cbvaluemax: super::sqltypes::SQLINTEGER, pcbvalue: *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetStmtAttrA(hstmt : super::sqltypes::SQLHSTMT, fattribute : super::sqltypes::SQLINTEGER, rgbvalue : super::sqltypes::SQLPOINTER, cbvaluemax : super::sqltypes::SQLINTEGER, pcbvalue : *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
    unsafe { SQLGetStmtAttrA(hstmt, fattribute, rgbvalue, cbvaluemax, pcbvalue as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetStmtAttrW(hstmt: super::sqltypes::SQLHSTMT, fattribute: super::sqltypes::SQLINTEGER, rgbvalue: super::sqltypes::SQLPOINTER, cbvaluemax: super::sqltypes::SQLINTEGER, pcbvalue: *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetStmtAttrW(hstmt : super::sqltypes::SQLHSTMT, fattribute : super::sqltypes::SQLINTEGER, rgbvalue : super::sqltypes::SQLPOINTER, cbvaluemax : super::sqltypes::SQLINTEGER, pcbvalue : *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
    unsafe { SQLGetStmtAttrW(hstmt, fattribute, rgbvalue, cbvaluemax, pcbvalue as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetTypeInfoA(statementhandle: super::sqltypes::SQLHSTMT, datatype: super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetTypeInfoA(statementhandle : super::sqltypes::SQLHSTMT, datatype : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe { SQLGetTypeInfoA(statementhandle, datatype) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetTypeInfoW(statementhandle: super::sqltypes::SQLHSTMT, datatype: super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetTypeInfoW(statementhandle : super::sqltypes::SQLHSTMT, datatype : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe { SQLGetTypeInfoW(statementhandle, datatype) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLNativeSqlA(hdbc: super::sqltypes::SQLHDBC, szsqlstrin: &[super::sqltypes::SQLCHAR], szsqlstr: Option<&mut [super::sqltypes::SQLCHAR]>, pcbsqlstr: *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLNativeSqlA(hdbc : super::sqltypes::SQLHDBC, szsqlstrin : *const super::sqltypes::SQLCHAR, cbsqlstrin : super::sqltypes::SQLINTEGER, szsqlstr : *mut super::sqltypes::SQLCHAR, cbsqlstrmax : super::sqltypes::SQLINTEGER, pcbsqlstr : *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
    unsafe { SQLNativeSqlA(hdbc, szsqlstrin.as_ptr(), super::sqltypes::SQLINTEGER(szsqlstrin.len().try_into().unwrap()), szsqlstr.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), szsqlstr.as_deref().map_or(super::sqltypes::SQLINTEGER(0), |slice| super::sqltypes::SQLINTEGER(slice.len().try_into().unwrap())), pcbsqlstr as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLNativeSqlW(hdbc: super::sqltypes::SQLHDBC, szsqlstrin: &[super::sqltypes::SQLWCHAR], szsqlstr: Option<&mut [super::sqltypes::SQLWCHAR]>, pcchsqlstr: *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLNativeSqlW(hdbc : super::sqltypes::SQLHDBC, szsqlstrin : *const super::sqltypes::SQLWCHAR, cchsqlstrin : super::sqltypes::SQLINTEGER, szsqlstr : *mut super::sqltypes::SQLWCHAR, cchsqlstrmax : super::sqltypes::SQLINTEGER, pcchsqlstr : *mut super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
    unsafe { SQLNativeSqlW(hdbc, szsqlstrin.as_ptr(), super::sqltypes::SQLINTEGER(szsqlstrin.len().try_into().unwrap()), szsqlstr.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), szsqlstr.as_deref().map_or(super::sqltypes::SQLINTEGER(0), |slice| super::sqltypes::SQLINTEGER(slice.len().try_into().unwrap())), pcchsqlstr as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLPrepareA(hstmt: super::sqltypes::SQLHSTMT, szsqlstr: &[super::sqltypes::SQLCHAR]) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLPrepareA(hstmt : super::sqltypes::SQLHSTMT, szsqlstr : *const super::sqltypes::SQLCHAR, cbsqlstr : super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
    unsafe { SQLPrepareA(hstmt, szsqlstr.as_ptr(), super::sqltypes::SQLINTEGER(szsqlstr.len().try_into().unwrap())) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLPrepareW(hstmt: super::sqltypes::SQLHSTMT, szsqlstr: &[super::sqltypes::SQLWCHAR]) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLPrepareW(hstmt : super::sqltypes::SQLHSTMT, szsqlstr : *const super::sqltypes::SQLWCHAR, cchsqlstr : super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
    unsafe { SQLPrepareW(hstmt, szsqlstr.as_ptr(), super::sqltypes::SQLINTEGER(szsqlstr.len().try_into().unwrap())) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLPrimaryKeysA(hstmt: super::sqltypes::SQLHSTMT, szcatalogname: Option<&[super::sqltypes::SQLCHAR]>, szschemaname: Option<&[super::sqltypes::SQLCHAR]>, sztablename: Option<&[super::sqltypes::SQLCHAR]>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLPrimaryKeysA(hstmt : super::sqltypes::SQLHSTMT, szcatalogname : *const super::sqltypes::SQLCHAR, cbcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLCHAR, cbschemaname : super::sqltypes::SQLSMALLINT, sztablename : *const super::sqltypes::SQLCHAR, cbtablename : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe {
        SQLPrimaryKeysA(
            hstmt,
            szcatalogname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szcatalogname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szschemaname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szschemaname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            sztablename.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            sztablename.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
        )
    }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLPrimaryKeysW(hstmt: super::sqltypes::SQLHSTMT, szcatalogname: Option<&[super::sqltypes::SQLWCHAR]>, szschemaname: Option<&[super::sqltypes::SQLWCHAR]>, sztablename: Option<&[super::sqltypes::SQLWCHAR]>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLPrimaryKeysW(hstmt : super::sqltypes::SQLHSTMT, szcatalogname : *const super::sqltypes::SQLWCHAR, cchcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLWCHAR, cchschemaname : super::sqltypes::SQLSMALLINT, sztablename : *const super::sqltypes::SQLWCHAR, cchtablename : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe {
        SQLPrimaryKeysW(
            hstmt,
            szcatalogname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szcatalogname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szschemaname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szschemaname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            sztablename.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            sztablename.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
        )
    }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLProcedureColumnsA(hstmt: super::sqltypes::SQLHSTMT, szcatalogname: Option<&[super::sqltypes::SQLCHAR]>, szschemaname: Option<&[super::sqltypes::SQLCHAR]>, szprocname: Option<&[super::sqltypes::SQLCHAR]>, szcolumnname: Option<&[super::sqltypes::SQLCHAR]>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLProcedureColumnsA(hstmt : super::sqltypes::SQLHSTMT, szcatalogname : *const super::sqltypes::SQLCHAR, cbcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLCHAR, cbschemaname : super::sqltypes::SQLSMALLINT, szprocname : *const super::sqltypes::SQLCHAR, cbprocname : super::sqltypes::SQLSMALLINT, szcolumnname : *const super::sqltypes::SQLCHAR, cbcolumnname : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe {
        SQLProcedureColumnsA(
            hstmt,
            szcatalogname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szcatalogname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szschemaname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szschemaname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szprocname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szprocname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szcolumnname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szcolumnname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
        )
    }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLProcedureColumnsW(hstmt: super::sqltypes::SQLHSTMT, szcatalogname: Option<&[super::sqltypes::SQLWCHAR]>, szschemaname: Option<&[super::sqltypes::SQLWCHAR]>, szprocname: Option<&[super::sqltypes::SQLWCHAR]>, szcolumnname: Option<&[super::sqltypes::SQLWCHAR]>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLProcedureColumnsW(hstmt : super::sqltypes::SQLHSTMT, szcatalogname : *const super::sqltypes::SQLWCHAR, cchcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLWCHAR, cchschemaname : super::sqltypes::SQLSMALLINT, szprocname : *const super::sqltypes::SQLWCHAR, cchprocname : super::sqltypes::SQLSMALLINT, szcolumnname : *const super::sqltypes::SQLWCHAR, cchcolumnname : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe {
        SQLProcedureColumnsW(
            hstmt,
            szcatalogname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szcatalogname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szschemaname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szschemaname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szprocname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szprocname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szcolumnname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szcolumnname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
        )
    }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLProceduresA(hstmt: super::sqltypes::SQLHSTMT, szcatalogname: Option<&[super::sqltypes::SQLCHAR]>, szschemaname: Option<&[super::sqltypes::SQLCHAR]>, szprocname: Option<&[super::sqltypes::SQLCHAR]>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLProceduresA(hstmt : super::sqltypes::SQLHSTMT, szcatalogname : *const super::sqltypes::SQLCHAR, cbcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLCHAR, cbschemaname : super::sqltypes::SQLSMALLINT, szprocname : *const super::sqltypes::SQLCHAR, cbprocname : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe {
        SQLProceduresA(
            hstmt,
            szcatalogname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szcatalogname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szschemaname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szschemaname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szprocname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szprocname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
        )
    }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLProceduresW(hstmt: super::sqltypes::SQLHSTMT, szcatalogname: Option<&[super::sqltypes::SQLWCHAR]>, szschemaname: Option<&[super::sqltypes::SQLWCHAR]>, szprocname: Option<&[super::sqltypes::SQLWCHAR]>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLProceduresW(hstmt : super::sqltypes::SQLHSTMT, szcatalogname : *const super::sqltypes::SQLWCHAR, cchcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLWCHAR, cchschemaname : super::sqltypes::SQLSMALLINT, szprocname : *const super::sqltypes::SQLWCHAR, cchprocname : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe {
        SQLProceduresW(
            hstmt,
            szcatalogname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szcatalogname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szschemaname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szschemaname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szprocname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szprocname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
        )
    }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSetConnectAttrA(hdbc: super::sqltypes::SQLHDBC, fattribute: super::sqltypes::SQLINTEGER, rgbvalue: Option<super::sqltypes::SQLPOINTER>, cbvalue: super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetConnectAttrA(hdbc : super::sqltypes::SQLHDBC, fattribute : super::sqltypes::SQLINTEGER, rgbvalue : super::sqltypes::SQLPOINTER, cbvalue : super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
    unsafe { SQLSetConnectAttrA(hdbc, fattribute, rgbvalue.unwrap_or(core::mem::zeroed()) as _, cbvalue) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSetConnectAttrW(hdbc: super::sqltypes::SQLHDBC, fattribute: super::sqltypes::SQLINTEGER, rgbvalue: Option<super::sqltypes::SQLPOINTER>, cbvalue: super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetConnectAttrW(hdbc : super::sqltypes::SQLHDBC, fattribute : super::sqltypes::SQLINTEGER, rgbvalue : super::sqltypes::SQLPOINTER, cbvalue : super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
    unsafe { SQLSetConnectAttrW(hdbc, fattribute, rgbvalue.unwrap_or(core::mem::zeroed()) as _, cbvalue) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSetConnectOptionA(hdbc: super::sqltypes::SQLHDBC, foption: super::sqltypes::SQLUSMALLINT, vparam: super::sqltypes::SQLUINTEGER) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetConnectOptionA(hdbc : super::sqltypes::SQLHDBC, foption : super::sqltypes::SQLUSMALLINT, vparam : super::sqltypes::SQLUINTEGER) -> super::sqltypes::SQLRETURN);
    unsafe { SQLSetConnectOptionA(hdbc, foption, vparam) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSetConnectOptionA(hdbc: super::sqltypes::SQLHDBC, foption: super::sqltypes::SQLUSMALLINT, vparam: super::sqltypes::SQLULEN) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetConnectOptionA(hdbc : super::sqltypes::SQLHDBC, foption : super::sqltypes::SQLUSMALLINT, vparam : super::sqltypes::SQLULEN) -> super::sqltypes::SQLRETURN);
    unsafe { SQLSetConnectOptionA(hdbc, foption, vparam) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSetConnectOptionW(hdbc: super::sqltypes::SQLHDBC, foption: super::sqltypes::SQLUSMALLINT, vparam: super::sqltypes::SQLUINTEGER) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetConnectOptionW(hdbc : super::sqltypes::SQLHDBC, foption : super::sqltypes::SQLUSMALLINT, vparam : super::sqltypes::SQLUINTEGER) -> super::sqltypes::SQLRETURN);
    unsafe { SQLSetConnectOptionW(hdbc, foption, vparam) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSetConnectOptionW(hdbc: super::sqltypes::SQLHDBC, foption: super::sqltypes::SQLUSMALLINT, vparam: super::sqltypes::SQLULEN) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetConnectOptionW(hdbc : super::sqltypes::SQLHDBC, foption : super::sqltypes::SQLUSMALLINT, vparam : super::sqltypes::SQLULEN) -> super::sqltypes::SQLRETURN);
    unsafe { SQLSetConnectOptionW(hdbc, foption, vparam) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSetCursorNameA(hstmt: super::sqltypes::SQLHSTMT, szcursor: &[super::sqltypes::SQLCHAR]) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetCursorNameA(hstmt : super::sqltypes::SQLHSTMT, szcursor : *const super::sqltypes::SQLCHAR, cbcursor : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe { SQLSetCursorNameA(hstmt, szcursor.as_ptr(), super::sqltypes::SQLSMALLINT(szcursor.len().try_into().unwrap())) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSetCursorNameW(hstmt: super::sqltypes::SQLHSTMT, szcursor: &[super::sqltypes::SQLWCHAR]) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetCursorNameW(hstmt : super::sqltypes::SQLHSTMT, szcursor : *const super::sqltypes::SQLWCHAR, cchcursor : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe { SQLSetCursorNameW(hstmt, szcursor.as_ptr(), super::sqltypes::SQLSMALLINT(szcursor.len().try_into().unwrap())) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSetDescFieldW(descriptorhandle: super::sqltypes::SQLHDESC, recnumber: super::sqltypes::SQLSMALLINT, fieldidentifier: super::sqltypes::SQLSMALLINT, value: super::sqltypes::SQLPOINTER, bufferlength: super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetDescFieldW(descriptorhandle : super::sqltypes::SQLHDESC, recnumber : super::sqltypes::SQLSMALLINT, fieldidentifier : super::sqltypes::SQLSMALLINT, value : super::sqltypes::SQLPOINTER, bufferlength : super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
    unsafe { SQLSetDescFieldW(descriptorhandle, recnumber, fieldidentifier, value, bufferlength) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSetStmtAttrW(hstmt: super::sqltypes::SQLHSTMT, fattribute: super::sqltypes::SQLINTEGER, rgbvalue: super::sqltypes::SQLPOINTER, cbvaluemax: super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetStmtAttrW(hstmt : super::sqltypes::SQLHSTMT, fattribute : super::sqltypes::SQLINTEGER, rgbvalue : super::sqltypes::SQLPOINTER, cbvaluemax : super::sqltypes::SQLINTEGER) -> super::sqltypes::SQLRETURN);
    unsafe { SQLSetStmtAttrW(hstmt, fattribute, rgbvalue, cbvaluemax) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSpecialColumnsA(hstmt: super::sqltypes::SQLHSTMT, fcoltype: super::sqltypes::SQLUSMALLINT, szcatalogname: Option<&[super::sqltypes::SQLCHAR]>, szschemaname: Option<&[super::sqltypes::SQLCHAR]>, sztablename: Option<&[super::sqltypes::SQLCHAR]>, fscope: super::sqltypes::SQLUSMALLINT, fnullable: super::sqltypes::SQLUSMALLINT) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSpecialColumnsA(hstmt : super::sqltypes::SQLHSTMT, fcoltype : super::sqltypes::SQLUSMALLINT, szcatalogname : *const super::sqltypes::SQLCHAR, cbcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLCHAR, cbschemaname : super::sqltypes::SQLSMALLINT, sztablename : *const super::sqltypes::SQLCHAR, cbtablename : super::sqltypes::SQLSMALLINT, fscope : super::sqltypes::SQLUSMALLINT, fnullable : super::sqltypes::SQLUSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe {
        SQLSpecialColumnsA(
            hstmt,
            fcoltype,
            szcatalogname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szcatalogname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szschemaname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szschemaname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            sztablename.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            sztablename.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            fscope,
            fnullable,
        )
    }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSpecialColumnsW(hstmt: super::sqltypes::SQLHSTMT, fcoltype: super::sqltypes::SQLUSMALLINT, szcatalogname: Option<&[super::sqltypes::SQLWCHAR]>, szschemaname: Option<&[super::sqltypes::SQLWCHAR]>, sztablename: Option<&[super::sqltypes::SQLWCHAR]>, fscope: super::sqltypes::SQLUSMALLINT, fnullable: super::sqltypes::SQLUSMALLINT) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSpecialColumnsW(hstmt : super::sqltypes::SQLHSTMT, fcoltype : super::sqltypes::SQLUSMALLINT, szcatalogname : *const super::sqltypes::SQLWCHAR, cchcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLWCHAR, cchschemaname : super::sqltypes::SQLSMALLINT, sztablename : *const super::sqltypes::SQLWCHAR, cchtablename : super::sqltypes::SQLSMALLINT, fscope : super::sqltypes::SQLUSMALLINT, fnullable : super::sqltypes::SQLUSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe {
        SQLSpecialColumnsW(
            hstmt,
            fcoltype,
            szcatalogname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szcatalogname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szschemaname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szschemaname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            sztablename.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            sztablename.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            fscope,
            fnullable,
        )
    }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLStatisticsA(hstmt: super::sqltypes::SQLHSTMT, szcatalogname: Option<&[super::sqltypes::SQLCHAR]>, szschemaname: Option<&[super::sqltypes::SQLCHAR]>, sztablename: Option<&[super::sqltypes::SQLCHAR]>, funique: super::sqltypes::SQLUSMALLINT, faccuracy: super::sqltypes::SQLUSMALLINT) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLStatisticsA(hstmt : super::sqltypes::SQLHSTMT, szcatalogname : *const super::sqltypes::SQLCHAR, cbcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLCHAR, cbschemaname : super::sqltypes::SQLSMALLINT, sztablename : *const super::sqltypes::SQLCHAR, cbtablename : super::sqltypes::SQLSMALLINT, funique : super::sqltypes::SQLUSMALLINT, faccuracy : super::sqltypes::SQLUSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe {
        SQLStatisticsA(
            hstmt,
            szcatalogname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szcatalogname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szschemaname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szschemaname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            sztablename.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            sztablename.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            funique,
            faccuracy,
        )
    }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLStatisticsW(hstmt: super::sqltypes::SQLHSTMT, szcatalogname: Option<&[super::sqltypes::SQLWCHAR]>, szschemaname: Option<&[super::sqltypes::SQLWCHAR]>, sztablename: Option<&[super::sqltypes::SQLWCHAR]>, funique: super::sqltypes::SQLUSMALLINT, faccuracy: super::sqltypes::SQLUSMALLINT) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLStatisticsW(hstmt : super::sqltypes::SQLHSTMT, szcatalogname : *const super::sqltypes::SQLWCHAR, cchcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLWCHAR, cchschemaname : super::sqltypes::SQLSMALLINT, sztablename : *const super::sqltypes::SQLWCHAR, cchtablename : super::sqltypes::SQLSMALLINT, funique : super::sqltypes::SQLUSMALLINT, faccuracy : super::sqltypes::SQLUSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe {
        SQLStatisticsW(
            hstmt,
            szcatalogname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szcatalogname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szschemaname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szschemaname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            sztablename.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            sztablename.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            funique,
            faccuracy,
        )
    }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLTablePrivilegesA(hstmt: super::sqltypes::SQLHSTMT, szcatalogname: Option<&[super::sqltypes::SQLCHAR]>, szschemaname: Option<&[super::sqltypes::SQLCHAR]>, sztablename: Option<&[super::sqltypes::SQLCHAR]>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLTablePrivilegesA(hstmt : super::sqltypes::SQLHSTMT, szcatalogname : *const super::sqltypes::SQLCHAR, cbcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLCHAR, cbschemaname : super::sqltypes::SQLSMALLINT, sztablename : *const super::sqltypes::SQLCHAR, cbtablename : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe {
        SQLTablePrivilegesA(
            hstmt,
            szcatalogname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szcatalogname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szschemaname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szschemaname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            sztablename.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            sztablename.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
        )
    }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLTablePrivilegesW(hstmt: super::sqltypes::SQLHSTMT, szcatalogname: Option<&[super::sqltypes::SQLWCHAR]>, szschemaname: Option<&[super::sqltypes::SQLWCHAR]>, sztablename: Option<&[super::sqltypes::SQLWCHAR]>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLTablePrivilegesW(hstmt : super::sqltypes::SQLHSTMT, szcatalogname : *const super::sqltypes::SQLWCHAR, cchcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLWCHAR, cchschemaname : super::sqltypes::SQLSMALLINT, sztablename : *const super::sqltypes::SQLWCHAR, cchtablename : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe {
        SQLTablePrivilegesW(
            hstmt,
            szcatalogname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szcatalogname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szschemaname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szschemaname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            sztablename.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            sztablename.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
        )
    }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLTablesA(hstmt: super::sqltypes::SQLHSTMT, szcatalogname: Option<&[super::sqltypes::SQLCHAR]>, szschemaname: Option<&[super::sqltypes::SQLCHAR]>, sztablename: Option<&[super::sqltypes::SQLCHAR]>, sztabletype: Option<&[super::sqltypes::SQLCHAR]>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLTablesA(hstmt : super::sqltypes::SQLHSTMT, szcatalogname : *const super::sqltypes::SQLCHAR, cbcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLCHAR, cbschemaname : super::sqltypes::SQLSMALLINT, sztablename : *const super::sqltypes::SQLCHAR, cbtablename : super::sqltypes::SQLSMALLINT, sztabletype : *const super::sqltypes::SQLCHAR, cbtabletype : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe {
        SQLTablesA(
            hstmt,
            szcatalogname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szcatalogname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szschemaname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szschemaname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            sztablename.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            sztablename.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            sztabletype.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            sztabletype.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
        )
    }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLTablesW(hstmt: super::sqltypes::SQLHSTMT, szcatalogname: Option<&[super::sqltypes::SQLWCHAR]>, szschemaname: Option<&[super::sqltypes::SQLWCHAR]>, sztablename: Option<&[super::sqltypes::SQLWCHAR]>, sztabletype: Option<&[super::sqltypes::SQLWCHAR]>) -> super::sqltypes::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLTablesW(hstmt : super::sqltypes::SQLHSTMT, szcatalogname : *const super::sqltypes::SQLWCHAR, cchcatalogname : super::sqltypes::SQLSMALLINT, szschemaname : *const super::sqltypes::SQLWCHAR, cchschemaname : super::sqltypes::SQLSMALLINT, sztablename : *const super::sqltypes::SQLWCHAR, cchtablename : super::sqltypes::SQLSMALLINT, sztabletype : *const super::sqltypes::SQLWCHAR, cchtabletype : super::sqltypes::SQLSMALLINT) -> super::sqltypes::SQLRETURN);
    unsafe {
        SQLTablesW(
            hstmt,
            szcatalogname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szcatalogname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            szschemaname.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            szschemaname.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            sztablename.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            sztablename.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
            sztabletype.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            sztabletype.map_or(super::sqltypes::SQLSMALLINT(0), |slice| super::sqltypes::SQLSMALLINT(slice.len().try_into().unwrap())),
        )
    }
}
pub const SQL_C_TCHAR: u32 = 1;
pub const SQL_C_WCHAR: i32 = -8;
pub const SQL_SQLSTATE_SIZEW: u32 = 10;
pub const SQL_WCHAR: i32 = -8;
pub const SQL_WLONGVARCHAR: i32 = -10;
pub const SQL_WVARCHAR: i32 = -9;
