pub trait IDummyHICONIncluderImpl: Sized {
    fn Dummy();
}
pub trait IThumbnailExtractorImpl: Sized {
    fn ExtractThumbnail();
    fn OnFileUpdated();
}
