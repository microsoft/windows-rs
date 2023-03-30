#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddLogContainer<P0, P1>(hlog: P0, pcbcontainer: ::core::option::Option<*const u64>, pwszcontainerpath: P1, preserved: ::core::option::Option<*mut ::core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "clfsw32.dll""system" fn AddLogContainer ( hlog : super::super::Foundation:: HANDLE , pcbcontainer : *const u64 , pwszcontainerpath : ::windows::core::PCWSTR , preserved : *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    AddLogContainer(hlog.into_param().abi(), ::core::mem::transmute(pcbcontainer.unwrap_or(::std::ptr::null())), pwszcontainerpath.into_param().abi(), ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddLogContainerSet<P0>(hlog: P0, pcbcontainer: ::core::option::Option<*const u64>, rgwszcontainerpath: &[::windows::core::PCWSTR], preserved: ::core::option::Option<*mut ::core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "clfsw32.dll""system" fn AddLogContainerSet ( hlog : super::super::Foundation:: HANDLE , ccontainer : u16 , pcbcontainer : *const u64 , rgwszcontainerpath : *const ::windows::core::PCWSTR , preserved : *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    AddLogContainerSet(hlog.into_param().abi(), rgwszcontainerpath.len() as _, ::core::mem::transmute(pcbcontainer.unwrap_or(::std::ptr::null())), ::core::mem::transmute(rgwszcontainerpath.as_ptr()), ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn AddUsersToEncryptedFile<P0>(lpfilename: P0, pencryptioncertificates: *const ENCRYPTION_CERTIFICATE_LIST) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn AddUsersToEncryptedFile ( lpfilename : ::windows::core::PCWSTR , pencryptioncertificates : *const ENCRYPTION_CERTIFICATE_LIST ) -> u32 );
    AddUsersToEncryptedFile(lpfilename.into_param().abi(), pencryptioncertificates)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn AdvanceLogBase(pvmarshal: *mut ::core::ffi::c_void, plsnbase: *mut CLS_LSN, fflags: u32, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "clfsw32.dll""system" fn AdvanceLogBase ( pvmarshal : *mut ::core::ffi::c_void , plsnbase : *mut CLS_LSN , fflags : u32 , poverlapped : *mut super::super::System::IO:: OVERLAPPED ) -> super::super::Foundation:: BOOL );
    AdvanceLogBase(pvmarshal, plsnbase, fflags, poverlapped)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AlignReservedLog(pvmarshal: *mut ::core::ffi::c_void, creservedrecords: u32, rgcbreservation: *mut i64, pcbalignreservation: *mut i64) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "clfsw32.dll""system" fn AlignReservedLog ( pvmarshal : *mut ::core::ffi::c_void , creservedrecords : u32 , rgcbreservation : *mut i64 , pcbalignreservation : *mut i64 ) -> super::super::Foundation:: BOOL );
    AlignReservedLog(pvmarshal, creservedrecords, rgcbreservation, pcbalignreservation)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AllocReservedLog(pvmarshal: *mut ::core::ffi::c_void, creservedrecords: u32, pcbadjustment: *mut i64) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "clfsw32.dll""system" fn AllocReservedLog ( pvmarshal : *mut ::core::ffi::c_void , creservedrecords : u32 , pcbadjustment : *mut i64 ) -> super::super::Foundation:: BOOL );
    AllocReservedLog(pvmarshal, creservedrecords, pcbadjustment)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AreFileApisANSI() -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn AreFileApisANSI ( ) -> super::super::Foundation:: BOOL );
    AreFileApisANSI()
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AreShortNamesEnabled<P0>(handle: P0, enabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn AreShortNamesEnabled ( handle : super::super::Foundation:: HANDLE , enabled : *mut super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    AreShortNamesEnabled(handle.into_param().abi(), enabled)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BackupRead<P0, P1, P2>(hfile: P0, lpbuffer: &mut [u8], lpnumberofbytesread: *mut u32, babort: P1, bprocesssecurity: P2, lpcontext: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn BackupRead ( hfile : super::super::Foundation:: HANDLE , lpbuffer : *mut u8 , nnumberofbytestoread : u32 , lpnumberofbytesread : *mut u32 , babort : super::super::Foundation:: BOOL , bprocesssecurity : super::super::Foundation:: BOOL , lpcontext : *mut *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    BackupRead(hfile.into_param().abi(), ::core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len() as _, lpnumberofbytesread, babort.into_param().abi(), bprocesssecurity.into_param().abi(), lpcontext)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BackupSeek<P0>(hfile: P0, dwlowbytestoseek: u32, dwhighbytestoseek: u32, lpdwlowbyteseeked: *mut u32, lpdwhighbyteseeked: *mut u32, lpcontext: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn BackupSeek ( hfile : super::super::Foundation:: HANDLE , dwlowbytestoseek : u32 , dwhighbytestoseek : u32 , lpdwlowbyteseeked : *mut u32 , lpdwhighbyteseeked : *mut u32 , lpcontext : *mut *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    BackupSeek(hfile.into_param().abi(), dwlowbytestoseek, dwhighbytestoseek, lpdwlowbyteseeked, lpdwhighbyteseeked, lpcontext)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BackupWrite<P0, P1, P2>(hfile: P0, lpbuffer: &[u8], lpnumberofbyteswritten: *mut u32, babort: P1, bprocesssecurity: P2, lpcontext: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn BackupWrite ( hfile : super::super::Foundation:: HANDLE , lpbuffer : *const u8 , nnumberofbytestowrite : u32 , lpnumberofbyteswritten : *mut u32 , babort : super::super::Foundation:: BOOL , bprocesssecurity : super::super::Foundation:: BOOL , lpcontext : *mut *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    BackupWrite(hfile.into_param().abi(), ::core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len() as _, lpnumberofbyteswritten, babort.into_param().abi(), bprocesssecurity.into_param().abi(), lpcontext)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BuildIoRingCancelRequest(ioring: *const HIORING__, file: IORING_HANDLE_REF, optocancel: usize, userdata: usize) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "api-ms-win-core-ioring-l1-1-0.dll""system" fn BuildIoRingCancelRequest ( ioring : *const HIORING__ , file : IORING_HANDLE_REF , optocancel : usize , userdata : usize ) -> ::windows::core::HRESULT );
    BuildIoRingCancelRequest(ioring, ::core::mem::transmute(file), optocancel, userdata).ok()
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BuildIoRingReadFile(ioring: *const HIORING__, fileref: IORING_HANDLE_REF, dataref: IORING_BUFFER_REF, numberofbytestoread: u32, fileoffset: u64, userdata: usize, sqeflags: IORING_SQE_FLAGS) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "api-ms-win-core-ioring-l1-1-0.dll""system" fn BuildIoRingReadFile ( ioring : *const HIORING__ , fileref : IORING_HANDLE_REF , dataref : IORING_BUFFER_REF , numberofbytestoread : u32 , fileoffset : u64 , userdata : usize , sqeflags : IORING_SQE_FLAGS ) -> ::windows::core::HRESULT );
    BuildIoRingReadFile(ioring, ::core::mem::transmute(fileref), ::core::mem::transmute(dataref), numberofbytestoread, fileoffset, userdata, sqeflags).ok()
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn BuildIoRingRegisterBuffers(ioring: *const HIORING__, buffers: &[IORING_BUFFER_INFO], userdata: usize) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "api-ms-win-core-ioring-l1-1-0.dll""system" fn BuildIoRingRegisterBuffers ( ioring : *const HIORING__ , count : u32 , buffers : *const IORING_BUFFER_INFO , userdata : usize ) -> ::windows::core::HRESULT );
    BuildIoRingRegisterBuffers(ioring, buffers.len() as _, ::core::mem::transmute(buffers.as_ptr()), userdata).ok()
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BuildIoRingRegisterFileHandles(ioring: *const HIORING__, handles: &[super::super::Foundation::HANDLE], userdata: usize) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "api-ms-win-core-ioring-l1-1-0.dll""system" fn BuildIoRingRegisterFileHandles ( ioring : *const HIORING__ , count : u32 , handles : *const super::super::Foundation:: HANDLE , userdata : usize ) -> ::windows::core::HRESULT );
    BuildIoRingRegisterFileHandles(ioring, handles.len() as _, ::core::mem::transmute(handles.as_ptr()), userdata).ok()
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckNameLegalDOS8Dot3A<P0>(lpname: P0, lpoemname: ::core::option::Option<&mut [u8]>, pbnamecontainsspaces: ::core::option::Option<*mut super::super::Foundation::BOOL>, pbnamelegal: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CheckNameLegalDOS8Dot3A ( lpname : ::windows::core::PCSTR , lpoemname : ::windows::core::PSTR , oemnamesize : u32 , pbnamecontainsspaces : *mut super::super::Foundation:: BOOL , pbnamelegal : *mut super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    CheckNameLegalDOS8Dot3A(lpname.into_param().abi(), ::core::mem::transmute(lpoemname.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpoemname.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pbnamecontainsspaces.unwrap_or(::std::ptr::null_mut())), pbnamelegal)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckNameLegalDOS8Dot3W<P0>(lpname: P0, lpoemname: ::core::option::Option<&mut [u8]>, pbnamecontainsspaces: ::core::option::Option<*mut super::super::Foundation::BOOL>, pbnamelegal: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CheckNameLegalDOS8Dot3W ( lpname : ::windows::core::PCWSTR , lpoemname : ::windows::core::PSTR , oemnamesize : u32 , pbnamecontainsspaces : *mut super::super::Foundation:: BOOL , pbnamelegal : *mut super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    CheckNameLegalDOS8Dot3W(lpname.into_param().abi(), ::core::mem::transmute(lpoemname.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpoemname.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pbnamecontainsspaces.unwrap_or(::std::ptr::null_mut())), pbnamelegal)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseAndResetLogFile<P0>(hlog: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "clfsw32.dll""system" fn CloseAndResetLogFile ( hlog : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    CloseAndResetLogFile(hlog.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn CloseEncryptedFileRaw(pvcontext: *const ::core::ffi::c_void) {
    ::windows_targets::link ! ( "advapi32.dll""system" fn CloseEncryptedFileRaw ( pvcontext : *const ::core::ffi::c_void ) -> ( ) );
    CloseEncryptedFileRaw(pvcontext)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn CloseIoRing(ioring: *const HIORING__) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "api-ms-win-core-ioring-l1-1-0.dll""system" fn CloseIoRing ( ioring : *const HIORING__ ) -> ::windows::core::HRESULT );
    CloseIoRing(ioring).ok()
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CommitComplete<P0>(enlistmenthandle: P0, tmvirtualclock: *mut i64) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ktmw32.dll""system" fn CommitComplete ( enlistmenthandle : super::super::Foundation:: HANDLE , tmvirtualclock : *mut i64 ) -> super::super::Foundation:: BOOL );
    CommitComplete(enlistmenthandle.into_param().abi(), tmvirtualclock)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CommitEnlistment<P0>(enlistmenthandle: P0, tmvirtualclock: *mut i64) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ktmw32.dll""system" fn CommitEnlistment ( enlistmenthandle : super::super::Foundation:: HANDLE , tmvirtualclock : *mut i64 ) -> super::super::Foundation:: BOOL );
    CommitEnlistment(enlistmenthandle.into_param().abi(), tmvirtualclock)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CommitTransaction<P0>(transactionhandle: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ktmw32.dll""system" fn CommitTransaction ( transactionhandle : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    CommitTransaction(transactionhandle.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CommitTransactionAsync<P0>(transactionhandle: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ktmw32.dll""system" fn CommitTransactionAsync ( transactionhandle : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    CommitTransactionAsync(transactionhandle.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CompareFileTime(lpfiletime1: *const super::super::Foundation::FILETIME, lpfiletime2: *const super::super::Foundation::FILETIME) -> i32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn CompareFileTime ( lpfiletime1 : *const super::super::Foundation:: FILETIME , lpfiletime2 : *const super::super::Foundation:: FILETIME ) -> i32 );
    CompareFileTime(lpfiletime1, lpfiletime2)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CopyFile2<P0, P1>(pwszexistingfilename: P0, pwsznewfilename: P1, pextendedparameters: ::core::option::Option<*const COPYFILE2_EXTENDED_PARAMETERS>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CopyFile2 ( pwszexistingfilename : ::windows::core::PCWSTR , pwsznewfilename : ::windows::core::PCWSTR , pextendedparameters : *const COPYFILE2_EXTENDED_PARAMETERS ) -> ::windows::core::HRESULT );
    CopyFile2(pwszexistingfilename.into_param().abi(), pwsznewfilename.into_param().abi(), ::core::mem::transmute(pextendedparameters.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CopyFileA<P0, P1, P2>(lpexistingfilename: P0, lpnewfilename: P1, bfailifexists: P2) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CopyFileA ( lpexistingfilename : ::windows::core::PCSTR , lpnewfilename : ::windows::core::PCSTR , bfailifexists : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    CopyFileA(lpexistingfilename.into_param().abi(), lpnewfilename.into_param().abi(), bfailifexists.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CopyFileExA<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: ::core::option::Option<*const ::core::ffi::c_void>, pbcancel: ::core::option::Option<*mut i32>, dwcopyflags: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CopyFileExA ( lpexistingfilename : ::windows::core::PCSTR , lpnewfilename : ::windows::core::PCSTR , lpprogressroutine : LPPROGRESS_ROUTINE , lpdata : *const ::core::ffi::c_void , pbcancel : *mut i32 , dwcopyflags : u32 ) -> super::super::Foundation:: BOOL );
    CopyFileExA(lpexistingfilename.into_param().abi(), lpnewfilename.into_param().abi(), lpprogressroutine, ::core::mem::transmute(lpdata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pbcancel.unwrap_or(::std::ptr::null_mut())), dwcopyflags)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CopyFileExW<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: ::core::option::Option<*const ::core::ffi::c_void>, pbcancel: ::core::option::Option<*mut i32>, dwcopyflags: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CopyFileExW ( lpexistingfilename : ::windows::core::PCWSTR , lpnewfilename : ::windows::core::PCWSTR , lpprogressroutine : LPPROGRESS_ROUTINE , lpdata : *const ::core::ffi::c_void , pbcancel : *mut i32 , dwcopyflags : u32 ) -> super::super::Foundation:: BOOL );
    CopyFileExW(lpexistingfilename.into_param().abi(), lpnewfilename.into_param().abi(), lpprogressroutine, ::core::mem::transmute(lpdata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pbcancel.unwrap_or(::std::ptr::null_mut())), dwcopyflags)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CopyFileFromAppW<P0, P1, P2>(lpexistingfilename: P0, lpnewfilename: P1, bfailifexists: P2) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "api-ms-win-core-file-fromapp-l1-1-0.dll""system" fn CopyFileFromAppW ( lpexistingfilename : ::windows::core::PCWSTR , lpnewfilename : ::windows::core::PCWSTR , bfailifexists : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    CopyFileFromAppW(lpexistingfilename.into_param().abi(), lpnewfilename.into_param().abi(), bfailifexists.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CopyFileTransactedA<P0, P1, P2>(lpexistingfilename: P0, lpnewfilename: P1, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: ::core::option::Option<*const ::core::ffi::c_void>, pbcancel: ::core::option::Option<*const i32>, dwcopyflags: u32, htransaction: P2) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CopyFileTransactedA ( lpexistingfilename : ::windows::core::PCSTR , lpnewfilename : ::windows::core::PCSTR , lpprogressroutine : LPPROGRESS_ROUTINE , lpdata : *const ::core::ffi::c_void , pbcancel : *const i32 , dwcopyflags : u32 , htransaction : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    CopyFileTransactedA(lpexistingfilename.into_param().abi(), lpnewfilename.into_param().abi(), lpprogressroutine, ::core::mem::transmute(lpdata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pbcancel.unwrap_or(::std::ptr::null())), dwcopyflags, htransaction.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CopyFileTransactedW<P0, P1, P2>(lpexistingfilename: P0, lpnewfilename: P1, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: ::core::option::Option<*const ::core::ffi::c_void>, pbcancel: ::core::option::Option<*const i32>, dwcopyflags: u32, htransaction: P2) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CopyFileTransactedW ( lpexistingfilename : ::windows::core::PCWSTR , lpnewfilename : ::windows::core::PCWSTR , lpprogressroutine : LPPROGRESS_ROUTINE , lpdata : *const ::core::ffi::c_void , pbcancel : *const i32 , dwcopyflags : u32 , htransaction : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    CopyFileTransactedW(lpexistingfilename.into_param().abi(), lpnewfilename.into_param().abi(), lpprogressroutine, ::core::mem::transmute(lpdata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pbcancel.unwrap_or(::std::ptr::null())), dwcopyflags, htransaction.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CopyFileW<P0, P1, P2>(lpexistingfilename: P0, lpnewfilename: P1, bfailifexists: P2) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CopyFileW ( lpexistingfilename : ::windows::core::PCWSTR , lpnewfilename : ::windows::core::PCWSTR , bfailifexists : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    CopyFileW(lpexistingfilename.into_param().abi(), lpnewfilename.into_param().abi(), bfailifexists.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn CopyLZFile(hfsource: i32, hfdest: i32) -> i32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn CopyLZFile ( hfsource : i32 , hfdest : i32 ) -> i32 );
    CopyLZFile(hfsource, hfdest)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateDirectoryA<P0>(lppathname: P0, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CreateDirectoryA ( lppathname : ::windows::core::PCSTR , lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES ) -> super::super::Foundation:: BOOL );
    CreateDirectoryA(lppathname.into_param().abi(), ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateDirectoryExA<P0, P1>(lptemplatedirectory: P0, lpnewdirectory: P1, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CreateDirectoryExA ( lptemplatedirectory : ::windows::core::PCSTR , lpnewdirectory : ::windows::core::PCSTR , lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES ) -> super::super::Foundation:: BOOL );
    CreateDirectoryExA(lptemplatedirectory.into_param().abi(), lpnewdirectory.into_param().abi(), ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateDirectoryExW<P0, P1>(lptemplatedirectory: P0, lpnewdirectory: P1, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CreateDirectoryExW ( lptemplatedirectory : ::windows::core::PCWSTR , lpnewdirectory : ::windows::core::PCWSTR , lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES ) -> super::super::Foundation:: BOOL );
    CreateDirectoryExW(lptemplatedirectory.into_param().abi(), lpnewdirectory.into_param().abi(), ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateDirectoryFromAppW<P0>(lppathname: P0, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "api-ms-win-core-file-fromapp-l1-1-0.dll""system" fn CreateDirectoryFromAppW ( lppathname : ::windows::core::PCWSTR , lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES ) -> super::super::Foundation:: BOOL );
    CreateDirectoryFromAppW(lppathname.into_param().abi(), ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateDirectoryTransactedA<P0, P1, P2>(lptemplatedirectory: P0, lpnewdirectory: P1, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, htransaction: P2) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CreateDirectoryTransactedA ( lptemplatedirectory : ::windows::core::PCSTR , lpnewdirectory : ::windows::core::PCSTR , lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , htransaction : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    CreateDirectoryTransactedA(lptemplatedirectory.into_param().abi(), lpnewdirectory.into_param().abi(), ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null())), htransaction.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateDirectoryTransactedW<P0, P1, P2>(lptemplatedirectory: P0, lpnewdirectory: P1, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, htransaction: P2) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CreateDirectoryTransactedW ( lptemplatedirectory : ::windows::core::PCWSTR , lpnewdirectory : ::windows::core::PCWSTR , lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , htransaction : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    CreateDirectoryTransactedW(lptemplatedirectory.into_param().abi(), lpnewdirectory.into_param().abi(), ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null())), htransaction.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateDirectoryW<P0>(lppathname: P0, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CreateDirectoryW ( lppathname : ::windows::core::PCWSTR , lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES ) -> super::super::Foundation:: BOOL );
    CreateDirectoryW(lppathname.into_param().abi(), ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateEnlistment<P0, P1>(lpenlistmentattributes: *mut super::super::Security::SECURITY_ATTRIBUTES, resourcemanagerhandle: P0, transactionhandle: P1, notificationmask: u32, createoptions: u32, enlistmentkey: *mut ::core::ffi::c_void) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ktmw32.dll""system" fn CreateEnlistment ( lpenlistmentattributes : *mut super::super::Security:: SECURITY_ATTRIBUTES , resourcemanagerhandle : super::super::Foundation:: HANDLE , transactionhandle : super::super::Foundation:: HANDLE , notificationmask : u32 , createoptions : u32 , enlistmentkey : *mut ::core::ffi::c_void ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateEnlistment(lpenlistmentattributes, resourcemanagerhandle.into_param().abi(), transactionhandle.into_param().abi(), notificationmask, createoptions, enlistmentkey);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateFile2<P0>(lpfilename: P0, dwdesiredaccess: u32, dwsharemode: FILE_SHARE_MODE, dwcreationdisposition: FILE_CREATION_DISPOSITION, pcreateexparams: ::core::option::Option<*const CREATEFILE2_EXTENDED_PARAMETERS>) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CreateFile2 ( lpfilename : ::windows::core::PCWSTR , dwdesiredaccess : u32 , dwsharemode : FILE_SHARE_MODE , dwcreationdisposition : FILE_CREATION_DISPOSITION , pcreateexparams : *const CREATEFILE2_EXTENDED_PARAMETERS ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateFile2(lpfilename.into_param().abi(), dwdesiredaccess, dwsharemode, dwcreationdisposition, ::core::mem::transmute(pcreateexparams.unwrap_or(::std::ptr::null())));
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateFile2FromAppW<P0>(lpfilename: P0, dwdesiredaccess: u32, dwsharemode: u32, dwcreationdisposition: u32, pcreateexparams: ::core::option::Option<*const CREATEFILE2_EXTENDED_PARAMETERS>) -> super::super::Foundation::HANDLE
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "api-ms-win-core-file-fromapp-l1-1-0.dll""system" fn CreateFile2FromAppW ( lpfilename : ::windows::core::PCWSTR , dwdesiredaccess : u32 , dwsharemode : u32 , dwcreationdisposition : u32 , pcreateexparams : *const CREATEFILE2_EXTENDED_PARAMETERS ) -> super::super::Foundation:: HANDLE );
    CreateFile2FromAppW(lpfilename.into_param().abi(), dwdesiredaccess, dwsharemode, dwcreationdisposition, ::core::mem::transmute(pcreateexparams.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateFileA<P0, P1>(lpfilename: P0, dwdesiredaccess: u32, dwsharemode: FILE_SHARE_MODE, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, dwcreationdisposition: FILE_CREATION_DISPOSITION, dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES, htemplatefile: P1) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CreateFileA ( lpfilename : ::windows::core::PCSTR , dwdesiredaccess : u32 , dwsharemode : FILE_SHARE_MODE , lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , dwcreationdisposition : FILE_CREATION_DISPOSITION , dwflagsandattributes : FILE_FLAGS_AND_ATTRIBUTES , htemplatefile : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateFileA(lpfilename.into_param().abi(), dwdesiredaccess, dwsharemode, ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null())), dwcreationdisposition, dwflagsandattributes, htemplatefile.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateFileFromAppW<P0, P1>(lpfilename: P0, dwdesiredaccess: u32, dwsharemode: u32, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, dwcreationdisposition: u32, dwflagsandattributes: u32, htemplatefile: P1) -> super::super::Foundation::HANDLE
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "api-ms-win-core-file-fromapp-l1-1-0.dll""system" fn CreateFileFromAppW ( lpfilename : ::windows::core::PCWSTR , dwdesiredaccess : u32 , dwsharemode : u32 , lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , dwcreationdisposition : u32 , dwflagsandattributes : u32 , htemplatefile : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: HANDLE );
    CreateFileFromAppW(lpfilename.into_param().abi(), dwdesiredaccess, dwsharemode, ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null())), dwcreationdisposition, dwflagsandattributes, htemplatefile.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateFileTransactedA<P0, P1, P2>(lpfilename: P0, dwdesiredaccess: u32, dwsharemode: FILE_SHARE_MODE, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, dwcreationdisposition: FILE_CREATION_DISPOSITION, dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES, htemplatefile: P1, htransaction: P2, pusminiversion: ::core::option::Option<*const TXFS_MINIVERSION>, lpextendedparameter: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P2: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CreateFileTransactedA ( lpfilename : ::windows::core::PCSTR , dwdesiredaccess : u32 , dwsharemode : FILE_SHARE_MODE , lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , dwcreationdisposition : FILE_CREATION_DISPOSITION , dwflagsandattributes : FILE_FLAGS_AND_ATTRIBUTES , htemplatefile : super::super::Foundation:: HANDLE , htransaction : super::super::Foundation:: HANDLE , pusminiversion : *const TXFS_MINIVERSION , lpextendedparameter : *const ::core::ffi::c_void ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateFileTransactedA(lpfilename.into_param().abi(), dwdesiredaccess, dwsharemode, ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null())), dwcreationdisposition, dwflagsandattributes, htemplatefile.into_param().abi(), htransaction.into_param().abi(), ::core::mem::transmute(pusminiversion.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lpextendedparameter.unwrap_or(::std::ptr::null())));
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateFileTransactedW<P0, P1, P2>(lpfilename: P0, dwdesiredaccess: u32, dwsharemode: FILE_SHARE_MODE, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, dwcreationdisposition: FILE_CREATION_DISPOSITION, dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES, htemplatefile: P1, htransaction: P2, pusminiversion: ::core::option::Option<*const TXFS_MINIVERSION>, lpextendedparameter: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P2: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CreateFileTransactedW ( lpfilename : ::windows::core::PCWSTR , dwdesiredaccess : u32 , dwsharemode : FILE_SHARE_MODE , lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , dwcreationdisposition : FILE_CREATION_DISPOSITION , dwflagsandattributes : FILE_FLAGS_AND_ATTRIBUTES , htemplatefile : super::super::Foundation:: HANDLE , htransaction : super::super::Foundation:: HANDLE , pusminiversion : *const TXFS_MINIVERSION , lpextendedparameter : *const ::core::ffi::c_void ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateFileTransactedW(lpfilename.into_param().abi(), dwdesiredaccess, dwsharemode, ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null())), dwcreationdisposition, dwflagsandattributes, htemplatefile.into_param().abi(), htransaction.into_param().abi(), ::core::mem::transmute(pusminiversion.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lpextendedparameter.unwrap_or(::std::ptr::null())));
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateFileW<P0, P1>(lpfilename: P0, dwdesiredaccess: u32, dwsharemode: FILE_SHARE_MODE, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, dwcreationdisposition: FILE_CREATION_DISPOSITION, dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES, htemplatefile: P1) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CreateFileW ( lpfilename : ::windows::core::PCWSTR , dwdesiredaccess : u32 , dwsharemode : FILE_SHARE_MODE , lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , dwcreationdisposition : FILE_CREATION_DISPOSITION , dwflagsandattributes : FILE_FLAGS_AND_ATTRIBUTES , htemplatefile : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateFileW(lpfilename.into_param().abi(), dwdesiredaccess, dwsharemode, ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null())), dwcreationdisposition, dwflagsandattributes, htemplatefile.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateHardLinkA<P0, P1>(lpfilename: P0, lpexistingfilename: P1, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CreateHardLinkA ( lpfilename : ::windows::core::PCSTR , lpexistingfilename : ::windows::core::PCSTR , lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES ) -> super::super::Foundation:: BOOL );
    CreateHardLinkA(lpfilename.into_param().abi(), lpexistingfilename.into_param().abi(), ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateHardLinkTransactedA<P0, P1, P2>(lpfilename: P0, lpexistingfilename: P1, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, htransaction: P2) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CreateHardLinkTransactedA ( lpfilename : ::windows::core::PCSTR , lpexistingfilename : ::windows::core::PCSTR , lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , htransaction : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    CreateHardLinkTransactedA(lpfilename.into_param().abi(), lpexistingfilename.into_param().abi(), ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null())), htransaction.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateHardLinkTransactedW<P0, P1, P2>(lpfilename: P0, lpexistingfilename: P1, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, htransaction: P2) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CreateHardLinkTransactedW ( lpfilename : ::windows::core::PCWSTR , lpexistingfilename : ::windows::core::PCWSTR , lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , htransaction : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    CreateHardLinkTransactedW(lpfilename.into_param().abi(), lpexistingfilename.into_param().abi(), ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null())), htransaction.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateHardLinkW<P0, P1>(lpfilename: P0, lpexistingfilename: P1, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CreateHardLinkW ( lpfilename : ::windows::core::PCWSTR , lpexistingfilename : ::windows::core::PCWSTR , lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES ) -> super::super::Foundation:: BOOL );
    CreateHardLinkW(lpfilename.into_param().abi(), lpexistingfilename.into_param().abi(), ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn CreateIoRing(ioringversion: IORING_VERSION, flags: IORING_CREATE_FLAGS, submissionqueuesize: u32, completionqueuesize: u32) -> ::windows::core::Result<*mut HIORING__> {
    ::windows_targets::link ! ( "api-ms-win-core-ioring-l1-1-0.dll""system" fn CreateIoRing ( ioringversion : IORING_VERSION , flags : IORING_CREATE_FLAGS , submissionqueuesize : u32 , completionqueuesize : u32 , h : *mut *mut HIORING__ ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<*mut HIORING__>();
    CreateIoRing(ioringversion, ::core::mem::transmute(flags), submissionqueuesize, completionqueuesize, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn CreateLogContainerScanContext<P0>(hlog: P0, cfromcontainer: u32, ccontainers: u32, escanmode: u8, pcxscan: *mut CLS_SCAN_CONTEXT, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "clfsw32.dll""system" fn CreateLogContainerScanContext ( hlog : super::super::Foundation:: HANDLE , cfromcontainer : u32 , ccontainers : u32 , escanmode : u8 , pcxscan : *mut CLS_SCAN_CONTEXT , poverlapped : *mut super::super::System::IO:: OVERLAPPED ) -> super::super::Foundation:: BOOL );
    CreateLogContainerScanContext(hlog.into_param().abi(), cfromcontainer, ccontainers, escanmode, pcxscan, poverlapped)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateLogFile<P0>(pszlogfilename: P0, fdesiredaccess: u32, dwsharemode: FILE_SHARE_MODE, psalogfile: *mut super::super::Security::SECURITY_ATTRIBUTES, fcreatedisposition: FILE_CREATION_DISPOSITION, fflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "clfsw32.dll""system" fn CreateLogFile ( pszlogfilename : ::windows::core::PCWSTR , fdesiredaccess : u32 , dwsharemode : FILE_SHARE_MODE , psalogfile : *mut super::super::Security:: SECURITY_ATTRIBUTES , fcreatedisposition : FILE_CREATION_DISPOSITION , fflagsandattributes : FILE_FLAGS_AND_ATTRIBUTES ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateLogFile(pszlogfilename.into_param().abi(), fdesiredaccess, dwsharemode, psalogfile, fcreatedisposition, fflagsandattributes);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateLogMarshallingArea<P0>(hlog: P0, pfnallocbuffer: CLFS_BLOCK_ALLOCATION, pfnfreebuffer: CLFS_BLOCK_DEALLOCATION, pvblockalloccontext: *mut ::core::ffi::c_void, cbmarshallingbuffer: u32, cmaxwritebuffers: u32, cmaxreadbuffers: u32, ppvmarshal: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "clfsw32.dll""system" fn CreateLogMarshallingArea ( hlog : super::super::Foundation:: HANDLE , pfnallocbuffer : CLFS_BLOCK_ALLOCATION , pfnfreebuffer : CLFS_BLOCK_DEALLOCATION , pvblockalloccontext : *mut ::core::ffi::c_void , cbmarshallingbuffer : u32 , cmaxwritebuffers : u32 , cmaxreadbuffers : u32 , ppvmarshal : *mut *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    CreateLogMarshallingArea(hlog.into_param().abi(), pfnallocbuffer, pfnfreebuffer, pvblockalloccontext, cbmarshallingbuffer, cmaxwritebuffers, cmaxreadbuffers, ppvmarshal)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateResourceManager<P0, P1>(lpresourcemanagerattributes: *mut super::super::Security::SECURITY_ATTRIBUTES, resourcemanagerid: *mut ::windows::core::GUID, createoptions: u32, tmhandle: P0, description: P1) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "ktmw32.dll""system" fn CreateResourceManager ( lpresourcemanagerattributes : *mut super::super::Security:: SECURITY_ATTRIBUTES , resourcemanagerid : *mut ::windows::core::GUID , createoptions : u32 , tmhandle : super::super::Foundation:: HANDLE , description : ::windows::core::PCWSTR ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateResourceManager(lpresourcemanagerattributes, resourcemanagerid, createoptions, tmhandle.into_param().abi(), description.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateSymbolicLinkA<P0, P1>(lpsymlinkfilename: P0, lptargetfilename: P1, dwflags: SYMBOLIC_LINK_FLAGS) -> super::super::Foundation::BOOLEAN
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CreateSymbolicLinkA ( lpsymlinkfilename : ::windows::core::PCSTR , lptargetfilename : ::windows::core::PCSTR , dwflags : SYMBOLIC_LINK_FLAGS ) -> super::super::Foundation:: BOOLEAN );
    CreateSymbolicLinkA(lpsymlinkfilename.into_param().abi(), lptargetfilename.into_param().abi(), dwflags)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateSymbolicLinkTransactedA<P0, P1, P2>(lpsymlinkfilename: P0, lptargetfilename: P1, dwflags: SYMBOLIC_LINK_FLAGS, htransaction: P2) -> super::super::Foundation::BOOLEAN
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CreateSymbolicLinkTransactedA ( lpsymlinkfilename : ::windows::core::PCSTR , lptargetfilename : ::windows::core::PCSTR , dwflags : SYMBOLIC_LINK_FLAGS , htransaction : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOLEAN );
    CreateSymbolicLinkTransactedA(lpsymlinkfilename.into_param().abi(), lptargetfilename.into_param().abi(), dwflags, htransaction.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateSymbolicLinkTransactedW<P0, P1, P2>(lpsymlinkfilename: P0, lptargetfilename: P1, dwflags: SYMBOLIC_LINK_FLAGS, htransaction: P2) -> super::super::Foundation::BOOLEAN
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CreateSymbolicLinkTransactedW ( lpsymlinkfilename : ::windows::core::PCWSTR , lptargetfilename : ::windows::core::PCWSTR , dwflags : SYMBOLIC_LINK_FLAGS , htransaction : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOLEAN );
    CreateSymbolicLinkTransactedW(lpsymlinkfilename.into_param().abi(), lptargetfilename.into_param().abi(), dwflags, htransaction.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateSymbolicLinkW<P0, P1>(lpsymlinkfilename: P0, lptargetfilename: P1, dwflags: SYMBOLIC_LINK_FLAGS) -> super::super::Foundation::BOOLEAN
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CreateSymbolicLinkW ( lpsymlinkfilename : ::windows::core::PCWSTR , lptargetfilename : ::windows::core::PCWSTR , dwflags : SYMBOLIC_LINK_FLAGS ) -> super::super::Foundation:: BOOLEAN );
    CreateSymbolicLinkW(lpsymlinkfilename.into_param().abi(), lptargetfilename.into_param().abi(), dwflags)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateTapePartition<P0>(hdevice: P0, dwpartitionmethod: CREATE_TAPE_PARTITION_METHOD, dwcount: u32, dwsize: u32) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CreateTapePartition ( hdevice : super::super::Foundation:: HANDLE , dwpartitionmethod : CREATE_TAPE_PARTITION_METHOD , dwcount : u32 , dwsize : u32 ) -> u32 );
    CreateTapePartition(hdevice.into_param().abi(), dwpartitionmethod, dwcount, dwsize)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateTransaction<P0>(lptransactionattributes: *mut super::super::Security::SECURITY_ATTRIBUTES, uow: *mut ::windows::core::GUID, createoptions: u32, isolationlevel: u32, isolationflags: u32, timeout: u32, description: P0) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "ktmw32.dll""system" fn CreateTransaction ( lptransactionattributes : *mut super::super::Security:: SECURITY_ATTRIBUTES , uow : *mut ::windows::core::GUID , createoptions : u32 , isolationlevel : u32 , isolationflags : u32 , timeout : u32 , description : ::windows::core::PCWSTR ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateTransaction(lptransactionattributes, uow, createoptions, isolationlevel, isolationflags, timeout, description.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateTransactionManager<P0>(lptransactionattributes: *mut super::super::Security::SECURITY_ATTRIBUTES, logfilename: P0, createoptions: u32, commitstrength: u32) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "ktmw32.dll""system" fn CreateTransactionManager ( lptransactionattributes : *mut super::super::Security:: SECURITY_ATTRIBUTES , logfilename : ::windows::core::PCWSTR , createoptions : u32 , commitstrength : u32 ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateTransactionManager(lptransactionattributes, logfilename.into_param().abi(), createoptions, commitstrength);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DecryptFileA<P0>(lpfilename: P0, dwreserved: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn DecryptFileA ( lpfilename : ::windows::core::PCSTR , dwreserved : u32 ) -> super::super::Foundation:: BOOL );
    DecryptFileA(lpfilename.into_param().abi(), dwreserved)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DecryptFileW<P0>(lpfilename: P0, dwreserved: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn DecryptFileW ( lpfilename : ::windows::core::PCWSTR , dwreserved : u32 ) -> super::super::Foundation:: BOOL );
    DecryptFileW(lpfilename.into_param().abi(), dwreserved)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DefineDosDeviceA<P0, P1>(dwflags: DEFINE_DOS_DEVICE_FLAGS, lpdevicename: P0, lptargetpath: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn DefineDosDeviceA ( dwflags : DEFINE_DOS_DEVICE_FLAGS , lpdevicename : ::windows::core::PCSTR , lptargetpath : ::windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    DefineDosDeviceA(dwflags, lpdevicename.into_param().abi(), lptargetpath.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DefineDosDeviceW<P0, P1>(dwflags: DEFINE_DOS_DEVICE_FLAGS, lpdevicename: P0, lptargetpath: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn DefineDosDeviceW ( dwflags : DEFINE_DOS_DEVICE_FLAGS , lpdevicename : ::windows::core::PCWSTR , lptargetpath : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    DefineDosDeviceW(dwflags, lpdevicename.into_param().abi(), lptargetpath.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteFileA<P0>(lpfilename: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn DeleteFileA ( lpfilename : ::windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    DeleteFileA(lpfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteFileFromAppW<P0>(lpfilename: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "api-ms-win-core-file-fromapp-l1-1-0.dll""system" fn DeleteFileFromAppW ( lpfilename : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    DeleteFileFromAppW(lpfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteFileTransactedA<P0, P1>(lpfilename: P0, htransaction: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn DeleteFileTransactedA ( lpfilename : ::windows::core::PCSTR , htransaction : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    DeleteFileTransactedA(lpfilename.into_param().abi(), htransaction.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteFileTransactedW<P0, P1>(lpfilename: P0, htransaction: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn DeleteFileTransactedW ( lpfilename : ::windows::core::PCWSTR , htransaction : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    DeleteFileTransactedW(lpfilename.into_param().abi(), htransaction.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteFileW<P0>(lpfilename: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn DeleteFileW ( lpfilename : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    DeleteFileW(lpfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteLogByHandle<P0>(hlog: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "clfsw32.dll""system" fn DeleteLogByHandle ( hlog : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    DeleteLogByHandle(hlog.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteLogFile<P0>(pszlogfilename: P0, pvreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "clfsw32.dll""system" fn DeleteLogFile ( pszlogfilename : ::windows::core::PCWSTR , pvreserved : *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    DeleteLogFile(pszlogfilename.into_param().abi(), pvreserved)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteLogMarshallingArea(pvmarshal: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "clfsw32.dll""system" fn DeleteLogMarshallingArea ( pvmarshal : *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    DeleteLogMarshallingArea(pvmarshal)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteVolumeMountPointA<P0>(lpszvolumemountpoint: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn DeleteVolumeMountPointA ( lpszvolumemountpoint : ::windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    DeleteVolumeMountPointA(lpszvolumemountpoint.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteVolumeMountPointW<P0>(lpszvolumemountpoint: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn DeleteVolumeMountPointW ( lpszvolumemountpoint : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    DeleteVolumeMountPointW(lpszvolumemountpoint.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeregisterManageableLogClient<P0>(hlog: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "clfsw32.dll""system" fn DeregisterManageableLogClient ( hlog : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    DeregisterManageableLogClient(hlog.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn DuplicateEncryptionInfoFile<P0, P1>(srcfilename: P0, dstfilename: P1, dwcreationdistribution: u32, dwattributes: u32, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn DuplicateEncryptionInfoFile ( srcfilename : ::windows::core::PCWSTR , dstfilename : ::windows::core::PCWSTR , dwcreationdistribution : u32 , dwattributes : u32 , lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES ) -> u32 );
    DuplicateEncryptionInfoFile(srcfilename.into_param().abi(), dstfilename.into_param().abi(), dwcreationdistribution, dwattributes, ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EncryptFileA<P0>(lpfilename: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn EncryptFileA ( lpfilename : ::windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    EncryptFileA(lpfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EncryptFileW<P0>(lpfilename: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn EncryptFileW ( lpfilename : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    EncryptFileW(lpfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EncryptionDisable<P0, P1>(dirpath: P0, disable: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn EncryptionDisable ( dirpath : ::windows::core::PCWSTR , disable : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    EncryptionDisable(dirpath.into_param().abi(), disable.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EraseTape<P0, P1>(hdevice: P0, dwerasetype: ERASE_TAPE_TYPE, bimmediate: P1) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn EraseTape ( hdevice : super::super::Foundation:: HANDLE , dwerasetype : ERASE_TAPE_TYPE , bimmediate : super::super::Foundation:: BOOL ) -> u32 );
    EraseTape(hdevice.into_param().abi(), dwerasetype, bimmediate.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FileEncryptionStatusA<P0>(lpfilename: P0, lpstatus: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn FileEncryptionStatusA ( lpfilename : ::windows::core::PCSTR , lpstatus : *mut u32 ) -> super::super::Foundation:: BOOL );
    FileEncryptionStatusA(lpfilename.into_param().abi(), lpstatus)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FileEncryptionStatusW<P0>(lpfilename: P0, lpstatus: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn FileEncryptionStatusW ( lpfilename : ::windows::core::PCWSTR , lpstatus : *mut u32 ) -> super::super::Foundation:: BOOL );
    FileEncryptionStatusW(lpfilename.into_param().abi(), lpstatus)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FileTimeToLocalFileTime(lpfiletime: *const super::super::Foundation::FILETIME, lplocalfiletime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn FileTimeToLocalFileTime ( lpfiletime : *const super::super::Foundation:: FILETIME , lplocalfiletime : *mut super::super::Foundation:: FILETIME ) -> super::super::Foundation:: BOOL );
    FileTimeToLocalFileTime(lpfiletime, lplocalfiletime)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindClose<P0>(hfindfile: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<FindFileHandle>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn FindClose ( hfindfile : FindFileHandle ) -> super::super::Foundation:: BOOL );
    FindClose(hfindfile.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindCloseChangeNotification<P0>(hchangehandle: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<FindChangeNotificationHandle>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn FindCloseChangeNotification ( hchangehandle : FindChangeNotificationHandle ) -> super::super::Foundation:: BOOL );
    FindCloseChangeNotification(hchangehandle.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstChangeNotificationA<P0, P1>(lppathname: P0, bwatchsubtree: P1, dwnotifyfilter: FILE_NOTIFY_CHANGE) -> ::windows::core::Result<FindChangeNotificationHandle>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn FindFirstChangeNotificationA ( lppathname : ::windows::core::PCSTR , bwatchsubtree : super::super::Foundation:: BOOL , dwnotifyfilter : FILE_NOTIFY_CHANGE ) -> FindChangeNotificationHandle );
    let result__ = FindFirstChangeNotificationA(lppathname.into_param().abi(), bwatchsubtree.into_param().abi(), dwnotifyfilter);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstChangeNotificationW<P0, P1>(lppathname: P0, bwatchsubtree: P1, dwnotifyfilter: FILE_NOTIFY_CHANGE) -> ::windows::core::Result<FindChangeNotificationHandle>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn FindFirstChangeNotificationW ( lppathname : ::windows::core::PCWSTR , bwatchsubtree : super::super::Foundation:: BOOL , dwnotifyfilter : FILE_NOTIFY_CHANGE ) -> FindChangeNotificationHandle );
    let result__ = FindFirstChangeNotificationW(lppathname.into_param().abi(), bwatchsubtree.into_param().abi(), dwnotifyfilter);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstFileA<P0>(lpfilename: P0, lpfindfiledata: *mut WIN32_FIND_DATAA) -> ::windows::core::Result<FindFileHandle>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn FindFirstFileA ( lpfilename : ::windows::core::PCSTR , lpfindfiledata : *mut WIN32_FIND_DATAA ) -> FindFileHandle );
    let result__ = FindFirstFileA(lpfilename.into_param().abi(), lpfindfiledata);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn FindFirstFileExA<P0>(lpfilename: P0, finfolevelid: FINDEX_INFO_LEVELS, lpfindfiledata: *mut ::core::ffi::c_void, fsearchop: FINDEX_SEARCH_OPS, lpsearchfilter: ::core::option::Option<*const ::core::ffi::c_void>, dwadditionalflags: FIND_FIRST_EX_FLAGS) -> ::windows::core::Result<FindFileHandle>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn FindFirstFileExA ( lpfilename : ::windows::core::PCSTR , finfolevelid : FINDEX_INFO_LEVELS , lpfindfiledata : *mut ::core::ffi::c_void , fsearchop : FINDEX_SEARCH_OPS , lpsearchfilter : *const ::core::ffi::c_void , dwadditionalflags : FIND_FIRST_EX_FLAGS ) -> FindFileHandle );
    let result__ = FindFirstFileExA(lpfilename.into_param().abi(), finfolevelid, lpfindfiledata, fsearchop, ::core::mem::transmute(lpsearchfilter.unwrap_or(::std::ptr::null())), dwadditionalflags);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstFileExFromAppW<P0>(lpfilename: P0, finfolevelid: FINDEX_INFO_LEVELS, lpfindfiledata: *mut ::core::ffi::c_void, fsearchop: FINDEX_SEARCH_OPS, lpsearchfilter: ::core::option::Option<*const ::core::ffi::c_void>, dwadditionalflags: u32) -> super::super::Foundation::HANDLE
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "api-ms-win-core-file-fromapp-l1-1-0.dll""system" fn FindFirstFileExFromAppW ( lpfilename : ::windows::core::PCWSTR , finfolevelid : FINDEX_INFO_LEVELS , lpfindfiledata : *mut ::core::ffi::c_void , fsearchop : FINDEX_SEARCH_OPS , lpsearchfilter : *const ::core::ffi::c_void , dwadditionalflags : u32 ) -> super::super::Foundation:: HANDLE );
    FindFirstFileExFromAppW(lpfilename.into_param().abi(), finfolevelid, lpfindfiledata, fsearchop, ::core::mem::transmute(lpsearchfilter.unwrap_or(::std::ptr::null())), dwadditionalflags)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn FindFirstFileExW<P0>(lpfilename: P0, finfolevelid: FINDEX_INFO_LEVELS, lpfindfiledata: *mut ::core::ffi::c_void, fsearchop: FINDEX_SEARCH_OPS, lpsearchfilter: ::core::option::Option<*const ::core::ffi::c_void>, dwadditionalflags: FIND_FIRST_EX_FLAGS) -> ::windows::core::Result<FindFileHandle>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn FindFirstFileExW ( lpfilename : ::windows::core::PCWSTR , finfolevelid : FINDEX_INFO_LEVELS , lpfindfiledata : *mut ::core::ffi::c_void , fsearchop : FINDEX_SEARCH_OPS , lpsearchfilter : *const ::core::ffi::c_void , dwadditionalflags : FIND_FIRST_EX_FLAGS ) -> FindFileHandle );
    let result__ = FindFirstFileExW(lpfilename.into_param().abi(), finfolevelid, lpfindfiledata, fsearchop, ::core::mem::transmute(lpsearchfilter.unwrap_or(::std::ptr::null())), dwadditionalflags);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstFileNameTransactedW<P0, P1>(lpfilename: P0, dwflags: u32, stringlength: *mut u32, linkname: ::windows::core::PWSTR, htransaction: P1) -> ::windows::core::Result<FindFileNameHandle>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn FindFirstFileNameTransactedW ( lpfilename : ::windows::core::PCWSTR , dwflags : u32 , stringlength : *mut u32 , linkname : ::windows::core::PWSTR , htransaction : super::super::Foundation:: HANDLE ) -> FindFileNameHandle );
    let result__ = FindFirstFileNameTransactedW(lpfilename.into_param().abi(), dwflags, stringlength, ::core::mem::transmute(linkname), htransaction.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn FindFirstFileNameW<P0>(lpfilename: P0, dwflags: u32, stringlength: *mut u32, linkname: ::windows::core::PWSTR) -> ::windows::core::Result<FindFileNameHandle>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn FindFirstFileNameW ( lpfilename : ::windows::core::PCWSTR , dwflags : u32 , stringlength : *mut u32 , linkname : ::windows::core::PWSTR ) -> FindFileNameHandle );
    let result__ = FindFirstFileNameW(lpfilename.into_param().abi(), dwflags, stringlength, ::core::mem::transmute(linkname));
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstFileTransactedA<P0, P1>(lpfilename: P0, finfolevelid: FINDEX_INFO_LEVELS, lpfindfiledata: *mut ::core::ffi::c_void, fsearchop: FINDEX_SEARCH_OPS, lpsearchfilter: ::core::option::Option<*const ::core::ffi::c_void>, dwadditionalflags: u32, htransaction: P1) -> ::windows::core::Result<FindFileHandle>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn FindFirstFileTransactedA ( lpfilename : ::windows::core::PCSTR , finfolevelid : FINDEX_INFO_LEVELS , lpfindfiledata : *mut ::core::ffi::c_void , fsearchop : FINDEX_SEARCH_OPS , lpsearchfilter : *const ::core::ffi::c_void , dwadditionalflags : u32 , htransaction : super::super::Foundation:: HANDLE ) -> FindFileHandle );
    let result__ = FindFirstFileTransactedA(lpfilename.into_param().abi(), finfolevelid, lpfindfiledata, fsearchop, ::core::mem::transmute(lpsearchfilter.unwrap_or(::std::ptr::null())), dwadditionalflags, htransaction.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstFileTransactedW<P0, P1>(lpfilename: P0, finfolevelid: FINDEX_INFO_LEVELS, lpfindfiledata: *mut ::core::ffi::c_void, fsearchop: FINDEX_SEARCH_OPS, lpsearchfilter: ::core::option::Option<*const ::core::ffi::c_void>, dwadditionalflags: u32, htransaction: P1) -> ::windows::core::Result<FindFileHandle>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn FindFirstFileTransactedW ( lpfilename : ::windows::core::PCWSTR , finfolevelid : FINDEX_INFO_LEVELS , lpfindfiledata : *mut ::core::ffi::c_void , fsearchop : FINDEX_SEARCH_OPS , lpsearchfilter : *const ::core::ffi::c_void , dwadditionalflags : u32 , htransaction : super::super::Foundation:: HANDLE ) -> FindFileHandle );
    let result__ = FindFirstFileTransactedW(lpfilename.into_param().abi(), finfolevelid, lpfindfiledata, fsearchop, ::core::mem::transmute(lpsearchfilter.unwrap_or(::std::ptr::null())), dwadditionalflags, htransaction.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstFileW<P0>(lpfilename: P0, lpfindfiledata: *mut WIN32_FIND_DATAW) -> ::windows::core::Result<FindFileHandle>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn FindFirstFileW ( lpfilename : ::windows::core::PCWSTR , lpfindfiledata : *mut WIN32_FIND_DATAW ) -> FindFileHandle );
    let result__ = FindFirstFileW(lpfilename.into_param().abi(), lpfindfiledata);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstStreamTransactedW<P0, P1>(lpfilename: P0, infolevel: STREAM_INFO_LEVELS, lpfindstreamdata: *mut ::core::ffi::c_void, dwflags: u32, htransaction: P1) -> ::windows::core::Result<FindStreamHandle>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn FindFirstStreamTransactedW ( lpfilename : ::windows::core::PCWSTR , infolevel : STREAM_INFO_LEVELS , lpfindstreamdata : *mut ::core::ffi::c_void , dwflags : u32 , htransaction : super::super::Foundation:: HANDLE ) -> FindStreamHandle );
    let result__ = FindFirstStreamTransactedW(lpfilename.into_param().abi(), infolevel, lpfindstreamdata, dwflags, htransaction.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn FindFirstStreamW<P0>(lpfilename: P0, infolevel: STREAM_INFO_LEVELS, lpfindstreamdata: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::Result<FindStreamHandle>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn FindFirstStreamW ( lpfilename : ::windows::core::PCWSTR , infolevel : STREAM_INFO_LEVELS , lpfindstreamdata : *mut ::core::ffi::c_void , dwflags : u32 ) -> FindStreamHandle );
    let result__ = FindFirstStreamW(lpfilename.into_param().abi(), infolevel, lpfindstreamdata, dwflags);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn FindFirstVolumeA(lpszvolumename: &mut [u8]) -> ::windows::core::Result<FindVolumeHandle> {
    ::windows_targets::link ! ( "kernel32.dll""system" fn FindFirstVolumeA ( lpszvolumename : ::windows::core::PSTR , cchbufferlength : u32 ) -> FindVolumeHandle );
    let result__ = FindFirstVolumeA(::core::mem::transmute(lpszvolumename.as_ptr()), lpszvolumename.len() as _);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn FindFirstVolumeMountPointA<P0>(lpszrootpathname: P0, lpszvolumemountpoint: &mut [u8]) -> ::windows::core::Result<FindVolumeMountPointHandle>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn FindFirstVolumeMountPointA ( lpszrootpathname : ::windows::core::PCSTR , lpszvolumemountpoint : ::windows::core::PSTR , cchbufferlength : u32 ) -> FindVolumeMountPointHandle );
    let result__ = FindFirstVolumeMountPointA(lpszrootpathname.into_param().abi(), ::core::mem::transmute(lpszvolumemountpoint.as_ptr()), lpszvolumemountpoint.len() as _);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn FindFirstVolumeMountPointW<P0>(lpszrootpathname: P0, lpszvolumemountpoint: &mut [u16]) -> ::windows::core::Result<FindVolumeMountPointHandle>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn FindFirstVolumeMountPointW ( lpszrootpathname : ::windows::core::PCWSTR , lpszvolumemountpoint : ::windows::core::PWSTR , cchbufferlength : u32 ) -> FindVolumeMountPointHandle );
    let result__ = FindFirstVolumeMountPointW(lpszrootpathname.into_param().abi(), ::core::mem::transmute(lpszvolumemountpoint.as_ptr()), lpszvolumemountpoint.len() as _);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn FindFirstVolumeW(lpszvolumename: &mut [u16]) -> ::windows::core::Result<FindVolumeHandle> {
    ::windows_targets::link ! ( "kernel32.dll""system" fn FindFirstVolumeW ( lpszvolumename : ::windows::core::PWSTR , cchbufferlength : u32 ) -> FindVolumeHandle );
    let result__ = FindFirstVolumeW(::core::mem::transmute(lpszvolumename.as_ptr()), lpszvolumename.len() as _);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindNextChangeNotification<P0>(hchangehandle: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<FindChangeNotificationHandle>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn FindNextChangeNotification ( hchangehandle : FindChangeNotificationHandle ) -> super::super::Foundation:: BOOL );
    FindNextChangeNotification(hchangehandle.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindNextFileA<P0>(hfindfile: P0, lpfindfiledata: *mut WIN32_FIND_DATAA) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<FindFileHandle>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn FindNextFileA ( hfindfile : FindFileHandle , lpfindfiledata : *mut WIN32_FIND_DATAA ) -> super::super::Foundation:: BOOL );
    FindNextFileA(hfindfile.into_param().abi(), lpfindfiledata)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindNextFileNameW<P0>(hfindstream: P0, stringlength: *mut u32, linkname: ::windows::core::PWSTR) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<FindFileNameHandle>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn FindNextFileNameW ( hfindstream : FindFileNameHandle , stringlength : *mut u32 , linkname : ::windows::core::PWSTR ) -> super::super::Foundation:: BOOL );
    FindNextFileNameW(hfindstream.into_param().abi(), stringlength, ::core::mem::transmute(linkname))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindNextFileW<P0>(hfindfile: P0, lpfindfiledata: *mut WIN32_FIND_DATAW) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<FindFileHandle>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn FindNextFileW ( hfindfile : FindFileHandle , lpfindfiledata : *mut WIN32_FIND_DATAW ) -> super::super::Foundation:: BOOL );
    FindNextFileW(hfindfile.into_param().abi(), lpfindfiledata)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindNextStreamW<P0>(hfindstream: P0, lpfindstreamdata: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<FindStreamHandle>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn FindNextStreamW ( hfindstream : FindStreamHandle , lpfindstreamdata : *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    FindNextStreamW(hfindstream.into_param().abi(), lpfindstreamdata)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindNextVolumeA<P0>(hfindvolume: P0, lpszvolumename: &mut [u8]) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<FindVolumeHandle>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn FindNextVolumeA ( hfindvolume : FindVolumeHandle , lpszvolumename : ::windows::core::PSTR , cchbufferlength : u32 ) -> super::super::Foundation:: BOOL );
    FindNextVolumeA(hfindvolume.into_param().abi(), ::core::mem::transmute(lpszvolumename.as_ptr()), lpszvolumename.len() as _)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindNextVolumeMountPointA<P0>(hfindvolumemountpoint: P0, lpszvolumemountpoint: &mut [u8]) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<FindVolumeMountPointHandle>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn FindNextVolumeMountPointA ( hfindvolumemountpoint : FindVolumeMountPointHandle , lpszvolumemountpoint : ::windows::core::PSTR , cchbufferlength : u32 ) -> super::super::Foundation:: BOOL );
    FindNextVolumeMountPointA(hfindvolumemountpoint.into_param().abi(), ::core::mem::transmute(lpszvolumemountpoint.as_ptr()), lpszvolumemountpoint.len() as _)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindNextVolumeMountPointW<P0>(hfindvolumemountpoint: P0, lpszvolumemountpoint: &mut [u16]) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<FindVolumeMountPointHandle>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn FindNextVolumeMountPointW ( hfindvolumemountpoint : FindVolumeMountPointHandle , lpszvolumemountpoint : ::windows::core::PWSTR , cchbufferlength : u32 ) -> super::super::Foundation:: BOOL );
    FindNextVolumeMountPointW(hfindvolumemountpoint.into_param().abi(), ::core::mem::transmute(lpszvolumemountpoint.as_ptr()), lpszvolumemountpoint.len() as _)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindNextVolumeW<P0>(hfindvolume: P0, lpszvolumename: &mut [u16]) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<FindVolumeHandle>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn FindNextVolumeW ( hfindvolume : FindVolumeHandle , lpszvolumename : ::windows::core::PWSTR , cchbufferlength : u32 ) -> super::super::Foundation:: BOOL );
    FindNextVolumeW(hfindvolume.into_param().abi(), ::core::mem::transmute(lpszvolumename.as_ptr()), lpszvolumename.len() as _)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindVolumeClose<P0>(hfindvolume: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<FindVolumeHandle>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn FindVolumeClose ( hfindvolume : FindVolumeHandle ) -> super::super::Foundation:: BOOL );
    FindVolumeClose(hfindvolume.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindVolumeMountPointClose<P0>(hfindvolumemountpoint: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<FindVolumeMountPointHandle>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn FindVolumeMountPointClose ( hfindvolumemountpoint : FindVolumeMountPointHandle ) -> super::super::Foundation:: BOOL );
    FindVolumeMountPointClose(hfindvolumemountpoint.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FlushFileBuffers<P0>(hfile: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn FlushFileBuffers ( hfile : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    FlushFileBuffers(hfile.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn FlushLogBuffers(pvmarshal: *const ::core::ffi::c_void, poverlapped: ::core::option::Option<*mut super::super::System::IO::OVERLAPPED>) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "clfsw32.dll""system" fn FlushLogBuffers ( pvmarshal : *const ::core::ffi::c_void , poverlapped : *mut super::super::System::IO:: OVERLAPPED ) -> super::super::Foundation:: BOOL );
    FlushLogBuffers(pvmarshal, ::core::mem::transmute(poverlapped.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn FlushLogToLsn(pvmarshalcontext: *mut ::core::ffi::c_void, plsnflush: *mut CLS_LSN, plsnlastflushed: *mut CLS_LSN, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "clfsw32.dll""system" fn FlushLogToLsn ( pvmarshalcontext : *mut ::core::ffi::c_void , plsnflush : *mut CLS_LSN , plsnlastflushed : *mut CLS_LSN , poverlapped : *mut super::super::System::IO:: OVERLAPPED ) -> super::super::Foundation:: BOOL );
    FlushLogToLsn(pvmarshalcontext, plsnflush, plsnlastflushed, poverlapped)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn FreeEncryptedFileMetadata(pbmetadata: *const u8) {
    ::windows_targets::link ! ( "advapi32.dll""system" fn FreeEncryptedFileMetadata ( pbmetadata : *const u8 ) -> ( ) );
    FreeEncryptedFileMetadata(pbmetadata)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FreeEncryptionCertificateHashList(pusers: *const ENCRYPTION_CERTIFICATE_HASH_LIST) {
    ::windows_targets::link ! ( "advapi32.dll""system" fn FreeEncryptionCertificateHashList ( pusers : *const ENCRYPTION_CERTIFICATE_HASH_LIST ) -> ( ) );
    FreeEncryptionCertificateHashList(pusers)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeReservedLog(pvmarshal: *mut ::core::ffi::c_void, creservedrecords: u32, pcbadjustment: *mut i64) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "clfsw32.dll""system" fn FreeReservedLog ( pvmarshal : *mut ::core::ffi::c_void , creservedrecords : u32 , pcbadjustment : *mut i64 ) -> super::super::Foundation:: BOOL );
    FreeReservedLog(pvmarshal, creservedrecords, pcbadjustment)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetBinaryTypeA<P0>(lpapplicationname: P0, lpbinarytype: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetBinaryTypeA ( lpapplicationname : ::windows::core::PCSTR , lpbinarytype : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetBinaryTypeA(lpapplicationname.into_param().abi(), lpbinarytype)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetBinaryTypeW<P0>(lpapplicationname: P0, lpbinarytype: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetBinaryTypeW ( lpapplicationname : ::windows::core::PCWSTR , lpbinarytype : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetBinaryTypeW(lpapplicationname.into_param().abi(), lpbinarytype)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn GetCompressedFileSizeA<P0>(lpfilename: P0, lpfilesizehigh: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetCompressedFileSizeA ( lpfilename : ::windows::core::PCSTR , lpfilesizehigh : *mut u32 ) -> u32 );
    GetCompressedFileSizeA(lpfilename.into_param().abi(), ::core::mem::transmute(lpfilesizehigh.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCompressedFileSizeTransactedA<P0, P1>(lpfilename: P0, lpfilesizehigh: ::core::option::Option<*mut u32>, htransaction: P1) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetCompressedFileSizeTransactedA ( lpfilename : ::windows::core::PCSTR , lpfilesizehigh : *mut u32 , htransaction : super::super::Foundation:: HANDLE ) -> u32 );
    GetCompressedFileSizeTransactedA(lpfilename.into_param().abi(), ::core::mem::transmute(lpfilesizehigh.unwrap_or(::std::ptr::null_mut())), htransaction.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCompressedFileSizeTransactedW<P0, P1>(lpfilename: P0, lpfilesizehigh: ::core::option::Option<*mut u32>, htransaction: P1) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetCompressedFileSizeTransactedW ( lpfilename : ::windows::core::PCWSTR , lpfilesizehigh : *mut u32 , htransaction : super::super::Foundation:: HANDLE ) -> u32 );
    GetCompressedFileSizeTransactedW(lpfilename.into_param().abi(), ::core::mem::transmute(lpfilesizehigh.unwrap_or(::std::ptr::null_mut())), htransaction.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn GetCompressedFileSizeW<P0>(lpfilename: P0, lpfilesizehigh: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetCompressedFileSizeW ( lpfilename : ::windows::core::PCWSTR , lpfilesizehigh : *mut u32 ) -> u32 );
    GetCompressedFileSizeW(lpfilename.into_param().abi(), ::core::mem::transmute(lpfilesizehigh.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentClockTransactionManager<P0>(transactionmanagerhandle: P0, tmvirtualclock: *mut i64) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ktmw32.dll""system" fn GetCurrentClockTransactionManager ( transactionmanagerhandle : super::super::Foundation:: HANDLE , tmvirtualclock : *mut i64 ) -> super::super::Foundation:: BOOL );
    GetCurrentClockTransactionManager(transactionmanagerhandle.into_param().abi(), tmvirtualclock)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDiskFreeSpaceA<P0>(lprootpathname: P0, lpsectorspercluster: ::core::option::Option<*mut u32>, lpbytespersector: ::core::option::Option<*mut u32>, lpnumberoffreeclusters: ::core::option::Option<*mut u32>, lptotalnumberofclusters: ::core::option::Option<*mut u32>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetDiskFreeSpaceA ( lprootpathname : ::windows::core::PCSTR , lpsectorspercluster : *mut u32 , lpbytespersector : *mut u32 , lpnumberoffreeclusters : *mut u32 , lptotalnumberofclusters : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetDiskFreeSpaceA(lprootpathname.into_param().abi(), ::core::mem::transmute(lpsectorspercluster.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpbytespersector.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpnumberoffreeclusters.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lptotalnumberofclusters.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDiskFreeSpaceExA<P0>(lpdirectoryname: P0, lpfreebytesavailabletocaller: ::core::option::Option<*mut u64>, lptotalnumberofbytes: ::core::option::Option<*mut u64>, lptotalnumberoffreebytes: ::core::option::Option<*mut u64>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetDiskFreeSpaceExA ( lpdirectoryname : ::windows::core::PCSTR , lpfreebytesavailabletocaller : *mut u64 , lptotalnumberofbytes : *mut u64 , lptotalnumberoffreebytes : *mut u64 ) -> super::super::Foundation:: BOOL );
    GetDiskFreeSpaceExA(lpdirectoryname.into_param().abi(), ::core::mem::transmute(lpfreebytesavailabletocaller.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lptotalnumberofbytes.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lptotalnumberoffreebytes.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDiskFreeSpaceExW<P0>(lpdirectoryname: P0, lpfreebytesavailabletocaller: ::core::option::Option<*mut u64>, lptotalnumberofbytes: ::core::option::Option<*mut u64>, lptotalnumberoffreebytes: ::core::option::Option<*mut u64>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetDiskFreeSpaceExW ( lpdirectoryname : ::windows::core::PCWSTR , lpfreebytesavailabletocaller : *mut u64 , lptotalnumberofbytes : *mut u64 , lptotalnumberoffreebytes : *mut u64 ) -> super::super::Foundation:: BOOL );
    GetDiskFreeSpaceExW(lpdirectoryname.into_param().abi(), ::core::mem::transmute(lpfreebytesavailabletocaller.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lptotalnumberofbytes.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lptotalnumberoffreebytes.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDiskFreeSpaceW<P0>(lprootpathname: P0, lpsectorspercluster: ::core::option::Option<*mut u32>, lpbytespersector: ::core::option::Option<*mut u32>, lpnumberoffreeclusters: ::core::option::Option<*mut u32>, lptotalnumberofclusters: ::core::option::Option<*mut u32>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetDiskFreeSpaceW ( lprootpathname : ::windows::core::PCWSTR , lpsectorspercluster : *mut u32 , lpbytespersector : *mut u32 , lpnumberoffreeclusters : *mut u32 , lptotalnumberofclusters : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetDiskFreeSpaceW(lprootpathname.into_param().abi(), ::core::mem::transmute(lpsectorspercluster.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpbytespersector.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpnumberoffreeclusters.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lptotalnumberofclusters.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn GetDiskSpaceInformationA<P0>(rootpath: P0, diskspaceinfo: *mut DISK_SPACE_INFORMATION) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetDiskSpaceInformationA ( rootpath : ::windows::core::PCSTR , diskspaceinfo : *mut DISK_SPACE_INFORMATION ) -> ::windows::core::HRESULT );
    GetDiskSpaceInformationA(rootpath.into_param().abi(), diskspaceinfo).ok()
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn GetDiskSpaceInformationW<P0>(rootpath: P0, diskspaceinfo: *mut DISK_SPACE_INFORMATION) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetDiskSpaceInformationW ( rootpath : ::windows::core::PCWSTR , diskspaceinfo : *mut DISK_SPACE_INFORMATION ) -> ::windows::core::HRESULT );
    GetDiskSpaceInformationW(rootpath.into_param().abi(), diskspaceinfo).ok()
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn GetDriveTypeA<P0>(lprootpathname: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetDriveTypeA ( lprootpathname : ::windows::core::PCSTR ) -> u32 );
    GetDriveTypeA(lprootpathname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn GetDriveTypeW<P0>(lprootpathname: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetDriveTypeW ( lprootpathname : ::windows::core::PCWSTR ) -> u32 );
    GetDriveTypeW(lprootpathname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn GetEncryptedFileMetadata<P0>(lpfilename: P0, pcbmetadata: *mut u32, ppbmetadata: *mut *mut u8) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn GetEncryptedFileMetadata ( lpfilename : ::windows::core::PCWSTR , pcbmetadata : *mut u32 , ppbmetadata : *mut *mut u8 ) -> u32 );
    GetEncryptedFileMetadata(lpfilename.into_param().abi(), pcbmetadata, ppbmetadata)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetEnlistmentId<P0>(enlistmenthandle: P0, enlistmentid: *mut ::windows::core::GUID) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ktmw32.dll""system" fn GetEnlistmentId ( enlistmenthandle : super::super::Foundation:: HANDLE , enlistmentid : *mut ::windows::core::GUID ) -> super::super::Foundation:: BOOL );
    GetEnlistmentId(enlistmenthandle.into_param().abi(), enlistmentid)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetEnlistmentRecoveryInformation<P0>(enlistmenthandle: P0, buffersize: u32, buffer: *mut ::core::ffi::c_void, bufferused: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ktmw32.dll""system" fn GetEnlistmentRecoveryInformation ( enlistmenthandle : super::super::Foundation:: HANDLE , buffersize : u32 , buffer : *mut ::core::ffi::c_void , bufferused : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetEnlistmentRecoveryInformation(enlistmenthandle.into_param().abi(), buffersize, buffer, bufferused)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn GetExpandedNameA<P0>(lpszsource: P0, lpszbuffer: &mut [u8; 260]) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetExpandedNameA ( lpszsource : ::windows::core::PCSTR , lpszbuffer : ::windows::core::PSTR ) -> i32 );
    GetExpandedNameA(lpszsource.into_param().abi(), ::core::mem::transmute(lpszbuffer.as_ptr()))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn GetExpandedNameW<P0>(lpszsource: P0, lpszbuffer: &mut [u16; 260]) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetExpandedNameW ( lpszsource : ::windows::core::PCWSTR , lpszbuffer : ::windows::core::PWSTR ) -> i32 );
    GetExpandedNameW(lpszsource.into_param().abi(), ::core::mem::transmute(lpszbuffer.as_ptr()))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn GetFileAttributesA<P0>(lpfilename: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetFileAttributesA ( lpfilename : ::windows::core::PCSTR ) -> u32 );
    GetFileAttributesA(lpfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileAttributesExA<P0>(lpfilename: P0, finfolevelid: GET_FILEEX_INFO_LEVELS, lpfileinformation: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetFileAttributesExA ( lpfilename : ::windows::core::PCSTR , finfolevelid : GET_FILEEX_INFO_LEVELS , lpfileinformation : *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    GetFileAttributesExA(lpfilename.into_param().abi(), finfolevelid, lpfileinformation)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileAttributesExFromAppW<P0>(lpfilename: P0, finfolevelid: GET_FILEEX_INFO_LEVELS, lpfileinformation: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "api-ms-win-core-file-fromapp-l1-1-0.dll""system" fn GetFileAttributesExFromAppW ( lpfilename : ::windows::core::PCWSTR , finfolevelid : GET_FILEEX_INFO_LEVELS , lpfileinformation : *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    GetFileAttributesExFromAppW(lpfilename.into_param().abi(), finfolevelid, lpfileinformation)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileAttributesExW<P0>(lpfilename: P0, finfolevelid: GET_FILEEX_INFO_LEVELS, lpfileinformation: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetFileAttributesExW ( lpfilename : ::windows::core::PCWSTR , finfolevelid : GET_FILEEX_INFO_LEVELS , lpfileinformation : *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    GetFileAttributesExW(lpfilename.into_param().abi(), finfolevelid, lpfileinformation)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileAttributesTransactedA<P0, P1>(lpfilename: P0, finfolevelid: GET_FILEEX_INFO_LEVELS, lpfileinformation: *mut ::core::ffi::c_void, htransaction: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetFileAttributesTransactedA ( lpfilename : ::windows::core::PCSTR , finfolevelid : GET_FILEEX_INFO_LEVELS , lpfileinformation : *mut ::core::ffi::c_void , htransaction : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    GetFileAttributesTransactedA(lpfilename.into_param().abi(), finfolevelid, lpfileinformation, htransaction.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileAttributesTransactedW<P0, P1>(lpfilename: P0, finfolevelid: GET_FILEEX_INFO_LEVELS, lpfileinformation: *mut ::core::ffi::c_void, htransaction: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetFileAttributesTransactedW ( lpfilename : ::windows::core::PCWSTR , finfolevelid : GET_FILEEX_INFO_LEVELS , lpfileinformation : *mut ::core::ffi::c_void , htransaction : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    GetFileAttributesTransactedW(lpfilename.into_param().abi(), finfolevelid, lpfileinformation, htransaction.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn GetFileAttributesW<P0>(lpfilename: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetFileAttributesW ( lpfilename : ::windows::core::PCWSTR ) -> u32 );
    GetFileAttributesW(lpfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileBandwidthReservation<P0>(hfile: P0, lpperiodmilliseconds: *mut u32, lpbytesperperiod: *mut u32, pdiscardable: *mut i32, lptransfersize: *mut u32, lpnumoutstandingrequests: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetFileBandwidthReservation ( hfile : super::super::Foundation:: HANDLE , lpperiodmilliseconds : *mut u32 , lpbytesperperiod : *mut u32 , pdiscardable : *mut i32 , lptransfersize : *mut u32 , lpnumoutstandingrequests : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetFileBandwidthReservation(hfile.into_param().abi(), lpperiodmilliseconds, lpbytesperperiod, pdiscardable, lptransfersize, lpnumoutstandingrequests)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileInformationByHandle<P0>(hfile: P0, lpfileinformation: *mut BY_HANDLE_FILE_INFORMATION) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetFileInformationByHandle ( hfile : super::super::Foundation:: HANDLE , lpfileinformation : *mut BY_HANDLE_FILE_INFORMATION ) -> super::super::Foundation:: BOOL );
    GetFileInformationByHandle(hfile.into_param().abi(), lpfileinformation)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileInformationByHandleEx<P0>(hfile: P0, fileinformationclass: FILE_INFO_BY_HANDLE_CLASS, lpfileinformation: *mut ::core::ffi::c_void, dwbuffersize: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetFileInformationByHandleEx ( hfile : super::super::Foundation:: HANDLE , fileinformationclass : FILE_INFO_BY_HANDLE_CLASS , lpfileinformation : *mut ::core::ffi::c_void , dwbuffersize : u32 ) -> super::super::Foundation:: BOOL );
    GetFileInformationByHandleEx(hfile.into_param().abi(), fileinformationclass, lpfileinformation, dwbuffersize)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileSize<P0>(hfile: P0, lpfilesizehigh: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetFileSize ( hfile : super::super::Foundation:: HANDLE , lpfilesizehigh : *mut u32 ) -> u32 );
    GetFileSize(hfile.into_param().abi(), ::core::mem::transmute(lpfilesizehigh.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileSizeEx<P0>(hfile: P0, lpfilesize: *mut i64) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetFileSizeEx ( hfile : super::super::Foundation:: HANDLE , lpfilesize : *mut i64 ) -> super::super::Foundation:: BOOL );
    GetFileSizeEx(hfile.into_param().abi(), lpfilesize)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileTime<P0>(hfile: P0, lpcreationtime: ::core::option::Option<*mut super::super::Foundation::FILETIME>, lplastaccesstime: ::core::option::Option<*mut super::super::Foundation::FILETIME>, lplastwritetime: ::core::option::Option<*mut super::super::Foundation::FILETIME>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetFileTime ( hfile : super::super::Foundation:: HANDLE , lpcreationtime : *mut super::super::Foundation:: FILETIME , lplastaccesstime : *mut super::super::Foundation:: FILETIME , lplastwritetime : *mut super::super::Foundation:: FILETIME ) -> super::super::Foundation:: BOOL );
    GetFileTime(hfile.into_param().abi(), ::core::mem::transmute(lpcreationtime.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lplastaccesstime.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lplastwritetime.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileType<P0>(hfile: P0) -> FILE_TYPE
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetFileType ( hfile : super::super::Foundation:: HANDLE ) -> FILE_TYPE );
    GetFileType(hfile.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileVersionInfoA<P0>(lptstrfilename: P0, dwhandle: u32, dwlen: u32, lpdata: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "version.dll""system" fn GetFileVersionInfoA ( lptstrfilename : ::windows::core::PCSTR , dwhandle : u32 , dwlen : u32 , lpdata : *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    GetFileVersionInfoA(lptstrfilename.into_param().abi(), dwhandle, dwlen, lpdata)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileVersionInfoExA<P0>(dwflags: GET_FILE_VERSION_INFO_FLAGS, lpwstrfilename: P0, dwhandle: u32, dwlen: u32, lpdata: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "version.dll""system" fn GetFileVersionInfoExA ( dwflags : GET_FILE_VERSION_INFO_FLAGS , lpwstrfilename : ::windows::core::PCSTR , dwhandle : u32 , dwlen : u32 , lpdata : *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    GetFileVersionInfoExA(dwflags, lpwstrfilename.into_param().abi(), dwhandle, dwlen, lpdata)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileVersionInfoExW<P0>(dwflags: GET_FILE_VERSION_INFO_FLAGS, lpwstrfilename: P0, dwhandle: u32, dwlen: u32, lpdata: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "version.dll""system" fn GetFileVersionInfoExW ( dwflags : GET_FILE_VERSION_INFO_FLAGS , lpwstrfilename : ::windows::core::PCWSTR , dwhandle : u32 , dwlen : u32 , lpdata : *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    GetFileVersionInfoExW(dwflags, lpwstrfilename.into_param().abi(), dwhandle, dwlen, lpdata)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn GetFileVersionInfoSizeA<P0>(lptstrfilename: P0, lpdwhandle: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "version.dll""system" fn GetFileVersionInfoSizeA ( lptstrfilename : ::windows::core::PCSTR , lpdwhandle : *mut u32 ) -> u32 );
    GetFileVersionInfoSizeA(lptstrfilename.into_param().abi(), ::core::mem::transmute(lpdwhandle.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn GetFileVersionInfoSizeExA<P0>(dwflags: GET_FILE_VERSION_INFO_FLAGS, lpwstrfilename: P0, lpdwhandle: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "version.dll""system" fn GetFileVersionInfoSizeExA ( dwflags : GET_FILE_VERSION_INFO_FLAGS , lpwstrfilename : ::windows::core::PCSTR , lpdwhandle : *mut u32 ) -> u32 );
    GetFileVersionInfoSizeExA(dwflags, lpwstrfilename.into_param().abi(), lpdwhandle)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn GetFileVersionInfoSizeExW<P0>(dwflags: GET_FILE_VERSION_INFO_FLAGS, lpwstrfilename: P0, lpdwhandle: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "version.dll""system" fn GetFileVersionInfoSizeExW ( dwflags : GET_FILE_VERSION_INFO_FLAGS , lpwstrfilename : ::windows::core::PCWSTR , lpdwhandle : *mut u32 ) -> u32 );
    GetFileVersionInfoSizeExW(dwflags, lpwstrfilename.into_param().abi(), lpdwhandle)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn GetFileVersionInfoSizeW<P0>(lptstrfilename: P0, lpdwhandle: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "version.dll""system" fn GetFileVersionInfoSizeW ( lptstrfilename : ::windows::core::PCWSTR , lpdwhandle : *mut u32 ) -> u32 );
    GetFileVersionInfoSizeW(lptstrfilename.into_param().abi(), ::core::mem::transmute(lpdwhandle.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileVersionInfoW<P0>(lptstrfilename: P0, dwhandle: u32, dwlen: u32, lpdata: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "version.dll""system" fn GetFileVersionInfoW ( lptstrfilename : ::windows::core::PCWSTR , dwhandle : u32 , dwlen : u32 , lpdata : *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    GetFileVersionInfoW(lptstrfilename.into_param().abi(), dwhandle, dwlen, lpdata)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFinalPathNameByHandleA<P0>(hfile: P0, lpszfilepath: &mut [u8], dwflags: FILE_NAME) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetFinalPathNameByHandleA ( hfile : super::super::Foundation:: HANDLE , lpszfilepath : ::windows::core::PSTR , cchfilepath : u32 , dwflags : FILE_NAME ) -> u32 );
    GetFinalPathNameByHandleA(hfile.into_param().abi(), ::core::mem::transmute(lpszfilepath.as_ptr()), lpszfilepath.len() as _, dwflags)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFinalPathNameByHandleW<P0>(hfile: P0, lpszfilepath: &mut [u16], dwflags: FILE_NAME) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetFinalPathNameByHandleW ( hfile : super::super::Foundation:: HANDLE , lpszfilepath : ::windows::core::PWSTR , cchfilepath : u32 , dwflags : FILE_NAME ) -> u32 );
    GetFinalPathNameByHandleW(hfile.into_param().abi(), ::core::mem::transmute(lpszfilepath.as_ptr()), lpszfilepath.len() as _, dwflags)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn GetFullPathNameA<P0>(lpfilename: P0, lpbuffer: ::core::option::Option<&mut [u8]>, lpfilepart: ::core::option::Option<*mut ::windows::core::PSTR>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetFullPathNameA ( lpfilename : ::windows::core::PCSTR , nbufferlength : u32 , lpbuffer : ::windows::core::PSTR , lpfilepart : *mut ::windows::core::PSTR ) -> u32 );
    GetFullPathNameA(lpfilename.into_param().abi(), lpbuffer.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lpbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ::core::mem::transmute(lpfilepart.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFullPathNameTransactedA<P0, P1>(lpfilename: P0, lpbuffer: ::core::option::Option<&mut [u8]>, lpfilepart: ::core::option::Option<*mut ::windows::core::PSTR>, htransaction: P1) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetFullPathNameTransactedA ( lpfilename : ::windows::core::PCSTR , nbufferlength : u32 , lpbuffer : ::windows::core::PSTR , lpfilepart : *mut ::windows::core::PSTR , htransaction : super::super::Foundation:: HANDLE ) -> u32 );
    GetFullPathNameTransactedA(lpfilename.into_param().abi(), lpbuffer.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lpbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ::core::mem::transmute(lpfilepart.unwrap_or(::std::ptr::null_mut())), htransaction.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFullPathNameTransactedW<P0, P1>(lpfilename: P0, lpbuffer: ::core::option::Option<&mut [u16]>, lpfilepart: ::core::option::Option<*mut ::windows::core::PWSTR>, htransaction: P1) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetFullPathNameTransactedW ( lpfilename : ::windows::core::PCWSTR , nbufferlength : u32 , lpbuffer : ::windows::core::PWSTR , lpfilepart : *mut ::windows::core::PWSTR , htransaction : super::super::Foundation:: HANDLE ) -> u32 );
    GetFullPathNameTransactedW(lpfilename.into_param().abi(), lpbuffer.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lpbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ::core::mem::transmute(lpfilepart.unwrap_or(::std::ptr::null_mut())), htransaction.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn GetFullPathNameW<P0>(lpfilename: P0, lpbuffer: ::core::option::Option<&mut [u16]>, lpfilepart: ::core::option::Option<*mut ::windows::core::PWSTR>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetFullPathNameW ( lpfilename : ::windows::core::PCWSTR , nbufferlength : u32 , lpbuffer : ::windows::core::PWSTR , lpfilepart : *mut ::windows::core::PWSTR ) -> u32 );
    GetFullPathNameW(lpfilename.into_param().abi(), lpbuffer.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lpbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ::core::mem::transmute(lpfilepart.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn GetIoRingInfo(ioring: *const HIORING__, info: *mut IORING_INFO) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "api-ms-win-core-ioring-l1-1-0.dll""system" fn GetIoRingInfo ( ioring : *const HIORING__ , info : *mut IORING_INFO ) -> ::windows::core::HRESULT );
    GetIoRingInfo(ioring, info).ok()
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetLogContainerName<P0, P1>(hlog: P0, cidlogicalcontainer: u32, pwstrcontainername: P1, clencontainername: u32, pcactuallencontainername: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "clfsw32.dll""system" fn GetLogContainerName ( hlog : super::super::Foundation:: HANDLE , cidlogicalcontainer : u32 , pwstrcontainername : ::windows::core::PCWSTR , clencontainername : u32 , pcactuallencontainername : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetLogContainerName(hlog.into_param().abi(), cidlogicalcontainer, pwstrcontainername.into_param().abi(), clencontainername, pcactuallencontainername)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetLogFileInformation<P0>(hlog: P0, pinfobuffer: *mut CLS_INFORMATION, cbbuffer: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "clfsw32.dll""system" fn GetLogFileInformation ( hlog : super::super::Foundation:: HANDLE , pinfobuffer : *mut CLS_INFORMATION , cbbuffer : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetLogFileInformation(hlog.into_param().abi(), pinfobuffer, cbbuffer)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetLogIoStatistics<P0>(hlog: P0, pvstatsbuffer: *mut ::core::ffi::c_void, cbstatsbuffer: u32, estatsclass: CLFS_IOSTATS_CLASS, pcbstatswritten: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "clfsw32.dll""system" fn GetLogIoStatistics ( hlog : super::super::Foundation:: HANDLE , pvstatsbuffer : *mut ::core::ffi::c_void , cbstatsbuffer : u32 , estatsclass : CLFS_IOSTATS_CLASS , pcbstatswritten : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetLogIoStatistics(hlog.into_param().abi(), pvstatsbuffer, cbstatsbuffer, estatsclass, pcbstatswritten)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetLogReservationInfo(pvmarshal: *const ::core::ffi::c_void, pcbrecordnumber: *mut u32, pcbuserreservation: *mut i64, pcbcommitreservation: *mut i64) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "clfsw32.dll""system" fn GetLogReservationInfo ( pvmarshal : *const ::core::ffi::c_void , pcbrecordnumber : *mut u32 , pcbuserreservation : *mut i64 , pcbcommitreservation : *mut i64 ) -> super::super::Foundation:: BOOL );
    GetLogReservationInfo(pvmarshal, pcbrecordnumber, pcbuserreservation, pcbcommitreservation)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn GetLogicalDriveStringsA(lpbuffer: ::core::option::Option<&mut [u8]>) -> u32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetLogicalDriveStringsA ( nbufferlength : u32 , lpbuffer : ::windows::core::PSTR ) -> u32 );
    GetLogicalDriveStringsA(lpbuffer.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lpbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn GetLogicalDriveStringsW(lpbuffer: ::core::option::Option<&mut [u16]>) -> u32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetLogicalDriveStringsW ( nbufferlength : u32 , lpbuffer : ::windows::core::PWSTR ) -> u32 );
    GetLogicalDriveStringsW(lpbuffer.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lpbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn GetLogicalDrives() -> u32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetLogicalDrives ( ) -> u32 );
    GetLogicalDrives()
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn GetLongPathNameA<P0>(lpszshortpath: P0, lpszlongpath: ::core::option::Option<&mut [u8]>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetLongPathNameA ( lpszshortpath : ::windows::core::PCSTR , lpszlongpath : ::windows::core::PSTR , cchbuffer : u32 ) -> u32 );
    GetLongPathNameA(lpszshortpath.into_param().abi(), ::core::mem::transmute(lpszlongpath.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpszlongpath.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetLongPathNameTransactedA<P0, P1>(lpszshortpath: P0, lpszlongpath: ::core::option::Option<&mut [u8]>, htransaction: P1) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetLongPathNameTransactedA ( lpszshortpath : ::windows::core::PCSTR , lpszlongpath : ::windows::core::PSTR , cchbuffer : u32 , htransaction : super::super::Foundation:: HANDLE ) -> u32 );
    GetLongPathNameTransactedA(lpszshortpath.into_param().abi(), ::core::mem::transmute(lpszlongpath.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpszlongpath.as_deref().map_or(0, |slice| slice.len() as _), htransaction.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetLongPathNameTransactedW<P0, P1>(lpszshortpath: P0, lpszlongpath: ::core::option::Option<&mut [u16]>, htransaction: P1) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetLongPathNameTransactedW ( lpszshortpath : ::windows::core::PCWSTR , lpszlongpath : ::windows::core::PWSTR , cchbuffer : u32 , htransaction : super::super::Foundation:: HANDLE ) -> u32 );
    GetLongPathNameTransactedW(lpszshortpath.into_param().abi(), ::core::mem::transmute(lpszlongpath.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpszlongpath.as_deref().map_or(0, |slice| slice.len() as _), htransaction.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn GetLongPathNameW<P0>(lpszshortpath: P0, lpszlongpath: ::core::option::Option<&mut [u16]>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetLongPathNameW ( lpszshortpath : ::windows::core::PCWSTR , lpszlongpath : ::windows::core::PWSTR , cchbuffer : u32 ) -> u32 );
    GetLongPathNameW(lpszshortpath.into_param().abi(), ::core::mem::transmute(lpszlongpath.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpszlongpath.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNextLogArchiveExtent(pvarchivecontext: *mut ::core::ffi::c_void, rgadextent: *mut CLS_ARCHIVE_DESCRIPTOR, cdescriptors: u32, pcdescriptorsreturned: *mut u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "clfsw32.dll""system" fn GetNextLogArchiveExtent ( pvarchivecontext : *mut ::core::ffi::c_void , rgadextent : *mut CLS_ARCHIVE_DESCRIPTOR , cdescriptors : u32 , pcdescriptorsreturned : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetNextLogArchiveExtent(pvarchivecontext, rgadextent, cdescriptors, pcdescriptorsreturned)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNotificationResourceManager<P0>(resourcemanagerhandle: P0, transactionnotification: *mut TRANSACTION_NOTIFICATION, notificationlength: u32, dwmilliseconds: u32, returnlength: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ktmw32.dll""system" fn GetNotificationResourceManager ( resourcemanagerhandle : super::super::Foundation:: HANDLE , transactionnotification : *mut TRANSACTION_NOTIFICATION , notificationlength : u32 , dwmilliseconds : u32 , returnlength : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetNotificationResourceManager(resourcemanagerhandle.into_param().abi(), transactionnotification, notificationlength, dwmilliseconds, returnlength)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn GetNotificationResourceManagerAsync<P0>(resourcemanagerhandle: P0, transactionnotification: *mut TRANSACTION_NOTIFICATION, transactionnotificationlength: u32, returnlength: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ktmw32.dll""system" fn GetNotificationResourceManagerAsync ( resourcemanagerhandle : super::super::Foundation:: HANDLE , transactionnotification : *mut TRANSACTION_NOTIFICATION , transactionnotificationlength : u32 , returnlength : *mut u32 , lpoverlapped : *mut super::super::System::IO:: OVERLAPPED ) -> super::super::Foundation:: BOOL );
    GetNotificationResourceManagerAsync(resourcemanagerhandle.into_param().abi(), transactionnotification, transactionnotificationlength, returnlength, lpoverlapped)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn GetShortPathNameA<P0>(lpszlongpath: P0, lpszshortpath: ::core::option::Option<&mut [u8]>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetShortPathNameA ( lpszlongpath : ::windows::core::PCSTR , lpszshortpath : ::windows::core::PSTR , cchbuffer : u32 ) -> u32 );
    GetShortPathNameA(lpszlongpath.into_param().abi(), ::core::mem::transmute(lpszshortpath.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpszshortpath.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn GetShortPathNameW<P0>(lpszlongpath: P0, lpszshortpath: ::core::option::Option<&mut [u16]>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetShortPathNameW ( lpszlongpath : ::windows::core::PCWSTR , lpszshortpath : ::windows::core::PWSTR , cchbuffer : u32 ) -> u32 );
    GetShortPathNameW(lpszlongpath.into_param().abi(), ::core::mem::transmute(lpszshortpath.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpszshortpath.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTapeParameters<P0>(hdevice: P0, dwoperation: GET_TAPE_DRIVE_PARAMETERS_OPERATION, lpdwsize: *mut u32, lptapeinformation: *mut ::core::ffi::c_void) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetTapeParameters ( hdevice : super::super::Foundation:: HANDLE , dwoperation : GET_TAPE_DRIVE_PARAMETERS_OPERATION , lpdwsize : *mut u32 , lptapeinformation : *mut ::core::ffi::c_void ) -> u32 );
    GetTapeParameters(hdevice.into_param().abi(), dwoperation, lpdwsize, lptapeinformation)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTapePosition<P0>(hdevice: P0, dwpositiontype: TAPE_POSITION_TYPE, lpdwpartition: *mut u32, lpdwoffsetlow: *mut u32, lpdwoffsethigh: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetTapePosition ( hdevice : super::super::Foundation:: HANDLE , dwpositiontype : TAPE_POSITION_TYPE , lpdwpartition : *mut u32 , lpdwoffsetlow : *mut u32 , lpdwoffsethigh : *mut u32 ) -> u32 );
    GetTapePosition(hdevice.into_param().abi(), dwpositiontype, lpdwpartition, lpdwoffsetlow, lpdwoffsethigh)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTapeStatus<P0>(hdevice: P0) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetTapeStatus ( hdevice : super::super::Foundation:: HANDLE ) -> u32 );
    GetTapeStatus(hdevice.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn GetTempFileNameA<P0, P1>(lppathname: P0, lpprefixstring: P1, uunique: u32, lptempfilename: &mut [u8; 260]) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetTempFileNameA ( lppathname : ::windows::core::PCSTR , lpprefixstring : ::windows::core::PCSTR , uunique : u32 , lptempfilename : ::windows::core::PSTR ) -> u32 );
    GetTempFileNameA(lppathname.into_param().abi(), lpprefixstring.into_param().abi(), uunique, ::core::mem::transmute(lptempfilename.as_ptr()))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn GetTempFileNameW<P0, P1>(lppathname: P0, lpprefixstring: P1, uunique: u32, lptempfilename: &mut [u16; 260]) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetTempFileNameW ( lppathname : ::windows::core::PCWSTR , lpprefixstring : ::windows::core::PCWSTR , uunique : u32 , lptempfilename : ::windows::core::PWSTR ) -> u32 );
    GetTempFileNameW(lppathname.into_param().abi(), lpprefixstring.into_param().abi(), uunique, ::core::mem::transmute(lptempfilename.as_ptr()))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn GetTempPath2A(buffer: ::core::option::Option<&mut [u8]>) -> u32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetTempPath2A ( bufferlength : u32 , buffer : ::windows::core::PSTR ) -> u32 );
    GetTempPath2A(buffer.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(buffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn GetTempPath2W(buffer: ::core::option::Option<&mut [u16]>) -> u32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetTempPath2W ( bufferlength : u32 , buffer : ::windows::core::PWSTR ) -> u32 );
    GetTempPath2W(buffer.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(buffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn GetTempPathA(lpbuffer: ::core::option::Option<&mut [u8]>) -> u32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetTempPathA ( nbufferlength : u32 , lpbuffer : ::windows::core::PSTR ) -> u32 );
    GetTempPathA(lpbuffer.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lpbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn GetTempPathW(lpbuffer: ::core::option::Option<&mut [u16]>) -> u32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetTempPathW ( nbufferlength : u32 , lpbuffer : ::windows::core::PWSTR ) -> u32 );
    GetTempPathW(lpbuffer.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lpbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTransactionId<P0>(transactionhandle: P0, transactionid: *mut ::windows::core::GUID) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ktmw32.dll""system" fn GetTransactionId ( transactionhandle : super::super::Foundation:: HANDLE , transactionid : *mut ::windows::core::GUID ) -> super::super::Foundation:: BOOL );
    GetTransactionId(transactionhandle.into_param().abi(), transactionid)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTransactionInformation<P0>(transactionhandle: P0, outcome: *mut u32, isolationlevel: *mut u32, isolationflags: *mut u32, timeout: *mut u32, description: ::core::option::Option<&mut [u16]>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ktmw32.dll""system" fn GetTransactionInformation ( transactionhandle : super::super::Foundation:: HANDLE , outcome : *mut u32 , isolationlevel : *mut u32 , isolationflags : *mut u32 , timeout : *mut u32 , bufferlength : u32 , description : ::windows::core::PWSTR ) -> super::super::Foundation:: BOOL );
    GetTransactionInformation(transactionhandle.into_param().abi(), outcome, isolationlevel, isolationflags, timeout, description.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(description.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTransactionManagerId<P0>(transactionmanagerhandle: P0, transactionmanagerid: *mut ::windows::core::GUID) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ktmw32.dll""system" fn GetTransactionManagerId ( transactionmanagerhandle : super::super::Foundation:: HANDLE , transactionmanagerid : *mut ::windows::core::GUID ) -> super::super::Foundation:: BOOL );
    GetTransactionManagerId(transactionmanagerhandle.into_param().abi(), transactionmanagerid)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVolumeInformationA<P0>(lprootpathname: P0, lpvolumenamebuffer: ::core::option::Option<&mut [u8]>, lpvolumeserialnumber: ::core::option::Option<*mut u32>, lpmaximumcomponentlength: ::core::option::Option<*mut u32>, lpfilesystemflags: ::core::option::Option<*mut u32>, lpfilesystemnamebuffer: ::core::option::Option<&mut [u8]>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetVolumeInformationA ( lprootpathname : ::windows::core::PCSTR , lpvolumenamebuffer : ::windows::core::PSTR , nvolumenamesize : u32 , lpvolumeserialnumber : *mut u32 , lpmaximumcomponentlength : *mut u32 , lpfilesystemflags : *mut u32 , lpfilesystemnamebuffer : ::windows::core::PSTR , nfilesystemnamesize : u32 ) -> super::super::Foundation:: BOOL );
    GetVolumeInformationA(
        lprootpathname.into_param().abi(),
        ::core::mem::transmute(lpvolumenamebuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        lpvolumenamebuffer.as_deref().map_or(0, |slice| slice.len() as _),
        ::core::mem::transmute(lpvolumeserialnumber.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpmaximumcomponentlength.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpfilesystemflags.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpfilesystemnamebuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        lpfilesystemnamebuffer.as_deref().map_or(0, |slice| slice.len() as _),
    )
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVolumeInformationByHandleW<P0>(hfile: P0, lpvolumenamebuffer: ::core::option::Option<&mut [u16]>, lpvolumeserialnumber: ::core::option::Option<*mut u32>, lpmaximumcomponentlength: ::core::option::Option<*mut u32>, lpfilesystemflags: ::core::option::Option<*mut u32>, lpfilesystemnamebuffer: ::core::option::Option<&mut [u16]>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetVolumeInformationByHandleW ( hfile : super::super::Foundation:: HANDLE , lpvolumenamebuffer : ::windows::core::PWSTR , nvolumenamesize : u32 , lpvolumeserialnumber : *mut u32 , lpmaximumcomponentlength : *mut u32 , lpfilesystemflags : *mut u32 , lpfilesystemnamebuffer : ::windows::core::PWSTR , nfilesystemnamesize : u32 ) -> super::super::Foundation:: BOOL );
    GetVolumeInformationByHandleW(
        hfile.into_param().abi(),
        ::core::mem::transmute(lpvolumenamebuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        lpvolumenamebuffer.as_deref().map_or(0, |slice| slice.len() as _),
        ::core::mem::transmute(lpvolumeserialnumber.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpmaximumcomponentlength.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpfilesystemflags.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpfilesystemnamebuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        lpfilesystemnamebuffer.as_deref().map_or(0, |slice| slice.len() as _),
    )
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVolumeInformationW<P0>(lprootpathname: P0, lpvolumenamebuffer: ::core::option::Option<&mut [u16]>, lpvolumeserialnumber: ::core::option::Option<*mut u32>, lpmaximumcomponentlength: ::core::option::Option<*mut u32>, lpfilesystemflags: ::core::option::Option<*mut u32>, lpfilesystemnamebuffer: ::core::option::Option<&mut [u16]>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetVolumeInformationW ( lprootpathname : ::windows::core::PCWSTR , lpvolumenamebuffer : ::windows::core::PWSTR , nvolumenamesize : u32 , lpvolumeserialnumber : *mut u32 , lpmaximumcomponentlength : *mut u32 , lpfilesystemflags : *mut u32 , lpfilesystemnamebuffer : ::windows::core::PWSTR , nfilesystemnamesize : u32 ) -> super::super::Foundation:: BOOL );
    GetVolumeInformationW(
        lprootpathname.into_param().abi(),
        ::core::mem::transmute(lpvolumenamebuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        lpvolumenamebuffer.as_deref().map_or(0, |slice| slice.len() as _),
        ::core::mem::transmute(lpvolumeserialnumber.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpmaximumcomponentlength.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpfilesystemflags.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpfilesystemnamebuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        lpfilesystemnamebuffer.as_deref().map_or(0, |slice| slice.len() as _),
    )
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVolumeNameForVolumeMountPointA<P0>(lpszvolumemountpoint: P0, lpszvolumename: &mut [u8]) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetVolumeNameForVolumeMountPointA ( lpszvolumemountpoint : ::windows::core::PCSTR , lpszvolumename : ::windows::core::PSTR , cchbufferlength : u32 ) -> super::super::Foundation:: BOOL );
    GetVolumeNameForVolumeMountPointA(lpszvolumemountpoint.into_param().abi(), ::core::mem::transmute(lpszvolumename.as_ptr()), lpszvolumename.len() as _)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVolumeNameForVolumeMountPointW<P0>(lpszvolumemountpoint: P0, lpszvolumename: &mut [u16]) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetVolumeNameForVolumeMountPointW ( lpszvolumemountpoint : ::windows::core::PCWSTR , lpszvolumename : ::windows::core::PWSTR , cchbufferlength : u32 ) -> super::super::Foundation:: BOOL );
    GetVolumeNameForVolumeMountPointW(lpszvolumemountpoint.into_param().abi(), ::core::mem::transmute(lpszvolumename.as_ptr()), lpszvolumename.len() as _)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVolumePathNameA<P0>(lpszfilename: P0, lpszvolumepathname: &mut [u8]) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetVolumePathNameA ( lpszfilename : ::windows::core::PCSTR , lpszvolumepathname : ::windows::core::PSTR , cchbufferlength : u32 ) -> super::super::Foundation:: BOOL );
    GetVolumePathNameA(lpszfilename.into_param().abi(), ::core::mem::transmute(lpszvolumepathname.as_ptr()), lpszvolumepathname.len() as _)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVolumePathNameW<P0>(lpszfilename: P0, lpszvolumepathname: &mut [u16]) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetVolumePathNameW ( lpszfilename : ::windows::core::PCWSTR , lpszvolumepathname : ::windows::core::PWSTR , cchbufferlength : u32 ) -> super::super::Foundation:: BOOL );
    GetVolumePathNameW(lpszfilename.into_param().abi(), ::core::mem::transmute(lpszvolumepathname.as_ptr()), lpszvolumepathname.len() as _)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVolumePathNamesForVolumeNameA<P0>(lpszvolumename: P0, lpszvolumepathnames: ::core::option::Option<&mut [u8]>, lpcchreturnlength: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetVolumePathNamesForVolumeNameA ( lpszvolumename : ::windows::core::PCSTR , lpszvolumepathnames : ::windows::core::PSTR , cchbufferlength : u32 , lpcchreturnlength : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetVolumePathNamesForVolumeNameA(lpszvolumename.into_param().abi(), ::core::mem::transmute(lpszvolumepathnames.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpszvolumepathnames.as_deref().map_or(0, |slice| slice.len() as _), lpcchreturnlength)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVolumePathNamesForVolumeNameW<P0>(lpszvolumename: P0, lpszvolumepathnames: ::core::option::Option<&mut [u16]>, lpcchreturnlength: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetVolumePathNamesForVolumeNameW ( lpszvolumename : ::windows::core::PCWSTR , lpszvolumepathnames : ::windows::core::PWSTR , cchbufferlength : u32 , lpcchreturnlength : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetVolumePathNamesForVolumeNameW(lpszvolumename.into_param().abi(), ::core::mem::transmute(lpszvolumepathnames.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpszvolumepathnames.as_deref().map_or(0, |slice| slice.len() as _), lpcchreturnlength)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HandleLogFull<P0>(hlog: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "clfsw32.dll""system" fn HandleLogFull ( hlog : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    HandleLogFull(hlog.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InstallLogPolicy<P0>(hlog: P0, ppolicy: *mut CLFS_MGMT_POLICY) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "clfsw32.dll""system" fn InstallLogPolicy ( hlog : super::super::Foundation:: HANDLE , ppolicy : *mut CLFS_MGMT_POLICY ) -> super::super::Foundation:: BOOL );
    InstallLogPolicy(hlog.into_param().abi(), ppolicy)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsIoRingOpSupported(ioring: *const HIORING__, op: IORING_OP_CODE) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "api-ms-win-core-ioring-l1-1-0.dll""system" fn IsIoRingOpSupported ( ioring : *const HIORING__ , op : IORING_OP_CODE ) -> super::super::Foundation:: BOOL );
    IsIoRingOpSupported(ioring, op)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn LZClose(hfile: i32) {
    ::windows_targets::link ! ( "kernel32.dll""system" fn LZClose ( hfile : i32 ) -> ( ) );
    LZClose(hfile)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn LZCopy(hfsource: i32, hfdest: i32) -> i32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn LZCopy ( hfsource : i32 , hfdest : i32 ) -> i32 );
    LZCopy(hfsource, hfdest)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn LZDone() {
    ::windows_targets::link ! ( "kernel32.dll""system" fn LZDone ( ) -> ( ) );
    LZDone()
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn LZInit(hfsource: i32) -> i32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn LZInit ( hfsource : i32 ) -> i32 );
    LZInit(hfsource)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn LZOpenFileA<P0>(lpfilename: P0, lpreopenbuf: *mut OFSTRUCT, wstyle: LZOPENFILE_STYLE) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn LZOpenFileA ( lpfilename : ::windows::core::PCSTR , lpreopenbuf : *mut OFSTRUCT , wstyle : LZOPENFILE_STYLE ) -> i32 );
    LZOpenFileA(lpfilename.into_param().abi(), lpreopenbuf, wstyle)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn LZOpenFileW<P0>(lpfilename: P0, lpreopenbuf: *mut OFSTRUCT, wstyle: LZOPENFILE_STYLE) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn LZOpenFileW ( lpfilename : ::windows::core::PCWSTR , lpreopenbuf : *mut OFSTRUCT , wstyle : LZOPENFILE_STYLE ) -> i32 );
    LZOpenFileW(lpfilename.into_param().abi(), lpreopenbuf, wstyle)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn LZRead(hfile: i32, lpbuffer: &mut [u8]) -> i32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn LZRead ( hfile : i32 , lpbuffer : ::windows::core::PSTR , cbread : i32 ) -> i32 );
    LZRead(hfile, ::core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len() as _)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn LZSeek(hfile: i32, loffset: i32, iorigin: i32) -> i32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn LZSeek ( hfile : i32 , loffset : i32 , iorigin : i32 ) -> i32 );
    LZSeek(hfile, loffset, iorigin)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn LZStart() -> i32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn LZStart ( ) -> i32 );
    LZStart()
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LocalFileTimeToFileTime(lplocalfiletime: *const super::super::Foundation::FILETIME, lpfiletime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn LocalFileTimeToFileTime ( lplocalfiletime : *const super::super::Foundation:: FILETIME , lpfiletime : *mut super::super::Foundation:: FILETIME ) -> super::super::Foundation:: BOOL );
    LocalFileTimeToFileTime(lplocalfiletime, lpfiletime)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LockFile<P0>(hfile: P0, dwfileoffsetlow: u32, dwfileoffsethigh: u32, nnumberofbytestolocklow: u32, nnumberofbytestolockhigh: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn LockFile ( hfile : super::super::Foundation:: HANDLE , dwfileoffsetlow : u32 , dwfileoffsethigh : u32 , nnumberofbytestolocklow : u32 , nnumberofbytestolockhigh : u32 ) -> super::super::Foundation:: BOOL );
    LockFile(hfile.into_param().abi(), dwfileoffsetlow, dwfileoffsethigh, nnumberofbytestolocklow, nnumberofbytestolockhigh)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn LockFileEx<P0>(hfile: P0, dwflags: LOCK_FILE_FLAGS, dwreserved: u32, nnumberofbytestolocklow: u32, nnumberofbytestolockhigh: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn LockFileEx ( hfile : super::super::Foundation:: HANDLE , dwflags : LOCK_FILE_FLAGS , dwreserved : u32 , nnumberofbytestolocklow : u32 , nnumberofbytestolockhigh : u32 , lpoverlapped : *mut super::super::System::IO:: OVERLAPPED ) -> super::super::Foundation:: BOOL );
    LockFileEx(hfile.into_param().abi(), dwflags, dwreserved, nnumberofbytestolocklow, nnumberofbytestolockhigh, lpoverlapped)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LogTailAdvanceFailure<P0>(hlog: P0, dwreason: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "clfsw32.dll""system" fn LogTailAdvanceFailure ( hlog : super::super::Foundation:: HANDLE , dwreason : u32 ) -> super::super::Foundation:: BOOL );
    LogTailAdvanceFailure(hlog.into_param().abi(), dwreason)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn LsnBlockOffset(plsn: *const CLS_LSN) -> u32 {
    ::windows_targets::link ! ( "clfsw32.dll""system" fn LsnBlockOffset ( plsn : *const CLS_LSN ) -> u32 );
    LsnBlockOffset(plsn)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn LsnContainer(plsn: *const CLS_LSN) -> u32 {
    ::windows_targets::link ! ( "clfsw32.dll""system" fn LsnContainer ( plsn : *const CLS_LSN ) -> u32 );
    LsnContainer(plsn)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn LsnCreate(cidcontainer: u32, offblock: u32, crecord: u32) -> CLS_LSN {
    ::windows_targets::link ! ( "clfsw32.dll""system" fn LsnCreate ( cidcontainer : u32 , offblock : u32 , crecord : u32 ) -> CLS_LSN );
    LsnCreate(cidcontainer, offblock, crecord)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsnEqual(plsn1: *const CLS_LSN, plsn2: *const CLS_LSN) -> super::super::Foundation::BOOLEAN {
    ::windows_targets::link ! ( "clfsw32.dll""system" fn LsnEqual ( plsn1 : *const CLS_LSN , plsn2 : *const CLS_LSN ) -> super::super::Foundation:: BOOLEAN );
    LsnEqual(plsn1, plsn2)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsnGreater(plsn1: *const CLS_LSN, plsn2: *const CLS_LSN) -> super::super::Foundation::BOOLEAN {
    ::windows_targets::link ! ( "clfsw32.dll""system" fn LsnGreater ( plsn1 : *const CLS_LSN , plsn2 : *const CLS_LSN ) -> super::super::Foundation:: BOOLEAN );
    LsnGreater(plsn1, plsn2)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn LsnIncrement(plsn: *const CLS_LSN) -> CLS_LSN {
    ::windows_targets::link ! ( "clfsw32.dll""system" fn LsnIncrement ( plsn : *const CLS_LSN ) -> CLS_LSN );
    LsnIncrement(plsn)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsnInvalid(plsn: *const CLS_LSN) -> super::super::Foundation::BOOLEAN {
    ::windows_targets::link ! ( "clfsw32.dll""system" fn LsnInvalid ( plsn : *const CLS_LSN ) -> super::super::Foundation:: BOOLEAN );
    LsnInvalid(plsn)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsnLess(plsn1: *const CLS_LSN, plsn2: *const CLS_LSN) -> super::super::Foundation::BOOLEAN {
    ::windows_targets::link ! ( "clfsw32.dll""system" fn LsnLess ( plsn1 : *const CLS_LSN , plsn2 : *const CLS_LSN ) -> super::super::Foundation:: BOOLEAN );
    LsnLess(plsn1, plsn2)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LsnNull(plsn: *const CLS_LSN) -> super::super::Foundation::BOOLEAN {
    ::windows_targets::link ! ( "clfsw32.dll""system" fn LsnNull ( plsn : *const CLS_LSN ) -> super::super::Foundation:: BOOLEAN );
    LsnNull(plsn)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn LsnRecordSequence(plsn: *const CLS_LSN) -> u32 {
    ::windows_targets::link ! ( "clfsw32.dll""system" fn LsnRecordSequence ( plsn : *const CLS_LSN ) -> u32 );
    LsnRecordSequence(plsn)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoveFileA<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn MoveFileA ( lpexistingfilename : ::windows::core::PCSTR , lpnewfilename : ::windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    MoveFileA(lpexistingfilename.into_param().abi(), lpnewfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoveFileExA<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1, dwflags: MOVE_FILE_FLAGS) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn MoveFileExA ( lpexistingfilename : ::windows::core::PCSTR , lpnewfilename : ::windows::core::PCSTR , dwflags : MOVE_FILE_FLAGS ) -> super::super::Foundation:: BOOL );
    MoveFileExA(lpexistingfilename.into_param().abi(), lpnewfilename.into_param().abi(), dwflags)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoveFileExW<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1, dwflags: MOVE_FILE_FLAGS) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn MoveFileExW ( lpexistingfilename : ::windows::core::PCWSTR , lpnewfilename : ::windows::core::PCWSTR , dwflags : MOVE_FILE_FLAGS ) -> super::super::Foundation:: BOOL );
    MoveFileExW(lpexistingfilename.into_param().abi(), lpnewfilename.into_param().abi(), dwflags)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoveFileFromAppW<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "api-ms-win-core-file-fromapp-l1-1-0.dll""system" fn MoveFileFromAppW ( lpexistingfilename : ::windows::core::PCWSTR , lpnewfilename : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    MoveFileFromAppW(lpexistingfilename.into_param().abi(), lpnewfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoveFileTransactedA<P0, P1, P2>(lpexistingfilename: P0, lpnewfilename: P1, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: ::core::option::Option<*const ::core::ffi::c_void>, dwflags: MOVE_FILE_FLAGS, htransaction: P2) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn MoveFileTransactedA ( lpexistingfilename : ::windows::core::PCSTR , lpnewfilename : ::windows::core::PCSTR , lpprogressroutine : LPPROGRESS_ROUTINE , lpdata : *const ::core::ffi::c_void , dwflags : MOVE_FILE_FLAGS , htransaction : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    MoveFileTransactedA(lpexistingfilename.into_param().abi(), lpnewfilename.into_param().abi(), lpprogressroutine, ::core::mem::transmute(lpdata.unwrap_or(::std::ptr::null())), dwflags, htransaction.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoveFileTransactedW<P0, P1, P2>(lpexistingfilename: P0, lpnewfilename: P1, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: ::core::option::Option<*const ::core::ffi::c_void>, dwflags: MOVE_FILE_FLAGS, htransaction: P2) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn MoveFileTransactedW ( lpexistingfilename : ::windows::core::PCWSTR , lpnewfilename : ::windows::core::PCWSTR , lpprogressroutine : LPPROGRESS_ROUTINE , lpdata : *const ::core::ffi::c_void , dwflags : MOVE_FILE_FLAGS , htransaction : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    MoveFileTransactedW(lpexistingfilename.into_param().abi(), lpnewfilename.into_param().abi(), lpprogressroutine, ::core::mem::transmute(lpdata.unwrap_or(::std::ptr::null())), dwflags, htransaction.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoveFileW<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn MoveFileW ( lpexistingfilename : ::windows::core::PCWSTR , lpnewfilename : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    MoveFileW(lpexistingfilename.into_param().abi(), lpnewfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoveFileWithProgressA<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: ::core::option::Option<*const ::core::ffi::c_void>, dwflags: MOVE_FILE_FLAGS) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn MoveFileWithProgressA ( lpexistingfilename : ::windows::core::PCSTR , lpnewfilename : ::windows::core::PCSTR , lpprogressroutine : LPPROGRESS_ROUTINE , lpdata : *const ::core::ffi::c_void , dwflags : MOVE_FILE_FLAGS ) -> super::super::Foundation:: BOOL );
    MoveFileWithProgressA(lpexistingfilename.into_param().abi(), lpnewfilename.into_param().abi(), lpprogressroutine, ::core::mem::transmute(lpdata.unwrap_or(::std::ptr::null())), dwflags)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoveFileWithProgressW<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: ::core::option::Option<*const ::core::ffi::c_void>, dwflags: MOVE_FILE_FLAGS) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn MoveFileWithProgressW ( lpexistingfilename : ::windows::core::PCWSTR , lpnewfilename : ::windows::core::PCWSTR , lpprogressroutine : LPPROGRESS_ROUTINE , lpdata : *const ::core::ffi::c_void , dwflags : MOVE_FILE_FLAGS ) -> super::super::Foundation:: BOOL );
    MoveFileWithProgressW(lpexistingfilename.into_param().abi(), lpnewfilename.into_param().abi(), lpprogressroutine, ::core::mem::transmute(lpdata.unwrap_or(::std::ptr::null())), dwflags)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn NetConnectionEnum<P0, P1>(servername: P0, qualifier: P1, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "netapi32.dll""system" fn NetConnectionEnum ( servername : ::windows::core::PCWSTR , qualifier : ::windows::core::PCWSTR , level : u32 , bufptr : *mut *mut u8 , prefmaxlen : u32 , entriesread : *mut u32 , totalentries : *mut u32 , resume_handle : *mut u32 ) -> u32 );
    NetConnectionEnum(servername.into_param().abi(), qualifier.into_param().abi(), level, bufptr, prefmaxlen, entriesread, totalentries, ::core::mem::transmute(resume_handle.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn NetFileClose<P0>(servername: P0, fileid: u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "netapi32.dll""system" fn NetFileClose ( servername : ::windows::core::PCWSTR , fileid : u32 ) -> u32 );
    NetFileClose(servername.into_param().abi(), fileid)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn NetFileEnum<P0, P1, P2>(servername: P0, basepath: P1, username: P2, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: ::core::option::Option<*mut usize>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "netapi32.dll""system" fn NetFileEnum ( servername : ::windows::core::PCWSTR , basepath : ::windows::core::PCWSTR , username : ::windows::core::PCWSTR , level : u32 , bufptr : *mut *mut u8 , prefmaxlen : u32 , entriesread : *mut u32 , totalentries : *mut u32 , resume_handle : *mut usize ) -> u32 );
    NetFileEnum(servername.into_param().abi(), basepath.into_param().abi(), username.into_param().abi(), level, bufptr, prefmaxlen, entriesread, totalentries, ::core::mem::transmute(resume_handle.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn NetFileGetInfo<P0>(servername: P0, fileid: u32, level: u32, bufptr: *mut *mut u8) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "netapi32.dll""system" fn NetFileGetInfo ( servername : ::windows::core::PCWSTR , fileid : u32 , level : u32 , bufptr : *mut *mut u8 ) -> u32 );
    NetFileGetInfo(servername.into_param().abi(), fileid, level, bufptr)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn NetServerAliasAdd<P0>(servername: P0, level: u32, buf: *const u8) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "netapi32.dll""system" fn NetServerAliasAdd ( servername : ::windows::core::PCWSTR , level : u32 , buf : *const u8 ) -> u32 );
    NetServerAliasAdd(servername.into_param().abi(), level, buf)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn NetServerAliasDel<P0>(servername: P0, level: u32, buf: *const u8) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "netapi32.dll""system" fn NetServerAliasDel ( servername : ::windows::core::PCWSTR , level : u32 , buf : *const u8 ) -> u32 );
    NetServerAliasDel(servername.into_param().abi(), level, buf)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn NetServerAliasEnum<P0>(servername: P0, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "netapi32.dll""system" fn NetServerAliasEnum ( servername : ::windows::core::PCWSTR , level : u32 , bufptr : *mut *mut u8 , prefmaxlen : u32 , entriesread : *mut u32 , totalentries : *mut u32 , resumehandle : *mut u32 ) -> u32 );
    NetServerAliasEnum(servername.into_param().abi(), level, bufptr, prefmaxlen, entriesread, totalentries, ::core::mem::transmute(resumehandle.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn NetSessionDel<P0, P1, P2>(servername: P0, uncclientname: P1, username: P2) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "netapi32.dll""system" fn NetSessionDel ( servername : ::windows::core::PCWSTR , uncclientname : ::windows::core::PCWSTR , username : ::windows::core::PCWSTR ) -> u32 );
    NetSessionDel(servername.into_param().abi(), uncclientname.into_param().abi(), username.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn NetSessionEnum<P0, P1, P2>(servername: P0, uncclientname: P1, username: P2, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "netapi32.dll""system" fn NetSessionEnum ( servername : ::windows::core::PCWSTR , uncclientname : ::windows::core::PCWSTR , username : ::windows::core::PCWSTR , level : u32 , bufptr : *mut *mut u8 , prefmaxlen : u32 , entriesread : *mut u32 , totalentries : *mut u32 , resume_handle : *mut u32 ) -> u32 );
    NetSessionEnum(servername.into_param().abi(), uncclientname.into_param().abi(), username.into_param().abi(), level, bufptr, prefmaxlen, entriesread, totalentries, ::core::mem::transmute(resume_handle.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn NetSessionGetInfo<P0, P1, P2>(servername: P0, uncclientname: P1, username: P2, level: u32, bufptr: *mut *mut u8) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "netapi32.dll""system" fn NetSessionGetInfo ( servername : ::windows::core::PCWSTR , uncclientname : ::windows::core::PCWSTR , username : ::windows::core::PCWSTR , level : u32 , bufptr : *mut *mut u8 ) -> u32 );
    NetSessionGetInfo(servername.into_param().abi(), uncclientname.into_param().abi(), username.into_param().abi(), level, bufptr)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn NetShareAdd<P0>(servername: P0, level: u32, buf: *const u8, parm_err: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "netapi32.dll""system" fn NetShareAdd ( servername : ::windows::core::PCWSTR , level : u32 , buf : *const u8 , parm_err : *mut u32 ) -> u32 );
    NetShareAdd(servername.into_param().abi(), level, buf, ::core::mem::transmute(parm_err.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn NetShareCheck<P0, P1>(servername: P0, device: P1, r#type: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "netapi32.dll""system" fn NetShareCheck ( servername : ::windows::core::PCWSTR , device : ::windows::core::PCWSTR , r#type : *mut u32 ) -> u32 );
    NetShareCheck(servername.into_param().abi(), device.into_param().abi(), r#type)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn NetShareDel<P0, P1>(servername: P0, netname: P1, reserved: u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "netapi32.dll""system" fn NetShareDel ( servername : ::windows::core::PCWSTR , netname : ::windows::core::PCWSTR , reserved : u32 ) -> u32 );
    NetShareDel(servername.into_param().abi(), netname.into_param().abi(), reserved)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn NetShareDelEx<P0>(servername: P0, level: u32, buf: *const u8) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "netapi32.dll""system" fn NetShareDelEx ( servername : ::windows::core::PCWSTR , level : u32 , buf : *const u8 ) -> u32 );
    NetShareDelEx(servername.into_param().abi(), level, buf)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn NetShareDelSticky<P0, P1>(servername: P0, netname: P1, reserved: u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "netapi32.dll""system" fn NetShareDelSticky ( servername : ::windows::core::PCWSTR , netname : ::windows::core::PCWSTR , reserved : u32 ) -> u32 );
    NetShareDelSticky(servername.into_param().abi(), netname.into_param().abi(), reserved)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn NetShareEnum<P0>(servername: P0, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "netapi32.dll""system" fn NetShareEnum ( servername : ::windows::core::PCWSTR , level : u32 , bufptr : *mut *mut u8 , prefmaxlen : u32 , entriesread : *mut u32 , totalentries : *mut u32 , resume_handle : *mut u32 ) -> u32 );
    NetShareEnum(servername.into_param().abi(), level, bufptr, prefmaxlen, entriesread, totalentries, ::core::mem::transmute(resume_handle.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn NetShareEnumSticky<P0>(servername: P0, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "netapi32.dll""system" fn NetShareEnumSticky ( servername : ::windows::core::PCWSTR , level : u32 , bufptr : *mut *mut u8 , prefmaxlen : u32 , entriesread : *mut u32 , totalentries : *mut u32 , resume_handle : *mut u32 ) -> u32 );
    NetShareEnumSticky(servername.into_param().abi(), level, bufptr, prefmaxlen, entriesread, totalentries, ::core::mem::transmute(resume_handle.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn NetShareGetInfo<P0, P1>(servername: P0, netname: P1, level: u32, bufptr: *mut *mut u8) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "netapi32.dll""system" fn NetShareGetInfo ( servername : ::windows::core::PCWSTR , netname : ::windows::core::PCWSTR , level : u32 , bufptr : *mut *mut u8 ) -> u32 );
    NetShareGetInfo(servername.into_param().abi(), netname.into_param().abi(), level, bufptr)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn NetShareSetInfo<P0, P1>(servername: P0, netname: P1, level: u32, buf: *const u8, parm_err: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "netapi32.dll""system" fn NetShareSetInfo ( servername : ::windows::core::PCWSTR , netname : ::windows::core::PCWSTR , level : u32 , buf : *const u8 , parm_err : *mut u32 ) -> u32 );
    NetShareSetInfo(servername.into_param().abi(), netname.into_param().abi(), level, buf, ::core::mem::transmute(parm_err.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn NetStatisticsGet(servername: *const i8, service: *const i8, level: u32, options: u32, buffer: *mut *mut u8) -> u32 {
    ::windows_targets::link ! ( "netapi32.dll""system" fn NetStatisticsGet ( servername : *const i8 , service : *const i8 , level : u32 , options : u32 , buffer : *mut *mut u8 ) -> u32 );
    NetStatisticsGet(servername, service, level, options, buffer)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
#[inline]
pub unsafe fn NtCreateFile(filehandle: *mut super::super::Foundation::HANDLE, desiredaccess: u32, objectattributes: *mut super::super::System::WindowsProgramming::OBJECT_ATTRIBUTES, iostatusblock: *mut super::super::System::WindowsProgramming::IO_STATUS_BLOCK, allocationsize: *mut i64, fileattributes: u32, shareaccess: FILE_SHARE_MODE, createdisposition: NT_CREATE_FILE_DISPOSITION, createoptions: u32, eabuffer: *mut ::core::ffi::c_void, ealength: u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "ntdll.dll""system" fn NtCreateFile ( filehandle : *mut super::super::Foundation:: HANDLE , desiredaccess : u32 , objectattributes : *mut super::super::System::WindowsProgramming:: OBJECT_ATTRIBUTES , iostatusblock : *mut super::super::System::WindowsProgramming:: IO_STATUS_BLOCK , allocationsize : *mut i64 , fileattributes : u32 , shareaccess : FILE_SHARE_MODE , createdisposition : NT_CREATE_FILE_DISPOSITION , createoptions : u32 , eabuffer : *mut ::core::ffi::c_void , ealength : u32 ) -> super::super::Foundation:: NTSTATUS );
    NtCreateFile(filehandle, desiredaccess, objectattributes, iostatusblock, allocationsize, fileattributes, shareaccess, createdisposition, createoptions, eabuffer, ealength).ok()
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn OpenEncryptedFileRawA<P0>(lpfilename: P0, ulflags: u32, pvcontext: *mut *mut ::core::ffi::c_void) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn OpenEncryptedFileRawA ( lpfilename : ::windows::core::PCSTR , ulflags : u32 , pvcontext : *mut *mut ::core::ffi::c_void ) -> u32 );
    OpenEncryptedFileRawA(lpfilename.into_param().abi(), ulflags, pvcontext)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn OpenEncryptedFileRawW<P0>(lpfilename: P0, ulflags: u32, pvcontext: *mut *mut ::core::ffi::c_void) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn OpenEncryptedFileRawW ( lpfilename : ::windows::core::PCWSTR , ulflags : u32 , pvcontext : *mut *mut ::core::ffi::c_void ) -> u32 );
    OpenEncryptedFileRawW(lpfilename.into_param().abi(), ulflags, pvcontext)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenEnlistment<P0>(dwdesiredaccess: u32, resourcemanagerhandle: P0, enlistmentid: *mut ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ktmw32.dll""system" fn OpenEnlistment ( dwdesiredaccess : u32 , resourcemanagerhandle : super::super::Foundation:: HANDLE , enlistmentid : *mut ::windows::core::GUID ) -> super::super::Foundation:: HANDLE );
    let result__ = OpenEnlistment(dwdesiredaccess, resourcemanagerhandle.into_param().abi(), enlistmentid);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn OpenFile<P0>(lpfilename: P0, lpreopenbuff: *mut OFSTRUCT, ustyle: u32) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn OpenFile ( lpfilename : ::windows::core::PCSTR , lpreopenbuff : *mut OFSTRUCT , ustyle : u32 ) -> i32 );
    OpenFile(lpfilename.into_param().abi(), lpreopenbuff, ustyle)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn OpenFileById<P0>(hvolumehint: P0, lpfileid: *const FILE_ID_DESCRIPTOR, dwdesiredaccess: u32, dwsharemode: FILE_SHARE_MODE, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn OpenFileById ( hvolumehint : super::super::Foundation:: HANDLE , lpfileid : *const FILE_ID_DESCRIPTOR , dwdesiredaccess : u32 , dwsharemode : FILE_SHARE_MODE , lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , dwflagsandattributes : FILE_FLAGS_AND_ATTRIBUTES ) -> super::super::Foundation:: HANDLE );
    let result__ = OpenFileById(hvolumehint.into_param().abi(), lpfileid, dwdesiredaccess, dwsharemode, ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null())), dwflagsandattributes);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenResourceManager<P0>(dwdesiredaccess: u32, tmhandle: P0, resourcemanagerid: *mut ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ktmw32.dll""system" fn OpenResourceManager ( dwdesiredaccess : u32 , tmhandle : super::super::Foundation:: HANDLE , resourcemanagerid : *mut ::windows::core::GUID ) -> super::super::Foundation:: HANDLE );
    let result__ = OpenResourceManager(dwdesiredaccess, tmhandle.into_param().abi(), resourcemanagerid);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenTransaction(dwdesiredaccess: u32, transactionid: *mut ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    ::windows_targets::link ! ( "ktmw32.dll""system" fn OpenTransaction ( dwdesiredaccess : u32 , transactionid : *mut ::windows::core::GUID ) -> super::super::Foundation:: HANDLE );
    let result__ = OpenTransaction(dwdesiredaccess, transactionid);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenTransactionManager<P0>(logfilename: P0, desiredaccess: u32, openoptions: u32) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "ktmw32.dll""system" fn OpenTransactionManager ( logfilename : ::windows::core::PCWSTR , desiredaccess : u32 , openoptions : u32 ) -> super::super::Foundation:: HANDLE );
    let result__ = OpenTransactionManager(logfilename.into_param().abi(), desiredaccess, openoptions);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenTransactionManagerById(transactionmanagerid: *const ::windows::core::GUID, desiredaccess: u32, openoptions: u32) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    ::windows_targets::link ! ( "ktmw32.dll""system" fn OpenTransactionManagerById ( transactionmanagerid : *const ::windows::core::GUID , desiredaccess : u32 , openoptions : u32 ) -> super::super::Foundation:: HANDLE );
    let result__ = OpenTransactionManagerById(transactionmanagerid, desiredaccess, openoptions);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn PopIoRingCompletion(ioring: *const HIORING__) -> ::windows::core::Result<IORING_CQE> {
    ::windows_targets::link ! ( "api-ms-win-core-ioring-l1-1-0.dll""system" fn PopIoRingCompletion ( ioring : *const HIORING__ , cqe : *mut IORING_CQE ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IORING_CQE>();
    PopIoRingCompletion(ioring, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrePrepareComplete<P0>(enlistmenthandle: P0, tmvirtualclock: *mut i64) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ktmw32.dll""system" fn PrePrepareComplete ( enlistmenthandle : super::super::Foundation:: HANDLE , tmvirtualclock : *mut i64 ) -> super::super::Foundation:: BOOL );
    PrePrepareComplete(enlistmenthandle.into_param().abi(), tmvirtualclock)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrePrepareEnlistment<P0>(enlistmenthandle: P0, tmvirtualclock: *mut i64) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ktmw32.dll""system" fn PrePrepareEnlistment ( enlistmenthandle : super::super::Foundation:: HANDLE , tmvirtualclock : *mut i64 ) -> super::super::Foundation:: BOOL );
    PrePrepareEnlistment(enlistmenthandle.into_param().abi(), tmvirtualclock)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrepareComplete<P0>(enlistmenthandle: P0, tmvirtualclock: *mut i64) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ktmw32.dll""system" fn PrepareComplete ( enlistmenthandle : super::super::Foundation:: HANDLE , tmvirtualclock : *mut i64 ) -> super::super::Foundation:: BOOL );
    PrepareComplete(enlistmenthandle.into_param().abi(), tmvirtualclock)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrepareEnlistment<P0>(enlistmenthandle: P0, tmvirtualclock: *mut i64) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ktmw32.dll""system" fn PrepareEnlistment ( enlistmenthandle : super::super::Foundation:: HANDLE , tmvirtualclock : *mut i64 ) -> super::super::Foundation:: BOOL );
    PrepareEnlistment(enlistmenthandle.into_param().abi(), tmvirtualclock)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrepareLogArchive<P0>(hlog: P0, pszbaselogfilename: &mut [u16], plsnlow: ::core::option::Option<*const CLS_LSN>, plsnhigh: ::core::option::Option<*const CLS_LSN>, pcactuallength: ::core::option::Option<*mut u32>, poffbaselogfiledata: *mut u64, pcbbaselogfilelength: *mut u64, plsnbase: *mut CLS_LSN, plsnlast: *mut CLS_LSN, plsncurrentarchivetail: *mut CLS_LSN, ppvarchivecontext: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "clfsw32.dll""system" fn PrepareLogArchive ( hlog : super::super::Foundation:: HANDLE , pszbaselogfilename : ::windows::core::PWSTR , clen : u32 , plsnlow : *const CLS_LSN , plsnhigh : *const CLS_LSN , pcactuallength : *mut u32 , poffbaselogfiledata : *mut u64 , pcbbaselogfilelength : *mut u64 , plsnbase : *mut CLS_LSN , plsnlast : *mut CLS_LSN , plsncurrentarchivetail : *mut CLS_LSN , ppvarchivecontext : *mut *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    PrepareLogArchive(hlog.into_param().abi(), ::core::mem::transmute(pszbaselogfilename.as_ptr()), pszbaselogfilename.len() as _, ::core::mem::transmute(plsnlow.unwrap_or(::std::ptr::null())), ::core::mem::transmute(plsnhigh.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pcactuallength.unwrap_or(::std::ptr::null_mut())), poffbaselogfiledata, pcbbaselogfilelength, plsnbase, plsnlast, plsncurrentarchivetail, ppvarchivecontext)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrepareTape<P0, P1>(hdevice: P0, dwoperation: PREPARE_TAPE_OPERATION, bimmediate: P1) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn PrepareTape ( hdevice : super::super::Foundation:: HANDLE , dwoperation : PREPARE_TAPE_OPERATION , bimmediate : super::super::Foundation:: BOOL ) -> u32 );
    PrepareTape(hdevice.into_param().abi(), dwoperation, bimmediate.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn QueryDosDeviceA<P0>(lpdevicename: P0, lptargetpath: ::core::option::Option<&mut [u8]>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn QueryDosDeviceA ( lpdevicename : ::windows::core::PCSTR , lptargetpath : ::windows::core::PSTR , ucchmax : u32 ) -> u32 );
    QueryDosDeviceA(lpdevicename.into_param().abi(), ::core::mem::transmute(lptargetpath.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lptargetpath.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn QueryDosDeviceW<P0>(lpdevicename: P0, lptargetpath: ::core::option::Option<&mut [u16]>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn QueryDosDeviceW ( lpdevicename : ::windows::core::PCWSTR , lptargetpath : ::windows::core::PWSTR , ucchmax : u32 ) -> u32 );
    QueryDosDeviceW(lpdevicename.into_param().abi(), ::core::mem::transmute(lptargetpath.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lptargetpath.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn QueryIoRingCapabilities() -> ::windows::core::Result<IORING_CAPABILITIES> {
    ::windows_targets::link ! ( "api-ms-win-core-ioring-l1-1-0.dll""system" fn QueryIoRingCapabilities ( capabilities : *mut IORING_CAPABILITIES ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IORING_CAPABILITIES>();
    QueryIoRingCapabilities(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryLogPolicy<P0>(hlog: P0, epolicytype: CLFS_MGMT_POLICY_TYPE, ppolicybuffer: *mut CLFS_MGMT_POLICY, pcbpolicybuffer: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "clfsw32.dll""system" fn QueryLogPolicy ( hlog : super::super::Foundation:: HANDLE , epolicytype : CLFS_MGMT_POLICY_TYPE , ppolicybuffer : *mut CLFS_MGMT_POLICY , pcbpolicybuffer : *mut u32 ) -> super::super::Foundation:: BOOL );
    QueryLogPolicy(hlog.into_param().abi(), epolicytype, ppolicybuffer, pcbpolicybuffer)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn QueryRecoveryAgentsOnEncryptedFile<P0>(lpfilename: P0, precoveryagents: *mut *mut ENCRYPTION_CERTIFICATE_HASH_LIST) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn QueryRecoveryAgentsOnEncryptedFile ( lpfilename : ::windows::core::PCWSTR , precoveryagents : *mut *mut ENCRYPTION_CERTIFICATE_HASH_LIST ) -> u32 );
    QueryRecoveryAgentsOnEncryptedFile(lpfilename.into_param().abi(), precoveryagents)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn QueryUsersOnEncryptedFile<P0>(lpfilename: P0, pusers: *mut *mut ENCRYPTION_CERTIFICATE_HASH_LIST) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn QueryUsersOnEncryptedFile ( lpfilename : ::windows::core::PCWSTR , pusers : *mut *mut ENCRYPTION_CERTIFICATE_HASH_LIST ) -> u32 );
    QueryUsersOnEncryptedFile(lpfilename.into_param().abi(), pusers)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReOpenFile<P0>(horiginalfile: P0, dwdesiredaccess: u32, dwsharemode: FILE_SHARE_MODE, dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn ReOpenFile ( horiginalfile : super::super::Foundation:: HANDLE , dwdesiredaccess : u32 , dwsharemode : FILE_SHARE_MODE , dwflagsandattributes : FILE_FLAGS_AND_ATTRIBUTES ) -> super::super::Foundation:: HANDLE );
    let result__ = ReOpenFile(horiginalfile.into_param().abi(), dwdesiredaccess, dwsharemode, dwflagsandattributes);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn ReadDirectoryChangesExW<P0, P1>(hdirectory: P0, lpbuffer: *mut ::core::ffi::c_void, nbufferlength: u32, bwatchsubtree: P1, dwnotifyfilter: FILE_NOTIFY_CHANGE, lpbytesreturned: ::core::option::Option<*mut u32>, lpoverlapped: ::core::option::Option<*mut super::super::System::IO::OVERLAPPED>, lpcompletionroutine: super::super::System::IO::LPOVERLAPPED_COMPLETION_ROUTINE, readdirectorynotifyinformationclass: READ_DIRECTORY_NOTIFY_INFORMATION_CLASS) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn ReadDirectoryChangesExW ( hdirectory : super::super::Foundation:: HANDLE , lpbuffer : *mut ::core::ffi::c_void , nbufferlength : u32 , bwatchsubtree : super::super::Foundation:: BOOL , dwnotifyfilter : FILE_NOTIFY_CHANGE , lpbytesreturned : *mut u32 , lpoverlapped : *mut super::super::System::IO:: OVERLAPPED , lpcompletionroutine : super::super::System::IO:: LPOVERLAPPED_COMPLETION_ROUTINE , readdirectorynotifyinformationclass : READ_DIRECTORY_NOTIFY_INFORMATION_CLASS ) -> super::super::Foundation:: BOOL );
    ReadDirectoryChangesExW(hdirectory.into_param().abi(), lpbuffer, nbufferlength, bwatchsubtree.into_param().abi(), dwnotifyfilter, ::core::mem::transmute(lpbytesreturned.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpoverlapped.unwrap_or(::std::ptr::null_mut())), lpcompletionroutine, readdirectorynotifyinformationclass)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn ReadDirectoryChangesW<P0, P1>(hdirectory: P0, lpbuffer: *mut ::core::ffi::c_void, nbufferlength: u32, bwatchsubtree: P1, dwnotifyfilter: FILE_NOTIFY_CHANGE, lpbytesreturned: ::core::option::Option<*mut u32>, lpoverlapped: ::core::option::Option<*mut super::super::System::IO::OVERLAPPED>, lpcompletionroutine: super::super::System::IO::LPOVERLAPPED_COMPLETION_ROUTINE) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn ReadDirectoryChangesW ( hdirectory : super::super::Foundation:: HANDLE , lpbuffer : *mut ::core::ffi::c_void , nbufferlength : u32 , bwatchsubtree : super::super::Foundation:: BOOL , dwnotifyfilter : FILE_NOTIFY_CHANGE , lpbytesreturned : *mut u32 , lpoverlapped : *mut super::super::System::IO:: OVERLAPPED , lpcompletionroutine : super::super::System::IO:: LPOVERLAPPED_COMPLETION_ROUTINE ) -> super::super::Foundation:: BOOL );
    ReadDirectoryChangesW(hdirectory.into_param().abi(), lpbuffer, nbufferlength, bwatchsubtree.into_param().abi(), dwnotifyfilter, ::core::mem::transmute(lpbytesreturned.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpoverlapped.unwrap_or(::std::ptr::null_mut())), lpcompletionroutine)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn ReadEncryptedFileRaw(pfexportcallback: PFE_EXPORT_FUNC, pvcallbackcontext: ::core::option::Option<*const ::core::ffi::c_void>, pvcontext: *const ::core::ffi::c_void) -> u32 {
    ::windows_targets::link ! ( "advapi32.dll""system" fn ReadEncryptedFileRaw ( pfexportcallback : PFE_EXPORT_FUNC , pvcallbackcontext : *const ::core::ffi::c_void , pvcontext : *const ::core::ffi::c_void ) -> u32 );
    ReadEncryptedFileRaw(pfexportcallback, ::core::mem::transmute(pvcallbackcontext.unwrap_or(::std::ptr::null())), pvcontext)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn ReadFile<P0>(hfile: P0, lpbuffer: ::core::option::Option<*mut ::core::ffi::c_void>, nnumberofbytestoread: u32, lpnumberofbytesread: ::core::option::Option<*mut u32>, lpoverlapped: ::core::option::Option<*mut super::super::System::IO::OVERLAPPED>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn ReadFile ( hfile : super::super::Foundation:: HANDLE , lpbuffer : *mut ::core::ffi::c_void , nnumberofbytestoread : u32 , lpnumberofbytesread : *mut u32 , lpoverlapped : *mut super::super::System::IO:: OVERLAPPED ) -> super::super::Foundation:: BOOL );
    ReadFile(hfile.into_param().abi(), ::core::mem::transmute(lpbuffer.unwrap_or(::std::ptr::null_mut())), nnumberofbytestoread, ::core::mem::transmute(lpnumberofbytesread.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpoverlapped.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn ReadFileEx<P0>(hfile: P0, lpbuffer: ::core::option::Option<*mut ::core::ffi::c_void>, nnumberofbytestoread: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: super::super::System::IO::LPOVERLAPPED_COMPLETION_ROUTINE) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn ReadFileEx ( hfile : super::super::Foundation:: HANDLE , lpbuffer : *mut ::core::ffi::c_void , nnumberofbytestoread : u32 , lpoverlapped : *mut super::super::System::IO:: OVERLAPPED , lpcompletionroutine : super::super::System::IO:: LPOVERLAPPED_COMPLETION_ROUTINE ) -> super::super::Foundation:: BOOL );
    ReadFileEx(hfile.into_param().abi(), ::core::mem::transmute(lpbuffer.unwrap_or(::std::ptr::null_mut())), nnumberofbytestoread, lpoverlapped, lpcompletionroutine)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn ReadFileScatter<P0>(hfile: P0, asegmentarray: *const FILE_SEGMENT_ELEMENT, nnumberofbytestoread: u32, lpreserved: ::core::option::Option<*const u32>, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn ReadFileScatter ( hfile : super::super::Foundation:: HANDLE , asegmentarray : *const FILE_SEGMENT_ELEMENT , nnumberofbytestoread : u32 , lpreserved : *const u32 , lpoverlapped : *mut super::super::System::IO:: OVERLAPPED ) -> super::super::Foundation:: BOOL );
    ReadFileScatter(hfile.into_param().abi(), asegmentarray, nnumberofbytestoread, ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())), lpoverlapped)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadLogArchiveMetadata(pvarchivecontext: *mut ::core::ffi::c_void, cboffset: u32, cbbytestoread: u32, pbreadbuffer: *mut u8, pcbbytesread: *mut u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "clfsw32.dll""system" fn ReadLogArchiveMetadata ( pvarchivecontext : *mut ::core::ffi::c_void , cboffset : u32 , cbbytestoread : u32 , pbreadbuffer : *mut u8 , pcbbytesread : *mut u32 ) -> super::super::Foundation:: BOOL );
    ReadLogArchiveMetadata(pvarchivecontext, cboffset, cbbytestoread, pbreadbuffer, pcbbytesread)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn ReadLogNotification<P0>(hlog: P0, pnotification: *mut CLFS_MGMT_NOTIFICATION, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "clfsw32.dll""system" fn ReadLogNotification ( hlog : super::super::Foundation:: HANDLE , pnotification : *mut CLFS_MGMT_NOTIFICATION , lpoverlapped : *mut super::super::System::IO:: OVERLAPPED ) -> super::super::Foundation:: BOOL );
    ReadLogNotification(hlog.into_param().abi(), pnotification, lpoverlapped)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn ReadLogRecord(pvmarshal: *mut ::core::ffi::c_void, plsnfirst: *mut CLS_LSN, econtextmode: CLFS_CONTEXT_MODE, ppvreadbuffer: *mut *mut ::core::ffi::c_void, pcbreadbuffer: *mut u32, perecordtype: *mut u8, plsnundonext: *mut CLS_LSN, plsnprevious: *mut CLS_LSN, ppvreadcontext: *mut *mut ::core::ffi::c_void, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "clfsw32.dll""system" fn ReadLogRecord ( pvmarshal : *mut ::core::ffi::c_void , plsnfirst : *mut CLS_LSN , econtextmode : CLFS_CONTEXT_MODE , ppvreadbuffer : *mut *mut ::core::ffi::c_void , pcbreadbuffer : *mut u32 , perecordtype : *mut u8 , plsnundonext : *mut CLS_LSN , plsnprevious : *mut CLS_LSN , ppvreadcontext : *mut *mut ::core::ffi::c_void , poverlapped : *mut super::super::System::IO:: OVERLAPPED ) -> super::super::Foundation:: BOOL );
    ReadLogRecord(pvmarshal, plsnfirst, econtextmode, ppvreadbuffer, pcbreadbuffer, perecordtype, plsnundonext, plsnprevious, ppvreadcontext, poverlapped)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn ReadLogRestartArea(pvmarshal: *mut ::core::ffi::c_void, ppvrestartbuffer: *mut *mut ::core::ffi::c_void, pcbrestartbuffer: *mut u32, plsn: *mut CLS_LSN, ppvcontext: *mut *mut ::core::ffi::c_void, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "clfsw32.dll""system" fn ReadLogRestartArea ( pvmarshal : *mut ::core::ffi::c_void , ppvrestartbuffer : *mut *mut ::core::ffi::c_void , pcbrestartbuffer : *mut u32 , plsn : *mut CLS_LSN , ppvcontext : *mut *mut ::core::ffi::c_void , poverlapped : *mut super::super::System::IO:: OVERLAPPED ) -> super::super::Foundation:: BOOL );
    ReadLogRestartArea(pvmarshal, ppvrestartbuffer, pcbrestartbuffer, plsn, ppvcontext, poverlapped)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn ReadNextLogRecord(pvreadcontext: *mut ::core::ffi::c_void, ppvbuffer: *mut *mut ::core::ffi::c_void, pcbbuffer: *mut u32, perecordtype: *mut u8, plsnuser: *mut CLS_LSN, plsnundonext: *mut CLS_LSN, plsnprevious: *mut CLS_LSN, plsnrecord: *mut CLS_LSN, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "clfsw32.dll""system" fn ReadNextLogRecord ( pvreadcontext : *mut ::core::ffi::c_void , ppvbuffer : *mut *mut ::core::ffi::c_void , pcbbuffer : *mut u32 , perecordtype : *mut u8 , plsnuser : *mut CLS_LSN , plsnundonext : *mut CLS_LSN , plsnprevious : *mut CLS_LSN , plsnrecord : *mut CLS_LSN , poverlapped : *mut super::super::System::IO:: OVERLAPPED ) -> super::super::Foundation:: BOOL );
    ReadNextLogRecord(pvreadcontext, ppvbuffer, pcbbuffer, perecordtype, plsnuser, plsnundonext, plsnprevious, plsnrecord, poverlapped)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadOnlyEnlistment<P0>(enlistmenthandle: P0, tmvirtualclock: *mut i64) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ktmw32.dll""system" fn ReadOnlyEnlistment ( enlistmenthandle : super::super::Foundation:: HANDLE , tmvirtualclock : *mut i64 ) -> super::super::Foundation:: BOOL );
    ReadOnlyEnlistment(enlistmenthandle.into_param().abi(), tmvirtualclock)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn ReadPreviousLogRestartArea(pvreadcontext: *mut ::core::ffi::c_void, ppvrestartbuffer: *mut *mut ::core::ffi::c_void, pcbrestartbuffer: *mut u32, plsnrestart: *mut CLS_LSN, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "clfsw32.dll""system" fn ReadPreviousLogRestartArea ( pvreadcontext : *mut ::core::ffi::c_void , ppvrestartbuffer : *mut *mut ::core::ffi::c_void , pcbrestartbuffer : *mut u32 , plsnrestart : *mut CLS_LSN , poverlapped : *mut super::super::System::IO:: OVERLAPPED ) -> super::super::Foundation:: BOOL );
    ReadPreviousLogRestartArea(pvreadcontext, ppvrestartbuffer, pcbrestartbuffer, plsnrestart, poverlapped)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RecoverEnlistment<P0>(enlistmenthandle: P0, enlistmentkey: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ktmw32.dll""system" fn RecoverEnlistment ( enlistmenthandle : super::super::Foundation:: HANDLE , enlistmentkey : *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    RecoverEnlistment(enlistmenthandle.into_param().abi(), enlistmentkey)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RecoverResourceManager<P0>(resourcemanagerhandle: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ktmw32.dll""system" fn RecoverResourceManager ( resourcemanagerhandle : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    RecoverResourceManager(resourcemanagerhandle.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RecoverTransactionManager<P0>(transactionmanagerhandle: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ktmw32.dll""system" fn RecoverTransactionManager ( transactionmanagerhandle : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    RecoverTransactionManager(transactionmanagerhandle.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterForLogWriteNotification<P0, P1>(hlog: P0, cbthreshold: u32, fenable: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "clfsw32.dll""system" fn RegisterForLogWriteNotification ( hlog : super::super::Foundation:: HANDLE , cbthreshold : u32 , fenable : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    RegisterForLogWriteNotification(hlog.into_param().abi(), cbthreshold, fenable.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterManageableLogClient<P0>(hlog: P0, pcallbacks: *mut LOG_MANAGEMENT_CALLBACKS) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "clfsw32.dll""system" fn RegisterManageableLogClient ( hlog : super::super::Foundation:: HANDLE , pcallbacks : *mut LOG_MANAGEMENT_CALLBACKS ) -> super::super::Foundation:: BOOL );
    RegisterManageableLogClient(hlog.into_param().abi(), pcallbacks)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemoveDirectoryA<P0>(lppathname: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn RemoveDirectoryA ( lppathname : ::windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    RemoveDirectoryA(lppathname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemoveDirectoryFromAppW<P0>(lppathname: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "api-ms-win-core-file-fromapp-l1-1-0.dll""system" fn RemoveDirectoryFromAppW ( lppathname : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    RemoveDirectoryFromAppW(lppathname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemoveDirectoryTransactedA<P0, P1>(lppathname: P0, htransaction: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn RemoveDirectoryTransactedA ( lppathname : ::windows::core::PCSTR , htransaction : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    RemoveDirectoryTransactedA(lppathname.into_param().abi(), htransaction.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemoveDirectoryTransactedW<P0, P1>(lppathname: P0, htransaction: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn RemoveDirectoryTransactedW ( lppathname : ::windows::core::PCWSTR , htransaction : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    RemoveDirectoryTransactedW(lppathname.into_param().abi(), htransaction.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemoveDirectoryW<P0>(lppathname: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn RemoveDirectoryW ( lppathname : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    RemoveDirectoryW(lppathname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemoveLogContainer<P0, P1, P2>(hlog: P0, pwszcontainerpath: P1, fforce: P2, preserved: ::core::option::Option<*mut ::core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "clfsw32.dll""system" fn RemoveLogContainer ( hlog : super::super::Foundation:: HANDLE , pwszcontainerpath : ::windows::core::PCWSTR , fforce : super::super::Foundation:: BOOL , preserved : *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    RemoveLogContainer(hlog.into_param().abi(), pwszcontainerpath.into_param().abi(), fforce.into_param().abi(), ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemoveLogContainerSet<P0, P1>(hlog: P0, rgwszcontainerpath: &[::windows::core::PCWSTR], fforce: P1, preserved: ::core::option::Option<*mut ::core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "clfsw32.dll""system" fn RemoveLogContainerSet ( hlog : super::super::Foundation:: HANDLE , ccontainer : u16 , rgwszcontainerpath : *const ::windows::core::PCWSTR , fforce : super::super::Foundation:: BOOL , preserved : *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    RemoveLogContainerSet(hlog.into_param().abi(), rgwszcontainerpath.len() as _, ::core::mem::transmute(rgwszcontainerpath.as_ptr()), fforce.into_param().abi(), ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemoveLogPolicy<P0>(hlog: P0, epolicytype: CLFS_MGMT_POLICY_TYPE) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "clfsw32.dll""system" fn RemoveLogPolicy ( hlog : super::super::Foundation:: HANDLE , epolicytype : CLFS_MGMT_POLICY_TYPE ) -> super::super::Foundation:: BOOL );
    RemoveLogPolicy(hlog.into_param().abi(), epolicytype)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RemoveUsersFromEncryptedFile<P0>(lpfilename: P0, phashes: *const ENCRYPTION_CERTIFICATE_HASH_LIST) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn RemoveUsersFromEncryptedFile ( lpfilename : ::windows::core::PCWSTR , phashes : *const ENCRYPTION_CERTIFICATE_HASH_LIST ) -> u32 );
    RemoveUsersFromEncryptedFile(lpfilename.into_param().abi(), phashes)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RenameTransactionManager<P0>(logfilename: P0, existingtransactionmanagerguid: *mut ::windows::core::GUID) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "ktmw32.dll""system" fn RenameTransactionManager ( logfilename : ::windows::core::PCWSTR , existingtransactionmanagerguid : *mut ::windows::core::GUID ) -> super::super::Foundation:: BOOL );
    RenameTransactionManager(logfilename.into_param().abi(), existingtransactionmanagerguid)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReplaceFileA<P0, P1, P2>(lpreplacedfilename: P0, lpreplacementfilename: P1, lpbackupfilename: P2, dwreplaceflags: REPLACE_FILE_FLAGS, lpexclude: ::core::option::Option<*const ::core::ffi::c_void>, lpreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn ReplaceFileA ( lpreplacedfilename : ::windows::core::PCSTR , lpreplacementfilename : ::windows::core::PCSTR , lpbackupfilename : ::windows::core::PCSTR , dwreplaceflags : REPLACE_FILE_FLAGS , lpexclude : *const ::core::ffi::c_void , lpreserved : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    ReplaceFileA(lpreplacedfilename.into_param().abi(), lpreplacementfilename.into_param().abi(), lpbackupfilename.into_param().abi(), dwreplaceflags, ::core::mem::transmute(lpexclude.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReplaceFileFromAppW<P0, P1, P2>(lpreplacedfilename: P0, lpreplacementfilename: P1, lpbackupfilename: P2, dwreplaceflags: u32, lpexclude: ::core::option::Option<*const ::core::ffi::c_void>, lpreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "api-ms-win-core-file-fromapp-l1-1-0.dll""system" fn ReplaceFileFromAppW ( lpreplacedfilename : ::windows::core::PCWSTR , lpreplacementfilename : ::windows::core::PCWSTR , lpbackupfilename : ::windows::core::PCWSTR , dwreplaceflags : u32 , lpexclude : *const ::core::ffi::c_void , lpreserved : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    ReplaceFileFromAppW(lpreplacedfilename.into_param().abi(), lpreplacementfilename.into_param().abi(), lpbackupfilename.into_param().abi(), dwreplaceflags, ::core::mem::transmute(lpexclude.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReplaceFileW<P0, P1, P2>(lpreplacedfilename: P0, lpreplacementfilename: P1, lpbackupfilename: P2, dwreplaceflags: REPLACE_FILE_FLAGS, lpexclude: ::core::option::Option<*const ::core::ffi::c_void>, lpreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn ReplaceFileW ( lpreplacedfilename : ::windows::core::PCWSTR , lpreplacementfilename : ::windows::core::PCWSTR , lpbackupfilename : ::windows::core::PCWSTR , dwreplaceflags : REPLACE_FILE_FLAGS , lpexclude : *const ::core::ffi::c_void , lpreserved : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    ReplaceFileW(lpreplacedfilename.into_param().abi(), lpreplacementfilename.into_param().abi(), lpbackupfilename.into_param().abi(), dwreplaceflags, ::core::mem::transmute(lpexclude.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn ReserveAndAppendLog(pvmarshal: *mut ::core::ffi::c_void, rgwriteentries: *mut CLS_WRITE_ENTRY, cwriteentries: u32, plsnundonext: *mut CLS_LSN, plsnprevious: *mut CLS_LSN, creserverecords: u32, rgcbreservation: *mut i64, fflags: CLFS_FLAG, plsn: *mut CLS_LSN, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "clfsw32.dll""system" fn ReserveAndAppendLog ( pvmarshal : *mut ::core::ffi::c_void , rgwriteentries : *mut CLS_WRITE_ENTRY , cwriteentries : u32 , plsnundonext : *mut CLS_LSN , plsnprevious : *mut CLS_LSN , creserverecords : u32 , rgcbreservation : *mut i64 , fflags : CLFS_FLAG , plsn : *mut CLS_LSN , poverlapped : *mut super::super::System::IO:: OVERLAPPED ) -> super::super::Foundation:: BOOL );
    ReserveAndAppendLog(pvmarshal, rgwriteentries, cwriteentries, plsnundonext, plsnprevious, creserverecords, rgcbreservation, fflags, plsn, poverlapped)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn ReserveAndAppendLogAligned(pvmarshal: *mut ::core::ffi::c_void, rgwriteentries: *mut CLS_WRITE_ENTRY, cwriteentries: u32, cbentryalignment: u32, plsnundonext: *mut CLS_LSN, plsnprevious: *mut CLS_LSN, creserverecords: u32, rgcbreservation: *mut i64, fflags: CLFS_FLAG, plsn: *mut CLS_LSN, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "clfsw32.dll""system" fn ReserveAndAppendLogAligned ( pvmarshal : *mut ::core::ffi::c_void , rgwriteentries : *mut CLS_WRITE_ENTRY , cwriteentries : u32 , cbentryalignment : u32 , plsnundonext : *mut CLS_LSN , plsnprevious : *mut CLS_LSN , creserverecords : u32 , rgcbreservation : *mut i64 , fflags : CLFS_FLAG , plsn : *mut CLS_LSN , poverlapped : *mut super::super::System::IO:: OVERLAPPED ) -> super::super::Foundation:: BOOL );
    ReserveAndAppendLogAligned(pvmarshal, rgwriteentries, cwriteentries, cbentryalignment, plsnundonext, plsnprevious, creserverecords, rgcbreservation, fflags, plsn, poverlapped)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RollbackComplete<P0>(enlistmenthandle: P0, tmvirtualclock: *mut i64) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ktmw32.dll""system" fn RollbackComplete ( enlistmenthandle : super::super::Foundation:: HANDLE , tmvirtualclock : *mut i64 ) -> super::super::Foundation:: BOOL );
    RollbackComplete(enlistmenthandle.into_param().abi(), tmvirtualclock)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RollbackEnlistment<P0>(enlistmenthandle: P0, tmvirtualclock: *mut i64) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ktmw32.dll""system" fn RollbackEnlistment ( enlistmenthandle : super::super::Foundation:: HANDLE , tmvirtualclock : *mut i64 ) -> super::super::Foundation:: BOOL );
    RollbackEnlistment(enlistmenthandle.into_param().abi(), tmvirtualclock)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RollbackTransaction<P0>(transactionhandle: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ktmw32.dll""system" fn RollbackTransaction ( transactionhandle : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    RollbackTransaction(transactionhandle.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RollbackTransactionAsync<P0>(transactionhandle: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ktmw32.dll""system" fn RollbackTransactionAsync ( transactionhandle : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    RollbackTransactionAsync(transactionhandle.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RollforwardTransactionManager<P0>(transactionmanagerhandle: P0, tmvirtualclock: *mut i64) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ktmw32.dll""system" fn RollforwardTransactionManager ( transactionmanagerhandle : super::super::Foundation:: HANDLE , tmvirtualclock : *mut i64 ) -> super::super::Foundation:: BOOL );
    RollforwardTransactionManager(transactionmanagerhandle.into_param().abi(), tmvirtualclock)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ScanLogContainers(pcxscan: *mut CLS_SCAN_CONTEXT, escanmode: u8, preserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "clfsw32.dll""system" fn ScanLogContainers ( pcxscan : *mut CLS_SCAN_CONTEXT , escanmode : u8 , preserved : *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    ScanLogContainers(pcxscan, escanmode, preserved)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn SearchPathA<P0, P1, P2>(lppath: P0, lpfilename: P1, lpextension: P2, lpbuffer: ::core::option::Option<&mut [u8]>, lpfilepart: ::core::option::Option<*mut ::windows::core::PSTR>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SearchPathA ( lppath : ::windows::core::PCSTR , lpfilename : ::windows::core::PCSTR , lpextension : ::windows::core::PCSTR , nbufferlength : u32 , lpbuffer : ::windows::core::PSTR , lpfilepart : *mut ::windows::core::PSTR ) -> u32 );
    SearchPathA(lppath.into_param().abi(), lpfilename.into_param().abi(), lpextension.into_param().abi(), lpbuffer.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lpbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ::core::mem::transmute(lpfilepart.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn SearchPathW<P0, P1, P2>(lppath: P0, lpfilename: P1, lpextension: P2, lpbuffer: ::core::option::Option<&mut [u16]>, lpfilepart: ::core::option::Option<*mut ::windows::core::PWSTR>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SearchPathW ( lppath : ::windows::core::PCWSTR , lpfilename : ::windows::core::PCWSTR , lpextension : ::windows::core::PCWSTR , nbufferlength : u32 , lpbuffer : ::windows::core::PWSTR , lpfilepart : *mut ::windows::core::PWSTR ) -> u32 );
    SearchPathW(lppath.into_param().abi(), lpfilename.into_param().abi(), lpextension.into_param().abi(), lpbuffer.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lpbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ::core::mem::transmute(lpfilepart.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn SetEncryptedFileMetadata<P0>(lpfilename: P0, pboldmetadata: ::core::option::Option<*const u8>, pbnewmetadata: *const u8, pownerhash: *const ENCRYPTION_CERTIFICATE_HASH, dwoperation: u32, pcertificatesadded: ::core::option::Option<*const ENCRYPTION_CERTIFICATE_HASH_LIST>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn SetEncryptedFileMetadata ( lpfilename : ::windows::core::PCWSTR , pboldmetadata : *const u8 , pbnewmetadata : *const u8 , pownerhash : *const ENCRYPTION_CERTIFICATE_HASH , dwoperation : u32 , pcertificatesadded : *const ENCRYPTION_CERTIFICATE_HASH_LIST ) -> u32 );
    SetEncryptedFileMetadata(lpfilename.into_param().abi(), ::core::mem::transmute(pboldmetadata.unwrap_or(::std::ptr::null())), pbnewmetadata, pownerhash, dwoperation, ::core::mem::transmute(pcertificatesadded.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetEndOfFile<P0>(hfile: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetEndOfFile ( hfile : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    SetEndOfFile(hfile.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn SetEndOfLog<P0>(hlog: P0, plsnend: *mut CLS_LSN, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "clfsw32.dll""system" fn SetEndOfLog ( hlog : super::super::Foundation:: HANDLE , plsnend : *mut CLS_LSN , lpoverlapped : *mut super::super::System::IO:: OVERLAPPED ) -> super::super::Foundation:: BOOL );
    SetEndOfLog(hlog.into_param().abi(), plsnend, lpoverlapped)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetEnlistmentRecoveryInformation<P0>(enlistmenthandle: P0, buffersize: u32, buffer: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ktmw32.dll""system" fn SetEnlistmentRecoveryInformation ( enlistmenthandle : super::super::Foundation:: HANDLE , buffersize : u32 , buffer : *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    SetEnlistmentRecoveryInformation(enlistmenthandle.into_param().abi(), buffersize, buffer)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn SetFileApisToANSI() {
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetFileApisToANSI ( ) -> ( ) );
    SetFileApisToANSI()
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn SetFileApisToOEM() {
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetFileApisToOEM ( ) -> ( ) );
    SetFileApisToOEM()
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFileAttributesA<P0>(lpfilename: P0, dwfileattributes: FILE_FLAGS_AND_ATTRIBUTES) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetFileAttributesA ( lpfilename : ::windows::core::PCSTR , dwfileattributes : FILE_FLAGS_AND_ATTRIBUTES ) -> super::super::Foundation:: BOOL );
    SetFileAttributesA(lpfilename.into_param().abi(), dwfileattributes)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFileAttributesFromAppW<P0>(lpfilename: P0, dwfileattributes: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "api-ms-win-core-file-fromapp-l1-1-0.dll""system" fn SetFileAttributesFromAppW ( lpfilename : ::windows::core::PCWSTR , dwfileattributes : u32 ) -> super::super::Foundation:: BOOL );
    SetFileAttributesFromAppW(lpfilename.into_param().abi(), dwfileattributes)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFileAttributesTransactedA<P0, P1>(lpfilename: P0, dwfileattributes: u32, htransaction: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetFileAttributesTransactedA ( lpfilename : ::windows::core::PCSTR , dwfileattributes : u32 , htransaction : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    SetFileAttributesTransactedA(lpfilename.into_param().abi(), dwfileattributes, htransaction.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFileAttributesTransactedW<P0, P1>(lpfilename: P0, dwfileattributes: u32, htransaction: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetFileAttributesTransactedW ( lpfilename : ::windows::core::PCWSTR , dwfileattributes : u32 , htransaction : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    SetFileAttributesTransactedW(lpfilename.into_param().abi(), dwfileattributes, htransaction.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFileAttributesW<P0>(lpfilename: P0, dwfileattributes: FILE_FLAGS_AND_ATTRIBUTES) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetFileAttributesW ( lpfilename : ::windows::core::PCWSTR , dwfileattributes : FILE_FLAGS_AND_ATTRIBUTES ) -> super::super::Foundation:: BOOL );
    SetFileAttributesW(lpfilename.into_param().abi(), dwfileattributes)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFileBandwidthReservation<P0, P1>(hfile: P0, nperiodmilliseconds: u32, nbytesperperiod: u32, bdiscardable: P1, lptransfersize: *mut u32, lpnumoutstandingrequests: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetFileBandwidthReservation ( hfile : super::super::Foundation:: HANDLE , nperiodmilliseconds : u32 , nbytesperperiod : u32 , bdiscardable : super::super::Foundation:: BOOL , lptransfersize : *mut u32 , lpnumoutstandingrequests : *mut u32 ) -> super::super::Foundation:: BOOL );
    SetFileBandwidthReservation(hfile.into_param().abi(), nperiodmilliseconds, nbytesperperiod, bdiscardable.into_param().abi(), lptransfersize, lpnumoutstandingrequests)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFileCompletionNotificationModes<P0>(filehandle: P0, flags: u8) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetFileCompletionNotificationModes ( filehandle : super::super::Foundation:: HANDLE , flags : u8 ) -> super::super::Foundation:: BOOL );
    SetFileCompletionNotificationModes(filehandle.into_param().abi(), flags)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFileInformationByHandle<P0>(hfile: P0, fileinformationclass: FILE_INFO_BY_HANDLE_CLASS, lpfileinformation: *const ::core::ffi::c_void, dwbuffersize: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetFileInformationByHandle ( hfile : super::super::Foundation:: HANDLE , fileinformationclass : FILE_INFO_BY_HANDLE_CLASS , lpfileinformation : *const ::core::ffi::c_void , dwbuffersize : u32 ) -> super::super::Foundation:: BOOL );
    SetFileInformationByHandle(hfile.into_param().abi(), fileinformationclass, lpfileinformation, dwbuffersize)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFileIoOverlappedRange<P0>(filehandle: P0, overlappedrangestart: *const u8, length: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetFileIoOverlappedRange ( filehandle : super::super::Foundation:: HANDLE , overlappedrangestart : *const u8 , length : u32 ) -> super::super::Foundation:: BOOL );
    SetFileIoOverlappedRange(filehandle.into_param().abi(), overlappedrangestart, length)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFilePointer<P0>(hfile: P0, ldistancetomove: i32, lpdistancetomovehigh: ::core::option::Option<*mut i32>, dwmovemethod: SET_FILE_POINTER_MOVE_METHOD) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetFilePointer ( hfile : super::super::Foundation:: HANDLE , ldistancetomove : i32 , lpdistancetomovehigh : *mut i32 , dwmovemethod : SET_FILE_POINTER_MOVE_METHOD ) -> u32 );
    SetFilePointer(hfile.into_param().abi(), ldistancetomove, ::core::mem::transmute(lpdistancetomovehigh.unwrap_or(::std::ptr::null_mut())), dwmovemethod)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFilePointerEx<P0>(hfile: P0, lidistancetomove: i64, lpnewfilepointer: ::core::option::Option<*mut i64>, dwmovemethod: SET_FILE_POINTER_MOVE_METHOD) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetFilePointerEx ( hfile : super::super::Foundation:: HANDLE , lidistancetomove : i64 , lpnewfilepointer : *mut i64 , dwmovemethod : SET_FILE_POINTER_MOVE_METHOD ) -> super::super::Foundation:: BOOL );
    SetFilePointerEx(hfile.into_param().abi(), lidistancetomove, ::core::mem::transmute(lpnewfilepointer.unwrap_or(::std::ptr::null_mut())), dwmovemethod)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFileShortNameA<P0, P1>(hfile: P0, lpshortname: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetFileShortNameA ( hfile : super::super::Foundation:: HANDLE , lpshortname : ::windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    SetFileShortNameA(hfile.into_param().abi(), lpshortname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFileShortNameW<P0, P1>(hfile: P0, lpshortname: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetFileShortNameW ( hfile : super::super::Foundation:: HANDLE , lpshortname : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    SetFileShortNameW(hfile.into_param().abi(), lpshortname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFileTime<P0>(hfile: P0, lpcreationtime: ::core::option::Option<*const super::super::Foundation::FILETIME>, lplastaccesstime: ::core::option::Option<*const super::super::Foundation::FILETIME>, lplastwritetime: ::core::option::Option<*const super::super::Foundation::FILETIME>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetFileTime ( hfile : super::super::Foundation:: HANDLE , lpcreationtime : *const super::super::Foundation:: FILETIME , lplastaccesstime : *const super::super::Foundation:: FILETIME , lplastwritetime : *const super::super::Foundation:: FILETIME ) -> super::super::Foundation:: BOOL );
    SetFileTime(hfile.into_param().abi(), ::core::mem::transmute(lpcreationtime.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lplastaccesstime.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lplastwritetime.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFileValidData<P0>(hfile: P0, validdatalength: i64) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetFileValidData ( hfile : super::super::Foundation:: HANDLE , validdatalength : i64 ) -> super::super::Foundation:: BOOL );
    SetFileValidData(hfile.into_param().abi(), validdatalength)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetIoRingCompletionEvent<P0>(ioring: *const HIORING__, hevent: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "api-ms-win-core-ioring-l1-1-0.dll""system" fn SetIoRingCompletionEvent ( ioring : *const HIORING__ , hevent : super::super::Foundation:: HANDLE ) -> ::windows::core::HRESULT );
    SetIoRingCompletionEvent(ioring, hevent.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetLogArchiveMode<P0>(hlog: P0, emode: CLFS_LOG_ARCHIVE_MODE) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "clfsw32.dll""system" fn SetLogArchiveMode ( hlog : super::super::Foundation:: HANDLE , emode : CLFS_LOG_ARCHIVE_MODE ) -> super::super::Foundation:: BOOL );
    SetLogArchiveMode(hlog.into_param().abi(), emode)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetLogArchiveTail<P0>(hlog: P0, plsnarchivetail: *mut CLS_LSN, preserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "clfsw32.dll""system" fn SetLogArchiveTail ( hlog : super::super::Foundation:: HANDLE , plsnarchivetail : *mut CLS_LSN , preserved : *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    SetLogArchiveTail(hlog.into_param().abi(), plsnarchivetail, preserved)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetLogFileSizeWithPolicy<P0>(hlog: P0, pdesiredsize: *const u64, presultingsize: *mut u64) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "clfsw32.dll""system" fn SetLogFileSizeWithPolicy ( hlog : super::super::Foundation:: HANDLE , pdesiredsize : *const u64 , presultingsize : *mut u64 ) -> super::super::Foundation:: BOOL );
    SetLogFileSizeWithPolicy(hlog.into_param().abi(), pdesiredsize, presultingsize)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetResourceManagerCompletionPort<P0, P1>(resourcemanagerhandle: P0, iocompletionporthandle: P1, completionkey: usize) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ktmw32.dll""system" fn SetResourceManagerCompletionPort ( resourcemanagerhandle : super::super::Foundation:: HANDLE , iocompletionporthandle : super::super::Foundation:: HANDLE , completionkey : usize ) -> super::super::Foundation:: BOOL );
    SetResourceManagerCompletionPort(resourcemanagerhandle.into_param().abi(), iocompletionporthandle.into_param().abi(), completionkey)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetSearchPathMode(flags: u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetSearchPathMode ( flags : u32 ) -> super::super::Foundation:: BOOL );
    SetSearchPathMode(flags)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetTapeParameters<P0>(hdevice: P0, dwoperation: TAPE_INFORMATION_TYPE, lptapeinformation: *const ::core::ffi::c_void) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetTapeParameters ( hdevice : super::super::Foundation:: HANDLE , dwoperation : TAPE_INFORMATION_TYPE , lptapeinformation : *const ::core::ffi::c_void ) -> u32 );
    SetTapeParameters(hdevice.into_param().abi(), dwoperation, lptapeinformation)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetTapePosition<P0, P1>(hdevice: P0, dwpositionmethod: TAPE_POSITION_METHOD, dwpartition: u32, dwoffsetlow: u32, dwoffsethigh: u32, bimmediate: P1) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetTapePosition ( hdevice : super::super::Foundation:: HANDLE , dwpositionmethod : TAPE_POSITION_METHOD , dwpartition : u32 , dwoffsetlow : u32 , dwoffsethigh : u32 , bimmediate : super::super::Foundation:: BOOL ) -> u32 );
    SetTapePosition(hdevice.into_param().abi(), dwpositionmethod, dwpartition, dwoffsetlow, dwoffsethigh, bimmediate.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetTransactionInformation<P0, P1>(transactionhandle: P0, isolationlevel: u32, isolationflags: u32, timeout: u32, description: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "ktmw32.dll""system" fn SetTransactionInformation ( transactionhandle : super::super::Foundation:: HANDLE , isolationlevel : u32 , isolationflags : u32 , timeout : u32 , description : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    SetTransactionInformation(transactionhandle.into_param().abi(), isolationlevel, isolationflags, timeout, description.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn SetUserFileEncryptionKey(pencryptioncertificate: ::core::option::Option<*const ENCRYPTION_CERTIFICATE>) -> u32 {
    ::windows_targets::link ! ( "advapi32.dll""system" fn SetUserFileEncryptionKey ( pencryptioncertificate : *const ENCRYPTION_CERTIFICATE ) -> u32 );
    SetUserFileEncryptionKey(::core::mem::transmute(pencryptioncertificate.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn SetUserFileEncryptionKeyEx(pencryptioncertificate: ::core::option::Option<*const ENCRYPTION_CERTIFICATE>, dwcapabilities: u32, dwflags: u32, pvreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> u32 {
    ::windows_targets::link ! ( "advapi32.dll""system" fn SetUserFileEncryptionKeyEx ( pencryptioncertificate : *const ENCRYPTION_CERTIFICATE , dwcapabilities : u32 , dwflags : u32 , pvreserved : *const ::core::ffi::c_void ) -> u32 );
    SetUserFileEncryptionKeyEx(::core::mem::transmute(pencryptioncertificate.unwrap_or(::std::ptr::null())), dwcapabilities, dwflags, ::core::mem::transmute(pvreserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetVolumeLabelA<P0, P1>(lprootpathname: P0, lpvolumename: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetVolumeLabelA ( lprootpathname : ::windows::core::PCSTR , lpvolumename : ::windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    SetVolumeLabelA(lprootpathname.into_param().abi(), lpvolumename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetVolumeLabelW<P0, P1>(lprootpathname: P0, lpvolumename: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetVolumeLabelW ( lprootpathname : ::windows::core::PCWSTR , lpvolumename : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    SetVolumeLabelW(lprootpathname.into_param().abi(), lpvolumename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetVolumeMountPointA<P0, P1>(lpszvolumemountpoint: P0, lpszvolumename: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetVolumeMountPointA ( lpszvolumemountpoint : ::windows::core::PCSTR , lpszvolumename : ::windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    SetVolumeMountPointA(lpszvolumemountpoint.into_param().abi(), lpszvolumename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetVolumeMountPointW<P0, P1>(lpszvolumemountpoint: P0, lpszvolumename: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetVolumeMountPointW ( lpszvolumemountpoint : ::windows::core::PCWSTR , lpszvolumename : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    SetVolumeMountPointW(lpszvolumemountpoint.into_param().abi(), lpszvolumename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SinglePhaseReject<P0>(enlistmenthandle: P0, tmvirtualclock: *mut i64) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ktmw32.dll""system" fn SinglePhaseReject ( enlistmenthandle : super::super::Foundation:: HANDLE , tmvirtualclock : *mut i64 ) -> super::super::Foundation:: BOOL );
    SinglePhaseReject(enlistmenthandle.into_param().abi(), tmvirtualclock)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn SubmitIoRing(ioring: *const HIORING__, waitoperations: u32, milliseconds: u32, submittedentries: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "api-ms-win-core-ioring-l1-1-0.dll""system" fn SubmitIoRing ( ioring : *const HIORING__ , waitoperations : u32 , milliseconds : u32 , submittedentries : *mut u32 ) -> ::windows::core::HRESULT );
    SubmitIoRing(ioring, waitoperations, milliseconds, ::core::mem::transmute(submittedentries.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TerminateLogArchive(pvarchivecontext: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "clfsw32.dll""system" fn TerminateLogArchive ( pvarchivecontext : *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    TerminateLogArchive(pvarchivecontext)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TerminateReadLog(pvcursorcontext: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "clfsw32.dll""system" fn TerminateReadLog ( pvcursorcontext : *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    TerminateReadLog(pvcursorcontext)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn TruncateLog(pvmarshal: *const ::core::ffi::c_void, plsnend: *const CLS_LSN, lpoverlapped: ::core::option::Option<*mut super::super::System::IO::OVERLAPPED>) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "clfsw32.dll""system" fn TruncateLog ( pvmarshal : *const ::core::ffi::c_void , plsnend : *const CLS_LSN , lpoverlapped : *mut super::super::System::IO:: OVERLAPPED ) -> super::super::Foundation:: BOOL );
    TruncateLog(pvmarshal, plsnend, ::core::mem::transmute(lpoverlapped.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn TxfGetThreadMiniVersionForCreate() -> u16 {
    ::windows_targets::link ! ( "txfw32.dll""system" fn TxfGetThreadMiniVersionForCreate ( miniversion : *mut u16 ) -> ( ) );
    let mut result__ = ::windows::core::zeroed::<u16>();
    TxfGetThreadMiniVersionForCreate(&mut result__);
    ::std::mem::transmute(result__)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TxfLogCreateFileReadContext<P0>(logpath: P0, beginninglsn: CLS_LSN, endinglsn: CLS_LSN, txffileid: *const TXF_ID, txflogcontext: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "txfw32.dll""system" fn TxfLogCreateFileReadContext ( logpath : ::windows::core::PCWSTR , beginninglsn : CLS_LSN , endinglsn : CLS_LSN , txffileid : *const TXF_ID , txflogcontext : *mut *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    TxfLogCreateFileReadContext(logpath.into_param().abi(), ::core::mem::transmute(beginninglsn), ::core::mem::transmute(endinglsn), txffileid, txflogcontext)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TxfLogCreateRangeReadContext<P0>(logpath: P0, beginninglsn: CLS_LSN, endinglsn: CLS_LSN, beginningvirtualclock: *const i64, endingvirtualclock: *const i64, recordtypemask: u32, txflogcontext: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "txfw32.dll""system" fn TxfLogCreateRangeReadContext ( logpath : ::windows::core::PCWSTR , beginninglsn : CLS_LSN , endinglsn : CLS_LSN , beginningvirtualclock : *const i64 , endingvirtualclock : *const i64 , recordtypemask : u32 , txflogcontext : *mut *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    TxfLogCreateRangeReadContext(logpath.into_param().abi(), ::core::mem::transmute(beginninglsn), ::core::mem::transmute(endinglsn), beginningvirtualclock, endingvirtualclock, recordtypemask, txflogcontext)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TxfLogDestroyReadContext(txflogcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "txfw32.dll""system" fn TxfLogDestroyReadContext ( txflogcontext : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    TxfLogDestroyReadContext(txflogcontext)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TxfLogReadRecords(txflogcontext: *const ::core::ffi::c_void, bufferlength: u32, buffer: *mut ::core::ffi::c_void, bytesused: *mut u32, recordcount: *mut u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "txfw32.dll""system" fn TxfLogReadRecords ( txflogcontext : *const ::core::ffi::c_void , bufferlength : u32 , buffer : *mut ::core::ffi::c_void , bytesused : *mut u32 , recordcount : *mut u32 ) -> super::super::Foundation:: BOOL );
    TxfLogReadRecords(txflogcontext, bufferlength, buffer, bytesused, recordcount)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TxfLogRecordGetFileName(recordbuffer: *const ::core::ffi::c_void, recordbufferlengthinbytes: u32, namebuffer: ::windows::core::PWSTR, namebufferlengthinbytes: *mut u32, txfid: ::core::option::Option<*mut TXF_ID>) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "txfw32.dll""system" fn TxfLogRecordGetFileName ( recordbuffer : *const ::core::ffi::c_void , recordbufferlengthinbytes : u32 , namebuffer : ::windows::core::PWSTR , namebufferlengthinbytes : *mut u32 , txfid : *mut TXF_ID ) -> super::super::Foundation:: BOOL );
    TxfLogRecordGetFileName(recordbuffer, recordbufferlengthinbytes, ::core::mem::transmute(namebuffer), namebufferlengthinbytes, ::core::mem::transmute(txfid.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TxfLogRecordGetGenericType(recordbuffer: *const ::core::ffi::c_void, recordbufferlengthinbytes: u32, generictype: *mut u32, virtualclock: ::core::option::Option<*mut i64>) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "txfw32.dll""system" fn TxfLogRecordGetGenericType ( recordbuffer : *const ::core::ffi::c_void , recordbufferlengthinbytes : u32 , generictype : *mut u32 , virtualclock : *mut i64 ) -> super::super::Foundation:: BOOL );
    TxfLogRecordGetGenericType(recordbuffer, recordbufferlengthinbytes, generictype, ::core::mem::transmute(virtualclock.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TxfReadMetadataInfo<P0>(filehandle: P0, txffileid: *mut TXF_ID, lastlsn: *mut CLS_LSN, transactionstate: *mut u32, lockingtransaction: *mut ::windows::core::GUID) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "txfw32.dll""system" fn TxfReadMetadataInfo ( filehandle : super::super::Foundation:: HANDLE , txffileid : *mut TXF_ID , lastlsn : *mut CLS_LSN , transactionstate : *mut u32 , lockingtransaction : *mut ::windows::core::GUID ) -> super::super::Foundation:: BOOL );
    TxfReadMetadataInfo(filehandle.into_param().abi(), txffileid, lastlsn, transactionstate, lockingtransaction)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn TxfSetThreadMiniVersionForCreate(miniversion: u16) {
    ::windows_targets::link ! ( "txfw32.dll""system" fn TxfSetThreadMiniVersionForCreate ( miniversion : u16 ) -> ( ) );
    TxfSetThreadMiniVersionForCreate(miniversion)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnlockFile<P0>(hfile: P0, dwfileoffsetlow: u32, dwfileoffsethigh: u32, nnumberofbytestounlocklow: u32, nnumberofbytestounlockhigh: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn UnlockFile ( hfile : super::super::Foundation:: HANDLE , dwfileoffsetlow : u32 , dwfileoffsethigh : u32 , nnumberofbytestounlocklow : u32 , nnumberofbytestounlockhigh : u32 ) -> super::super::Foundation:: BOOL );
    UnlockFile(hfile.into_param().abi(), dwfileoffsetlow, dwfileoffsethigh, nnumberofbytestounlocklow, nnumberofbytestounlockhigh)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn UnlockFileEx<P0>(hfile: P0, dwreserved: u32, nnumberofbytestounlocklow: u32, nnumberofbytestounlockhigh: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn UnlockFileEx ( hfile : super::super::Foundation:: HANDLE , dwreserved : u32 , nnumberofbytestounlocklow : u32 , nnumberofbytestounlockhigh : u32 , lpoverlapped : *mut super::super::System::IO:: OVERLAPPED ) -> super::super::Foundation:: BOOL );
    UnlockFileEx(hfile.into_param().abi(), dwreserved, nnumberofbytestounlocklow, nnumberofbytestounlockhigh, lpoverlapped)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ValidateLog<P0>(pszlogfilename: P0, psalogfile: *mut super::super::Security::SECURITY_ATTRIBUTES, pinfobuffer: *mut CLS_INFORMATION, pcbbuffer: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "clfsw32.dll""system" fn ValidateLog ( pszlogfilename : ::windows::core::PCWSTR , psalogfile : *mut super::super::Security:: SECURITY_ATTRIBUTES , pinfobuffer : *mut CLS_INFORMATION , pcbbuffer : *mut u32 ) -> super::super::Foundation:: BOOL );
    ValidateLog(pszlogfilename.into_param().abi(), psalogfile, pinfobuffer, pcbbuffer)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn VerFindFileA<P0, P1, P2>(uflags: VER_FIND_FILE_FLAGS, szfilename: P0, szwindir: P1, szappdir: P2, szcurdir: ::windows::core::PSTR, pucurdirlen: *mut u32, szdestdir: ::windows::core::PSTR, pudestdirlen: *mut u32) -> VER_FIND_FILE_STATUS
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "version.dll""system" fn VerFindFileA ( uflags : VER_FIND_FILE_FLAGS , szfilename : ::windows::core::PCSTR , szwindir : ::windows::core::PCSTR , szappdir : ::windows::core::PCSTR , szcurdir : ::windows::core::PSTR , pucurdirlen : *mut u32 , szdestdir : ::windows::core::PSTR , pudestdirlen : *mut u32 ) -> VER_FIND_FILE_STATUS );
    VerFindFileA(uflags, szfilename.into_param().abi(), szwindir.into_param().abi(), szappdir.into_param().abi(), ::core::mem::transmute(szcurdir), pucurdirlen, ::core::mem::transmute(szdestdir), pudestdirlen)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn VerFindFileW<P0, P1, P2>(uflags: VER_FIND_FILE_FLAGS, szfilename: P0, szwindir: P1, szappdir: P2, szcurdir: ::windows::core::PWSTR, pucurdirlen: *mut u32, szdestdir: ::windows::core::PWSTR, pudestdirlen: *mut u32) -> VER_FIND_FILE_STATUS
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "version.dll""system" fn VerFindFileW ( uflags : VER_FIND_FILE_FLAGS , szfilename : ::windows::core::PCWSTR , szwindir : ::windows::core::PCWSTR , szappdir : ::windows::core::PCWSTR , szcurdir : ::windows::core::PWSTR , pucurdirlen : *mut u32 , szdestdir : ::windows::core::PWSTR , pudestdirlen : *mut u32 ) -> VER_FIND_FILE_STATUS );
    VerFindFileW(uflags, szfilename.into_param().abi(), szwindir.into_param().abi(), szappdir.into_param().abi(), ::core::mem::transmute(szcurdir), pucurdirlen, ::core::mem::transmute(szdestdir), pudestdirlen)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn VerInstallFileA<P0, P1, P2, P3, P4>(uflags: VER_INSTALL_FILE_FLAGS, szsrcfilename: P0, szdestfilename: P1, szsrcdir: P2, szdestdir: P3, szcurdir: P4, sztmpfile: ::windows::core::PSTR, putmpfilelen: *mut u32) -> VER_INSTALL_FILE_STATUS
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P4: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "version.dll""system" fn VerInstallFileA ( uflags : VER_INSTALL_FILE_FLAGS , szsrcfilename : ::windows::core::PCSTR , szdestfilename : ::windows::core::PCSTR , szsrcdir : ::windows::core::PCSTR , szdestdir : ::windows::core::PCSTR , szcurdir : ::windows::core::PCSTR , sztmpfile : ::windows::core::PSTR , putmpfilelen : *mut u32 ) -> VER_INSTALL_FILE_STATUS );
    VerInstallFileA(uflags, szsrcfilename.into_param().abi(), szdestfilename.into_param().abi(), szsrcdir.into_param().abi(), szdestdir.into_param().abi(), szcurdir.into_param().abi(), ::core::mem::transmute(sztmpfile), putmpfilelen)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn VerInstallFileW<P0, P1, P2, P3, P4>(uflags: VER_INSTALL_FILE_FLAGS, szsrcfilename: P0, szdestfilename: P1, szsrcdir: P2, szdestdir: P3, szcurdir: P4, sztmpfile: ::windows::core::PWSTR, putmpfilelen: *mut u32) -> VER_INSTALL_FILE_STATUS
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P4: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "version.dll""system" fn VerInstallFileW ( uflags : VER_INSTALL_FILE_FLAGS , szsrcfilename : ::windows::core::PCWSTR , szdestfilename : ::windows::core::PCWSTR , szsrcdir : ::windows::core::PCWSTR , szdestdir : ::windows::core::PCWSTR , szcurdir : ::windows::core::PCWSTR , sztmpfile : ::windows::core::PWSTR , putmpfilelen : *mut u32 ) -> VER_INSTALL_FILE_STATUS );
    VerInstallFileW(uflags, szsrcfilename.into_param().abi(), szdestfilename.into_param().abi(), szsrcdir.into_param().abi(), szdestdir.into_param().abi(), szcurdir.into_param().abi(), ::core::mem::transmute(sztmpfile), putmpfilelen)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn VerLanguageNameA(wlang: u32, szlang: &mut [u8]) -> u32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn VerLanguageNameA ( wlang : u32 , szlang : ::windows::core::PSTR , cchlang : u32 ) -> u32 );
    VerLanguageNameA(wlang, ::core::mem::transmute(szlang.as_ptr()), szlang.len() as _)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn VerLanguageNameW(wlang: u32, szlang: &mut [u16]) -> u32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn VerLanguageNameW ( wlang : u32 , szlang : ::windows::core::PWSTR , cchlang : u32 ) -> u32 );
    VerLanguageNameW(wlang, ::core::mem::transmute(szlang.as_ptr()), szlang.len() as _)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VerQueryValueA<P0>(pblock: *const ::core::ffi::c_void, lpsubblock: P0, lplpbuffer: *mut *mut ::core::ffi::c_void, pulen: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "version.dll""system" fn VerQueryValueA ( pblock : *const ::core::ffi::c_void , lpsubblock : ::windows::core::PCSTR , lplpbuffer : *mut *mut ::core::ffi::c_void , pulen : *mut u32 ) -> super::super::Foundation:: BOOL );
    VerQueryValueA(pblock, lpsubblock.into_param().abi(), lplpbuffer, pulen)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VerQueryValueW<P0>(pblock: *const ::core::ffi::c_void, lpsubblock: P0, lplpbuffer: *mut *mut ::core::ffi::c_void, pulen: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "version.dll""system" fn VerQueryValueW ( pblock : *const ::core::ffi::c_void , lpsubblock : ::windows::core::PCWSTR , lplpbuffer : *mut *mut ::core::ffi::c_void , pulen : *mut u32 ) -> super::super::Foundation:: BOOL );
    VerQueryValueW(pblock, lpsubblock.into_param().abi(), lplpbuffer, pulen)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WofEnumEntries<P0>(volumename: P0, provider: u32, enumproc: WofEnumEntryProc, userdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wofutil.dll""system" fn WofEnumEntries ( volumename : ::windows::core::PCWSTR , provider : u32 , enumproc : WofEnumEntryProc , userdata : *const ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    WofEnumEntries(volumename.into_param().abi(), provider, enumproc, ::core::mem::transmute(userdata.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WofFileEnumFiles<P0>(volumename: P0, algorithm: u32, enumproc: WofEnumFilesProc, userdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wofutil.dll""system" fn WofFileEnumFiles ( volumename : ::windows::core::PCWSTR , algorithm : u32 , enumproc : WofEnumFilesProc , userdata : *const ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    WofFileEnumFiles(volumename.into_param().abi(), algorithm, enumproc, ::core::mem::transmute(userdata.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WofGetDriverVersion<P0>(fileorvolumehandle: P0, provider: u32) -> ::windows::core::Result<u32>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wofutil.dll""system" fn WofGetDriverVersion ( fileorvolumehandle : super::super::Foundation:: HANDLE , provider : u32 , wofversion : *mut u32 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<u32>();
    WofGetDriverVersion(fileorvolumehandle.into_param().abi(), provider, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WofIsExternalFile<P0>(filepath: P0, isexternalfile: ::core::option::Option<*mut super::super::Foundation::BOOL>, provider: ::core::option::Option<*mut u32>, externalfileinfo: ::core::option::Option<*mut ::core::ffi::c_void>, bufferlength: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wofutil.dll""system" fn WofIsExternalFile ( filepath : ::windows::core::PCWSTR , isexternalfile : *mut super::super::Foundation:: BOOL , provider : *mut u32 , externalfileinfo : *mut ::core::ffi::c_void , bufferlength : *mut u32 ) -> ::windows::core::HRESULT );
    WofIsExternalFile(filepath.into_param().abi(), ::core::mem::transmute(isexternalfile.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(provider.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(externalfileinfo.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(bufferlength.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WofSetFileDataLocation<P0>(filehandle: P0, provider: u32, externalfileinfo: *const ::core::ffi::c_void, length: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wofutil.dll""system" fn WofSetFileDataLocation ( filehandle : super::super::Foundation:: HANDLE , provider : u32 , externalfileinfo : *const ::core::ffi::c_void , length : u32 ) -> ::windows::core::HRESULT );
    WofSetFileDataLocation(filehandle.into_param().abi(), provider, externalfileinfo, length).ok()
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WofShouldCompressBinaries<P0>(volume: P0, algorithm: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wofutil.dll""system" fn WofShouldCompressBinaries ( volume : ::windows::core::PCWSTR , algorithm : *mut u32 ) -> super::super::Foundation:: BOOL );
    WofShouldCompressBinaries(volume.into_param().abi(), algorithm)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn WofWimAddEntry<P0, P1>(volumename: P0, wimpath: P1, wimtype: u32, wimindex: u32) -> ::windows::core::Result<i64>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wofutil.dll""system" fn WofWimAddEntry ( volumename : ::windows::core::PCWSTR , wimpath : ::windows::core::PCWSTR , wimtype : u32 , wimindex : u32 , datasourceid : *mut i64 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<i64>();
    WofWimAddEntry(volumename.into_param().abi(), wimpath.into_param().abi(), wimtype, wimindex, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WofWimEnumFiles<P0>(volumename: P0, datasourceid: i64, enumproc: WofEnumFilesProc, userdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wofutil.dll""system" fn WofWimEnumFiles ( volumename : ::windows::core::PCWSTR , datasourceid : i64 , enumproc : WofEnumFilesProc , userdata : *const ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    WofWimEnumFiles(volumename.into_param().abi(), datasourceid, enumproc, ::core::mem::transmute(userdata.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn WofWimRemoveEntry<P0>(volumename: P0, datasourceid: i64) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wofutil.dll""system" fn WofWimRemoveEntry ( volumename : ::windows::core::PCWSTR , datasourceid : i64 ) -> ::windows::core::HRESULT );
    WofWimRemoveEntry(volumename.into_param().abi(), datasourceid).ok()
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn WofWimSuspendEntry<P0>(volumename: P0, datasourceid: i64) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wofutil.dll""system" fn WofWimSuspendEntry ( volumename : ::windows::core::PCWSTR , datasourceid : i64 ) -> ::windows::core::HRESULT );
    WofWimSuspendEntry(volumename.into_param().abi(), datasourceid).ok()
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn WofWimUpdateEntry<P0, P1>(volumename: P0, datasourceid: i64, newwimpath: P1) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wofutil.dll""system" fn WofWimUpdateEntry ( volumename : ::windows::core::PCWSTR , datasourceid : i64 , newwimpath : ::windows::core::PCWSTR ) -> ::windows::core::HRESULT );
    WofWimUpdateEntry(volumename.into_param().abi(), datasourceid, newwimpath.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Wow64DisableWow64FsRedirection(oldvalue: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn Wow64DisableWow64FsRedirection ( oldvalue : *mut *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    Wow64DisableWow64FsRedirection(oldvalue)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Wow64EnableWow64FsRedirection<P0>(wow64fsenableredirection: P0) -> super::super::Foundation::BOOLEAN
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOLEAN>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn Wow64EnableWow64FsRedirection ( wow64fsenableredirection : super::super::Foundation:: BOOLEAN ) -> super::super::Foundation:: BOOLEAN );
    Wow64EnableWow64FsRedirection(wow64fsenableredirection.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Wow64RevertWow64FsRedirection(olvalue: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn Wow64RevertWow64FsRedirection ( olvalue : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    Wow64RevertWow64FsRedirection(olvalue)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[inline]
pub unsafe fn WriteEncryptedFileRaw(pfimportcallback: PFE_IMPORT_FUNC, pvcallbackcontext: ::core::option::Option<*const ::core::ffi::c_void>, pvcontext: *const ::core::ffi::c_void) -> u32 {
    ::windows_targets::link ! ( "advapi32.dll""system" fn WriteEncryptedFileRaw ( pfimportcallback : PFE_IMPORT_FUNC , pvcallbackcontext : *const ::core::ffi::c_void , pvcontext : *const ::core::ffi::c_void ) -> u32 );
    WriteEncryptedFileRaw(pfimportcallback, ::core::mem::transmute(pvcallbackcontext.unwrap_or(::std::ptr::null())), pvcontext)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn WriteFile<P0>(hfile: P0, lpbuffer: ::core::option::Option<&[u8]>, lpnumberofbyteswritten: ::core::option::Option<*mut u32>, lpoverlapped: ::core::option::Option<*mut super::super::System::IO::OVERLAPPED>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn WriteFile ( hfile : super::super::Foundation:: HANDLE , lpbuffer : *const u8 , nnumberofbytestowrite : u32 , lpnumberofbyteswritten : *mut u32 , lpoverlapped : *mut super::super::System::IO:: OVERLAPPED ) -> super::super::Foundation:: BOOL );
    WriteFile(hfile.into_param().abi(), ::core::mem::transmute(lpbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpbuffer.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lpnumberofbyteswritten.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpoverlapped.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn WriteFileEx<P0>(hfile: P0, lpbuffer: ::core::option::Option<&[u8]>, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: super::super::System::IO::LPOVERLAPPED_COMPLETION_ROUTINE) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn WriteFileEx ( hfile : super::super::Foundation:: HANDLE , lpbuffer : *const u8 , nnumberofbytestowrite : u32 , lpoverlapped : *mut super::super::System::IO:: OVERLAPPED , lpcompletionroutine : super::super::System::IO:: LPOVERLAPPED_COMPLETION_ROUTINE ) -> super::super::Foundation:: BOOL );
    WriteFileEx(hfile.into_param().abi(), ::core::mem::transmute(lpbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpbuffer.as_deref().map_or(0, |slice| slice.len() as _), lpoverlapped, lpcompletionroutine)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn WriteFileGather<P0>(hfile: P0, asegmentarray: *const FILE_SEGMENT_ELEMENT, nnumberofbytestowrite: u32, lpreserved: ::core::option::Option<*const u32>, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn WriteFileGather ( hfile : super::super::Foundation:: HANDLE , asegmentarray : *const FILE_SEGMENT_ELEMENT , nnumberofbytestowrite : u32 , lpreserved : *const u32 , lpoverlapped : *mut super::super::System::IO:: OVERLAPPED ) -> super::super::Foundation:: BOOL );
    WriteFileGather(hfile.into_param().abi(), asegmentarray, nnumberofbytestowrite, ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())), lpoverlapped)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn WriteLogRestartArea(pvmarshal: *mut ::core::ffi::c_void, pvrestartbuffer: *mut ::core::ffi::c_void, cbrestartbuffer: u32, plsnbase: *mut CLS_LSN, fflags: CLFS_FLAG, pcbwritten: *mut u32, plsnnext: *mut CLS_LSN, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "clfsw32.dll""system" fn WriteLogRestartArea ( pvmarshal : *mut ::core::ffi::c_void , pvrestartbuffer : *mut ::core::ffi::c_void , cbrestartbuffer : u32 , plsnbase : *mut CLS_LSN , fflags : CLFS_FLAG , pcbwritten : *mut u32 , plsnnext : *mut CLS_LSN , poverlapped : *mut super::super::System::IO:: OVERLAPPED ) -> super::super::Foundation:: BOOL );
    WriteLogRestartArea(pvmarshal, pvrestartbuffer, cbrestartbuffer, plsnbase, fflags, pcbwritten, plsnnext, poverlapped)
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WriteTapemark<P0, P1>(hdevice: P0, dwtapemarktype: TAPEMARK_TYPE, dwtapemarkcount: u32, bimmediate: P1) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn WriteTapemark ( hdevice : super::super::Foundation:: HANDLE , dwtapemarktype : TAPEMARK_TYPE , dwtapemarkcount : u32 , bimmediate : super::super::Foundation:: BOOL ) -> u32 );
    WriteTapemark(hdevice.into_param().abi(), dwtapemarktype, dwtapemarkcount, bimmediate.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IDiskQuotaControl(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDiskQuotaControl {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumConnectionPoints(&self) -> ::windows::core::Result<super::super::System::Com::IEnumConnectionPoints> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IEnumConnectionPoints>();
        (::windows::core::Interface::vtable(self).base__.EnumConnectionPoints)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FindConnectionPoint(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::System::Com::IConnectionPoint> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IConnectionPoint>();
        (::windows::core::Interface::vtable(self).base__.FindConnectionPoint)(::windows::core::Interface::as_raw(self), riid, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<P0, P1>(&self, pszpath: P0, breadwrite: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), pszpath.into_param().abi(), breadwrite.into_param().abi()).ok()
    }
    pub unsafe fn SetQuotaState(&self, dwstate: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetQuotaState)(::windows::core::Interface::as_raw(self), dwstate).ok()
    }
    pub unsafe fn GetQuotaState(&self, pdwstate: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetQuotaState)(::windows::core::Interface::as_raw(self), pdwstate).ok()
    }
    pub unsafe fn SetQuotaLogFlags(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetQuotaLogFlags)(::windows::core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn GetQuotaLogFlags(&self, pdwflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetQuotaLogFlags)(::windows::core::Interface::as_raw(self), pdwflags).ok()
    }
    pub unsafe fn SetDefaultQuotaThreshold(&self, llthreshold: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDefaultQuotaThreshold)(::windows::core::Interface::as_raw(self), llthreshold).ok()
    }
    pub unsafe fn GetDefaultQuotaThreshold(&self, pllthreshold: *mut i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDefaultQuotaThreshold)(::windows::core::Interface::as_raw(self), pllthreshold).ok()
    }
    pub unsafe fn GetDefaultQuotaThresholdText<P0>(&self, psztext: P0, cchtext: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetDefaultQuotaThresholdText)(::windows::core::Interface::as_raw(self), psztext.into_param().abi(), cchtext).ok()
    }
    pub unsafe fn SetDefaultQuotaLimit(&self, lllimit: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDefaultQuotaLimit)(::windows::core::Interface::as_raw(self), lllimit).ok()
    }
    pub unsafe fn GetDefaultQuotaLimit(&self, plllimit: *mut i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDefaultQuotaLimit)(::windows::core::Interface::as_raw(self), plllimit).ok()
    }
    pub unsafe fn GetDefaultQuotaLimitText<P0>(&self, psztext: P0, cchtext: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetDefaultQuotaLimitText)(::windows::core::Interface::as_raw(self), psztext.into_param().abi(), cchtext).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddUserSid<P0>(&self, pusersid: P0, fnameresolution: DISKQUOTA_USERNAME_RESOLVE) -> ::windows::core::Result<IDiskQuotaUser>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::PSID>,
    {
        let mut result__ = ::windows::core::zeroed::<IDiskQuotaUser>();
        (::windows::core::Interface::vtable(self).AddUserSid)(::windows::core::Interface::as_raw(self), pusersid.into_param().abi(), fnameresolution, &mut result__).from_abi(result__)
    }
    pub unsafe fn AddUserName<P0>(&self, pszlogonname: P0, fnameresolution: DISKQUOTA_USERNAME_RESOLVE) -> ::windows::core::Result<IDiskQuotaUser>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IDiskQuotaUser>();
        (::windows::core::Interface::vtable(self).AddUserName)(::windows::core::Interface::as_raw(self), pszlogonname.into_param().abi(), fnameresolution, &mut result__).from_abi(result__)
    }
    pub unsafe fn DeleteUser<P0>(&self, puser: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDiskQuotaUser>,
    {
        (::windows::core::Interface::vtable(self).DeleteUser)(::windows::core::Interface::as_raw(self), puser.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindUserSid<P0>(&self, pusersid: P0, fnameresolution: DISKQUOTA_USERNAME_RESOLVE) -> ::windows::core::Result<IDiskQuotaUser>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::PSID>,
    {
        let mut result__ = ::windows::core::zeroed::<IDiskQuotaUser>();
        (::windows::core::Interface::vtable(self).FindUserSid)(::windows::core::Interface::as_raw(self), pusersid.into_param().abi(), fnameresolution, &mut result__).from_abi(result__)
    }
    pub unsafe fn FindUserName<P0>(&self, pszlogonname: P0) -> ::windows::core::Result<IDiskQuotaUser>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IDiskQuotaUser>();
        (::windows::core::Interface::vtable(self).FindUserName)(::windows::core::Interface::as_raw(self), pszlogonname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateEnumUsers(&self, rgpusersids: *mut super::super::Foundation::PSID, cpsids: u32, fnameresolution: DISKQUOTA_USERNAME_RESOLVE, ppenum: *mut ::core::option::Option<IEnumDiskQuotaUsers>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateEnumUsers)(::windows::core::Interface::as_raw(self), rgpusersids, cpsids, fnameresolution, ::core::mem::transmute(ppenum)).ok()
    }
    pub unsafe fn CreateUserBatch(&self) -> ::windows::core::Result<IDiskQuotaUserBatch> {
        let mut result__ = ::windows::core::zeroed::<IDiskQuotaUserBatch>();
        (::windows::core::Interface::vtable(self).CreateUserBatch)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn InvalidateSidNameCache(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InvalidateSidNameCache)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GiveUserNameResolutionPriority<P0>(&self, puser: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDiskQuotaUser>,
    {
        (::windows::core::Interface::vtable(self).GiveUserNameResolutionPriority)(::windows::core::Interface::as_raw(self), puser.into_param().abi()).ok()
    }
    pub unsafe fn ShutdownNameResolution(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ShutdownNameResolution)(::windows::core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IDiskQuotaControl, ::windows::core::IUnknown, super::super::System::Com::IConnectionPointContainer);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IDiskQuotaControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IDiskQuotaControl {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IDiskQuotaControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDiskQuotaControl").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IDiskQuotaControl {
    type Vtable = IDiskQuotaControl_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IDiskQuotaControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IDiskQuotaControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7988b572_ec89_11cf_9c00_00aa00a14f56);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiskQuotaControl_Vtbl {
    pub base__: super::super::System::Com::IConnectionPointContainer_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, breadwrite: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    pub SetQuotaState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwstate: u32) -> ::windows::core::HRESULT,
    pub GetQuotaState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT,
    pub SetQuotaLogFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    pub GetQuotaLogFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT,
    pub SetDefaultQuotaThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, llthreshold: i64) -> ::windows::core::HRESULT,
    pub GetDefaultQuotaThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pllthreshold: *mut i64) -> ::windows::core::HRESULT,
    pub GetDefaultQuotaThresholdText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztext: ::windows::core::PCWSTR, cchtext: u32) -> ::windows::core::HRESULT,
    pub SetDefaultQuotaLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lllimit: i64) -> ::windows::core::HRESULT,
    pub GetDefaultQuotaLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plllimit: *mut i64) -> ::windows::core::HRESULT,
    pub GetDefaultQuotaLimitText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztext: ::windows::core::PCWSTR, cchtext: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddUserSid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pusersid: super::super::Foundation::PSID, fnameresolution: DISKQUOTA_USERNAME_RESOLVE, ppuser: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddUserSid: usize,
    pub AddUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszlogonname: ::windows::core::PCWSTR, fnameresolution: DISKQUOTA_USERNAME_RESOLVE, ppuser: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DeleteUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puser: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FindUserSid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pusersid: super::super::Foundation::PSID, fnameresolution: DISKQUOTA_USERNAME_RESOLVE, ppuser: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FindUserSid: usize,
    pub FindUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszlogonname: ::windows::core::PCWSTR, ppuser: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateEnumUsers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rgpusersids: *mut super::super::Foundation::PSID, cpsids: u32, fnameresolution: DISKQUOTA_USERNAME_RESOLVE, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateEnumUsers: usize,
    pub CreateUserBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbatch: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InvalidateSidNameCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GiveUserNameResolutionPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puser: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ShutdownNameResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
pub struct IDiskQuotaEvents(::windows::core::IUnknown);
impl IDiskQuotaEvents {
    pub unsafe fn OnUserNameChanged<P0>(&self, puser: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDiskQuotaUser>,
    {
        (::windows::core::Interface::vtable(self).OnUserNameChanged)(::windows::core::Interface::as_raw(self), puser.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IDiskQuotaEvents, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDiskQuotaEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDiskQuotaEvents {}
impl ::core::fmt::Debug for IDiskQuotaEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDiskQuotaEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDiskQuotaEvents {
    type Vtable = IDiskQuotaEvents_Vtbl;
}
impl ::core::clone::Clone for IDiskQuotaEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDiskQuotaEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7988b579_ec89_11cf_9c00_00aa00a14f56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiskQuotaEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnUserNameChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puser: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
pub struct IDiskQuotaUser(::windows::core::IUnknown);
impl IDiskQuotaUser {
    pub unsafe fn GetID(&self, pulid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetID)(::windows::core::Interface::as_raw(self), pulid).ok()
    }
    pub unsafe fn GetName<P0, P1, P2>(&self, pszaccountcontainer: P0, cchaccountcontainer: u32, pszlogonname: P1, cchlogonname: u32, pszdisplayname: P2, cchdisplayname: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetName)(::windows::core::Interface::as_raw(self), pszaccountcontainer.into_param().abi(), cchaccountcontainer, pszlogonname.into_param().abi(), cchlogonname, pszdisplayname.into_param().abi(), cchdisplayname).ok()
    }
    pub unsafe fn GetSidLength(&self, pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSidLength)(::windows::core::Interface::as_raw(self), pdwlength).ok()
    }
    pub unsafe fn GetSid(&self, pbsidbuffer: *mut u8, cbsidbuffer: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSid)(::windows::core::Interface::as_raw(self), pbsidbuffer, cbsidbuffer).ok()
    }
    pub unsafe fn GetQuotaThreshold(&self, pllthreshold: *mut i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetQuotaThreshold)(::windows::core::Interface::as_raw(self), pllthreshold).ok()
    }
    pub unsafe fn GetQuotaThresholdText<P0>(&self, psztext: P0, cchtext: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetQuotaThresholdText)(::windows::core::Interface::as_raw(self), psztext.into_param().abi(), cchtext).ok()
    }
    pub unsafe fn GetQuotaLimit(&self, plllimit: *mut i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetQuotaLimit)(::windows::core::Interface::as_raw(self), plllimit).ok()
    }
    pub unsafe fn GetQuotaLimitText<P0>(&self, psztext: P0, cchtext: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetQuotaLimitText)(::windows::core::Interface::as_raw(self), psztext.into_param().abi(), cchtext).ok()
    }
    pub unsafe fn GetQuotaUsed(&self, pllused: *mut i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetQuotaUsed)(::windows::core::Interface::as_raw(self), pllused).ok()
    }
    pub unsafe fn GetQuotaUsedText<P0>(&self, psztext: P0, cchtext: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetQuotaUsedText)(::windows::core::Interface::as_raw(self), psztext.into_param().abi(), cchtext).ok()
    }
    pub unsafe fn GetQuotaInformation(&self, pbquotainfo: *mut ::core::ffi::c_void, cbquotainfo: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetQuotaInformation)(::windows::core::Interface::as_raw(self), pbquotainfo, cbquotainfo).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetQuotaThreshold<P0>(&self, llthreshold: i64, fwritethrough: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetQuotaThreshold)(::windows::core::Interface::as_raw(self), llthreshold, fwritethrough.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetQuotaLimit<P0>(&self, lllimit: i64, fwritethrough: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetQuotaLimit)(::windows::core::Interface::as_raw(self), lllimit, fwritethrough.into_param().abi()).ok()
    }
    pub unsafe fn Invalidate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Invalidate)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetAccountStatus(&self, pdwstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAccountStatus)(::windows::core::Interface::as_raw(self), pdwstatus).ok()
    }
}
::windows::imp::interface_hierarchy!(IDiskQuotaUser, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDiskQuotaUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDiskQuotaUser {}
impl ::core::fmt::Debug for IDiskQuotaUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDiskQuotaUser").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDiskQuotaUser {
    type Vtable = IDiskQuotaUser_Vtbl;
}
impl ::core::clone::Clone for IDiskQuotaUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDiskQuotaUser {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7988b574_ec89_11cf_9c00_00aa00a14f56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiskQuotaUser_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulid: *mut u32) -> ::windows::core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszaccountcontainer: ::windows::core::PCWSTR, cchaccountcontainer: u32, pszlogonname: ::windows::core::PCWSTR, cchlogonname: u32, pszdisplayname: ::windows::core::PCWSTR, cchdisplayname: u32) -> ::windows::core::HRESULT,
    pub GetSidLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT,
    pub GetSid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsidbuffer: *mut u8, cbsidbuffer: u32) -> ::windows::core::HRESULT,
    pub GetQuotaThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pllthreshold: *mut i64) -> ::windows::core::HRESULT,
    pub GetQuotaThresholdText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztext: ::windows::core::PCWSTR, cchtext: u32) -> ::windows::core::HRESULT,
    pub GetQuotaLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plllimit: *mut i64) -> ::windows::core::HRESULT,
    pub GetQuotaLimitText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztext: ::windows::core::PCWSTR, cchtext: u32) -> ::windows::core::HRESULT,
    pub GetQuotaUsed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pllused: *mut i64) -> ::windows::core::HRESULT,
    pub GetQuotaUsedText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztext: ::windows::core::PCWSTR, cchtext: u32) -> ::windows::core::HRESULT,
    pub GetQuotaInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbquotainfo: *mut ::core::ffi::c_void, cbquotainfo: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetQuotaThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, llthreshold: i64, fwritethrough: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetQuotaThreshold: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetQuotaLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lllimit: i64, fwritethrough: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetQuotaLimit: usize,
    pub Invalidate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetAccountStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
pub struct IDiskQuotaUserBatch(::windows::core::IUnknown);
impl IDiskQuotaUserBatch {
    pub unsafe fn Add<P0>(&self, puser: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDiskQuotaUser>,
    {
        (::windows::core::Interface::vtable(self).Add)(::windows::core::Interface::as_raw(self), puser.into_param().abi()).ok()
    }
    pub unsafe fn Remove<P0>(&self, puser: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDiskQuotaUser>,
    {
        (::windows::core::Interface::vtable(self).Remove)(::windows::core::Interface::as_raw(self), puser.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAll(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAll)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn FlushToDisk(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FlushToDisk)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IDiskQuotaUserBatch, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDiskQuotaUserBatch {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDiskQuotaUserBatch {}
impl ::core::fmt::Debug for IDiskQuotaUserBatch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDiskQuotaUserBatch").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDiskQuotaUserBatch {
    type Vtable = IDiskQuotaUserBatch_Vtbl;
}
impl ::core::clone::Clone for IDiskQuotaUserBatch {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDiskQuotaUserBatch {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7988b576_ec89_11cf_9c00_00aa00a14f56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiskQuotaUserBatch_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puser: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puser: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FlushToDisk: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
pub struct IEnumDiskQuotaUsers(::windows::core::IUnknown);
impl IEnumDiskQuotaUsers {
    pub unsafe fn Next(&self, cusers: u32, rgusers: *mut ::core::option::Option<IDiskQuotaUser>, pcusersfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), cusers, ::core::mem::transmute(rgusers), pcusersfetched).ok()
    }
    pub unsafe fn Skip(&self, cusers: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), cusers).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumDiskQuotaUsers> {
        let mut result__ = ::windows::core::zeroed::<IEnumDiskQuotaUsers>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IEnumDiskQuotaUsers, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumDiskQuotaUsers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumDiskQuotaUsers {}
impl ::core::fmt::Debug for IEnumDiskQuotaUsers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumDiskQuotaUsers").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumDiskQuotaUsers {
    type Vtable = IEnumDiskQuotaUsers_Vtbl;
}
impl ::core::clone::Clone for IEnumDiskQuotaUsers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumDiskQuotaUsers {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7988b577_ec89_11cf_9c00_00aa00a14f56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDiskQuotaUsers_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cusers: u32, rgusers: *mut *mut ::core::ffi::c_void, pcusersfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cusers: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_BASELOG_EXTENSION: ::windows::core::PCWSTR = ::windows::core::w!(".blf");
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_CONTAINER_RELATIVE_PREFIX: ::windows::core::PCWSTR = ::windows::core::w!("%BLF%\\");
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_CONTAINER_STREAM_PREFIX: ::windows::core::PCWSTR = ::windows::core::w!("%BLF%:");
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_FLAG_FILTER_INTERMEDIATE_LEVEL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_FLAG_FILTER_TOP_LEVEL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_FLAG_HIDDEN_SYSTEM_LOG: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_FLAG_IGNORE_SHARE_ACCESS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_FLAG_MINIFILTER_LEVEL: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_FLAG_NON_REENTRANT_FILTER: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_FLAG_READ_IN_PROGRESS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_FLAG_REENTRANT_FILE_SYSTEM: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_FLAG_REENTRANT_FILTER: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_MARSHALLING_FLAG_DISABLE_BUFF_INIT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_MARSHALLING_FLAG_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_MAX_CONTAINER_INFO: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_MGMT_CLIENT_REGISTRATION_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_MGMT_POLICY_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_SCAN_BACKWARD: u8 = 4u8;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_SCAN_BUFFERED: u8 = 32u8;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_SCAN_CLOSE: u8 = 8u8;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_SCAN_FORWARD: u8 = 2u8;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_SCAN_INIT: u8 = 1u8;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_SCAN_INITIALIZED: u8 = 16u8;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLSID_DiskQuotaControl: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7988b571_ec89_11cf_9c00_00aa00a14f56);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CRM_PROTOCOL_DYNAMIC_MARSHAL_INFO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CRM_PROTOCOL_EXPLICIT_MARSHAL_ONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CRM_PROTOCOL_MAXIMUM_OPTION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CSC_CACHE_AUTO_REINT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CSC_CACHE_MANUAL_REINT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CSC_CACHE_NONE: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CSC_CACHE_VDO: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CSC_MASK: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CSC_MASK_EXT: u32 = 8240u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CSV_BLOCK_AND_FILE_CACHE_CALLBACK_VERSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CSV_BLOCK_CACHE_CALLBACK_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsClientRecord: u8 = 3u8;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsContainerActive: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsContainerActivePendingDelete: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsContainerInactive: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsContainerInitializing: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsContainerPendingArchive: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsContainerPendingArchiveAndDelete: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsDataRecord: u8 = 1u8;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsNullRecord: u8 = 0u8;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsRestartRecord: u8 = 2u8;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClsContainerActive: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClsContainerActivePendingDelete: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClsContainerInactive: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClsContainerInitializing: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClsContainerPendingArchive: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClsContainerPendingArchiveAndDelete: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DISKQUOTA_FILESTATE_INCOMPLETE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DISKQUOTA_FILESTATE_MASK: u32 = 768u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DISKQUOTA_FILESTATE_REBUILDING: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DISKQUOTA_LOGFLAG_USER_LIMIT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DISKQUOTA_LOGFLAG_USER_THRESHOLD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DISKQUOTA_STATE_DISABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DISKQUOTA_STATE_ENFORCE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DISKQUOTA_STATE_MASK: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DISKQUOTA_STATE_TRACK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DISKQUOTA_USER_ACCOUNT_DELETED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DISKQUOTA_USER_ACCOUNT_INVALID: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DISKQUOTA_USER_ACCOUNT_RESOLVED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DISKQUOTA_USER_ACCOUNT_UNAVAILABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DISKQUOTA_USER_ACCOUNT_UNKNOWN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DISKQUOTA_USER_ACCOUNT_UNRESOLVED: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const EA_CONTAINER_NAME: ::windows::core::PCSTR = ::windows::core::s!("ContainerName");
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const EA_CONTAINER_SIZE: ::windows::core::PCSTR = ::windows::core::s!("ContainerSize");
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const EFS_COMPATIBILITY_VERSION_NCRYPT_PROTECTOR: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const EFS_COMPATIBILITY_VERSION_PFILE_PROTECTOR: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const EFS_EFS_SUBVER_EFS_CERT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const EFS_METADATA_ADD_USER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const EFS_METADATA_GENERAL_OP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const EFS_METADATA_REMOVE_USER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const EFS_METADATA_REPLACE_USER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const EFS_PFILE_SUBVER_APPX: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const EFS_PFILE_SUBVER_RMS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const EFS_SUBVER_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ENLISTMENT_MAXIMUM_OPTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ENLISTMENT_OBJECT_PATH: ::windows::core::PCWSTR = ::windows::core::w!("\\Enlistment\\");
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ENLISTMENT_SUPERIOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_PROVIDER_COMPRESSION_LZX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_PROVIDER_COMPRESSION_XPRESS16K: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_PROVIDER_COMPRESSION_XPRESS4K: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_PROVIDER_COMPRESSION_XPRESS8K: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const INVALID_FILE_ATTRIBUTES: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const INVALID_FILE_SIZE: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const INVALID_SET_FILE_POINTER: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_ALLOCATE_BC_STREAM: u32 = 5685312u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_BASE: u32 = 86u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_BC_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_FREE_BC_STREAM: u32 = 5685316u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_GET_BC_PROPERTIES: u32 = 5652540u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_GET_CSVBLOCKCACHE_CALLBACK: u32 = 5685352u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_GET_GPT_ATTRIBUTES: u32 = 5636152u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_GET_VOLUME_DISK_EXTENTS: u32 = 5636096u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_IS_CLUSTERED: u32 = 5636144u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_IS_CSV: u32 = 5636192u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_IS_DYNAMIC: u32 = 5636168u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_IS_IO_CAPABLE: u32 = 5636116u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_IS_OFFLINE: u32 = 5636112u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_IS_PARTITION: u32 = 5636136u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_LOGICAL_TO_PHYSICAL: u32 = 5636128u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_OFFLINE: u32 = 5685260u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_ONLINE: u32 = 5685256u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_PHYSICAL_TO_LOGICAL: u32 = 5636132u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_POST_ONLINE: u32 = 5685348u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_PREPARE_FOR_CRITICAL_IO: u32 = 5685324u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_PREPARE_FOR_SHRINK: u32 = 5685340u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_QUERY_ALLOCATION_HINT: u32 = 5652562u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_QUERY_FAILOVER_SET: u32 = 5636120u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_QUERY_MINIMUM_SHRINK_SIZE: u32 = 5652568u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_QUERY_VOLUME_NUMBER: u32 = 5636124u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_READ_PLEX: u32 = 5652526u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_SET_GPT_ATTRIBUTES: u32 = 5636148u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_SUPPORTS_ONLINE_OFFLINE: u32 = 5636100u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_UPDATE_PROPERTIES: u32 = 5636180u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const KTM_MARSHAL_BLOB_VERSION_MAJOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const KTM_MARSHAL_BLOB_VERSION_MINOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const LOG_POLICY_OVERWRITE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const LOG_POLICY_PERSIST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const LZERROR_BADINHANDLE: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const LZERROR_BADOUTHANDLE: i32 = -2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const LZERROR_BADVALUE: i32 = -7i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const LZERROR_GLOBALLOC: i32 = -5i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const LZERROR_GLOBLOCK: i32 = -6i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const LZERROR_READ: i32 = -3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const LZERROR_UNKNOWNALG: i32 = -8i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const LZERROR_WRITE: i32 = -4i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const MAXIMUM_REPARSE_DATA_BUFFER_SIZE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const MAX_RESOURCEMANAGER_DESCRIPTION_LENGTH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const MAX_SID_SIZE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const MAX_TRANSACTION_DESCRIPTION_LENGTH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMSMLI_MAXAPPDESCR: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMSMLI_MAXIDSIZE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMSMLI_MAXTYPE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_APPLICATIONNAME_LENGTH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_BARCODE_LENGTH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_COMPUTERNAME_LENGTH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_DESCRIPTION_LENGTH: u32 = 127u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_DEVICENAME_LENGTH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_I1_MESSAGE_LENGTH: u32 = 127u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MAXATTR_LENGTH: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MAXATTR_NAMELEN: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MESSAGE_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OBJECTNAME_LENGTH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OMIDLABELID_LENGTH: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OMIDLABELINFO_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OMIDLABELTYPE_LENGTH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_POOLHIERARCHY_LENGTH: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PRODUCTNAME_LENGTH: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_REVISION_LENGTH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_SEQUENCE_LENGTH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_SERIALNUMBER_LENGTH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_USERNAME_LENGTH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_VENDORNAME_LENGTH: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PARTITION_BASIC_DATA_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xebd0a0a2_b9e5_4433_87c0_68b6b72699c7);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PARTITION_BSP_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57434f53_4df9_45b9_8e9e_2370f006457c);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PARTITION_CLUSTER_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb97dba9_0840_4bae_97f0_ffb9a327c7e1);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PARTITION_DPP_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57434f53_94cb_43f0_a533_d73c10cfa57d);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PARTITION_ENTRY_UNUSED_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000000);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PARTITION_LDM_DATA_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf9b60a0_1431_4f62_bc68_3311714a69ad);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PARTITION_LDM_METADATA_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5808c8aa_7e8f_42e0_85d2_e1e90434cfb3);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PARTITION_LEGACY_BL_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x424ca0e2_7cb2_4fb9_8143_c52a99398bc6);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PARTITION_LEGACY_BL_GUID_BACKUP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x424c3e6c_d79f_49cb_935d_36d71467a288);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PARTITION_MAIN_OS_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57434f53_8f45_405e_8a23_186d8a4330d3);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PARTITION_MSFT_RECOVERY_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde94bba4_06d1_4d40_a16a_bfd50179d6ac);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PARTITION_MSFT_RESERVED_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3c9e316_0b5c_4db8_817d_f92df00215ae);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PARTITION_MSFT_SNAPSHOT_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcaddebf1_4400_4de8_b103_12117dcf3ccf);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PARTITION_OS_DATA_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57434f53_23f2_44d5_a830_67bbdaa609f9);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PARTITION_PATCH_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8967a686_96aa_6aa8_9589_a84256541090);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PARTITION_PRE_INSTALLED_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57434f53_7fe0_4196_9b42_427b51643484);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PARTITION_SBL_CACHE_HDD_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03aaa829_ebfc_4e7e_aac9_c4d76c63b24b);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PARTITION_SBL_CACHE_SSD_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeeff8352_dd2a_44db_ae83_bee1cf7481dc);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PARTITION_SBL_CACHE_SSD_RESERVED_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcc0c7c1_55ad_4f17_9d43_4bc776e0117e);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PARTITION_SERVICING_FILES_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57434f53_432e_4014_ae4c_8deaa9c0006a);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PARTITION_SERVICING_METADATA_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57434f53_c691_4a05_bb4e_703dafd229ce);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PARTITION_SERVICING_RESERVE_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57434f53_4b81_460b_a319_ffb6fe136d14);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PARTITION_SERVICING_STAGING_ROOT_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57434f53_e84d_4e84_aaf3_ecbbbd04b9df);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PARTITION_SPACES_DATA_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7addcb4_dc34_4539_9a76_ebbd07be6f7e);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PARTITION_SPACES_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe75caf8f_f680_4cee_afa3_b001e56efc2d);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PARTITION_SYSTEM_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc12a7328_f81f_11d2_ba4b_00a0c93ec93b);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PARTITION_WINDOWS_SYSTEM_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57434f53_e3e3_4631_a5c5_26d2243873aa);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const RESOURCE_MANAGER_COMMUNICATION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const RESOURCE_MANAGER_MAXIMUM_OPTION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const RESOURCE_MANAGER_OBJECT_PATH: ::windows::core::PCWSTR = ::windows::core::w!("\\ResourceManager\\");
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const RESOURCE_MANAGER_VOLATILE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SESI1_NUM_ELEMENTS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SESI2_NUM_ELEMENTS: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHARE_CURRENT_USES_PARMNUM: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHARE_FILE_SD_PARMNUM: u32 = 501u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHARE_MAX_USES_PARMNUM: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHARE_NETNAME_PARMNUM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHARE_PASSWD_PARMNUM: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHARE_PATH_PARMNUM: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHARE_PERMISSIONS_PARMNUM: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHARE_QOS_POLICY_PARMNUM: u32 = 504u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHARE_REMARK_PARMNUM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHARE_SERVER_PARMNUM: u32 = 503u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHARE_TYPE_PARMNUM: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI1005_FLAGS_ACCESS_BASED_DIRECTORY_ENUM: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI1005_FLAGS_ALLOW_NAMESPACE_CACHING: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI1005_FLAGS_CLUSTER_MANAGED: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI1005_FLAGS_COMPRESS_DATA: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI1005_FLAGS_DFS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI1005_FLAGS_DFS_ROOT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI1005_FLAGS_DISABLE_CLIENT_BUFFERING: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI1005_FLAGS_DISABLE_DIRECTORY_HANDLE_LEASING: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI1005_FLAGS_ENABLE_CA: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI1005_FLAGS_ENABLE_HASH: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI1005_FLAGS_ENCRYPT_DATA: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI1005_FLAGS_FORCE_LEVELII_OPLOCK: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI1005_FLAGS_FORCE_SHARED_DELETE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI1005_FLAGS_IDENTITY_REMOTING: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI1005_FLAGS_ISOLATED_TRANSPORT: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI1005_FLAGS_RESERVED: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI1005_FLAGS_RESTRICT_EXCLUSIVE_OPENS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI1_NUM_ELEMENTS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI2_NUM_ELEMENTS: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI_USES_UNLIMITED: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STATSOPT_CLR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STYPE_RESERVED1: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STYPE_RESERVED2: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STYPE_RESERVED3: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STYPE_RESERVED4: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STYPE_RESERVED5: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STYPE_RESERVED_ALL: u32 = 1073741568u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTIONMANAGER_OBJECT_PATH: ::windows::core::PCWSTR = ::windows::core::w!("\\TransactionManager\\");
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_DO_NOT_PROMOTE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_MANAGER_COMMIT_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_MANAGER_COMMIT_LOWEST: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_MANAGER_COMMIT_SYSTEM_HIVES: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_MANAGER_COMMIT_SYSTEM_VOLUME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_MANAGER_CORRUPT_FOR_PROGRESS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_MANAGER_CORRUPT_FOR_RECOVERY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_MANAGER_MAXIMUM_OPTION: u32 = 63u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_MANAGER_VOLATILE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_MAXIMUM_OPTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFICATION_TM_ONLINE_FLAG_IS_CLUSTERED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_COMMIT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_COMMIT_COMPLETE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_COMMIT_FINALIZE: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_COMMIT_REQUEST: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_DELEGATE_COMMIT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_ENLIST_MASK: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_ENLIST_PREPREPARE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_INDOUBT: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_LAST_RECOVER: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_MARSHAL: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_MASK: u32 = 1073741823u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_PREPARE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_PREPARE_COMPLETE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_PREPREPARE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_PREPREPARE_COMPLETE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_PROMOTE: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_PROMOTE_NEW: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_PROPAGATE_PULL: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_PROPAGATE_PUSH: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_RECOVER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_RECOVER_QUERY: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_REQUEST_OUTCOME: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_RM_DISCONNECTED: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_ROLLBACK: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_ROLLBACK_COMPLETE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_SINGLE_PHASE_COMMIT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_TM_ONLINE: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_OBJECT_PATH: ::windows::core::PCWSTR = ::windows::core::w!("\\Transaction\\");
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TXF_LOG_RECORD_GENERIC_TYPE_ABORT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TXF_LOG_RECORD_GENERIC_TYPE_COMMIT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TXF_LOG_RECORD_GENERIC_TYPE_DATA: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TXF_LOG_RECORD_GENERIC_TYPE_PREPARE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VS_FFI_FILEFLAGSMASK: i32 = 63i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VS_FFI_SIGNATURE: i32 = -17890115i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VS_FFI_STRUCVERSION: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VS_USER_DEFINED: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VS_VERSION_INFO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const WIM_BOOT_NOT_OS_WIM: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const WIM_BOOT_OS_WIM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const WIM_ENTRY_FLAG_NOT_ACTIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const WIM_ENTRY_FLAG_SUSPENDED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const WIM_EXTERNAL_FILE_INFO_FLAG_NOT_ACTIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const WIM_EXTERNAL_FILE_INFO_FLAG_SUSPENDED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const WIM_PROVIDER_HASH_SIZE: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const WINEFS_SETUSERKEY_SET_CAPABILITIES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const WOF_PROVIDER_FILE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const WOF_PROVIDER_WIM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const _FT_TYPES_DEFINITION_: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLFS_CONTEXT_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsContextNone: CLFS_CONTEXT_MODE = CLFS_CONTEXT_MODE(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsContextUndoNext: CLFS_CONTEXT_MODE = CLFS_CONTEXT_MODE(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsContextPrevious: CLFS_CONTEXT_MODE = CLFS_CONTEXT_MODE(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsContextForward: CLFS_CONTEXT_MODE = CLFS_CONTEXT_MODE(3i32);
impl ::core::marker::Copy for CLFS_CONTEXT_MODE {}
impl ::core::clone::Clone for CLFS_CONTEXT_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CLFS_CONTEXT_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CLFS_CONTEXT_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CLFS_CONTEXT_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLFS_CONTEXT_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLFS_FLAG(pub u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_FLAG_FORCE_APPEND: CLFS_FLAG = CLFS_FLAG(1u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_FLAG_FORCE_FLUSH: CLFS_FLAG = CLFS_FLAG(2u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_FLAG_NO_FLAGS: CLFS_FLAG = CLFS_FLAG(0u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_FLAG_USE_RESERVATION: CLFS_FLAG = CLFS_FLAG(4u32);
impl ::core::marker::Copy for CLFS_FLAG {}
impl ::core::clone::Clone for CLFS_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CLFS_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CLFS_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CLFS_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLFS_FLAG").field(&self.0).finish()
    }
}
impl CLFS_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for CLFS_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CLFS_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CLFS_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CLFS_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CLFS_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLFS_IOSTATS_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsIoStatsDefault: CLFS_IOSTATS_CLASS = CLFS_IOSTATS_CLASS(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsIoStatsMax: CLFS_IOSTATS_CLASS = CLFS_IOSTATS_CLASS(65535i32);
impl ::core::marker::Copy for CLFS_IOSTATS_CLASS {}
impl ::core::clone::Clone for CLFS_IOSTATS_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CLFS_IOSTATS_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CLFS_IOSTATS_CLASS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CLFS_IOSTATS_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLFS_IOSTATS_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLFS_LOG_ARCHIVE_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsLogArchiveEnabled: CLFS_LOG_ARCHIVE_MODE = CLFS_LOG_ARCHIVE_MODE(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsLogArchiveDisabled: CLFS_LOG_ARCHIVE_MODE = CLFS_LOG_ARCHIVE_MODE(2i32);
impl ::core::marker::Copy for CLFS_LOG_ARCHIVE_MODE {}
impl ::core::clone::Clone for CLFS_LOG_ARCHIVE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CLFS_LOG_ARCHIVE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CLFS_LOG_ARCHIVE_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CLFS_LOG_ARCHIVE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLFS_LOG_ARCHIVE_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLFS_MGMT_NOTIFICATION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsMgmtAdvanceTailNotification: CLFS_MGMT_NOTIFICATION_TYPE = CLFS_MGMT_NOTIFICATION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsMgmtLogFullHandlerNotification: CLFS_MGMT_NOTIFICATION_TYPE = CLFS_MGMT_NOTIFICATION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsMgmtLogUnpinnedNotification: CLFS_MGMT_NOTIFICATION_TYPE = CLFS_MGMT_NOTIFICATION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsMgmtLogWriteNotification: CLFS_MGMT_NOTIFICATION_TYPE = CLFS_MGMT_NOTIFICATION_TYPE(3i32);
impl ::core::marker::Copy for CLFS_MGMT_NOTIFICATION_TYPE {}
impl ::core::clone::Clone for CLFS_MGMT_NOTIFICATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CLFS_MGMT_NOTIFICATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CLFS_MGMT_NOTIFICATION_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CLFS_MGMT_NOTIFICATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLFS_MGMT_NOTIFICATION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLFS_MGMT_POLICY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsMgmtPolicyMaximumSize: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsMgmtPolicyMinimumSize: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsMgmtPolicyNewContainerSize: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsMgmtPolicyGrowthRate: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsMgmtPolicyLogTail: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsMgmtPolicyAutoShrink: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsMgmtPolicyAutoGrow: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsMgmtPolicyNewContainerPrefix: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsMgmtPolicyNewContainerSuffix: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsMgmtPolicyNewContainerExtension: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsMgmtPolicyInvalid: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(10i32);
impl ::core::marker::Copy for CLFS_MGMT_POLICY_TYPE {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CLFS_MGMT_POLICY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CLFS_MGMT_POLICY_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CLFS_MGMT_POLICY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLFS_MGMT_POLICY_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLS_CONTEXT_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClsContextNone: CLS_CONTEXT_MODE = CLS_CONTEXT_MODE(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClsContextUndoNext: CLS_CONTEXT_MODE = CLS_CONTEXT_MODE(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClsContextPrevious: CLS_CONTEXT_MODE = CLS_CONTEXT_MODE(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClsContextForward: CLS_CONTEXT_MODE = CLS_CONTEXT_MODE(3i32);
impl ::core::marker::Copy for CLS_CONTEXT_MODE {}
impl ::core::clone::Clone for CLS_CONTEXT_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CLS_CONTEXT_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CLS_CONTEXT_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CLS_CONTEXT_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLS_CONTEXT_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLS_IOSTATS_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClsIoStatsDefault: CLS_IOSTATS_CLASS = CLS_IOSTATS_CLASS(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClsIoStatsMax: CLS_IOSTATS_CLASS = CLS_IOSTATS_CLASS(65535i32);
impl ::core::marker::Copy for CLS_IOSTATS_CLASS {}
impl ::core::clone::Clone for CLS_IOSTATS_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CLS_IOSTATS_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CLS_IOSTATS_CLASS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CLS_IOSTATS_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLS_IOSTATS_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLS_LOG_INFORMATION_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsLogBasicInformation: CLS_LOG_INFORMATION_CLASS = CLS_LOG_INFORMATION_CLASS(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsLogBasicInformationPhysical: CLS_LOG_INFORMATION_CLASS = CLS_LOG_INFORMATION_CLASS(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsLogPhysicalNameInformation: CLS_LOG_INFORMATION_CLASS = CLS_LOG_INFORMATION_CLASS(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsLogStreamIdentifierInformation: CLS_LOG_INFORMATION_CLASS = CLS_LOG_INFORMATION_CLASS(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsLogSystemMarkingInformation: CLS_LOG_INFORMATION_CLASS = CLS_LOG_INFORMATION_CLASS(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsLogPhysicalLsnInformation: CLS_LOG_INFORMATION_CLASS = CLS_LOG_INFORMATION_CLASS(5i32);
impl ::core::marker::Copy for CLS_LOG_INFORMATION_CLASS {}
impl ::core::clone::Clone for CLS_LOG_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CLS_LOG_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CLS_LOG_INFORMATION_CLASS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CLS_LOG_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLS_LOG_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMPRESSION_FORMAT(pub u16);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COMPRESSION_FORMAT_NONE: COMPRESSION_FORMAT = COMPRESSION_FORMAT(0u16);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COMPRESSION_FORMAT_DEFAULT: COMPRESSION_FORMAT = COMPRESSION_FORMAT(1u16);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COMPRESSION_FORMAT_LZNT1: COMPRESSION_FORMAT = COMPRESSION_FORMAT(2u16);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COMPRESSION_FORMAT_XPRESS: COMPRESSION_FORMAT = COMPRESSION_FORMAT(3u16);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COMPRESSION_FORMAT_XPRESS_HUFF: COMPRESSION_FORMAT = COMPRESSION_FORMAT(4u16);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COMPRESSION_FORMAT_XP10: COMPRESSION_FORMAT = COMPRESSION_FORMAT(5u16);
impl ::core::marker::Copy for COMPRESSION_FORMAT {}
impl ::core::clone::Clone for COMPRESSION_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMPRESSION_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for COMPRESSION_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for COMPRESSION_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMPRESSION_FORMAT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COPYFILE2_COPY_PHASE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_PHASE_NONE: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_PHASE_PREPARE_SOURCE: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_PHASE_PREPARE_DEST: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_PHASE_READ_SOURCE: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_PHASE_WRITE_DESTINATION: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_PHASE_SERVER_COPY: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(5i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_PHASE_NAMEGRAFT_COPY: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(6i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_PHASE_MAX: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(7i32);
impl ::core::marker::Copy for COPYFILE2_COPY_PHASE {}
impl ::core::clone::Clone for COPYFILE2_COPY_PHASE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COPYFILE2_COPY_PHASE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for COPYFILE2_COPY_PHASE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for COPYFILE2_COPY_PHASE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COPYFILE2_COPY_PHASE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COPYFILE2_MESSAGE_ACTION(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_PROGRESS_CONTINUE: COPYFILE2_MESSAGE_ACTION = COPYFILE2_MESSAGE_ACTION(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_PROGRESS_CANCEL: COPYFILE2_MESSAGE_ACTION = COPYFILE2_MESSAGE_ACTION(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_PROGRESS_STOP: COPYFILE2_MESSAGE_ACTION = COPYFILE2_MESSAGE_ACTION(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_PROGRESS_QUIET: COPYFILE2_MESSAGE_ACTION = COPYFILE2_MESSAGE_ACTION(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_PROGRESS_PAUSE: COPYFILE2_MESSAGE_ACTION = COPYFILE2_MESSAGE_ACTION(4i32);
impl ::core::marker::Copy for COPYFILE2_MESSAGE_ACTION {}
impl ::core::clone::Clone for COPYFILE2_MESSAGE_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COPYFILE2_MESSAGE_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for COPYFILE2_MESSAGE_ACTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for COPYFILE2_MESSAGE_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COPYFILE2_MESSAGE_ACTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COPYFILE2_MESSAGE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_CALLBACK_NONE: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_CALLBACK_CHUNK_STARTED: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_CALLBACK_CHUNK_FINISHED: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_CALLBACK_STREAM_STARTED: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_CALLBACK_STREAM_FINISHED: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_CALLBACK_POLL_CONTINUE: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_CALLBACK_ERROR: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_CALLBACK_MAX: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(7i32);
impl ::core::marker::Copy for COPYFILE2_MESSAGE_TYPE {}
impl ::core::clone::Clone for COPYFILE2_MESSAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COPYFILE2_MESSAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for COPYFILE2_MESSAGE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for COPYFILE2_MESSAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COPYFILE2_MESSAGE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CREATE_TAPE_PARTITION_METHOD(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_FIXED_PARTITIONS: CREATE_TAPE_PARTITION_METHOD = CREATE_TAPE_PARTITION_METHOD(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_INITIATOR_PARTITIONS: CREATE_TAPE_PARTITION_METHOD = CREATE_TAPE_PARTITION_METHOD(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_SELECT_PARTITIONS: CREATE_TAPE_PARTITION_METHOD = CREATE_TAPE_PARTITION_METHOD(1i32);
impl ::core::marker::Copy for CREATE_TAPE_PARTITION_METHOD {}
impl ::core::clone::Clone for CREATE_TAPE_PARTITION_METHOD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CREATE_TAPE_PARTITION_METHOD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CREATE_TAPE_PARTITION_METHOD {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CREATE_TAPE_PARTITION_METHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREATE_TAPE_PARTITION_METHOD").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DEFINE_DOS_DEVICE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DDD_RAW_TARGET_PATH: DEFINE_DOS_DEVICE_FLAGS = DEFINE_DOS_DEVICE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DDD_REMOVE_DEFINITION: DEFINE_DOS_DEVICE_FLAGS = DEFINE_DOS_DEVICE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DDD_EXACT_MATCH_ON_REMOVE: DEFINE_DOS_DEVICE_FLAGS = DEFINE_DOS_DEVICE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DDD_NO_BROADCAST_SYSTEM: DEFINE_DOS_DEVICE_FLAGS = DEFINE_DOS_DEVICE_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DDD_LUID_BROADCAST_DRIVE: DEFINE_DOS_DEVICE_FLAGS = DEFINE_DOS_DEVICE_FLAGS(16u32);
impl ::core::marker::Copy for DEFINE_DOS_DEVICE_FLAGS {}
impl ::core::clone::Clone for DEFINE_DOS_DEVICE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DEFINE_DOS_DEVICE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DEFINE_DOS_DEVICE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DEFINE_DOS_DEVICE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEFINE_DOS_DEVICE_FLAGS").field(&self.0).finish()
    }
}
impl DEFINE_DOS_DEVICE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for DEFINE_DOS_DEVICE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DEFINE_DOS_DEVICE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DEFINE_DOS_DEVICE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DEFINE_DOS_DEVICE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DEFINE_DOS_DEVICE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISKQUOTA_USERNAME_RESOLVE(pub u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DISKQUOTA_USERNAME_RESOLVE_ASYNC: DISKQUOTA_USERNAME_RESOLVE = DISKQUOTA_USERNAME_RESOLVE(2u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DISKQUOTA_USERNAME_RESOLVE_NONE: DISKQUOTA_USERNAME_RESOLVE = DISKQUOTA_USERNAME_RESOLVE(0u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DISKQUOTA_USERNAME_RESOLVE_SYNC: DISKQUOTA_USERNAME_RESOLVE = DISKQUOTA_USERNAME_RESOLVE(1u32);
impl ::core::marker::Copy for DISKQUOTA_USERNAME_RESOLVE {}
impl ::core::clone::Clone for DISKQUOTA_USERNAME_RESOLVE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISKQUOTA_USERNAME_RESOLVE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISKQUOTA_USERNAME_RESOLVE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISKQUOTA_USERNAME_RESOLVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISKQUOTA_USERNAME_RESOLVE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ERASE_TAPE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_ERASE_LONG: ERASE_TAPE_TYPE = ERASE_TAPE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_ERASE_SHORT: ERASE_TAPE_TYPE = ERASE_TAPE_TYPE(0i32);
impl ::core::marker::Copy for ERASE_TAPE_TYPE {}
impl ::core::clone::Clone for ERASE_TAPE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ERASE_TAPE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ERASE_TAPE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ERASE_TAPE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ERASE_TAPE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FILE_ACCESS_RIGHTS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_READ_DATA: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(1u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_LIST_DIRECTORY: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(1u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_WRITE_DATA: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(2u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ADD_FILE: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(2u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_APPEND_DATA: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(4u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ADD_SUBDIRECTORY: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(4u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_CREATE_PIPE_INSTANCE: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(4u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_READ_EA: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(8u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_WRITE_EA: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(16u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_EXECUTE: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(32u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_TRAVERSE: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(32u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_DELETE_CHILD: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(64u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_READ_ATTRIBUTES: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(128u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_WRITE_ATTRIBUTES: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(256u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DELETE: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(65536u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const READ_CONTROL: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(131072u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const WRITE_DAC: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(262144u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const WRITE_OWNER: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(524288u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SYNCHRONIZE: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(1048576u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STANDARD_RIGHTS_REQUIRED: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(983040u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STANDARD_RIGHTS_READ: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(131072u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STANDARD_RIGHTS_WRITE: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(131072u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STANDARD_RIGHTS_EXECUTE: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(131072u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STANDARD_RIGHTS_ALL: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(2031616u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SPECIFIC_RIGHTS_ALL: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(65535u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ALL_ACCESS: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(2032127u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_GENERIC_READ: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(1179785u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_GENERIC_WRITE: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(1179926u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_GENERIC_EXECUTE: FILE_ACCESS_RIGHTS = FILE_ACCESS_RIGHTS(1179808u32);
impl ::core::marker::Copy for FILE_ACCESS_RIGHTS {}
impl ::core::clone::Clone for FILE_ACCESS_RIGHTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILE_ACCESS_RIGHTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for FILE_ACCESS_RIGHTS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FILE_ACCESS_RIGHTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_ACCESS_RIGHTS").field(&self.0).finish()
    }
}
impl FILE_ACCESS_RIGHTS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for FILE_ACCESS_RIGHTS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FILE_ACCESS_RIGHTS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FILE_ACCESS_RIGHTS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FILE_ACCESS_RIGHTS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FILE_ACCESS_RIGHTS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FILE_ACTION(pub u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ACTION_ADDED: FILE_ACTION = FILE_ACTION(1u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ACTION_REMOVED: FILE_ACTION = FILE_ACTION(2u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ACTION_MODIFIED: FILE_ACTION = FILE_ACTION(3u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ACTION_RENAMED_OLD_NAME: FILE_ACTION = FILE_ACTION(4u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ACTION_RENAMED_NEW_NAME: FILE_ACTION = FILE_ACTION(5u32);
impl ::core::marker::Copy for FILE_ACTION {}
impl ::core::clone::Clone for FILE_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILE_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for FILE_ACTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FILE_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_ACTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FILE_CREATION_DISPOSITION(pub u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CREATE_NEW: FILE_CREATION_DISPOSITION = FILE_CREATION_DISPOSITION(1u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CREATE_ALWAYS: FILE_CREATION_DISPOSITION = FILE_CREATION_DISPOSITION(2u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const OPEN_EXISTING: FILE_CREATION_DISPOSITION = FILE_CREATION_DISPOSITION(3u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const OPEN_ALWAYS: FILE_CREATION_DISPOSITION = FILE_CREATION_DISPOSITION(4u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRUNCATE_EXISTING: FILE_CREATION_DISPOSITION = FILE_CREATION_DISPOSITION(5u32);
impl ::core::marker::Copy for FILE_CREATION_DISPOSITION {}
impl ::core::clone::Clone for FILE_CREATION_DISPOSITION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILE_CREATION_DISPOSITION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for FILE_CREATION_DISPOSITION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FILE_CREATION_DISPOSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_CREATION_DISPOSITION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FILE_DEVICE_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_CD_ROM: FILE_DEVICE_TYPE = FILE_DEVICE_TYPE(2u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_DISK: FILE_DEVICE_TYPE = FILE_DEVICE_TYPE(7u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_TAPE: FILE_DEVICE_TYPE = FILE_DEVICE_TYPE(31u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_DVD: FILE_DEVICE_TYPE = FILE_DEVICE_TYPE(51u32);
impl ::core::marker::Copy for FILE_DEVICE_TYPE {}
impl ::core::clone::Clone for FILE_DEVICE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILE_DEVICE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for FILE_DEVICE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FILE_DEVICE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_DEVICE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FILE_FLAGS_AND_ATTRIBUTES(pub u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_READONLY: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(1u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_HIDDEN: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(2u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_SYSTEM: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(4u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_DIRECTORY: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(16u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_ARCHIVE: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(32u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_DEVICE: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(64u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_NORMAL: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(128u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_TEMPORARY: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(256u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_SPARSE_FILE: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(512u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_REPARSE_POINT: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(1024u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_COMPRESSED: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(2048u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_OFFLINE: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(4096u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_NOT_CONTENT_INDEXED: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(8192u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_ENCRYPTED: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(16384u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_INTEGRITY_STREAM: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(32768u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_VIRTUAL: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(65536u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_NO_SCRUB_DATA: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(131072u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_EA: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(262144u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_PINNED: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(524288u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_UNPINNED: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(1048576u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_RECALL_ON_OPEN: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(262144u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(4194304u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_FLAG_WRITE_THROUGH: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(2147483648u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_FLAG_OVERLAPPED: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(1073741824u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_FLAG_NO_BUFFERING: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(536870912u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_FLAG_RANDOM_ACCESS: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(268435456u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_FLAG_SEQUENTIAL_SCAN: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(134217728u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_FLAG_DELETE_ON_CLOSE: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(67108864u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_FLAG_BACKUP_SEMANTICS: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(33554432u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_FLAG_POSIX_SEMANTICS: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(16777216u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_FLAG_SESSION_AWARE: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(8388608u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_FLAG_OPEN_REPARSE_POINT: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(2097152u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_FLAG_OPEN_NO_RECALL: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(1048576u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_FLAG_FIRST_PIPE_INSTANCE: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(524288u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PIPE_ACCESS_DUPLEX: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(3u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PIPE_ACCESS_INBOUND: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(1u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PIPE_ACCESS_OUTBOUND: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(2u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SECURITY_ANONYMOUS: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(0u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SECURITY_IDENTIFICATION: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(65536u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SECURITY_IMPERSONATION: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(131072u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SECURITY_DELEGATION: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(196608u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SECURITY_CONTEXT_TRACKING: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(262144u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SECURITY_EFFECTIVE_ONLY: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(524288u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SECURITY_SQOS_PRESENT: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(1048576u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SECURITY_VALID_SQOS_FLAGS: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(2031616u32);
impl ::core::marker::Copy for FILE_FLAGS_AND_ATTRIBUTES {}
impl ::core::clone::Clone for FILE_FLAGS_AND_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILE_FLAGS_AND_ATTRIBUTES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for FILE_FLAGS_AND_ATTRIBUTES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FILE_FLAGS_AND_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_FLAGS_AND_ATTRIBUTES").field(&self.0).finish()
    }
}
impl FILE_FLAGS_AND_ATTRIBUTES {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for FILE_FLAGS_AND_ATTRIBUTES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FILE_FLAGS_AND_ATTRIBUTES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FILE_FLAGS_AND_ATTRIBUTES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FILE_FLAGS_AND_ATTRIBUTES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FILE_FLAGS_AND_ATTRIBUTES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FILE_ID_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileIdType: FILE_ID_TYPE = FILE_ID_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ObjectIdType: FILE_ID_TYPE = FILE_ID_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ExtendedFileIdType: FILE_ID_TYPE = FILE_ID_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const MaximumFileIdType: FILE_ID_TYPE = FILE_ID_TYPE(3i32);
impl ::core::marker::Copy for FILE_ID_TYPE {}
impl ::core::clone::Clone for FILE_ID_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILE_ID_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for FILE_ID_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FILE_ID_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_ID_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FILE_INFO_BY_HANDLE_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileBasicInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileStandardInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileNameInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileRenameInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileDispositionInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileAllocationInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(5i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileEndOfFileInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(6i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileStreamInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(7i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileCompressionInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(8i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileAttributeTagInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(9i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileIdBothDirectoryInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(10i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileIdBothDirectoryRestartInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(11i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileIoPriorityHintInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(12i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileRemoteProtocolInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(13i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileFullDirectoryInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(14i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileFullDirectoryRestartInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(15i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileStorageInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(16i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileAlignmentInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(17i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileIdInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(18i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileIdExtdDirectoryInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(19i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileIdExtdDirectoryRestartInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(20i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileDispositionInfoEx: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(21i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileRenameInfoEx: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(22i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileCaseSensitiveInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(23i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileNormalizedNameInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(24i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const MaximumFileInfoByHandleClass: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(25i32);
impl ::core::marker::Copy for FILE_INFO_BY_HANDLE_CLASS {}
impl ::core::clone::Clone for FILE_INFO_BY_HANDLE_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILE_INFO_BY_HANDLE_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for FILE_INFO_BY_HANDLE_CLASS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FILE_INFO_BY_HANDLE_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_INFO_BY_HANDLE_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FILE_INFO_FLAGS_PERMISSIONS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PERM_FILE_READ: FILE_INFO_FLAGS_PERMISSIONS = FILE_INFO_FLAGS_PERMISSIONS(1u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PERM_FILE_WRITE: FILE_INFO_FLAGS_PERMISSIONS = FILE_INFO_FLAGS_PERMISSIONS(2u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PERM_FILE_CREATE: FILE_INFO_FLAGS_PERMISSIONS = FILE_INFO_FLAGS_PERMISSIONS(4u32);
impl ::core::marker::Copy for FILE_INFO_FLAGS_PERMISSIONS {}
impl ::core::clone::Clone for FILE_INFO_FLAGS_PERMISSIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILE_INFO_FLAGS_PERMISSIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for FILE_INFO_FLAGS_PERMISSIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FILE_INFO_FLAGS_PERMISSIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_INFO_FLAGS_PERMISSIONS").field(&self.0).finish()
    }
}
impl FILE_INFO_FLAGS_PERMISSIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for FILE_INFO_FLAGS_PERMISSIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FILE_INFO_FLAGS_PERMISSIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FILE_INFO_FLAGS_PERMISSIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FILE_INFO_FLAGS_PERMISSIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FILE_INFO_FLAGS_PERMISSIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FILE_NAME(pub u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_NAME_NORMALIZED: FILE_NAME = FILE_NAME(0u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_NAME_OPENED: FILE_NAME = FILE_NAME(8u32);
impl ::core::marker::Copy for FILE_NAME {}
impl ::core::clone::Clone for FILE_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILE_NAME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for FILE_NAME {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FILE_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_NAME").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FILE_NOTIFY_CHANGE(pub u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_NOTIFY_CHANGE_FILE_NAME: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(1u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_NOTIFY_CHANGE_DIR_NAME: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(2u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_NOTIFY_CHANGE_ATTRIBUTES: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(4u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_NOTIFY_CHANGE_SIZE: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(8u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_NOTIFY_CHANGE_LAST_WRITE: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(16u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_NOTIFY_CHANGE_LAST_ACCESS: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(32u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_NOTIFY_CHANGE_CREATION: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(64u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_NOTIFY_CHANGE_SECURITY: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(256u32);
impl ::core::marker::Copy for FILE_NOTIFY_CHANGE {}
impl ::core::clone::Clone for FILE_NOTIFY_CHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILE_NOTIFY_CHANGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for FILE_NOTIFY_CHANGE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FILE_NOTIFY_CHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_NOTIFY_CHANGE").field(&self.0).finish()
    }
}
impl FILE_NOTIFY_CHANGE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for FILE_NOTIFY_CHANGE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FILE_NOTIFY_CHANGE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FILE_NOTIFY_CHANGE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FILE_NOTIFY_CHANGE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FILE_NOTIFY_CHANGE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FILE_SHARE_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_SHARE_NONE: FILE_SHARE_MODE = FILE_SHARE_MODE(0u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_SHARE_DELETE: FILE_SHARE_MODE = FILE_SHARE_MODE(4u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_SHARE_READ: FILE_SHARE_MODE = FILE_SHARE_MODE(1u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_SHARE_WRITE: FILE_SHARE_MODE = FILE_SHARE_MODE(2u32);
impl ::core::marker::Copy for FILE_SHARE_MODE {}
impl ::core::clone::Clone for FILE_SHARE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILE_SHARE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for FILE_SHARE_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FILE_SHARE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_SHARE_MODE").field(&self.0).finish()
    }
}
impl FILE_SHARE_MODE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for FILE_SHARE_MODE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FILE_SHARE_MODE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FILE_SHARE_MODE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FILE_SHARE_MODE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FILE_SHARE_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FILE_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_TYPE_UNKNOWN: FILE_TYPE = FILE_TYPE(0u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_TYPE_DISK: FILE_TYPE = FILE_TYPE(1u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_TYPE_CHAR: FILE_TYPE = FILE_TYPE(2u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_TYPE_PIPE: FILE_TYPE = FILE_TYPE(3u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_TYPE_REMOTE: FILE_TYPE = FILE_TYPE(32768u32);
impl ::core::marker::Copy for FILE_TYPE {}
impl ::core::clone::Clone for FILE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for FILE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FILE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FINDEX_INFO_LEVELS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FindExInfoStandard: FINDEX_INFO_LEVELS = FINDEX_INFO_LEVELS(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FindExInfoBasic: FINDEX_INFO_LEVELS = FINDEX_INFO_LEVELS(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FindExInfoMaxInfoLevel: FINDEX_INFO_LEVELS = FINDEX_INFO_LEVELS(2i32);
impl ::core::marker::Copy for FINDEX_INFO_LEVELS {}
impl ::core::clone::Clone for FINDEX_INFO_LEVELS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FINDEX_INFO_LEVELS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for FINDEX_INFO_LEVELS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FINDEX_INFO_LEVELS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FINDEX_INFO_LEVELS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FINDEX_SEARCH_OPS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FindExSearchNameMatch: FINDEX_SEARCH_OPS = FINDEX_SEARCH_OPS(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FindExSearchLimitToDirectories: FINDEX_SEARCH_OPS = FINDEX_SEARCH_OPS(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FindExSearchLimitToDevices: FINDEX_SEARCH_OPS = FINDEX_SEARCH_OPS(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FindExSearchMaxSearchOp: FINDEX_SEARCH_OPS = FINDEX_SEARCH_OPS(3i32);
impl ::core::marker::Copy for FINDEX_SEARCH_OPS {}
impl ::core::clone::Clone for FINDEX_SEARCH_OPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FINDEX_SEARCH_OPS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for FINDEX_SEARCH_OPS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FINDEX_SEARCH_OPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FINDEX_SEARCH_OPS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FIND_FIRST_EX_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FIND_FIRST_EX_CASE_SENSITIVE: FIND_FIRST_EX_FLAGS = FIND_FIRST_EX_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FIND_FIRST_EX_LARGE_FETCH: FIND_FIRST_EX_FLAGS = FIND_FIRST_EX_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FIND_FIRST_EX_ON_DISK_ENTRIES_ONLY: FIND_FIRST_EX_FLAGS = FIND_FIRST_EX_FLAGS(4u32);
impl ::core::marker::Copy for FIND_FIRST_EX_FLAGS {}
impl ::core::clone::Clone for FIND_FIRST_EX_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FIND_FIRST_EX_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for FIND_FIRST_EX_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FIND_FIRST_EX_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FIND_FIRST_EX_FLAGS").field(&self.0).finish()
    }
}
impl FIND_FIRST_EX_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for FIND_FIRST_EX_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FIND_FIRST_EX_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FIND_FIRST_EX_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FIND_FIRST_EX_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FIND_FIRST_EX_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GET_FILEEX_INFO_LEVELS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const GetFileExInfoStandard: GET_FILEEX_INFO_LEVELS = GET_FILEEX_INFO_LEVELS(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const GetFileExMaxInfoLevel: GET_FILEEX_INFO_LEVELS = GET_FILEEX_INFO_LEVELS(1i32);
impl ::core::marker::Copy for GET_FILEEX_INFO_LEVELS {}
impl ::core::clone::Clone for GET_FILEEX_INFO_LEVELS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GET_FILEEX_INFO_LEVELS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for GET_FILEEX_INFO_LEVELS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for GET_FILEEX_INFO_LEVELS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_FILEEX_INFO_LEVELS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GET_FILE_VERSION_INFO_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_VER_GET_LOCALISED: GET_FILE_VERSION_INFO_FLAGS = GET_FILE_VERSION_INFO_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_VER_GET_NEUTRAL: GET_FILE_VERSION_INFO_FLAGS = GET_FILE_VERSION_INFO_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_VER_GET_PREFETCHED: GET_FILE_VERSION_INFO_FLAGS = GET_FILE_VERSION_INFO_FLAGS(4u32);
impl ::core::marker::Copy for GET_FILE_VERSION_INFO_FLAGS {}
impl ::core::clone::Clone for GET_FILE_VERSION_INFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GET_FILE_VERSION_INFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for GET_FILE_VERSION_INFO_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for GET_FILE_VERSION_INFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_FILE_VERSION_INFO_FLAGS").field(&self.0).finish()
    }
}
impl GET_FILE_VERSION_INFO_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for GET_FILE_VERSION_INFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GET_FILE_VERSION_INFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GET_FILE_VERSION_INFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GET_FILE_VERSION_INFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GET_FILE_VERSION_INFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GET_TAPE_DRIVE_PARAMETERS_OPERATION(pub u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const GET_TAPE_DRIVE_INFORMATION: GET_TAPE_DRIVE_PARAMETERS_OPERATION = GET_TAPE_DRIVE_PARAMETERS_OPERATION(1u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const GET_TAPE_MEDIA_INFORMATION: GET_TAPE_DRIVE_PARAMETERS_OPERATION = GET_TAPE_DRIVE_PARAMETERS_OPERATION(0u32);
impl ::core::marker::Copy for GET_TAPE_DRIVE_PARAMETERS_OPERATION {}
impl ::core::clone::Clone for GET_TAPE_DRIVE_PARAMETERS_OPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GET_TAPE_DRIVE_PARAMETERS_OPERATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for GET_TAPE_DRIVE_PARAMETERS_OPERATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for GET_TAPE_DRIVE_PARAMETERS_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_TAPE_DRIVE_PARAMETERS_OPERATION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IORING_CREATE_ADVISORY_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IORING_CREATE_ADVISORY_FLAGS_NONE: IORING_CREATE_ADVISORY_FLAGS = IORING_CREATE_ADVISORY_FLAGS(0i32);
impl ::core::marker::Copy for IORING_CREATE_ADVISORY_FLAGS {}
impl ::core::clone::Clone for IORING_CREATE_ADVISORY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IORING_CREATE_ADVISORY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for IORING_CREATE_ADVISORY_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for IORING_CREATE_ADVISORY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IORING_CREATE_ADVISORY_FLAGS").field(&self.0).finish()
    }
}
impl IORING_CREATE_ADVISORY_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for IORING_CREATE_ADVISORY_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IORING_CREATE_ADVISORY_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IORING_CREATE_ADVISORY_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IORING_CREATE_ADVISORY_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IORING_CREATE_ADVISORY_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IORING_CREATE_REQUIRED_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IORING_CREATE_REQUIRED_FLAGS_NONE: IORING_CREATE_REQUIRED_FLAGS = IORING_CREATE_REQUIRED_FLAGS(0i32);
impl ::core::marker::Copy for IORING_CREATE_REQUIRED_FLAGS {}
impl ::core::clone::Clone for IORING_CREATE_REQUIRED_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IORING_CREATE_REQUIRED_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for IORING_CREATE_REQUIRED_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for IORING_CREATE_REQUIRED_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IORING_CREATE_REQUIRED_FLAGS").field(&self.0).finish()
    }
}
impl IORING_CREATE_REQUIRED_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for IORING_CREATE_REQUIRED_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IORING_CREATE_REQUIRED_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IORING_CREATE_REQUIRED_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IORING_CREATE_REQUIRED_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IORING_CREATE_REQUIRED_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IORING_FEATURE_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IORING_FEATURE_FLAGS_NONE: IORING_FEATURE_FLAGS = IORING_FEATURE_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IORING_FEATURE_UM_EMULATION: IORING_FEATURE_FLAGS = IORING_FEATURE_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IORING_FEATURE_SET_COMPLETION_EVENT: IORING_FEATURE_FLAGS = IORING_FEATURE_FLAGS(2i32);
impl ::core::marker::Copy for IORING_FEATURE_FLAGS {}
impl ::core::clone::Clone for IORING_FEATURE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IORING_FEATURE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for IORING_FEATURE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for IORING_FEATURE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IORING_FEATURE_FLAGS").field(&self.0).finish()
    }
}
impl IORING_FEATURE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for IORING_FEATURE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IORING_FEATURE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IORING_FEATURE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IORING_FEATURE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IORING_FEATURE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IORING_OP_CODE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IORING_OP_NOP: IORING_OP_CODE = IORING_OP_CODE(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IORING_OP_READ: IORING_OP_CODE = IORING_OP_CODE(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IORING_OP_REGISTER_FILES: IORING_OP_CODE = IORING_OP_CODE(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IORING_OP_REGISTER_BUFFERS: IORING_OP_CODE = IORING_OP_CODE(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IORING_OP_CANCEL: IORING_OP_CODE = IORING_OP_CODE(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IORING_OP_WRITE: IORING_OP_CODE = IORING_OP_CODE(5i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IORING_OP_FLUSH: IORING_OP_CODE = IORING_OP_CODE(6i32);
impl ::core::marker::Copy for IORING_OP_CODE {}
impl ::core::clone::Clone for IORING_OP_CODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IORING_OP_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for IORING_OP_CODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for IORING_OP_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IORING_OP_CODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IORING_REF_KIND(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IORING_REF_RAW: IORING_REF_KIND = IORING_REF_KIND(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IORING_REF_REGISTERED: IORING_REF_KIND = IORING_REF_KIND(1i32);
impl ::core::marker::Copy for IORING_REF_KIND {}
impl ::core::clone::Clone for IORING_REF_KIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IORING_REF_KIND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for IORING_REF_KIND {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for IORING_REF_KIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IORING_REF_KIND").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IORING_SQE_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOSQE_FLAGS_NONE: IORING_SQE_FLAGS = IORING_SQE_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOSQE_FLAGS_DRAIN_PRECEDING_OPS: IORING_SQE_FLAGS = IORING_SQE_FLAGS(1i32);
impl ::core::marker::Copy for IORING_SQE_FLAGS {}
impl ::core::clone::Clone for IORING_SQE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IORING_SQE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for IORING_SQE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for IORING_SQE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IORING_SQE_FLAGS").field(&self.0).finish()
    }
}
impl IORING_SQE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for IORING_SQE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IORING_SQE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IORING_SQE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IORING_SQE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IORING_SQE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IORING_VERSION(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IORING_VERSION_INVALID: IORING_VERSION = IORING_VERSION(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IORING_VERSION_1: IORING_VERSION = IORING_VERSION(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IORING_VERSION_2: IORING_VERSION = IORING_VERSION(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IORING_VERSION_3: IORING_VERSION = IORING_VERSION(300i32);
impl ::core::marker::Copy for IORING_VERSION {}
impl ::core::clone::Clone for IORING_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IORING_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for IORING_VERSION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for IORING_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IORING_VERSION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LOCK_FILE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const LOCKFILE_EXCLUSIVE_LOCK: LOCK_FILE_FLAGS = LOCK_FILE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const LOCKFILE_FAIL_IMMEDIATELY: LOCK_FILE_FLAGS = LOCK_FILE_FLAGS(1u32);
impl ::core::marker::Copy for LOCK_FILE_FLAGS {}
impl ::core::clone::Clone for LOCK_FILE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LOCK_FILE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for LOCK_FILE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for LOCK_FILE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LOCK_FILE_FLAGS").field(&self.0).finish()
    }
}
impl LOCK_FILE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for LOCK_FILE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LOCK_FILE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LOCK_FILE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LOCK_FILE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LOCK_FILE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LPPROGRESS_ROUTINE_CALLBACK_REASON(pub u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CALLBACK_CHUNK_FINISHED: LPPROGRESS_ROUTINE_CALLBACK_REASON = LPPROGRESS_ROUTINE_CALLBACK_REASON(0u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CALLBACK_STREAM_SWITCH: LPPROGRESS_ROUTINE_CALLBACK_REASON = LPPROGRESS_ROUTINE_CALLBACK_REASON(1u32);
impl ::core::marker::Copy for LPPROGRESS_ROUTINE_CALLBACK_REASON {}
impl ::core::clone::Clone for LPPROGRESS_ROUTINE_CALLBACK_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LPPROGRESS_ROUTINE_CALLBACK_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for LPPROGRESS_ROUTINE_CALLBACK_REASON {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for LPPROGRESS_ROUTINE_CALLBACK_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LPPROGRESS_ROUTINE_CALLBACK_REASON").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LZOPENFILE_STYLE(pub u16);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const OF_CANCEL: LZOPENFILE_STYLE = LZOPENFILE_STYLE(2048u16);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const OF_CREATE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(4096u16);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const OF_DELETE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(512u16);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const OF_EXIST: LZOPENFILE_STYLE = LZOPENFILE_STYLE(16384u16);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const OF_PARSE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(256u16);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const OF_PROMPT: LZOPENFILE_STYLE = LZOPENFILE_STYLE(8192u16);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const OF_READ: LZOPENFILE_STYLE = LZOPENFILE_STYLE(0u16);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const OF_READWRITE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(2u16);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const OF_REOPEN: LZOPENFILE_STYLE = LZOPENFILE_STYLE(32768u16);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const OF_SHARE_DENY_NONE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(64u16);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const OF_SHARE_DENY_READ: LZOPENFILE_STYLE = LZOPENFILE_STYLE(48u16);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const OF_SHARE_DENY_WRITE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(32u16);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const OF_SHARE_EXCLUSIVE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(16u16);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const OF_WRITE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(1u16);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const OF_SHARE_COMPAT: LZOPENFILE_STYLE = LZOPENFILE_STYLE(0u16);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const OF_VERIFY: LZOPENFILE_STYLE = LZOPENFILE_STYLE(1024u16);
impl ::core::marker::Copy for LZOPENFILE_STYLE {}
impl ::core::clone::Clone for LZOPENFILE_STYLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LZOPENFILE_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for LZOPENFILE_STYLE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for LZOPENFILE_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LZOPENFILE_STYLE").field(&self.0).finish()
    }
}
impl LZOPENFILE_STYLE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for LZOPENFILE_STYLE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LZOPENFILE_STYLE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LZOPENFILE_STYLE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LZOPENFILE_STYLE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LZOPENFILE_STYLE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MOVE_FILE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const MOVEFILE_COPY_ALLOWED: MOVE_FILE_FLAGS = MOVE_FILE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const MOVEFILE_CREATE_HARDLINK: MOVE_FILE_FLAGS = MOVE_FILE_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const MOVEFILE_DELAY_UNTIL_REBOOT: MOVE_FILE_FLAGS = MOVE_FILE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const MOVEFILE_REPLACE_EXISTING: MOVE_FILE_FLAGS = MOVE_FILE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const MOVEFILE_WRITE_THROUGH: MOVE_FILE_FLAGS = MOVE_FILE_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const MOVEFILE_FAIL_IF_NOT_TRACKABLE: MOVE_FILE_FLAGS = MOVE_FILE_FLAGS(32u32);
impl ::core::marker::Copy for MOVE_FILE_FLAGS {}
impl ::core::clone::Clone for MOVE_FILE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MOVE_FILE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MOVE_FILE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MOVE_FILE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MOVE_FILE_FLAGS").field(&self.0).finish()
    }
}
impl MOVE_FILE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for MOVE_FILE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MOVE_FILE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MOVE_FILE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MOVE_FILE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MOVE_FILE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NTMS_OMID_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OMID_TYPE_FILESYSTEM_INFO: NTMS_OMID_TYPE = NTMS_OMID_TYPE(2u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OMID_TYPE_RAW_LABEL: NTMS_OMID_TYPE = NTMS_OMID_TYPE(1u32);
impl ::core::marker::Copy for NTMS_OMID_TYPE {}
impl ::core::clone::Clone for NTMS_OMID_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NTMS_OMID_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NTMS_OMID_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NTMS_OMID_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NTMS_OMID_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NT_CREATE_FILE_DISPOSITION(pub u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_SUPERSEDE: NT_CREATE_FILE_DISPOSITION = NT_CREATE_FILE_DISPOSITION(0u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_CREATE: NT_CREATE_FILE_DISPOSITION = NT_CREATE_FILE_DISPOSITION(2u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_OPEN: NT_CREATE_FILE_DISPOSITION = NT_CREATE_FILE_DISPOSITION(1u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_OPEN_IF: NT_CREATE_FILE_DISPOSITION = NT_CREATE_FILE_DISPOSITION(3u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_OVERWRITE: NT_CREATE_FILE_DISPOSITION = NT_CREATE_FILE_DISPOSITION(4u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_OVERWRITE_IF: NT_CREATE_FILE_DISPOSITION = NT_CREATE_FILE_DISPOSITION(5u32);
impl ::core::marker::Copy for NT_CREATE_FILE_DISPOSITION {}
impl ::core::clone::Clone for NT_CREATE_FILE_DISPOSITION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NT_CREATE_FILE_DISPOSITION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NT_CREATE_FILE_DISPOSITION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NT_CREATE_FILE_DISPOSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NT_CREATE_FILE_DISPOSITION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsAccessMask(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_USE_ACCESS: NtmsAccessMask = NtmsAccessMask(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MODIFY_ACCESS: NtmsAccessMask = NtmsAccessMask(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_CONTROL_ACCESS: NtmsAccessMask = NtmsAccessMask(4i32);
impl ::core::marker::Copy for NtmsAccessMask {}
impl ::core::clone::Clone for NtmsAccessMask {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsAccessMask {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsAccessMask {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsAccessMask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsAccessMask").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsAllocateOptions(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_ALLOCATE_NEW: NtmsAllocateOptions = NtmsAllocateOptions(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_ALLOCATE_NEXT: NtmsAllocateOptions = NtmsAllocateOptions(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_ALLOCATE_ERROR_IF_UNAVAILABLE: NtmsAllocateOptions = NtmsAllocateOptions(4i32);
impl ::core::marker::Copy for NtmsAllocateOptions {}
impl ::core::clone::Clone for NtmsAllocateOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsAllocateOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsAllocateOptions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsAllocateOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsAllocateOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsAllocationPolicy(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_ALLOCATE_FROMSCRATCH: NtmsAllocationPolicy = NtmsAllocationPolicy(1i32);
impl ::core::marker::Copy for NtmsAllocationPolicy {}
impl ::core::clone::Clone for NtmsAllocationPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsAllocationPolicy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsAllocationPolicy {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsAllocationPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsAllocationPolicy").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsAsyncOperations(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_ASYNCOP_MOUNT: NtmsAsyncOperations = NtmsAsyncOperations(1i32);
impl ::core::marker::Copy for NtmsAsyncOperations {}
impl ::core::clone::Clone for NtmsAsyncOperations {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsAsyncOperations {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsAsyncOperations {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsAsyncOperations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsAsyncOperations").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsAsyncStatus(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_ASYNCSTATE_QUEUED: NtmsAsyncStatus = NtmsAsyncStatus(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_ASYNCSTATE_WAIT_RESOURCE: NtmsAsyncStatus = NtmsAsyncStatus(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_ASYNCSTATE_WAIT_OPERATOR: NtmsAsyncStatus = NtmsAsyncStatus(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_ASYNCSTATE_INPROCESS: NtmsAsyncStatus = NtmsAsyncStatus(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_ASYNCSTATE_COMPLETE: NtmsAsyncStatus = NtmsAsyncStatus(4i32);
impl ::core::marker::Copy for NtmsAsyncStatus {}
impl ::core::clone::Clone for NtmsAsyncStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsAsyncStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsAsyncStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsAsyncStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsAsyncStatus").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsBarCodeState(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_BARCODESTATE_OK: NtmsBarCodeState = NtmsBarCodeState(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_BARCODESTATE_UNREADABLE: NtmsBarCodeState = NtmsBarCodeState(2i32);
impl ::core::marker::Copy for NtmsBarCodeState {}
impl ::core::clone::Clone for NtmsBarCodeState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsBarCodeState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsBarCodeState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsBarCodeState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsBarCodeState").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsCreateNtmsMediaOptions(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_ERROR_ON_DUPLICATE: NtmsCreateNtmsMediaOptions = NtmsCreateNtmsMediaOptions(1i32);
impl ::core::marker::Copy for NtmsCreateNtmsMediaOptions {}
impl ::core::clone::Clone for NtmsCreateNtmsMediaOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsCreateNtmsMediaOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsCreateNtmsMediaOptions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsCreateNtmsMediaOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsCreateNtmsMediaOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsCreateOptions(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPEN_EXISTING: NtmsCreateOptions = NtmsCreateOptions(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_CREATE_NEW: NtmsCreateOptions = NtmsCreateOptions(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPEN_ALWAYS: NtmsCreateOptions = NtmsCreateOptions(3i32);
impl ::core::marker::Copy for NtmsCreateOptions {}
impl ::core::clone::Clone for NtmsCreateOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsCreateOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsCreateOptions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsCreateOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsCreateOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsDeallocationPolicy(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_DEALLOCATE_TOSCRATCH: NtmsDeallocationPolicy = NtmsDeallocationPolicy(1i32);
impl ::core::marker::Copy for NtmsDeallocationPolicy {}
impl ::core::clone::Clone for NtmsDeallocationPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsDeallocationPolicy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsDeallocationPolicy {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsDeallocationPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsDeallocationPolicy").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsDismountOptions(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_DISMOUNT_DEFERRED: NtmsDismountOptions = NtmsDismountOptions(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_DISMOUNT_IMMEDIATE: NtmsDismountOptions = NtmsDismountOptions(2i32);
impl ::core::marker::Copy for NtmsDismountOptions {}
impl ::core::clone::Clone for NtmsDismountOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsDismountOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsDismountOptions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsDismountOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsDismountOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsDoorState(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_DOORSTATE_UNKNOWN: NtmsDoorState = NtmsDoorState(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_DOORSTATE_CLOSED: NtmsDoorState = NtmsDoorState(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_DOORSTATE_OPEN: NtmsDoorState = NtmsDoorState(2i32);
impl ::core::marker::Copy for NtmsDoorState {}
impl ::core::clone::Clone for NtmsDoorState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsDoorState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsDoorState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsDoorState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsDoorState").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsDriveState(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_DRIVESTATE_DISMOUNTED: NtmsDriveState = NtmsDriveState(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_DRIVESTATE_MOUNTED: NtmsDriveState = NtmsDriveState(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_DRIVESTATE_LOADED: NtmsDriveState = NtmsDriveState(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_DRIVESTATE_UNLOADED: NtmsDriveState = NtmsDriveState(5i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_DRIVESTATE_BEING_CLEANED: NtmsDriveState = NtmsDriveState(6i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_DRIVESTATE_DISMOUNTABLE: NtmsDriveState = NtmsDriveState(7i32);
impl ::core::marker::Copy for NtmsDriveState {}
impl ::core::clone::Clone for NtmsDriveState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsDriveState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsDriveState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsDriveState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsDriveState").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsDriveType(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_UNKNOWN_DRIVE: NtmsDriveType = NtmsDriveType(0i32);
impl ::core::marker::Copy for NtmsDriveType {}
impl ::core::clone::Clone for NtmsDriveType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsDriveType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsDriveType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsDriveType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsDriveType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsEjectOperation(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_EJECT_START: NtmsEjectOperation = NtmsEjectOperation(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_EJECT_STOP: NtmsEjectOperation = NtmsEjectOperation(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_EJECT_QUEUE: NtmsEjectOperation = NtmsEjectOperation(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_EJECT_FORCE: NtmsEjectOperation = NtmsEjectOperation(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_EJECT_IMMEDIATE: NtmsEjectOperation = NtmsEjectOperation(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_EJECT_ASK_USER: NtmsEjectOperation = NtmsEjectOperation(5i32);
impl ::core::marker::Copy for NtmsEjectOperation {}
impl ::core::clone::Clone for NtmsEjectOperation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsEjectOperation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsEjectOperation {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsEjectOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsEjectOperation").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsEnumerateOption(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_ENUM_DEFAULT: NtmsEnumerateOption = NtmsEnumerateOption(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_ENUM_ROOTPOOL: NtmsEnumerateOption = NtmsEnumerateOption(1i32);
impl ::core::marker::Copy for NtmsEnumerateOption {}
impl ::core::clone::Clone for NtmsEnumerateOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsEnumerateOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsEnumerateOption {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsEnumerateOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsEnumerateOption").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsInjectOperation(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_INJECT_START: NtmsInjectOperation = NtmsInjectOperation(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_INJECT_STOP: NtmsInjectOperation = NtmsInjectOperation(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_INJECT_RETRACT: NtmsInjectOperation = NtmsInjectOperation(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_INJECT_STARTMANY: NtmsInjectOperation = NtmsInjectOperation(3i32);
impl ::core::marker::Copy for NtmsInjectOperation {}
impl ::core::clone::Clone for NtmsInjectOperation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsInjectOperation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsInjectOperation {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsInjectOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsInjectOperation").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsInventoryMethod(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_INVENTORY_NONE: NtmsInventoryMethod = NtmsInventoryMethod(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_INVENTORY_FAST: NtmsInventoryMethod = NtmsInventoryMethod(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_INVENTORY_OMID: NtmsInventoryMethod = NtmsInventoryMethod(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_INVENTORY_DEFAULT: NtmsInventoryMethod = NtmsInventoryMethod(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_INVENTORY_SLOT: NtmsInventoryMethod = NtmsInventoryMethod(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_INVENTORY_STOP: NtmsInventoryMethod = NtmsInventoryMethod(5i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_INVENTORY_MAX: NtmsInventoryMethod = NtmsInventoryMethod(6i32);
impl ::core::marker::Copy for NtmsInventoryMethod {}
impl ::core::clone::Clone for NtmsInventoryMethod {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsInventoryMethod {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsInventoryMethod {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsInventoryMethod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsInventoryMethod").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsLibRequestFlags(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LIBREQFLAGS_NOAUTOPURGE: NtmsLibRequestFlags = NtmsLibRequestFlags(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LIBREQFLAGS_NOFAILEDPURGE: NtmsLibRequestFlags = NtmsLibRequestFlags(2i32);
impl ::core::marker::Copy for NtmsLibRequestFlags {}
impl ::core::clone::Clone for NtmsLibRequestFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsLibRequestFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsLibRequestFlags {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsLibRequestFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsLibRequestFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsLibraryFlags(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LIBRARYFLAG_FIXEDOFFLINE: NtmsLibraryFlags = NtmsLibraryFlags(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LIBRARYFLAG_CLEANERPRESENT: NtmsLibraryFlags = NtmsLibraryFlags(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LIBRARYFLAG_AUTODETECTCHANGE: NtmsLibraryFlags = NtmsLibraryFlags(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LIBRARYFLAG_IGNORECLEANERUSESREMAINING: NtmsLibraryFlags = NtmsLibraryFlags(8i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LIBRARYFLAG_RECOGNIZECLEANERBARCODE: NtmsLibraryFlags = NtmsLibraryFlags(16i32);
impl ::core::marker::Copy for NtmsLibraryFlags {}
impl ::core::clone::Clone for NtmsLibraryFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsLibraryFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsLibraryFlags {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsLibraryFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsLibraryFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsLibraryType(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LIBRARYTYPE_UNKNOWN: NtmsLibraryType = NtmsLibraryType(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LIBRARYTYPE_OFFLINE: NtmsLibraryType = NtmsLibraryType(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LIBRARYTYPE_ONLINE: NtmsLibraryType = NtmsLibraryType(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LIBRARYTYPE_STANDALONE: NtmsLibraryType = NtmsLibraryType(3i32);
impl ::core::marker::Copy for NtmsLibraryType {}
impl ::core::clone::Clone for NtmsLibraryType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsLibraryType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsLibraryType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsLibraryType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsLibraryType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsLmOperation(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_REMOVE: NtmsLmOperation = NtmsLmOperation(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_DISABLECHANGER: NtmsLmOperation = NtmsLmOperation(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_DISABLELIBRARY: NtmsLmOperation = NtmsLmOperation(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_ENABLECHANGER: NtmsLmOperation = NtmsLmOperation(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_ENABLELIBRARY: NtmsLmOperation = NtmsLmOperation(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_DISABLEDRIVE: NtmsLmOperation = NtmsLmOperation(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_ENABLEDRIVE: NtmsLmOperation = NtmsLmOperation(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_DISABLEMEDIA: NtmsLmOperation = NtmsLmOperation(5i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_ENABLEMEDIA: NtmsLmOperation = NtmsLmOperation(6i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_UPDATEOMID: NtmsLmOperation = NtmsLmOperation(7i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_INVENTORY: NtmsLmOperation = NtmsLmOperation(8i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_DOORACCESS: NtmsLmOperation = NtmsLmOperation(9i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_EJECT: NtmsLmOperation = NtmsLmOperation(10i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_EJECTCLEANER: NtmsLmOperation = NtmsLmOperation(11i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_INJECT: NtmsLmOperation = NtmsLmOperation(12i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_INJECTCLEANER: NtmsLmOperation = NtmsLmOperation(13i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_PROCESSOMID: NtmsLmOperation = NtmsLmOperation(14i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_CLEANDRIVE: NtmsLmOperation = NtmsLmOperation(15i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_DISMOUNT: NtmsLmOperation = NtmsLmOperation(16i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_MOUNT: NtmsLmOperation = NtmsLmOperation(17i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_WRITESCRATCH: NtmsLmOperation = NtmsLmOperation(18i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_CLASSIFY: NtmsLmOperation = NtmsLmOperation(19i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_RESERVECLEANER: NtmsLmOperation = NtmsLmOperation(20i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_RELEASECLEANER: NtmsLmOperation = NtmsLmOperation(21i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_MAXWORKITEM: NtmsLmOperation = NtmsLmOperation(22i32);
impl ::core::marker::Copy for NtmsLmOperation {}
impl ::core::clone::Clone for NtmsLmOperation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsLmOperation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsLmOperation {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsLmOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsLmOperation").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsLmState(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_QUEUED: NtmsLmState = NtmsLmState(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_INPROCESS: NtmsLmState = NtmsLmState(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_PASSED: NtmsLmState = NtmsLmState(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_FAILED: NtmsLmState = NtmsLmState(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_INVALID: NtmsLmState = NtmsLmState(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_WAITING: NtmsLmState = NtmsLmState(5i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_DEFERRED: NtmsLmState = NtmsLmState(6i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_DEFFERED: NtmsLmState = NtmsLmState(6i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_CANCELLED: NtmsLmState = NtmsLmState(7i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_STOPPED: NtmsLmState = NtmsLmState(8i32);
impl ::core::marker::Copy for NtmsLmState {}
impl ::core::clone::Clone for NtmsLmState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsLmState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsLmState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsLmState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsLmState").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsMediaPoolPolicy(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_POOLPOLICY_PURGEOFFLINESCRATCH: NtmsMediaPoolPolicy = NtmsMediaPoolPolicy(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_POOLPOLICY_KEEPOFFLINEIMPORT: NtmsMediaPoolPolicy = NtmsMediaPoolPolicy(2i32);
impl ::core::marker::Copy for NtmsMediaPoolPolicy {}
impl ::core::clone::Clone for NtmsMediaPoolPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsMediaPoolPolicy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsMediaPoolPolicy {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsMediaPoolPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsMediaPoolPolicy").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsMediaState(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MEDIASTATE_IDLE: NtmsMediaState = NtmsMediaState(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MEDIASTATE_INUSE: NtmsMediaState = NtmsMediaState(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MEDIASTATE_MOUNTED: NtmsMediaState = NtmsMediaState(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MEDIASTATE_LOADED: NtmsMediaState = NtmsMediaState(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MEDIASTATE_UNLOADED: NtmsMediaState = NtmsMediaState(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MEDIASTATE_OPERROR: NtmsMediaState = NtmsMediaState(5i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MEDIASTATE_OPREQ: NtmsMediaState = NtmsMediaState(6i32);
impl ::core::marker::Copy for NtmsMediaState {}
impl ::core::clone::Clone for NtmsMediaState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsMediaState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsMediaState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsMediaState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsMediaState").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsMountOptions(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MOUNT_READ: NtmsMountOptions = NtmsMountOptions(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MOUNT_WRITE: NtmsMountOptions = NtmsMountOptions(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MOUNT_ERROR_NOT_AVAILABLE: NtmsMountOptions = NtmsMountOptions(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MOUNT_ERROR_IF_UNAVAILABLE: NtmsMountOptions = NtmsMountOptions(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MOUNT_ERROR_OFFLINE: NtmsMountOptions = NtmsMountOptions(8i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MOUNT_ERROR_IF_OFFLINE: NtmsMountOptions = NtmsMountOptions(8i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MOUNT_SPECIFIC_DRIVE: NtmsMountOptions = NtmsMountOptions(16i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MOUNT_NOWAIT: NtmsMountOptions = NtmsMountOptions(32i32);
impl ::core::marker::Copy for NtmsMountOptions {}
impl ::core::clone::Clone for NtmsMountOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsMountOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsMountOptions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsMountOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsMountOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsMountPriority(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PRIORITY_DEFAULT: NtmsMountPriority = NtmsMountPriority(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PRIORITY_HIGHEST: NtmsMountPriority = NtmsMountPriority(15i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PRIORITY_HIGH: NtmsMountPriority = NtmsMountPriority(7i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PRIORITY_NORMAL: NtmsMountPriority = NtmsMountPriority(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PRIORITY_LOW: NtmsMountPriority = NtmsMountPriority(-7i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PRIORITY_LOWEST: NtmsMountPriority = NtmsMountPriority(-15i32);
impl ::core::marker::Copy for NtmsMountPriority {}
impl ::core::clone::Clone for NtmsMountPriority {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsMountPriority {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsMountPriority {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsMountPriority {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsMountPriority").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsNotificationOperations(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OBJ_UPDATE: NtmsNotificationOperations = NtmsNotificationOperations(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OBJ_INSERT: NtmsNotificationOperations = NtmsNotificationOperations(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OBJ_DELETE: NtmsNotificationOperations = NtmsNotificationOperations(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_EVENT_SIGNAL: NtmsNotificationOperations = NtmsNotificationOperations(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_EVENT_COMPLETE: NtmsNotificationOperations = NtmsNotificationOperations(5i32);
impl ::core::marker::Copy for NtmsNotificationOperations {}
impl ::core::clone::Clone for NtmsNotificationOperations {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsNotificationOperations {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsNotificationOperations {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsNotificationOperations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsNotificationOperations").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsObjectsTypes(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_UNKNOWN: NtmsObjectsTypes = NtmsObjectsTypes(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OBJECT: NtmsObjectsTypes = NtmsObjectsTypes(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_CHANGER: NtmsObjectsTypes = NtmsObjectsTypes(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_CHANGER_TYPE: NtmsObjectsTypes = NtmsObjectsTypes(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_COMPUTER: NtmsObjectsTypes = NtmsObjectsTypes(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_DRIVE: NtmsObjectsTypes = NtmsObjectsTypes(5i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_DRIVE_TYPE: NtmsObjectsTypes = NtmsObjectsTypes(6i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_IEDOOR: NtmsObjectsTypes = NtmsObjectsTypes(7i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_IEPORT: NtmsObjectsTypes = NtmsObjectsTypes(8i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LIBRARY: NtmsObjectsTypes = NtmsObjectsTypes(9i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LIBREQUEST: NtmsObjectsTypes = NtmsObjectsTypes(10i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LOGICAL_MEDIA: NtmsObjectsTypes = NtmsObjectsTypes(11i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MEDIA_POOL: NtmsObjectsTypes = NtmsObjectsTypes(12i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MEDIA_TYPE: NtmsObjectsTypes = NtmsObjectsTypes(13i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PARTITION: NtmsObjectsTypes = NtmsObjectsTypes(14i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PHYSICAL_MEDIA: NtmsObjectsTypes = NtmsObjectsTypes(15i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_STORAGESLOT: NtmsObjectsTypes = NtmsObjectsTypes(16i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPREQUEST: NtmsObjectsTypes = NtmsObjectsTypes(17i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_UI_DESTINATION: NtmsObjectsTypes = NtmsObjectsTypes(18i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_NUMBER_OF_OBJECT_TYPES: NtmsObjectsTypes = NtmsObjectsTypes(19i32);
impl ::core::marker::Copy for NtmsObjectsTypes {}
impl ::core::clone::Clone for NtmsObjectsTypes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsObjectsTypes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsObjectsTypes {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsObjectsTypes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsObjectsTypes").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsOpRequestFlags(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPREQFLAGS_NOAUTOPURGE: NtmsOpRequestFlags = NtmsOpRequestFlags(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPREQFLAGS_NOFAILEDPURGE: NtmsOpRequestFlags = NtmsOpRequestFlags(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPREQFLAGS_NOALERTS: NtmsOpRequestFlags = NtmsOpRequestFlags(16i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPREQFLAGS_NOTRAYICON: NtmsOpRequestFlags = NtmsOpRequestFlags(32i32);
impl ::core::marker::Copy for NtmsOpRequestFlags {}
impl ::core::clone::Clone for NtmsOpRequestFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsOpRequestFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsOpRequestFlags {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsOpRequestFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsOpRequestFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsOperationalState(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_READY: NtmsOperationalState = NtmsOperationalState(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_INITIALIZING: NtmsOperationalState = NtmsOperationalState(10i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_NEEDS_SERVICE: NtmsOperationalState = NtmsOperationalState(20i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_NOT_PRESENT: NtmsOperationalState = NtmsOperationalState(21i32);
impl ::core::marker::Copy for NtmsOperationalState {}
impl ::core::clone::Clone for NtmsOperationalState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsOperationalState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsOperationalState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsOperationalState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsOperationalState").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsOpreqCommand(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPREQ_UNKNOWN: NtmsOpreqCommand = NtmsOpreqCommand(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPREQ_NEWMEDIA: NtmsOpreqCommand = NtmsOpreqCommand(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPREQ_CLEANER: NtmsOpreqCommand = NtmsOpreqCommand(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPREQ_DEVICESERVICE: NtmsOpreqCommand = NtmsOpreqCommand(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPREQ_MOVEMEDIA: NtmsOpreqCommand = NtmsOpreqCommand(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPREQ_MESSAGE: NtmsOpreqCommand = NtmsOpreqCommand(5i32);
impl ::core::marker::Copy for NtmsOpreqCommand {}
impl ::core::clone::Clone for NtmsOpreqCommand {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsOpreqCommand {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsOpreqCommand {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsOpreqCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsOpreqCommand").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsOpreqState(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPSTATE_UNKNOWN: NtmsOpreqState = NtmsOpreqState(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPSTATE_SUBMITTED: NtmsOpreqState = NtmsOpreqState(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPSTATE_ACTIVE: NtmsOpreqState = NtmsOpreqState(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPSTATE_INPROGRESS: NtmsOpreqState = NtmsOpreqState(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPSTATE_REFUSED: NtmsOpreqState = NtmsOpreqState(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPSTATE_COMPLETE: NtmsOpreqState = NtmsOpreqState(5i32);
impl ::core::marker::Copy for NtmsOpreqState {}
impl ::core::clone::Clone for NtmsOpreqState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsOpreqState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsOpreqState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsOpreqState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsOpreqState").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsPartitionState(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PARTSTATE_UNKNOWN: NtmsPartitionState = NtmsPartitionState(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PARTSTATE_UNPREPARED: NtmsPartitionState = NtmsPartitionState(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PARTSTATE_INCOMPATIBLE: NtmsPartitionState = NtmsPartitionState(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PARTSTATE_DECOMMISSIONED: NtmsPartitionState = NtmsPartitionState(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PARTSTATE_AVAILABLE: NtmsPartitionState = NtmsPartitionState(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PARTSTATE_ALLOCATED: NtmsPartitionState = NtmsPartitionState(5i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PARTSTATE_COMPLETE: NtmsPartitionState = NtmsPartitionState(6i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PARTSTATE_FOREIGN: NtmsPartitionState = NtmsPartitionState(7i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PARTSTATE_IMPORT: NtmsPartitionState = NtmsPartitionState(8i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PARTSTATE_RESERVED: NtmsPartitionState = NtmsPartitionState(9i32);
impl ::core::marker::Copy for NtmsPartitionState {}
impl ::core::clone::Clone for NtmsPartitionState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsPartitionState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsPartitionState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsPartitionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsPartitionState").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsPoolType(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_POOLTYPE_UNKNOWN: NtmsPoolType = NtmsPoolType(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_POOLTYPE_SCRATCH: NtmsPoolType = NtmsPoolType(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_POOLTYPE_FOREIGN: NtmsPoolType = NtmsPoolType(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_POOLTYPE_IMPORT: NtmsPoolType = NtmsPoolType(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_POOLTYPE_APPLICATION: NtmsPoolType = NtmsPoolType(1000i32);
impl ::core::marker::Copy for NtmsPoolType {}
impl ::core::clone::Clone for NtmsPoolType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsPoolType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsPoolType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsPoolType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsPoolType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsPortContent(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PORTCONTENT_UNKNOWN: NtmsPortContent = NtmsPortContent(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PORTCONTENT_FULL: NtmsPortContent = NtmsPortContent(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PORTCONTENT_EMPTY: NtmsPortContent = NtmsPortContent(2i32);
impl ::core::marker::Copy for NtmsPortContent {}
impl ::core::clone::Clone for NtmsPortContent {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsPortContent {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsPortContent {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsPortContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsPortContent").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsPortPosition(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PORTPOSITION_UNKNOWN: NtmsPortPosition = NtmsPortPosition(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PORTPOSITION_EXTENDED: NtmsPortPosition = NtmsPortPosition(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PORTPOSITION_RETRACTED: NtmsPortPosition = NtmsPortPosition(2i32);
impl ::core::marker::Copy for NtmsPortPosition {}
impl ::core::clone::Clone for NtmsPortPosition {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsPortPosition {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsPortPosition {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsPortPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsPortPosition").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsReadWriteCharacteristics(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MEDIARW_UNKNOWN: NtmsReadWriteCharacteristics = NtmsReadWriteCharacteristics(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MEDIARW_REWRITABLE: NtmsReadWriteCharacteristics = NtmsReadWriteCharacteristics(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MEDIARW_WRITEONCE: NtmsReadWriteCharacteristics = NtmsReadWriteCharacteristics(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MEDIARW_READONLY: NtmsReadWriteCharacteristics = NtmsReadWriteCharacteristics(3i32);
impl ::core::marker::Copy for NtmsReadWriteCharacteristics {}
impl ::core::clone::Clone for NtmsReadWriteCharacteristics {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsReadWriteCharacteristics {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsReadWriteCharacteristics {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsReadWriteCharacteristics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsReadWriteCharacteristics").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsSessionOptions(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_SESSION_QUERYEXPEDITE: NtmsSessionOptions = NtmsSessionOptions(1i32);
impl ::core::marker::Copy for NtmsSessionOptions {}
impl ::core::clone::Clone for NtmsSessionOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsSessionOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsSessionOptions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsSessionOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsSessionOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsSlotState(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_SLOTSTATE_UNKNOWN: NtmsSlotState = NtmsSlotState(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_SLOTSTATE_FULL: NtmsSlotState = NtmsSlotState(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_SLOTSTATE_EMPTY: NtmsSlotState = NtmsSlotState(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_SLOTSTATE_NOTPRESENT: NtmsSlotState = NtmsSlotState(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_SLOTSTATE_NEEDSINVENTORY: NtmsSlotState = NtmsSlotState(4i32);
impl ::core::marker::Copy for NtmsSlotState {}
impl ::core::clone::Clone for NtmsSlotState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsSlotState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsSlotState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsSlotState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsSlotState").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsUIOperations(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_UIDEST_ADD: NtmsUIOperations = NtmsUIOperations(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_UIDEST_DELETE: NtmsUIOperations = NtmsUIOperations(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_UIDEST_DELETEALL: NtmsUIOperations = NtmsUIOperations(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_UIOPERATION_MAX: NtmsUIOperations = NtmsUIOperations(4i32);
impl ::core::marker::Copy for NtmsUIOperations {}
impl ::core::clone::Clone for NtmsUIOperations {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsUIOperations {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsUIOperations {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsUIOperations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsUIOperations").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NtmsUITypes(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_UITYPE_INVALID: NtmsUITypes = NtmsUITypes(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_UITYPE_INFO: NtmsUITypes = NtmsUITypes(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_UITYPE_REQ: NtmsUITypes = NtmsUITypes(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_UITYPE_ERR: NtmsUITypes = NtmsUITypes(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_UITYPE_MAX: NtmsUITypes = NtmsUITypes(4i32);
impl ::core::marker::Copy for NtmsUITypes {}
impl ::core::clone::Clone for NtmsUITypes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NtmsUITypes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NtmsUITypes {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NtmsUITypes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsUITypes").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PREPARE_TAPE_OPERATION(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_FORMAT: PREPARE_TAPE_OPERATION = PREPARE_TAPE_OPERATION(5i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_LOAD: PREPARE_TAPE_OPERATION = PREPARE_TAPE_OPERATION(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_LOCK: PREPARE_TAPE_OPERATION = PREPARE_TAPE_OPERATION(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_TENSION: PREPARE_TAPE_OPERATION = PREPARE_TAPE_OPERATION(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_UNLOAD: PREPARE_TAPE_OPERATION = PREPARE_TAPE_OPERATION(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_UNLOCK: PREPARE_TAPE_OPERATION = PREPARE_TAPE_OPERATION(4i32);
impl ::core::marker::Copy for PREPARE_TAPE_OPERATION {}
impl ::core::clone::Clone for PREPARE_TAPE_OPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PREPARE_TAPE_OPERATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PREPARE_TAPE_OPERATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PREPARE_TAPE_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PREPARE_TAPE_OPERATION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PRIORITY_HINT(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IoPriorityHintVeryLow: PRIORITY_HINT = PRIORITY_HINT(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IoPriorityHintLow: PRIORITY_HINT = PRIORITY_HINT(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IoPriorityHintNormal: PRIORITY_HINT = PRIORITY_HINT(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const MaximumIoPriorityHintType: PRIORITY_HINT = PRIORITY_HINT(3i32);
impl ::core::marker::Copy for PRIORITY_HINT {}
impl ::core::clone::Clone for PRIORITY_HINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PRIORITY_HINT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PRIORITY_HINT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PRIORITY_HINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PRIORITY_HINT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct READ_DIRECTORY_NOTIFY_INFORMATION_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ReadDirectoryNotifyInformation: READ_DIRECTORY_NOTIFY_INFORMATION_CLASS = READ_DIRECTORY_NOTIFY_INFORMATION_CLASS(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ReadDirectoryNotifyExtendedInformation: READ_DIRECTORY_NOTIFY_INFORMATION_CLASS = READ_DIRECTORY_NOTIFY_INFORMATION_CLASS(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ReadDirectoryNotifyFullInformation: READ_DIRECTORY_NOTIFY_INFORMATION_CLASS = READ_DIRECTORY_NOTIFY_INFORMATION_CLASS(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ReadDirectoryNotifyMaximumInformation: READ_DIRECTORY_NOTIFY_INFORMATION_CLASS = READ_DIRECTORY_NOTIFY_INFORMATION_CLASS(4i32);
impl ::core::marker::Copy for READ_DIRECTORY_NOTIFY_INFORMATION_CLASS {}
impl ::core::clone::Clone for READ_DIRECTORY_NOTIFY_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for READ_DIRECTORY_NOTIFY_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for READ_DIRECTORY_NOTIFY_INFORMATION_CLASS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for READ_DIRECTORY_NOTIFY_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("READ_DIRECTORY_NOTIFY_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct REPLACE_FILE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const REPLACEFILE_WRITE_THROUGH: REPLACE_FILE_FLAGS = REPLACE_FILE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const REPLACEFILE_IGNORE_MERGE_ERRORS: REPLACE_FILE_FLAGS = REPLACE_FILE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const REPLACEFILE_IGNORE_ACL_ERRORS: REPLACE_FILE_FLAGS = REPLACE_FILE_FLAGS(4u32);
impl ::core::marker::Copy for REPLACE_FILE_FLAGS {}
impl ::core::clone::Clone for REPLACE_FILE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REPLACE_FILE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for REPLACE_FILE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for REPLACE_FILE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REPLACE_FILE_FLAGS").field(&self.0).finish()
    }
}
impl REPLACE_FILE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for REPLACE_FILE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for REPLACE_FILE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for REPLACE_FILE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for REPLACE_FILE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for REPLACE_FILE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SERVER_CERTIFICATE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const QUIC: SERVER_CERTIFICATE_TYPE = SERVER_CERTIFICATE_TYPE(0i32);
impl ::core::marker::Copy for SERVER_CERTIFICATE_TYPE {}
impl ::core::clone::Clone for SERVER_CERTIFICATE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SERVER_CERTIFICATE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SERVER_CERTIFICATE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SERVER_CERTIFICATE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SERVER_CERTIFICATE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SESSION_INFO_USER_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SESS_GUEST: SESSION_INFO_USER_FLAGS = SESSION_INFO_USER_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SESS_NOENCRYPTION: SESSION_INFO_USER_FLAGS = SESSION_INFO_USER_FLAGS(2u32);
impl ::core::marker::Copy for SESSION_INFO_USER_FLAGS {}
impl ::core::clone::Clone for SESSION_INFO_USER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SESSION_INFO_USER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SESSION_INFO_USER_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SESSION_INFO_USER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SESSION_INFO_USER_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SET_FILE_POINTER_MOVE_METHOD(pub u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_BEGIN: SET_FILE_POINTER_MOVE_METHOD = SET_FILE_POINTER_MOVE_METHOD(0u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_CURRENT: SET_FILE_POINTER_MOVE_METHOD = SET_FILE_POINTER_MOVE_METHOD(1u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_END: SET_FILE_POINTER_MOVE_METHOD = SET_FILE_POINTER_MOVE_METHOD(2u32);
impl ::core::marker::Copy for SET_FILE_POINTER_MOVE_METHOD {}
impl ::core::clone::Clone for SET_FILE_POINTER_MOVE_METHOD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SET_FILE_POINTER_MOVE_METHOD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SET_FILE_POINTER_MOVE_METHOD {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SET_FILE_POINTER_MOVE_METHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SET_FILE_POINTER_MOVE_METHOD").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SHARE_INFO_PERMISSIONS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ACCESS_READ: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(1u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ACCESS_WRITE: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(2u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ACCESS_CREATE: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(4u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ACCESS_EXEC: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(8u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ACCESS_DELETE: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(16u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ACCESS_ATRIB: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(32u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ACCESS_PERM: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(64u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ACCESS_ALL: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(32768u32);
impl ::core::marker::Copy for SHARE_INFO_PERMISSIONS {}
impl ::core::clone::Clone for SHARE_INFO_PERMISSIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SHARE_INFO_PERMISSIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SHARE_INFO_PERMISSIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SHARE_INFO_PERMISSIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHARE_INFO_PERMISSIONS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SHARE_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STYPE_DISKTREE: SHARE_TYPE = SHARE_TYPE(0u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STYPE_PRINTQ: SHARE_TYPE = SHARE_TYPE(1u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STYPE_DEVICE: SHARE_TYPE = SHARE_TYPE(2u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STYPE_IPC: SHARE_TYPE = SHARE_TYPE(3u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STYPE_SPECIAL: SHARE_TYPE = SHARE_TYPE(2147483648u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STYPE_TEMPORARY: SHARE_TYPE = SHARE_TYPE(1073741824u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STYPE_MASK: SHARE_TYPE = SHARE_TYPE(255u32);
impl ::core::marker::Copy for SHARE_TYPE {}
impl ::core::clone::Clone for SHARE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SHARE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SHARE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SHARE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHARE_TYPE").field(&self.0).finish()
    }
}
impl SHARE_TYPE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for SHARE_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SHARE_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SHARE_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SHARE_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SHARE_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct STORAGE_BUS_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeUnknown: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeScsi: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeAtapi: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeAta: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusType1394: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeSsa: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeFibre: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeUsb: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeRAID: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeiScsi: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeSas: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeSata: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeSd: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeMmc: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(13i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeVirtual: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(14i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeFileBackedVirtual: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(15i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeSpaces: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(16i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeNvme: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(17i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeSCM: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(18i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeUfs: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(19i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeMax: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(20i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeMaxReserved: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(127i32);
impl ::core::marker::Copy for STORAGE_BUS_TYPE {}
impl ::core::clone::Clone for STORAGE_BUS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_BUS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for STORAGE_BUS_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for STORAGE_BUS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_BUS_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct STREAM_INFO_LEVELS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FindStreamInfoStandard: STREAM_INFO_LEVELS = STREAM_INFO_LEVELS(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FindStreamInfoMaxInfoLevel: STREAM_INFO_LEVELS = STREAM_INFO_LEVELS(1i32);
impl ::core::marker::Copy for STREAM_INFO_LEVELS {}
impl ::core::clone::Clone for STREAM_INFO_LEVELS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STREAM_INFO_LEVELS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for STREAM_INFO_LEVELS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for STREAM_INFO_LEVELS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STREAM_INFO_LEVELS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SYMBOLIC_LINK_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SYMBOLIC_LINK_FLAG_DIRECTORY: SYMBOLIC_LINK_FLAGS = SYMBOLIC_LINK_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SYMBOLIC_LINK_FLAG_ALLOW_UNPRIVILEGED_CREATE: SYMBOLIC_LINK_FLAGS = SYMBOLIC_LINK_FLAGS(2u32);
impl ::core::marker::Copy for SYMBOLIC_LINK_FLAGS {}
impl ::core::clone::Clone for SYMBOLIC_LINK_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYMBOLIC_LINK_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SYMBOLIC_LINK_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SYMBOLIC_LINK_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYMBOLIC_LINK_FLAGS").field(&self.0).finish()
    }
}
impl SYMBOLIC_LINK_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for SYMBOLIC_LINK_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SYMBOLIC_LINK_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SYMBOLIC_LINK_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SYMBOLIC_LINK_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SYMBOLIC_LINK_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TAPEMARK_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_FILEMARKS: TAPEMARK_TYPE = TAPEMARK_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_LONG_FILEMARKS: TAPEMARK_TYPE = TAPEMARK_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_SETMARKS: TAPEMARK_TYPE = TAPEMARK_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_SHORT_FILEMARKS: TAPEMARK_TYPE = TAPEMARK_TYPE(2i32);
impl ::core::marker::Copy for TAPEMARK_TYPE {}
impl ::core::clone::Clone for TAPEMARK_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TAPEMARK_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TAPEMARK_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TAPEMARK_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TAPEMARK_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TAPE_INFORMATION_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SET_TAPE_DRIVE_INFORMATION: TAPE_INFORMATION_TYPE = TAPE_INFORMATION_TYPE(1u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SET_TAPE_MEDIA_INFORMATION: TAPE_INFORMATION_TYPE = TAPE_INFORMATION_TYPE(0u32);
impl ::core::marker::Copy for TAPE_INFORMATION_TYPE {}
impl ::core::clone::Clone for TAPE_INFORMATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TAPE_INFORMATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TAPE_INFORMATION_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TAPE_INFORMATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TAPE_INFORMATION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TAPE_POSITION_METHOD(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_ABSOLUTE_BLOCK: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_LOGICAL_BLOCK: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_REWIND: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_SPACE_END_OF_DATA: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_SPACE_FILEMARKS: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(6i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_SPACE_RELATIVE_BLOCKS: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(5i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_SPACE_SEQUENTIAL_FMKS: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(7i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_SPACE_SEQUENTIAL_SMKS: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(9i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_SPACE_SETMARKS: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(8i32);
impl ::core::marker::Copy for TAPE_POSITION_METHOD {}
impl ::core::clone::Clone for TAPE_POSITION_METHOD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TAPE_POSITION_METHOD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TAPE_POSITION_METHOD {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TAPE_POSITION_METHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TAPE_POSITION_METHOD").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TAPE_POSITION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_ABSOLUTE_POSITION: TAPE_POSITION_TYPE = TAPE_POSITION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_LOGICAL_POSITION: TAPE_POSITION_TYPE = TAPE_POSITION_TYPE(1i32);
impl ::core::marker::Copy for TAPE_POSITION_TYPE {}
impl ::core::clone::Clone for TAPE_POSITION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TAPE_POSITION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TAPE_POSITION_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TAPE_POSITION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TAPE_POSITION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TRANSACTION_OUTCOME(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TransactionOutcomeUndetermined: TRANSACTION_OUTCOME = TRANSACTION_OUTCOME(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TransactionOutcomeCommitted: TRANSACTION_OUTCOME = TRANSACTION_OUTCOME(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TransactionOutcomeAborted: TRANSACTION_OUTCOME = TRANSACTION_OUTCOME(3i32);
impl ::core::marker::Copy for TRANSACTION_OUTCOME {}
impl ::core::clone::Clone for TRANSACTION_OUTCOME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TRANSACTION_OUTCOME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TRANSACTION_OUTCOME {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TRANSACTION_OUTCOME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRANSACTION_OUTCOME").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TXFS_MINIVERSION(pub u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TXFS_MINIVERSION_COMMITTED_VIEW: TXFS_MINIVERSION = TXFS_MINIVERSION(0u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TXFS_MINIVERSION_DIRTY_VIEW: TXFS_MINIVERSION = TXFS_MINIVERSION(65535u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TXFS_MINIVERSION_DEFAULT_VIEW: TXFS_MINIVERSION = TXFS_MINIVERSION(65534u32);
impl ::core::marker::Copy for TXFS_MINIVERSION {}
impl ::core::clone::Clone for TXFS_MINIVERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TXFS_MINIVERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TXFS_MINIVERSION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TXFS_MINIVERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TXFS_MINIVERSION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TXF_LOG_RECORD_TYPE(pub u16);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TXF_LOG_RECORD_TYPE_AFFECTED_FILE: TXF_LOG_RECORD_TYPE = TXF_LOG_RECORD_TYPE(4u16);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TXF_LOG_RECORD_TYPE_TRUNCATE: TXF_LOG_RECORD_TYPE = TXF_LOG_RECORD_TYPE(2u16);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TXF_LOG_RECORD_TYPE_WRITE: TXF_LOG_RECORD_TYPE = TXF_LOG_RECORD_TYPE(1u16);
impl ::core::marker::Copy for TXF_LOG_RECORD_TYPE {}
impl ::core::clone::Clone for TXF_LOG_RECORD_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TXF_LOG_RECORD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TXF_LOG_RECORD_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TXF_LOG_RECORD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TXF_LOG_RECORD_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VER_FIND_FILE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFFF_ISSHAREDFILE: VER_FIND_FILE_FLAGS = VER_FIND_FILE_FLAGS(1u32);
impl ::core::marker::Copy for VER_FIND_FILE_FLAGS {}
impl ::core::clone::Clone for VER_FIND_FILE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VER_FIND_FILE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VER_FIND_FILE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VER_FIND_FILE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VER_FIND_FILE_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VER_FIND_FILE_STATUS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFF_CURNEDEST: VER_FIND_FILE_STATUS = VER_FIND_FILE_STATUS(1u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFF_FILEINUSE: VER_FIND_FILE_STATUS = VER_FIND_FILE_STATUS(2u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFF_BUFFTOOSMALL: VER_FIND_FILE_STATUS = VER_FIND_FILE_STATUS(4u32);
impl ::core::marker::Copy for VER_FIND_FILE_STATUS {}
impl ::core::clone::Clone for VER_FIND_FILE_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VER_FIND_FILE_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VER_FIND_FILE_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VER_FIND_FILE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VER_FIND_FILE_STATUS").field(&self.0).finish()
    }
}
impl VER_FIND_FILE_STATUS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for VER_FIND_FILE_STATUS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for VER_FIND_FILE_STATUS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for VER_FIND_FILE_STATUS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for VER_FIND_FILE_STATUS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for VER_FIND_FILE_STATUS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VER_INSTALL_FILE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIFF_FORCEINSTALL: VER_INSTALL_FILE_FLAGS = VER_INSTALL_FILE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIFF_DONTDELETEOLD: VER_INSTALL_FILE_FLAGS = VER_INSTALL_FILE_FLAGS(2u32);
impl ::core::marker::Copy for VER_INSTALL_FILE_FLAGS {}
impl ::core::clone::Clone for VER_INSTALL_FILE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VER_INSTALL_FILE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VER_INSTALL_FILE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VER_INSTALL_FILE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VER_INSTALL_FILE_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VER_INSTALL_FILE_STATUS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_TEMPFILE: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(1u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_MISMATCH: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(2u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_SRCOLD: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(4u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_DIFFLANG: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(8u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_DIFFCODEPG: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(16u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_DIFFTYPE: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(32u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_WRITEPROT: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(64u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_FILEINUSE: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(128u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_OUTOFSPACE: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(256u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_ACCESSVIOLATION: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(512u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_SHARINGVIOLATION: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(1024u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_CANNOTCREATE: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(2048u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_CANNOTDELETE: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(4096u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_CANNOTRENAME: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(8192u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_CANNOTDELETECUR: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(16384u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_OUTOFMEMORY: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(32768u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_CANNOTREADSRC: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(65536u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_CANNOTREADDST: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(131072u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_BUFFTOOSMALL: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(262144u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_CANNOTLOADLZ32: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(524288u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_CANNOTLOADCABINET: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(1048576u32);
impl ::core::marker::Copy for VER_INSTALL_FILE_STATUS {}
impl ::core::clone::Clone for VER_INSTALL_FILE_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VER_INSTALL_FILE_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VER_INSTALL_FILE_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VER_INSTALL_FILE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VER_INSTALL_FILE_STATUS").field(&self.0).finish()
    }
}
impl VER_INSTALL_FILE_STATUS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for VER_INSTALL_FILE_STATUS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for VER_INSTALL_FILE_STATUS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for VER_INSTALL_FILE_STATUS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for VER_INSTALL_FILE_STATUS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for VER_INSTALL_FILE_STATUS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VS_FIXEDFILEINFO_FILE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VS_FF_DEBUG: VS_FIXEDFILEINFO_FILE_FLAGS = VS_FIXEDFILEINFO_FILE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VS_FF_PRERELEASE: VS_FIXEDFILEINFO_FILE_FLAGS = VS_FIXEDFILEINFO_FILE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VS_FF_PATCHED: VS_FIXEDFILEINFO_FILE_FLAGS = VS_FIXEDFILEINFO_FILE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VS_FF_PRIVATEBUILD: VS_FIXEDFILEINFO_FILE_FLAGS = VS_FIXEDFILEINFO_FILE_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VS_FF_INFOINFERRED: VS_FIXEDFILEINFO_FILE_FLAGS = VS_FIXEDFILEINFO_FILE_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VS_FF_SPECIALBUILD: VS_FIXEDFILEINFO_FILE_FLAGS = VS_FIXEDFILEINFO_FILE_FLAGS(32u32);
impl ::core::marker::Copy for VS_FIXEDFILEINFO_FILE_FLAGS {}
impl ::core::clone::Clone for VS_FIXEDFILEINFO_FILE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VS_FIXEDFILEINFO_FILE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VS_FIXEDFILEINFO_FILE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VS_FIXEDFILEINFO_FILE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VS_FIXEDFILEINFO_FILE_FLAGS").field(&self.0).finish()
    }
}
impl VS_FIXEDFILEINFO_FILE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for VS_FIXEDFILEINFO_FILE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for VS_FIXEDFILEINFO_FILE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for VS_FIXEDFILEINFO_FILE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for VS_FIXEDFILEINFO_FILE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for VS_FIXEDFILEINFO_FILE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VS_FIXEDFILEINFO_FILE_OS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VOS_UNKNOWN: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VOS_DOS: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(65536i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VOS_OS216: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(131072i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VOS_OS232: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(196608i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VOS_NT: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(262144i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VOS_WINCE: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(327680i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VOS__BASE: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VOS__WINDOWS16: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VOS__PM16: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VOS__PM32: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VOS__WINDOWS32: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VOS_DOS_WINDOWS16: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(65537i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VOS_DOS_WINDOWS32: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(65540i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VOS_OS216_PM16: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(131074i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VOS_OS232_PM32: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(196611i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VOS_NT_WINDOWS32: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(262148i32);
impl ::core::marker::Copy for VS_FIXEDFILEINFO_FILE_OS {}
impl ::core::clone::Clone for VS_FIXEDFILEINFO_FILE_OS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VS_FIXEDFILEINFO_FILE_OS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VS_FIXEDFILEINFO_FILE_OS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VS_FIXEDFILEINFO_FILE_OS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VS_FIXEDFILEINFO_FILE_OS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VS_FIXEDFILEINFO_FILE_SUBTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT2_UNKNOWN: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT2_DRV_PRINTER: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT2_DRV_KEYBOARD: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT2_DRV_LANGUAGE: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT2_DRV_DISPLAY: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT2_DRV_MOUSE: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(5i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT2_DRV_NETWORK: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(6i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT2_DRV_SYSTEM: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(7i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT2_DRV_INSTALLABLE: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(8i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT2_DRV_SOUND: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(9i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT2_DRV_COMM: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(10i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT2_DRV_INPUTMETHOD: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(11i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT2_DRV_VERSIONED_PRINTER: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(12i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT2_FONT_RASTER: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT2_FONT_VECTOR: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT2_FONT_TRUETYPE: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(3i32);
impl ::core::marker::Copy for VS_FIXEDFILEINFO_FILE_SUBTYPE {}
impl ::core::clone::Clone for VS_FIXEDFILEINFO_FILE_SUBTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VS_FIXEDFILEINFO_FILE_SUBTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VS_FIXEDFILEINFO_FILE_SUBTYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VS_FIXEDFILEINFO_FILE_SUBTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VS_FIXEDFILEINFO_FILE_SUBTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VS_FIXEDFILEINFO_FILE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT_UNKNOWN: VS_FIXEDFILEINFO_FILE_TYPE = VS_FIXEDFILEINFO_FILE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT_APP: VS_FIXEDFILEINFO_FILE_TYPE = VS_FIXEDFILEINFO_FILE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT_DLL: VS_FIXEDFILEINFO_FILE_TYPE = VS_FIXEDFILEINFO_FILE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT_DRV: VS_FIXEDFILEINFO_FILE_TYPE = VS_FIXEDFILEINFO_FILE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT_FONT: VS_FIXEDFILEINFO_FILE_TYPE = VS_FIXEDFILEINFO_FILE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT_VXD: VS_FIXEDFILEINFO_FILE_TYPE = VS_FIXEDFILEINFO_FILE_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT_STATIC_LIB: VS_FIXEDFILEINFO_FILE_TYPE = VS_FIXEDFILEINFO_FILE_TYPE(7i32);
impl ::core::marker::Copy for VS_FIXEDFILEINFO_FILE_TYPE {}
impl ::core::clone::Clone for VS_FIXEDFILEINFO_FILE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VS_FIXEDFILEINFO_FILE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VS_FIXEDFILEINFO_FILE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VS_FIXEDFILEINFO_FILE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VS_FIXEDFILEINFO_FILE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WIN_STREAM_ID(pub u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BACKUP_ALTERNATE_DATA: WIN_STREAM_ID = WIN_STREAM_ID(4u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BACKUP_DATA: WIN_STREAM_ID = WIN_STREAM_ID(1u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BACKUP_EA_DATA: WIN_STREAM_ID = WIN_STREAM_ID(2u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BACKUP_LINK: WIN_STREAM_ID = WIN_STREAM_ID(5u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BACKUP_OBJECT_ID: WIN_STREAM_ID = WIN_STREAM_ID(7u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BACKUP_PROPERTY_DATA: WIN_STREAM_ID = WIN_STREAM_ID(6u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BACKUP_REPARSE_DATA: WIN_STREAM_ID = WIN_STREAM_ID(8u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BACKUP_SECURITY_DATA: WIN_STREAM_ID = WIN_STREAM_ID(3u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BACKUP_SPARSE_BLOCK: WIN_STREAM_ID = WIN_STREAM_ID(9u32);
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BACKUP_TXFS_DATA: WIN_STREAM_ID = WIN_STREAM_ID(10u32);
impl ::core::marker::Copy for WIN_STREAM_ID {}
impl ::core::clone::Clone for WIN_STREAM_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WIN_STREAM_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WIN_STREAM_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WIN_STREAM_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WIN_STREAM_ID").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct BY_HANDLE_FILE_INFORMATION {
    pub dwFileAttributes: u32,
    pub ftCreationTime: super::super::Foundation::FILETIME,
    pub ftLastAccessTime: super::super::Foundation::FILETIME,
    pub ftLastWriteTime: super::super::Foundation::FILETIME,
    pub dwVolumeSerialNumber: u32,
    pub nFileSizeHigh: u32,
    pub nFileSizeLow: u32,
    pub nNumberOfLinks: u32,
    pub nFileIndexHigh: u32,
    pub nFileIndexLow: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for BY_HANDLE_FILE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for BY_HANDLE_FILE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for BY_HANDLE_FILE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BY_HANDLE_FILE_INFORMATION")
            .field("dwFileAttributes", &self.dwFileAttributes)
            .field("ftCreationTime", &self.ftCreationTime)
            .field("ftLastAccessTime", &self.ftLastAccessTime)
            .field("ftLastWriteTime", &self.ftLastWriteTime)
            .field("dwVolumeSerialNumber", &self.dwVolumeSerialNumber)
            .field("nFileSizeHigh", &self.nFileSizeHigh)
            .field("nFileSizeLow", &self.nFileSizeLow)
            .field("nNumberOfLinks", &self.nNumberOfLinks)
            .field("nFileIndexHigh", &self.nFileIndexHigh)
            .field("nFileIndexLow", &self.nFileIndexLow)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for BY_HANDLE_FILE_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for BY_HANDLE_FILE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwFileAttributes == other.dwFileAttributes && self.ftCreationTime == other.ftCreationTime && self.ftLastAccessTime == other.ftLastAccessTime && self.ftLastWriteTime == other.ftLastWriteTime && self.dwVolumeSerialNumber == other.dwVolumeSerialNumber && self.nFileSizeHigh == other.nFileSizeHigh && self.nFileSizeLow == other.nFileSizeLow && self.nNumberOfLinks == other.nNumberOfLinks && self.nFileIndexHigh == other.nFileIndexHigh && self.nFileIndexLow == other.nFileIndexLow
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for BY_HANDLE_FILE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BY_HANDLE_FILE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLFS_LOG_NAME_INFORMATION {
    pub NameLengthInBytes: u16,
    pub Name: [u16; 1],
}
impl ::core::marker::Copy for CLFS_LOG_NAME_INFORMATION {}
impl ::core::clone::Clone for CLFS_LOG_NAME_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLFS_LOG_NAME_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLFS_LOG_NAME_INFORMATION").field("NameLengthInBytes", &self.NameLengthInBytes).field("Name", &self.Name).finish()
    }
}
impl ::windows::core::TypeKind for CLFS_LOG_NAME_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CLFS_LOG_NAME_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.NameLengthInBytes == other.NameLengthInBytes && self.Name == other.Name
    }
}
impl ::core::cmp::Eq for CLFS_LOG_NAME_INFORMATION {}
impl ::core::default::Default for CLFS_LOG_NAME_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLFS_MGMT_NOTIFICATION {
    pub Notification: CLFS_MGMT_NOTIFICATION_TYPE,
    pub Lsn: CLS_LSN,
    pub LogIsPinned: u16,
}
impl ::core::marker::Copy for CLFS_MGMT_NOTIFICATION {}
impl ::core::clone::Clone for CLFS_MGMT_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLFS_MGMT_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLFS_MGMT_NOTIFICATION").field("Notification", &self.Notification).field("Lsn", &self.Lsn).field("LogIsPinned", &self.LogIsPinned).finish()
    }
}
impl ::windows::core::TypeKind for CLFS_MGMT_NOTIFICATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CLFS_MGMT_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.Notification == other.Notification && self.Lsn == other.Lsn && self.LogIsPinned == other.LogIsPinned
    }
}
impl ::core::cmp::Eq for CLFS_MGMT_NOTIFICATION {}
impl ::core::default::Default for CLFS_MGMT_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLFS_MGMT_POLICY {
    pub Version: u32,
    pub LengthInBytes: u32,
    pub PolicyFlags: u32,
    pub PolicyType: CLFS_MGMT_POLICY_TYPE,
    pub PolicyParameters: CLFS_MGMT_POLICY_0,
}
impl ::core::marker::Copy for CLFS_MGMT_POLICY {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for CLFS_MGMT_POLICY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for CLFS_MGMT_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub union CLFS_MGMT_POLICY_0 {
    pub MaximumSize: CLFS_MGMT_POLICY_0_4,
    pub MinimumSize: CLFS_MGMT_POLICY_0_5,
    pub NewContainerSize: CLFS_MGMT_POLICY_0_8,
    pub GrowthRate: CLFS_MGMT_POLICY_0_2,
    pub LogTail: CLFS_MGMT_POLICY_0_3,
    pub AutoShrink: CLFS_MGMT_POLICY_0_1,
    pub AutoGrow: CLFS_MGMT_POLICY_0_0,
    pub NewContainerPrefix: CLFS_MGMT_POLICY_0_7,
    pub NewContainerSuffix: CLFS_MGMT_POLICY_0_9,
    pub NewContainerExtension: CLFS_MGMT_POLICY_0_6,
}
impl ::core::marker::Copy for CLFS_MGMT_POLICY_0 {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for CLFS_MGMT_POLICY_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for CLFS_MGMT_POLICY_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLFS_MGMT_POLICY_0_0 {
    pub Enabled: u32,
}
impl ::core::marker::Copy for CLFS_MGMT_POLICY_0_0 {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLFS_MGMT_POLICY_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLFS_MGMT_POLICY_0_0").field("Enabled", &self.Enabled).finish()
    }
}
impl ::windows::core::TypeKind for CLFS_MGMT_POLICY_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CLFS_MGMT_POLICY_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Enabled == other.Enabled
    }
}
impl ::core::cmp::Eq for CLFS_MGMT_POLICY_0_0 {}
impl ::core::default::Default for CLFS_MGMT_POLICY_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLFS_MGMT_POLICY_0_1 {
    pub Percentage: u32,
}
impl ::core::marker::Copy for CLFS_MGMT_POLICY_0_1 {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLFS_MGMT_POLICY_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLFS_MGMT_POLICY_0_1").field("Percentage", &self.Percentage).finish()
    }
}
impl ::windows::core::TypeKind for CLFS_MGMT_POLICY_0_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CLFS_MGMT_POLICY_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Percentage == other.Percentage
    }
}
impl ::core::cmp::Eq for CLFS_MGMT_POLICY_0_1 {}
impl ::core::default::Default for CLFS_MGMT_POLICY_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLFS_MGMT_POLICY_0_2 {
    pub AbsoluteGrowthInContainers: u32,
    pub RelativeGrowthPercentage: u32,
}
impl ::core::marker::Copy for CLFS_MGMT_POLICY_0_2 {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLFS_MGMT_POLICY_0_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLFS_MGMT_POLICY_0_2").field("AbsoluteGrowthInContainers", &self.AbsoluteGrowthInContainers).field("RelativeGrowthPercentage", &self.RelativeGrowthPercentage).finish()
    }
}
impl ::windows::core::TypeKind for CLFS_MGMT_POLICY_0_2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CLFS_MGMT_POLICY_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.AbsoluteGrowthInContainers == other.AbsoluteGrowthInContainers && self.RelativeGrowthPercentage == other.RelativeGrowthPercentage
    }
}
impl ::core::cmp::Eq for CLFS_MGMT_POLICY_0_2 {}
impl ::core::default::Default for CLFS_MGMT_POLICY_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLFS_MGMT_POLICY_0_3 {
    pub MinimumAvailablePercentage: u32,
    pub MinimumAvailableContainers: u32,
}
impl ::core::marker::Copy for CLFS_MGMT_POLICY_0_3 {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLFS_MGMT_POLICY_0_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLFS_MGMT_POLICY_0_3").field("MinimumAvailablePercentage", &self.MinimumAvailablePercentage).field("MinimumAvailableContainers", &self.MinimumAvailableContainers).finish()
    }
}
impl ::windows::core::TypeKind for CLFS_MGMT_POLICY_0_3 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CLFS_MGMT_POLICY_0_3 {
    fn eq(&self, other: &Self) -> bool {
        self.MinimumAvailablePercentage == other.MinimumAvailablePercentage && self.MinimumAvailableContainers == other.MinimumAvailableContainers
    }
}
impl ::core::cmp::Eq for CLFS_MGMT_POLICY_0_3 {}
impl ::core::default::Default for CLFS_MGMT_POLICY_0_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLFS_MGMT_POLICY_0_4 {
    pub Containers: u32,
}
impl ::core::marker::Copy for CLFS_MGMT_POLICY_0_4 {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY_0_4 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLFS_MGMT_POLICY_0_4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLFS_MGMT_POLICY_0_4").field("Containers", &self.Containers).finish()
    }
}
impl ::windows::core::TypeKind for CLFS_MGMT_POLICY_0_4 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CLFS_MGMT_POLICY_0_4 {
    fn eq(&self, other: &Self) -> bool {
        self.Containers == other.Containers
    }
}
impl ::core::cmp::Eq for CLFS_MGMT_POLICY_0_4 {}
impl ::core::default::Default for CLFS_MGMT_POLICY_0_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLFS_MGMT_POLICY_0_5 {
    pub Containers: u32,
}
impl ::core::marker::Copy for CLFS_MGMT_POLICY_0_5 {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY_0_5 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLFS_MGMT_POLICY_0_5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLFS_MGMT_POLICY_0_5").field("Containers", &self.Containers).finish()
    }
}
impl ::windows::core::TypeKind for CLFS_MGMT_POLICY_0_5 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CLFS_MGMT_POLICY_0_5 {
    fn eq(&self, other: &Self) -> bool {
        self.Containers == other.Containers
    }
}
impl ::core::cmp::Eq for CLFS_MGMT_POLICY_0_5 {}
impl ::core::default::Default for CLFS_MGMT_POLICY_0_5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLFS_MGMT_POLICY_0_6 {
    pub ExtensionLengthInBytes: u16,
    pub ExtensionString: [u16; 1],
}
impl ::core::marker::Copy for CLFS_MGMT_POLICY_0_6 {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY_0_6 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLFS_MGMT_POLICY_0_6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLFS_MGMT_POLICY_0_6").field("ExtensionLengthInBytes", &self.ExtensionLengthInBytes).field("ExtensionString", &self.ExtensionString).finish()
    }
}
impl ::windows::core::TypeKind for CLFS_MGMT_POLICY_0_6 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CLFS_MGMT_POLICY_0_6 {
    fn eq(&self, other: &Self) -> bool {
        self.ExtensionLengthInBytes == other.ExtensionLengthInBytes && self.ExtensionString == other.ExtensionString
    }
}
impl ::core::cmp::Eq for CLFS_MGMT_POLICY_0_6 {}
impl ::core::default::Default for CLFS_MGMT_POLICY_0_6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLFS_MGMT_POLICY_0_7 {
    pub PrefixLengthInBytes: u16,
    pub PrefixString: [u16; 1],
}
impl ::core::marker::Copy for CLFS_MGMT_POLICY_0_7 {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY_0_7 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLFS_MGMT_POLICY_0_7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLFS_MGMT_POLICY_0_7").field("PrefixLengthInBytes", &self.PrefixLengthInBytes).field("PrefixString", &self.PrefixString).finish()
    }
}
impl ::windows::core::TypeKind for CLFS_MGMT_POLICY_0_7 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CLFS_MGMT_POLICY_0_7 {
    fn eq(&self, other: &Self) -> bool {
        self.PrefixLengthInBytes == other.PrefixLengthInBytes && self.PrefixString == other.PrefixString
    }
}
impl ::core::cmp::Eq for CLFS_MGMT_POLICY_0_7 {}
impl ::core::default::Default for CLFS_MGMT_POLICY_0_7 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLFS_MGMT_POLICY_0_8 {
    pub SizeInBytes: u32,
}
impl ::core::marker::Copy for CLFS_MGMT_POLICY_0_8 {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY_0_8 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLFS_MGMT_POLICY_0_8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLFS_MGMT_POLICY_0_8").field("SizeInBytes", &self.SizeInBytes).finish()
    }
}
impl ::windows::core::TypeKind for CLFS_MGMT_POLICY_0_8 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CLFS_MGMT_POLICY_0_8 {
    fn eq(&self, other: &Self) -> bool {
        self.SizeInBytes == other.SizeInBytes
    }
}
impl ::core::cmp::Eq for CLFS_MGMT_POLICY_0_8 {}
impl ::core::default::Default for CLFS_MGMT_POLICY_0_8 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLFS_MGMT_POLICY_0_9 {
    pub NextContainerSuffix: u64,
}
impl ::core::marker::Copy for CLFS_MGMT_POLICY_0_9 {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY_0_9 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLFS_MGMT_POLICY_0_9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLFS_MGMT_POLICY_0_9").field("NextContainerSuffix", &self.NextContainerSuffix).finish()
    }
}
impl ::windows::core::TypeKind for CLFS_MGMT_POLICY_0_9 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CLFS_MGMT_POLICY_0_9 {
    fn eq(&self, other: &Self) -> bool {
        self.NextContainerSuffix == other.NextContainerSuffix
    }
}
impl ::core::cmp::Eq for CLFS_MGMT_POLICY_0_9 {}
impl ::core::default::Default for CLFS_MGMT_POLICY_0_9 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLFS_NODE_ID {
    pub cType: u32,
    pub cbNode: u32,
}
impl ::core::marker::Copy for CLFS_NODE_ID {}
impl ::core::clone::Clone for CLFS_NODE_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLFS_NODE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLFS_NODE_ID").field("cType", &self.cType).field("cbNode", &self.cbNode).finish()
    }
}
impl ::windows::core::TypeKind for CLFS_NODE_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CLFS_NODE_ID {
    fn eq(&self, other: &Self) -> bool {
        self.cType == other.cType && self.cbNode == other.cbNode
    }
}
impl ::core::cmp::Eq for CLFS_NODE_ID {}
impl ::core::default::Default for CLFS_NODE_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLFS_PHYSICAL_LSN_INFORMATION {
    pub StreamIdentifier: u8,
    pub VirtualLsn: CLS_LSN,
    pub PhysicalLsn: CLS_LSN,
}
impl ::core::marker::Copy for CLFS_PHYSICAL_LSN_INFORMATION {}
impl ::core::clone::Clone for CLFS_PHYSICAL_LSN_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLFS_PHYSICAL_LSN_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLFS_PHYSICAL_LSN_INFORMATION").field("StreamIdentifier", &self.StreamIdentifier).field("VirtualLsn", &self.VirtualLsn).field("PhysicalLsn", &self.PhysicalLsn).finish()
    }
}
impl ::windows::core::TypeKind for CLFS_PHYSICAL_LSN_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CLFS_PHYSICAL_LSN_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.StreamIdentifier == other.StreamIdentifier && self.VirtualLsn == other.VirtualLsn && self.PhysicalLsn == other.PhysicalLsn
    }
}
impl ::core::cmp::Eq for CLFS_PHYSICAL_LSN_INFORMATION {}
impl ::core::default::Default for CLFS_PHYSICAL_LSN_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLFS_STREAM_ID_INFORMATION {
    pub StreamIdentifier: u8,
}
impl ::core::marker::Copy for CLFS_STREAM_ID_INFORMATION {}
impl ::core::clone::Clone for CLFS_STREAM_ID_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLFS_STREAM_ID_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLFS_STREAM_ID_INFORMATION").field("StreamIdentifier", &self.StreamIdentifier).finish()
    }
}
impl ::windows::core::TypeKind for CLFS_STREAM_ID_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CLFS_STREAM_ID_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.StreamIdentifier == other.StreamIdentifier
    }
}
impl ::core::cmp::Eq for CLFS_STREAM_ID_INFORMATION {}
impl ::core::default::Default for CLFS_STREAM_ID_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLS_ARCHIVE_DESCRIPTOR {
    pub coffLow: u64,
    pub coffHigh: u64,
    pub infoContainer: CLS_CONTAINER_INFORMATION,
}
impl ::core::marker::Copy for CLS_ARCHIVE_DESCRIPTOR {}
impl ::core::clone::Clone for CLS_ARCHIVE_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLS_ARCHIVE_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLS_ARCHIVE_DESCRIPTOR").field("coffLow", &self.coffLow).field("coffHigh", &self.coffHigh).field("infoContainer", &self.infoContainer).finish()
    }
}
impl ::windows::core::TypeKind for CLS_ARCHIVE_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CLS_ARCHIVE_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.coffLow == other.coffLow && self.coffHigh == other.coffHigh && self.infoContainer == other.infoContainer
    }
}
impl ::core::cmp::Eq for CLS_ARCHIVE_DESCRIPTOR {}
impl ::core::default::Default for CLS_ARCHIVE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLS_CONTAINER_INFORMATION {
    pub FileAttributes: u32,
    pub CreationTime: u64,
    pub LastAccessTime: u64,
    pub LastWriteTime: u64,
    pub ContainerSize: i64,
    pub FileNameActualLength: u32,
    pub FileNameLength: u32,
    pub FileName: [u16; 256],
    pub State: u32,
    pub PhysicalContainerId: u32,
    pub LogicalContainerId: u32,
}
impl ::core::marker::Copy for CLS_CONTAINER_INFORMATION {}
impl ::core::clone::Clone for CLS_CONTAINER_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLS_CONTAINER_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLS_CONTAINER_INFORMATION")
            .field("FileAttributes", &self.FileAttributes)
            .field("CreationTime", &self.CreationTime)
            .field("LastAccessTime", &self.LastAccessTime)
            .field("LastWriteTime", &self.LastWriteTime)
            .field("ContainerSize", &self.ContainerSize)
            .field("FileNameActualLength", &self.FileNameActualLength)
            .field("FileNameLength", &self.FileNameLength)
            .field("FileName", &self.FileName)
            .field("State", &self.State)
            .field("PhysicalContainerId", &self.PhysicalContainerId)
            .field("LogicalContainerId", &self.LogicalContainerId)
            .finish()
    }
}
impl ::windows::core::TypeKind for CLS_CONTAINER_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CLS_CONTAINER_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.FileAttributes == other.FileAttributes && self.CreationTime == other.CreationTime && self.LastAccessTime == other.LastAccessTime && self.LastWriteTime == other.LastWriteTime && self.ContainerSize == other.ContainerSize && self.FileNameActualLength == other.FileNameActualLength && self.FileNameLength == other.FileNameLength && self.FileName == other.FileName && self.State == other.State && self.PhysicalContainerId == other.PhysicalContainerId && self.LogicalContainerId == other.LogicalContainerId
    }
}
impl ::core::cmp::Eq for CLS_CONTAINER_INFORMATION {}
impl ::core::default::Default for CLS_CONTAINER_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLS_INFORMATION {
    pub TotalAvailable: i64,
    pub CurrentAvailable: i64,
    pub TotalReservation: i64,
    pub BaseFileSize: u64,
    pub ContainerSize: u64,
    pub TotalContainers: u32,
    pub FreeContainers: u32,
    pub TotalClients: u32,
    pub Attributes: u32,
    pub FlushThreshold: u32,
    pub SectorSize: u32,
    pub MinArchiveTailLsn: CLS_LSN,
    pub BaseLsn: CLS_LSN,
    pub LastFlushedLsn: CLS_LSN,
    pub LastLsn: CLS_LSN,
    pub RestartLsn: CLS_LSN,
    pub Identity: ::windows::core::GUID,
}
impl ::core::marker::Copy for CLS_INFORMATION {}
impl ::core::clone::Clone for CLS_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLS_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLS_INFORMATION")
            .field("TotalAvailable", &self.TotalAvailable)
            .field("CurrentAvailable", &self.CurrentAvailable)
            .field("TotalReservation", &self.TotalReservation)
            .field("BaseFileSize", &self.BaseFileSize)
            .field("ContainerSize", &self.ContainerSize)
            .field("TotalContainers", &self.TotalContainers)
            .field("FreeContainers", &self.FreeContainers)
            .field("TotalClients", &self.TotalClients)
            .field("Attributes", &self.Attributes)
            .field("FlushThreshold", &self.FlushThreshold)
            .field("SectorSize", &self.SectorSize)
            .field("MinArchiveTailLsn", &self.MinArchiveTailLsn)
            .field("BaseLsn", &self.BaseLsn)
            .field("LastFlushedLsn", &self.LastFlushedLsn)
            .field("LastLsn", &self.LastLsn)
            .field("RestartLsn", &self.RestartLsn)
            .field("Identity", &self.Identity)
            .finish()
    }
}
impl ::windows::core::TypeKind for CLS_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CLS_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.TotalAvailable == other.TotalAvailable && self.CurrentAvailable == other.CurrentAvailable && self.TotalReservation == other.TotalReservation && self.BaseFileSize == other.BaseFileSize && self.ContainerSize == other.ContainerSize && self.TotalContainers == other.TotalContainers && self.FreeContainers == other.FreeContainers && self.TotalClients == other.TotalClients && self.Attributes == other.Attributes && self.FlushThreshold == other.FlushThreshold && self.SectorSize == other.SectorSize && self.MinArchiveTailLsn == other.MinArchiveTailLsn && self.BaseLsn == other.BaseLsn && self.LastFlushedLsn == other.LastFlushedLsn && self.LastLsn == other.LastLsn && self.RestartLsn == other.RestartLsn && self.Identity == other.Identity
    }
}
impl ::core::cmp::Eq for CLS_INFORMATION {}
impl ::core::default::Default for CLS_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLS_IO_STATISTICS {
    pub hdrIoStats: CLS_IO_STATISTICS_HEADER,
    pub cFlush: u64,
    pub cbFlush: u64,
    pub cMetaFlush: u64,
    pub cbMetaFlush: u64,
}
impl ::core::marker::Copy for CLS_IO_STATISTICS {}
impl ::core::clone::Clone for CLS_IO_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLS_IO_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLS_IO_STATISTICS").field("hdrIoStats", &self.hdrIoStats).field("cFlush", &self.cFlush).field("cbFlush", &self.cbFlush).field("cMetaFlush", &self.cMetaFlush).field("cbMetaFlush", &self.cbMetaFlush).finish()
    }
}
impl ::windows::core::TypeKind for CLS_IO_STATISTICS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CLS_IO_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.hdrIoStats == other.hdrIoStats && self.cFlush == other.cFlush && self.cbFlush == other.cbFlush && self.cMetaFlush == other.cMetaFlush && self.cbMetaFlush == other.cbMetaFlush
    }
}
impl ::core::cmp::Eq for CLS_IO_STATISTICS {}
impl ::core::default::Default for CLS_IO_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLS_IO_STATISTICS_HEADER {
    pub ubMajorVersion: u8,
    pub ubMinorVersion: u8,
    pub eStatsClass: CLFS_IOSTATS_CLASS,
    pub cbLength: u16,
    pub coffData: u32,
}
impl ::core::marker::Copy for CLS_IO_STATISTICS_HEADER {}
impl ::core::clone::Clone for CLS_IO_STATISTICS_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLS_IO_STATISTICS_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLS_IO_STATISTICS_HEADER").field("ubMajorVersion", &self.ubMajorVersion).field("ubMinorVersion", &self.ubMinorVersion).field("eStatsClass", &self.eStatsClass).field("cbLength", &self.cbLength).field("coffData", &self.coffData).finish()
    }
}
impl ::windows::core::TypeKind for CLS_IO_STATISTICS_HEADER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CLS_IO_STATISTICS_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.ubMajorVersion == other.ubMajorVersion && self.ubMinorVersion == other.ubMinorVersion && self.eStatsClass == other.eStatsClass && self.cbLength == other.cbLength && self.coffData == other.coffData
    }
}
impl ::core::cmp::Eq for CLS_IO_STATISTICS_HEADER {}
impl ::core::default::Default for CLS_IO_STATISTICS_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLS_LSN {
    pub Internal: u64,
}
impl ::core::marker::Copy for CLS_LSN {}
impl ::core::clone::Clone for CLS_LSN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLS_LSN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLS_LSN").field("Internal", &self.Internal).finish()
    }
}
impl ::windows::core::TypeKind for CLS_LSN {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CLS_LSN {
    fn eq(&self, other: &Self) -> bool {
        self.Internal == other.Internal
    }
}
impl ::core::cmp::Eq for CLS_LSN {}
impl ::core::default::Default for CLS_LSN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CLS_SCAN_CONTEXT {
    pub cidNode: CLFS_NODE_ID,
    pub hLog: super::super::Foundation::HANDLE,
    pub cIndex: u32,
    pub cContainers: u32,
    pub cContainersReturned: u32,
    pub eScanMode: u8,
    pub pinfoContainer: *mut CLS_CONTAINER_INFORMATION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLS_SCAN_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLS_SCAN_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CLS_SCAN_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLS_SCAN_CONTEXT").field("cidNode", &self.cidNode).field("hLog", &self.hLog).field("cIndex", &self.cIndex).field("cContainers", &self.cContainers).field("cContainersReturned", &self.cContainersReturned).field("eScanMode", &self.eScanMode).field("pinfoContainer", &self.pinfoContainer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for CLS_SCAN_CONTEXT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CLS_SCAN_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.cidNode == other.cidNode && self.hLog == other.hLog && self.cIndex == other.cIndex && self.cContainers == other.cContainers && self.cContainersReturned == other.cContainersReturned && self.eScanMode == other.eScanMode && self.pinfoContainer == other.pinfoContainer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CLS_SCAN_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CLS_SCAN_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLS_WRITE_ENTRY {
    pub Buffer: *mut ::core::ffi::c_void,
    pub ByteLength: u32,
}
impl ::core::marker::Copy for CLS_WRITE_ENTRY {}
impl ::core::clone::Clone for CLS_WRITE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLS_WRITE_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLS_WRITE_ENTRY").field("Buffer", &self.Buffer).field("ByteLength", &self.ByteLength).finish()
    }
}
impl ::windows::core::TypeKind for CLS_WRITE_ENTRY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CLS_WRITE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.Buffer == other.Buffer && self.ByteLength == other.ByteLength
    }
}
impl ::core::cmp::Eq for CLS_WRITE_ENTRY {}
impl ::core::default::Default for CLS_WRITE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CONNECTION_INFO_0 {
    pub coni0_id: u32,
}
impl ::core::marker::Copy for CONNECTION_INFO_0 {}
impl ::core::clone::Clone for CONNECTION_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CONNECTION_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONNECTION_INFO_0").field("coni0_id", &self.coni0_id).finish()
    }
}
impl ::windows::core::TypeKind for CONNECTION_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CONNECTION_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.coni0_id == other.coni0_id
    }
}
impl ::core::cmp::Eq for CONNECTION_INFO_0 {}
impl ::core::default::Default for CONNECTION_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CONNECTION_INFO_1 {
    pub coni1_id: u32,
    pub coni1_type: SHARE_TYPE,
    pub coni1_num_opens: u32,
    pub coni1_num_users: u32,
    pub coni1_time: u32,
    pub coni1_username: ::windows::core::PWSTR,
    pub coni1_netname: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for CONNECTION_INFO_1 {}
impl ::core::clone::Clone for CONNECTION_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CONNECTION_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONNECTION_INFO_1").field("coni1_id", &self.coni1_id).field("coni1_type", &self.coni1_type).field("coni1_num_opens", &self.coni1_num_opens).field("coni1_num_users", &self.coni1_num_users).field("coni1_time", &self.coni1_time).field("coni1_username", &self.coni1_username).field("coni1_netname", &self.coni1_netname).finish()
    }
}
impl ::windows::core::TypeKind for CONNECTION_INFO_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CONNECTION_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.coni1_id == other.coni1_id && self.coni1_type == other.coni1_type && self.coni1_num_opens == other.coni1_num_opens && self.coni1_num_users == other.coni1_num_users && self.coni1_time == other.coni1_time && self.coni1_username == other.coni1_username && self.coni1_netname == other.coni1_netname
    }
}
impl ::core::cmp::Eq for CONNECTION_INFO_1 {}
impl ::core::default::Default for CONNECTION_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COPYFILE2_EXTENDED_PARAMETERS {
    pub dwSize: u32,
    pub dwCopyFlags: u32,
    pub pfCancel: *mut super::super::Foundation::BOOL,
    pub pProgressRoutine: PCOPYFILE2_PROGRESS_ROUTINE,
    pub pvCallbackContext: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COPYFILE2_EXTENDED_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COPYFILE2_EXTENDED_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for COPYFILE2_EXTENDED_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COPYFILE2_EXTENDED_PARAMETERS").field("dwSize", &self.dwSize).field("dwCopyFlags", &self.dwCopyFlags).field("pfCancel", &self.pfCancel).field("pvCallbackContext", &self.pvCallbackContext).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for COPYFILE2_EXTENDED_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COPYFILE2_EXTENDED_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COPYFILE2_EXTENDED_PARAMETERS_V2 {
    pub dwSize: u32,
    pub dwCopyFlags: u32,
    pub pfCancel: *mut super::super::Foundation::BOOL,
    pub pProgressRoutine: PCOPYFILE2_PROGRESS_ROUTINE,
    pub pvCallbackContext: *mut ::core::ffi::c_void,
    pub dwCopyFlagsV2: u32,
    pub ioDesiredSize: u32,
    pub ioDesiredRate: u32,
    pub reserved: [*mut ::core::ffi::c_void; 8],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COPYFILE2_EXTENDED_PARAMETERS_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COPYFILE2_EXTENDED_PARAMETERS_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for COPYFILE2_EXTENDED_PARAMETERS_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COPYFILE2_EXTENDED_PARAMETERS_V2").field("dwSize", &self.dwSize).field("dwCopyFlags", &self.dwCopyFlags).field("pfCancel", &self.pfCancel).field("pvCallbackContext", &self.pvCallbackContext).field("dwCopyFlagsV2", &self.dwCopyFlagsV2).field("ioDesiredSize", &self.ioDesiredSize).field("ioDesiredRate", &self.ioDesiredRate).field("reserved", &self.reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for COPYFILE2_EXTENDED_PARAMETERS_V2 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COPYFILE2_EXTENDED_PARAMETERS_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COPYFILE2_MESSAGE {
    pub Type: COPYFILE2_MESSAGE_TYPE,
    pub dwPadding: u32,
    pub Info: COPYFILE2_MESSAGE_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COPYFILE2_MESSAGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COPYFILE2_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for COPYFILE2_MESSAGE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COPYFILE2_MESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union COPYFILE2_MESSAGE_0 {
    pub ChunkStarted: COPYFILE2_MESSAGE_0_1,
    pub ChunkFinished: COPYFILE2_MESSAGE_0_0,
    pub StreamStarted: COPYFILE2_MESSAGE_0_5,
    pub StreamFinished: COPYFILE2_MESSAGE_0_4,
    pub PollContinue: COPYFILE2_MESSAGE_0_3,
    pub Error: COPYFILE2_MESSAGE_0_2,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COPYFILE2_MESSAGE_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COPYFILE2_MESSAGE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for COPYFILE2_MESSAGE_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COPYFILE2_MESSAGE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COPYFILE2_MESSAGE_0_0 {
    pub dwStreamNumber: u32,
    pub dwFlags: u32,
    pub hSourceFile: super::super::Foundation::HANDLE,
    pub hDestinationFile: super::super::Foundation::HANDLE,
    pub uliChunkNumber: u64,
    pub uliChunkSize: u64,
    pub uliStreamSize: u64,
    pub uliStreamBytesTransferred: u64,
    pub uliTotalFileSize: u64,
    pub uliTotalBytesTransferred: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COPYFILE2_MESSAGE_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COPYFILE2_MESSAGE_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for COPYFILE2_MESSAGE_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COPYFILE2_MESSAGE_0_0")
            .field("dwStreamNumber", &self.dwStreamNumber)
            .field("dwFlags", &self.dwFlags)
            .field("hSourceFile", &self.hSourceFile)
            .field("hDestinationFile", &self.hDestinationFile)
            .field("uliChunkNumber", &self.uliChunkNumber)
            .field("uliChunkSize", &self.uliChunkSize)
            .field("uliStreamSize", &self.uliStreamSize)
            .field("uliStreamBytesTransferred", &self.uliStreamBytesTransferred)
            .field("uliTotalFileSize", &self.uliTotalFileSize)
            .field("uliTotalBytesTransferred", &self.uliTotalBytesTransferred)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for COPYFILE2_MESSAGE_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for COPYFILE2_MESSAGE_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwStreamNumber == other.dwStreamNumber && self.dwFlags == other.dwFlags && self.hSourceFile == other.hSourceFile && self.hDestinationFile == other.hDestinationFile && self.uliChunkNumber == other.uliChunkNumber && self.uliChunkSize == other.uliChunkSize && self.uliStreamSize == other.uliStreamSize && self.uliStreamBytesTransferred == other.uliStreamBytesTransferred && self.uliTotalFileSize == other.uliTotalFileSize && self.uliTotalBytesTransferred == other.uliTotalBytesTransferred
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for COPYFILE2_MESSAGE_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COPYFILE2_MESSAGE_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COPYFILE2_MESSAGE_0_1 {
    pub dwStreamNumber: u32,
    pub dwReserved: u32,
    pub hSourceFile: super::super::Foundation::HANDLE,
    pub hDestinationFile: super::super::Foundation::HANDLE,
    pub uliChunkNumber: u64,
    pub uliChunkSize: u64,
    pub uliStreamSize: u64,
    pub uliTotalFileSize: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COPYFILE2_MESSAGE_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COPYFILE2_MESSAGE_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for COPYFILE2_MESSAGE_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COPYFILE2_MESSAGE_0_1").field("dwStreamNumber", &self.dwStreamNumber).field("dwReserved", &self.dwReserved).field("hSourceFile", &self.hSourceFile).field("hDestinationFile", &self.hDestinationFile).field("uliChunkNumber", &self.uliChunkNumber).field("uliChunkSize", &self.uliChunkSize).field("uliStreamSize", &self.uliStreamSize).field("uliTotalFileSize", &self.uliTotalFileSize).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for COPYFILE2_MESSAGE_0_1 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for COPYFILE2_MESSAGE_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwStreamNumber == other.dwStreamNumber && self.dwReserved == other.dwReserved && self.hSourceFile == other.hSourceFile && self.hDestinationFile == other.hDestinationFile && self.uliChunkNumber == other.uliChunkNumber && self.uliChunkSize == other.uliChunkSize && self.uliStreamSize == other.uliStreamSize && self.uliTotalFileSize == other.uliTotalFileSize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for COPYFILE2_MESSAGE_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COPYFILE2_MESSAGE_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COPYFILE2_MESSAGE_0_2 {
    pub CopyPhase: COPYFILE2_COPY_PHASE,
    pub dwStreamNumber: u32,
    pub hrFailure: ::windows::core::HRESULT,
    pub dwReserved: u32,
    pub uliChunkNumber: u64,
    pub uliStreamSize: u64,
    pub uliStreamBytesTransferred: u64,
    pub uliTotalFileSize: u64,
    pub uliTotalBytesTransferred: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COPYFILE2_MESSAGE_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COPYFILE2_MESSAGE_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for COPYFILE2_MESSAGE_0_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COPYFILE2_MESSAGE_0_2").field("CopyPhase", &self.CopyPhase).field("dwStreamNumber", &self.dwStreamNumber).field("hrFailure", &self.hrFailure).field("dwReserved", &self.dwReserved).field("uliChunkNumber", &self.uliChunkNumber).field("uliStreamSize", &self.uliStreamSize).field("uliStreamBytesTransferred", &self.uliStreamBytesTransferred).field("uliTotalFileSize", &self.uliTotalFileSize).field("uliTotalBytesTransferred", &self.uliTotalBytesTransferred).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for COPYFILE2_MESSAGE_0_2 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for COPYFILE2_MESSAGE_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.CopyPhase == other.CopyPhase && self.dwStreamNumber == other.dwStreamNumber && self.hrFailure == other.hrFailure && self.dwReserved == other.dwReserved && self.uliChunkNumber == other.uliChunkNumber && self.uliStreamSize == other.uliStreamSize && self.uliStreamBytesTransferred == other.uliStreamBytesTransferred && self.uliTotalFileSize == other.uliTotalFileSize && self.uliTotalBytesTransferred == other.uliTotalBytesTransferred
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for COPYFILE2_MESSAGE_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COPYFILE2_MESSAGE_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COPYFILE2_MESSAGE_0_3 {
    pub dwReserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COPYFILE2_MESSAGE_0_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COPYFILE2_MESSAGE_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for COPYFILE2_MESSAGE_0_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COPYFILE2_MESSAGE_0_3").field("dwReserved", &self.dwReserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for COPYFILE2_MESSAGE_0_3 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for COPYFILE2_MESSAGE_0_3 {
    fn eq(&self, other: &Self) -> bool {
        self.dwReserved == other.dwReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for COPYFILE2_MESSAGE_0_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COPYFILE2_MESSAGE_0_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COPYFILE2_MESSAGE_0_4 {
    pub dwStreamNumber: u32,
    pub dwReserved: u32,
    pub hSourceFile: super::super::Foundation::HANDLE,
    pub hDestinationFile: super::super::Foundation::HANDLE,
    pub uliStreamSize: u64,
    pub uliStreamBytesTransferred: u64,
    pub uliTotalFileSize: u64,
    pub uliTotalBytesTransferred: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COPYFILE2_MESSAGE_0_4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COPYFILE2_MESSAGE_0_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for COPYFILE2_MESSAGE_0_4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COPYFILE2_MESSAGE_0_4").field("dwStreamNumber", &self.dwStreamNumber).field("dwReserved", &self.dwReserved).field("hSourceFile", &self.hSourceFile).field("hDestinationFile", &self.hDestinationFile).field("uliStreamSize", &self.uliStreamSize).field("uliStreamBytesTransferred", &self.uliStreamBytesTransferred).field("uliTotalFileSize", &self.uliTotalFileSize).field("uliTotalBytesTransferred", &self.uliTotalBytesTransferred).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for COPYFILE2_MESSAGE_0_4 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for COPYFILE2_MESSAGE_0_4 {
    fn eq(&self, other: &Self) -> bool {
        self.dwStreamNumber == other.dwStreamNumber && self.dwReserved == other.dwReserved && self.hSourceFile == other.hSourceFile && self.hDestinationFile == other.hDestinationFile && self.uliStreamSize == other.uliStreamSize && self.uliStreamBytesTransferred == other.uliStreamBytesTransferred && self.uliTotalFileSize == other.uliTotalFileSize && self.uliTotalBytesTransferred == other.uliTotalBytesTransferred
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for COPYFILE2_MESSAGE_0_4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COPYFILE2_MESSAGE_0_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COPYFILE2_MESSAGE_0_5 {
    pub dwStreamNumber: u32,
    pub dwReserved: u32,
    pub hSourceFile: super::super::Foundation::HANDLE,
    pub hDestinationFile: super::super::Foundation::HANDLE,
    pub uliStreamSize: u64,
    pub uliTotalFileSize: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COPYFILE2_MESSAGE_0_5 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COPYFILE2_MESSAGE_0_5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for COPYFILE2_MESSAGE_0_5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COPYFILE2_MESSAGE_0_5").field("dwStreamNumber", &self.dwStreamNumber).field("dwReserved", &self.dwReserved).field("hSourceFile", &self.hSourceFile).field("hDestinationFile", &self.hDestinationFile).field("uliStreamSize", &self.uliStreamSize).field("uliTotalFileSize", &self.uliTotalFileSize).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for COPYFILE2_MESSAGE_0_5 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for COPYFILE2_MESSAGE_0_5 {
    fn eq(&self, other: &Self) -> bool {
        self.dwStreamNumber == other.dwStreamNumber && self.dwReserved == other.dwReserved && self.hSourceFile == other.hSourceFile && self.hDestinationFile == other.hDestinationFile && self.uliStreamSize == other.uliStreamSize && self.uliTotalFileSize == other.uliTotalFileSize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for COPYFILE2_MESSAGE_0_5 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COPYFILE2_MESSAGE_0_5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct CREATEFILE2_EXTENDED_PARAMETERS {
    pub dwSize: u32,
    pub dwFileAttributes: u32,
    pub dwFileFlags: u32,
    pub dwSecurityQosFlags: u32,
    pub lpSecurityAttributes: *mut super::super::Security::SECURITY_ATTRIBUTES,
    pub hTemplateFile: super::super::Foundation::HANDLE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for CREATEFILE2_EXTENDED_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for CREATEFILE2_EXTENDED_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for CREATEFILE2_EXTENDED_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREATEFILE2_EXTENDED_PARAMETERS").field("dwSize", &self.dwSize).field("dwFileAttributes", &self.dwFileAttributes).field("dwFileFlags", &self.dwFileFlags).field("dwSecurityQosFlags", &self.dwSecurityQosFlags).field("lpSecurityAttributes", &self.lpSecurityAttributes).field("hTemplateFile", &self.hTemplateFile).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::windows::core::TypeKind for CREATEFILE2_EXTENDED_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for CREATEFILE2_EXTENDED_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFileAttributes == other.dwFileAttributes && self.dwFileFlags == other.dwFileFlags && self.dwSecurityQosFlags == other.dwSecurityQosFlags && self.lpSecurityAttributes == other.lpSecurityAttributes && self.hTemplateFile == other.hTemplateFile
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for CREATEFILE2_EXTENDED_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for CREATEFILE2_EXTENDED_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct DISKQUOTA_USER_INFORMATION {
    pub QuotaUsed: i64,
    pub QuotaThreshold: i64,
    pub QuotaLimit: i64,
}
impl ::core::marker::Copy for DISKQUOTA_USER_INFORMATION {}
impl ::core::clone::Clone for DISKQUOTA_USER_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DISKQUOTA_USER_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISKQUOTA_USER_INFORMATION").field("QuotaUsed", &self.QuotaUsed).field("QuotaThreshold", &self.QuotaThreshold).field("QuotaLimit", &self.QuotaLimit).finish()
    }
}
impl ::windows::core::TypeKind for DISKQUOTA_USER_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DISKQUOTA_USER_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.QuotaUsed == other.QuotaUsed && self.QuotaThreshold == other.QuotaThreshold && self.QuotaLimit == other.QuotaLimit
    }
}
impl ::core::cmp::Eq for DISKQUOTA_USER_INFORMATION {}
impl ::core::default::Default for DISKQUOTA_USER_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct DISK_SPACE_INFORMATION {
    pub ActualTotalAllocationUnits: u64,
    pub ActualAvailableAllocationUnits: u64,
    pub ActualPoolUnavailableAllocationUnits: u64,
    pub CallerTotalAllocationUnits: u64,
    pub CallerAvailableAllocationUnits: u64,
    pub CallerPoolUnavailableAllocationUnits: u64,
    pub UsedAllocationUnits: u64,
    pub TotalReservedAllocationUnits: u64,
    pub VolumeStorageReserveAllocationUnits: u64,
    pub AvailableCommittedAllocationUnits: u64,
    pub PoolAvailableAllocationUnits: u64,
    pub SectorsPerAllocationUnit: u32,
    pub BytesPerSector: u32,
}
impl ::core::marker::Copy for DISK_SPACE_INFORMATION {}
impl ::core::clone::Clone for DISK_SPACE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DISK_SPACE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISK_SPACE_INFORMATION")
            .field("ActualTotalAllocationUnits", &self.ActualTotalAllocationUnits)
            .field("ActualAvailableAllocationUnits", &self.ActualAvailableAllocationUnits)
            .field("ActualPoolUnavailableAllocationUnits", &self.ActualPoolUnavailableAllocationUnits)
            .field("CallerTotalAllocationUnits", &self.CallerTotalAllocationUnits)
            .field("CallerAvailableAllocationUnits", &self.CallerAvailableAllocationUnits)
            .field("CallerPoolUnavailableAllocationUnits", &self.CallerPoolUnavailableAllocationUnits)
            .field("UsedAllocationUnits", &self.UsedAllocationUnits)
            .field("TotalReservedAllocationUnits", &self.TotalReservedAllocationUnits)
            .field("VolumeStorageReserveAllocationUnits", &self.VolumeStorageReserveAllocationUnits)
            .field("AvailableCommittedAllocationUnits", &self.AvailableCommittedAllocationUnits)
            .field("PoolAvailableAllocationUnits", &self.PoolAvailableAllocationUnits)
            .field("SectorsPerAllocationUnit", &self.SectorsPerAllocationUnit)
            .field("BytesPerSector", &self.BytesPerSector)
            .finish()
    }
}
impl ::windows::core::TypeKind for DISK_SPACE_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DISK_SPACE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ActualTotalAllocationUnits == other.ActualTotalAllocationUnits
            && self.ActualAvailableAllocationUnits == other.ActualAvailableAllocationUnits
            && self.ActualPoolUnavailableAllocationUnits == other.ActualPoolUnavailableAllocationUnits
            && self.CallerTotalAllocationUnits == other.CallerTotalAllocationUnits
            && self.CallerAvailableAllocationUnits == other.CallerAvailableAllocationUnits
            && self.CallerPoolUnavailableAllocationUnits == other.CallerPoolUnavailableAllocationUnits
            && self.UsedAllocationUnits == other.UsedAllocationUnits
            && self.TotalReservedAllocationUnits == other.TotalReservedAllocationUnits
            && self.VolumeStorageReserveAllocationUnits == other.VolumeStorageReserveAllocationUnits
            && self.AvailableCommittedAllocationUnits == other.AvailableCommittedAllocationUnits
            && self.PoolAvailableAllocationUnits == other.PoolAvailableAllocationUnits
            && self.SectorsPerAllocationUnit == other.SectorsPerAllocationUnit
            && self.BytesPerSector == other.BytesPerSector
    }
}
impl ::core::cmp::Eq for DISK_SPACE_INFORMATION {}
impl ::core::default::Default for DISK_SPACE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct EFS_CERTIFICATE_BLOB {
    pub dwCertEncodingType: u32,
    pub cbData: u32,
    pub pbData: *mut u8,
}
impl ::core::marker::Copy for EFS_CERTIFICATE_BLOB {}
impl ::core::clone::Clone for EFS_CERTIFICATE_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EFS_CERTIFICATE_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EFS_CERTIFICATE_BLOB").field("dwCertEncodingType", &self.dwCertEncodingType).field("cbData", &self.cbData).field("pbData", &self.pbData).finish()
    }
}
impl ::windows::core::TypeKind for EFS_CERTIFICATE_BLOB {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for EFS_CERTIFICATE_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.dwCertEncodingType == other.dwCertEncodingType && self.cbData == other.cbData && self.pbData == other.pbData
    }
}
impl ::core::cmp::Eq for EFS_CERTIFICATE_BLOB {}
impl ::core::default::Default for EFS_CERTIFICATE_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct EFS_COMPATIBILITY_INFO {
    pub EfsVersion: u32,
}
impl ::core::marker::Copy for EFS_COMPATIBILITY_INFO {}
impl ::core::clone::Clone for EFS_COMPATIBILITY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EFS_COMPATIBILITY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EFS_COMPATIBILITY_INFO").field("EfsVersion", &self.EfsVersion).finish()
    }
}
impl ::windows::core::TypeKind for EFS_COMPATIBILITY_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for EFS_COMPATIBILITY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.EfsVersion == other.EfsVersion
    }
}
impl ::core::cmp::Eq for EFS_COMPATIBILITY_INFO {}
impl ::core::default::Default for EFS_COMPATIBILITY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct EFS_DECRYPTION_STATUS_INFO {
    pub dwDecryptionError: u32,
    pub dwHashOffset: u32,
    pub cbHash: u32,
}
impl ::core::marker::Copy for EFS_DECRYPTION_STATUS_INFO {}
impl ::core::clone::Clone for EFS_DECRYPTION_STATUS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EFS_DECRYPTION_STATUS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EFS_DECRYPTION_STATUS_INFO").field("dwDecryptionError", &self.dwDecryptionError).field("dwHashOffset", &self.dwHashOffset).field("cbHash", &self.cbHash).finish()
    }
}
impl ::windows::core::TypeKind for EFS_DECRYPTION_STATUS_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for EFS_DECRYPTION_STATUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwDecryptionError == other.dwDecryptionError && self.dwHashOffset == other.dwHashOffset && self.cbHash == other.cbHash
    }
}
impl ::core::cmp::Eq for EFS_DECRYPTION_STATUS_INFO {}
impl ::core::default::Default for EFS_DECRYPTION_STATUS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EFS_ENCRYPTION_STATUS_INFO {
    pub bHasCurrentKey: super::super::Foundation::BOOL,
    pub dwEncryptionError: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EFS_ENCRYPTION_STATUS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EFS_ENCRYPTION_STATUS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EFS_ENCRYPTION_STATUS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EFS_ENCRYPTION_STATUS_INFO").field("bHasCurrentKey", &self.bHasCurrentKey).field("dwEncryptionError", &self.dwEncryptionError).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for EFS_ENCRYPTION_STATUS_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EFS_ENCRYPTION_STATUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.bHasCurrentKey == other.bHasCurrentKey && self.dwEncryptionError == other.dwEncryptionError
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EFS_ENCRYPTION_STATUS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EFS_ENCRYPTION_STATUS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct EFS_HASH_BLOB {
    pub cbData: u32,
    pub pbData: *mut u8,
}
impl ::core::marker::Copy for EFS_HASH_BLOB {}
impl ::core::clone::Clone for EFS_HASH_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EFS_HASH_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EFS_HASH_BLOB").field("cbData", &self.cbData).field("pbData", &self.pbData).finish()
    }
}
impl ::windows::core::TypeKind for EFS_HASH_BLOB {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for EFS_HASH_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.cbData == other.cbData && self.pbData == other.pbData
    }
}
impl ::core::cmp::Eq for EFS_HASH_BLOB {}
impl ::core::default::Default for EFS_HASH_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct EFS_KEY_INFO {
    pub dwVersion: u32,
    pub Entropy: u32,
    pub Algorithm: u32,
    pub KeyLength: u32,
}
impl ::core::marker::Copy for EFS_KEY_INFO {}
impl ::core::clone::Clone for EFS_KEY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EFS_KEY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EFS_KEY_INFO").field("dwVersion", &self.dwVersion).field("Entropy", &self.Entropy).field("Algorithm", &self.Algorithm).field("KeyLength", &self.KeyLength).finish()
    }
}
impl ::windows::core::TypeKind for EFS_KEY_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for EFS_KEY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.Entropy == other.Entropy && self.Algorithm == other.Algorithm && self.KeyLength == other.KeyLength
    }
}
impl ::core::cmp::Eq for EFS_KEY_INFO {}
impl ::core::default::Default for EFS_KEY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct EFS_PIN_BLOB {
    pub cbPadding: u32,
    pub cbData: u32,
    pub pbData: *mut u8,
}
impl ::core::marker::Copy for EFS_PIN_BLOB {}
impl ::core::clone::Clone for EFS_PIN_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EFS_PIN_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EFS_PIN_BLOB").field("cbPadding", &self.cbPadding).field("cbData", &self.cbData).field("pbData", &self.pbData).finish()
    }
}
impl ::windows::core::TypeKind for EFS_PIN_BLOB {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for EFS_PIN_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.cbPadding == other.cbPadding && self.cbData == other.cbData && self.pbData == other.pbData
    }
}
impl ::core::cmp::Eq for EFS_PIN_BLOB {}
impl ::core::default::Default for EFS_PIN_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct EFS_RPC_BLOB {
    pub cbData: u32,
    pub pbData: *mut u8,
}
impl ::core::marker::Copy for EFS_RPC_BLOB {}
impl ::core::clone::Clone for EFS_RPC_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EFS_RPC_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EFS_RPC_BLOB").field("cbData", &self.cbData).field("pbData", &self.pbData).finish()
    }
}
impl ::windows::core::TypeKind for EFS_RPC_BLOB {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for EFS_RPC_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.cbData == other.cbData && self.pbData == other.pbData
    }
}
impl ::core::cmp::Eq for EFS_RPC_BLOB {}
impl ::core::default::Default for EFS_RPC_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct EFS_VERSION_INFO {
    pub EfsVersion: u32,
    pub SubVersion: u32,
}
impl ::core::marker::Copy for EFS_VERSION_INFO {}
impl ::core::clone::Clone for EFS_VERSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EFS_VERSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EFS_VERSION_INFO").field("EfsVersion", &self.EfsVersion).field("SubVersion", &self.SubVersion).finish()
    }
}
impl ::windows::core::TypeKind for EFS_VERSION_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for EFS_VERSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.EfsVersion == other.EfsVersion && self.SubVersion == other.SubVersion
    }
}
impl ::core::cmp::Eq for EFS_VERSION_INFO {}
impl ::core::default::Default for EFS_VERSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct ENCRYPTED_FILE_METADATA_SIGNATURE {
    pub dwEfsAccessType: u32,
    pub pCertificatesAdded: *mut ENCRYPTION_CERTIFICATE_HASH_LIST,
    pub pEncryptionCertificate: *mut ENCRYPTION_CERTIFICATE,
    pub pEfsStreamSignature: *mut EFS_RPC_BLOB,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for ENCRYPTED_FILE_METADATA_SIGNATURE {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for ENCRYPTED_FILE_METADATA_SIGNATURE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::fmt::Debug for ENCRYPTED_FILE_METADATA_SIGNATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENCRYPTED_FILE_METADATA_SIGNATURE").field("dwEfsAccessType", &self.dwEfsAccessType).field("pCertificatesAdded", &self.pCertificatesAdded).field("pEncryptionCertificate", &self.pEncryptionCertificate).field("pEfsStreamSignature", &self.pEfsStreamSignature).finish()
    }
}
#[cfg(feature = "Win32_Security")]
impl ::windows::core::TypeKind for ENCRYPTED_FILE_METADATA_SIGNATURE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::PartialEq for ENCRYPTED_FILE_METADATA_SIGNATURE {
    fn eq(&self, other: &Self) -> bool {
        self.dwEfsAccessType == other.dwEfsAccessType && self.pCertificatesAdded == other.pCertificatesAdded && self.pEncryptionCertificate == other.pEncryptionCertificate && self.pEfsStreamSignature == other.pEfsStreamSignature
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::Eq for ENCRYPTED_FILE_METADATA_SIGNATURE {}
#[cfg(feature = "Win32_Security")]
impl ::core::default::Default for ENCRYPTED_FILE_METADATA_SIGNATURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct ENCRYPTION_CERTIFICATE {
    pub cbTotalLength: u32,
    pub pUserSid: *mut super::super::Security::SID,
    pub pCertBlob: *mut EFS_CERTIFICATE_BLOB,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for ENCRYPTION_CERTIFICATE {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for ENCRYPTION_CERTIFICATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::fmt::Debug for ENCRYPTION_CERTIFICATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENCRYPTION_CERTIFICATE").field("cbTotalLength", &self.cbTotalLength).field("pUserSid", &self.pUserSid).field("pCertBlob", &self.pCertBlob).finish()
    }
}
#[cfg(feature = "Win32_Security")]
impl ::windows::core::TypeKind for ENCRYPTION_CERTIFICATE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::PartialEq for ENCRYPTION_CERTIFICATE {
    fn eq(&self, other: &Self) -> bool {
        self.cbTotalLength == other.cbTotalLength && self.pUserSid == other.pUserSid && self.pCertBlob == other.pCertBlob
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::Eq for ENCRYPTION_CERTIFICATE {}
#[cfg(feature = "Win32_Security")]
impl ::core::default::Default for ENCRYPTION_CERTIFICATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct ENCRYPTION_CERTIFICATE_HASH {
    pub cbTotalLength: u32,
    pub pUserSid: *mut super::super::Security::SID,
    pub pHash: *mut EFS_HASH_BLOB,
    pub lpDisplayInformation: ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for ENCRYPTION_CERTIFICATE_HASH {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for ENCRYPTION_CERTIFICATE_HASH {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::fmt::Debug for ENCRYPTION_CERTIFICATE_HASH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENCRYPTION_CERTIFICATE_HASH").field("cbTotalLength", &self.cbTotalLength).field("pUserSid", &self.pUserSid).field("pHash", &self.pHash).field("lpDisplayInformation", &self.lpDisplayInformation).finish()
    }
}
#[cfg(feature = "Win32_Security")]
impl ::windows::core::TypeKind for ENCRYPTION_CERTIFICATE_HASH {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::PartialEq for ENCRYPTION_CERTIFICATE_HASH {
    fn eq(&self, other: &Self) -> bool {
        self.cbTotalLength == other.cbTotalLength && self.pUserSid == other.pUserSid && self.pHash == other.pHash && self.lpDisplayInformation == other.lpDisplayInformation
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::Eq for ENCRYPTION_CERTIFICATE_HASH {}
#[cfg(feature = "Win32_Security")]
impl ::core::default::Default for ENCRYPTION_CERTIFICATE_HASH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct ENCRYPTION_CERTIFICATE_HASH_LIST {
    pub nCert_Hash: u32,
    pub pUsers: *mut *mut ENCRYPTION_CERTIFICATE_HASH,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for ENCRYPTION_CERTIFICATE_HASH_LIST {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for ENCRYPTION_CERTIFICATE_HASH_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::fmt::Debug for ENCRYPTION_CERTIFICATE_HASH_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENCRYPTION_CERTIFICATE_HASH_LIST").field("nCert_Hash", &self.nCert_Hash).field("pUsers", &self.pUsers).finish()
    }
}
#[cfg(feature = "Win32_Security")]
impl ::windows::core::TypeKind for ENCRYPTION_CERTIFICATE_HASH_LIST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::PartialEq for ENCRYPTION_CERTIFICATE_HASH_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.nCert_Hash == other.nCert_Hash && self.pUsers == other.pUsers
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::Eq for ENCRYPTION_CERTIFICATE_HASH_LIST {}
#[cfg(feature = "Win32_Security")]
impl ::core::default::Default for ENCRYPTION_CERTIFICATE_HASH_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct ENCRYPTION_CERTIFICATE_LIST {
    pub nUsers: u32,
    pub pUsers: *mut *mut ENCRYPTION_CERTIFICATE,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for ENCRYPTION_CERTIFICATE_LIST {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for ENCRYPTION_CERTIFICATE_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::fmt::Debug for ENCRYPTION_CERTIFICATE_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENCRYPTION_CERTIFICATE_LIST").field("nUsers", &self.nUsers).field("pUsers", &self.pUsers).finish()
    }
}
#[cfg(feature = "Win32_Security")]
impl ::windows::core::TypeKind for ENCRYPTION_CERTIFICATE_LIST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::PartialEq for ENCRYPTION_CERTIFICATE_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.nUsers == other.nUsers && self.pUsers == other.pUsers
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::Eq for ENCRYPTION_CERTIFICATE_LIST {}
#[cfg(feature = "Win32_Security")]
impl ::core::default::Default for ENCRYPTION_CERTIFICATE_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct ENCRYPTION_PROTECTOR {
    pub cbTotalLength: u32,
    pub pUserSid: *mut super::super::Security::SID,
    pub lpProtectorDescriptor: ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for ENCRYPTION_PROTECTOR {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for ENCRYPTION_PROTECTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::fmt::Debug for ENCRYPTION_PROTECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENCRYPTION_PROTECTOR").field("cbTotalLength", &self.cbTotalLength).field("pUserSid", &self.pUserSid).field("lpProtectorDescriptor", &self.lpProtectorDescriptor).finish()
    }
}
#[cfg(feature = "Win32_Security")]
impl ::windows::core::TypeKind for ENCRYPTION_PROTECTOR {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::PartialEq for ENCRYPTION_PROTECTOR {
    fn eq(&self, other: &Self) -> bool {
        self.cbTotalLength == other.cbTotalLength && self.pUserSid == other.pUserSid && self.lpProtectorDescriptor == other.lpProtectorDescriptor
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::Eq for ENCRYPTION_PROTECTOR {}
#[cfg(feature = "Win32_Security")]
impl ::core::default::Default for ENCRYPTION_PROTECTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct ENCRYPTION_PROTECTOR_LIST {
    pub nProtectors: u32,
    pub pProtectors: *mut *mut ENCRYPTION_PROTECTOR,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for ENCRYPTION_PROTECTOR_LIST {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for ENCRYPTION_PROTECTOR_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::fmt::Debug for ENCRYPTION_PROTECTOR_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENCRYPTION_PROTECTOR_LIST").field("nProtectors", &self.nProtectors).field("pProtectors", &self.pProtectors).finish()
    }
}
#[cfg(feature = "Win32_Security")]
impl ::windows::core::TypeKind for ENCRYPTION_PROTECTOR_LIST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::PartialEq for ENCRYPTION_PROTECTOR_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.nProtectors == other.nProtectors && self.pProtectors == other.pProtectors
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::Eq for ENCRYPTION_PROTECTOR_LIST {}
#[cfg(feature = "Win32_Security")]
impl ::core::default::Default for ENCRYPTION_PROTECTOR_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FH_OVERLAPPED {
    pub Internal: usize,
    pub InternalHigh: usize,
    pub Offset: u32,
    pub OffsetHigh: u32,
    pub hEvent: super::super::Foundation::HANDLE,
    pub pfnCompletion: PFN_IO_COMPLETION,
    pub Reserved1: usize,
    pub Reserved2: usize,
    pub Reserved3: usize,
    pub Reserved4: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FH_OVERLAPPED {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FH_OVERLAPPED {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FH_OVERLAPPED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FH_OVERLAPPED").field("Internal", &self.Internal).field("InternalHigh", &self.InternalHigh).field("Offset", &self.Offset).field("OffsetHigh", &self.OffsetHigh).field("hEvent", &self.hEvent).field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).field("Reserved3", &self.Reserved3).field("Reserved4", &self.Reserved4).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for FH_OVERLAPPED {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FH_OVERLAPPED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_ALIGNMENT_INFO {
    pub AlignmentRequirement: u32,
}
impl ::core::marker::Copy for FILE_ALIGNMENT_INFO {}
impl ::core::clone::Clone for FILE_ALIGNMENT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_ALIGNMENT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_ALIGNMENT_INFO").field("AlignmentRequirement", &self.AlignmentRequirement).finish()
    }
}
impl ::windows::core::TypeKind for FILE_ALIGNMENT_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for FILE_ALIGNMENT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.AlignmentRequirement == other.AlignmentRequirement
    }
}
impl ::core::cmp::Eq for FILE_ALIGNMENT_INFO {}
impl ::core::default::Default for FILE_ALIGNMENT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_ALLOCATION_INFO {
    pub AllocationSize: i64,
}
impl ::core::marker::Copy for FILE_ALLOCATION_INFO {}
impl ::core::clone::Clone for FILE_ALLOCATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_ALLOCATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_ALLOCATION_INFO").field("AllocationSize", &self.AllocationSize).finish()
    }
}
impl ::windows::core::TypeKind for FILE_ALLOCATION_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for FILE_ALLOCATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.AllocationSize == other.AllocationSize
    }
}
impl ::core::cmp::Eq for FILE_ALLOCATION_INFO {}
impl ::core::default::Default for FILE_ALLOCATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_ATTRIBUTE_TAG_INFO {
    pub FileAttributes: u32,
    pub ReparseTag: u32,
}
impl ::core::marker::Copy for FILE_ATTRIBUTE_TAG_INFO {}
impl ::core::clone::Clone for FILE_ATTRIBUTE_TAG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_ATTRIBUTE_TAG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_ATTRIBUTE_TAG_INFO").field("FileAttributes", &self.FileAttributes).field("ReparseTag", &self.ReparseTag).finish()
    }
}
impl ::windows::core::TypeKind for FILE_ATTRIBUTE_TAG_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for FILE_ATTRIBUTE_TAG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.FileAttributes == other.FileAttributes && self.ReparseTag == other.ReparseTag
    }
}
impl ::core::cmp::Eq for FILE_ATTRIBUTE_TAG_INFO {}
impl ::core::default::Default for FILE_ATTRIBUTE_TAG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_BASIC_INFO {
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub FileAttributes: u32,
}
impl ::core::marker::Copy for FILE_BASIC_INFO {}
impl ::core::clone::Clone for FILE_BASIC_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_BASIC_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_BASIC_INFO").field("CreationTime", &self.CreationTime).field("LastAccessTime", &self.LastAccessTime).field("LastWriteTime", &self.LastWriteTime).field("ChangeTime", &self.ChangeTime).field("FileAttributes", &self.FileAttributes).finish()
    }
}
impl ::windows::core::TypeKind for FILE_BASIC_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for FILE_BASIC_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.CreationTime == other.CreationTime && self.LastAccessTime == other.LastAccessTime && self.LastWriteTime == other.LastWriteTime && self.ChangeTime == other.ChangeTime && self.FileAttributes == other.FileAttributes
    }
}
impl ::core::cmp::Eq for FILE_BASIC_INFO {}
impl ::core::default::Default for FILE_BASIC_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_COMPRESSION_INFO {
    pub CompressedFileSize: i64,
    pub CompressionFormat: COMPRESSION_FORMAT,
    pub CompressionUnitShift: u8,
    pub ChunkShift: u8,
    pub ClusterShift: u8,
    pub Reserved: [u8; 3],
}
impl ::core::marker::Copy for FILE_COMPRESSION_INFO {}
impl ::core::clone::Clone for FILE_COMPRESSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_COMPRESSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_COMPRESSION_INFO").field("CompressedFileSize", &self.CompressedFileSize).field("CompressionFormat", &self.CompressionFormat).field("CompressionUnitShift", &self.CompressionUnitShift).field("ChunkShift", &self.ChunkShift).field("ClusterShift", &self.ClusterShift).field("Reserved", &self.Reserved).finish()
    }
}
impl ::windows::core::TypeKind for FILE_COMPRESSION_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for FILE_COMPRESSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.CompressedFileSize == other.CompressedFileSize && self.CompressionFormat == other.CompressionFormat && self.CompressionUnitShift == other.CompressionUnitShift && self.ChunkShift == other.ChunkShift && self.ClusterShift == other.ClusterShift && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for FILE_COMPRESSION_INFO {}
impl ::core::default::Default for FILE_COMPRESSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_DISPOSITION_INFO {
    pub DeleteFile: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_DISPOSITION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_DISPOSITION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FILE_DISPOSITION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_DISPOSITION_INFO").field("DeleteFile", &self.DeleteFile).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for FILE_DISPOSITION_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FILE_DISPOSITION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.DeleteFile == other.DeleteFile
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FILE_DISPOSITION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FILE_DISPOSITION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_END_OF_FILE_INFO {
    pub EndOfFile: i64,
}
impl ::core::marker::Copy for FILE_END_OF_FILE_INFO {}
impl ::core::clone::Clone for FILE_END_OF_FILE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_END_OF_FILE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_END_OF_FILE_INFO").field("EndOfFile", &self.EndOfFile).finish()
    }
}
impl ::windows::core::TypeKind for FILE_END_OF_FILE_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for FILE_END_OF_FILE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.EndOfFile == other.EndOfFile
    }
}
impl ::core::cmp::Eq for FILE_END_OF_FILE_INFO {}
impl ::core::default::Default for FILE_END_OF_FILE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_EXTENT {
    pub VolumeOffset: u64,
    pub ExtentLength: u64,
}
impl ::core::marker::Copy for FILE_EXTENT {}
impl ::core::clone::Clone for FILE_EXTENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_EXTENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_EXTENT").field("VolumeOffset", &self.VolumeOffset).field("ExtentLength", &self.ExtentLength).finish()
    }
}
impl ::windows::core::TypeKind for FILE_EXTENT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for FILE_EXTENT {
    fn eq(&self, other: &Self) -> bool {
        self.VolumeOffset == other.VolumeOffset && self.ExtentLength == other.ExtentLength
    }
}
impl ::core::cmp::Eq for FILE_EXTENT {}
impl ::core::default::Default for FILE_EXTENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_FULL_DIR_INFO {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FILE_FULL_DIR_INFO {}
impl ::core::clone::Clone for FILE_FULL_DIR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_FULL_DIR_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_FULL_DIR_INFO")
            .field("NextEntryOffset", &self.NextEntryOffset)
            .field("FileIndex", &self.FileIndex)
            .field("CreationTime", &self.CreationTime)
            .field("LastAccessTime", &self.LastAccessTime)
            .field("LastWriteTime", &self.LastWriteTime)
            .field("ChangeTime", &self.ChangeTime)
            .field("EndOfFile", &self.EndOfFile)
            .field("AllocationSize", &self.AllocationSize)
            .field("FileAttributes", &self.FileAttributes)
            .field("FileNameLength", &self.FileNameLength)
            .field("EaSize", &self.EaSize)
            .field("FileName", &self.FileName)
            .finish()
    }
}
impl ::windows::core::TypeKind for FILE_FULL_DIR_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for FILE_FULL_DIR_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset && self.FileIndex == other.FileIndex && self.CreationTime == other.CreationTime && self.LastAccessTime == other.LastAccessTime && self.LastWriteTime == other.LastWriteTime && self.ChangeTime == other.ChangeTime && self.EndOfFile == other.EndOfFile && self.AllocationSize == other.AllocationSize && self.FileAttributes == other.FileAttributes && self.FileNameLength == other.FileNameLength && self.EaSize == other.EaSize && self.FileName == other.FileName
    }
}
impl ::core::cmp::Eq for FILE_FULL_DIR_INFO {}
impl ::core::default::Default for FILE_FULL_DIR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_ID_128 {
    pub Identifier: [u8; 16],
}
impl ::core::marker::Copy for FILE_ID_128 {}
impl ::core::clone::Clone for FILE_ID_128 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_ID_128 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_ID_128").field("Identifier", &self.Identifier).finish()
    }
}
impl ::windows::core::TypeKind for FILE_ID_128 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for FILE_ID_128 {
    fn eq(&self, other: &Self) -> bool {
        self.Identifier == other.Identifier
    }
}
impl ::core::cmp::Eq for FILE_ID_128 {}
impl ::core::default::Default for FILE_ID_128 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_ID_BOTH_DIR_INFO {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ShortNameLength: i8,
    pub ShortName: [u16; 12],
    pub FileId: i64,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FILE_ID_BOTH_DIR_INFO {}
impl ::core::clone::Clone for FILE_ID_BOTH_DIR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_ID_BOTH_DIR_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_ID_BOTH_DIR_INFO")
            .field("NextEntryOffset", &self.NextEntryOffset)
            .field("FileIndex", &self.FileIndex)
            .field("CreationTime", &self.CreationTime)
            .field("LastAccessTime", &self.LastAccessTime)
            .field("LastWriteTime", &self.LastWriteTime)
            .field("ChangeTime", &self.ChangeTime)
            .field("EndOfFile", &self.EndOfFile)
            .field("AllocationSize", &self.AllocationSize)
            .field("FileAttributes", &self.FileAttributes)
            .field("FileNameLength", &self.FileNameLength)
            .field("EaSize", &self.EaSize)
            .field("ShortNameLength", &self.ShortNameLength)
            .field("ShortName", &self.ShortName)
            .field("FileId", &self.FileId)
            .field("FileName", &self.FileName)
            .finish()
    }
}
impl ::windows::core::TypeKind for FILE_ID_BOTH_DIR_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for FILE_ID_BOTH_DIR_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset && self.FileIndex == other.FileIndex && self.CreationTime == other.CreationTime && self.LastAccessTime == other.LastAccessTime && self.LastWriteTime == other.LastWriteTime && self.ChangeTime == other.ChangeTime && self.EndOfFile == other.EndOfFile && self.AllocationSize == other.AllocationSize && self.FileAttributes == other.FileAttributes && self.FileNameLength == other.FileNameLength && self.EaSize == other.EaSize && self.ShortNameLength == other.ShortNameLength && self.ShortName == other.ShortName && self.FileId == other.FileId && self.FileName == other.FileName
    }
}
impl ::core::cmp::Eq for FILE_ID_BOTH_DIR_INFO {}
impl ::core::default::Default for FILE_ID_BOTH_DIR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_ID_DESCRIPTOR {
    pub dwSize: u32,
    pub Type: FILE_ID_TYPE,
    pub Anonymous: FILE_ID_DESCRIPTOR_0,
}
impl ::core::marker::Copy for FILE_ID_DESCRIPTOR {}
impl ::core::clone::Clone for FILE_ID_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for FILE_ID_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for FILE_ID_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub union FILE_ID_DESCRIPTOR_0 {
    pub FileId: i64,
    pub ObjectId: ::windows::core::GUID,
    pub ExtendedFileId: FILE_ID_128,
}
impl ::core::marker::Copy for FILE_ID_DESCRIPTOR_0 {}
impl ::core::clone::Clone for FILE_ID_DESCRIPTOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for FILE_ID_DESCRIPTOR_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for FILE_ID_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_ID_EXTD_DIR_INFO {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ReparsePointTag: u32,
    pub FileId: FILE_ID_128,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FILE_ID_EXTD_DIR_INFO {}
impl ::core::clone::Clone for FILE_ID_EXTD_DIR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_ID_EXTD_DIR_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_ID_EXTD_DIR_INFO")
            .field("NextEntryOffset", &self.NextEntryOffset)
            .field("FileIndex", &self.FileIndex)
            .field("CreationTime", &self.CreationTime)
            .field("LastAccessTime", &self.LastAccessTime)
            .field("LastWriteTime", &self.LastWriteTime)
            .field("ChangeTime", &self.ChangeTime)
            .field("EndOfFile", &self.EndOfFile)
            .field("AllocationSize", &self.AllocationSize)
            .field("FileAttributes", &self.FileAttributes)
            .field("FileNameLength", &self.FileNameLength)
            .field("EaSize", &self.EaSize)
            .field("ReparsePointTag", &self.ReparsePointTag)
            .field("FileId", &self.FileId)
            .field("FileName", &self.FileName)
            .finish()
    }
}
impl ::windows::core::TypeKind for FILE_ID_EXTD_DIR_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for FILE_ID_EXTD_DIR_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset && self.FileIndex == other.FileIndex && self.CreationTime == other.CreationTime && self.LastAccessTime == other.LastAccessTime && self.LastWriteTime == other.LastWriteTime && self.ChangeTime == other.ChangeTime && self.EndOfFile == other.EndOfFile && self.AllocationSize == other.AllocationSize && self.FileAttributes == other.FileAttributes && self.FileNameLength == other.FileNameLength && self.EaSize == other.EaSize && self.ReparsePointTag == other.ReparsePointTag && self.FileId == other.FileId && self.FileName == other.FileName
    }
}
impl ::core::cmp::Eq for FILE_ID_EXTD_DIR_INFO {}
impl ::core::default::Default for FILE_ID_EXTD_DIR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_ID_INFO {
    pub VolumeSerialNumber: u64,
    pub FileId: FILE_ID_128,
}
impl ::core::marker::Copy for FILE_ID_INFO {}
impl ::core::clone::Clone for FILE_ID_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_ID_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_ID_INFO").field("VolumeSerialNumber", &self.VolumeSerialNumber).field("FileId", &self.FileId).finish()
    }
}
impl ::windows::core::TypeKind for FILE_ID_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for FILE_ID_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.VolumeSerialNumber == other.VolumeSerialNumber && self.FileId == other.FileId
    }
}
impl ::core::cmp::Eq for FILE_ID_INFO {}
impl ::core::default::Default for FILE_ID_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_INFO_2 {
    pub fi2_id: u32,
}
impl ::core::marker::Copy for FILE_INFO_2 {}
impl ::core::clone::Clone for FILE_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_INFO_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_INFO_2").field("fi2_id", &self.fi2_id).finish()
    }
}
impl ::windows::core::TypeKind for FILE_INFO_2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for FILE_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.fi2_id == other.fi2_id
    }
}
impl ::core::cmp::Eq for FILE_INFO_2 {}
impl ::core::default::Default for FILE_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_INFO_3 {
    pub fi3_id: u32,
    pub fi3_permissions: FILE_INFO_FLAGS_PERMISSIONS,
    pub fi3_num_locks: u32,
    pub fi3_pathname: ::windows::core::PWSTR,
    pub fi3_username: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for FILE_INFO_3 {}
impl ::core::clone::Clone for FILE_INFO_3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_INFO_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_INFO_3").field("fi3_id", &self.fi3_id).field("fi3_permissions", &self.fi3_permissions).field("fi3_num_locks", &self.fi3_num_locks).field("fi3_pathname", &self.fi3_pathname).field("fi3_username", &self.fi3_username).finish()
    }
}
impl ::windows::core::TypeKind for FILE_INFO_3 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for FILE_INFO_3 {
    fn eq(&self, other: &Self) -> bool {
        self.fi3_id == other.fi3_id && self.fi3_permissions == other.fi3_permissions && self.fi3_num_locks == other.fi3_num_locks && self.fi3_pathname == other.fi3_pathname && self.fi3_username == other.fi3_username
    }
}
impl ::core::cmp::Eq for FILE_INFO_3 {}
impl ::core::default::Default for FILE_INFO_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_IO_PRIORITY_HINT_INFO {
    pub PriorityHint: PRIORITY_HINT,
}
impl ::core::marker::Copy for FILE_IO_PRIORITY_HINT_INFO {}
impl ::core::clone::Clone for FILE_IO_PRIORITY_HINT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_IO_PRIORITY_HINT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_IO_PRIORITY_HINT_INFO").field("PriorityHint", &self.PriorityHint).finish()
    }
}
impl ::windows::core::TypeKind for FILE_IO_PRIORITY_HINT_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for FILE_IO_PRIORITY_HINT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.PriorityHint == other.PriorityHint
    }
}
impl ::core::cmp::Eq for FILE_IO_PRIORITY_HINT_INFO {}
impl ::core::default::Default for FILE_IO_PRIORITY_HINT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_NAME_INFO {
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FILE_NAME_INFO {}
impl ::core::clone::Clone for FILE_NAME_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_NAME_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_NAME_INFO").field("FileNameLength", &self.FileNameLength).field("FileName", &self.FileName).finish()
    }
}
impl ::windows::core::TypeKind for FILE_NAME_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for FILE_NAME_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.FileNameLength == other.FileNameLength && self.FileName == other.FileName
    }
}
impl ::core::cmp::Eq for FILE_NAME_INFO {}
impl ::core::default::Default for FILE_NAME_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_NOTIFY_EXTENDED_INFORMATION {
    pub NextEntryOffset: u32,
    pub Action: FILE_ACTION,
    pub CreationTime: i64,
    pub LastModificationTime: i64,
    pub LastChangeTime: i64,
    pub LastAccessTime: i64,
    pub AllocatedLength: i64,
    pub FileSize: i64,
    pub FileAttributes: u32,
    pub Anonymous: FILE_NOTIFY_EXTENDED_INFORMATION_0,
    pub FileId: i64,
    pub ParentFileId: i64,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FILE_NOTIFY_EXTENDED_INFORMATION {}
impl ::core::clone::Clone for FILE_NOTIFY_EXTENDED_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for FILE_NOTIFY_EXTENDED_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for FILE_NOTIFY_EXTENDED_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub union FILE_NOTIFY_EXTENDED_INFORMATION_0 {
    pub ReparsePointTag: u32,
    pub EaSize: u32,
}
impl ::core::marker::Copy for FILE_NOTIFY_EXTENDED_INFORMATION_0 {}
impl ::core::clone::Clone for FILE_NOTIFY_EXTENDED_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for FILE_NOTIFY_EXTENDED_INFORMATION_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for FILE_NOTIFY_EXTENDED_INFORMATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_NOTIFY_INFORMATION {
    pub NextEntryOffset: u32,
    pub Action: FILE_ACTION,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FILE_NOTIFY_INFORMATION {}
impl ::core::clone::Clone for FILE_NOTIFY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_NOTIFY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_NOTIFY_INFORMATION").field("NextEntryOffset", &self.NextEntryOffset).field("Action", &self.Action).field("FileNameLength", &self.FileNameLength).field("FileName", &self.FileName).finish()
    }
}
impl ::windows::core::TypeKind for FILE_NOTIFY_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for FILE_NOTIFY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset && self.Action == other.Action && self.FileNameLength == other.FileNameLength && self.FileName == other.FileName
    }
}
impl ::core::cmp::Eq for FILE_NOTIFY_INFORMATION {}
impl ::core::default::Default for FILE_NOTIFY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_REMOTE_PROTOCOL_INFO {
    pub StructureVersion: u16,
    pub StructureSize: u16,
    pub Protocol: u32,
    pub ProtocolMajorVersion: u16,
    pub ProtocolMinorVersion: u16,
    pub ProtocolRevision: u16,
    pub Reserved: u16,
    pub Flags: u32,
    pub GenericReserved: FILE_REMOTE_PROTOCOL_INFO_0,
    pub ProtocolSpecific: FILE_REMOTE_PROTOCOL_INFO_1,
}
impl ::core::marker::Copy for FILE_REMOTE_PROTOCOL_INFO {}
impl ::core::clone::Clone for FILE_REMOTE_PROTOCOL_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for FILE_REMOTE_PROTOCOL_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for FILE_REMOTE_PROTOCOL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_REMOTE_PROTOCOL_INFO_0 {
    pub Reserved: [u32; 8],
}
impl ::core::marker::Copy for FILE_REMOTE_PROTOCOL_INFO_0 {}
impl ::core::clone::Clone for FILE_REMOTE_PROTOCOL_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_REMOTE_PROTOCOL_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_REMOTE_PROTOCOL_INFO_0").field("Reserved", &self.Reserved).finish()
    }
}
impl ::windows::core::TypeKind for FILE_REMOTE_PROTOCOL_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for FILE_REMOTE_PROTOCOL_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for FILE_REMOTE_PROTOCOL_INFO_0 {}
impl ::core::default::Default for FILE_REMOTE_PROTOCOL_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub union FILE_REMOTE_PROTOCOL_INFO_1 {
    pub Smb2: FILE_REMOTE_PROTOCOL_INFO_1_0,
    pub Reserved: [u32; 16],
}
impl ::core::marker::Copy for FILE_REMOTE_PROTOCOL_INFO_1 {}
impl ::core::clone::Clone for FILE_REMOTE_PROTOCOL_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for FILE_REMOTE_PROTOCOL_INFO_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for FILE_REMOTE_PROTOCOL_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_REMOTE_PROTOCOL_INFO_1_0 {
    pub Server: FILE_REMOTE_PROTOCOL_INFO_1_0_0,
    pub Share: FILE_REMOTE_PROTOCOL_INFO_1_0_1,
}
impl ::core::marker::Copy for FILE_REMOTE_PROTOCOL_INFO_1_0 {}
impl ::core::clone::Clone for FILE_REMOTE_PROTOCOL_INFO_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_REMOTE_PROTOCOL_INFO_1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_REMOTE_PROTOCOL_INFO_1_0").field("Server", &self.Server).field("Share", &self.Share).finish()
    }
}
impl ::windows::core::TypeKind for FILE_REMOTE_PROTOCOL_INFO_1_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for FILE_REMOTE_PROTOCOL_INFO_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Server == other.Server && self.Share == other.Share
    }
}
impl ::core::cmp::Eq for FILE_REMOTE_PROTOCOL_INFO_1_0 {}
impl ::core::default::Default for FILE_REMOTE_PROTOCOL_INFO_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_REMOTE_PROTOCOL_INFO_1_0_0 {
    pub Capabilities: u32,
}
impl ::core::marker::Copy for FILE_REMOTE_PROTOCOL_INFO_1_0_0 {}
impl ::core::clone::Clone for FILE_REMOTE_PROTOCOL_INFO_1_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_REMOTE_PROTOCOL_INFO_1_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_REMOTE_PROTOCOL_INFO_1_0_0").field("Capabilities", &self.Capabilities).finish()
    }
}
impl ::windows::core::TypeKind for FILE_REMOTE_PROTOCOL_INFO_1_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for FILE_REMOTE_PROTOCOL_INFO_1_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Capabilities == other.Capabilities
    }
}
impl ::core::cmp::Eq for FILE_REMOTE_PROTOCOL_INFO_1_0_0 {}
impl ::core::default::Default for FILE_REMOTE_PROTOCOL_INFO_1_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_REMOTE_PROTOCOL_INFO_1_0_1 {
    pub Capabilities: u32,
    pub ShareFlags: u32,
}
impl ::core::marker::Copy for FILE_REMOTE_PROTOCOL_INFO_1_0_1 {}
impl ::core::clone::Clone for FILE_REMOTE_PROTOCOL_INFO_1_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_REMOTE_PROTOCOL_INFO_1_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_REMOTE_PROTOCOL_INFO_1_0_1").field("Capabilities", &self.Capabilities).field("ShareFlags", &self.ShareFlags).finish()
    }
}
impl ::windows::core::TypeKind for FILE_REMOTE_PROTOCOL_INFO_1_0_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for FILE_REMOTE_PROTOCOL_INFO_1_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Capabilities == other.Capabilities && self.ShareFlags == other.ShareFlags
    }
}
impl ::core::cmp::Eq for FILE_REMOTE_PROTOCOL_INFO_1_0_1 {}
impl ::core::default::Default for FILE_REMOTE_PROTOCOL_INFO_1_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_RENAME_INFO {
    pub Anonymous: FILE_RENAME_INFO_0,
    pub RootDirectory: super::super::Foundation::HANDLE,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_RENAME_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_RENAME_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for FILE_RENAME_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FILE_RENAME_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union FILE_RENAME_INFO_0 {
    pub ReplaceIfExists: super::super::Foundation::BOOLEAN,
    pub Flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_RENAME_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_RENAME_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for FILE_RENAME_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FILE_RENAME_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub union FILE_SEGMENT_ELEMENT {
    pub Buffer: *mut ::core::ffi::c_void,
    pub Alignment: u64,
}
impl ::core::marker::Copy for FILE_SEGMENT_ELEMENT {}
impl ::core::clone::Clone for FILE_SEGMENT_ELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for FILE_SEGMENT_ELEMENT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for FILE_SEGMENT_ELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_STANDARD_INFO {
    pub AllocationSize: i64,
    pub EndOfFile: i64,
    pub NumberOfLinks: u32,
    pub DeletePending: super::super::Foundation::BOOLEAN,
    pub Directory: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_STANDARD_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_STANDARD_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FILE_STANDARD_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_STANDARD_INFO").field("AllocationSize", &self.AllocationSize).field("EndOfFile", &self.EndOfFile).field("NumberOfLinks", &self.NumberOfLinks).field("DeletePending", &self.DeletePending).field("Directory", &self.Directory).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for FILE_STANDARD_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FILE_STANDARD_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.AllocationSize == other.AllocationSize && self.EndOfFile == other.EndOfFile && self.NumberOfLinks == other.NumberOfLinks && self.DeletePending == other.DeletePending && self.Directory == other.Directory
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FILE_STANDARD_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FILE_STANDARD_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_STORAGE_INFO {
    pub LogicalBytesPerSector: u32,
    pub PhysicalBytesPerSectorForAtomicity: u32,
    pub PhysicalBytesPerSectorForPerformance: u32,
    pub FileSystemEffectivePhysicalBytesPerSectorForAtomicity: u32,
    pub Flags: u32,
    pub ByteOffsetForSectorAlignment: u32,
    pub ByteOffsetForPartitionAlignment: u32,
}
impl ::core::marker::Copy for FILE_STORAGE_INFO {}
impl ::core::clone::Clone for FILE_STORAGE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_STORAGE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_STORAGE_INFO")
            .field("LogicalBytesPerSector", &self.LogicalBytesPerSector)
            .field("PhysicalBytesPerSectorForAtomicity", &self.PhysicalBytesPerSectorForAtomicity)
            .field("PhysicalBytesPerSectorForPerformance", &self.PhysicalBytesPerSectorForPerformance)
            .field("FileSystemEffectivePhysicalBytesPerSectorForAtomicity", &self.FileSystemEffectivePhysicalBytesPerSectorForAtomicity)
            .field("Flags", &self.Flags)
            .field("ByteOffsetForSectorAlignment", &self.ByteOffsetForSectorAlignment)
            .field("ByteOffsetForPartitionAlignment", &self.ByteOffsetForPartitionAlignment)
            .finish()
    }
}
impl ::windows::core::TypeKind for FILE_STORAGE_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for FILE_STORAGE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.LogicalBytesPerSector == other.LogicalBytesPerSector && self.PhysicalBytesPerSectorForAtomicity == other.PhysicalBytesPerSectorForAtomicity && self.PhysicalBytesPerSectorForPerformance == other.PhysicalBytesPerSectorForPerformance && self.FileSystemEffectivePhysicalBytesPerSectorForAtomicity == other.FileSystemEffectivePhysicalBytesPerSectorForAtomicity && self.Flags == other.Flags && self.ByteOffsetForSectorAlignment == other.ByteOffsetForSectorAlignment && self.ByteOffsetForPartitionAlignment == other.ByteOffsetForPartitionAlignment
    }
}
impl ::core::cmp::Eq for FILE_STORAGE_INFO {}
impl ::core::default::Default for FILE_STORAGE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_STREAM_INFO {
    pub NextEntryOffset: u32,
    pub StreamNameLength: u32,
    pub StreamSize: i64,
    pub StreamAllocationSize: i64,
    pub StreamName: [u16; 1],
}
impl ::core::marker::Copy for FILE_STREAM_INFO {}
impl ::core::clone::Clone for FILE_STREAM_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_STREAM_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_STREAM_INFO").field("NextEntryOffset", &self.NextEntryOffset).field("StreamNameLength", &self.StreamNameLength).field("StreamSize", &self.StreamSize).field("StreamAllocationSize", &self.StreamAllocationSize).field("StreamName", &self.StreamName).finish()
    }
}
impl ::windows::core::TypeKind for FILE_STREAM_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for FILE_STREAM_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset && self.StreamNameLength == other.StreamNameLength && self.StreamSize == other.StreamSize && self.StreamAllocationSize == other.StreamAllocationSize && self.StreamName == other.StreamName
    }
}
impl ::core::cmp::Eq for FILE_STREAM_INFO {}
impl ::core::default::Default for FILE_STREAM_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FIO_CONTEXT {
    pub m_dwTempHack: u32,
    pub m_dwSignature: u32,
    pub m_hFile: super::super::Foundation::HANDLE,
    pub m_dwLinesOffset: u32,
    pub m_dwHeaderLength: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FIO_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FIO_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FIO_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FIO_CONTEXT").field("m_dwTempHack", &self.m_dwTempHack).field("m_dwSignature", &self.m_dwSignature).field("m_hFile", &self.m_hFile).field("m_dwLinesOffset", &self.m_dwLinesOffset).field("m_dwHeaderLength", &self.m_dwHeaderLength).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for FIO_CONTEXT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FIO_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.m_dwTempHack == other.m_dwTempHack && self.m_dwSignature == other.m_dwSignature && self.m_hFile == other.m_hFile && self.m_dwLinesOffset == other.m_dwLinesOffset && self.m_dwHeaderLength == other.m_dwHeaderLength
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FIO_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FIO_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FindChangeNotificationHandle(pub isize);
impl FindChangeNotificationHandle {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for FindChangeNotificationHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for FindChangeNotificationHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for FindChangeNotificationHandle {}
impl ::core::fmt::Debug for FindChangeNotificationHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FindChangeNotificationHandle").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for FindChangeNotificationHandle {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FindFileHandle(pub isize);
impl FindFileHandle {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for FindFileHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for FindFileHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for FindFileHandle {}
impl ::core::fmt::Debug for FindFileHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FindFileHandle").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for FindFileHandle {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FindFileNameHandle(pub isize);
impl FindFileNameHandle {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for FindFileNameHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for FindFileNameHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for FindFileNameHandle {}
impl ::core::fmt::Debug for FindFileNameHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FindFileNameHandle").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for FindFileNameHandle {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FindStreamHandle(pub isize);
impl FindStreamHandle {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for FindStreamHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for FindStreamHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for FindStreamHandle {}
impl ::core::fmt::Debug for FindStreamHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FindStreamHandle").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for FindStreamHandle {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FindVolumeHandle(pub isize);
impl FindVolumeHandle {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for FindVolumeHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for FindVolumeHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for FindVolumeHandle {}
impl ::core::fmt::Debug for FindVolumeHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FindVolumeHandle").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for FindVolumeHandle {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FindVolumeMountPointHandle(pub isize);
impl FindVolumeMountPointHandle {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for FindVolumeMountPointHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for FindVolumeMountPointHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for FindVolumeMountPointHandle {}
impl ::core::fmt::Debug for FindVolumeMountPointHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FindVolumeMountPointHandle").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for FindVolumeMountPointHandle {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct HIORING__ {
    pub unused: i32,
}
impl ::core::marker::Copy for HIORING__ {}
impl ::core::clone::Clone for HIORING__ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HIORING__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HIORING__").field("unused", &self.unused).finish()
    }
}
impl ::windows::core::TypeKind for HIORING__ {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for HIORING__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::core::cmp::Eq for HIORING__ {}
impl ::core::default::Default for HIORING__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct IORING_BUFFER_INFO {
    pub Address: *mut ::core::ffi::c_void,
    pub Length: u32,
}
impl ::core::marker::Copy for IORING_BUFFER_INFO {}
impl ::core::clone::Clone for IORING_BUFFER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IORING_BUFFER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IORING_BUFFER_INFO").field("Address", &self.Address).field("Length", &self.Length).finish()
    }
}
impl ::windows::core::TypeKind for IORING_BUFFER_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IORING_BUFFER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Address == other.Address && self.Length == other.Length
    }
}
impl ::core::cmp::Eq for IORING_BUFFER_INFO {}
impl ::core::default::Default for IORING_BUFFER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct IORING_BUFFER_REF {
    pub Kind: IORING_REF_KIND,
    pub Buffer: IORING_BUFFER_REF_0,
}
impl ::core::marker::Copy for IORING_BUFFER_REF {}
impl ::core::clone::Clone for IORING_BUFFER_REF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for IORING_BUFFER_REF {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for IORING_BUFFER_REF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub union IORING_BUFFER_REF_0 {
    pub Address: *mut ::core::ffi::c_void,
    pub IndexAndOffset: IORING_REGISTERED_BUFFER,
}
impl ::core::marker::Copy for IORING_BUFFER_REF_0 {}
impl ::core::clone::Clone for IORING_BUFFER_REF_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for IORING_BUFFER_REF_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for IORING_BUFFER_REF_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct IORING_CAPABILITIES {
    pub MaxVersion: IORING_VERSION,
    pub MaxSubmissionQueueSize: u32,
    pub MaxCompletionQueueSize: u32,
    pub FeatureFlags: IORING_FEATURE_FLAGS,
}
impl ::core::marker::Copy for IORING_CAPABILITIES {}
impl ::core::clone::Clone for IORING_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IORING_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IORING_CAPABILITIES").field("MaxVersion", &self.MaxVersion).field("MaxSubmissionQueueSize", &self.MaxSubmissionQueueSize).field("MaxCompletionQueueSize", &self.MaxCompletionQueueSize).field("FeatureFlags", &self.FeatureFlags).finish()
    }
}
impl ::windows::core::TypeKind for IORING_CAPABILITIES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IORING_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.MaxVersion == other.MaxVersion && self.MaxSubmissionQueueSize == other.MaxSubmissionQueueSize && self.MaxCompletionQueueSize == other.MaxCompletionQueueSize && self.FeatureFlags == other.FeatureFlags
    }
}
impl ::core::cmp::Eq for IORING_CAPABILITIES {}
impl ::core::default::Default for IORING_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct IORING_CQE {
    pub UserData: usize,
    pub ResultCode: ::windows::core::HRESULT,
    pub Information: usize,
}
impl ::core::marker::Copy for IORING_CQE {}
impl ::core::clone::Clone for IORING_CQE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IORING_CQE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IORING_CQE").field("UserData", &self.UserData).field("ResultCode", &self.ResultCode).field("Information", &self.Information).finish()
    }
}
impl ::windows::core::TypeKind for IORING_CQE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IORING_CQE {
    fn eq(&self, other: &Self) -> bool {
        self.UserData == other.UserData && self.ResultCode == other.ResultCode && self.Information == other.Information
    }
}
impl ::core::cmp::Eq for IORING_CQE {}
impl ::core::default::Default for IORING_CQE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct IORING_CREATE_FLAGS {
    pub Required: IORING_CREATE_REQUIRED_FLAGS,
    pub Advisory: IORING_CREATE_ADVISORY_FLAGS,
}
impl ::core::marker::Copy for IORING_CREATE_FLAGS {}
impl ::core::clone::Clone for IORING_CREATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IORING_CREATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IORING_CREATE_FLAGS").field("Required", &self.Required).field("Advisory", &self.Advisory).finish()
    }
}
impl ::windows::core::TypeKind for IORING_CREATE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IORING_CREATE_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self.Required == other.Required && self.Advisory == other.Advisory
    }
}
impl ::core::cmp::Eq for IORING_CREATE_FLAGS {}
impl ::core::default::Default for IORING_CREATE_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IORING_HANDLE_REF {
    pub Kind: IORING_REF_KIND,
    pub Handle: IORING_HANDLE_REF_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IORING_HANDLE_REF {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IORING_HANDLE_REF {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for IORING_HANDLE_REF {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IORING_HANDLE_REF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union IORING_HANDLE_REF_0 {
    pub Handle: super::super::Foundation::HANDLE,
    pub Index: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IORING_HANDLE_REF_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IORING_HANDLE_REF_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for IORING_HANDLE_REF_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IORING_HANDLE_REF_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct IORING_INFO {
    pub IoRingVersion: IORING_VERSION,
    pub Flags: IORING_CREATE_FLAGS,
    pub SubmissionQueueSize: u32,
    pub CompletionQueueSize: u32,
}
impl ::core::marker::Copy for IORING_INFO {}
impl ::core::clone::Clone for IORING_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IORING_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IORING_INFO").field("IoRingVersion", &self.IoRingVersion).field("Flags", &self.Flags).field("SubmissionQueueSize", &self.SubmissionQueueSize).field("CompletionQueueSize", &self.CompletionQueueSize).finish()
    }
}
impl ::windows::core::TypeKind for IORING_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IORING_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.IoRingVersion == other.IoRingVersion && self.Flags == other.Flags && self.SubmissionQueueSize == other.SubmissionQueueSize && self.CompletionQueueSize == other.CompletionQueueSize
    }
}
impl ::core::cmp::Eq for IORING_INFO {}
impl ::core::default::Default for IORING_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct IORING_REGISTERED_BUFFER {
    pub BufferIndex: u32,
    pub Offset: u32,
}
impl ::core::marker::Copy for IORING_REGISTERED_BUFFER {}
impl ::core::clone::Clone for IORING_REGISTERED_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IORING_REGISTERED_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IORING_REGISTERED_BUFFER").field("BufferIndex", &self.BufferIndex).field("Offset", &self.Offset).finish()
    }
}
impl ::windows::core::TypeKind for IORING_REGISTERED_BUFFER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IORING_REGISTERED_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.BufferIndex == other.BufferIndex && self.Offset == other.Offset
    }
}
impl ::core::cmp::Eq for IORING_REGISTERED_BUFFER {}
impl ::core::default::Default for IORING_REGISTERED_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct KCRM_MARSHAL_HEADER {
    pub VersionMajor: u32,
    pub VersionMinor: u32,
    pub NumProtocols: u32,
    pub Unused: u32,
}
impl ::core::marker::Copy for KCRM_MARSHAL_HEADER {}
impl ::core::clone::Clone for KCRM_MARSHAL_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for KCRM_MARSHAL_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KCRM_MARSHAL_HEADER").field("VersionMajor", &self.VersionMajor).field("VersionMinor", &self.VersionMinor).field("NumProtocols", &self.NumProtocols).field("Unused", &self.Unused).finish()
    }
}
impl ::windows::core::TypeKind for KCRM_MARSHAL_HEADER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for KCRM_MARSHAL_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.VersionMajor == other.VersionMajor && self.VersionMinor == other.VersionMinor && self.NumProtocols == other.NumProtocols && self.Unused == other.Unused
    }
}
impl ::core::cmp::Eq for KCRM_MARSHAL_HEADER {}
impl ::core::default::Default for KCRM_MARSHAL_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct KCRM_PROTOCOL_BLOB {
    pub ProtocolId: ::windows::core::GUID,
    pub StaticInfoLength: u32,
    pub TransactionIdInfoLength: u32,
    pub Unused1: u32,
    pub Unused2: u32,
}
impl ::core::marker::Copy for KCRM_PROTOCOL_BLOB {}
impl ::core::clone::Clone for KCRM_PROTOCOL_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for KCRM_PROTOCOL_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KCRM_PROTOCOL_BLOB").field("ProtocolId", &self.ProtocolId).field("StaticInfoLength", &self.StaticInfoLength).field("TransactionIdInfoLength", &self.TransactionIdInfoLength).field("Unused1", &self.Unused1).field("Unused2", &self.Unused2).finish()
    }
}
impl ::windows::core::TypeKind for KCRM_PROTOCOL_BLOB {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for KCRM_PROTOCOL_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.ProtocolId == other.ProtocolId && self.StaticInfoLength == other.StaticInfoLength && self.TransactionIdInfoLength == other.TransactionIdInfoLength && self.Unused1 == other.Unused1 && self.Unused2 == other.Unused2
    }
}
impl ::core::cmp::Eq for KCRM_PROTOCOL_BLOB {}
impl ::core::default::Default for KCRM_PROTOCOL_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct KCRM_TRANSACTION_BLOB {
    pub UOW: ::windows::core::GUID,
    pub TmIdentity: ::windows::core::GUID,
    pub IsolationLevel: u32,
    pub IsolationFlags: u32,
    pub Timeout: u32,
    pub Description: [u16; 64],
}
impl ::core::marker::Copy for KCRM_TRANSACTION_BLOB {}
impl ::core::clone::Clone for KCRM_TRANSACTION_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for KCRM_TRANSACTION_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KCRM_TRANSACTION_BLOB").field("UOW", &self.UOW).field("TmIdentity", &self.TmIdentity).field("IsolationLevel", &self.IsolationLevel).field("IsolationFlags", &self.IsolationFlags).field("Timeout", &self.Timeout).field("Description", &self.Description).finish()
    }
}
impl ::windows::core::TypeKind for KCRM_TRANSACTION_BLOB {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for KCRM_TRANSACTION_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.UOW == other.UOW && self.TmIdentity == other.TmIdentity && self.IsolationLevel == other.IsolationLevel && self.IsolationFlags == other.IsolationFlags && self.Timeout == other.Timeout && self.Description == other.Description
    }
}
impl ::core::cmp::Eq for KCRM_TRANSACTION_BLOB {}
impl ::core::default::Default for KCRM_TRANSACTION_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LOG_MANAGEMENT_CALLBACKS {
    pub CallbackContext: *mut ::core::ffi::c_void,
    pub AdvanceTailCallback: PLOG_TAIL_ADVANCE_CALLBACK,
    pub LogFullHandlerCallback: PLOG_FULL_HANDLER_CALLBACK,
    pub LogUnpinnedCallback: PLOG_UNPINNED_CALLBACK,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LOG_MANAGEMENT_CALLBACKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LOG_MANAGEMENT_CALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LOG_MANAGEMENT_CALLBACKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOG_MANAGEMENT_CALLBACKS").field("CallbackContext", &self.CallbackContext).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for LOG_MANAGEMENT_CALLBACKS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LOG_MANAGEMENT_CALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct MediaLabelInfo {
    pub LabelType: [u16; 64],
    pub LabelIDSize: u32,
    pub LabelID: [u8; 256],
    pub LabelAppDescr: [u16; 256],
}
impl ::core::marker::Copy for MediaLabelInfo {}
impl ::core::clone::Clone for MediaLabelInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MediaLabelInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MediaLabelInfo").field("LabelType", &self.LabelType).field("LabelIDSize", &self.LabelIDSize).field("LabelID", &self.LabelID).field("LabelAppDescr", &self.LabelAppDescr).finish()
    }
}
impl ::windows::core::TypeKind for MediaLabelInfo {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MediaLabelInfo {
    fn eq(&self, other: &Self) -> bool {
        self.LabelType == other.LabelType && self.LabelIDSize == other.LabelIDSize && self.LabelID == other.LabelID && self.LabelAppDescr == other.LabelAppDescr
    }
}
impl ::core::cmp::Eq for MediaLabelInfo {}
impl ::core::default::Default for MediaLabelInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NAME_CACHE_CONTEXT {
    pub m_dwSignature: u32,
}
impl ::core::marker::Copy for NAME_CACHE_CONTEXT {}
impl ::core::clone::Clone for NAME_CACHE_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NAME_CACHE_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NAME_CACHE_CONTEXT").field("m_dwSignature", &self.m_dwSignature).finish()
    }
}
impl ::windows::core::TypeKind for NAME_CACHE_CONTEXT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NAME_CACHE_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.m_dwSignature == other.m_dwSignature
    }
}
impl ::core::cmp::Eq for NAME_CACHE_CONTEXT {}
impl ::core::default::Default for NAME_CACHE_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_ALLOCATION_INFORMATION {
    pub dwSize: u32,
    pub lpReserved: *mut ::core::ffi::c_void,
    pub AllocatedFrom: ::windows::core::GUID,
}
impl ::core::marker::Copy for NTMS_ALLOCATION_INFORMATION {}
impl ::core::clone::Clone for NTMS_ALLOCATION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_ALLOCATION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_ALLOCATION_INFORMATION").field("dwSize", &self.dwSize).field("lpReserved", &self.lpReserved).field("AllocatedFrom", &self.AllocatedFrom).finish()
    }
}
impl ::windows::core::TypeKind for NTMS_ALLOCATION_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NTMS_ALLOCATION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.lpReserved == other.lpReserved && self.AllocatedFrom == other.AllocatedFrom
    }
}
impl ::core::cmp::Eq for NTMS_ALLOCATION_INFORMATION {}
impl ::core::default::Default for NTMS_ALLOCATION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_ASYNC_IO {
    pub OperationId: ::windows::core::GUID,
    pub EventId: ::windows::core::GUID,
    pub dwOperationType: u32,
    pub dwResult: u32,
    pub dwAsyncState: u32,
    pub hEvent: super::super::Foundation::HANDLE,
    pub bOnStateChange: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_ASYNC_IO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_ASYNC_IO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NTMS_ASYNC_IO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_ASYNC_IO").field("OperationId", &self.OperationId).field("EventId", &self.EventId).field("dwOperationType", &self.dwOperationType).field("dwResult", &self.dwResult).field("dwAsyncState", &self.dwAsyncState).field("hEvent", &self.hEvent).field("bOnStateChange", &self.bOnStateChange).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for NTMS_ASYNC_IO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NTMS_ASYNC_IO {
    fn eq(&self, other: &Self) -> bool {
        self.OperationId == other.OperationId && self.EventId == other.EventId && self.dwOperationType == other.dwOperationType && self.dwResult == other.dwResult && self.dwAsyncState == other.dwAsyncState && self.hEvent == other.hEvent && self.bOnStateChange == other.bOnStateChange
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NTMS_ASYNC_IO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_ASYNC_IO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_CHANGERINFORMATIONA {
    pub Number: u32,
    pub ChangerType: ::windows::core::GUID,
    pub szSerialNumber: [u8; 32],
    pub szRevision: [u8; 32],
    pub szDeviceName: [u8; 64],
    pub ScsiPort: u16,
    pub ScsiBus: u16,
    pub ScsiTarget: u16,
    pub ScsiLun: u16,
    pub Library: ::windows::core::GUID,
}
impl ::core::marker::Copy for NTMS_CHANGERINFORMATIONA {}
impl ::core::clone::Clone for NTMS_CHANGERINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_CHANGERINFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_CHANGERINFORMATIONA").field("Number", &self.Number).field("ChangerType", &self.ChangerType).field("szSerialNumber", &self.szSerialNumber).field("szRevision", &self.szRevision).field("szDeviceName", &self.szDeviceName).field("ScsiPort", &self.ScsiPort).field("ScsiBus", &self.ScsiBus).field("ScsiTarget", &self.ScsiTarget).field("ScsiLun", &self.ScsiLun).field("Library", &self.Library).finish()
    }
}
impl ::windows::core::TypeKind for NTMS_CHANGERINFORMATIONA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NTMS_CHANGERINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.Number == other.Number && self.ChangerType == other.ChangerType && self.szSerialNumber == other.szSerialNumber && self.szRevision == other.szRevision && self.szDeviceName == other.szDeviceName && self.ScsiPort == other.ScsiPort && self.ScsiBus == other.ScsiBus && self.ScsiTarget == other.ScsiTarget && self.ScsiLun == other.ScsiLun && self.Library == other.Library
    }
}
impl ::core::cmp::Eq for NTMS_CHANGERINFORMATIONA {}
impl ::core::default::Default for NTMS_CHANGERINFORMATIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_CHANGERINFORMATIONW {
    pub Number: u32,
    pub ChangerType: ::windows::core::GUID,
    pub szSerialNumber: [u16; 32],
    pub szRevision: [u16; 32],
    pub szDeviceName: [u16; 64],
    pub ScsiPort: u16,
    pub ScsiBus: u16,
    pub ScsiTarget: u16,
    pub ScsiLun: u16,
    pub Library: ::windows::core::GUID,
}
impl ::core::marker::Copy for NTMS_CHANGERINFORMATIONW {}
impl ::core::clone::Clone for NTMS_CHANGERINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_CHANGERINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_CHANGERINFORMATIONW").field("Number", &self.Number).field("ChangerType", &self.ChangerType).field("szSerialNumber", &self.szSerialNumber).field("szRevision", &self.szRevision).field("szDeviceName", &self.szDeviceName).field("ScsiPort", &self.ScsiPort).field("ScsiBus", &self.ScsiBus).field("ScsiTarget", &self.ScsiTarget).field("ScsiLun", &self.ScsiLun).field("Library", &self.Library).finish()
    }
}
impl ::windows::core::TypeKind for NTMS_CHANGERINFORMATIONW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NTMS_CHANGERINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.Number == other.Number && self.ChangerType == other.ChangerType && self.szSerialNumber == other.szSerialNumber && self.szRevision == other.szRevision && self.szDeviceName == other.szDeviceName && self.ScsiPort == other.ScsiPort && self.ScsiBus == other.ScsiBus && self.ScsiTarget == other.ScsiTarget && self.ScsiLun == other.ScsiLun && self.Library == other.Library
    }
}
impl ::core::cmp::Eq for NTMS_CHANGERINFORMATIONW {}
impl ::core::default::Default for NTMS_CHANGERINFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_CHANGERTYPEINFORMATIONA {
    pub szVendor: [u8; 128],
    pub szProduct: [u8; 128],
    pub DeviceType: u32,
}
impl ::core::marker::Copy for NTMS_CHANGERTYPEINFORMATIONA {}
impl ::core::clone::Clone for NTMS_CHANGERTYPEINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_CHANGERTYPEINFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_CHANGERTYPEINFORMATIONA").field("szVendor", &self.szVendor).field("szProduct", &self.szProduct).field("DeviceType", &self.DeviceType).finish()
    }
}
impl ::windows::core::TypeKind for NTMS_CHANGERTYPEINFORMATIONA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NTMS_CHANGERTYPEINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.szVendor == other.szVendor && self.szProduct == other.szProduct && self.DeviceType == other.DeviceType
    }
}
impl ::core::cmp::Eq for NTMS_CHANGERTYPEINFORMATIONA {}
impl ::core::default::Default for NTMS_CHANGERTYPEINFORMATIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_CHANGERTYPEINFORMATIONW {
    pub szVendor: [u16; 128],
    pub szProduct: [u16; 128],
    pub DeviceType: u32,
}
impl ::core::marker::Copy for NTMS_CHANGERTYPEINFORMATIONW {}
impl ::core::clone::Clone for NTMS_CHANGERTYPEINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_CHANGERTYPEINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_CHANGERTYPEINFORMATIONW").field("szVendor", &self.szVendor).field("szProduct", &self.szProduct).field("DeviceType", &self.DeviceType).finish()
    }
}
impl ::windows::core::TypeKind for NTMS_CHANGERTYPEINFORMATIONW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NTMS_CHANGERTYPEINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.szVendor == other.szVendor && self.szProduct == other.szProduct && self.DeviceType == other.DeviceType
    }
}
impl ::core::cmp::Eq for NTMS_CHANGERTYPEINFORMATIONW {}
impl ::core::default::Default for NTMS_CHANGERTYPEINFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_COMPUTERINFORMATION {
    pub dwLibRequestPurgeTime: u32,
    pub dwOpRequestPurgeTime: u32,
    pub dwLibRequestFlags: u32,
    pub dwOpRequestFlags: u32,
    pub dwMediaPoolPolicy: u32,
}
impl ::core::marker::Copy for NTMS_COMPUTERINFORMATION {}
impl ::core::clone::Clone for NTMS_COMPUTERINFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_COMPUTERINFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_COMPUTERINFORMATION").field("dwLibRequestPurgeTime", &self.dwLibRequestPurgeTime).field("dwOpRequestPurgeTime", &self.dwOpRequestPurgeTime).field("dwLibRequestFlags", &self.dwLibRequestFlags).field("dwOpRequestFlags", &self.dwOpRequestFlags).field("dwMediaPoolPolicy", &self.dwMediaPoolPolicy).finish()
    }
}
impl ::windows::core::TypeKind for NTMS_COMPUTERINFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NTMS_COMPUTERINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwLibRequestPurgeTime == other.dwLibRequestPurgeTime && self.dwOpRequestPurgeTime == other.dwOpRequestPurgeTime && self.dwLibRequestFlags == other.dwLibRequestFlags && self.dwOpRequestFlags == other.dwOpRequestFlags && self.dwMediaPoolPolicy == other.dwMediaPoolPolicy
    }
}
impl ::core::cmp::Eq for NTMS_COMPUTERINFORMATION {}
impl ::core::default::Default for NTMS_COMPUTERINFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_DRIVEINFORMATIONA {
    pub Number: u32,
    pub State: NtmsDriveState,
    pub DriveType: ::windows::core::GUID,
    pub szDeviceName: [u8; 64],
    pub szSerialNumber: [u8; 32],
    pub szRevision: [u8; 32],
    pub ScsiPort: u16,
    pub ScsiBus: u16,
    pub ScsiTarget: u16,
    pub ScsiLun: u16,
    pub dwMountCount: u32,
    pub LastCleanedTs: super::super::Foundation::SYSTEMTIME,
    pub SavedPartitionId: ::windows::core::GUID,
    pub Library: ::windows::core::GUID,
    pub Reserved: ::windows::core::GUID,
    pub dwDeferDismountDelay: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_DRIVEINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_DRIVEINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NTMS_DRIVEINFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_DRIVEINFORMATIONA")
            .field("Number", &self.Number)
            .field("State", &self.State)
            .field("DriveType", &self.DriveType)
            .field("szDeviceName", &self.szDeviceName)
            .field("szSerialNumber", &self.szSerialNumber)
            .field("szRevision", &self.szRevision)
            .field("ScsiPort", &self.ScsiPort)
            .field("ScsiBus", &self.ScsiBus)
            .field("ScsiTarget", &self.ScsiTarget)
            .field("ScsiLun", &self.ScsiLun)
            .field("dwMountCount", &self.dwMountCount)
            .field("LastCleanedTs", &self.LastCleanedTs)
            .field("SavedPartitionId", &self.SavedPartitionId)
            .field("Library", &self.Library)
            .field("Reserved", &self.Reserved)
            .field("dwDeferDismountDelay", &self.dwDeferDismountDelay)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for NTMS_DRIVEINFORMATIONA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NTMS_DRIVEINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.Number == other.Number && self.State == other.State && self.DriveType == other.DriveType && self.szDeviceName == other.szDeviceName && self.szSerialNumber == other.szSerialNumber && self.szRevision == other.szRevision && self.ScsiPort == other.ScsiPort && self.ScsiBus == other.ScsiBus && self.ScsiTarget == other.ScsiTarget && self.ScsiLun == other.ScsiLun && self.dwMountCount == other.dwMountCount && self.LastCleanedTs == other.LastCleanedTs && self.SavedPartitionId == other.SavedPartitionId && self.Library == other.Library && self.Reserved == other.Reserved && self.dwDeferDismountDelay == other.dwDeferDismountDelay
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NTMS_DRIVEINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_DRIVEINFORMATIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_DRIVEINFORMATIONW {
    pub Number: u32,
    pub State: NtmsDriveState,
    pub DriveType: ::windows::core::GUID,
    pub szDeviceName: [u16; 64],
    pub szSerialNumber: [u16; 32],
    pub szRevision: [u16; 32],
    pub ScsiPort: u16,
    pub ScsiBus: u16,
    pub ScsiTarget: u16,
    pub ScsiLun: u16,
    pub dwMountCount: u32,
    pub LastCleanedTs: super::super::Foundation::SYSTEMTIME,
    pub SavedPartitionId: ::windows::core::GUID,
    pub Library: ::windows::core::GUID,
    pub Reserved: ::windows::core::GUID,
    pub dwDeferDismountDelay: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_DRIVEINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_DRIVEINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NTMS_DRIVEINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_DRIVEINFORMATIONW")
            .field("Number", &self.Number)
            .field("State", &self.State)
            .field("DriveType", &self.DriveType)
            .field("szDeviceName", &self.szDeviceName)
            .field("szSerialNumber", &self.szSerialNumber)
            .field("szRevision", &self.szRevision)
            .field("ScsiPort", &self.ScsiPort)
            .field("ScsiBus", &self.ScsiBus)
            .field("ScsiTarget", &self.ScsiTarget)
            .field("ScsiLun", &self.ScsiLun)
            .field("dwMountCount", &self.dwMountCount)
            .field("LastCleanedTs", &self.LastCleanedTs)
            .field("SavedPartitionId", &self.SavedPartitionId)
            .field("Library", &self.Library)
            .field("Reserved", &self.Reserved)
            .field("dwDeferDismountDelay", &self.dwDeferDismountDelay)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for NTMS_DRIVEINFORMATIONW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NTMS_DRIVEINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.Number == other.Number && self.State == other.State && self.DriveType == other.DriveType && self.szDeviceName == other.szDeviceName && self.szSerialNumber == other.szSerialNumber && self.szRevision == other.szRevision && self.ScsiPort == other.ScsiPort && self.ScsiBus == other.ScsiBus && self.ScsiTarget == other.ScsiTarget && self.ScsiLun == other.ScsiLun && self.dwMountCount == other.dwMountCount && self.LastCleanedTs == other.LastCleanedTs && self.SavedPartitionId == other.SavedPartitionId && self.Library == other.Library && self.Reserved == other.Reserved && self.dwDeferDismountDelay == other.dwDeferDismountDelay
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NTMS_DRIVEINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_DRIVEINFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_DRIVETYPEINFORMATIONA {
    pub szVendor: [u8; 128],
    pub szProduct: [u8; 128],
    pub NumberOfHeads: u32,
    pub DeviceType: FILE_DEVICE_TYPE,
}
impl ::core::marker::Copy for NTMS_DRIVETYPEINFORMATIONA {}
impl ::core::clone::Clone for NTMS_DRIVETYPEINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_DRIVETYPEINFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_DRIVETYPEINFORMATIONA").field("szVendor", &self.szVendor).field("szProduct", &self.szProduct).field("NumberOfHeads", &self.NumberOfHeads).field("DeviceType", &self.DeviceType).finish()
    }
}
impl ::windows::core::TypeKind for NTMS_DRIVETYPEINFORMATIONA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NTMS_DRIVETYPEINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.szVendor == other.szVendor && self.szProduct == other.szProduct && self.NumberOfHeads == other.NumberOfHeads && self.DeviceType == other.DeviceType
    }
}
impl ::core::cmp::Eq for NTMS_DRIVETYPEINFORMATIONA {}
impl ::core::default::Default for NTMS_DRIVETYPEINFORMATIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_DRIVETYPEINFORMATIONW {
    pub szVendor: [u16; 128],
    pub szProduct: [u16; 128],
    pub NumberOfHeads: u32,
    pub DeviceType: FILE_DEVICE_TYPE,
}
impl ::core::marker::Copy for NTMS_DRIVETYPEINFORMATIONW {}
impl ::core::clone::Clone for NTMS_DRIVETYPEINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_DRIVETYPEINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_DRIVETYPEINFORMATIONW").field("szVendor", &self.szVendor).field("szProduct", &self.szProduct).field("NumberOfHeads", &self.NumberOfHeads).field("DeviceType", &self.DeviceType).finish()
    }
}
impl ::windows::core::TypeKind for NTMS_DRIVETYPEINFORMATIONW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NTMS_DRIVETYPEINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.szVendor == other.szVendor && self.szProduct == other.szProduct && self.NumberOfHeads == other.NumberOfHeads && self.DeviceType == other.DeviceType
    }
}
impl ::core::cmp::Eq for NTMS_DRIVETYPEINFORMATIONW {}
impl ::core::default::Default for NTMS_DRIVETYPEINFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_FILESYSTEM_INFO {
    pub FileSystemType: [u16; 64],
    pub VolumeName: [u16; 256],
    pub SerialNumber: u32,
}
impl ::core::marker::Copy for NTMS_FILESYSTEM_INFO {}
impl ::core::clone::Clone for NTMS_FILESYSTEM_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_FILESYSTEM_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_FILESYSTEM_INFO").field("FileSystemType", &self.FileSystemType).field("VolumeName", &self.VolumeName).field("SerialNumber", &self.SerialNumber).finish()
    }
}
impl ::windows::core::TypeKind for NTMS_FILESYSTEM_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NTMS_FILESYSTEM_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.FileSystemType == other.FileSystemType && self.VolumeName == other.VolumeName && self.SerialNumber == other.SerialNumber
    }
}
impl ::core::cmp::Eq for NTMS_FILESYSTEM_INFO {}
impl ::core::default::Default for NTMS_FILESYSTEM_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_I1_LIBRARYINFORMATION {
    pub LibraryType: u32,
    pub CleanerSlot: ::windows::core::GUID,
    pub CleanerSlotDefault: ::windows::core::GUID,
    pub LibrarySupportsDriveCleaning: super::super::Foundation::BOOL,
    pub BarCodeReaderInstalled: super::super::Foundation::BOOL,
    pub InventoryMethod: u32,
    pub dwCleanerUsesRemaining: u32,
    pub FirstDriveNumber: u32,
    pub dwNumberOfDrives: u32,
    pub FirstSlotNumber: u32,
    pub dwNumberOfSlots: u32,
    pub FirstDoorNumber: u32,
    pub dwNumberOfDoors: u32,
    pub FirstPortNumber: u32,
    pub dwNumberOfPorts: u32,
    pub FirstChangerNumber: u32,
    pub dwNumberOfChangers: u32,
    pub dwNumberOfMedia: u32,
    pub dwNumberOfMediaTypes: u32,
    pub dwNumberOfLibRequests: u32,
    pub Reserved: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_I1_LIBRARYINFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_I1_LIBRARYINFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NTMS_I1_LIBRARYINFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_I1_LIBRARYINFORMATION")
            .field("LibraryType", &self.LibraryType)
            .field("CleanerSlot", &self.CleanerSlot)
            .field("CleanerSlotDefault", &self.CleanerSlotDefault)
            .field("LibrarySupportsDriveCleaning", &self.LibrarySupportsDriveCleaning)
            .field("BarCodeReaderInstalled", &self.BarCodeReaderInstalled)
            .field("InventoryMethod", &self.InventoryMethod)
            .field("dwCleanerUsesRemaining", &self.dwCleanerUsesRemaining)
            .field("FirstDriveNumber", &self.FirstDriveNumber)
            .field("dwNumberOfDrives", &self.dwNumberOfDrives)
            .field("FirstSlotNumber", &self.FirstSlotNumber)
            .field("dwNumberOfSlots", &self.dwNumberOfSlots)
            .field("FirstDoorNumber", &self.FirstDoorNumber)
            .field("dwNumberOfDoors", &self.dwNumberOfDoors)
            .field("FirstPortNumber", &self.FirstPortNumber)
            .field("dwNumberOfPorts", &self.dwNumberOfPorts)
            .field("FirstChangerNumber", &self.FirstChangerNumber)
            .field("dwNumberOfChangers", &self.dwNumberOfChangers)
            .field("dwNumberOfMedia", &self.dwNumberOfMedia)
            .field("dwNumberOfMediaTypes", &self.dwNumberOfMediaTypes)
            .field("dwNumberOfLibRequests", &self.dwNumberOfLibRequests)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for NTMS_I1_LIBRARYINFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NTMS_I1_LIBRARYINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.LibraryType == other.LibraryType
            && self.CleanerSlot == other.CleanerSlot
            && self.CleanerSlotDefault == other.CleanerSlotDefault
            && self.LibrarySupportsDriveCleaning == other.LibrarySupportsDriveCleaning
            && self.BarCodeReaderInstalled == other.BarCodeReaderInstalled
            && self.InventoryMethod == other.InventoryMethod
            && self.dwCleanerUsesRemaining == other.dwCleanerUsesRemaining
            && self.FirstDriveNumber == other.FirstDriveNumber
            && self.dwNumberOfDrives == other.dwNumberOfDrives
            && self.FirstSlotNumber == other.FirstSlotNumber
            && self.dwNumberOfSlots == other.dwNumberOfSlots
            && self.FirstDoorNumber == other.FirstDoorNumber
            && self.dwNumberOfDoors == other.dwNumberOfDoors
            && self.FirstPortNumber == other.FirstPortNumber
            && self.dwNumberOfPorts == other.dwNumberOfPorts
            && self.FirstChangerNumber == other.FirstChangerNumber
            && self.dwNumberOfChangers == other.dwNumberOfChangers
            && self.dwNumberOfMedia == other.dwNumberOfMedia
            && self.dwNumberOfMediaTypes == other.dwNumberOfMediaTypes
            && self.dwNumberOfLibRequests == other.dwNumberOfLibRequests
            && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NTMS_I1_LIBRARYINFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_I1_LIBRARYINFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_I1_LIBREQUESTINFORMATIONA {
    pub OperationCode: u32,
    pub OperationOption: u32,
    pub State: u32,
    pub PartitionId: ::windows::core::GUID,
    pub DriveId: ::windows::core::GUID,
    pub PhysMediaId: ::windows::core::GUID,
    pub Library: ::windows::core::GUID,
    pub SlotId: ::windows::core::GUID,
    pub TimeQueued: super::super::Foundation::SYSTEMTIME,
    pub TimeCompleted: super::super::Foundation::SYSTEMTIME,
    pub szApplication: [u8; 64],
    pub szUser: [u8; 64],
    pub szComputer: [u8; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_I1_LIBREQUESTINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_I1_LIBREQUESTINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NTMS_I1_LIBREQUESTINFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_I1_LIBREQUESTINFORMATIONA")
            .field("OperationCode", &self.OperationCode)
            .field("OperationOption", &self.OperationOption)
            .field("State", &self.State)
            .field("PartitionId", &self.PartitionId)
            .field("DriveId", &self.DriveId)
            .field("PhysMediaId", &self.PhysMediaId)
            .field("Library", &self.Library)
            .field("SlotId", &self.SlotId)
            .field("TimeQueued", &self.TimeQueued)
            .field("TimeCompleted", &self.TimeCompleted)
            .field("szApplication", &self.szApplication)
            .field("szUser", &self.szUser)
            .field("szComputer", &self.szComputer)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for NTMS_I1_LIBREQUESTINFORMATIONA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NTMS_I1_LIBREQUESTINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.OperationCode == other.OperationCode && self.OperationOption == other.OperationOption && self.State == other.State && self.PartitionId == other.PartitionId && self.DriveId == other.DriveId && self.PhysMediaId == other.PhysMediaId && self.Library == other.Library && self.SlotId == other.SlotId && self.TimeQueued == other.TimeQueued && self.TimeCompleted == other.TimeCompleted && self.szApplication == other.szApplication && self.szUser == other.szUser && self.szComputer == other.szComputer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NTMS_I1_LIBREQUESTINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_I1_LIBREQUESTINFORMATIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_I1_LIBREQUESTINFORMATIONW {
    pub OperationCode: u32,
    pub OperationOption: u32,
    pub State: u32,
    pub PartitionId: ::windows::core::GUID,
    pub DriveId: ::windows::core::GUID,
    pub PhysMediaId: ::windows::core::GUID,
    pub Library: ::windows::core::GUID,
    pub SlotId: ::windows::core::GUID,
    pub TimeQueued: super::super::Foundation::SYSTEMTIME,
    pub TimeCompleted: super::super::Foundation::SYSTEMTIME,
    pub szApplication: [u16; 64],
    pub szUser: [u16; 64],
    pub szComputer: [u16; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_I1_LIBREQUESTINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_I1_LIBREQUESTINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NTMS_I1_LIBREQUESTINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_I1_LIBREQUESTINFORMATIONW")
            .field("OperationCode", &self.OperationCode)
            .field("OperationOption", &self.OperationOption)
            .field("State", &self.State)
            .field("PartitionId", &self.PartitionId)
            .field("DriveId", &self.DriveId)
            .field("PhysMediaId", &self.PhysMediaId)
            .field("Library", &self.Library)
            .field("SlotId", &self.SlotId)
            .field("TimeQueued", &self.TimeQueued)
            .field("TimeCompleted", &self.TimeCompleted)
            .field("szApplication", &self.szApplication)
            .field("szUser", &self.szUser)
            .field("szComputer", &self.szComputer)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for NTMS_I1_LIBREQUESTINFORMATIONW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NTMS_I1_LIBREQUESTINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.OperationCode == other.OperationCode && self.OperationOption == other.OperationOption && self.State == other.State && self.PartitionId == other.PartitionId && self.DriveId == other.DriveId && self.PhysMediaId == other.PhysMediaId && self.Library == other.Library && self.SlotId == other.SlotId && self.TimeQueued == other.TimeQueued && self.TimeCompleted == other.TimeCompleted && self.szApplication == other.szApplication && self.szUser == other.szUser && self.szComputer == other.szComputer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NTMS_I1_LIBREQUESTINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_I1_LIBREQUESTINFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_I1_OBJECTINFORMATIONA {
    pub dwSize: u32,
    pub dwType: u32,
    pub Created: super::super::Foundation::SYSTEMTIME,
    pub Modified: super::super::Foundation::SYSTEMTIME,
    pub ObjectGuid: ::windows::core::GUID,
    pub Enabled: super::super::Foundation::BOOL,
    pub dwOperationalState: u32,
    pub szName: [u8; 64],
    pub szDescription: [u8; 127],
    pub Info: NTMS_I1_OBJECTINFORMATIONA_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_I1_OBJECTINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_I1_OBJECTINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for NTMS_I1_OBJECTINFORMATIONA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_I1_OBJECTINFORMATIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union NTMS_I1_OBJECTINFORMATIONA_0 {
    pub Drive: NTMS_DRIVEINFORMATIONA,
    pub DriveType: NTMS_DRIVETYPEINFORMATIONA,
    pub Library: NTMS_I1_LIBRARYINFORMATION,
    pub Changer: NTMS_CHANGERINFORMATIONA,
    pub ChangerType: NTMS_CHANGERTYPEINFORMATIONA,
    pub StorageSlot: NTMS_STORAGESLOTINFORMATION,
    pub IEDoor: NTMS_IEDOORINFORMATION,
    pub IEPort: NTMS_IEPORTINFORMATION,
    pub PhysicalMedia: NTMS_I1_PMIDINFORMATIONA,
    pub LogicalMedia: NTMS_LMIDINFORMATION,
    pub Partition: NTMS_I1_PARTITIONINFORMATIONA,
    pub MediaPool: NTMS_MEDIAPOOLINFORMATION,
    pub MediaType: NTMS_MEDIATYPEINFORMATION,
    pub LibRequest: NTMS_I1_LIBREQUESTINFORMATIONA,
    pub OpRequest: NTMS_I1_OPREQUESTINFORMATIONA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_I1_OBJECTINFORMATIONA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_I1_OBJECTINFORMATIONA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for NTMS_I1_OBJECTINFORMATIONA_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_I1_OBJECTINFORMATIONA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_I1_OBJECTINFORMATIONW {
    pub dwSize: u32,
    pub dwType: u32,
    pub Created: super::super::Foundation::SYSTEMTIME,
    pub Modified: super::super::Foundation::SYSTEMTIME,
    pub ObjectGuid: ::windows::core::GUID,
    pub Enabled: super::super::Foundation::BOOL,
    pub dwOperationalState: u32,
    pub szName: [u16; 64],
    pub szDescription: [u16; 127],
    pub Info: NTMS_I1_OBJECTINFORMATIONW_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_I1_OBJECTINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_I1_OBJECTINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for NTMS_I1_OBJECTINFORMATIONW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_I1_OBJECTINFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union NTMS_I1_OBJECTINFORMATIONW_0 {
    pub Drive: NTMS_DRIVEINFORMATIONW,
    pub DriveType: NTMS_DRIVETYPEINFORMATIONW,
    pub Library: NTMS_I1_LIBRARYINFORMATION,
    pub Changer: NTMS_CHANGERINFORMATIONW,
    pub ChangerType: NTMS_CHANGERTYPEINFORMATIONW,
    pub StorageSlot: NTMS_STORAGESLOTINFORMATION,
    pub IEDoor: NTMS_IEDOORINFORMATION,
    pub IEPort: NTMS_IEPORTINFORMATION,
    pub PhysicalMedia: NTMS_I1_PMIDINFORMATIONW,
    pub LogicalMedia: NTMS_LMIDINFORMATION,
    pub Partition: NTMS_I1_PARTITIONINFORMATIONW,
    pub MediaPool: NTMS_MEDIAPOOLINFORMATION,
    pub MediaType: NTMS_MEDIATYPEINFORMATION,
    pub LibRequest: NTMS_I1_LIBREQUESTINFORMATIONW,
    pub OpRequest: NTMS_I1_OPREQUESTINFORMATIONW,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_I1_OBJECTINFORMATIONW_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_I1_OBJECTINFORMATIONW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for NTMS_I1_OBJECTINFORMATIONW_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_I1_OBJECTINFORMATIONW_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_I1_OPREQUESTINFORMATIONA {
    pub Request: u32,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
    pub State: u32,
    pub szMessage: [u8; 127],
    pub Arg1Type: u32,
    pub Arg1: ::windows::core::GUID,
    pub Arg2Type: u32,
    pub Arg2: ::windows::core::GUID,
    pub szApplication: [u8; 64],
    pub szUser: [u8; 64],
    pub szComputer: [u8; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_I1_OPREQUESTINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_I1_OPREQUESTINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NTMS_I1_OPREQUESTINFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_I1_OPREQUESTINFORMATIONA").field("Request", &self.Request).field("Submitted", &self.Submitted).field("State", &self.State).field("szMessage", &self.szMessage).field("Arg1Type", &self.Arg1Type).field("Arg1", &self.Arg1).field("Arg2Type", &self.Arg2Type).field("Arg2", &self.Arg2).field("szApplication", &self.szApplication).field("szUser", &self.szUser).field("szComputer", &self.szComputer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for NTMS_I1_OPREQUESTINFORMATIONA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NTMS_I1_OPREQUESTINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.Request == other.Request && self.Submitted == other.Submitted && self.State == other.State && self.szMessage == other.szMessage && self.Arg1Type == other.Arg1Type && self.Arg1 == other.Arg1 && self.Arg2Type == other.Arg2Type && self.Arg2 == other.Arg2 && self.szApplication == other.szApplication && self.szUser == other.szUser && self.szComputer == other.szComputer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NTMS_I1_OPREQUESTINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_I1_OPREQUESTINFORMATIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_I1_OPREQUESTINFORMATIONW {
    pub Request: u32,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
    pub State: u32,
    pub szMessage: [u16; 127],
    pub Arg1Type: u32,
    pub Arg1: ::windows::core::GUID,
    pub Arg2Type: u32,
    pub Arg2: ::windows::core::GUID,
    pub szApplication: [u16; 64],
    pub szUser: [u16; 64],
    pub szComputer: [u16; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_I1_OPREQUESTINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_I1_OPREQUESTINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NTMS_I1_OPREQUESTINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_I1_OPREQUESTINFORMATIONW").field("Request", &self.Request).field("Submitted", &self.Submitted).field("State", &self.State).field("szMessage", &self.szMessage).field("Arg1Type", &self.Arg1Type).field("Arg1", &self.Arg1).field("Arg2Type", &self.Arg2Type).field("Arg2", &self.Arg2).field("szApplication", &self.szApplication).field("szUser", &self.szUser).field("szComputer", &self.szComputer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for NTMS_I1_OPREQUESTINFORMATIONW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NTMS_I1_OPREQUESTINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.Request == other.Request && self.Submitted == other.Submitted && self.State == other.State && self.szMessage == other.szMessage && self.Arg1Type == other.Arg1Type && self.Arg1 == other.Arg1 && self.Arg2Type == other.Arg2Type && self.Arg2 == other.Arg2 && self.szApplication == other.szApplication && self.szUser == other.szUser && self.szComputer == other.szComputer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NTMS_I1_OPREQUESTINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_I1_OPREQUESTINFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_I1_PARTITIONINFORMATIONA {
    pub PhysicalMedia: ::windows::core::GUID,
    pub LogicalMedia: ::windows::core::GUID,
    pub State: u32,
    pub Side: u16,
    pub dwOmidLabelIdLength: u32,
    pub OmidLabelId: [u8; 255],
    pub szOmidLabelType: [u8; 64],
    pub szOmidLabelInfo: [u8; 256],
    pub dwMountCount: u32,
    pub dwAllocateCount: u32,
}
impl ::core::marker::Copy for NTMS_I1_PARTITIONINFORMATIONA {}
impl ::core::clone::Clone for NTMS_I1_PARTITIONINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_I1_PARTITIONINFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_I1_PARTITIONINFORMATIONA").field("PhysicalMedia", &self.PhysicalMedia).field("LogicalMedia", &self.LogicalMedia).field("State", &self.State).field("Side", &self.Side).field("dwOmidLabelIdLength", &self.dwOmidLabelIdLength).field("OmidLabelId", &self.OmidLabelId).field("szOmidLabelType", &self.szOmidLabelType).field("szOmidLabelInfo", &self.szOmidLabelInfo).field("dwMountCount", &self.dwMountCount).field("dwAllocateCount", &self.dwAllocateCount).finish()
    }
}
impl ::windows::core::TypeKind for NTMS_I1_PARTITIONINFORMATIONA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NTMS_I1_PARTITIONINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.PhysicalMedia == other.PhysicalMedia && self.LogicalMedia == other.LogicalMedia && self.State == other.State && self.Side == other.Side && self.dwOmidLabelIdLength == other.dwOmidLabelIdLength && self.OmidLabelId == other.OmidLabelId && self.szOmidLabelType == other.szOmidLabelType && self.szOmidLabelInfo == other.szOmidLabelInfo && self.dwMountCount == other.dwMountCount && self.dwAllocateCount == other.dwAllocateCount
    }
}
impl ::core::cmp::Eq for NTMS_I1_PARTITIONINFORMATIONA {}
impl ::core::default::Default for NTMS_I1_PARTITIONINFORMATIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_I1_PARTITIONINFORMATIONW {
    pub PhysicalMedia: ::windows::core::GUID,
    pub LogicalMedia: ::windows::core::GUID,
    pub State: u32,
    pub Side: u16,
    pub dwOmidLabelIdLength: u32,
    pub OmidLabelId: [u8; 255],
    pub szOmidLabelType: [u16; 64],
    pub szOmidLabelInfo: [u16; 256],
    pub dwMountCount: u32,
    pub dwAllocateCount: u32,
}
impl ::core::marker::Copy for NTMS_I1_PARTITIONINFORMATIONW {}
impl ::core::clone::Clone for NTMS_I1_PARTITIONINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_I1_PARTITIONINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_I1_PARTITIONINFORMATIONW").field("PhysicalMedia", &self.PhysicalMedia).field("LogicalMedia", &self.LogicalMedia).field("State", &self.State).field("Side", &self.Side).field("dwOmidLabelIdLength", &self.dwOmidLabelIdLength).field("OmidLabelId", &self.OmidLabelId).field("szOmidLabelType", &self.szOmidLabelType).field("szOmidLabelInfo", &self.szOmidLabelInfo).field("dwMountCount", &self.dwMountCount).field("dwAllocateCount", &self.dwAllocateCount).finish()
    }
}
impl ::windows::core::TypeKind for NTMS_I1_PARTITIONINFORMATIONW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NTMS_I1_PARTITIONINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.PhysicalMedia == other.PhysicalMedia && self.LogicalMedia == other.LogicalMedia && self.State == other.State && self.Side == other.Side && self.dwOmidLabelIdLength == other.dwOmidLabelIdLength && self.OmidLabelId == other.OmidLabelId && self.szOmidLabelType == other.szOmidLabelType && self.szOmidLabelInfo == other.szOmidLabelInfo && self.dwMountCount == other.dwMountCount && self.dwAllocateCount == other.dwAllocateCount
    }
}
impl ::core::cmp::Eq for NTMS_I1_PARTITIONINFORMATIONW {}
impl ::core::default::Default for NTMS_I1_PARTITIONINFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_I1_PMIDINFORMATIONA {
    pub CurrentLibrary: ::windows::core::GUID,
    pub MediaPool: ::windows::core::GUID,
    pub Location: ::windows::core::GUID,
    pub LocationType: u32,
    pub MediaType: ::windows::core::GUID,
    pub HomeSlot: ::windows::core::GUID,
    pub szBarCode: [u8; 64],
    pub BarCodeState: u32,
    pub szSequenceNumber: [u8; 32],
    pub MediaState: u32,
    pub dwNumberOfPartitions: u32,
}
impl ::core::marker::Copy for NTMS_I1_PMIDINFORMATIONA {}
impl ::core::clone::Clone for NTMS_I1_PMIDINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_I1_PMIDINFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_I1_PMIDINFORMATIONA")
            .field("CurrentLibrary", &self.CurrentLibrary)
            .field("MediaPool", &self.MediaPool)
            .field("Location", &self.Location)
            .field("LocationType", &self.LocationType)
            .field("MediaType", &self.MediaType)
            .field("HomeSlot", &self.HomeSlot)
            .field("szBarCode", &self.szBarCode)
            .field("BarCodeState", &self.BarCodeState)
            .field("szSequenceNumber", &self.szSequenceNumber)
            .field("MediaState", &self.MediaState)
            .field("dwNumberOfPartitions", &self.dwNumberOfPartitions)
            .finish()
    }
}
impl ::windows::core::TypeKind for NTMS_I1_PMIDINFORMATIONA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NTMS_I1_PMIDINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.CurrentLibrary == other.CurrentLibrary && self.MediaPool == other.MediaPool && self.Location == other.Location && self.LocationType == other.LocationType && self.MediaType == other.MediaType && self.HomeSlot == other.HomeSlot && self.szBarCode == other.szBarCode && self.BarCodeState == other.BarCodeState && self.szSequenceNumber == other.szSequenceNumber && self.MediaState == other.MediaState && self.dwNumberOfPartitions == other.dwNumberOfPartitions
    }
}
impl ::core::cmp::Eq for NTMS_I1_PMIDINFORMATIONA {}
impl ::core::default::Default for NTMS_I1_PMIDINFORMATIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_I1_PMIDINFORMATIONW {
    pub CurrentLibrary: ::windows::core::GUID,
    pub MediaPool: ::windows::core::GUID,
    pub Location: ::windows::core::GUID,
    pub LocationType: u32,
    pub MediaType: ::windows::core::GUID,
    pub HomeSlot: ::windows::core::GUID,
    pub szBarCode: [u16; 64],
    pub BarCodeState: u32,
    pub szSequenceNumber: [u16; 32],
    pub MediaState: u32,
    pub dwNumberOfPartitions: u32,
}
impl ::core::marker::Copy for NTMS_I1_PMIDINFORMATIONW {}
impl ::core::clone::Clone for NTMS_I1_PMIDINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_I1_PMIDINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_I1_PMIDINFORMATIONW")
            .field("CurrentLibrary", &self.CurrentLibrary)
            .field("MediaPool", &self.MediaPool)
            .field("Location", &self.Location)
            .field("LocationType", &self.LocationType)
            .field("MediaType", &self.MediaType)
            .field("HomeSlot", &self.HomeSlot)
            .field("szBarCode", &self.szBarCode)
            .field("BarCodeState", &self.BarCodeState)
            .field("szSequenceNumber", &self.szSequenceNumber)
            .field("MediaState", &self.MediaState)
            .field("dwNumberOfPartitions", &self.dwNumberOfPartitions)
            .finish()
    }
}
impl ::windows::core::TypeKind for NTMS_I1_PMIDINFORMATIONW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NTMS_I1_PMIDINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.CurrentLibrary == other.CurrentLibrary && self.MediaPool == other.MediaPool && self.Location == other.Location && self.LocationType == other.LocationType && self.MediaType == other.MediaType && self.HomeSlot == other.HomeSlot && self.szBarCode == other.szBarCode && self.BarCodeState == other.BarCodeState && self.szSequenceNumber == other.szSequenceNumber && self.MediaState == other.MediaState && self.dwNumberOfPartitions == other.dwNumberOfPartitions
    }
}
impl ::core::cmp::Eq for NTMS_I1_PMIDINFORMATIONW {}
impl ::core::default::Default for NTMS_I1_PMIDINFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_IEDOORINFORMATION {
    pub Number: u32,
    pub State: NtmsDoorState,
    pub MaxOpenSecs: u16,
    pub Library: ::windows::core::GUID,
}
impl ::core::marker::Copy for NTMS_IEDOORINFORMATION {}
impl ::core::clone::Clone for NTMS_IEDOORINFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_IEDOORINFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_IEDOORINFORMATION").field("Number", &self.Number).field("State", &self.State).field("MaxOpenSecs", &self.MaxOpenSecs).field("Library", &self.Library).finish()
    }
}
impl ::windows::core::TypeKind for NTMS_IEDOORINFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NTMS_IEDOORINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Number == other.Number && self.State == other.State && self.MaxOpenSecs == other.MaxOpenSecs && self.Library == other.Library
    }
}
impl ::core::cmp::Eq for NTMS_IEDOORINFORMATION {}
impl ::core::default::Default for NTMS_IEDOORINFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_IEPORTINFORMATION {
    pub Number: u32,
    pub Content: NtmsPortContent,
    pub Position: NtmsPortPosition,
    pub MaxExtendSecs: u16,
    pub Library: ::windows::core::GUID,
}
impl ::core::marker::Copy for NTMS_IEPORTINFORMATION {}
impl ::core::clone::Clone for NTMS_IEPORTINFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_IEPORTINFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_IEPORTINFORMATION").field("Number", &self.Number).field("Content", &self.Content).field("Position", &self.Position).field("MaxExtendSecs", &self.MaxExtendSecs).field("Library", &self.Library).finish()
    }
}
impl ::windows::core::TypeKind for NTMS_IEPORTINFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NTMS_IEPORTINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Number == other.Number && self.Content == other.Content && self.Position == other.Position && self.MaxExtendSecs == other.MaxExtendSecs && self.Library == other.Library
    }
}
impl ::core::cmp::Eq for NTMS_IEPORTINFORMATION {}
impl ::core::default::Default for NTMS_IEPORTINFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_LIBRARYINFORMATION {
    pub LibraryType: NtmsLibraryType,
    pub CleanerSlot: ::windows::core::GUID,
    pub CleanerSlotDefault: ::windows::core::GUID,
    pub LibrarySupportsDriveCleaning: super::super::Foundation::BOOL,
    pub BarCodeReaderInstalled: super::super::Foundation::BOOL,
    pub InventoryMethod: NtmsInventoryMethod,
    pub dwCleanerUsesRemaining: u32,
    pub FirstDriveNumber: u32,
    pub dwNumberOfDrives: u32,
    pub FirstSlotNumber: u32,
    pub dwNumberOfSlots: u32,
    pub FirstDoorNumber: u32,
    pub dwNumberOfDoors: u32,
    pub FirstPortNumber: u32,
    pub dwNumberOfPorts: u32,
    pub FirstChangerNumber: u32,
    pub dwNumberOfChangers: u32,
    pub dwNumberOfMedia: u32,
    pub dwNumberOfMediaTypes: u32,
    pub dwNumberOfLibRequests: u32,
    pub Reserved: ::windows::core::GUID,
    pub AutoRecovery: super::super::Foundation::BOOL,
    pub dwFlags: NtmsLibraryFlags,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_LIBRARYINFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_LIBRARYINFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NTMS_LIBRARYINFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_LIBRARYINFORMATION")
            .field("LibraryType", &self.LibraryType)
            .field("CleanerSlot", &self.CleanerSlot)
            .field("CleanerSlotDefault", &self.CleanerSlotDefault)
            .field("LibrarySupportsDriveCleaning", &self.LibrarySupportsDriveCleaning)
            .field("BarCodeReaderInstalled", &self.BarCodeReaderInstalled)
            .field("InventoryMethod", &self.InventoryMethod)
            .field("dwCleanerUsesRemaining", &self.dwCleanerUsesRemaining)
            .field("FirstDriveNumber", &self.FirstDriveNumber)
            .field("dwNumberOfDrives", &self.dwNumberOfDrives)
            .field("FirstSlotNumber", &self.FirstSlotNumber)
            .field("dwNumberOfSlots", &self.dwNumberOfSlots)
            .field("FirstDoorNumber", &self.FirstDoorNumber)
            .field("dwNumberOfDoors", &self.dwNumberOfDoors)
            .field("FirstPortNumber", &self.FirstPortNumber)
            .field("dwNumberOfPorts", &self.dwNumberOfPorts)
            .field("FirstChangerNumber", &self.FirstChangerNumber)
            .field("dwNumberOfChangers", &self.dwNumberOfChangers)
            .field("dwNumberOfMedia", &self.dwNumberOfMedia)
            .field("dwNumberOfMediaTypes", &self.dwNumberOfMediaTypes)
            .field("dwNumberOfLibRequests", &self.dwNumberOfLibRequests)
            .field("Reserved", &self.Reserved)
            .field("AutoRecovery", &self.AutoRecovery)
            .field("dwFlags", &self.dwFlags)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for NTMS_LIBRARYINFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NTMS_LIBRARYINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.LibraryType == other.LibraryType
            && self.CleanerSlot == other.CleanerSlot
            && self.CleanerSlotDefault == other.CleanerSlotDefault
            && self.LibrarySupportsDriveCleaning == other.LibrarySupportsDriveCleaning
            && self.BarCodeReaderInstalled == other.BarCodeReaderInstalled
            && self.InventoryMethod == other.InventoryMethod
            && self.dwCleanerUsesRemaining == other.dwCleanerUsesRemaining
            && self.FirstDriveNumber == other.FirstDriveNumber
            && self.dwNumberOfDrives == other.dwNumberOfDrives
            && self.FirstSlotNumber == other.FirstSlotNumber
            && self.dwNumberOfSlots == other.dwNumberOfSlots
            && self.FirstDoorNumber == other.FirstDoorNumber
            && self.dwNumberOfDoors == other.dwNumberOfDoors
            && self.FirstPortNumber == other.FirstPortNumber
            && self.dwNumberOfPorts == other.dwNumberOfPorts
            && self.FirstChangerNumber == other.FirstChangerNumber
            && self.dwNumberOfChangers == other.dwNumberOfChangers
            && self.dwNumberOfMedia == other.dwNumberOfMedia
            && self.dwNumberOfMediaTypes == other.dwNumberOfMediaTypes
            && self.dwNumberOfLibRequests == other.dwNumberOfLibRequests
            && self.Reserved == other.Reserved
            && self.AutoRecovery == other.AutoRecovery
            && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NTMS_LIBRARYINFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_LIBRARYINFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_LIBREQUESTINFORMATIONA {
    pub OperationCode: NtmsLmOperation,
    pub OperationOption: u32,
    pub State: NtmsLmState,
    pub PartitionId: ::windows::core::GUID,
    pub DriveId: ::windows::core::GUID,
    pub PhysMediaId: ::windows::core::GUID,
    pub Library: ::windows::core::GUID,
    pub SlotId: ::windows::core::GUID,
    pub TimeQueued: super::super::Foundation::SYSTEMTIME,
    pub TimeCompleted: super::super::Foundation::SYSTEMTIME,
    pub szApplication: [u8; 64],
    pub szUser: [u8; 64],
    pub szComputer: [u8; 64],
    pub dwErrorCode: u32,
    pub WorkItemId: ::windows::core::GUID,
    pub dwPriority: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_LIBREQUESTINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_LIBREQUESTINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NTMS_LIBREQUESTINFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_LIBREQUESTINFORMATIONA")
            .field("OperationCode", &self.OperationCode)
            .field("OperationOption", &self.OperationOption)
            .field("State", &self.State)
            .field("PartitionId", &self.PartitionId)
            .field("DriveId", &self.DriveId)
            .field("PhysMediaId", &self.PhysMediaId)
            .field("Library", &self.Library)
            .field("SlotId", &self.SlotId)
            .field("TimeQueued", &self.TimeQueued)
            .field("TimeCompleted", &self.TimeCompleted)
            .field("szApplication", &self.szApplication)
            .field("szUser", &self.szUser)
            .field("szComputer", &self.szComputer)
            .field("dwErrorCode", &self.dwErrorCode)
            .field("WorkItemId", &self.WorkItemId)
            .field("dwPriority", &self.dwPriority)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for NTMS_LIBREQUESTINFORMATIONA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NTMS_LIBREQUESTINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.OperationCode == other.OperationCode && self.OperationOption == other.OperationOption && self.State == other.State && self.PartitionId == other.PartitionId && self.DriveId == other.DriveId && self.PhysMediaId == other.PhysMediaId && self.Library == other.Library && self.SlotId == other.SlotId && self.TimeQueued == other.TimeQueued && self.TimeCompleted == other.TimeCompleted && self.szApplication == other.szApplication && self.szUser == other.szUser && self.szComputer == other.szComputer && self.dwErrorCode == other.dwErrorCode && self.WorkItemId == other.WorkItemId && self.dwPriority == other.dwPriority
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NTMS_LIBREQUESTINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_LIBREQUESTINFORMATIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_LIBREQUESTINFORMATIONW {
    pub OperationCode: NtmsLmOperation,
    pub OperationOption: u32,
    pub State: NtmsLmState,
    pub PartitionId: ::windows::core::GUID,
    pub DriveId: ::windows::core::GUID,
    pub PhysMediaId: ::windows::core::GUID,
    pub Library: ::windows::core::GUID,
    pub SlotId: ::windows::core::GUID,
    pub TimeQueued: super::super::Foundation::SYSTEMTIME,
    pub TimeCompleted: super::super::Foundation::SYSTEMTIME,
    pub szApplication: [u16; 64],
    pub szUser: [u16; 64],
    pub szComputer: [u16; 64],
    pub dwErrorCode: u32,
    pub WorkItemId: ::windows::core::GUID,
    pub dwPriority: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_LIBREQUESTINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_LIBREQUESTINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NTMS_LIBREQUESTINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_LIBREQUESTINFORMATIONW")
            .field("OperationCode", &self.OperationCode)
            .field("OperationOption", &self.OperationOption)
            .field("State", &self.State)
            .field("PartitionId", &self.PartitionId)
            .field("DriveId", &self.DriveId)
            .field("PhysMediaId", &self.PhysMediaId)
            .field("Library", &self.Library)
            .field("SlotId", &self.SlotId)
            .field("TimeQueued", &self.TimeQueued)
            .field("TimeCompleted", &self.TimeCompleted)
            .field("szApplication", &self.szApplication)
            .field("szUser", &self.szUser)
            .field("szComputer", &self.szComputer)
            .field("dwErrorCode", &self.dwErrorCode)
            .field("WorkItemId", &self.WorkItemId)
            .field("dwPriority", &self.dwPriority)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for NTMS_LIBREQUESTINFORMATIONW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NTMS_LIBREQUESTINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.OperationCode == other.OperationCode && self.OperationOption == other.OperationOption && self.State == other.State && self.PartitionId == other.PartitionId && self.DriveId == other.DriveId && self.PhysMediaId == other.PhysMediaId && self.Library == other.Library && self.SlotId == other.SlotId && self.TimeQueued == other.TimeQueued && self.TimeCompleted == other.TimeCompleted && self.szApplication == other.szApplication && self.szUser == other.szUser && self.szComputer == other.szComputer && self.dwErrorCode == other.dwErrorCode && self.WorkItemId == other.WorkItemId && self.dwPriority == other.dwPriority
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NTMS_LIBREQUESTINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_LIBREQUESTINFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_LMIDINFORMATION {
    pub MediaPool: ::windows::core::GUID,
    pub dwNumberOfPartitions: u32,
}
impl ::core::marker::Copy for NTMS_LMIDINFORMATION {}
impl ::core::clone::Clone for NTMS_LMIDINFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_LMIDINFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_LMIDINFORMATION").field("MediaPool", &self.MediaPool).field("dwNumberOfPartitions", &self.dwNumberOfPartitions).finish()
    }
}
impl ::windows::core::TypeKind for NTMS_LMIDINFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NTMS_LMIDINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.MediaPool == other.MediaPool && self.dwNumberOfPartitions == other.dwNumberOfPartitions
    }
}
impl ::core::cmp::Eq for NTMS_LMIDINFORMATION {}
impl ::core::default::Default for NTMS_LMIDINFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_MEDIAPOOLINFORMATION {
    pub PoolType: u32,
    pub MediaType: ::windows::core::GUID,
    pub Parent: ::windows::core::GUID,
    pub AllocationPolicy: u32,
    pub DeallocationPolicy: u32,
    pub dwMaxAllocates: u32,
    pub dwNumberOfPhysicalMedia: u32,
    pub dwNumberOfLogicalMedia: u32,
    pub dwNumberOfMediaPools: u32,
}
impl ::core::marker::Copy for NTMS_MEDIAPOOLINFORMATION {}
impl ::core::clone::Clone for NTMS_MEDIAPOOLINFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_MEDIAPOOLINFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_MEDIAPOOLINFORMATION")
            .field("PoolType", &self.PoolType)
            .field("MediaType", &self.MediaType)
            .field("Parent", &self.Parent)
            .field("AllocationPolicy", &self.AllocationPolicy)
            .field("DeallocationPolicy", &self.DeallocationPolicy)
            .field("dwMaxAllocates", &self.dwMaxAllocates)
            .field("dwNumberOfPhysicalMedia", &self.dwNumberOfPhysicalMedia)
            .field("dwNumberOfLogicalMedia", &self.dwNumberOfLogicalMedia)
            .field("dwNumberOfMediaPools", &self.dwNumberOfMediaPools)
            .finish()
    }
}
impl ::windows::core::TypeKind for NTMS_MEDIAPOOLINFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NTMS_MEDIAPOOLINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.PoolType == other.PoolType && self.MediaType == other.MediaType && self.Parent == other.Parent && self.AllocationPolicy == other.AllocationPolicy && self.DeallocationPolicy == other.DeallocationPolicy && self.dwMaxAllocates == other.dwMaxAllocates && self.dwNumberOfPhysicalMedia == other.dwNumberOfPhysicalMedia && self.dwNumberOfLogicalMedia == other.dwNumberOfLogicalMedia && self.dwNumberOfMediaPools == other.dwNumberOfMediaPools
    }
}
impl ::core::cmp::Eq for NTMS_MEDIAPOOLINFORMATION {}
impl ::core::default::Default for NTMS_MEDIAPOOLINFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_MEDIATYPEINFORMATION {
    pub MediaType: u32,
    pub NumberOfSides: u32,
    pub ReadWriteCharacteristics: NtmsReadWriteCharacteristics,
    pub DeviceType: FILE_DEVICE_TYPE,
}
impl ::core::marker::Copy for NTMS_MEDIATYPEINFORMATION {}
impl ::core::clone::Clone for NTMS_MEDIATYPEINFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_MEDIATYPEINFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_MEDIATYPEINFORMATION").field("MediaType", &self.MediaType).field("NumberOfSides", &self.NumberOfSides).field("ReadWriteCharacteristics", &self.ReadWriteCharacteristics).field("DeviceType", &self.DeviceType).finish()
    }
}
impl ::windows::core::TypeKind for NTMS_MEDIATYPEINFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NTMS_MEDIATYPEINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.MediaType == other.MediaType && self.NumberOfSides == other.NumberOfSides && self.ReadWriteCharacteristics == other.ReadWriteCharacteristics && self.DeviceType == other.DeviceType
    }
}
impl ::core::cmp::Eq for NTMS_MEDIATYPEINFORMATION {}
impl ::core::default::Default for NTMS_MEDIATYPEINFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_MOUNT_INFORMATION {
    pub dwSize: u32,
    pub lpReserved: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NTMS_MOUNT_INFORMATION {}
impl ::core::clone::Clone for NTMS_MOUNT_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_MOUNT_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_MOUNT_INFORMATION").field("dwSize", &self.dwSize).field("lpReserved", &self.lpReserved).finish()
    }
}
impl ::windows::core::TypeKind for NTMS_MOUNT_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NTMS_MOUNT_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.lpReserved == other.lpReserved
    }
}
impl ::core::cmp::Eq for NTMS_MOUNT_INFORMATION {}
impl ::core::default::Default for NTMS_MOUNT_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_NOTIFICATIONINFORMATION {
    pub dwOperation: NtmsNotificationOperations,
    pub ObjectId: ::windows::core::GUID,
}
impl ::core::marker::Copy for NTMS_NOTIFICATIONINFORMATION {}
impl ::core::clone::Clone for NTMS_NOTIFICATIONINFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_NOTIFICATIONINFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_NOTIFICATIONINFORMATION").field("dwOperation", &self.dwOperation).field("ObjectId", &self.ObjectId).finish()
    }
}
impl ::windows::core::TypeKind for NTMS_NOTIFICATIONINFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NTMS_NOTIFICATIONINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwOperation == other.dwOperation && self.ObjectId == other.ObjectId
    }
}
impl ::core::cmp::Eq for NTMS_NOTIFICATIONINFORMATION {}
impl ::core::default::Default for NTMS_NOTIFICATIONINFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_OBJECTINFORMATIONA {
    pub dwSize: u32,
    pub dwType: NtmsObjectsTypes,
    pub Created: super::super::Foundation::SYSTEMTIME,
    pub Modified: super::super::Foundation::SYSTEMTIME,
    pub ObjectGuid: ::windows::core::GUID,
    pub Enabled: super::super::Foundation::BOOL,
    pub dwOperationalState: NtmsOperationalState,
    pub szName: [u8; 64],
    pub szDescription: [u8; 127],
    pub Info: NTMS_OBJECTINFORMATIONA_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_OBJECTINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_OBJECTINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for NTMS_OBJECTINFORMATIONA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_OBJECTINFORMATIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union NTMS_OBJECTINFORMATIONA_0 {
    pub Drive: NTMS_DRIVEINFORMATIONA,
    pub DriveType: NTMS_DRIVETYPEINFORMATIONA,
    pub Library: NTMS_LIBRARYINFORMATION,
    pub Changer: NTMS_CHANGERINFORMATIONA,
    pub ChangerType: NTMS_CHANGERTYPEINFORMATIONA,
    pub StorageSlot: NTMS_STORAGESLOTINFORMATION,
    pub IEDoor: NTMS_IEDOORINFORMATION,
    pub IEPort: NTMS_IEPORTINFORMATION,
    pub PhysicalMedia: NTMS_PMIDINFORMATIONA,
    pub LogicalMedia: NTMS_LMIDINFORMATION,
    pub Partition: NTMS_PARTITIONINFORMATIONA,
    pub MediaPool: NTMS_MEDIAPOOLINFORMATION,
    pub MediaType: NTMS_MEDIATYPEINFORMATION,
    pub LibRequest: NTMS_LIBREQUESTINFORMATIONA,
    pub OpRequest: NTMS_OPREQUESTINFORMATIONA,
    pub Computer: NTMS_COMPUTERINFORMATION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_OBJECTINFORMATIONA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_OBJECTINFORMATIONA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for NTMS_OBJECTINFORMATIONA_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_OBJECTINFORMATIONA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_OBJECTINFORMATIONW {
    pub dwSize: u32,
    pub dwType: NtmsObjectsTypes,
    pub Created: super::super::Foundation::SYSTEMTIME,
    pub Modified: super::super::Foundation::SYSTEMTIME,
    pub ObjectGuid: ::windows::core::GUID,
    pub Enabled: super::super::Foundation::BOOL,
    pub dwOperationalState: NtmsOperationalState,
    pub szName: [u16; 64],
    pub szDescription: [u16; 127],
    pub Info: NTMS_OBJECTINFORMATIONW_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_OBJECTINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_OBJECTINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for NTMS_OBJECTINFORMATIONW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_OBJECTINFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union NTMS_OBJECTINFORMATIONW_0 {
    pub Drive: NTMS_DRIVEINFORMATIONW,
    pub DriveType: NTMS_DRIVETYPEINFORMATIONW,
    pub Library: NTMS_LIBRARYINFORMATION,
    pub Changer: NTMS_CHANGERINFORMATIONW,
    pub ChangerType: NTMS_CHANGERTYPEINFORMATIONW,
    pub StorageSlot: NTMS_STORAGESLOTINFORMATION,
    pub IEDoor: NTMS_IEDOORINFORMATION,
    pub IEPort: NTMS_IEPORTINFORMATION,
    pub PhysicalMedia: NTMS_PMIDINFORMATIONW,
    pub LogicalMedia: NTMS_LMIDINFORMATION,
    pub Partition: NTMS_PARTITIONINFORMATIONW,
    pub MediaPool: NTMS_MEDIAPOOLINFORMATION,
    pub MediaType: NTMS_MEDIATYPEINFORMATION,
    pub LibRequest: NTMS_LIBREQUESTINFORMATIONW,
    pub OpRequest: NTMS_OPREQUESTINFORMATIONW,
    pub Computer: NTMS_COMPUTERINFORMATION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_OBJECTINFORMATIONW_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_OBJECTINFORMATIONW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for NTMS_OBJECTINFORMATIONW_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_OBJECTINFORMATIONW_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_OPREQUESTINFORMATIONA {
    pub Request: NtmsOpreqCommand,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
    pub State: NtmsOpreqState,
    pub szMessage: [u8; 256],
    pub Arg1Type: NtmsObjectsTypes,
    pub Arg1: ::windows::core::GUID,
    pub Arg2Type: NtmsObjectsTypes,
    pub Arg2: ::windows::core::GUID,
    pub szApplication: [u8; 64],
    pub szUser: [u8; 64],
    pub szComputer: [u8; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_OPREQUESTINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_OPREQUESTINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NTMS_OPREQUESTINFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_OPREQUESTINFORMATIONA").field("Request", &self.Request).field("Submitted", &self.Submitted).field("State", &self.State).field("szMessage", &self.szMessage).field("Arg1Type", &self.Arg1Type).field("Arg1", &self.Arg1).field("Arg2Type", &self.Arg2Type).field("Arg2", &self.Arg2).field("szApplication", &self.szApplication).field("szUser", &self.szUser).field("szComputer", &self.szComputer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for NTMS_OPREQUESTINFORMATIONA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NTMS_OPREQUESTINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.Request == other.Request && self.Submitted == other.Submitted && self.State == other.State && self.szMessage == other.szMessage && self.Arg1Type == other.Arg1Type && self.Arg1 == other.Arg1 && self.Arg2Type == other.Arg2Type && self.Arg2 == other.Arg2 && self.szApplication == other.szApplication && self.szUser == other.szUser && self.szComputer == other.szComputer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NTMS_OPREQUESTINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_OPREQUESTINFORMATIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_OPREQUESTINFORMATIONW {
    pub Request: NtmsOpreqCommand,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
    pub State: NtmsOpreqState,
    pub szMessage: [u16; 256],
    pub Arg1Type: NtmsObjectsTypes,
    pub Arg1: ::windows::core::GUID,
    pub Arg2Type: NtmsObjectsTypes,
    pub Arg2: ::windows::core::GUID,
    pub szApplication: [u16; 64],
    pub szUser: [u16; 64],
    pub szComputer: [u16; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_OPREQUESTINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_OPREQUESTINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NTMS_OPREQUESTINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_OPREQUESTINFORMATIONW").field("Request", &self.Request).field("Submitted", &self.Submitted).field("State", &self.State).field("szMessage", &self.szMessage).field("Arg1Type", &self.Arg1Type).field("Arg1", &self.Arg1).field("Arg2Type", &self.Arg2Type).field("Arg2", &self.Arg2).field("szApplication", &self.szApplication).field("szUser", &self.szUser).field("szComputer", &self.szComputer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for NTMS_OPREQUESTINFORMATIONW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NTMS_OPREQUESTINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.Request == other.Request && self.Submitted == other.Submitted && self.State == other.State && self.szMessage == other.szMessage && self.Arg1Type == other.Arg1Type && self.Arg1 == other.Arg1 && self.Arg2Type == other.Arg2Type && self.Arg2 == other.Arg2 && self.szApplication == other.szApplication && self.szUser == other.szUser && self.szComputer == other.szComputer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NTMS_OPREQUESTINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_OPREQUESTINFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_PARTITIONINFORMATIONA {
    pub PhysicalMedia: ::windows::core::GUID,
    pub LogicalMedia: ::windows::core::GUID,
    pub State: NtmsPartitionState,
    pub Side: u16,
    pub dwOmidLabelIdLength: u32,
    pub OmidLabelId: [u8; 255],
    pub szOmidLabelType: [u8; 64],
    pub szOmidLabelInfo: [u8; 256],
    pub dwMountCount: u32,
    pub dwAllocateCount: u32,
    pub Capacity: i64,
}
impl ::core::marker::Copy for NTMS_PARTITIONINFORMATIONA {}
impl ::core::clone::Clone for NTMS_PARTITIONINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_PARTITIONINFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_PARTITIONINFORMATIONA")
            .field("PhysicalMedia", &self.PhysicalMedia)
            .field("LogicalMedia", &self.LogicalMedia)
            .field("State", &self.State)
            .field("Side", &self.Side)
            .field("dwOmidLabelIdLength", &self.dwOmidLabelIdLength)
            .field("OmidLabelId", &self.OmidLabelId)
            .field("szOmidLabelType", &self.szOmidLabelType)
            .field("szOmidLabelInfo", &self.szOmidLabelInfo)
            .field("dwMountCount", &self.dwMountCount)
            .field("dwAllocateCount", &self.dwAllocateCount)
            .field("Capacity", &self.Capacity)
            .finish()
    }
}
impl ::windows::core::TypeKind for NTMS_PARTITIONINFORMATIONA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NTMS_PARTITIONINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.PhysicalMedia == other.PhysicalMedia && self.LogicalMedia == other.LogicalMedia && self.State == other.State && self.Side == other.Side && self.dwOmidLabelIdLength == other.dwOmidLabelIdLength && self.OmidLabelId == other.OmidLabelId && self.szOmidLabelType == other.szOmidLabelType && self.szOmidLabelInfo == other.szOmidLabelInfo && self.dwMountCount == other.dwMountCount && self.dwAllocateCount == other.dwAllocateCount && self.Capacity == other.Capacity
    }
}
impl ::core::cmp::Eq for NTMS_PARTITIONINFORMATIONA {}
impl ::core::default::Default for NTMS_PARTITIONINFORMATIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_PARTITIONINFORMATIONW {
    pub PhysicalMedia: ::windows::core::GUID,
    pub LogicalMedia: ::windows::core::GUID,
    pub State: NtmsPartitionState,
    pub Side: u16,
    pub dwOmidLabelIdLength: u32,
    pub OmidLabelId: [u8; 255],
    pub szOmidLabelType: [u16; 64],
    pub szOmidLabelInfo: [u16; 256],
    pub dwMountCount: u32,
    pub dwAllocateCount: u32,
    pub Capacity: i64,
}
impl ::core::marker::Copy for NTMS_PARTITIONINFORMATIONW {}
impl ::core::clone::Clone for NTMS_PARTITIONINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_PARTITIONINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_PARTITIONINFORMATIONW")
            .field("PhysicalMedia", &self.PhysicalMedia)
            .field("LogicalMedia", &self.LogicalMedia)
            .field("State", &self.State)
            .field("Side", &self.Side)
            .field("dwOmidLabelIdLength", &self.dwOmidLabelIdLength)
            .field("OmidLabelId", &self.OmidLabelId)
            .field("szOmidLabelType", &self.szOmidLabelType)
            .field("szOmidLabelInfo", &self.szOmidLabelInfo)
            .field("dwMountCount", &self.dwMountCount)
            .field("dwAllocateCount", &self.dwAllocateCount)
            .field("Capacity", &self.Capacity)
            .finish()
    }
}
impl ::windows::core::TypeKind for NTMS_PARTITIONINFORMATIONW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NTMS_PARTITIONINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.PhysicalMedia == other.PhysicalMedia && self.LogicalMedia == other.LogicalMedia && self.State == other.State && self.Side == other.Side && self.dwOmidLabelIdLength == other.dwOmidLabelIdLength && self.OmidLabelId == other.OmidLabelId && self.szOmidLabelType == other.szOmidLabelType && self.szOmidLabelInfo == other.szOmidLabelInfo && self.dwMountCount == other.dwMountCount && self.dwAllocateCount == other.dwAllocateCount && self.Capacity == other.Capacity
    }
}
impl ::core::cmp::Eq for NTMS_PARTITIONINFORMATIONW {}
impl ::core::default::Default for NTMS_PARTITIONINFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_PMIDINFORMATIONA {
    pub CurrentLibrary: ::windows::core::GUID,
    pub MediaPool: ::windows::core::GUID,
    pub Location: ::windows::core::GUID,
    pub LocationType: u32,
    pub MediaType: ::windows::core::GUID,
    pub HomeSlot: ::windows::core::GUID,
    pub szBarCode: [u8; 64],
    pub BarCodeState: NtmsBarCodeState,
    pub szSequenceNumber: [u8; 32],
    pub MediaState: NtmsMediaState,
    pub dwNumberOfPartitions: u32,
    pub dwMediaTypeCode: u32,
    pub dwDensityCode: u32,
    pub MountedPartition: ::windows::core::GUID,
}
impl ::core::marker::Copy for NTMS_PMIDINFORMATIONA {}
impl ::core::clone::Clone for NTMS_PMIDINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_PMIDINFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_PMIDINFORMATIONA")
            .field("CurrentLibrary", &self.CurrentLibrary)
            .field("MediaPool", &self.MediaPool)
            .field("Location", &self.Location)
            .field("LocationType", &self.LocationType)
            .field("MediaType", &self.MediaType)
            .field("HomeSlot", &self.HomeSlot)
            .field("szBarCode", &self.szBarCode)
            .field("BarCodeState", &self.BarCodeState)
            .field("szSequenceNumber", &self.szSequenceNumber)
            .field("MediaState", &self.MediaState)
            .field("dwNumberOfPartitions", &self.dwNumberOfPartitions)
            .field("dwMediaTypeCode", &self.dwMediaTypeCode)
            .field("dwDensityCode", &self.dwDensityCode)
            .field("MountedPartition", &self.MountedPartition)
            .finish()
    }
}
impl ::windows::core::TypeKind for NTMS_PMIDINFORMATIONA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NTMS_PMIDINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.CurrentLibrary == other.CurrentLibrary && self.MediaPool == other.MediaPool && self.Location == other.Location && self.LocationType == other.LocationType && self.MediaType == other.MediaType && self.HomeSlot == other.HomeSlot && self.szBarCode == other.szBarCode && self.BarCodeState == other.BarCodeState && self.szSequenceNumber == other.szSequenceNumber && self.MediaState == other.MediaState && self.dwNumberOfPartitions == other.dwNumberOfPartitions && self.dwMediaTypeCode == other.dwMediaTypeCode && self.dwDensityCode == other.dwDensityCode && self.MountedPartition == other.MountedPartition
    }
}
impl ::core::cmp::Eq for NTMS_PMIDINFORMATIONA {}
impl ::core::default::Default for NTMS_PMIDINFORMATIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_PMIDINFORMATIONW {
    pub CurrentLibrary: ::windows::core::GUID,
    pub MediaPool: ::windows::core::GUID,
    pub Location: ::windows::core::GUID,
    pub LocationType: u32,
    pub MediaType: ::windows::core::GUID,
    pub HomeSlot: ::windows::core::GUID,
    pub szBarCode: [u16; 64],
    pub BarCodeState: NtmsBarCodeState,
    pub szSequenceNumber: [u16; 32],
    pub MediaState: NtmsMediaState,
    pub dwNumberOfPartitions: u32,
    pub dwMediaTypeCode: u32,
    pub dwDensityCode: u32,
    pub MountedPartition: ::windows::core::GUID,
}
impl ::core::marker::Copy for NTMS_PMIDINFORMATIONW {}
impl ::core::clone::Clone for NTMS_PMIDINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_PMIDINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_PMIDINFORMATIONW")
            .field("CurrentLibrary", &self.CurrentLibrary)
            .field("MediaPool", &self.MediaPool)
            .field("Location", &self.Location)
            .field("LocationType", &self.LocationType)
            .field("MediaType", &self.MediaType)
            .field("HomeSlot", &self.HomeSlot)
            .field("szBarCode", &self.szBarCode)
            .field("BarCodeState", &self.BarCodeState)
            .field("szSequenceNumber", &self.szSequenceNumber)
            .field("MediaState", &self.MediaState)
            .field("dwNumberOfPartitions", &self.dwNumberOfPartitions)
            .field("dwMediaTypeCode", &self.dwMediaTypeCode)
            .field("dwDensityCode", &self.dwDensityCode)
            .field("MountedPartition", &self.MountedPartition)
            .finish()
    }
}
impl ::windows::core::TypeKind for NTMS_PMIDINFORMATIONW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NTMS_PMIDINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.CurrentLibrary == other.CurrentLibrary && self.MediaPool == other.MediaPool && self.Location == other.Location && self.LocationType == other.LocationType && self.MediaType == other.MediaType && self.HomeSlot == other.HomeSlot && self.szBarCode == other.szBarCode && self.BarCodeState == other.BarCodeState && self.szSequenceNumber == other.szSequenceNumber && self.MediaState == other.MediaState && self.dwNumberOfPartitions == other.dwNumberOfPartitions && self.dwMediaTypeCode == other.dwMediaTypeCode && self.dwDensityCode == other.dwDensityCode && self.MountedPartition == other.MountedPartition
    }
}
impl ::core::cmp::Eq for NTMS_PMIDINFORMATIONW {}
impl ::core::default::Default for NTMS_PMIDINFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_STORAGESLOTINFORMATION {
    pub Number: u32,
    pub State: u32,
    pub Library: ::windows::core::GUID,
}
impl ::core::marker::Copy for NTMS_STORAGESLOTINFORMATION {}
impl ::core::clone::Clone for NTMS_STORAGESLOTINFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTMS_STORAGESLOTINFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_STORAGESLOTINFORMATION").field("Number", &self.Number).field("State", &self.State).field("Library", &self.Library).finish()
    }
}
impl ::windows::core::TypeKind for NTMS_STORAGESLOTINFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NTMS_STORAGESLOTINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Number == other.Number && self.State == other.State && self.Library == other.Library
    }
}
impl ::core::cmp::Eq for NTMS_STORAGESLOTINFORMATION {}
impl ::core::default::Default for NTMS_STORAGESLOTINFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct OFSTRUCT {
    pub cBytes: u8,
    pub fFixedDisk: u8,
    pub nErrCode: u16,
    pub Reserved1: u16,
    pub Reserved2: u16,
    pub szPathName: [u8; 128],
}
impl ::core::marker::Copy for OFSTRUCT {}
impl ::core::clone::Clone for OFSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OFSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OFSTRUCT").field("cBytes", &self.cBytes).field("fFixedDisk", &self.fFixedDisk).field("nErrCode", &self.nErrCode).field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).field("szPathName", &self.szPathName).finish()
    }
}
impl ::windows::core::TypeKind for OFSTRUCT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for OFSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.cBytes == other.cBytes && self.fFixedDisk == other.fFixedDisk && self.nErrCode == other.nErrCode && self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2 && self.szPathName == other.szPathName
    }
}
impl ::core::cmp::Eq for OFSTRUCT {}
impl ::core::default::Default for OFSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct REPARSE_GUID_DATA_BUFFER {
    pub ReparseTag: u32,
    pub ReparseDataLength: u16,
    pub Reserved: u16,
    pub ReparseGuid: ::windows::core::GUID,
    pub GenericReparseBuffer: REPARSE_GUID_DATA_BUFFER_0,
}
impl ::core::marker::Copy for REPARSE_GUID_DATA_BUFFER {}
impl ::core::clone::Clone for REPARSE_GUID_DATA_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REPARSE_GUID_DATA_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REPARSE_GUID_DATA_BUFFER").field("ReparseTag", &self.ReparseTag).field("ReparseDataLength", &self.ReparseDataLength).field("Reserved", &self.Reserved).field("ReparseGuid", &self.ReparseGuid).field("GenericReparseBuffer", &self.GenericReparseBuffer).finish()
    }
}
impl ::windows::core::TypeKind for REPARSE_GUID_DATA_BUFFER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for REPARSE_GUID_DATA_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.ReparseTag == other.ReparseTag && self.ReparseDataLength == other.ReparseDataLength && self.Reserved == other.Reserved && self.ReparseGuid == other.ReparseGuid && self.GenericReparseBuffer == other.GenericReparseBuffer
    }
}
impl ::core::cmp::Eq for REPARSE_GUID_DATA_BUFFER {}
impl ::core::default::Default for REPARSE_GUID_DATA_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct REPARSE_GUID_DATA_BUFFER_0 {
    pub DataBuffer: [u8; 1],
}
impl ::core::marker::Copy for REPARSE_GUID_DATA_BUFFER_0 {}
impl ::core::clone::Clone for REPARSE_GUID_DATA_BUFFER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REPARSE_GUID_DATA_BUFFER_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REPARSE_GUID_DATA_BUFFER_0").field("DataBuffer", &self.DataBuffer).finish()
    }
}
impl ::windows::core::TypeKind for REPARSE_GUID_DATA_BUFFER_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for REPARSE_GUID_DATA_BUFFER_0 {
    fn eq(&self, other: &Self) -> bool {
        self.DataBuffer == other.DataBuffer
    }
}
impl ::core::cmp::Eq for REPARSE_GUID_DATA_BUFFER_0 {}
impl ::core::default::Default for REPARSE_GUID_DATA_BUFFER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_ALIAS_INFO_0 {
    pub srvai0_alias: ::windows::core::PWSTR,
    pub srvai0_target: ::windows::core::PWSTR,
    pub srvai0_default: super::super::Foundation::BOOLEAN,
    pub srvai0_reserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_ALIAS_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_ALIAS_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_ALIAS_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_ALIAS_INFO_0").field("srvai0_alias", &self.srvai0_alias).field("srvai0_target", &self.srvai0_target).field("srvai0_default", &self.srvai0_default).field("srvai0_reserved", &self.srvai0_reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SERVER_ALIAS_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_ALIAS_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.srvai0_alias == other.srvai0_alias && self.srvai0_target == other.srvai0_target && self.srvai0_default == other.srvai0_default && self.srvai0_reserved == other.srvai0_reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_ALIAS_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_ALIAS_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct SERVER_CERTIFICATE_INFO_0 {
    pub srvci0_name: ::windows::core::PWSTR,
    pub srvci0_subject: ::windows::core::PWSTR,
    pub srvci0_issuer: ::windows::core::PWSTR,
    pub srvci0_thumbprint: ::windows::core::PWSTR,
    pub srvci0_friendlyname: ::windows::core::PWSTR,
    pub srvci0_notbefore: ::windows::core::PWSTR,
    pub srvci0_notafter: ::windows::core::PWSTR,
    pub srvci0_storelocation: ::windows::core::PWSTR,
    pub srvci0_storename: ::windows::core::PWSTR,
    pub srvci0_renewalchain: ::windows::core::PWSTR,
    pub srvci0_type: u32,
    pub srvci0_flags: u32,
    pub srvci0_mapping_status: u32,
}
impl ::core::marker::Copy for SERVER_CERTIFICATE_INFO_0 {}
impl ::core::clone::Clone for SERVER_CERTIFICATE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVER_CERTIFICATE_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_CERTIFICATE_INFO_0")
            .field("srvci0_name", &self.srvci0_name)
            .field("srvci0_subject", &self.srvci0_subject)
            .field("srvci0_issuer", &self.srvci0_issuer)
            .field("srvci0_thumbprint", &self.srvci0_thumbprint)
            .field("srvci0_friendlyname", &self.srvci0_friendlyname)
            .field("srvci0_notbefore", &self.srvci0_notbefore)
            .field("srvci0_notafter", &self.srvci0_notafter)
            .field("srvci0_storelocation", &self.srvci0_storelocation)
            .field("srvci0_storename", &self.srvci0_storename)
            .field("srvci0_renewalchain", &self.srvci0_renewalchain)
            .field("srvci0_type", &self.srvci0_type)
            .field("srvci0_flags", &self.srvci0_flags)
            .field("srvci0_mapping_status", &self.srvci0_mapping_status)
            .finish()
    }
}
impl ::windows::core::TypeKind for SERVER_CERTIFICATE_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SERVER_CERTIFICATE_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.srvci0_name == other.srvci0_name && self.srvci0_subject == other.srvci0_subject && self.srvci0_issuer == other.srvci0_issuer && self.srvci0_thumbprint == other.srvci0_thumbprint && self.srvci0_friendlyname == other.srvci0_friendlyname && self.srvci0_notbefore == other.srvci0_notbefore && self.srvci0_notafter == other.srvci0_notafter && self.srvci0_storelocation == other.srvci0_storelocation && self.srvci0_storename == other.srvci0_storename && self.srvci0_renewalchain == other.srvci0_renewalchain && self.srvci0_type == other.srvci0_type && self.srvci0_flags == other.srvci0_flags && self.srvci0_mapping_status == other.srvci0_mapping_status
    }
}
impl ::core::cmp::Eq for SERVER_CERTIFICATE_INFO_0 {}
impl ::core::default::Default for SERVER_CERTIFICATE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct SESSION_INFO_0 {
    pub sesi0_cname: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for SESSION_INFO_0 {}
impl ::core::clone::Clone for SESSION_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SESSION_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SESSION_INFO_0").field("sesi0_cname", &self.sesi0_cname).finish()
    }
}
impl ::windows::core::TypeKind for SESSION_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SESSION_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.sesi0_cname == other.sesi0_cname
    }
}
impl ::core::cmp::Eq for SESSION_INFO_0 {}
impl ::core::default::Default for SESSION_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct SESSION_INFO_1 {
    pub sesi1_cname: ::windows::core::PWSTR,
    pub sesi1_username: ::windows::core::PWSTR,
    pub sesi1_num_opens: u32,
    pub sesi1_time: u32,
    pub sesi1_idle_time: u32,
    pub sesi1_user_flags: SESSION_INFO_USER_FLAGS,
}
impl ::core::marker::Copy for SESSION_INFO_1 {}
impl ::core::clone::Clone for SESSION_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SESSION_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SESSION_INFO_1").field("sesi1_cname", &self.sesi1_cname).field("sesi1_username", &self.sesi1_username).field("sesi1_num_opens", &self.sesi1_num_opens).field("sesi1_time", &self.sesi1_time).field("sesi1_idle_time", &self.sesi1_idle_time).field("sesi1_user_flags", &self.sesi1_user_flags).finish()
    }
}
impl ::windows::core::TypeKind for SESSION_INFO_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SESSION_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.sesi1_cname == other.sesi1_cname && self.sesi1_username == other.sesi1_username && self.sesi1_num_opens == other.sesi1_num_opens && self.sesi1_time == other.sesi1_time && self.sesi1_idle_time == other.sesi1_idle_time && self.sesi1_user_flags == other.sesi1_user_flags
    }
}
impl ::core::cmp::Eq for SESSION_INFO_1 {}
impl ::core::default::Default for SESSION_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct SESSION_INFO_10 {
    pub sesi10_cname: ::windows::core::PWSTR,
    pub sesi10_username: ::windows::core::PWSTR,
    pub sesi10_time: u32,
    pub sesi10_idle_time: u32,
}
impl ::core::marker::Copy for SESSION_INFO_10 {}
impl ::core::clone::Clone for SESSION_INFO_10 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SESSION_INFO_10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SESSION_INFO_10").field("sesi10_cname", &self.sesi10_cname).field("sesi10_username", &self.sesi10_username).field("sesi10_time", &self.sesi10_time).field("sesi10_idle_time", &self.sesi10_idle_time).finish()
    }
}
impl ::windows::core::TypeKind for SESSION_INFO_10 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SESSION_INFO_10 {
    fn eq(&self, other: &Self) -> bool {
        self.sesi10_cname == other.sesi10_cname && self.sesi10_username == other.sesi10_username && self.sesi10_time == other.sesi10_time && self.sesi10_idle_time == other.sesi10_idle_time
    }
}
impl ::core::cmp::Eq for SESSION_INFO_10 {}
impl ::core::default::Default for SESSION_INFO_10 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct SESSION_INFO_2 {
    pub sesi2_cname: ::windows::core::PWSTR,
    pub sesi2_username: ::windows::core::PWSTR,
    pub sesi2_num_opens: u32,
    pub sesi2_time: u32,
    pub sesi2_idle_time: u32,
    pub sesi2_user_flags: SESSION_INFO_USER_FLAGS,
    pub sesi2_cltype_name: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for SESSION_INFO_2 {}
impl ::core::clone::Clone for SESSION_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SESSION_INFO_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SESSION_INFO_2").field("sesi2_cname", &self.sesi2_cname).field("sesi2_username", &self.sesi2_username).field("sesi2_num_opens", &self.sesi2_num_opens).field("sesi2_time", &self.sesi2_time).field("sesi2_idle_time", &self.sesi2_idle_time).field("sesi2_user_flags", &self.sesi2_user_flags).field("sesi2_cltype_name", &self.sesi2_cltype_name).finish()
    }
}
impl ::windows::core::TypeKind for SESSION_INFO_2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SESSION_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.sesi2_cname == other.sesi2_cname && self.sesi2_username == other.sesi2_username && self.sesi2_num_opens == other.sesi2_num_opens && self.sesi2_time == other.sesi2_time && self.sesi2_idle_time == other.sesi2_idle_time && self.sesi2_user_flags == other.sesi2_user_flags && self.sesi2_cltype_name == other.sesi2_cltype_name
    }
}
impl ::core::cmp::Eq for SESSION_INFO_2 {}
impl ::core::default::Default for SESSION_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct SESSION_INFO_502 {
    pub sesi502_cname: ::windows::core::PWSTR,
    pub sesi502_username: ::windows::core::PWSTR,
    pub sesi502_num_opens: u32,
    pub sesi502_time: u32,
    pub sesi502_idle_time: u32,
    pub sesi502_user_flags: SESSION_INFO_USER_FLAGS,
    pub sesi502_cltype_name: ::windows::core::PWSTR,
    pub sesi502_transport: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for SESSION_INFO_502 {}
impl ::core::clone::Clone for SESSION_INFO_502 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SESSION_INFO_502 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SESSION_INFO_502").field("sesi502_cname", &self.sesi502_cname).field("sesi502_username", &self.sesi502_username).field("sesi502_num_opens", &self.sesi502_num_opens).field("sesi502_time", &self.sesi502_time).field("sesi502_idle_time", &self.sesi502_idle_time).field("sesi502_user_flags", &self.sesi502_user_flags).field("sesi502_cltype_name", &self.sesi502_cltype_name).field("sesi502_transport", &self.sesi502_transport).finish()
    }
}
impl ::windows::core::TypeKind for SESSION_INFO_502 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SESSION_INFO_502 {
    fn eq(&self, other: &Self) -> bool {
        self.sesi502_cname == other.sesi502_cname && self.sesi502_username == other.sesi502_username && self.sesi502_num_opens == other.sesi502_num_opens && self.sesi502_time == other.sesi502_time && self.sesi502_idle_time == other.sesi502_idle_time && self.sesi502_user_flags == other.sesi502_user_flags && self.sesi502_cltype_name == other.sesi502_cltype_name && self.sesi502_transport == other.sesi502_transport
    }
}
impl ::core::cmp::Eq for SESSION_INFO_502 {}
impl ::core::default::Default for SESSION_INFO_502 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct SHARE_INFO_0 {
    pub shi0_netname: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for SHARE_INFO_0 {}
impl ::core::clone::Clone for SHARE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SHARE_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHARE_INFO_0").field("shi0_netname", &self.shi0_netname).finish()
    }
}
impl ::windows::core::TypeKind for SHARE_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SHARE_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.shi0_netname == other.shi0_netname
    }
}
impl ::core::cmp::Eq for SHARE_INFO_0 {}
impl ::core::default::Default for SHARE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct SHARE_INFO_1 {
    pub shi1_netname: ::windows::core::PWSTR,
    pub shi1_type: SHARE_TYPE,
    pub shi1_remark: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for SHARE_INFO_1 {}
impl ::core::clone::Clone for SHARE_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SHARE_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHARE_INFO_1").field("shi1_netname", &self.shi1_netname).field("shi1_type", &self.shi1_type).field("shi1_remark", &self.shi1_remark).finish()
    }
}
impl ::windows::core::TypeKind for SHARE_INFO_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SHARE_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.shi1_netname == other.shi1_netname && self.shi1_type == other.shi1_type && self.shi1_remark == other.shi1_remark
    }
}
impl ::core::cmp::Eq for SHARE_INFO_1 {}
impl ::core::default::Default for SHARE_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct SHARE_INFO_1004 {
    pub shi1004_remark: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for SHARE_INFO_1004 {}
impl ::core::clone::Clone for SHARE_INFO_1004 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SHARE_INFO_1004 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHARE_INFO_1004").field("shi1004_remark", &self.shi1004_remark).finish()
    }
}
impl ::windows::core::TypeKind for SHARE_INFO_1004 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SHARE_INFO_1004 {
    fn eq(&self, other: &Self) -> bool {
        self.shi1004_remark == other.shi1004_remark
    }
}
impl ::core::cmp::Eq for SHARE_INFO_1004 {}
impl ::core::default::Default for SHARE_INFO_1004 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct SHARE_INFO_1005 {
    pub shi1005_flags: u32,
}
impl ::core::marker::Copy for SHARE_INFO_1005 {}
impl ::core::clone::Clone for SHARE_INFO_1005 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SHARE_INFO_1005 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHARE_INFO_1005").field("shi1005_flags", &self.shi1005_flags).finish()
    }
}
impl ::windows::core::TypeKind for SHARE_INFO_1005 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SHARE_INFO_1005 {
    fn eq(&self, other: &Self) -> bool {
        self.shi1005_flags == other.shi1005_flags
    }
}
impl ::core::cmp::Eq for SHARE_INFO_1005 {}
impl ::core::default::Default for SHARE_INFO_1005 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct SHARE_INFO_1006 {
    pub shi1006_max_uses: u32,
}
impl ::core::marker::Copy for SHARE_INFO_1006 {}
impl ::core::clone::Clone for SHARE_INFO_1006 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SHARE_INFO_1006 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHARE_INFO_1006").field("shi1006_max_uses", &self.shi1006_max_uses).finish()
    }
}
impl ::windows::core::TypeKind for SHARE_INFO_1006 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SHARE_INFO_1006 {
    fn eq(&self, other: &Self) -> bool {
        self.shi1006_max_uses == other.shi1006_max_uses
    }
}
impl ::core::cmp::Eq for SHARE_INFO_1006 {}
impl ::core::default::Default for SHARE_INFO_1006 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct SHARE_INFO_1501 {
    pub shi1501_reserved: u32,
    pub shi1501_security_descriptor: super::super::Security::PSECURITY_DESCRIPTOR,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for SHARE_INFO_1501 {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for SHARE_INFO_1501 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::fmt::Debug for SHARE_INFO_1501 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHARE_INFO_1501").field("shi1501_reserved", &self.shi1501_reserved).field("shi1501_security_descriptor", &self.shi1501_security_descriptor).finish()
    }
}
#[cfg(feature = "Win32_Security")]
impl ::windows::core::TypeKind for SHARE_INFO_1501 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::PartialEq for SHARE_INFO_1501 {
    fn eq(&self, other: &Self) -> bool {
        self.shi1501_reserved == other.shi1501_reserved && self.shi1501_security_descriptor == other.shi1501_security_descriptor
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::Eq for SHARE_INFO_1501 {}
#[cfg(feature = "Win32_Security")]
impl ::core::default::Default for SHARE_INFO_1501 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct SHARE_INFO_1503 {
    pub shi1503_sharefilter: ::windows::core::GUID,
}
impl ::core::marker::Copy for SHARE_INFO_1503 {}
impl ::core::clone::Clone for SHARE_INFO_1503 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SHARE_INFO_1503 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHARE_INFO_1503").field("shi1503_sharefilter", &self.shi1503_sharefilter).finish()
    }
}
impl ::windows::core::TypeKind for SHARE_INFO_1503 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SHARE_INFO_1503 {
    fn eq(&self, other: &Self) -> bool {
        self.shi1503_sharefilter == other.shi1503_sharefilter
    }
}
impl ::core::cmp::Eq for SHARE_INFO_1503 {}
impl ::core::default::Default for SHARE_INFO_1503 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct SHARE_INFO_2 {
    pub shi2_netname: ::windows::core::PWSTR,
    pub shi2_type: SHARE_TYPE,
    pub shi2_remark: ::windows::core::PWSTR,
    pub shi2_permissions: SHARE_INFO_PERMISSIONS,
    pub shi2_max_uses: u32,
    pub shi2_current_uses: u32,
    pub shi2_path: ::windows::core::PWSTR,
    pub shi2_passwd: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for SHARE_INFO_2 {}
impl ::core::clone::Clone for SHARE_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SHARE_INFO_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHARE_INFO_2").field("shi2_netname", &self.shi2_netname).field("shi2_type", &self.shi2_type).field("shi2_remark", &self.shi2_remark).field("shi2_permissions", &self.shi2_permissions).field("shi2_max_uses", &self.shi2_max_uses).field("shi2_current_uses", &self.shi2_current_uses).field("shi2_path", &self.shi2_path).field("shi2_passwd", &self.shi2_passwd).finish()
    }
}
impl ::windows::core::TypeKind for SHARE_INFO_2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SHARE_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.shi2_netname == other.shi2_netname && self.shi2_type == other.shi2_type && self.shi2_remark == other.shi2_remark && self.shi2_permissions == other.shi2_permissions && self.shi2_max_uses == other.shi2_max_uses && self.shi2_current_uses == other.shi2_current_uses && self.shi2_path == other.shi2_path && self.shi2_passwd == other.shi2_passwd
    }
}
impl ::core::cmp::Eq for SHARE_INFO_2 {}
impl ::core::default::Default for SHARE_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct SHARE_INFO_501 {
    pub shi501_netname: ::windows::core::PWSTR,
    pub shi501_type: SHARE_TYPE,
    pub shi501_remark: ::windows::core::PWSTR,
    pub shi501_flags: u32,
}
impl ::core::marker::Copy for SHARE_INFO_501 {}
impl ::core::clone::Clone for SHARE_INFO_501 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SHARE_INFO_501 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHARE_INFO_501").field("shi501_netname", &self.shi501_netname).field("shi501_type", &self.shi501_type).field("shi501_remark", &self.shi501_remark).field("shi501_flags", &self.shi501_flags).finish()
    }
}
impl ::windows::core::TypeKind for SHARE_INFO_501 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SHARE_INFO_501 {
    fn eq(&self, other: &Self) -> bool {
        self.shi501_netname == other.shi501_netname && self.shi501_type == other.shi501_type && self.shi501_remark == other.shi501_remark && self.shi501_flags == other.shi501_flags
    }
}
impl ::core::cmp::Eq for SHARE_INFO_501 {}
impl ::core::default::Default for SHARE_INFO_501 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct SHARE_INFO_502 {
    pub shi502_netname: ::windows::core::PWSTR,
    pub shi502_type: SHARE_TYPE,
    pub shi502_remark: ::windows::core::PWSTR,
    pub shi502_permissions: SHARE_INFO_PERMISSIONS,
    pub shi502_max_uses: u32,
    pub shi502_current_uses: u32,
    pub shi502_path: ::windows::core::PWSTR,
    pub shi502_passwd: ::windows::core::PWSTR,
    pub shi502_reserved: u32,
    pub shi502_security_descriptor: super::super::Security::PSECURITY_DESCRIPTOR,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for SHARE_INFO_502 {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for SHARE_INFO_502 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::fmt::Debug for SHARE_INFO_502 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHARE_INFO_502")
            .field("shi502_netname", &self.shi502_netname)
            .field("shi502_type", &self.shi502_type)
            .field("shi502_remark", &self.shi502_remark)
            .field("shi502_permissions", &self.shi502_permissions)
            .field("shi502_max_uses", &self.shi502_max_uses)
            .field("shi502_current_uses", &self.shi502_current_uses)
            .field("shi502_path", &self.shi502_path)
            .field("shi502_passwd", &self.shi502_passwd)
            .field("shi502_reserved", &self.shi502_reserved)
            .field("shi502_security_descriptor", &self.shi502_security_descriptor)
            .finish()
    }
}
#[cfg(feature = "Win32_Security")]
impl ::windows::core::TypeKind for SHARE_INFO_502 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::PartialEq for SHARE_INFO_502 {
    fn eq(&self, other: &Self) -> bool {
        self.shi502_netname == other.shi502_netname && self.shi502_type == other.shi502_type && self.shi502_remark == other.shi502_remark && self.shi502_permissions == other.shi502_permissions && self.shi502_max_uses == other.shi502_max_uses && self.shi502_current_uses == other.shi502_current_uses && self.shi502_path == other.shi502_path && self.shi502_passwd == other.shi502_passwd && self.shi502_reserved == other.shi502_reserved && self.shi502_security_descriptor == other.shi502_security_descriptor
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::Eq for SHARE_INFO_502 {}
#[cfg(feature = "Win32_Security")]
impl ::core::default::Default for SHARE_INFO_502 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct SHARE_INFO_503 {
    pub shi503_netname: ::windows::core::PWSTR,
    pub shi503_type: SHARE_TYPE,
    pub shi503_remark: ::windows::core::PWSTR,
    pub shi503_permissions: SHARE_INFO_PERMISSIONS,
    pub shi503_max_uses: u32,
    pub shi503_current_uses: u32,
    pub shi503_path: ::windows::core::PWSTR,
    pub shi503_passwd: ::windows::core::PWSTR,
    pub shi503_servername: ::windows::core::PWSTR,
    pub shi503_reserved: u32,
    pub shi503_security_descriptor: super::super::Security::PSECURITY_DESCRIPTOR,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for SHARE_INFO_503 {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for SHARE_INFO_503 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::fmt::Debug for SHARE_INFO_503 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHARE_INFO_503")
            .field("shi503_netname", &self.shi503_netname)
            .field("shi503_type", &self.shi503_type)
            .field("shi503_remark", &self.shi503_remark)
            .field("shi503_permissions", &self.shi503_permissions)
            .field("shi503_max_uses", &self.shi503_max_uses)
            .field("shi503_current_uses", &self.shi503_current_uses)
            .field("shi503_path", &self.shi503_path)
            .field("shi503_passwd", &self.shi503_passwd)
            .field("shi503_servername", &self.shi503_servername)
            .field("shi503_reserved", &self.shi503_reserved)
            .field("shi503_security_descriptor", &self.shi503_security_descriptor)
            .finish()
    }
}
#[cfg(feature = "Win32_Security")]
impl ::windows::core::TypeKind for SHARE_INFO_503 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::PartialEq for SHARE_INFO_503 {
    fn eq(&self, other: &Self) -> bool {
        self.shi503_netname == other.shi503_netname && self.shi503_type == other.shi503_type && self.shi503_remark == other.shi503_remark && self.shi503_permissions == other.shi503_permissions && self.shi503_max_uses == other.shi503_max_uses && self.shi503_current_uses == other.shi503_current_uses && self.shi503_path == other.shi503_path && self.shi503_passwd == other.shi503_passwd && self.shi503_servername == other.shi503_servername && self.shi503_reserved == other.shi503_reserved && self.shi503_security_descriptor == other.shi503_security_descriptor
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::Eq for SHARE_INFO_503 {}
#[cfg(feature = "Win32_Security")]
impl ::core::default::Default for SHARE_INFO_503 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct STAT_SERVER_0 {
    pub sts0_start: u32,
    pub sts0_fopens: u32,
    pub sts0_devopens: u32,
    pub sts0_jobsqueued: u32,
    pub sts0_sopens: u32,
    pub sts0_stimedout: u32,
    pub sts0_serrorout: u32,
    pub sts0_pwerrors: u32,
    pub sts0_permerrors: u32,
    pub sts0_syserrors: u32,
    pub sts0_bytessent_low: u32,
    pub sts0_bytessent_high: u32,
    pub sts0_bytesrcvd_low: u32,
    pub sts0_bytesrcvd_high: u32,
    pub sts0_avresponse: u32,
    pub sts0_reqbufneed: u32,
    pub sts0_bigbufneed: u32,
}
impl ::core::marker::Copy for STAT_SERVER_0 {}
impl ::core::clone::Clone for STAT_SERVER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STAT_SERVER_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STAT_SERVER_0")
            .field("sts0_start", &self.sts0_start)
            .field("sts0_fopens", &self.sts0_fopens)
            .field("sts0_devopens", &self.sts0_devopens)
            .field("sts0_jobsqueued", &self.sts0_jobsqueued)
            .field("sts0_sopens", &self.sts0_sopens)
            .field("sts0_stimedout", &self.sts0_stimedout)
            .field("sts0_serrorout", &self.sts0_serrorout)
            .field("sts0_pwerrors", &self.sts0_pwerrors)
            .field("sts0_permerrors", &self.sts0_permerrors)
            .field("sts0_syserrors", &self.sts0_syserrors)
            .field("sts0_bytessent_low", &self.sts0_bytessent_low)
            .field("sts0_bytessent_high", &self.sts0_bytessent_high)
            .field("sts0_bytesrcvd_low", &self.sts0_bytesrcvd_low)
            .field("sts0_bytesrcvd_high", &self.sts0_bytesrcvd_high)
            .field("sts0_avresponse", &self.sts0_avresponse)
            .field("sts0_reqbufneed", &self.sts0_reqbufneed)
            .field("sts0_bigbufneed", &self.sts0_bigbufneed)
            .finish()
    }
}
impl ::windows::core::TypeKind for STAT_SERVER_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for STAT_SERVER_0 {
    fn eq(&self, other: &Self) -> bool {
        self.sts0_start == other.sts0_start
            && self.sts0_fopens == other.sts0_fopens
            && self.sts0_devopens == other.sts0_devopens
            && self.sts0_jobsqueued == other.sts0_jobsqueued
            && self.sts0_sopens == other.sts0_sopens
            && self.sts0_stimedout == other.sts0_stimedout
            && self.sts0_serrorout == other.sts0_serrorout
            && self.sts0_pwerrors == other.sts0_pwerrors
            && self.sts0_permerrors == other.sts0_permerrors
            && self.sts0_syserrors == other.sts0_syserrors
            && self.sts0_bytessent_low == other.sts0_bytessent_low
            && self.sts0_bytessent_high == other.sts0_bytessent_high
            && self.sts0_bytesrcvd_low == other.sts0_bytesrcvd_low
            && self.sts0_bytesrcvd_high == other.sts0_bytesrcvd_high
            && self.sts0_avresponse == other.sts0_avresponse
            && self.sts0_reqbufneed == other.sts0_reqbufneed
            && self.sts0_bigbufneed == other.sts0_bigbufneed
    }
}
impl ::core::cmp::Eq for STAT_SERVER_0 {}
impl ::core::default::Default for STAT_SERVER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct STAT_WORKSTATION_0 {
    pub StatisticsStartTime: i64,
    pub BytesReceived: i64,
    pub SmbsReceived: i64,
    pub PagingReadBytesRequested: i64,
    pub NonPagingReadBytesRequested: i64,
    pub CacheReadBytesRequested: i64,
    pub NetworkReadBytesRequested: i64,
    pub BytesTransmitted: i64,
    pub SmbsTransmitted: i64,
    pub PagingWriteBytesRequested: i64,
    pub NonPagingWriteBytesRequested: i64,
    pub CacheWriteBytesRequested: i64,
    pub NetworkWriteBytesRequested: i64,
    pub InitiallyFailedOperations: u32,
    pub FailedCompletionOperations: u32,
    pub ReadOperations: u32,
    pub RandomReadOperations: u32,
    pub ReadSmbs: u32,
    pub LargeReadSmbs: u32,
    pub SmallReadSmbs: u32,
    pub WriteOperations: u32,
    pub RandomWriteOperations: u32,
    pub WriteSmbs: u32,
    pub LargeWriteSmbs: u32,
    pub SmallWriteSmbs: u32,
    pub RawReadsDenied: u32,
    pub RawWritesDenied: u32,
    pub NetworkErrors: u32,
    pub Sessions: u32,
    pub FailedSessions: u32,
    pub Reconnects: u32,
    pub CoreConnects: u32,
    pub Lanman20Connects: u32,
    pub Lanman21Connects: u32,
    pub LanmanNtConnects: u32,
    pub ServerDisconnects: u32,
    pub HungSessions: u32,
    pub UseCount: u32,
    pub FailedUseCount: u32,
    pub CurrentCommands: u32,
}
impl ::core::marker::Copy for STAT_WORKSTATION_0 {}
impl ::core::clone::Clone for STAT_WORKSTATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STAT_WORKSTATION_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STAT_WORKSTATION_0")
            .field("StatisticsStartTime", &self.StatisticsStartTime)
            .field("BytesReceived", &self.BytesReceived)
            .field("SmbsReceived", &self.SmbsReceived)
            .field("PagingReadBytesRequested", &self.PagingReadBytesRequested)
            .field("NonPagingReadBytesRequested", &self.NonPagingReadBytesRequested)
            .field("CacheReadBytesRequested", &self.CacheReadBytesRequested)
            .field("NetworkReadBytesRequested", &self.NetworkReadBytesRequested)
            .field("BytesTransmitted", &self.BytesTransmitted)
            .field("SmbsTransmitted", &self.SmbsTransmitted)
            .field("PagingWriteBytesRequested", &self.PagingWriteBytesRequested)
            .field("NonPagingWriteBytesRequested", &self.NonPagingWriteBytesRequested)
            .field("CacheWriteBytesRequested", &self.CacheWriteBytesRequested)
            .field("NetworkWriteBytesRequested", &self.NetworkWriteBytesRequested)
            .field("InitiallyFailedOperations", &self.InitiallyFailedOperations)
            .field("FailedCompletionOperations", &self.FailedCompletionOperations)
            .field("ReadOperations", &self.ReadOperations)
            .field("RandomReadOperations", &self.RandomReadOperations)
            .field("ReadSmbs", &self.ReadSmbs)
            .field("LargeReadSmbs", &self.LargeReadSmbs)
            .field("SmallReadSmbs", &self.SmallReadSmbs)
            .field("WriteOperations", &self.WriteOperations)
            .field("RandomWriteOperations", &self.RandomWriteOperations)
            .field("WriteSmbs", &self.WriteSmbs)
            .field("LargeWriteSmbs", &self.LargeWriteSmbs)
            .field("SmallWriteSmbs", &self.SmallWriteSmbs)
            .field("RawReadsDenied", &self.RawReadsDenied)
            .field("RawWritesDenied", &self.RawWritesDenied)
            .field("NetworkErrors", &self.NetworkErrors)
            .field("Sessions", &self.Sessions)
            .field("FailedSessions", &self.FailedSessions)
            .field("Reconnects", &self.Reconnects)
            .field("CoreConnects", &self.CoreConnects)
            .field("Lanman20Connects", &self.Lanman20Connects)
            .field("Lanman21Connects", &self.Lanman21Connects)
            .field("LanmanNtConnects", &self.LanmanNtConnects)
            .field("ServerDisconnects", &self.ServerDisconnects)
            .field("HungSessions", &self.HungSessions)
            .field("UseCount", &self.UseCount)
            .field("FailedUseCount", &self.FailedUseCount)
            .field("CurrentCommands", &self.CurrentCommands)
            .finish()
    }
}
impl ::windows::core::TypeKind for STAT_WORKSTATION_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for STAT_WORKSTATION_0 {
    fn eq(&self, other: &Self) -> bool {
        self.StatisticsStartTime == other.StatisticsStartTime
            && self.BytesReceived == other.BytesReceived
            && self.SmbsReceived == other.SmbsReceived
            && self.PagingReadBytesRequested == other.PagingReadBytesRequested
            && self.NonPagingReadBytesRequested == other.NonPagingReadBytesRequested
            && self.CacheReadBytesRequested == other.CacheReadBytesRequested
            && self.NetworkReadBytesRequested == other.NetworkReadBytesRequested
            && self.BytesTransmitted == other.BytesTransmitted
            && self.SmbsTransmitted == other.SmbsTransmitted
            && self.PagingWriteBytesRequested == other.PagingWriteBytesRequested
            && self.NonPagingWriteBytesRequested == other.NonPagingWriteBytesRequested
            && self.CacheWriteBytesRequested == other.CacheWriteBytesRequested
            && self.NetworkWriteBytesRequested == other.NetworkWriteBytesRequested
            && self.InitiallyFailedOperations == other.InitiallyFailedOperations
            && self.FailedCompletionOperations == other.FailedCompletionOperations
            && self.ReadOperations == other.ReadOperations
            && self.RandomReadOperations == other.RandomReadOperations
            && self.ReadSmbs == other.ReadSmbs
            && self.LargeReadSmbs == other.LargeReadSmbs
            && self.SmallReadSmbs == other.SmallReadSmbs
            && self.WriteOperations == other.WriteOperations
            && self.RandomWriteOperations == other.RandomWriteOperations
            && self.WriteSmbs == other.WriteSmbs
            && self.LargeWriteSmbs == other.LargeWriteSmbs
            && self.SmallWriteSmbs == other.SmallWriteSmbs
            && self.RawReadsDenied == other.RawReadsDenied
            && self.RawWritesDenied == other.RawWritesDenied
            && self.NetworkErrors == other.NetworkErrors
            && self.Sessions == other.Sessions
            && self.FailedSessions == other.FailedSessions
            && self.Reconnects == other.Reconnects
            && self.CoreConnects == other.CoreConnects
            && self.Lanman20Connects == other.Lanman20Connects
            && self.Lanman21Connects == other.Lanman21Connects
            && self.LanmanNtConnects == other.LanmanNtConnects
            && self.ServerDisconnects == other.ServerDisconnects
            && self.HungSessions == other.HungSessions
            && self.UseCount == other.UseCount
            && self.FailedUseCount == other.FailedUseCount
            && self.CurrentCommands == other.CurrentCommands
    }
}
impl ::core::cmp::Eq for STAT_WORKSTATION_0 {}
impl ::core::default::Default for STAT_WORKSTATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TAPE_ERASE {
    pub Type: ERASE_TAPE_TYPE,
    pub Immediate: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TAPE_ERASE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TAPE_ERASE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TAPE_ERASE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TAPE_ERASE").field("Type", &self.Type).field("Immediate", &self.Immediate).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for TAPE_ERASE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TAPE_ERASE {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Immediate == other.Immediate
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TAPE_ERASE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TAPE_ERASE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct TAPE_GET_POSITION {
    pub Type: TAPE_POSITION_TYPE,
    pub Partition: u32,
    pub Offset: i64,
}
impl ::core::marker::Copy for TAPE_GET_POSITION {}
impl ::core::clone::Clone for TAPE_GET_POSITION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TAPE_GET_POSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TAPE_GET_POSITION").field("Type", &self.Type).field("Partition", &self.Partition).field("Offset", &self.Offset).finish()
    }
}
impl ::windows::core::TypeKind for TAPE_GET_POSITION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TAPE_GET_POSITION {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Partition == other.Partition && self.Offset == other.Offset
    }
}
impl ::core::cmp::Eq for TAPE_GET_POSITION {}
impl ::core::default::Default for TAPE_GET_POSITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TAPE_PREPARE {
    pub Operation: PREPARE_TAPE_OPERATION,
    pub Immediate: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TAPE_PREPARE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TAPE_PREPARE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TAPE_PREPARE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TAPE_PREPARE").field("Operation", &self.Operation).field("Immediate", &self.Immediate).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for TAPE_PREPARE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TAPE_PREPARE {
    fn eq(&self, other: &Self) -> bool {
        self.Operation == other.Operation && self.Immediate == other.Immediate
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TAPE_PREPARE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TAPE_PREPARE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TAPE_SET_POSITION {
    pub Method: TAPE_POSITION_METHOD,
    pub Partition: u32,
    pub Offset: i64,
    pub Immediate: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TAPE_SET_POSITION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TAPE_SET_POSITION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TAPE_SET_POSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TAPE_SET_POSITION").field("Method", &self.Method).field("Partition", &self.Partition).field("Offset", &self.Offset).field("Immediate", &self.Immediate).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for TAPE_SET_POSITION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TAPE_SET_POSITION {
    fn eq(&self, other: &Self) -> bool {
        self.Method == other.Method && self.Partition == other.Partition && self.Offset == other.Offset && self.Immediate == other.Immediate
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TAPE_SET_POSITION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TAPE_SET_POSITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TAPE_WRITE_MARKS {
    pub Type: TAPEMARK_TYPE,
    pub Count: u32,
    pub Immediate: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TAPE_WRITE_MARKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TAPE_WRITE_MARKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TAPE_WRITE_MARKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TAPE_WRITE_MARKS").field("Type", &self.Type).field("Count", &self.Count).field("Immediate", &self.Immediate).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for TAPE_WRITE_MARKS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TAPE_WRITE_MARKS {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Count == other.Count && self.Immediate == other.Immediate
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TAPE_WRITE_MARKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TAPE_WRITE_MARKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct TRANSACTION_NOTIFICATION {
    pub TransactionKey: *mut ::core::ffi::c_void,
    pub TransactionNotification: u32,
    pub TmVirtualClock: i64,
    pub ArgumentLength: u32,
}
impl ::core::marker::Copy for TRANSACTION_NOTIFICATION {}
impl ::core::clone::Clone for TRANSACTION_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRANSACTION_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSACTION_NOTIFICATION").field("TransactionKey", &self.TransactionKey).field("TransactionNotification", &self.TransactionNotification).field("TmVirtualClock", &self.TmVirtualClock).field("ArgumentLength", &self.ArgumentLength).finish()
    }
}
impl ::windows::core::TypeKind for TRANSACTION_NOTIFICATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TRANSACTION_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.TransactionKey == other.TransactionKey && self.TransactionNotification == other.TransactionNotification && self.TmVirtualClock == other.TmVirtualClock && self.ArgumentLength == other.ArgumentLength
    }
}
impl ::core::cmp::Eq for TRANSACTION_NOTIFICATION {}
impl ::core::default::Default for TRANSACTION_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT {
    pub MarshalCookie: u32,
    pub UOW: ::windows::core::GUID,
}
impl ::core::marker::Copy for TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT {}
impl ::core::clone::Clone for TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT").field("MarshalCookie", &self.MarshalCookie).field("UOW", &self.UOW).finish()
    }
}
impl ::windows::core::TypeKind for TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT {
    fn eq(&self, other: &Self) -> bool {
        self.MarshalCookie == other.MarshalCookie && self.UOW == other.UOW
    }
}
impl ::core::cmp::Eq for TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT {}
impl ::core::default::Default for TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT {
    pub PropagationCookie: u32,
    pub UOW: ::windows::core::GUID,
    pub TmIdentity: ::windows::core::GUID,
    pub BufferLength: u32,
}
impl ::core::marker::Copy for TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT {}
impl ::core::clone::Clone for TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT").field("PropagationCookie", &self.PropagationCookie).field("UOW", &self.UOW).field("TmIdentity", &self.TmIdentity).field("BufferLength", &self.BufferLength).finish()
    }
}
impl ::windows::core::TypeKind for TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT {
    fn eq(&self, other: &Self) -> bool {
        self.PropagationCookie == other.PropagationCookie && self.UOW == other.UOW && self.TmIdentity == other.TmIdentity && self.BufferLength == other.BufferLength
    }
}
impl ::core::cmp::Eq for TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT {}
impl ::core::default::Default for TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT {
    pub EnlistmentId: ::windows::core::GUID,
    pub UOW: ::windows::core::GUID,
}
impl ::core::marker::Copy for TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT {}
impl ::core::clone::Clone for TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT").field("EnlistmentId", &self.EnlistmentId).field("UOW", &self.UOW).finish()
    }
}
impl ::windows::core::TypeKind for TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT {
    fn eq(&self, other: &Self) -> bool {
        self.EnlistmentId == other.EnlistmentId && self.UOW == other.UOW
    }
}
impl ::core::cmp::Eq for TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT {}
impl ::core::default::Default for TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT {
    pub SavepointId: u32,
}
impl ::core::marker::Copy for TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT {}
impl ::core::clone::Clone for TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT").field("SavepointId", &self.SavepointId).finish()
    }
}
impl ::windows::core::TypeKind for TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT {
    fn eq(&self, other: &Self) -> bool {
        self.SavepointId == other.SavepointId
    }
}
impl ::core::cmp::Eq for TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT {}
impl ::core::default::Default for TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT {
    pub TmIdentity: ::windows::core::GUID,
    pub Flags: u32,
}
impl ::core::marker::Copy for TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT {}
impl ::core::clone::Clone for TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT").field("TmIdentity", &self.TmIdentity).field("Flags", &self.Flags).finish()
    }
}
impl ::windows::core::TypeKind for TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT {
    fn eq(&self, other: &Self) -> bool {
        self.TmIdentity == other.TmIdentity && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT {}
impl ::core::default::Default for TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct TXF_ID {
    pub Anonymous: TXF_ID_0,
}
impl ::core::marker::Copy for TXF_ID {}
impl ::core::clone::Clone for TXF_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for TXF_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for TXF_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct TXF_ID_0 {
    pub LowPart: i64,
    pub HighPart: i64,
}
impl ::core::marker::Copy for TXF_ID_0 {}
impl ::core::clone::Clone for TXF_ID_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for TXF_ID_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for TXF_ID_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct TXF_LOG_RECORD_AFFECTED_FILE {
    pub Version: u16,
    pub RecordLength: u32,
    pub Flags: u32,
    pub TxfFileId: TXF_ID,
    pub KtmGuid: ::windows::core::GUID,
    pub FileNameLength: u32,
    pub FileNameByteOffsetInStructure: u32,
}
impl ::core::marker::Copy for TXF_LOG_RECORD_AFFECTED_FILE {}
impl ::core::clone::Clone for TXF_LOG_RECORD_AFFECTED_FILE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for TXF_LOG_RECORD_AFFECTED_FILE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for TXF_LOG_RECORD_AFFECTED_FILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct TXF_LOG_RECORD_BASE {
    pub Version: u16,
    pub RecordType: TXF_LOG_RECORD_TYPE,
    pub RecordLength: u32,
}
impl ::core::marker::Copy for TXF_LOG_RECORD_BASE {}
impl ::core::clone::Clone for TXF_LOG_RECORD_BASE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for TXF_LOG_RECORD_BASE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for TXF_LOG_RECORD_BASE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct TXF_LOG_RECORD_TRUNCATE {
    pub Version: u16,
    pub RecordType: u16,
    pub RecordLength: u32,
    pub Flags: u32,
    pub TxfFileId: TXF_ID,
    pub KtmGuid: ::windows::core::GUID,
    pub NewFileSize: i64,
    pub FileNameLength: u32,
    pub FileNameByteOffsetInStructure: u32,
}
impl ::core::marker::Copy for TXF_LOG_RECORD_TRUNCATE {}
impl ::core::clone::Clone for TXF_LOG_RECORD_TRUNCATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for TXF_LOG_RECORD_TRUNCATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for TXF_LOG_RECORD_TRUNCATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct TXF_LOG_RECORD_WRITE {
    pub Version: u16,
    pub RecordType: u16,
    pub RecordLength: u32,
    pub Flags: u32,
    pub TxfFileId: TXF_ID,
    pub KtmGuid: ::windows::core::GUID,
    pub ByteOffsetInFile: i64,
    pub NumBytesWritten: u32,
    pub ByteOffsetInStructure: u32,
    pub FileNameLength: u32,
    pub FileNameByteOffsetInStructure: u32,
}
impl ::core::marker::Copy for TXF_LOG_RECORD_WRITE {}
impl ::core::clone::Clone for TXF_LOG_RECORD_WRITE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for TXF_LOG_RECORD_WRITE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for TXF_LOG_RECORD_WRITE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VOLUME_ALLOCATE_BC_STREAM_INPUT {
    pub Version: u32,
    pub RequestsPerPeriod: u32,
    pub Period: u32,
    pub RetryFailures: super::super::Foundation::BOOLEAN,
    pub Discardable: super::super::Foundation::BOOLEAN,
    pub Reserved1: [super::super::Foundation::BOOLEAN; 2],
    pub LowestByteOffset: u64,
    pub HighestByteOffset: u64,
    pub AccessType: u32,
    pub AccessMode: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VOLUME_ALLOCATE_BC_STREAM_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VOLUME_ALLOCATE_BC_STREAM_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VOLUME_ALLOCATE_BC_STREAM_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_ALLOCATE_BC_STREAM_INPUT").field("Version", &self.Version).field("RequestsPerPeriod", &self.RequestsPerPeriod).field("Period", &self.Period).field("RetryFailures", &self.RetryFailures).field("Discardable", &self.Discardable).field("Reserved1", &self.Reserved1).field("LowestByteOffset", &self.LowestByteOffset).field("HighestByteOffset", &self.HighestByteOffset).field("AccessType", &self.AccessType).field("AccessMode", &self.AccessMode).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for VOLUME_ALLOCATE_BC_STREAM_INPUT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VOLUME_ALLOCATE_BC_STREAM_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.RequestsPerPeriod == other.RequestsPerPeriod && self.Period == other.Period && self.RetryFailures == other.RetryFailures && self.Discardable == other.Discardable && self.Reserved1 == other.Reserved1 && self.LowestByteOffset == other.LowestByteOffset && self.HighestByteOffset == other.HighestByteOffset && self.AccessType == other.AccessType && self.AccessMode == other.AccessMode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VOLUME_ALLOCATE_BC_STREAM_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VOLUME_ALLOCATE_BC_STREAM_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct VOLUME_ALLOCATE_BC_STREAM_OUTPUT {
    pub RequestSize: u64,
    pub NumOutStandingRequests: u32,
}
impl ::core::marker::Copy for VOLUME_ALLOCATE_BC_STREAM_OUTPUT {}
impl ::core::clone::Clone for VOLUME_ALLOCATE_BC_STREAM_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VOLUME_ALLOCATE_BC_STREAM_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_ALLOCATE_BC_STREAM_OUTPUT").field("RequestSize", &self.RequestSize).field("NumOutStandingRequests", &self.NumOutStandingRequests).finish()
    }
}
impl ::windows::core::TypeKind for VOLUME_ALLOCATE_BC_STREAM_OUTPUT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VOLUME_ALLOCATE_BC_STREAM_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.RequestSize == other.RequestSize && self.NumOutStandingRequests == other.NumOutStandingRequests
    }
}
impl ::core::cmp::Eq for VOLUME_ALLOCATE_BC_STREAM_OUTPUT {}
impl ::core::default::Default for VOLUME_ALLOCATE_BC_STREAM_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct VOLUME_ALLOCATION_HINT_INPUT {
    pub ClusterSize: u32,
    pub NumberOfClusters: u32,
    pub StartingClusterNumber: i64,
}
impl ::core::marker::Copy for VOLUME_ALLOCATION_HINT_INPUT {}
impl ::core::clone::Clone for VOLUME_ALLOCATION_HINT_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VOLUME_ALLOCATION_HINT_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_ALLOCATION_HINT_INPUT").field("ClusterSize", &self.ClusterSize).field("NumberOfClusters", &self.NumberOfClusters).field("StartingClusterNumber", &self.StartingClusterNumber).finish()
    }
}
impl ::windows::core::TypeKind for VOLUME_ALLOCATION_HINT_INPUT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VOLUME_ALLOCATION_HINT_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.ClusterSize == other.ClusterSize && self.NumberOfClusters == other.NumberOfClusters && self.StartingClusterNumber == other.StartingClusterNumber
    }
}
impl ::core::cmp::Eq for VOLUME_ALLOCATION_HINT_INPUT {}
impl ::core::default::Default for VOLUME_ALLOCATION_HINT_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct VOLUME_ALLOCATION_HINT_OUTPUT {
    pub Bitmap: [u32; 1],
}
impl ::core::marker::Copy for VOLUME_ALLOCATION_HINT_OUTPUT {}
impl ::core::clone::Clone for VOLUME_ALLOCATION_HINT_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VOLUME_ALLOCATION_HINT_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_ALLOCATION_HINT_OUTPUT").field("Bitmap", &self.Bitmap).finish()
    }
}
impl ::windows::core::TypeKind for VOLUME_ALLOCATION_HINT_OUTPUT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VOLUME_ALLOCATION_HINT_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Bitmap == other.Bitmap
    }
}
impl ::core::cmp::Eq for VOLUME_ALLOCATION_HINT_OUTPUT {}
impl ::core::default::Default for VOLUME_ALLOCATION_HINT_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct VOLUME_CRITICAL_IO {
    pub AccessType: u32,
    pub ExtentsCount: u32,
    pub Extents: [FILE_EXTENT; 1],
}
impl ::core::marker::Copy for VOLUME_CRITICAL_IO {}
impl ::core::clone::Clone for VOLUME_CRITICAL_IO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VOLUME_CRITICAL_IO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_CRITICAL_IO").field("AccessType", &self.AccessType).field("ExtentsCount", &self.ExtentsCount).field("Extents", &self.Extents).finish()
    }
}
impl ::windows::core::TypeKind for VOLUME_CRITICAL_IO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VOLUME_CRITICAL_IO {
    fn eq(&self, other: &Self) -> bool {
        self.AccessType == other.AccessType && self.ExtentsCount == other.ExtentsCount && self.Extents == other.Extents
    }
}
impl ::core::cmp::Eq for VOLUME_CRITICAL_IO {}
impl ::core::default::Default for VOLUME_CRITICAL_IO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct VOLUME_FAILOVER_SET {
    pub NumberOfDisks: u32,
    pub DiskNumbers: [u32; 1],
}
impl ::core::marker::Copy for VOLUME_FAILOVER_SET {}
impl ::core::clone::Clone for VOLUME_FAILOVER_SET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VOLUME_FAILOVER_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_FAILOVER_SET").field("NumberOfDisks", &self.NumberOfDisks).field("DiskNumbers", &self.DiskNumbers).finish()
    }
}
impl ::windows::core::TypeKind for VOLUME_FAILOVER_SET {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VOLUME_FAILOVER_SET {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfDisks == other.NumberOfDisks && self.DiskNumbers == other.DiskNumbers
    }
}
impl ::core::cmp::Eq for VOLUME_FAILOVER_SET {}
impl ::core::default::Default for VOLUME_FAILOVER_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct VOLUME_GET_BC_PROPERTIES_INPUT {
    pub Version: u32,
    pub Reserved1: u32,
    pub LowestByteOffset: u64,
    pub HighestByteOffset: u64,
    pub AccessType: u32,
    pub AccessMode: u32,
}
impl ::core::marker::Copy for VOLUME_GET_BC_PROPERTIES_INPUT {}
impl ::core::clone::Clone for VOLUME_GET_BC_PROPERTIES_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VOLUME_GET_BC_PROPERTIES_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_GET_BC_PROPERTIES_INPUT").field("Version", &self.Version).field("Reserved1", &self.Reserved1).field("LowestByteOffset", &self.LowestByteOffset).field("HighestByteOffset", &self.HighestByteOffset).field("AccessType", &self.AccessType).field("AccessMode", &self.AccessMode).finish()
    }
}
impl ::windows::core::TypeKind for VOLUME_GET_BC_PROPERTIES_INPUT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VOLUME_GET_BC_PROPERTIES_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Reserved1 == other.Reserved1 && self.LowestByteOffset == other.LowestByteOffset && self.HighestByteOffset == other.HighestByteOffset && self.AccessType == other.AccessType && self.AccessMode == other.AccessMode
    }
}
impl ::core::cmp::Eq for VOLUME_GET_BC_PROPERTIES_INPUT {}
impl ::core::default::Default for VOLUME_GET_BC_PROPERTIES_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct VOLUME_GET_BC_PROPERTIES_OUTPUT {
    pub MaximumRequestsPerPeriod: u32,
    pub MinimumPeriod: u32,
    pub MaximumRequestSize: u64,
    pub EstimatedTimePerRequest: u32,
    pub NumOutStandingRequests: u32,
    pub RequestSize: u64,
}
impl ::core::marker::Copy for VOLUME_GET_BC_PROPERTIES_OUTPUT {}
impl ::core::clone::Clone for VOLUME_GET_BC_PROPERTIES_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VOLUME_GET_BC_PROPERTIES_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_GET_BC_PROPERTIES_OUTPUT").field("MaximumRequestsPerPeriod", &self.MaximumRequestsPerPeriod).field("MinimumPeriod", &self.MinimumPeriod).field("MaximumRequestSize", &self.MaximumRequestSize).field("EstimatedTimePerRequest", &self.EstimatedTimePerRequest).field("NumOutStandingRequests", &self.NumOutStandingRequests).field("RequestSize", &self.RequestSize).finish()
    }
}
impl ::windows::core::TypeKind for VOLUME_GET_BC_PROPERTIES_OUTPUT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VOLUME_GET_BC_PROPERTIES_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.MaximumRequestsPerPeriod == other.MaximumRequestsPerPeriod && self.MinimumPeriod == other.MinimumPeriod && self.MaximumRequestSize == other.MaximumRequestSize && self.EstimatedTimePerRequest == other.EstimatedTimePerRequest && self.NumOutStandingRequests == other.NumOutStandingRequests && self.RequestSize == other.RequestSize
    }
}
impl ::core::cmp::Eq for VOLUME_GET_BC_PROPERTIES_OUTPUT {}
impl ::core::default::Default for VOLUME_GET_BC_PROPERTIES_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct VOLUME_LOGICAL_OFFSET {
    pub LogicalOffset: i64,
}
impl ::core::marker::Copy for VOLUME_LOGICAL_OFFSET {}
impl ::core::clone::Clone for VOLUME_LOGICAL_OFFSET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VOLUME_LOGICAL_OFFSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_LOGICAL_OFFSET").field("LogicalOffset", &self.LogicalOffset).finish()
    }
}
impl ::windows::core::TypeKind for VOLUME_LOGICAL_OFFSET {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VOLUME_LOGICAL_OFFSET {
    fn eq(&self, other: &Self) -> bool {
        self.LogicalOffset == other.LogicalOffset
    }
}
impl ::core::cmp::Eq for VOLUME_LOGICAL_OFFSET {}
impl ::core::default::Default for VOLUME_LOGICAL_OFFSET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct VOLUME_NUMBER {
    pub VolumeNumber: u32,
    pub VolumeManagerName: [u16; 8],
}
impl ::core::marker::Copy for VOLUME_NUMBER {}
impl ::core::clone::Clone for VOLUME_NUMBER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VOLUME_NUMBER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_NUMBER").field("VolumeNumber", &self.VolumeNumber).field("VolumeManagerName", &self.VolumeManagerName).finish()
    }
}
impl ::windows::core::TypeKind for VOLUME_NUMBER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VOLUME_NUMBER {
    fn eq(&self, other: &Self) -> bool {
        self.VolumeNumber == other.VolumeNumber && self.VolumeManagerName == other.VolumeManagerName
    }
}
impl ::core::cmp::Eq for VOLUME_NUMBER {}
impl ::core::default::Default for VOLUME_NUMBER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct VOLUME_PHYSICAL_OFFSET {
    pub DiskNumber: u32,
    pub Offset: i64,
}
impl ::core::marker::Copy for VOLUME_PHYSICAL_OFFSET {}
impl ::core::clone::Clone for VOLUME_PHYSICAL_OFFSET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VOLUME_PHYSICAL_OFFSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_PHYSICAL_OFFSET").field("DiskNumber", &self.DiskNumber).field("Offset", &self.Offset).finish()
    }
}
impl ::windows::core::TypeKind for VOLUME_PHYSICAL_OFFSET {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VOLUME_PHYSICAL_OFFSET {
    fn eq(&self, other: &Self) -> bool {
        self.DiskNumber == other.DiskNumber && self.Offset == other.Offset
    }
}
impl ::core::cmp::Eq for VOLUME_PHYSICAL_OFFSET {}
impl ::core::default::Default for VOLUME_PHYSICAL_OFFSET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct VOLUME_PHYSICAL_OFFSETS {
    pub NumberOfPhysicalOffsets: u32,
    pub PhysicalOffset: [VOLUME_PHYSICAL_OFFSET; 1],
}
impl ::core::marker::Copy for VOLUME_PHYSICAL_OFFSETS {}
impl ::core::clone::Clone for VOLUME_PHYSICAL_OFFSETS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VOLUME_PHYSICAL_OFFSETS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_PHYSICAL_OFFSETS").field("NumberOfPhysicalOffsets", &self.NumberOfPhysicalOffsets).field("PhysicalOffset", &self.PhysicalOffset).finish()
    }
}
impl ::windows::core::TypeKind for VOLUME_PHYSICAL_OFFSETS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VOLUME_PHYSICAL_OFFSETS {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfPhysicalOffsets == other.NumberOfPhysicalOffsets && self.PhysicalOffset == other.PhysicalOffset
    }
}
impl ::core::cmp::Eq for VOLUME_PHYSICAL_OFFSETS {}
impl ::core::default::Default for VOLUME_PHYSICAL_OFFSETS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct VOLUME_READ_PLEX_INPUT {
    pub ByteOffset: i64,
    pub Length: u32,
    pub PlexNumber: u32,
}
impl ::core::marker::Copy for VOLUME_READ_PLEX_INPUT {}
impl ::core::clone::Clone for VOLUME_READ_PLEX_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VOLUME_READ_PLEX_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_READ_PLEX_INPUT").field("ByteOffset", &self.ByteOffset).field("Length", &self.Length).field("PlexNumber", &self.PlexNumber).finish()
    }
}
impl ::windows::core::TypeKind for VOLUME_READ_PLEX_INPUT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VOLUME_READ_PLEX_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.ByteOffset == other.ByteOffset && self.Length == other.Length && self.PlexNumber == other.PlexNumber
    }
}
impl ::core::cmp::Eq for VOLUME_READ_PLEX_INPUT {}
impl ::core::default::Default for VOLUME_READ_PLEX_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VOLUME_SET_GPT_ATTRIBUTES_INFORMATION {
    pub GptAttributes: u64,
    pub RevertOnClose: super::super::Foundation::BOOLEAN,
    pub ApplyToAllConnectedVolumes: super::super::Foundation::BOOLEAN,
    pub Reserved1: u16,
    pub Reserved2: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VOLUME_SET_GPT_ATTRIBUTES_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VOLUME_SET_GPT_ATTRIBUTES_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VOLUME_SET_GPT_ATTRIBUTES_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_SET_GPT_ATTRIBUTES_INFORMATION").field("GptAttributes", &self.GptAttributes).field("RevertOnClose", &self.RevertOnClose).field("ApplyToAllConnectedVolumes", &self.ApplyToAllConnectedVolumes).field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for VOLUME_SET_GPT_ATTRIBUTES_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VOLUME_SET_GPT_ATTRIBUTES_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.GptAttributes == other.GptAttributes && self.RevertOnClose == other.RevertOnClose && self.ApplyToAllConnectedVolumes == other.ApplyToAllConnectedVolumes && self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VOLUME_SET_GPT_ATTRIBUTES_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VOLUME_SET_GPT_ATTRIBUTES_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct VOLUME_SHRINK_INFO {
    pub VolumeSize: u64,
}
impl ::core::marker::Copy for VOLUME_SHRINK_INFO {}
impl ::core::clone::Clone for VOLUME_SHRINK_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VOLUME_SHRINK_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_SHRINK_INFO").field("VolumeSize", &self.VolumeSize).finish()
    }
}
impl ::windows::core::TypeKind for VOLUME_SHRINK_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VOLUME_SHRINK_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.VolumeSize == other.VolumeSize
    }
}
impl ::core::cmp::Eq for VOLUME_SHRINK_INFO {}
impl ::core::default::Default for VOLUME_SHRINK_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct VS_FIXEDFILEINFO {
    pub dwSignature: u32,
    pub dwStrucVersion: u32,
    pub dwFileVersionMS: u32,
    pub dwFileVersionLS: u32,
    pub dwProductVersionMS: u32,
    pub dwProductVersionLS: u32,
    pub dwFileFlagsMask: u32,
    pub dwFileFlags: VS_FIXEDFILEINFO_FILE_FLAGS,
    pub dwFileOS: VS_FIXEDFILEINFO_FILE_OS,
    pub dwFileType: VS_FIXEDFILEINFO_FILE_TYPE,
    pub dwFileSubtype: VS_FIXEDFILEINFO_FILE_SUBTYPE,
    pub dwFileDateMS: u32,
    pub dwFileDateLS: u32,
}
impl ::core::marker::Copy for VS_FIXEDFILEINFO {}
impl ::core::clone::Clone for VS_FIXEDFILEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VS_FIXEDFILEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VS_FIXEDFILEINFO")
            .field("dwSignature", &self.dwSignature)
            .field("dwStrucVersion", &self.dwStrucVersion)
            .field("dwFileVersionMS", &self.dwFileVersionMS)
            .field("dwFileVersionLS", &self.dwFileVersionLS)
            .field("dwProductVersionMS", &self.dwProductVersionMS)
            .field("dwProductVersionLS", &self.dwProductVersionLS)
            .field("dwFileFlagsMask", &self.dwFileFlagsMask)
            .field("dwFileFlags", &self.dwFileFlags)
            .field("dwFileOS", &self.dwFileOS)
            .field("dwFileType", &self.dwFileType)
            .field("dwFileSubtype", &self.dwFileSubtype)
            .field("dwFileDateMS", &self.dwFileDateMS)
            .field("dwFileDateLS", &self.dwFileDateLS)
            .finish()
    }
}
impl ::windows::core::TypeKind for VS_FIXEDFILEINFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VS_FIXEDFILEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSignature == other.dwSignature && self.dwStrucVersion == other.dwStrucVersion && self.dwFileVersionMS == other.dwFileVersionMS && self.dwFileVersionLS == other.dwFileVersionLS && self.dwProductVersionMS == other.dwProductVersionMS && self.dwProductVersionLS == other.dwProductVersionLS && self.dwFileFlagsMask == other.dwFileFlagsMask && self.dwFileFlags == other.dwFileFlags && self.dwFileOS == other.dwFileOS && self.dwFileType == other.dwFileType && self.dwFileSubtype == other.dwFileSubtype && self.dwFileDateMS == other.dwFileDateMS && self.dwFileDateLS == other.dwFileDateLS
    }
}
impl ::core::cmp::Eq for VS_FIXEDFILEINFO {}
impl ::core::default::Default for VS_FIXEDFILEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct WIM_ENTRY_INFO {
    pub WimEntryInfoSize: u32,
    pub WimType: u32,
    pub DataSourceId: i64,
    pub WimGuid: ::windows::core::GUID,
    pub WimPath: ::windows::core::PCWSTR,
    pub WimIndex: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for WIM_ENTRY_INFO {}
impl ::core::clone::Clone for WIM_ENTRY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIM_ENTRY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIM_ENTRY_INFO").field("WimEntryInfoSize", &self.WimEntryInfoSize).field("WimType", &self.WimType).field("DataSourceId", &self.DataSourceId).field("WimGuid", &self.WimGuid).field("WimPath", &self.WimPath).field("WimIndex", &self.WimIndex).field("Flags", &self.Flags).finish()
    }
}
impl ::windows::core::TypeKind for WIM_ENTRY_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WIM_ENTRY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.WimEntryInfoSize == other.WimEntryInfoSize && self.WimType == other.WimType && self.DataSourceId == other.DataSourceId && self.WimGuid == other.WimGuid && self.WimPath == other.WimPath && self.WimIndex == other.WimIndex && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for WIM_ENTRY_INFO {}
impl ::core::default::Default for WIM_ENTRY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct WIM_EXTERNAL_FILE_INFO {
    pub DataSourceId: i64,
    pub ResourceHash: [u8; 20],
    pub Flags: u32,
}
impl ::core::marker::Copy for WIM_EXTERNAL_FILE_INFO {}
impl ::core::clone::Clone for WIM_EXTERNAL_FILE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIM_EXTERNAL_FILE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIM_EXTERNAL_FILE_INFO").field("DataSourceId", &self.DataSourceId).field("ResourceHash", &self.ResourceHash).field("Flags", &self.Flags).finish()
    }
}
impl ::windows::core::TypeKind for WIM_EXTERNAL_FILE_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WIM_EXTERNAL_FILE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.DataSourceId == other.DataSourceId && self.ResourceHash == other.ResourceHash && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for WIM_EXTERNAL_FILE_INFO {}
impl ::core::default::Default for WIM_EXTERNAL_FILE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WIN32_FILE_ATTRIBUTE_DATA {
    pub dwFileAttributes: u32,
    pub ftCreationTime: super::super::Foundation::FILETIME,
    pub ftLastAccessTime: super::super::Foundation::FILETIME,
    pub ftLastWriteTime: super::super::Foundation::FILETIME,
    pub nFileSizeHigh: u32,
    pub nFileSizeLow: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WIN32_FILE_ATTRIBUTE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WIN32_FILE_ATTRIBUTE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WIN32_FILE_ATTRIBUTE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIN32_FILE_ATTRIBUTE_DATA").field("dwFileAttributes", &self.dwFileAttributes).field("ftCreationTime", &self.ftCreationTime).field("ftLastAccessTime", &self.ftLastAccessTime).field("ftLastWriteTime", &self.ftLastWriteTime).field("nFileSizeHigh", &self.nFileSizeHigh).field("nFileSizeLow", &self.nFileSizeLow).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WIN32_FILE_ATTRIBUTE_DATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WIN32_FILE_ATTRIBUTE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwFileAttributes == other.dwFileAttributes && self.ftCreationTime == other.ftCreationTime && self.ftLastAccessTime == other.ftLastAccessTime && self.ftLastWriteTime == other.ftLastWriteTime && self.nFileSizeHigh == other.nFileSizeHigh && self.nFileSizeLow == other.nFileSizeLow
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WIN32_FILE_ATTRIBUTE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WIN32_FILE_ATTRIBUTE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WIN32_FIND_DATAA {
    pub dwFileAttributes: u32,
    pub ftCreationTime: super::super::Foundation::FILETIME,
    pub ftLastAccessTime: super::super::Foundation::FILETIME,
    pub ftLastWriteTime: super::super::Foundation::FILETIME,
    pub nFileSizeHigh: u32,
    pub nFileSizeLow: u32,
    pub dwReserved0: u32,
    pub dwReserved1: u32,
    pub cFileName: [u8; 260],
    pub cAlternateFileName: [u8; 14],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WIN32_FIND_DATAA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WIN32_FIND_DATAA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WIN32_FIND_DATAA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIN32_FIND_DATAA")
            .field("dwFileAttributes", &self.dwFileAttributes)
            .field("ftCreationTime", &self.ftCreationTime)
            .field("ftLastAccessTime", &self.ftLastAccessTime)
            .field("ftLastWriteTime", &self.ftLastWriteTime)
            .field("nFileSizeHigh", &self.nFileSizeHigh)
            .field("nFileSizeLow", &self.nFileSizeLow)
            .field("dwReserved0", &self.dwReserved0)
            .field("dwReserved1", &self.dwReserved1)
            .field("cFileName", &self.cFileName)
            .field("cAlternateFileName", &self.cAlternateFileName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WIN32_FIND_DATAA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WIN32_FIND_DATAA {
    fn eq(&self, other: &Self) -> bool {
        self.dwFileAttributes == other.dwFileAttributes && self.ftCreationTime == other.ftCreationTime && self.ftLastAccessTime == other.ftLastAccessTime && self.ftLastWriteTime == other.ftLastWriteTime && self.nFileSizeHigh == other.nFileSizeHigh && self.nFileSizeLow == other.nFileSizeLow && self.dwReserved0 == other.dwReserved0 && self.dwReserved1 == other.dwReserved1 && self.cFileName == other.cFileName && self.cAlternateFileName == other.cAlternateFileName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WIN32_FIND_DATAA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WIN32_FIND_DATAA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WIN32_FIND_DATAW {
    pub dwFileAttributes: u32,
    pub ftCreationTime: super::super::Foundation::FILETIME,
    pub ftLastAccessTime: super::super::Foundation::FILETIME,
    pub ftLastWriteTime: super::super::Foundation::FILETIME,
    pub nFileSizeHigh: u32,
    pub nFileSizeLow: u32,
    pub dwReserved0: u32,
    pub dwReserved1: u32,
    pub cFileName: [u16; 260],
    pub cAlternateFileName: [u16; 14],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WIN32_FIND_DATAW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WIN32_FIND_DATAW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WIN32_FIND_DATAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIN32_FIND_DATAW")
            .field("dwFileAttributes", &self.dwFileAttributes)
            .field("ftCreationTime", &self.ftCreationTime)
            .field("ftLastAccessTime", &self.ftLastAccessTime)
            .field("ftLastWriteTime", &self.ftLastWriteTime)
            .field("nFileSizeHigh", &self.nFileSizeHigh)
            .field("nFileSizeLow", &self.nFileSizeLow)
            .field("dwReserved0", &self.dwReserved0)
            .field("dwReserved1", &self.dwReserved1)
            .field("cFileName", &self.cFileName)
            .field("cAlternateFileName", &self.cAlternateFileName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WIN32_FIND_DATAW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WIN32_FIND_DATAW {
    fn eq(&self, other: &Self) -> bool {
        self.dwFileAttributes == other.dwFileAttributes && self.ftCreationTime == other.ftCreationTime && self.ftLastAccessTime == other.ftLastAccessTime && self.ftLastWriteTime == other.ftLastWriteTime && self.nFileSizeHigh == other.nFileSizeHigh && self.nFileSizeLow == other.nFileSizeLow && self.dwReserved0 == other.dwReserved0 && self.dwReserved1 == other.dwReserved1 && self.cFileName == other.cFileName && self.cAlternateFileName == other.cAlternateFileName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WIN32_FIND_DATAW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WIN32_FIND_DATAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct WIN32_FIND_STREAM_DATA {
    pub StreamSize: i64,
    pub cStreamName: [u16; 296],
}
impl ::core::marker::Copy for WIN32_FIND_STREAM_DATA {}
impl ::core::clone::Clone for WIN32_FIND_STREAM_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIN32_FIND_STREAM_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIN32_FIND_STREAM_DATA").field("StreamSize", &self.StreamSize).field("cStreamName", &self.cStreamName).finish()
    }
}
impl ::windows::core::TypeKind for WIN32_FIND_STREAM_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WIN32_FIND_STREAM_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.StreamSize == other.StreamSize && self.cStreamName == other.cStreamName
    }
}
impl ::core::cmp::Eq for WIN32_FIND_STREAM_DATA {}
impl ::core::default::Default for WIN32_FIND_STREAM_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct WIN32_STREAM_ID {
    pub dwStreamId: WIN_STREAM_ID,
    pub dwStreamAttributes: u32,
    pub Size: i64,
    pub dwStreamNameSize: u32,
    pub cStreamName: [u16; 1],
}
impl ::core::marker::Copy for WIN32_STREAM_ID {}
impl ::core::clone::Clone for WIN32_STREAM_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIN32_STREAM_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIN32_STREAM_ID").field("dwStreamId", &self.dwStreamId).field("dwStreamAttributes", &self.dwStreamAttributes).field("Size", &self.Size).field("dwStreamNameSize", &self.dwStreamNameSize).field("cStreamName", &self.cStreamName).finish()
    }
}
impl ::windows::core::TypeKind for WIN32_STREAM_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WIN32_STREAM_ID {
    fn eq(&self, other: &Self) -> bool {
        self.dwStreamId == other.dwStreamId && self.dwStreamAttributes == other.dwStreamAttributes && self.Size == other.Size && self.dwStreamNameSize == other.dwStreamNameSize && self.cStreamName == other.cStreamName
    }
}
impl ::core::cmp::Eq for WIN32_STREAM_ID {}
impl ::core::default::Default for WIN32_STREAM_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct WOF_FILE_COMPRESSION_INFO_V0 {
    pub Algorithm: u32,
}
impl ::core::marker::Copy for WOF_FILE_COMPRESSION_INFO_V0 {}
impl ::core::clone::Clone for WOF_FILE_COMPRESSION_INFO_V0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WOF_FILE_COMPRESSION_INFO_V0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WOF_FILE_COMPRESSION_INFO_V0").field("Algorithm", &self.Algorithm).finish()
    }
}
impl ::windows::core::TypeKind for WOF_FILE_COMPRESSION_INFO_V0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WOF_FILE_COMPRESSION_INFO_V0 {
    fn eq(&self, other: &Self) -> bool {
        self.Algorithm == other.Algorithm
    }
}
impl ::core::cmp::Eq for WOF_FILE_COMPRESSION_INFO_V0 {}
impl ::core::default::Default for WOF_FILE_COMPRESSION_INFO_V0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct WOF_FILE_COMPRESSION_INFO_V1 {
    pub Algorithm: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for WOF_FILE_COMPRESSION_INFO_V1 {}
impl ::core::clone::Clone for WOF_FILE_COMPRESSION_INFO_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WOF_FILE_COMPRESSION_INFO_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WOF_FILE_COMPRESSION_INFO_V1").field("Algorithm", &self.Algorithm).field("Flags", &self.Flags).finish()
    }
}
impl ::windows::core::TypeKind for WOF_FILE_COMPRESSION_INFO_V1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WOF_FILE_COMPRESSION_INFO_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.Algorithm == other.Algorithm && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for WOF_FILE_COMPRESSION_INFO_V1 {}
impl ::core::default::Default for WOF_FILE_COMPRESSION_INFO_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type CACHE_ACCESS_CHECK = ::core::option::Option<unsafe extern "system" fn(psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR, hclienttoken: super::super::Foundation::HANDLE, dwdesiredaccess: u32, genericmapping: *mut super::super::Security::GENERIC_MAPPING, privilegeset: *mut super::super::Security::PRIVILEGE_SET, privilegesetlength: *mut u32, grantedaccess: *mut u32, accessstatus: *mut i32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type CACHE_DESTROY_CALLBACK = ::core::option::Option<unsafe extern "system" fn(cb: u32, lpb: *mut u8) -> ()>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type CACHE_KEY_COMPARE = ::core::option::Option<unsafe extern "system" fn(cbkey1: u32, lpbkey1: *mut u8, cbkey2: u32, lpbkey2: *mut u8) -> i32>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type CACHE_KEY_HASH = ::core::option::Option<unsafe extern "system" fn(lpbkey: *mut u8, cbkey: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type CACHE_READ_CALLBACK = ::core::option::Option<unsafe extern "system" fn(cb: u32, lpb: *mut u8, lpvcontext: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type CLAIMMEDIALABEL = ::core::option::Option<unsafe extern "system" fn(pbuffer: *const u8, nbuffersize: u32, plabelinfo: *mut MediaLabelInfo) -> u32>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type CLAIMMEDIALABELEX = ::core::option::Option<unsafe extern "system" fn(pbuffer: *const u8, nbuffersize: u32, plabelinfo: *mut MediaLabelInfo, labelguid: *mut ::windows::core::GUID) -> u32>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type CLFS_BLOCK_ALLOCATION = ::core::option::Option<unsafe extern "system" fn(cbbufferlength: u32, pvusercontext: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type CLFS_BLOCK_DEALLOCATION = ::core::option::Option<unsafe extern "system" fn(pvbuffer: *mut ::core::ffi::c_void, pvusercontext: *mut ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type FCACHE_CREATE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(lpstrname: ::windows::core::PCSTR, lpvdata: *mut ::core::ffi::c_void, cbfilesize: *mut u32, cbfilesizehigh: *mut u32) -> super::super::Foundation::HANDLE>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type FCACHE_RICHCREATE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(lpstrname: ::windows::core::PCSTR, lpvdata: *mut ::core::ffi::c_void, cbfilesize: *mut u32, cbfilesizehigh: *mut u32, pfdidwescanit: *mut super::super::Foundation::BOOL, pfisstuffed: *mut super::super::Foundation::BOOL, pfstoredwithdots: *mut super::super::Foundation::BOOL, pfstoredwithterminatingdot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::HANDLE>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPPROGRESS_ROUTINE = ::core::option::Option<unsafe extern "system" fn(totalfilesize: i64, totalbytestransferred: i64, streamsize: i64, streambytestransferred: i64, dwstreamnumber: u32, dwcallbackreason: LPPROGRESS_ROUTINE_CALLBACK_REASON, hsourcefile: super::super::Foundation::HANDLE, hdestinationfile: super::super::Foundation::HANDLE, lpdata: *const ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type MAXMEDIALABEL = ::core::option::Option<unsafe extern "system" fn(pmaxsize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type PCLFS_COMPLETION_ROUTINE = ::core::option::Option<unsafe extern "system" fn(pvoverlapped: *mut ::core::ffi::c_void, ulreserved: u32) -> ()>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCOPYFILE2_PROGRESS_ROUTINE = ::core::option::Option<unsafe extern "system" fn(pmessage: *const COPYFILE2_MESSAGE, pvcallbackcontext: *const ::core::ffi::c_void) -> COPYFILE2_MESSAGE_ACTION>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type PFE_EXPORT_FUNC = ::core::option::Option<unsafe extern "system" fn(pbdata: *const u8, pvcallbackcontext: *const ::core::ffi::c_void, ullength: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type PFE_IMPORT_FUNC = ::core::option::Option<unsafe extern "system" fn(pbdata: *mut u8, pvcallbackcontext: *const ::core::ffi::c_void, ullength: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_IO_COMPLETION = ::core::option::Option<unsafe extern "system" fn(pcontext: *mut FIO_CONTEXT, lpo: *mut FH_OVERLAPPED, cb: u32, dwcompletionstatus: u32) -> ()>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLOG_FULL_HANDLER_CALLBACK = ::core::option::Option<unsafe extern "system" fn(hlogfile: super::super::Foundation::HANDLE, dwerror: u32, flogispinned: super::super::Foundation::BOOL, pvclientcontext: *mut ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLOG_TAIL_ADVANCE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(hlogfile: super::super::Foundation::HANDLE, lsntarget: CLS_LSN, pvclientcontext: *mut ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLOG_UNPINNED_CALLBACK = ::core::option::Option<unsafe extern "system" fn(hlogfile: super::super::Foundation::HANDLE, pvclientcontext: *mut ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type WofEnumEntryProc = ::core::option::Option<unsafe extern "system" fn(entryinfo: *const ::core::ffi::c_void, userdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type WofEnumFilesProc = ::core::option::Option<unsafe extern "system" fn(filepath: ::windows::core::PCWSTR, externalfileinfo: *const ::core::ffi::c_void, userdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
