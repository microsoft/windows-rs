pub trait IDedupBackupSupportImpl: Sized {
    fn RestoreFiles();
}
pub trait IDedupChunkLibraryImpl: Sized {
    fn InitializeForPushBuffers();
    fn Uninitialize();
    fn SetParameter();
    fn StartChunking();
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
pub trait IDedupDataPortManagerImpl: Sized {
    fn GetConfiguration();
    fn GetVolumeStatus();
    fn GetVolumeDataPort();
}
pub trait IDedupIterateChunksHash32Impl: Sized {
    fn PushBuffer();
    fn Next();
    fn Drain();
    fn Reset();
}
pub trait IDedupReadFileCallbackImpl: Sized {
    fn ReadBackupFile();
    fn OrderContainersRestore();
    fn PreviewContainerRead();
}
