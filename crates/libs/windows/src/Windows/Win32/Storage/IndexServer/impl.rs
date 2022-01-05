pub trait IFilterImpl: Sized {
    fn Init();
    fn GetChunk();
    fn GetText();
    fn GetValue();
    fn BindRegion();
}
pub trait IPhraseSinkImpl: Sized {
    fn PutSmallPhrase();
    fn PutPhrase();
}
