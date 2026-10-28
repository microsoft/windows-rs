#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLBrowseConnectA(hdbc: super::SQLHDBC, szconnstrin: &[super::SQLCHAR], szconnstrout: Option<*mut super::SQLCHAR>, cbconnstroutmax: super::SQLSMALLINT, pcbconnstrout: Option<*mut super::SQLSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLBrowseConnectA(hdbc : super::SQLHDBC, szconnstrin : *const super::SQLCHAR, cbconnstrin : super::SQLSMALLINT, szconnstrout : *mut super::SQLCHAR, cbconnstroutmax : super::SQLSMALLINT, pcbconnstrout : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLBrowseConnectA(hdbc as _, szconnstrin.as_ptr(), szconnstrin.len().try_into().unwrap(), szconnstrout.unwrap_or(core::mem::zeroed()) as _, cbconnstroutmax, pcbconnstrout.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLBrowseConnectW(hdbc: super::SQLHDBC, szconnstrin: &[super::SQLWCHAR], szconnstrout: Option<*mut super::SQLWCHAR>, cchconnstroutmax: super::SQLSMALLINT, pcchconnstrout: Option<*mut super::SQLSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLBrowseConnectW(hdbc : super::SQLHDBC, szconnstrin : *const super::SQLWCHAR, cchconnstrin : super::SQLSMALLINT, szconnstrout : *mut super::SQLWCHAR, cchconnstroutmax : super::SQLSMALLINT, pcchconnstrout : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLBrowseConnectW(hdbc as _, szconnstrin.as_ptr(), szconnstrin.len().try_into().unwrap(), szconnstrout.unwrap_or(core::mem::zeroed()) as _, cchconnstroutmax, pcchconnstrout.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLColAttributeA(hstmt: super::SQLHSTMT, icol: super::SQLSMALLINT, ifield: super::SQLSMALLINT, pcharattr: Option<super::SQLPOINTER>, cbcharattrmax: super::SQLSMALLINT, pcbcharattr: Option<*mut super::SQLSMALLINT>, pnumattr: Option<super::SQLPOINTER>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLColAttributeA(hstmt : super::SQLHSTMT, icol : super::SQLSMALLINT, ifield : super::SQLSMALLINT, pcharattr : super::SQLPOINTER, cbcharattrmax : super::SQLSMALLINT, pcbcharattr : *mut super::SQLSMALLINT, pnumattr : super::SQLPOINTER) -> super::SQLRETURN);
    unsafe { SQLColAttributeA(hstmt as _, icol, ifield, pcharattr.unwrap_or(core::mem::zeroed()) as _, cbcharattrmax, pcbcharattr.unwrap_or(core::mem::zeroed()) as _, pnumattr.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLColAttributeA(hstmt: super::SQLHSTMT, icol: super::SQLSMALLINT, ifield: super::SQLSMALLINT, pcharattr: Option<super::SQLPOINTER>, cbcharattrmax: super::SQLSMALLINT, pcbcharattr: Option<*mut super::SQLSMALLINT>, pnumattr: Option<*mut super::SQLLEN>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLColAttributeA(hstmt : super::SQLHSTMT, icol : super::SQLSMALLINT, ifield : super::SQLSMALLINT, pcharattr : super::SQLPOINTER, cbcharattrmax : super::SQLSMALLINT, pcbcharattr : *mut super::SQLSMALLINT, pnumattr : *mut super::SQLLEN) -> super::SQLRETURN);
    unsafe { SQLColAttributeA(hstmt as _, icol, ifield, pcharattr.unwrap_or(core::mem::zeroed()) as _, cbcharattrmax, pcbcharattr.unwrap_or(core::mem::zeroed()) as _, pnumattr.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLColAttributeW(hstmt: super::SQLHSTMT, icol: super::SQLUSMALLINT, ifield: super::SQLUSMALLINT, pcharattr: Option<super::SQLPOINTER>, cbdescmax: super::SQLSMALLINT, pcbcharattr: Option<*mut super::SQLSMALLINT>, pnumattr: Option<super::SQLPOINTER>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLColAttributeW(hstmt : super::SQLHSTMT, icol : super::SQLUSMALLINT, ifield : super::SQLUSMALLINT, pcharattr : super::SQLPOINTER, cbdescmax : super::SQLSMALLINT, pcbcharattr : *mut super::SQLSMALLINT, pnumattr : super::SQLPOINTER) -> super::SQLRETURN);
    unsafe { SQLColAttributeW(hstmt as _, icol, ifield, pcharattr.unwrap_or(core::mem::zeroed()) as _, cbdescmax, pcbcharattr.unwrap_or(core::mem::zeroed()) as _, pnumattr.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLColAttributeW(hstmt: super::SQLHSTMT, icol: super::SQLUSMALLINT, ifield: super::SQLUSMALLINT, pcharattr: Option<super::SQLPOINTER>, cbdescmax: super::SQLSMALLINT, pcbcharattr: Option<*mut super::SQLSMALLINT>, pnumattr: Option<*mut super::SQLLEN>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLColAttributeW(hstmt : super::SQLHSTMT, icol : super::SQLUSMALLINT, ifield : super::SQLUSMALLINT, pcharattr : super::SQLPOINTER, cbdescmax : super::SQLSMALLINT, pcbcharattr : *mut super::SQLSMALLINT, pnumattr : *mut super::SQLLEN) -> super::SQLRETURN);
    unsafe { SQLColAttributeW(hstmt as _, icol, ifield, pcharattr.unwrap_or(core::mem::zeroed()) as _, cbdescmax, pcbcharattr.unwrap_or(core::mem::zeroed()) as _, pnumattr.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLColAttributesA(hstmt: super::SQLHSTMT, icol: super::SQLUSMALLINT, fdesctype: super::SQLUSMALLINT, rgbdesc: Option<super::SQLPOINTER>, cbdescmax: super::SQLSMALLINT, pcbdesc: Option<*mut super::SQLSMALLINT>, pfdesc: Option<*mut super::SQLINTEGER>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLColAttributesA(hstmt : super::SQLHSTMT, icol : super::SQLUSMALLINT, fdesctype : super::SQLUSMALLINT, rgbdesc : super::SQLPOINTER, cbdescmax : super::SQLSMALLINT, pcbdesc : *mut super::SQLSMALLINT, pfdesc : *mut super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLColAttributesA(hstmt as _, icol, fdesctype, rgbdesc.unwrap_or(core::mem::zeroed()) as _, cbdescmax, pcbdesc.unwrap_or(core::mem::zeroed()) as _, pfdesc.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLColAttributesA(hstmt: super::SQLHSTMT, icol: super::SQLUSMALLINT, fdesctype: super::SQLUSMALLINT, rgbdesc: Option<super::SQLPOINTER>, cbdescmax: super::SQLSMALLINT, pcbdesc: Option<*mut super::SQLSMALLINT>, pfdesc: Option<*mut super::SQLLEN>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLColAttributesA(hstmt : super::SQLHSTMT, icol : super::SQLUSMALLINT, fdesctype : super::SQLUSMALLINT, rgbdesc : super::SQLPOINTER, cbdescmax : super::SQLSMALLINT, pcbdesc : *mut super::SQLSMALLINT, pfdesc : *mut super::SQLLEN) -> super::SQLRETURN);
    unsafe { SQLColAttributesA(hstmt as _, icol, fdesctype, rgbdesc.unwrap_or(core::mem::zeroed()) as _, cbdescmax, pcbdesc.unwrap_or(core::mem::zeroed()) as _, pfdesc.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLColAttributesW(hstmt: super::SQLHSTMT, icol: super::SQLUSMALLINT, fdesctype: super::SQLUSMALLINT, rgbdesc: Option<super::SQLPOINTER>, cbdescmax: super::SQLSMALLINT, pcbdesc: Option<*mut super::SQLSMALLINT>, pfdesc: Option<*mut super::SQLINTEGER>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLColAttributesW(hstmt : super::SQLHSTMT, icol : super::SQLUSMALLINT, fdesctype : super::SQLUSMALLINT, rgbdesc : super::SQLPOINTER, cbdescmax : super::SQLSMALLINT, pcbdesc : *mut super::SQLSMALLINT, pfdesc : *mut super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLColAttributesW(hstmt as _, icol, fdesctype, rgbdesc.unwrap_or(core::mem::zeroed()) as _, cbdescmax, pcbdesc.unwrap_or(core::mem::zeroed()) as _, pfdesc.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLColAttributesW(hstmt: super::SQLHSTMT, icol: super::SQLUSMALLINT, fdesctype: super::SQLUSMALLINT, rgbdesc: Option<super::SQLPOINTER>, cbdescmax: super::SQLSMALLINT, pcbdesc: Option<*mut super::SQLSMALLINT>, pfdesc: Option<*mut super::SQLLEN>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLColAttributesW(hstmt : super::SQLHSTMT, icol : super::SQLUSMALLINT, fdesctype : super::SQLUSMALLINT, rgbdesc : super::SQLPOINTER, cbdescmax : super::SQLSMALLINT, pcbdesc : *mut super::SQLSMALLINT, pfdesc : *mut super::SQLLEN) -> super::SQLRETURN);
    unsafe { SQLColAttributesW(hstmt as _, icol, fdesctype, rgbdesc.unwrap_or(core::mem::zeroed()) as _, cbdescmax, pcbdesc.unwrap_or(core::mem::zeroed()) as _, pfdesc.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLColumnPrivilegesA(hstmt: super::SQLHSTMT, szcatalogname: Option<*const super::SQLCHAR>, cbcatalogname: super::SQLSMALLINT, szschemaname: Option<*const super::SQLCHAR>, cbschemaname: super::SQLSMALLINT, sztablename: Option<*const super::SQLCHAR>, cbtablename: super::SQLSMALLINT, szcolumnname: Option<*const super::SQLCHAR>, cbcolumnname: super::SQLSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLColumnPrivilegesA(hstmt : super::SQLHSTMT, szcatalogname : *const super::SQLCHAR, cbcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLCHAR, cbschemaname : super::SQLSMALLINT, sztablename : *const super::SQLCHAR, cbtablename : super::SQLSMALLINT, szcolumnname : *const super::SQLCHAR, cbcolumnname : super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLColumnPrivilegesA(hstmt as _, szcatalogname.unwrap_or(core::mem::zeroed()) as _, cbcatalogname, szschemaname.unwrap_or(core::mem::zeroed()) as _, cbschemaname, sztablename.unwrap_or(core::mem::zeroed()) as _, cbtablename, szcolumnname.unwrap_or(core::mem::zeroed()) as _, cbcolumnname) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLColumnPrivilegesW(hstmt: super::SQLHSTMT, szcatalogname: Option<*const super::SQLWCHAR>, cchcatalogname: super::SQLSMALLINT, szschemaname: Option<*const super::SQLWCHAR>, cchschemaname: super::SQLSMALLINT, sztablename: Option<*const super::SQLWCHAR>, cchtablename: super::SQLSMALLINT, szcolumnname: Option<*const super::SQLWCHAR>, cchcolumnname: super::SQLSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLColumnPrivilegesW(hstmt : super::SQLHSTMT, szcatalogname : *const super::SQLWCHAR, cchcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLWCHAR, cchschemaname : super::SQLSMALLINT, sztablename : *const super::SQLWCHAR, cchtablename : super::SQLSMALLINT, szcolumnname : *const super::SQLWCHAR, cchcolumnname : super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLColumnPrivilegesW(hstmt as _, szcatalogname.unwrap_or(core::mem::zeroed()) as _, cchcatalogname, szschemaname.unwrap_or(core::mem::zeroed()) as _, cchschemaname, sztablename.unwrap_or(core::mem::zeroed()) as _, cchtablename, szcolumnname.unwrap_or(core::mem::zeroed()) as _, cchcolumnname) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLColumnsA(hstmt: super::SQLHSTMT, szcatalogname: Option<*const super::SQLCHAR>, cbcatalogname: super::SQLSMALLINT, szschemaname: Option<*const super::SQLCHAR>, cbschemaname: super::SQLSMALLINT, sztablename: Option<*const super::SQLCHAR>, cbtablename: super::SQLSMALLINT, szcolumnname: Option<*const super::SQLCHAR>, cbcolumnname: super::SQLSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLColumnsA(hstmt : super::SQLHSTMT, szcatalogname : *const super::SQLCHAR, cbcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLCHAR, cbschemaname : super::SQLSMALLINT, sztablename : *const super::SQLCHAR, cbtablename : super::SQLSMALLINT, szcolumnname : *const super::SQLCHAR, cbcolumnname : super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLColumnsA(hstmt as _, szcatalogname.unwrap_or(core::mem::zeroed()) as _, cbcatalogname, szschemaname.unwrap_or(core::mem::zeroed()) as _, cbschemaname, sztablename.unwrap_or(core::mem::zeroed()) as _, cbtablename, szcolumnname.unwrap_or(core::mem::zeroed()) as _, cbcolumnname) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLColumnsW(hstmt: super::SQLHSTMT, szcatalogname: Option<*const super::SQLWCHAR>, cchcatalogname: super::SQLSMALLINT, szschemaname: Option<*const super::SQLWCHAR>, cchschemaname: super::SQLSMALLINT, sztablename: Option<*const super::SQLWCHAR>, cchtablename: super::SQLSMALLINT, szcolumnname: Option<*const super::SQLWCHAR>, cchcolumnname: super::SQLSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLColumnsW(hstmt : super::SQLHSTMT, szcatalogname : *const super::SQLWCHAR, cchcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLWCHAR, cchschemaname : super::SQLSMALLINT, sztablename : *const super::SQLWCHAR, cchtablename : super::SQLSMALLINT, szcolumnname : *const super::SQLWCHAR, cchcolumnname : super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLColumnsW(hstmt as _, szcatalogname.unwrap_or(core::mem::zeroed()) as _, cchcatalogname, szschemaname.unwrap_or(core::mem::zeroed()) as _, cchschemaname, sztablename.unwrap_or(core::mem::zeroed()) as _, cchtablename, szcolumnname.unwrap_or(core::mem::zeroed()) as _, cchcolumnname) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLConnectA(hdbc: super::SQLHDBC, szdsn: &[super::SQLCHAR], szuid: &[super::SQLCHAR], szauthstr: &[super::SQLCHAR]) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLConnectA(hdbc : super::SQLHDBC, szdsn : *const super::SQLCHAR, cbdsn : super::SQLSMALLINT, szuid : *const super::SQLCHAR, cbuid : super::SQLSMALLINT, szauthstr : *const super::SQLCHAR, cbauthstr : super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLConnectA(hdbc as _, szdsn.as_ptr(), szdsn.len().try_into().unwrap(), szuid.as_ptr(), szuid.len().try_into().unwrap(), szauthstr.as_ptr(), szauthstr.len().try_into().unwrap()) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLConnectW(hdbc: super::SQLHDBC, szdsn: &[super::SQLWCHAR], szuid: &[super::SQLWCHAR], szauthstr: &[super::SQLWCHAR]) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLConnectW(hdbc : super::SQLHDBC, szdsn : *const super::SQLWCHAR, cchdsn : super::SQLSMALLINT, szuid : *const super::SQLWCHAR, cchuid : super::SQLSMALLINT, szauthstr : *const super::SQLWCHAR, cchauthstr : super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLConnectW(hdbc as _, szdsn.as_ptr(), szdsn.len().try_into().unwrap(), szuid.as_ptr(), szuid.len().try_into().unwrap(), szauthstr.as_ptr(), szauthstr.len().try_into().unwrap()) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLDataSourcesA(henv: super::SQLHENV, fdirection: super::SQLUSMALLINT, szdsn: Option<*mut super::SQLCHAR>, cbdsnmax: super::SQLSMALLINT, pcbdsn: *mut super::SQLSMALLINT, szdescription: Option<*mut super::SQLCHAR>, cbdescriptionmax: super::SQLSMALLINT, pcbdescription: *mut super::SQLSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLDataSourcesA(henv : super::SQLHENV, fdirection : super::SQLUSMALLINT, szdsn : *mut super::SQLCHAR, cbdsnmax : super::SQLSMALLINT, pcbdsn : *mut super::SQLSMALLINT, szdescription : *mut super::SQLCHAR, cbdescriptionmax : super::SQLSMALLINT, pcbdescription : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLDataSourcesA(henv as _, fdirection, szdsn.unwrap_or(core::mem::zeroed()) as _, cbdsnmax, pcbdsn as _, szdescription.unwrap_or(core::mem::zeroed()) as _, cbdescriptionmax, pcbdescription as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLDataSourcesW(henv: super::SQLHENV, fdirection: super::SQLUSMALLINT, szdsn: Option<*mut super::SQLWCHAR>, cchdsnmax: super::SQLSMALLINT, pcchdsn: Option<*mut super::SQLSMALLINT>, wszdescription: Option<*mut super::SQLWCHAR>, cchdescriptionmax: super::SQLSMALLINT, pcchdescription: Option<*mut super::SQLSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLDataSourcesW(henv : super::SQLHENV, fdirection : super::SQLUSMALLINT, szdsn : *mut super::SQLWCHAR, cchdsnmax : super::SQLSMALLINT, pcchdsn : *mut super::SQLSMALLINT, wszdescription : *mut super::SQLWCHAR, cchdescriptionmax : super::SQLSMALLINT, pcchdescription : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLDataSourcesW(henv as _, fdirection, szdsn.unwrap_or(core::mem::zeroed()) as _, cchdsnmax, pcchdsn.unwrap_or(core::mem::zeroed()) as _, wszdescription.unwrap_or(core::mem::zeroed()) as _, cchdescriptionmax, pcchdescription.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLDescribeColA(hstmt: super::SQLHSTMT, icol: super::SQLUSMALLINT, szcolname: Option<*mut super::SQLCHAR>, cbcolnamemax: super::SQLSMALLINT, pcbcolname: Option<*mut super::SQLSMALLINT>, pfsqltype: Option<*mut super::SQLSMALLINT>, pcbcoldef: Option<*mut super::SQLUINTEGER>, pibscale: Option<*mut super::SQLSMALLINT>, pfnullable: Option<*mut super::SQLSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLDescribeColA(hstmt : super::SQLHSTMT, icol : super::SQLUSMALLINT, szcolname : *mut super::SQLCHAR, cbcolnamemax : super::SQLSMALLINT, pcbcolname : *mut super::SQLSMALLINT, pfsqltype : *mut super::SQLSMALLINT, pcbcoldef : *mut super::SQLUINTEGER, pibscale : *mut super::SQLSMALLINT, pfnullable : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLDescribeColA(hstmt as _, icol, szcolname.unwrap_or(core::mem::zeroed()) as _, cbcolnamemax, pcbcolname.unwrap_or(core::mem::zeroed()) as _, pfsqltype.unwrap_or(core::mem::zeroed()) as _, pcbcoldef.unwrap_or(core::mem::zeroed()) as _, pibscale.unwrap_or(core::mem::zeroed()) as _, pfnullable.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLDescribeColA(hstmt: super::SQLHSTMT, icol: super::SQLUSMALLINT, szcolname: Option<*mut super::SQLCHAR>, cbcolnamemax: super::SQLSMALLINT, pcbcolname: Option<*mut super::SQLSMALLINT>, pfsqltype: Option<*mut super::SQLSMALLINT>, pcbcoldef: Option<*mut super::SQLULEN>, pibscale: Option<*mut super::SQLSMALLINT>, pfnullable: Option<*mut super::SQLSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLDescribeColA(hstmt : super::SQLHSTMT, icol : super::SQLUSMALLINT, szcolname : *mut super::SQLCHAR, cbcolnamemax : super::SQLSMALLINT, pcbcolname : *mut super::SQLSMALLINT, pfsqltype : *mut super::SQLSMALLINT, pcbcoldef : *mut super::SQLULEN, pibscale : *mut super::SQLSMALLINT, pfnullable : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLDescribeColA(hstmt as _, icol, szcolname.unwrap_or(core::mem::zeroed()) as _, cbcolnamemax, pcbcolname.unwrap_or(core::mem::zeroed()) as _, pfsqltype.unwrap_or(core::mem::zeroed()) as _, pcbcoldef.unwrap_or(core::mem::zeroed()) as _, pibscale.unwrap_or(core::mem::zeroed()) as _, pfnullable.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLDescribeColW(hstmt: super::SQLHSTMT, icol: super::SQLUSMALLINT, szcolname: Option<*mut super::SQLWCHAR>, cchcolnamemax: super::SQLSMALLINT, pcchcolname: Option<*mut super::SQLSMALLINT>, pfsqltype: Option<*mut super::SQLSMALLINT>, pcbcoldef: Option<*mut super::SQLUINTEGER>, pibscale: Option<*mut super::SQLSMALLINT>, pfnullable: Option<*mut super::SQLSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLDescribeColW(hstmt : super::SQLHSTMT, icol : super::SQLUSMALLINT, szcolname : *mut super::SQLWCHAR, cchcolnamemax : super::SQLSMALLINT, pcchcolname : *mut super::SQLSMALLINT, pfsqltype : *mut super::SQLSMALLINT, pcbcoldef : *mut super::SQLUINTEGER, pibscale : *mut super::SQLSMALLINT, pfnullable : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLDescribeColW(hstmt as _, icol, szcolname.unwrap_or(core::mem::zeroed()) as _, cchcolnamemax, pcchcolname.unwrap_or(core::mem::zeroed()) as _, pfsqltype.unwrap_or(core::mem::zeroed()) as _, pcbcoldef.unwrap_or(core::mem::zeroed()) as _, pibscale.unwrap_or(core::mem::zeroed()) as _, pfnullable.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLDescribeColW(hstmt: super::SQLHSTMT, icol: super::SQLUSMALLINT, szcolname: Option<*mut super::SQLWCHAR>, cchcolnamemax: super::SQLSMALLINT, pcchcolname: Option<*mut super::SQLSMALLINT>, pfsqltype: Option<*mut super::SQLSMALLINT>, pcbcoldef: Option<*mut super::SQLULEN>, pibscale: Option<*mut super::SQLSMALLINT>, pfnullable: Option<*mut super::SQLSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLDescribeColW(hstmt : super::SQLHSTMT, icol : super::SQLUSMALLINT, szcolname : *mut super::SQLWCHAR, cchcolnamemax : super::SQLSMALLINT, pcchcolname : *mut super::SQLSMALLINT, pfsqltype : *mut super::SQLSMALLINT, pcbcoldef : *mut super::SQLULEN, pibscale : *mut super::SQLSMALLINT, pfnullable : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLDescribeColW(hstmt as _, icol, szcolname.unwrap_or(core::mem::zeroed()) as _, cchcolnamemax, pcchcolname.unwrap_or(core::mem::zeroed()) as _, pfsqltype.unwrap_or(core::mem::zeroed()) as _, pcbcoldef.unwrap_or(core::mem::zeroed()) as _, pibscale.unwrap_or(core::mem::zeroed()) as _, pfnullable.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "sqltypes", feature = "windef"))]
#[inline]
pub unsafe fn SQLDriverConnectA(hdbc: super::SQLHDBC, hwnd: super::SQLHWND, szconnstrin: &[super::SQLCHAR], szconnstrout: Option<*mut super::SQLCHAR>, cbconnstroutmax: super::SQLSMALLINT, pcbconnstrout: Option<*mut super::SQLSMALLINT>, fdrivercompletion: super::SQLUSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLDriverConnectA(hdbc : super::SQLHDBC, hwnd : super::SQLHWND, szconnstrin : *const super::SQLCHAR, cbconnstrin : super::SQLSMALLINT, szconnstrout : *mut super::SQLCHAR, cbconnstroutmax : super::SQLSMALLINT, pcbconnstrout : *mut super::SQLSMALLINT, fdrivercompletion : super::SQLUSMALLINT) -> super::SQLRETURN);
    unsafe { SQLDriverConnectA(hdbc as _, hwnd as _, szconnstrin.as_ptr(), szconnstrin.len().try_into().unwrap(), szconnstrout.unwrap_or(core::mem::zeroed()) as _, cbconnstroutmax, pcbconnstrout.unwrap_or(core::mem::zeroed()) as _, fdrivercompletion) }
}
#[cfg(all(feature = "sqltypes", feature = "windef"))]
#[inline]
pub unsafe fn SQLDriverConnectW(hdbc: super::SQLHDBC, hwnd: super::SQLHWND, szconnstrin: &[super::SQLWCHAR], szconnstrout: Option<*mut super::SQLWCHAR>, cchconnstroutmax: super::SQLSMALLINT, pcchconnstrout: Option<*mut super::SQLSMALLINT>, fdrivercompletion: super::SQLUSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLDriverConnectW(hdbc : super::SQLHDBC, hwnd : super::SQLHWND, szconnstrin : *const super::SQLWCHAR, cchconnstrin : super::SQLSMALLINT, szconnstrout : *mut super::SQLWCHAR, cchconnstroutmax : super::SQLSMALLINT, pcchconnstrout : *mut super::SQLSMALLINT, fdrivercompletion : super::SQLUSMALLINT) -> super::SQLRETURN);
    unsafe { SQLDriverConnectW(hdbc as _, hwnd as _, szconnstrin.as_ptr(), szconnstrin.len().try_into().unwrap(), szconnstrout.unwrap_or(core::mem::zeroed()) as _, cchconnstroutmax, pcchconnstrout.unwrap_or(core::mem::zeroed()) as _, fdrivercompletion) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLDriversA(henv: super::SQLHENV, fdirection: super::SQLUSMALLINT, szdriverdesc: Option<*mut super::SQLCHAR>, cbdriverdescmax: super::SQLSMALLINT, pcbdriverdesc: Option<*mut super::SQLSMALLINT>, szdriverattributes: Option<*mut super::SQLCHAR>, cbdrvrattrmax: super::SQLSMALLINT, pcbdrvrattr: Option<*mut super::SQLSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLDriversA(henv : super::SQLHENV, fdirection : super::SQLUSMALLINT, szdriverdesc : *mut super::SQLCHAR, cbdriverdescmax : super::SQLSMALLINT, pcbdriverdesc : *mut super::SQLSMALLINT, szdriverattributes : *mut super::SQLCHAR, cbdrvrattrmax : super::SQLSMALLINT, pcbdrvrattr : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLDriversA(henv as _, fdirection, szdriverdesc.unwrap_or(core::mem::zeroed()) as _, cbdriverdescmax, pcbdriverdesc.unwrap_or(core::mem::zeroed()) as _, szdriverattributes.unwrap_or(core::mem::zeroed()) as _, cbdrvrattrmax, pcbdrvrattr.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLDriversW(henv: super::SQLHENV, fdirection: super::SQLUSMALLINT, szdriverdesc: Option<*mut super::SQLWCHAR>, cchdriverdescmax: super::SQLSMALLINT, pcchdriverdesc: Option<*mut super::SQLSMALLINT>, szdriverattributes: Option<*mut super::SQLWCHAR>, cchdrvrattrmax: super::SQLSMALLINT, pcchdrvrattr: Option<*mut super::SQLSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLDriversW(henv : super::SQLHENV, fdirection : super::SQLUSMALLINT, szdriverdesc : *mut super::SQLWCHAR, cchdriverdescmax : super::SQLSMALLINT, pcchdriverdesc : *mut super::SQLSMALLINT, szdriverattributes : *mut super::SQLWCHAR, cchdrvrattrmax : super::SQLSMALLINT, pcchdrvrattr : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLDriversW(henv as _, fdirection, szdriverdesc.unwrap_or(core::mem::zeroed()) as _, cchdriverdescmax, pcchdriverdesc.unwrap_or(core::mem::zeroed()) as _, szdriverattributes.unwrap_or(core::mem::zeroed()) as _, cchdrvrattrmax, pcchdrvrattr.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLErrorA(henv: super::SQLHENV, hdbc: super::SQLHDBC, hstmt: super::SQLHSTMT, szsqlstate: *mut super::SQLCHAR, pfnativeerror: Option<*mut super::SQLINTEGER>, szerrormsg: Option<*mut super::SQLCHAR>, cberrormsgmax: super::SQLSMALLINT, pcberrormsg: Option<*mut super::SQLSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLErrorA(henv : super::SQLHENV, hdbc : super::SQLHDBC, hstmt : super::SQLHSTMT, szsqlstate : *mut super::SQLCHAR, pfnativeerror : *mut super::SQLINTEGER, szerrormsg : *mut super::SQLCHAR, cberrormsgmax : super::SQLSMALLINT, pcberrormsg : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLErrorA(henv as _, hdbc as _, hstmt as _, szsqlstate as _, pfnativeerror.unwrap_or(core::mem::zeroed()) as _, szerrormsg.unwrap_or(core::mem::zeroed()) as _, cberrormsgmax, pcberrormsg.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLErrorW(henv: super::SQLHENV, hdbc: super::SQLHDBC, hstmt: super::SQLHSTMT, wszsqlstate: *mut super::SQLWCHAR, pfnativeerror: Option<*mut super::SQLINTEGER>, wszerrormsg: Option<*mut super::SQLWCHAR>, ccherrormsgmax: super::SQLSMALLINT, pccherrormsg: Option<*mut super::SQLSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLErrorW(henv : super::SQLHENV, hdbc : super::SQLHDBC, hstmt : super::SQLHSTMT, wszsqlstate : *mut super::SQLWCHAR, pfnativeerror : *mut super::SQLINTEGER, wszerrormsg : *mut super::SQLWCHAR, ccherrormsgmax : super::SQLSMALLINT, pccherrormsg : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLErrorW(henv as _, hdbc as _, hstmt as _, wszsqlstate as _, pfnativeerror.unwrap_or(core::mem::zeroed()) as _, wszerrormsg.unwrap_or(core::mem::zeroed()) as _, ccherrormsgmax, pccherrormsg.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLExecDirectA(hstmt: super::SQLHSTMT, szsqlstr: Option<*const super::SQLCHAR>, cbsqlstr: super::SQLINTEGER) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLExecDirectA(hstmt : super::SQLHSTMT, szsqlstr : *const super::SQLCHAR, cbsqlstr : super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLExecDirectA(hstmt as _, szsqlstr.unwrap_or(core::mem::zeroed()) as _, cbsqlstr) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLExecDirectW(hstmt: super::SQLHSTMT, szsqlstr: Option<*const super::SQLWCHAR>, textlength: super::SQLINTEGER) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLExecDirectW(hstmt : super::SQLHSTMT, szsqlstr : *const super::SQLWCHAR, textlength : super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLExecDirectW(hstmt as _, szsqlstr.unwrap_or(core::mem::zeroed()) as _, textlength) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLForeignKeysA(hstmt: super::SQLHSTMT, szpkcatalogname: Option<*const super::SQLCHAR>, cbpkcatalogname: super::SQLSMALLINT, szpkschemaname: Option<*const super::SQLCHAR>, cbpkschemaname: super::SQLSMALLINT, szpktablename: Option<*const super::SQLCHAR>, cbpktablename: super::SQLSMALLINT, szfkcatalogname: Option<*const super::SQLCHAR>, cbfkcatalogname: super::SQLSMALLINT, szfkschemaname: Option<*const super::SQLCHAR>, cbfkschemaname: super::SQLSMALLINT, szfktablename: Option<*const super::SQLCHAR>, cbfktablename: super::SQLSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLForeignKeysA(hstmt : super::SQLHSTMT, szpkcatalogname : *const super::SQLCHAR, cbpkcatalogname : super::SQLSMALLINT, szpkschemaname : *const super::SQLCHAR, cbpkschemaname : super::SQLSMALLINT, szpktablename : *const super::SQLCHAR, cbpktablename : super::SQLSMALLINT, szfkcatalogname : *const super::SQLCHAR, cbfkcatalogname : super::SQLSMALLINT, szfkschemaname : *const super::SQLCHAR, cbfkschemaname : super::SQLSMALLINT, szfktablename : *const super::SQLCHAR, cbfktablename : super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLForeignKeysA(hstmt as _, szpkcatalogname.unwrap_or(core::mem::zeroed()) as _, cbpkcatalogname, szpkschemaname.unwrap_or(core::mem::zeroed()) as _, cbpkschemaname, szpktablename.unwrap_or(core::mem::zeroed()) as _, cbpktablename, szfkcatalogname.unwrap_or(core::mem::zeroed()) as _, cbfkcatalogname, szfkschemaname.unwrap_or(core::mem::zeroed()) as _, cbfkschemaname, szfktablename.unwrap_or(core::mem::zeroed()) as _, cbfktablename) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLForeignKeysW(hstmt: super::SQLHSTMT, szpkcatalogname: Option<*const super::SQLWCHAR>, cchpkcatalogname: super::SQLSMALLINT, szpkschemaname: Option<*const super::SQLWCHAR>, cchpkschemaname: super::SQLSMALLINT, szpktablename: Option<*const super::SQLWCHAR>, cchpktablename: super::SQLSMALLINT, szfkcatalogname: Option<*const super::SQLWCHAR>, cchfkcatalogname: super::SQLSMALLINT, szfkschemaname: Option<*const super::SQLWCHAR>, cchfkschemaname: super::SQLSMALLINT, szfktablename: Option<*const super::SQLWCHAR>, cchfktablename: super::SQLSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLForeignKeysW(hstmt : super::SQLHSTMT, szpkcatalogname : *const super::SQLWCHAR, cchpkcatalogname : super::SQLSMALLINT, szpkschemaname : *const super::SQLWCHAR, cchpkschemaname : super::SQLSMALLINT, szpktablename : *const super::SQLWCHAR, cchpktablename : super::SQLSMALLINT, szfkcatalogname : *const super::SQLWCHAR, cchfkcatalogname : super::SQLSMALLINT, szfkschemaname : *const super::SQLWCHAR, cchfkschemaname : super::SQLSMALLINT, szfktablename : *const super::SQLWCHAR, cchfktablename : super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLForeignKeysW(hstmt as _, szpkcatalogname.unwrap_or(core::mem::zeroed()) as _, cchpkcatalogname, szpkschemaname.unwrap_or(core::mem::zeroed()) as _, cchpkschemaname, szpktablename.unwrap_or(core::mem::zeroed()) as _, cchpktablename, szfkcatalogname.unwrap_or(core::mem::zeroed()) as _, cchfkcatalogname, szfkschemaname.unwrap_or(core::mem::zeroed()) as _, cchfkschemaname, szfktablename.unwrap_or(core::mem::zeroed()) as _, cchfktablename) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetConnectAttrA(hdbc: super::SQLHDBC, fattribute: super::SQLINTEGER, rgbvalue: Option<super::SQLPOINTER>, cbvaluemax: super::SQLINTEGER, pcbvalue: Option<*mut super::SQLINTEGER>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetConnectAttrA(hdbc : super::SQLHDBC, fattribute : super::SQLINTEGER, rgbvalue : super::SQLPOINTER, cbvaluemax : super::SQLINTEGER, pcbvalue : *mut super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLGetConnectAttrA(hdbc as _, fattribute, rgbvalue.unwrap_or(core::mem::zeroed()) as _, cbvaluemax, pcbvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetConnectAttrW(hdbc: super::SQLHDBC, fattribute: super::SQLINTEGER, rgbvalue: Option<super::SQLPOINTER>, cbvaluemax: super::SQLINTEGER, pcbvalue: Option<*mut super::SQLINTEGER>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetConnectAttrW(hdbc : super::SQLHDBC, fattribute : super::SQLINTEGER, rgbvalue : super::SQLPOINTER, cbvaluemax : super::SQLINTEGER, pcbvalue : *mut super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLGetConnectAttrW(hdbc as _, fattribute, rgbvalue.unwrap_or(core::mem::zeroed()) as _, cbvaluemax, pcbvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetConnectOptionA(hdbc: super::SQLHDBC, foption: super::SQLUSMALLINT, pvparam: super::SQLPOINTER) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetConnectOptionA(hdbc : super::SQLHDBC, foption : super::SQLUSMALLINT, pvparam : super::SQLPOINTER) -> super::SQLRETURN);
    unsafe { SQLGetConnectOptionA(hdbc as _, foption, pvparam as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetConnectOptionW(hdbc: super::SQLHDBC, foption: super::SQLUSMALLINT, pvparam: super::SQLPOINTER) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetConnectOptionW(hdbc : super::SQLHDBC, foption : super::SQLUSMALLINT, pvparam : super::SQLPOINTER) -> super::SQLRETURN);
    unsafe { SQLGetConnectOptionW(hdbc as _, foption, pvparam as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetCursorNameA(hstmt: super::SQLHSTMT, szcursor: Option<*mut super::SQLCHAR>, cbcursormax: super::SQLSMALLINT, pcbcursor: Option<*mut super::SQLSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetCursorNameA(hstmt : super::SQLHSTMT, szcursor : *mut super::SQLCHAR, cbcursormax : super::SQLSMALLINT, pcbcursor : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLGetCursorNameA(hstmt as _, szcursor.unwrap_or(core::mem::zeroed()) as _, cbcursormax, pcbcursor.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetCursorNameW(hstmt: super::SQLHSTMT, szcursor: Option<*mut super::SQLWCHAR>, cchcursormax: super::SQLSMALLINT, pcchcursor: Option<*mut super::SQLSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetCursorNameW(hstmt : super::SQLHSTMT, szcursor : *mut super::SQLWCHAR, cchcursormax : super::SQLSMALLINT, pcchcursor : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLGetCursorNameW(hstmt as _, szcursor.unwrap_or(core::mem::zeroed()) as _, cchcursormax, pcchcursor.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetDescFieldA(hdesc: super::SQLHDESC, irecord: super::SQLSMALLINT, ifield: super::SQLSMALLINT, rgbvalue: Option<super::SQLPOINTER>, cbbufferlength: super::SQLINTEGER, stringlength: Option<*mut super::SQLINTEGER>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetDescFieldA(hdesc : super::SQLHDESC, irecord : super::SQLSMALLINT, ifield : super::SQLSMALLINT, rgbvalue : super::SQLPOINTER, cbbufferlength : super::SQLINTEGER, stringlength : *mut super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLGetDescFieldA(hdesc as _, irecord, ifield, rgbvalue.unwrap_or(core::mem::zeroed()) as _, cbbufferlength, stringlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetDescFieldW(hdesc: super::SQLHDESC, irecord: super::SQLSMALLINT, ifield: super::SQLSMALLINT, rgbvalue: Option<super::SQLPOINTER>, cbbufferlength: super::SQLINTEGER, stringlength: Option<*mut super::SQLINTEGER>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetDescFieldW(hdesc : super::SQLHDESC, irecord : super::SQLSMALLINT, ifield : super::SQLSMALLINT, rgbvalue : super::SQLPOINTER, cbbufferlength : super::SQLINTEGER, stringlength : *mut super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLGetDescFieldW(hdesc as _, irecord, ifield, rgbvalue.unwrap_or(core::mem::zeroed()) as _, cbbufferlength, stringlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetDescRecA(hdesc: super::SQLHDESC, irecord: super::SQLSMALLINT, szname: Option<*mut super::SQLCHAR>, cbnamemax: super::SQLSMALLINT, pcbname: Option<*mut super::SQLSMALLINT>, pftype: Option<*mut super::SQLSMALLINT>, pfsubtype: Option<*mut super::SQLSMALLINT>, plength: Option<*mut super::SQLINTEGER>, pprecision: Option<*mut super::SQLSMALLINT>, pscale: Option<*mut super::SQLSMALLINT>, pnullable: Option<*mut super::SQLSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetDescRecA(hdesc : super::SQLHDESC, irecord : super::SQLSMALLINT, szname : *mut super::SQLCHAR, cbnamemax : super::SQLSMALLINT, pcbname : *mut super::SQLSMALLINT, pftype : *mut super::SQLSMALLINT, pfsubtype : *mut super::SQLSMALLINT, plength : *mut super::SQLINTEGER, pprecision : *mut super::SQLSMALLINT, pscale : *mut super::SQLSMALLINT, pnullable : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLGetDescRecA(hdesc as _, irecord, szname.unwrap_or(core::mem::zeroed()) as _, cbnamemax, pcbname.unwrap_or(core::mem::zeroed()) as _, pftype.unwrap_or(core::mem::zeroed()) as _, pfsubtype.unwrap_or(core::mem::zeroed()) as _, plength.unwrap_or(core::mem::zeroed()) as _, pprecision.unwrap_or(core::mem::zeroed()) as _, pscale.unwrap_or(core::mem::zeroed()) as _, pnullable.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetDescRecA(hdesc: super::SQLHDESC, irecord: super::SQLSMALLINT, szname: Option<*mut super::SQLCHAR>, cbnamemax: super::SQLSMALLINT, pcbname: Option<*mut super::SQLSMALLINT>, pftype: Option<*mut super::SQLSMALLINT>, pfsubtype: Option<*mut super::SQLSMALLINT>, plength: Option<*mut super::SQLLEN>, pprecision: Option<*mut super::SQLSMALLINT>, pscale: Option<*mut super::SQLSMALLINT>, pnullable: Option<*mut super::SQLSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetDescRecA(hdesc : super::SQLHDESC, irecord : super::SQLSMALLINT, szname : *mut super::SQLCHAR, cbnamemax : super::SQLSMALLINT, pcbname : *mut super::SQLSMALLINT, pftype : *mut super::SQLSMALLINT, pfsubtype : *mut super::SQLSMALLINT, plength : *mut super::SQLLEN, pprecision : *mut super::SQLSMALLINT, pscale : *mut super::SQLSMALLINT, pnullable : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLGetDescRecA(hdesc as _, irecord, szname.unwrap_or(core::mem::zeroed()) as _, cbnamemax, pcbname.unwrap_or(core::mem::zeroed()) as _, pftype.unwrap_or(core::mem::zeroed()) as _, pfsubtype.unwrap_or(core::mem::zeroed()) as _, plength.unwrap_or(core::mem::zeroed()) as _, pprecision.unwrap_or(core::mem::zeroed()) as _, pscale.unwrap_or(core::mem::zeroed()) as _, pnullable.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetDescRecW(hdesc: super::SQLHDESC, irecord: super::SQLSMALLINT, szname: Option<*mut super::SQLWCHAR>, cchnamemax: super::SQLSMALLINT, pcchname: Option<*mut super::SQLSMALLINT>, pftype: Option<*mut super::SQLSMALLINT>, pfsubtype: Option<*mut super::SQLSMALLINT>, plength: Option<*mut super::SQLINTEGER>, pprecision: Option<*mut super::SQLSMALLINT>, pscale: Option<*mut super::SQLSMALLINT>, pnullable: Option<*mut super::SQLSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetDescRecW(hdesc : super::SQLHDESC, irecord : super::SQLSMALLINT, szname : *mut super::SQLWCHAR, cchnamemax : super::SQLSMALLINT, pcchname : *mut super::SQLSMALLINT, pftype : *mut super::SQLSMALLINT, pfsubtype : *mut super::SQLSMALLINT, plength : *mut super::SQLINTEGER, pprecision : *mut super::SQLSMALLINT, pscale : *mut super::SQLSMALLINT, pnullable : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLGetDescRecW(hdesc as _, irecord, szname.unwrap_or(core::mem::zeroed()) as _, cchnamemax, pcchname.unwrap_or(core::mem::zeroed()) as _, pftype.unwrap_or(core::mem::zeroed()) as _, pfsubtype.unwrap_or(core::mem::zeroed()) as _, plength.unwrap_or(core::mem::zeroed()) as _, pprecision.unwrap_or(core::mem::zeroed()) as _, pscale.unwrap_or(core::mem::zeroed()) as _, pnullable.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetDescRecW(hdesc: super::SQLHDESC, irecord: super::SQLSMALLINT, szname: Option<*mut super::SQLWCHAR>, cchnamemax: super::SQLSMALLINT, pcchname: Option<*mut super::SQLSMALLINT>, pftype: Option<*mut super::SQLSMALLINT>, pfsubtype: Option<*mut super::SQLSMALLINT>, plength: Option<*mut super::SQLLEN>, pprecision: Option<*mut super::SQLSMALLINT>, pscale: Option<*mut super::SQLSMALLINT>, pnullable: Option<*mut super::SQLSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetDescRecW(hdesc : super::SQLHDESC, irecord : super::SQLSMALLINT, szname : *mut super::SQLWCHAR, cchnamemax : super::SQLSMALLINT, pcchname : *mut super::SQLSMALLINT, pftype : *mut super::SQLSMALLINT, pfsubtype : *mut super::SQLSMALLINT, plength : *mut super::SQLLEN, pprecision : *mut super::SQLSMALLINT, pscale : *mut super::SQLSMALLINT, pnullable : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLGetDescRecW(hdesc as _, irecord, szname.unwrap_or(core::mem::zeroed()) as _, cchnamemax, pcchname.unwrap_or(core::mem::zeroed()) as _, pftype.unwrap_or(core::mem::zeroed()) as _, pfsubtype.unwrap_or(core::mem::zeroed()) as _, plength.unwrap_or(core::mem::zeroed()) as _, pprecision.unwrap_or(core::mem::zeroed()) as _, pscale.unwrap_or(core::mem::zeroed()) as _, pnullable.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetDiagFieldA(fhandletype: super::SQLSMALLINT, handle: super::SQLHANDLE, irecord: super::SQLSMALLINT, fdiagfield: super::SQLSMALLINT, rgbdiaginfo: Option<super::SQLPOINTER>, cbdiaginfomax: super::SQLSMALLINT, pcbdiaginfo: Option<*mut super::SQLSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetDiagFieldA(fhandletype : super::SQLSMALLINT, handle : super::SQLHANDLE, irecord : super::SQLSMALLINT, fdiagfield : super::SQLSMALLINT, rgbdiaginfo : super::SQLPOINTER, cbdiaginfomax : super::SQLSMALLINT, pcbdiaginfo : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLGetDiagFieldA(fhandletype, handle as _, irecord, fdiagfield, rgbdiaginfo.unwrap_or(core::mem::zeroed()) as _, cbdiaginfomax, pcbdiaginfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetDiagFieldW(fhandletype: super::SQLSMALLINT, handle: super::SQLHANDLE, irecord: super::SQLSMALLINT, fdiagfield: super::SQLSMALLINT, rgbdiaginfo: Option<super::SQLPOINTER>, cbbufferlength: super::SQLSMALLINT, pcbstringlength: Option<*mut super::SQLSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetDiagFieldW(fhandletype : super::SQLSMALLINT, handle : super::SQLHANDLE, irecord : super::SQLSMALLINT, fdiagfield : super::SQLSMALLINT, rgbdiaginfo : super::SQLPOINTER, cbbufferlength : super::SQLSMALLINT, pcbstringlength : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLGetDiagFieldW(fhandletype, handle as _, irecord, fdiagfield, rgbdiaginfo.unwrap_or(core::mem::zeroed()) as _, cbbufferlength, pcbstringlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetDiagRecA(fhandletype: super::SQLSMALLINT, handle: super::SQLHANDLE, irecord: super::SQLSMALLINT, szsqlstate: Option<*mut super::SQLCHAR>, pfnativeerror: *mut super::SQLINTEGER, szerrormsg: Option<*mut super::SQLCHAR>, cberrormsgmax: super::SQLSMALLINT, pcberrormsg: *mut super::SQLSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetDiagRecA(fhandletype : super::SQLSMALLINT, handle : super::SQLHANDLE, irecord : super::SQLSMALLINT, szsqlstate : *mut super::SQLCHAR, pfnativeerror : *mut super::SQLINTEGER, szerrormsg : *mut super::SQLCHAR, cberrormsgmax : super::SQLSMALLINT, pcberrormsg : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLGetDiagRecA(fhandletype, handle as _, irecord, szsqlstate.unwrap_or(core::mem::zeroed()) as _, pfnativeerror as _, szerrormsg.unwrap_or(core::mem::zeroed()) as _, cberrormsgmax, pcberrormsg as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetDiagRecW(fhandletype: super::SQLSMALLINT, handle: super::SQLHANDLE, irecord: super::SQLSMALLINT, szsqlstate: Option<*mut super::SQLWCHAR>, pfnativeerror: *mut super::SQLINTEGER, szerrormsg: Option<*mut super::SQLWCHAR>, ccherrormsgmax: super::SQLSMALLINT, pccherrormsg: *mut super::SQLSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetDiagRecW(fhandletype : super::SQLSMALLINT, handle : super::SQLHANDLE, irecord : super::SQLSMALLINT, szsqlstate : *mut super::SQLWCHAR, pfnativeerror : *mut super::SQLINTEGER, szerrormsg : *mut super::SQLWCHAR, ccherrormsgmax : super::SQLSMALLINT, pccherrormsg : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLGetDiagRecW(fhandletype, handle as _, irecord, szsqlstate.unwrap_or(core::mem::zeroed()) as _, pfnativeerror as _, szerrormsg.unwrap_or(core::mem::zeroed()) as _, ccherrormsgmax, pccherrormsg as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetInfoA(hdbc: super::SQLHDBC, finfotype: super::SQLUSMALLINT, rgbinfovalue: Option<super::SQLPOINTER>, cbinfovaluemax: super::SQLSMALLINT, pcbinfovalue: Option<*mut super::SQLSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetInfoA(hdbc : super::SQLHDBC, finfotype : super::SQLUSMALLINT, rgbinfovalue : super::SQLPOINTER, cbinfovaluemax : super::SQLSMALLINT, pcbinfovalue : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLGetInfoA(hdbc as _, finfotype, rgbinfovalue.unwrap_or(core::mem::zeroed()) as _, cbinfovaluemax, pcbinfovalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetInfoW(hdbc: super::SQLHDBC, finfotype: super::SQLUSMALLINT, rgbinfovalue: Option<super::SQLPOINTER>, cbinfovaluemax: super::SQLSMALLINT, pcbinfovalue: Option<*mut super::SQLSMALLINT>) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetInfoW(hdbc : super::SQLHDBC, finfotype : super::SQLUSMALLINT, rgbinfovalue : super::SQLPOINTER, cbinfovaluemax : super::SQLSMALLINT, pcbinfovalue : *mut super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLGetInfoW(hdbc as _, finfotype, rgbinfovalue.unwrap_or(core::mem::zeroed()) as _, cbinfovaluemax, pcbinfovalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetStmtAttrA(hstmt: super::SQLHSTMT, fattribute: super::SQLINTEGER, rgbvalue: super::SQLPOINTER, cbvaluemax: super::SQLINTEGER, pcbvalue: *mut super::SQLINTEGER) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetStmtAttrA(hstmt : super::SQLHSTMT, fattribute : super::SQLINTEGER, rgbvalue : super::SQLPOINTER, cbvaluemax : super::SQLINTEGER, pcbvalue : *mut super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLGetStmtAttrA(hstmt as _, fattribute, rgbvalue as _, cbvaluemax, pcbvalue as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetStmtAttrW(hstmt: super::SQLHSTMT, fattribute: super::SQLINTEGER, rgbvalue: super::SQLPOINTER, cbvaluemax: super::SQLINTEGER, pcbvalue: *mut super::SQLINTEGER) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetStmtAttrW(hstmt : super::SQLHSTMT, fattribute : super::SQLINTEGER, rgbvalue : super::SQLPOINTER, cbvaluemax : super::SQLINTEGER, pcbvalue : *mut super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLGetStmtAttrW(hstmt as _, fattribute, rgbvalue as _, cbvaluemax, pcbvalue as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetTypeInfoA(statementhandle: super::SQLHSTMT, datatype: super::SQLSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetTypeInfoA(statementhandle : super::SQLHSTMT, datatype : super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLGetTypeInfoA(statementhandle as _, datatype) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLGetTypeInfoW(statementhandle: super::SQLHSTMT, datatype: super::SQLSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLGetTypeInfoW(statementhandle : super::SQLHSTMT, datatype : super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLGetTypeInfoW(statementhandle as _, datatype) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLNativeSqlA(hdbc: super::SQLHDBC, szsqlstrin: &[super::SQLCHAR], szsqlstr: Option<*mut super::SQLCHAR>, cbsqlstrmax: super::SQLINTEGER, pcbsqlstr: *mut super::SQLINTEGER) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLNativeSqlA(hdbc : super::SQLHDBC, szsqlstrin : *const super::SQLCHAR, cbsqlstrin : super::SQLINTEGER, szsqlstr : *mut super::SQLCHAR, cbsqlstrmax : super::SQLINTEGER, pcbsqlstr : *mut super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLNativeSqlA(hdbc as _, szsqlstrin.as_ptr(), szsqlstrin.len().try_into().unwrap(), szsqlstr.unwrap_or(core::mem::zeroed()) as _, cbsqlstrmax, pcbsqlstr as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLNativeSqlW(hdbc: super::SQLHDBC, szsqlstrin: &[super::SQLWCHAR], szsqlstr: Option<*mut super::SQLWCHAR>, cchsqlstrmax: super::SQLINTEGER, pcchsqlstr: *mut super::SQLINTEGER) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLNativeSqlW(hdbc : super::SQLHDBC, szsqlstrin : *const super::SQLWCHAR, cchsqlstrin : super::SQLINTEGER, szsqlstr : *mut super::SQLWCHAR, cchsqlstrmax : super::SQLINTEGER, pcchsqlstr : *mut super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLNativeSqlW(hdbc as _, szsqlstrin.as_ptr(), szsqlstrin.len().try_into().unwrap(), szsqlstr.unwrap_or(core::mem::zeroed()) as _, cchsqlstrmax, pcchsqlstr as _) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLPrepareA(hstmt: super::SQLHSTMT, szsqlstr: &[super::SQLCHAR]) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLPrepareA(hstmt : super::SQLHSTMT, szsqlstr : *const super::SQLCHAR, cbsqlstr : super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLPrepareA(hstmt as _, szsqlstr.as_ptr(), szsqlstr.len().try_into().unwrap()) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLPrepareW(hstmt: super::SQLHSTMT, szsqlstr: &[super::SQLWCHAR]) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLPrepareW(hstmt : super::SQLHSTMT, szsqlstr : *const super::SQLWCHAR, cchsqlstr : super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLPrepareW(hstmt as _, szsqlstr.as_ptr(), szsqlstr.len().try_into().unwrap()) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLPrimaryKeysA(hstmt: super::SQLHSTMT, szcatalogname: Option<*const super::SQLCHAR>, cbcatalogname: super::SQLSMALLINT, szschemaname: Option<*const super::SQLCHAR>, cbschemaname: super::SQLSMALLINT, sztablename: Option<*const super::SQLCHAR>, cbtablename: super::SQLSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLPrimaryKeysA(hstmt : super::SQLHSTMT, szcatalogname : *const super::SQLCHAR, cbcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLCHAR, cbschemaname : super::SQLSMALLINT, sztablename : *const super::SQLCHAR, cbtablename : super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLPrimaryKeysA(hstmt as _, szcatalogname.unwrap_or(core::mem::zeroed()) as _, cbcatalogname, szschemaname.unwrap_or(core::mem::zeroed()) as _, cbschemaname, sztablename.unwrap_or(core::mem::zeroed()) as _, cbtablename) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLPrimaryKeysW(hstmt: super::SQLHSTMT, szcatalogname: Option<*const super::SQLWCHAR>, cchcatalogname: super::SQLSMALLINT, szschemaname: Option<*const super::SQLWCHAR>, cchschemaname: super::SQLSMALLINT, sztablename: Option<*const super::SQLWCHAR>, cchtablename: super::SQLSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLPrimaryKeysW(hstmt : super::SQLHSTMT, szcatalogname : *const super::SQLWCHAR, cchcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLWCHAR, cchschemaname : super::SQLSMALLINT, sztablename : *const super::SQLWCHAR, cchtablename : super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLPrimaryKeysW(hstmt as _, szcatalogname.unwrap_or(core::mem::zeroed()) as _, cchcatalogname, szschemaname.unwrap_or(core::mem::zeroed()) as _, cchschemaname, sztablename.unwrap_or(core::mem::zeroed()) as _, cchtablename) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLProcedureColumnsA(hstmt: super::SQLHSTMT, szcatalogname: Option<*const super::SQLCHAR>, cbcatalogname: super::SQLSMALLINT, szschemaname: Option<*const super::SQLCHAR>, cbschemaname: super::SQLSMALLINT, szprocname: Option<*const super::SQLCHAR>, cbprocname: super::SQLSMALLINT, szcolumnname: Option<*const super::SQLCHAR>, cbcolumnname: super::SQLSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLProcedureColumnsA(hstmt : super::SQLHSTMT, szcatalogname : *const super::SQLCHAR, cbcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLCHAR, cbschemaname : super::SQLSMALLINT, szprocname : *const super::SQLCHAR, cbprocname : super::SQLSMALLINT, szcolumnname : *const super::SQLCHAR, cbcolumnname : super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLProcedureColumnsA(hstmt as _, szcatalogname.unwrap_or(core::mem::zeroed()) as _, cbcatalogname, szschemaname.unwrap_or(core::mem::zeroed()) as _, cbschemaname, szprocname.unwrap_or(core::mem::zeroed()) as _, cbprocname, szcolumnname.unwrap_or(core::mem::zeroed()) as _, cbcolumnname) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLProcedureColumnsW(hstmt: super::SQLHSTMT, szcatalogname: Option<*const super::SQLWCHAR>, cchcatalogname: super::SQLSMALLINT, szschemaname: Option<*const super::SQLWCHAR>, cchschemaname: super::SQLSMALLINT, szprocname: Option<*const super::SQLWCHAR>, cchprocname: super::SQLSMALLINT, szcolumnname: Option<*const super::SQLWCHAR>, cchcolumnname: super::SQLSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLProcedureColumnsW(hstmt : super::SQLHSTMT, szcatalogname : *const super::SQLWCHAR, cchcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLWCHAR, cchschemaname : super::SQLSMALLINT, szprocname : *const super::SQLWCHAR, cchprocname : super::SQLSMALLINT, szcolumnname : *const super::SQLWCHAR, cchcolumnname : super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLProcedureColumnsW(hstmt as _, szcatalogname.unwrap_or(core::mem::zeroed()) as _, cchcatalogname, szschemaname.unwrap_or(core::mem::zeroed()) as _, cchschemaname, szprocname.unwrap_or(core::mem::zeroed()) as _, cchprocname, szcolumnname.unwrap_or(core::mem::zeroed()) as _, cchcolumnname) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLProceduresA(hstmt: super::SQLHSTMT, szcatalogname: Option<*const super::SQLCHAR>, cbcatalogname: super::SQLSMALLINT, szschemaname: Option<*const super::SQLCHAR>, cbschemaname: super::SQLSMALLINT, szprocname: Option<*const super::SQLCHAR>, cbprocname: super::SQLSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLProceduresA(hstmt : super::SQLHSTMT, szcatalogname : *const super::SQLCHAR, cbcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLCHAR, cbschemaname : super::SQLSMALLINT, szprocname : *const super::SQLCHAR, cbprocname : super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLProceduresA(hstmt as _, szcatalogname.unwrap_or(core::mem::zeroed()) as _, cbcatalogname, szschemaname.unwrap_or(core::mem::zeroed()) as _, cbschemaname, szprocname.unwrap_or(core::mem::zeroed()) as _, cbprocname) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLProceduresW(hstmt: super::SQLHSTMT, szcatalogname: Option<*const super::SQLWCHAR>, cchcatalogname: super::SQLSMALLINT, szschemaname: Option<*const super::SQLWCHAR>, cchschemaname: super::SQLSMALLINT, szprocname: Option<*const super::SQLWCHAR>, cchprocname: super::SQLSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLProceduresW(hstmt : super::SQLHSTMT, szcatalogname : *const super::SQLWCHAR, cchcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLWCHAR, cchschemaname : super::SQLSMALLINT, szprocname : *const super::SQLWCHAR, cchprocname : super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLProceduresW(hstmt as _, szcatalogname.unwrap_or(core::mem::zeroed()) as _, cchcatalogname, szschemaname.unwrap_or(core::mem::zeroed()) as _, cchschemaname, szprocname.unwrap_or(core::mem::zeroed()) as _, cchprocname) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSetConnectAttrA(hdbc: super::SQLHDBC, fattribute: super::SQLINTEGER, rgbvalue: Option<super::SQLPOINTER>, cbvalue: super::SQLINTEGER) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetConnectAttrA(hdbc : super::SQLHDBC, fattribute : super::SQLINTEGER, rgbvalue : super::SQLPOINTER, cbvalue : super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLSetConnectAttrA(hdbc as _, fattribute, rgbvalue.unwrap_or(core::mem::zeroed()) as _, cbvalue) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSetConnectAttrW(hdbc: super::SQLHDBC, fattribute: super::SQLINTEGER, rgbvalue: Option<super::SQLPOINTER>, cbvalue: super::SQLINTEGER) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetConnectAttrW(hdbc : super::SQLHDBC, fattribute : super::SQLINTEGER, rgbvalue : super::SQLPOINTER, cbvalue : super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLSetConnectAttrW(hdbc as _, fattribute, rgbvalue.unwrap_or(core::mem::zeroed()) as _, cbvalue) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSetConnectOptionA(hdbc: super::SQLHDBC, foption: super::SQLUSMALLINT, vparam: super::SQLUINTEGER) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetConnectOptionA(hdbc : super::SQLHDBC, foption : super::SQLUSMALLINT, vparam : super::SQLUINTEGER) -> super::SQLRETURN);
    unsafe { SQLSetConnectOptionA(hdbc as _, foption, vparam) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSetConnectOptionA(hdbc: super::SQLHDBC, foption: super::SQLUSMALLINT, vparam: super::SQLULEN) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetConnectOptionA(hdbc : super::SQLHDBC, foption : super::SQLUSMALLINT, vparam : super::SQLULEN) -> super::SQLRETURN);
    unsafe { SQLSetConnectOptionA(hdbc as _, foption, vparam) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSetConnectOptionW(hdbc: super::SQLHDBC, foption: super::SQLUSMALLINT, vparam: super::SQLUINTEGER) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetConnectOptionW(hdbc : super::SQLHDBC, foption : super::SQLUSMALLINT, vparam : super::SQLUINTEGER) -> super::SQLRETURN);
    unsafe { SQLSetConnectOptionW(hdbc as _, foption, vparam) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSetConnectOptionW(hdbc: super::SQLHDBC, foption: super::SQLUSMALLINT, vparam: super::SQLULEN) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetConnectOptionW(hdbc : super::SQLHDBC, foption : super::SQLUSMALLINT, vparam : super::SQLULEN) -> super::SQLRETURN);
    unsafe { SQLSetConnectOptionW(hdbc as _, foption, vparam) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSetCursorNameA(hstmt: super::SQLHSTMT, szcursor: &[super::SQLCHAR]) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetCursorNameA(hstmt : super::SQLHSTMT, szcursor : *const super::SQLCHAR, cbcursor : super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLSetCursorNameA(hstmt as _, szcursor.as_ptr(), szcursor.len().try_into().unwrap()) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSetCursorNameW(hstmt: super::SQLHSTMT, szcursor: &[super::SQLWCHAR]) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetCursorNameW(hstmt : super::SQLHSTMT, szcursor : *const super::SQLWCHAR, cchcursor : super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLSetCursorNameW(hstmt as _, szcursor.as_ptr(), szcursor.len().try_into().unwrap()) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSetDescFieldW(descriptorhandle: super::SQLHDESC, recnumber: super::SQLSMALLINT, fieldidentifier: super::SQLSMALLINT, value: super::SQLPOINTER, bufferlength: super::SQLINTEGER) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetDescFieldW(descriptorhandle : super::SQLHDESC, recnumber : super::SQLSMALLINT, fieldidentifier : super::SQLSMALLINT, value : super::SQLPOINTER, bufferlength : super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLSetDescFieldW(descriptorhandle as _, recnumber, fieldidentifier, value as _, bufferlength) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSetStmtAttrW(hstmt: super::SQLHSTMT, fattribute: super::SQLINTEGER, rgbvalue: super::SQLPOINTER, cbvaluemax: super::SQLINTEGER) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSetStmtAttrW(hstmt : super::SQLHSTMT, fattribute : super::SQLINTEGER, rgbvalue : super::SQLPOINTER, cbvaluemax : super::SQLINTEGER) -> super::SQLRETURN);
    unsafe { SQLSetStmtAttrW(hstmt as _, fattribute, rgbvalue as _, cbvaluemax) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSpecialColumnsA(hstmt: super::SQLHSTMT, fcoltype: super::SQLUSMALLINT, szcatalogname: Option<*const super::SQLCHAR>, cbcatalogname: super::SQLSMALLINT, szschemaname: Option<*const super::SQLCHAR>, cbschemaname: super::SQLSMALLINT, sztablename: Option<*const super::SQLCHAR>, cbtablename: super::SQLSMALLINT, fscope: super::SQLUSMALLINT, fnullable: super::SQLUSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSpecialColumnsA(hstmt : super::SQLHSTMT, fcoltype : super::SQLUSMALLINT, szcatalogname : *const super::SQLCHAR, cbcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLCHAR, cbschemaname : super::SQLSMALLINT, sztablename : *const super::SQLCHAR, cbtablename : super::SQLSMALLINT, fscope : super::SQLUSMALLINT, fnullable : super::SQLUSMALLINT) -> super::SQLRETURN);
    unsafe { SQLSpecialColumnsA(hstmt as _, fcoltype, szcatalogname.unwrap_or(core::mem::zeroed()) as _, cbcatalogname, szschemaname.unwrap_or(core::mem::zeroed()) as _, cbschemaname, sztablename.unwrap_or(core::mem::zeroed()) as _, cbtablename, fscope, fnullable) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLSpecialColumnsW(hstmt: super::SQLHSTMT, fcoltype: super::SQLUSMALLINT, szcatalogname: Option<*const super::SQLWCHAR>, cchcatalogname: super::SQLSMALLINT, szschemaname: Option<*const super::SQLWCHAR>, cchschemaname: super::SQLSMALLINT, sztablename: Option<*const super::SQLWCHAR>, cchtablename: super::SQLSMALLINT, fscope: super::SQLUSMALLINT, fnullable: super::SQLUSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLSpecialColumnsW(hstmt : super::SQLHSTMT, fcoltype : super::SQLUSMALLINT, szcatalogname : *const super::SQLWCHAR, cchcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLWCHAR, cchschemaname : super::SQLSMALLINT, sztablename : *const super::SQLWCHAR, cchtablename : super::SQLSMALLINT, fscope : super::SQLUSMALLINT, fnullable : super::SQLUSMALLINT) -> super::SQLRETURN);
    unsafe { SQLSpecialColumnsW(hstmt as _, fcoltype, szcatalogname.unwrap_or(core::mem::zeroed()) as _, cchcatalogname, szschemaname.unwrap_or(core::mem::zeroed()) as _, cchschemaname, sztablename.unwrap_or(core::mem::zeroed()) as _, cchtablename, fscope, fnullable) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLStatisticsA(hstmt: super::SQLHSTMT, szcatalogname: Option<*const super::SQLCHAR>, cbcatalogname: super::SQLSMALLINT, szschemaname: Option<*const super::SQLCHAR>, cbschemaname: super::SQLSMALLINT, sztablename: Option<*const super::SQLCHAR>, cbtablename: super::SQLSMALLINT, funique: super::SQLUSMALLINT, faccuracy: super::SQLUSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLStatisticsA(hstmt : super::SQLHSTMT, szcatalogname : *const super::SQLCHAR, cbcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLCHAR, cbschemaname : super::SQLSMALLINT, sztablename : *const super::SQLCHAR, cbtablename : super::SQLSMALLINT, funique : super::SQLUSMALLINT, faccuracy : super::SQLUSMALLINT) -> super::SQLRETURN);
    unsafe { SQLStatisticsA(hstmt as _, szcatalogname.unwrap_or(core::mem::zeroed()) as _, cbcatalogname, szschemaname.unwrap_or(core::mem::zeroed()) as _, cbschemaname, sztablename.unwrap_or(core::mem::zeroed()) as _, cbtablename, funique, faccuracy) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLStatisticsW(hstmt: super::SQLHSTMT, szcatalogname: Option<*const super::SQLWCHAR>, cchcatalogname: super::SQLSMALLINT, szschemaname: Option<*const super::SQLWCHAR>, cchschemaname: super::SQLSMALLINT, sztablename: Option<*const super::SQLWCHAR>, cchtablename: super::SQLSMALLINT, funique: super::SQLUSMALLINT, faccuracy: super::SQLUSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLStatisticsW(hstmt : super::SQLHSTMT, szcatalogname : *const super::SQLWCHAR, cchcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLWCHAR, cchschemaname : super::SQLSMALLINT, sztablename : *const super::SQLWCHAR, cchtablename : super::SQLSMALLINT, funique : super::SQLUSMALLINT, faccuracy : super::SQLUSMALLINT) -> super::SQLRETURN);
    unsafe { SQLStatisticsW(hstmt as _, szcatalogname.unwrap_or(core::mem::zeroed()) as _, cchcatalogname, szschemaname.unwrap_or(core::mem::zeroed()) as _, cchschemaname, sztablename.unwrap_or(core::mem::zeroed()) as _, cchtablename, funique, faccuracy) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLTablePrivilegesA(hstmt: super::SQLHSTMT, szcatalogname: Option<*const super::SQLCHAR>, cbcatalogname: super::SQLSMALLINT, szschemaname: Option<*const super::SQLCHAR>, cbschemaname: super::SQLSMALLINT, sztablename: Option<*const super::SQLCHAR>, cbtablename: super::SQLSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLTablePrivilegesA(hstmt : super::SQLHSTMT, szcatalogname : *const super::SQLCHAR, cbcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLCHAR, cbschemaname : super::SQLSMALLINT, sztablename : *const super::SQLCHAR, cbtablename : super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLTablePrivilegesA(hstmt as _, szcatalogname.unwrap_or(core::mem::zeroed()) as _, cbcatalogname, szschemaname.unwrap_or(core::mem::zeroed()) as _, cbschemaname, sztablename.unwrap_or(core::mem::zeroed()) as _, cbtablename) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLTablePrivilegesW(hstmt: super::SQLHSTMT, szcatalogname: Option<*const super::SQLWCHAR>, cchcatalogname: super::SQLSMALLINT, szschemaname: Option<*const super::SQLWCHAR>, cchschemaname: super::SQLSMALLINT, sztablename: Option<*const super::SQLWCHAR>, cchtablename: super::SQLSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLTablePrivilegesW(hstmt : super::SQLHSTMT, szcatalogname : *const super::SQLWCHAR, cchcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLWCHAR, cchschemaname : super::SQLSMALLINT, sztablename : *const super::SQLWCHAR, cchtablename : super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLTablePrivilegesW(hstmt as _, szcatalogname.unwrap_or(core::mem::zeroed()) as _, cchcatalogname, szschemaname.unwrap_or(core::mem::zeroed()) as _, cchschemaname, sztablename.unwrap_or(core::mem::zeroed()) as _, cchtablename) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLTablesA(hstmt: super::SQLHSTMT, szcatalogname: Option<*const super::SQLCHAR>, cbcatalogname: super::SQLSMALLINT, szschemaname: Option<*const super::SQLCHAR>, cbschemaname: super::SQLSMALLINT, sztablename: Option<*const super::SQLCHAR>, cbtablename: super::SQLSMALLINT, sztabletype: Option<*const super::SQLCHAR>, cbtabletype: super::SQLSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLTablesA(hstmt : super::SQLHSTMT, szcatalogname : *const super::SQLCHAR, cbcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLCHAR, cbschemaname : super::SQLSMALLINT, sztablename : *const super::SQLCHAR, cbtablename : super::SQLSMALLINT, sztabletype : *const super::SQLCHAR, cbtabletype : super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLTablesA(hstmt as _, szcatalogname.unwrap_or(core::mem::zeroed()) as _, cbcatalogname, szschemaname.unwrap_or(core::mem::zeroed()) as _, cbschemaname, sztablename.unwrap_or(core::mem::zeroed()) as _, cbtablename, sztabletype.unwrap_or(core::mem::zeroed()) as _, cbtabletype) }
}
#[cfg(feature = "sqltypes")]
#[inline]
pub unsafe fn SQLTablesW(hstmt: super::SQLHSTMT, szcatalogname: Option<*const super::SQLWCHAR>, cchcatalogname: super::SQLSMALLINT, szschemaname: Option<*const super::SQLWCHAR>, cchschemaname: super::SQLSMALLINT, sztablename: Option<*const super::SQLWCHAR>, cchtablename: super::SQLSMALLINT, sztabletype: Option<*const super::SQLWCHAR>, cchtabletype: super::SQLSMALLINT) -> super::SQLRETURN {
    windows_core::link!("odbc32.dll" "system" fn SQLTablesW(hstmt : super::SQLHSTMT, szcatalogname : *const super::SQLWCHAR, cchcatalogname : super::SQLSMALLINT, szschemaname : *const super::SQLWCHAR, cchschemaname : super::SQLSMALLINT, sztablename : *const super::SQLWCHAR, cchtablename : super::SQLSMALLINT, sztabletype : *const super::SQLWCHAR, cchtabletype : super::SQLSMALLINT) -> super::SQLRETURN);
    unsafe { SQLTablesW(hstmt as _, szcatalogname.unwrap_or(core::mem::zeroed()) as _, cchcatalogname, szschemaname.unwrap_or(core::mem::zeroed()) as _, cchschemaname, sztablename.unwrap_or(core::mem::zeroed()) as _, cchtablename, sztabletype.unwrap_or(core::mem::zeroed()) as _, cchtabletype) }
}
pub const SQL_C_TCHAR: i32 = 1;
pub const SQL_C_WCHAR: i32 = -8;
pub const SQL_SQLSTATE_SIZEW: i32 = 10;
pub const SQL_WCHAR: i32 = -8;
pub const SQL_WLONGVARCHAR: i32 = -10;
pub const SQL_WVARCHAR: i32 = -9;
