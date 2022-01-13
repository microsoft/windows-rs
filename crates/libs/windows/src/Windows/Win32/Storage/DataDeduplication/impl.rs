#[cfg(feature = "Win32_Foundation")]
pub trait IDedupBackupSupportImpl: Sized {
    fn RestoreFiles(&mut self, numberoffiles: u32, filefullpaths: *const super::super::Foundation::BSTR, store: ::core::option::Option<IDedupReadFileCallback>, flags: u32, fileresults: *mut ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDedupBackupSupportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDedupBackupSupportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDedupBackupSupportVtbl {
        unsafe extern "system" fn RestoreFiles<Impl: IDedupBackupSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numberoffiles: u32, filefullpaths: *const super::super::Foundation::BSTR, store: ::windows::core::RawPtr, flags: u32, fileresults: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RestoreFiles(::core::mem::transmute_copy(&numberoffiles), ::core::mem::transmute_copy(&filefullpaths), ::core::mem::transmute(&store), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&fileresults)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), RestoreFiles: RestoreFiles::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDedupBackupSupport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDedupChunkLibraryImpl: Sized {
    fn InitializeForPushBuffers(&mut self) -> ::windows::core::Result<()>;
    fn Uninitialize(&mut self) -> ::windows::core::Result<()>;
    fn SetParameter(&mut self, dwparamtype: u32, vparamvalue: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn StartChunking(&mut self, iiditeratorinterfaceid: ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDedupChunkLibraryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDedupChunkLibraryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDedupChunkLibraryVtbl {
        unsafe extern "system" fn InitializeForPushBuffers<Impl: IDedupChunkLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeForPushBuffers().into()
        }
        unsafe extern "system" fn Uninitialize<Impl: IDedupChunkLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Uninitialize().into()
        }
        unsafe extern "system" fn SetParameter<Impl: IDedupChunkLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwparamtype: u32, vparamvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetParameter(::core::mem::transmute_copy(&dwparamtype), ::core::mem::transmute_copy(&vparamvalue)).into()
        }
        unsafe extern "system" fn StartChunking<Impl: IDedupChunkLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iiditeratorinterfaceid: ::windows::core::GUID, ppchunksenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartChunking(::core::mem::transmute_copy(&iiditeratorinterfaceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppchunksenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            InitializeForPushBuffers: InitializeForPushBuffers::<Impl, IMPL_OFFSET>,
            Uninitialize: Uninitialize::<Impl, IMPL_OFFSET>,
            SetParameter: SetParameter::<Impl, IMPL_OFFSET>,
            StartChunking: StartChunking::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDedupChunkLibrary as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDedupDataPortImpl: Sized {
    fn GetStatus(&mut self, pstatus: *mut DedupDataPortVolumeStatus, pdataheadroommb: *mut u32) -> ::windows::core::Result<()>;
    fn LookupChunks(&mut self, count: u32, phashes: *const DedupHash) -> ::windows::core::Result<::windows::core::GUID>;
    fn InsertChunks(&mut self, chunkcount: u32, pchunkmetadata: *const DedupChunk, databytecount: u32, pchunkdata: *const u8) -> ::windows::core::Result<::windows::core::GUID>;
    fn InsertChunksWithStream(&mut self, chunkcount: u32, pchunkmetadata: *const DedupChunk, databytecount: u32, pchunkdatastream: ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<::windows::core::GUID>;
    fn CommitStreams(&mut self, streamcount: u32, pstreams: *const DedupStream, entrycount: u32, pentries: *const DedupStreamEntry) -> ::windows::core::Result<::windows::core::GUID>;
    fn CommitStreamsWithStream(&mut self, streamcount: u32, pstreams: *const DedupStream, entrycount: u32, pentriesstream: ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetStreams(&mut self, streamcount: u32, pstreampaths: *const super::super::Foundation::BSTR) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetStreamsResults(&mut self, requestid: ::windows::core::GUID, maxwaitms: u32, streamentryindex: u32, pstreamcount: *mut u32, ppstreams: *mut *mut DedupStream, pentrycount: *mut u32, ppentries: *mut *mut DedupStreamEntry, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn GetChunks(&mut self, count: u32, phashes: *const DedupHash) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetChunksResults(&mut self, requestid: ::windows::core::GUID, maxwaitms: u32, chunkindex: u32, pchunkcount: *mut u32, ppchunkmetadata: *mut *mut DedupChunk, pdatabytecount: *mut u32, ppchunkdata: *mut *mut u8, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn GetRequestStatus(&mut self, requestid: ::windows::core::GUID) -> ::windows::core::Result<DedupDataPortRequestStatus>;
    fn GetRequestResults(&mut self, requestid: ::windows::core::GUID, maxwaitms: u32, pbatchresult: *mut ::windows::core::HRESULT, pbatchcount: *mut u32, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IDedupDataPortVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDedupDataPortImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDedupDataPortVtbl {
        unsafe extern "system" fn GetStatus<Impl: IDedupDataPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut DedupDataPortVolumeStatus, pdataheadroommb: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStatus(::core::mem::transmute_copy(&pstatus), ::core::mem::transmute_copy(&pdataheadroommb)).into()
        }
        unsafe extern "system" fn LookupChunks<Impl: IDedupDataPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, phashes: *const DedupHash, prequestid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupChunks(::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&phashes)) {
                ::core::result::Result::Ok(ok__) => {
                    *prequestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertChunks<Impl: IDedupDataPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, chunkcount: u32, pchunkmetadata: *const DedupChunk, databytecount: u32, pchunkdata: *const u8, prequestid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsertChunks(::core::mem::transmute_copy(&chunkcount), ::core::mem::transmute_copy(&pchunkmetadata), ::core::mem::transmute_copy(&databytecount), ::core::mem::transmute_copy(&pchunkdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *prequestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertChunksWithStream<Impl: IDedupDataPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, chunkcount: u32, pchunkmetadata: *const DedupChunk, databytecount: u32, pchunkdatastream: ::windows::core::RawPtr, prequestid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsertChunksWithStream(::core::mem::transmute_copy(&chunkcount), ::core::mem::transmute_copy(&pchunkmetadata), ::core::mem::transmute_copy(&databytecount), ::core::mem::transmute(&pchunkdatastream)) {
                ::core::result::Result::Ok(ok__) => {
                    *prequestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitStreams<Impl: IDedupDataPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamcount: u32, pstreams: *const DedupStream, entrycount: u32, pentries: *const DedupStreamEntry, prequestid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommitStreams(::core::mem::transmute_copy(&streamcount), ::core::mem::transmute_copy(&pstreams), ::core::mem::transmute_copy(&entrycount), ::core::mem::transmute_copy(&pentries)) {
                ::core::result::Result::Ok(ok__) => {
                    *prequestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitStreamsWithStream<Impl: IDedupDataPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamcount: u32, pstreams: *const DedupStream, entrycount: u32, pentriesstream: ::windows::core::RawPtr, prequestid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommitStreamsWithStream(::core::mem::transmute_copy(&streamcount), ::core::mem::transmute_copy(&pstreams), ::core::mem::transmute_copy(&entrycount), ::core::mem::transmute(&pentriesstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *prequestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreams<Impl: IDedupDataPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamcount: u32, pstreampaths: *const super::super::Foundation::BSTR, prequestid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStreams(::core::mem::transmute_copy(&streamcount), ::core::mem::transmute_copy(&pstreampaths)) {
                ::core::result::Result::Ok(ok__) => {
                    *prequestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamsResults<Impl: IDedupDataPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: ::windows::core::GUID, maxwaitms: u32, streamentryindex: u32, pstreamcount: *mut u32, ppstreams: *mut *mut DedupStream, pentrycount: *mut u32, ppentries: *mut *mut DedupStreamEntry, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStreamsResults(::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&maxwaitms), ::core::mem::transmute_copy(&streamentryindex), ::core::mem::transmute_copy(&pstreamcount), ::core::mem::transmute_copy(&ppstreams), ::core::mem::transmute_copy(&pentrycount), ::core::mem::transmute_copy(&ppentries), ::core::mem::transmute_copy(&pstatus), ::core::mem::transmute_copy(&ppitemresults)).into()
        }
        unsafe extern "system" fn GetChunks<Impl: IDedupDataPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, phashes: *const DedupHash, prequestid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChunks(::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&phashes)) {
                ::core::result::Result::Ok(ok__) => {
                    *prequestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChunksResults<Impl: IDedupDataPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: ::windows::core::GUID, maxwaitms: u32, chunkindex: u32, pchunkcount: *mut u32, ppchunkmetadata: *mut *mut DedupChunk, pdatabytecount: *mut u32, ppchunkdata: *mut *mut u8, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetChunksResults(::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&maxwaitms), ::core::mem::transmute_copy(&chunkindex), ::core::mem::transmute_copy(&pchunkcount), ::core::mem::transmute_copy(&ppchunkmetadata), ::core::mem::transmute_copy(&pdatabytecount), ::core::mem::transmute_copy(&ppchunkdata), ::core::mem::transmute_copy(&pstatus), ::core::mem::transmute_copy(&ppitemresults)).into()
        }
        unsafe extern "system" fn GetRequestStatus<Impl: IDedupDataPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: ::windows::core::GUID, pstatus: *mut DedupDataPortRequestStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRequestStatus(::core::mem::transmute_copy(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRequestResults<Impl: IDedupDataPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: ::windows::core::GUID, maxwaitms: u32, pbatchresult: *mut ::windows::core::HRESULT, pbatchcount: *mut u32, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRequestResults(::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&maxwaitms), ::core::mem::transmute_copy(&pbatchresult), ::core::mem::transmute_copy(&pbatchcount), ::core::mem::transmute_copy(&pstatus), ::core::mem::transmute_copy(&ppitemresults)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            LookupChunks: LookupChunks::<Impl, IMPL_OFFSET>,
            InsertChunks: InsertChunks::<Impl, IMPL_OFFSET>,
            InsertChunksWithStream: InsertChunksWithStream::<Impl, IMPL_OFFSET>,
            CommitStreams: CommitStreams::<Impl, IMPL_OFFSET>,
            CommitStreamsWithStream: CommitStreamsWithStream::<Impl, IMPL_OFFSET>,
            GetStreams: GetStreams::<Impl, IMPL_OFFSET>,
            GetStreamsResults: GetStreamsResults::<Impl, IMPL_OFFSET>,
            GetChunks: GetChunks::<Impl, IMPL_OFFSET>,
            GetChunksResults: GetChunksResults::<Impl, IMPL_OFFSET>,
            GetRequestStatus: GetRequestStatus::<Impl, IMPL_OFFSET>,
            GetRequestResults: GetRequestResults::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDedupDataPort as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDedupDataPortManagerImpl: Sized {
    fn GetConfiguration(&mut self, pminchunksize: *mut u32, pmaxchunksize: *mut u32, pchunkingalgorithm: *mut DedupChunkingAlgorithm, phashingalgorithm: *mut DedupHashingAlgorithm, pcompressionalgorithm: *mut DedupCompressionAlgorithm) -> ::windows::core::Result<()>;
    fn GetVolumeStatus(&mut self, options: u32, path: super::super::Foundation::BSTR) -> ::windows::core::Result<DedupDataPortVolumeStatus>;
    fn GetVolumeDataPort(&mut self, options: u32, path: super::super::Foundation::BSTR) -> ::windows::core::Result<IDedupDataPort>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDedupDataPortManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDedupDataPortManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDedupDataPortManagerVtbl {
        unsafe extern "system" fn GetConfiguration<Impl: IDedupDataPortManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pminchunksize: *mut u32, pmaxchunksize: *mut u32, pchunkingalgorithm: *mut DedupChunkingAlgorithm, phashingalgorithm: *mut DedupHashingAlgorithm, pcompressionalgorithm: *mut DedupCompressionAlgorithm) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConfiguration(::core::mem::transmute_copy(&pminchunksize), ::core::mem::transmute_copy(&pmaxchunksize), ::core::mem::transmute_copy(&pchunkingalgorithm), ::core::mem::transmute_copy(&phashingalgorithm), ::core::mem::transmute_copy(&pcompressionalgorithm)).into()
        }
        unsafe extern "system" fn GetVolumeStatus<Impl: IDedupDataPortManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: u32, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pstatus: *mut DedupDataPortVolumeStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVolumeStatus(::core::mem::transmute_copy(&options), ::core::mem::transmute_copy(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVolumeDataPort<Impl: IDedupDataPortManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: u32, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdataport: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVolumeDataPort(::core::mem::transmute_copy(&options), ::core::mem::transmute_copy(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdataport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetConfiguration: GetConfiguration::<Impl, IMPL_OFFSET>,
            GetVolumeStatus: GetVolumeStatus::<Impl, IMPL_OFFSET>,
            GetVolumeDataPort: GetVolumeDataPort::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDedupDataPortManager as ::windows::core::Interface>::IID
    }
}
pub trait IDedupIterateChunksHash32Impl: Sized {
    fn PushBuffer(&mut self, pbuffer: *const u8, ulbufferlength: u32) -> ::windows::core::Result<()>;
    fn Next(&mut self, ulmaxchunks: u32, parrchunks: *mut DEDUP_CHUNK_INFO_HASH32, pulfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Drain(&mut self) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
}
impl IDedupIterateChunksHash32Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDedupIterateChunksHash32Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDedupIterateChunksHash32Vtbl {
        unsafe extern "system" fn PushBuffer<Impl: IDedupIterateChunksHash32Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: *const u8, ulbufferlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PushBuffer(::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&ulbufferlength)).into()
        }
        unsafe extern "system" fn Next<Impl: IDedupIterateChunksHash32Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulmaxchunks: u32, parrchunks: *mut DEDUP_CHUNK_INFO_HASH32, pulfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ulmaxchunks), ::core::mem::transmute_copy(&parrchunks), ::core::mem::transmute_copy(&pulfetched)).into()
        }
        unsafe extern "system" fn Drain<Impl: IDedupIterateChunksHash32Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Drain().into()
        }
        unsafe extern "system" fn Reset<Impl: IDedupIterateChunksHash32Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            PushBuffer: PushBuffer::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
            Drain: Drain::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDedupIterateChunksHash32 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDedupReadFileCallbackImpl: Sized {
    fn ReadBackupFile(&mut self, filefullpath: super::super::Foundation::BSTR, fileoffset: i64, sizetoread: u32, filebuffer: *mut u8, returnedsize: *mut u32, flags: u32) -> ::windows::core::Result<()>;
    fn OrderContainersRestore(&mut self, numberofcontainers: u32, containerpaths: *const super::super::Foundation::BSTR, readplanentries: *mut u32, readplan: *mut *mut DEDUP_CONTAINER_EXTENT) -> ::windows::core::Result<()>;
    fn PreviewContainerRead(&mut self, filefullpath: super::super::Foundation::BSTR, numberofreads: u32, readoffsets: *const DDP_FILE_EXTENT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDedupReadFileCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDedupReadFileCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDedupReadFileCallbackVtbl {
        unsafe extern "system" fn ReadBackupFile<Impl: IDedupReadFileCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filefullpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fileoffset: i64, sizetoread: u32, filebuffer: *mut u8, returnedsize: *mut u32, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReadBackupFile(::core::mem::transmute_copy(&filefullpath), ::core::mem::transmute_copy(&fileoffset), ::core::mem::transmute_copy(&sizetoread), ::core::mem::transmute_copy(&filebuffer), ::core::mem::transmute_copy(&returnedsize), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn OrderContainersRestore<Impl: IDedupReadFileCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numberofcontainers: u32, containerpaths: *const super::super::Foundation::BSTR, readplanentries: *mut u32, readplan: *mut *mut DEDUP_CONTAINER_EXTENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OrderContainersRestore(::core::mem::transmute_copy(&numberofcontainers), ::core::mem::transmute_copy(&containerpaths), ::core::mem::transmute_copy(&readplanentries), ::core::mem::transmute_copy(&readplan)).into()
        }
        unsafe extern "system" fn PreviewContainerRead<Impl: IDedupReadFileCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filefullpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, numberofreads: u32, readoffsets: *const DDP_FILE_EXTENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PreviewContainerRead(::core::mem::transmute_copy(&filefullpath), ::core::mem::transmute_copy(&numberofreads), ::core::mem::transmute_copy(&readoffsets)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ReadBackupFile: ReadBackupFile::<Impl, IMPL_OFFSET>,
            OrderContainersRestore: OrderContainersRestore::<Impl, IMPL_OFFSET>,
            PreviewContainerRead: PreviewContainerRead::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDedupReadFileCallback as ::windows::core::Interface>::IID
    }
}
