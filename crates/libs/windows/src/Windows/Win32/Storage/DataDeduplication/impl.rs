#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`, `\"implement\"`*"]
pub trait IDedupBackupSupport_Impl: Sized {
    fn RestoreFiles(&self, numberoffiles: u32, filefullpaths: *const ::windows::core::BSTR, store: ::core::option::Option<&IDedupReadFileCallback>, flags: u32, fileresults: *mut ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDedupBackupSupport {}
impl IDedupBackupSupport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDedupBackupSupport_Impl, const OFFSET: isize>() -> IDedupBackupSupport_Vtbl {
        unsafe extern "system" fn RestoreFiles<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDedupBackupSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numberoffiles: u32, filefullpaths: *const ::std::mem::MaybeUninit<::windows::core::BSTR>, store: *mut ::core::ffi::c_void, flags: u32, fileresults: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RestoreFiles(::core::mem::transmute_copy(&numberoffiles), ::core::mem::transmute_copy(&filefullpaths), ::windows::core::from_raw_borrowed(&store), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&fileresults)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), RestoreFiles: RestoreFiles::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDedupBackupSupport as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDedupChunkLibrary_Impl: Sized {
    fn InitializeForPushBuffers(&self) -> ::windows::core::Result<()>;
    fn Uninitialize(&self) -> ::windows::core::Result<()>;
    fn SetParameter(&self, dwparamtype: u32, vparamvalue: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn StartChunking(&self, iiditeratorinterfaceid: &::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IDedupChunkLibrary {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDedupChunkLibrary_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDedupChunkLibrary_Impl, const OFFSET: isize>() -> IDedupChunkLibrary_Vtbl {
        unsafe extern "system" fn InitializeForPushBuffers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDedupChunkLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializeForPushBuffers().into()
        }
        unsafe extern "system" fn Uninitialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDedupChunkLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Uninitialize().into()
        }
        unsafe extern "system" fn SetParameter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDedupChunkLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwparamtype: u32, vparamvalue: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetParameter(::core::mem::transmute_copy(&dwparamtype), ::core::mem::transmute(&vparamvalue)).into()
        }
        unsafe extern "system" fn StartChunking<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDedupChunkLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iiditeratorinterfaceid: ::windows::core::GUID, ppchunksenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StartChunking(::core::mem::transmute(&iiditeratorinterfaceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppchunksenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitializeForPushBuffers: InitializeForPushBuffers::<Identity, Impl, OFFSET>,
            Uninitialize: Uninitialize::<Identity, Impl, OFFSET>,
            SetParameter: SetParameter::<Identity, Impl, OFFSET>,
            StartChunking: StartChunking::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDedupChunkLibrary as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IDedupDataPort_Impl: Sized {
    fn GetStatus(&self, pstatus: *mut DedupDataPortVolumeStatus, pdataheadroommb: *mut u32) -> ::windows::core::Result<()>;
    fn LookupChunks(&self, count: u32, phashes: *const DedupHash) -> ::windows::core::Result<::windows::core::GUID>;
    fn InsertChunks(&self, chunkcount: u32, pchunkmetadata: *const DedupChunk, databytecount: u32, pchunkdata: *const u8) -> ::windows::core::Result<::windows::core::GUID>;
    fn InsertChunksWithStream(&self, chunkcount: u32, pchunkmetadata: *const DedupChunk, databytecount: u32, pchunkdatastream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows::core::Result<::windows::core::GUID>;
    fn CommitStreams(&self, streamcount: u32, pstreams: *const DedupStream, entrycount: u32, pentries: *const DedupStreamEntry) -> ::windows::core::Result<::windows::core::GUID>;
    fn CommitStreamsWithStream(&self, streamcount: u32, pstreams: *const DedupStream, entrycount: u32, pentriesstream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetStreams(&self, streamcount: u32, pstreampaths: *const ::windows::core::BSTR) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetStreamsResults(&self, requestid: &::windows::core::GUID, maxwaitms: u32, streamentryindex: u32, pstreamcount: *mut u32, ppstreams: *mut *mut DedupStream, pentrycount: *mut u32, ppentries: *mut *mut DedupStreamEntry, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn GetChunks(&self, count: u32, phashes: *const DedupHash) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetChunksResults(&self, requestid: &::windows::core::GUID, maxwaitms: u32, chunkindex: u32, pchunkcount: *mut u32, ppchunkmetadata: *mut *mut DedupChunk, pdatabytecount: *mut u32, ppchunkdata: *mut *mut u8, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn GetRequestStatus(&self, requestid: &::windows::core::GUID) -> ::windows::core::Result<DedupDataPortRequestStatus>;
    fn GetRequestResults(&self, requestid: &::windows::core::GUID, maxwaitms: u32, pbatchresult: *mut ::windows::core::HRESULT, pbatchcount: *mut u32, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IDedupDataPort {}
#[cfg(feature = "Win32_System_Com")]
impl IDedupDataPort_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDedupDataPort_Impl, const OFFSET: isize>() -> IDedupDataPort_Vtbl {
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDedupDataPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut DedupDataPortVolumeStatus, pdataheadroommb: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStatus(::core::mem::transmute_copy(&pstatus), ::core::mem::transmute_copy(&pdataheadroommb)).into()
        }
        unsafe extern "system" fn LookupChunks<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDedupDataPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, phashes: *const DedupHash, prequestid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LookupChunks(::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&phashes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prequestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertChunks<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDedupDataPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, chunkcount: u32, pchunkmetadata: *const DedupChunk, databytecount: u32, pchunkdata: *const u8, prequestid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InsertChunks(::core::mem::transmute_copy(&chunkcount), ::core::mem::transmute_copy(&pchunkmetadata), ::core::mem::transmute_copy(&databytecount), ::core::mem::transmute_copy(&pchunkdata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prequestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertChunksWithStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDedupDataPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, chunkcount: u32, pchunkmetadata: *const DedupChunk, databytecount: u32, pchunkdatastream: *mut ::core::ffi::c_void, prequestid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InsertChunksWithStream(::core::mem::transmute_copy(&chunkcount), ::core::mem::transmute_copy(&pchunkmetadata), ::core::mem::transmute_copy(&databytecount), ::windows::core::from_raw_borrowed(&pchunkdatastream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prequestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitStreams<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDedupDataPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamcount: u32, pstreams: *const DedupStream, entrycount: u32, pentries: *const DedupStreamEntry, prequestid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CommitStreams(::core::mem::transmute_copy(&streamcount), ::core::mem::transmute_copy(&pstreams), ::core::mem::transmute_copy(&entrycount), ::core::mem::transmute_copy(&pentries)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prequestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitStreamsWithStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDedupDataPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamcount: u32, pstreams: *const DedupStream, entrycount: u32, pentriesstream: *mut ::core::ffi::c_void, prequestid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CommitStreamsWithStream(::core::mem::transmute_copy(&streamcount), ::core::mem::transmute_copy(&pstreams), ::core::mem::transmute_copy(&entrycount), ::windows::core::from_raw_borrowed(&pentriesstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prequestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreams<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDedupDataPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamcount: u32, pstreampaths: *const ::std::mem::MaybeUninit<::windows::core::BSTR>, prequestid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStreams(::core::mem::transmute_copy(&streamcount), ::core::mem::transmute_copy(&pstreampaths)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prequestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamsResults<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDedupDataPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: ::windows::core::GUID, maxwaitms: u32, streamentryindex: u32, pstreamcount: *mut u32, ppstreams: *mut *mut DedupStream, pentrycount: *mut u32, ppentries: *mut *mut DedupStreamEntry, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStreamsResults(::core::mem::transmute(&requestid), ::core::mem::transmute_copy(&maxwaitms), ::core::mem::transmute_copy(&streamentryindex), ::core::mem::transmute_copy(&pstreamcount), ::core::mem::transmute_copy(&ppstreams), ::core::mem::transmute_copy(&pentrycount), ::core::mem::transmute_copy(&ppentries), ::core::mem::transmute_copy(&pstatus), ::core::mem::transmute_copy(&ppitemresults)).into()
        }
        unsafe extern "system" fn GetChunks<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDedupDataPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, phashes: *const DedupHash, prequestid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetChunks(::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&phashes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prequestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChunksResults<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDedupDataPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: ::windows::core::GUID, maxwaitms: u32, chunkindex: u32, pchunkcount: *mut u32, ppchunkmetadata: *mut *mut DedupChunk, pdatabytecount: *mut u32, ppchunkdata: *mut *mut u8, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetChunksResults(::core::mem::transmute(&requestid), ::core::mem::transmute_copy(&maxwaitms), ::core::mem::transmute_copy(&chunkindex), ::core::mem::transmute_copy(&pchunkcount), ::core::mem::transmute_copy(&ppchunkmetadata), ::core::mem::transmute_copy(&pdatabytecount), ::core::mem::transmute_copy(&ppchunkdata), ::core::mem::transmute_copy(&pstatus), ::core::mem::transmute_copy(&ppitemresults)).into()
        }
        unsafe extern "system" fn GetRequestStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDedupDataPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: ::windows::core::GUID, pstatus: *mut DedupDataPortRequestStatus) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRequestStatus(::core::mem::transmute(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRequestResults<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDedupDataPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: ::windows::core::GUID, maxwaitms: u32, pbatchresult: *mut ::windows::core::HRESULT, pbatchcount: *mut u32, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRequestResults(::core::mem::transmute(&requestid), ::core::mem::transmute_copy(&maxwaitms), ::core::mem::transmute_copy(&pbatchresult), ::core::mem::transmute_copy(&pbatchcount), ::core::mem::transmute_copy(&pstatus), ::core::mem::transmute_copy(&ppitemresults)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            LookupChunks: LookupChunks::<Identity, Impl, OFFSET>,
            InsertChunks: InsertChunks::<Identity, Impl, OFFSET>,
            InsertChunksWithStream: InsertChunksWithStream::<Identity, Impl, OFFSET>,
            CommitStreams: CommitStreams::<Identity, Impl, OFFSET>,
            CommitStreamsWithStream: CommitStreamsWithStream::<Identity, Impl, OFFSET>,
            GetStreams: GetStreams::<Identity, Impl, OFFSET>,
            GetStreamsResults: GetStreamsResults::<Identity, Impl, OFFSET>,
            GetChunks: GetChunks::<Identity, Impl, OFFSET>,
            GetChunksResults: GetChunksResults::<Identity, Impl, OFFSET>,
            GetRequestStatus: GetRequestStatus::<Identity, Impl, OFFSET>,
            GetRequestResults: GetRequestResults::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDedupDataPort as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`, `\"implement\"`*"]
pub trait IDedupDataPortManager_Impl: Sized {
    fn GetConfiguration(&self, pminchunksize: *mut u32, pmaxchunksize: *mut u32, pchunkingalgorithm: *mut DedupChunkingAlgorithm, phashingalgorithm: *mut DedupHashingAlgorithm, pcompressionalgorithm: *mut DedupCompressionAlgorithm) -> ::windows::core::Result<()>;
    fn GetVolumeStatus(&self, options: u32, path: &::windows::core::BSTR) -> ::windows::core::Result<DedupDataPortVolumeStatus>;
    fn GetVolumeDataPort(&self, options: u32, path: &::windows::core::BSTR) -> ::windows::core::Result<IDedupDataPort>;
}
impl ::windows::core::RuntimeName for IDedupDataPortManager {}
impl IDedupDataPortManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDedupDataPortManager_Impl, const OFFSET: isize>() -> IDedupDataPortManager_Vtbl {
        unsafe extern "system" fn GetConfiguration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDedupDataPortManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pminchunksize: *mut u32, pmaxchunksize: *mut u32, pchunkingalgorithm: *mut DedupChunkingAlgorithm, phashingalgorithm: *mut DedupHashingAlgorithm, pcompressionalgorithm: *mut DedupCompressionAlgorithm) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetConfiguration(::core::mem::transmute_copy(&pminchunksize), ::core::mem::transmute_copy(&pmaxchunksize), ::core::mem::transmute_copy(&pchunkingalgorithm), ::core::mem::transmute_copy(&phashingalgorithm), ::core::mem::transmute_copy(&pcompressionalgorithm)).into()
        }
        unsafe extern "system" fn GetVolumeStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDedupDataPortManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: u32, path: ::std::mem::MaybeUninit<::windows::core::BSTR>, pstatus: *mut DedupDataPortVolumeStatus) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVolumeStatus(::core::mem::transmute_copy(&options), ::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVolumeDataPort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDedupDataPortManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: u32, path: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppdataport: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVolumeDataPort(::core::mem::transmute_copy(&options), ::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdataport, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetConfiguration: GetConfiguration::<Identity, Impl, OFFSET>,
            GetVolumeStatus: GetVolumeStatus::<Identity, Impl, OFFSET>,
            GetVolumeDataPort: GetVolumeDataPort::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDedupDataPortManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`, `\"implement\"`*"]
pub trait IDedupIterateChunksHash32_Impl: Sized {
    fn PushBuffer(&self, pbuffer: *const u8, ulbufferlength: u32) -> ::windows::core::Result<()>;
    fn Next(&self, ulmaxchunks: u32, parrchunks: *mut DEDUP_CHUNK_INFO_HASH32, pulfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Drain(&self) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDedupIterateChunksHash32 {}
impl IDedupIterateChunksHash32_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDedupIterateChunksHash32_Impl, const OFFSET: isize>() -> IDedupIterateChunksHash32_Vtbl {
        unsafe extern "system" fn PushBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDedupIterateChunksHash32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: *const u8, ulbufferlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PushBuffer(::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&ulbufferlength)).into()
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDedupIterateChunksHash32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulmaxchunks: u32, parrchunks: *mut DEDUP_CHUNK_INFO_HASH32, pulfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&ulmaxchunks), ::core::mem::transmute_copy(&parrchunks), ::core::mem::transmute_copy(&pulfetched)).into()
        }
        unsafe extern "system" fn Drain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDedupIterateChunksHash32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Drain().into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDedupIterateChunksHash32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PushBuffer: PushBuffer::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Drain: Drain::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDedupIterateChunksHash32 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`, `\"implement\"`*"]
pub trait IDedupReadFileCallback_Impl: Sized {
    fn ReadBackupFile(&self, filefullpath: &::windows::core::BSTR, fileoffset: i64, sizetoread: u32, filebuffer: *mut u8, returnedsize: *mut u32, flags: u32) -> ::windows::core::Result<()>;
    fn OrderContainersRestore(&self, numberofcontainers: u32, containerpaths: *const ::windows::core::BSTR, readplanentries: *mut u32, readplan: *mut *mut DEDUP_CONTAINER_EXTENT) -> ::windows::core::Result<()>;
    fn PreviewContainerRead(&self, filefullpath: &::windows::core::BSTR, numberofreads: u32, readoffsets: *const DDP_FILE_EXTENT) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDedupReadFileCallback {}
impl IDedupReadFileCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDedupReadFileCallback_Impl, const OFFSET: isize>() -> IDedupReadFileCallback_Vtbl {
        unsafe extern "system" fn ReadBackupFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDedupReadFileCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filefullpath: ::std::mem::MaybeUninit<::windows::core::BSTR>, fileoffset: i64, sizetoread: u32, filebuffer: *mut u8, returnedsize: *mut u32, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReadBackupFile(::core::mem::transmute(&filefullpath), ::core::mem::transmute_copy(&fileoffset), ::core::mem::transmute_copy(&sizetoread), ::core::mem::transmute_copy(&filebuffer), ::core::mem::transmute_copy(&returnedsize), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn OrderContainersRestore<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDedupReadFileCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numberofcontainers: u32, containerpaths: *const ::std::mem::MaybeUninit<::windows::core::BSTR>, readplanentries: *mut u32, readplan: *mut *mut DEDUP_CONTAINER_EXTENT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OrderContainersRestore(::core::mem::transmute_copy(&numberofcontainers), ::core::mem::transmute_copy(&containerpaths), ::core::mem::transmute_copy(&readplanentries), ::core::mem::transmute_copy(&readplan)).into()
        }
        unsafe extern "system" fn PreviewContainerRead<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDedupReadFileCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filefullpath: ::std::mem::MaybeUninit<::windows::core::BSTR>, numberofreads: u32, readoffsets: *const DDP_FILE_EXTENT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PreviewContainerRead(::core::mem::transmute(&filefullpath), ::core::mem::transmute_copy(&numberofreads), ::core::mem::transmute_copy(&readoffsets)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReadBackupFile: ReadBackupFile::<Identity, Impl, OFFSET>,
            OrderContainersRestore: OrderContainersRestore::<Identity, Impl, OFFSET>,
            PreviewContainerRead: PreviewContainerRead::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDedupReadFileCallback as ::windows::core::ComInterface>::IID
    }
}
