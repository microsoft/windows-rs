pub trait IBitmapDataImpl: Sized {
    fn CopyBytesTo();
    fn GetStride();
    fn GetBitmapDescription();
    fn GetSourceBitmapDescription();
}
pub trait IVisualTreeServiceImpl: Sized {
    fn AdviseVisualTreeChange();
    fn UnadviseVisualTreeChange();
    fn GetEnums();
    fn CreateInstance();
    fn GetPropertyValuesChain();
    fn SetProperty();
    fn ClearProperty();
    fn GetCollectionCount();
    fn GetCollectionElements();
    fn AddChild();
    fn RemoveChild();
    fn ClearChildren();
}
pub trait IVisualTreeService2Impl: Sized + IVisualTreeServiceImpl {
    fn GetPropertyIndex();
    fn GetProperty();
    fn ReplaceResource();
    fn RenderTargetBitmap();
}
pub trait IVisualTreeService3Impl: Sized + IVisualTreeService2Impl + IVisualTreeServiceImpl {
    fn ResolveResource();
    fn GetDictionaryItem();
    fn AddDictionaryItem();
    fn RemoveDictionaryItem();
}
pub trait IVisualTreeServiceCallbackImpl: Sized {
    fn OnVisualTreeChange();
}
pub trait IVisualTreeServiceCallback2Impl: Sized + IVisualTreeServiceCallbackImpl {
    fn OnElementStateChanged();
}
pub trait IXamlDiagnosticsImpl: Sized {
    fn GetDispatcher();
    fn GetUiLayer();
    fn GetApplication();
    fn GetIInspectableFromHandle();
    fn GetHandleFromIInspectable();
    fn HitTest();
    fn RegisterInstance();
    fn GetInitializationData();
}
