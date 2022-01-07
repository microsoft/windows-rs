pub trait IDedupBackupSupportImpl: Sized {
    fn RestoreFiles();
}
impl ::windows::core::RuntimeName for IDedupBackupSupport {
    const NAME: &'static str = "Windows.Win32.Storage.DataDeduplication.IDedupBackupSupport";
}
impl IDedupBackupSupportVtbl {
    pub const fn new<Impl: IDedupBackupSupportImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDedupBackupSupportVtbl {
        unsafe extern "system" fn RestoreFiles<Impl: IDedupBackupSupportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numberoffiles: u32, filefullpaths: *const super::super::Foundation::BSTR, store: ::windows::core::RawPtr, flags: u32, fileresults: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RestoreFiles(numberoffiles, &*(&filefullpaths as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&store as *const <IDedupReadFileCallback as ::windows::core::Abi>::Abi as *const <IDedupReadFileCallback as ::windows::core::DefaultType>::DefaultType), flags, ::core::mem::transmute_copy(&fileresults)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDedupBackupSupport>, base.5, RestoreFiles::<Impl, OFFSET>)
    }
}
pub trait IDedupChunkLibraryImpl: Sized {
    fn InitializeForPushBuffers();
    fn Uninitialize();
    fn SetParameter();
    fn StartChunking();
}
impl ::windows::core::RuntimeName for IDedupChunkLibrary {
    const NAME: &'static str = "Windows.Win32.Storage.DataDeduplication.IDedupChunkLibrary";
}
impl IDedupChunkLibraryVtbl {
    pub const fn new<Impl: IDedupChunkLibraryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDedupChunkLibraryVtbl {
        unsafe extern "system" fn InitializeForPushBuffers<Impl: IDedupChunkLibraryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InitializeForPushBuffers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uninitialize<Impl: IDedupChunkLibraryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Uninitialize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParameter<Impl: IDedupChunkLibraryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwparamtype: u32, vparamvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetParameter(dwparamtype, &*(&vparamvalue as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartChunking<Impl: IDedupChunkLibraryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iiditeratorinterfaceid: ::windows::core::GUID, ppchunksenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartChunking(&*(&iiditeratorinterfaceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppchunksenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDedupChunkLibrary>, base.5, InitializeForPushBuffers::<Impl, OFFSET>, Uninitialize::<Impl, OFFSET>, SetParameter::<Impl, OFFSET>, StartChunking::<Impl, OFFSET>)
    }
}
pub trait IDedupDataPortImpl: Sized {
    fn GetStatus();
    fn LookupChunks();
    fn InsertChunks();
    fn InsertChunksWithStream();
    fn CommitStreams();
    fn CommitStreamsWithStream();
    fn GetStreams();
    fn GetStreamsResults();
    fn GetChunks();
    fn GetChunksResults();
    fn GetRequestStatus();
    fn GetRequestResults();
}
impl ::windows::core::RuntimeName for IDedupDataPort {
    const NAME: &'static str = "Windows.Win32.Storage.DataDeduplication.IDedupDataPort";
}
impl IDedupDataPortVtbl {
    pub const fn new<Impl: IDedupDataPortImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDedupDataPortVtbl {
        unsafe extern "system" fn GetStatus<Impl: IDedupDataPortImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatus: *mut DedupDataPortVolumeStatus, pdataheadroommb: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStatus(::core::mem::transmute_copy(&pstatus), ::core::mem::transmute_copy(&pdataheadroommb)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupChunks<Impl: IDedupDataPortImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: u32, phashes: *const DedupHash, prequestid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LookupChunks(count, &*(&phashes as *const <DedupHash as ::windows::core::Abi>::Abi as *const <DedupHash as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&prequestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertChunks<Impl: IDedupDataPortImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, chunkcount: u32, pchunkmetadata: *const DedupChunk, databytecount: u32, pchunkdata: *const u8, prequestid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InsertChunks(chunkcount, &*(&pchunkmetadata as *const <DedupChunk as ::windows::core::Abi>::Abi as *const <DedupChunk as ::windows::core::DefaultType>::DefaultType), databytecount, pchunkdata, ::core::mem::transmute_copy(&prequestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertChunksWithStream<Impl: IDedupDataPortImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, chunkcount: u32, pchunkmetadata: *const DedupChunk, databytecount: u32, pchunkdatastream: ::windows::core::RawPtr, prequestid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InsertChunksWithStream(chunkcount, &*(&pchunkmetadata as *const <DedupChunk as ::windows::core::Abi>::Abi as *const <DedupChunk as ::windows::core::DefaultType>::DefaultType), databytecount, &*(&pchunkdatastream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&prequestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitStreams<Impl: IDedupDataPortImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, streamcount: u32, pstreams: *const DedupStream, entrycount: u32, pentries: *const DedupStreamEntry, prequestid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CommitStreams(streamcount, &*(&pstreams as *const <DedupStream as ::windows::core::Abi>::Abi as *const <DedupStream as ::windows::core::DefaultType>::DefaultType), entrycount, &*(&pentries as *const <DedupStreamEntry as ::windows::core::Abi>::Abi as *const <DedupStreamEntry as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&prequestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitStreamsWithStream<Impl: IDedupDataPortImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, streamcount: u32, pstreams: *const DedupStream, entrycount: u32, pentriesstream: ::windows::core::RawPtr, prequestid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CommitStreamsWithStream(streamcount, &*(&pstreams as *const <DedupStream as ::windows::core::Abi>::Abi as *const <DedupStream as ::windows::core::DefaultType>::DefaultType), entrycount, &*(&pentriesstream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&prequestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreams<Impl: IDedupDataPortImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, streamcount: u32, pstreampaths: *const super::super::Foundation::BSTR, prequestid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStreams(streamcount, &*(&pstreampaths as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&prequestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamsResults<Impl: IDedupDataPortImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestid: ::windows::core::GUID, maxwaitms: u32, streamentryindex: u32, pstreamcount: *mut u32, ppstreams: *mut *mut DedupStream, pentrycount: *mut u32, ppentries: *mut *mut DedupStreamEntry, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStreamsResults(&*(&requestid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), maxwaitms, streamentryindex, ::core::mem::transmute_copy(&pstreamcount), ::core::mem::transmute_copy(&ppstreams), ::core::mem::transmute_copy(&pentrycount), ::core::mem::transmute_copy(&ppentries), ::core::mem::transmute_copy(&pstatus), ::core::mem::transmute_copy(&ppitemresults)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChunks<Impl: IDedupDataPortImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: u32, phashes: *const DedupHash, prequestid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetChunks(count, &*(&phashes as *const <DedupHash as ::windows::core::Abi>::Abi as *const <DedupHash as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&prequestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChunksResults<Impl: IDedupDataPortImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestid: ::windows::core::GUID, maxwaitms: u32, chunkindex: u32, pchunkcount: *mut u32, ppchunkmetadata: *mut *mut DedupChunk, pdatabytecount: *mut u32, ppchunkdata: *mut *mut u8, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetChunksResults(&*(&requestid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), maxwaitms, chunkindex, ::core::mem::transmute_copy(&pchunkcount), ::core::mem::transmute_copy(&ppchunkmetadata), ::core::mem::transmute_copy(&pdatabytecount), ::core::mem::transmute_copy(&ppchunkdata), ::core::mem::transmute_copy(&pstatus), ::core::mem::transmute_copy(&ppitemresults)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRequestStatus<Impl: IDedupDataPortImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestid: ::windows::core::GUID, pstatus: *mut DedupDataPortRequestStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRequestStatus(&*(&requestid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRequestResults<Impl: IDedupDataPortImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestid: ::windows::core::GUID, maxwaitms: u32, pbatchresult: *mut ::windows::core::HRESULT, pbatchcount: *mut u32, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRequestResults(&*(&requestid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), maxwaitms, ::core::mem::transmute_copy(&pbatchresult), ::core::mem::transmute_copy(&pbatchcount), ::core::mem::transmute_copy(&pstatus), ::core::mem::transmute_copy(&ppitemresults)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IDedupDataPort>,
            base.5,
            GetStatus::<Impl, OFFSET>,
            LookupChunks::<Impl, OFFSET>,
            InsertChunks::<Impl, OFFSET>,
            InsertChunksWithStream::<Impl, OFFSET>,
            CommitStreams::<Impl, OFFSET>,
            CommitStreamsWithStream::<Impl, OFFSET>,
            GetStreams::<Impl, OFFSET>,
            GetStreamsResults::<Impl, OFFSET>,
            GetChunks::<Impl, OFFSET>,
            GetChunksResults::<Impl, OFFSET>,
            GetRequestStatus::<Impl, OFFSET>,
            GetRequestResults::<Impl, OFFSET>,
        )
    }
}
pub trait IDedupDataPortManagerImpl: Sized {
    fn GetConfiguration();
    fn GetVolumeStatus();
    fn GetVolumeDataPort();
}
impl ::windows::core::RuntimeName for IDedupDataPortManager {
    const NAME: &'static str = "Windows.Win32.Storage.DataDeduplication.IDedupDataPortManager";
}
impl IDedupDataPortManagerVtbl {
    pub const fn new<Impl: IDedupDataPortManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDedupDataPortManagerVtbl {
        unsafe extern "system" fn GetConfiguration<Impl: IDedupDataPortManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pminchunksize: *mut u32, pmaxchunksize: *mut u32, pchunkingalgorithm: *mut DedupChunkingAlgorithm, phashingalgorithm: *mut DedupHashingAlgorithm, pcompressionalgorithm: *mut DedupCompressionAlgorithm) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetConfiguration(::core::mem::transmute_copy(&pminchunksize), ::core::mem::transmute_copy(&pmaxchunksize), ::core::mem::transmute_copy(&pchunkingalgorithm), ::core::mem::transmute_copy(&phashingalgorithm), ::core::mem::transmute_copy(&pcompressionalgorithm)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVolumeStatus<Impl: IDedupDataPortManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: u32, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pstatus: *mut DedupDataPortVolumeStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVolumeStatus(options, &*(&path as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVolumeDataPort<Impl: IDedupDataPortManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: u32, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdataport: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVolumeDataPort(options, &*(&path as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdataport)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDedupDataPortManager>, base.5, GetConfiguration::<Impl, OFFSET>, GetVolumeStatus::<Impl, OFFSET>, GetVolumeDataPort::<Impl, OFFSET>)
    }
}
pub trait IDedupIterateChunksHash32Impl: Sized {
    fn PushBuffer();
    fn Next();
    fn Drain();
    fn Reset();
}
impl ::windows::core::RuntimeName for IDedupIterateChunksHash32 {
    const NAME: &'static str = "Windows.Win32.Storage.DataDeduplication.IDedupIterateChunksHash32";
}
impl IDedupIterateChunksHash32Vtbl {
    pub const fn new<Impl: IDedupIterateChunksHash32Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDedupIterateChunksHash32Vtbl {
        unsafe extern "system" fn PushBuffer<Impl: IDedupIterateChunksHash32Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbuffer: *const u8, ulbufferlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PushBuffer(pbuffer, ulbufferlength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IDedupIterateChunksHash32Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulmaxchunks: u32, parrchunks: *mut DEDUP_CHUNK_INFO_HASH32, pulfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(ulmaxchunks, ::core::mem::transmute_copy(&parrchunks), ::core::mem::transmute_copy(&pulfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Drain<Impl: IDedupIterateChunksHash32Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Drain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IDedupIterateChunksHash32Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDedupIterateChunksHash32>, base.5, PushBuffer::<Impl, OFFSET>, Next::<Impl, OFFSET>, Drain::<Impl, OFFSET>, Reset::<Impl, OFFSET>)
    }
}
pub trait IDedupReadFileCallbackImpl: Sized {
    fn ReadBackupFile();
    fn OrderContainersRestore();
    fn PreviewContainerRead();
}
impl ::windows::core::RuntimeName for IDedupReadFileCallback {
    const NAME: &'static str = "Windows.Win32.Storage.DataDeduplication.IDedupReadFileCallback";
}
impl IDedupReadFileCallbackVtbl {
    pub const fn new<Impl: IDedupReadFileCallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDedupReadFileCallbackVtbl {
        unsafe extern "system" fn ReadBackupFile<Impl: IDedupReadFileCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filefullpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fileoffset: i64, sizetoread: u32, filebuffer: *mut u8, returnedsize: *mut u32, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadBackupFile(&*(&filefullpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), fileoffset, sizetoread, ::core::mem::transmute_copy(&filebuffer), ::core::mem::transmute_copy(&returnedsize), flags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OrderContainersRestore<Impl: IDedupReadFileCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numberofcontainers: u32, containerpaths: *const super::super::Foundation::BSTR, readplanentries: *mut u32, readplan: *mut *mut DEDUP_CONTAINER_EXTENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OrderContainersRestore(numberofcontainers, &*(&containerpaths as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&readplanentries), ::core::mem::transmute_copy(&readplan)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreviewContainerRead<Impl: IDedupReadFileCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filefullpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, numberofreads: u32, readoffsets: *const DDP_FILE_EXTENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PreviewContainerRead(&*(&filefullpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), numberofreads, &*(&readoffsets as *const <DDP_FILE_EXTENT as ::windows::core::Abi>::Abi as *const <DDP_FILE_EXTENT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDedupReadFileCallback>, base.5, ReadBackupFile::<Impl, OFFSET>, OrderContainersRestore::<Impl, OFFSET>, PreviewContainerRead::<Impl, OFFSET>)
    }
}
