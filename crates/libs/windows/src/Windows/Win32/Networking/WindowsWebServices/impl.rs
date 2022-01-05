pub trait IContentPrefetcherTaskTriggerImpl: Sized {
    fn TriggerContentPrefetcherTask();
    fn IsRegisteredForContentPrefetch();
}
