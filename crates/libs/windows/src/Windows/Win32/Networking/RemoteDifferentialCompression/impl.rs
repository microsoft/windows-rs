pub trait IFindSimilarResultsImpl: Sized {
    fn GetSize();
    fn GetNextFileId();
}
pub trait IRdcComparatorImpl: Sized {
    fn Process();
}
pub trait IRdcFileReaderImpl: Sized {
    fn GetFileSize();
    fn Read();
    fn GetFilePosition();
}
pub trait IRdcFileWriterImpl: Sized + IRdcFileReaderImpl {
    fn Write();
    fn Truncate();
    fn DeleteOnClose();
}
pub trait IRdcGeneratorImpl: Sized {
    fn GetGeneratorParameters();
    fn Process();
}
pub trait IRdcGeneratorFilterMaxParametersImpl: Sized {
    fn GetHorizonSize();
    fn SetHorizonSize();
    fn GetHashWindowSize();
    fn SetHashWindowSize();
}
pub trait IRdcGeneratorParametersImpl: Sized {
    fn GetGeneratorParametersType();
    fn GetParametersVersion();
    fn GetSerializeSize();
    fn Serialize();
}
pub trait IRdcLibraryImpl: Sized {
    fn ComputeDefaultRecursionDepth();
    fn CreateGeneratorParameters();
    fn OpenGeneratorParameters();
    fn CreateGenerator();
    fn CreateComparator();
    fn CreateSignatureReader();
    fn GetRDCVersion();
}
pub trait IRdcSignatureReaderImpl: Sized {
    fn ReadHeader();
    fn ReadSignatures();
}
pub trait IRdcSimilarityGeneratorImpl: Sized {
    fn EnableSimilarity();
    fn Results();
}
pub trait ISimilarityImpl: Sized {
    fn CreateTable();
    fn CreateTableIndirect();
    fn CloseTable();
    fn Append();
    fn FindSimilarFileId();
    fn CopyAndSwap();
    fn GetRecordCount();
}
pub trait ISimilarityFileIdTableImpl: Sized {
    fn CreateTable();
    fn CreateTableIndirect();
    fn CloseTable();
    fn Append();
    fn Lookup();
    fn Invalidate();
    fn GetRecordCount();
}
pub trait ISimilarityReportProgressImpl: Sized {
    fn ReportProgress();
}
pub trait ISimilarityTableDumpStateImpl: Sized {
    fn GetNextData();
}
pub trait ISimilarityTraitsMappedViewImpl: Sized {
    fn Flush();
    fn Unmap();
    fn Get();
    fn GetView();
}
pub trait ISimilarityTraitsMappingImpl: Sized {
    fn CloseMapping();
    fn SetFileSize();
    fn GetFileSize();
    fn OpenMapping();
    fn ResizeMapping();
    fn GetPageSize();
    fn CreateView();
}
pub trait ISimilarityTraitsTableImpl: Sized {
    fn CreateTable();
    fn CreateTableIndirect();
    fn CloseTable();
    fn Append();
    fn FindSimilarFileIndex();
    fn BeginDump();
    fn GetLastIndex();
}
