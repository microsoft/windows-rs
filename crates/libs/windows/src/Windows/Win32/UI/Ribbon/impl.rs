pub trait IUIApplicationImpl: Sized {
    fn OnViewChanged();
    fn OnCreateUICommand();
    fn OnDestroyUICommand();
}
pub trait IUICollectionImpl: Sized {
    fn GetCount();
    fn GetItem();
    fn Add();
    fn Insert();
    fn RemoveAt();
    fn Replace();
    fn Clear();
}
pub trait IUICollectionChangedEventImpl: Sized {
    fn OnChanged();
}
pub trait IUICommandHandlerImpl: Sized {
    fn Execute();
    fn UpdateProperty();
}
pub trait IUIContextualUIImpl: Sized {
    fn ShowAtLocation();
}
pub trait IUIEventLoggerImpl: Sized {
    fn OnUIEvent();
}
pub trait IUIEventingManagerImpl: Sized {
    fn SetEventLogger();
}
pub trait IUIFrameworkImpl: Sized {
    fn Initialize();
    fn Destroy();
    fn LoadUI();
    fn GetView();
    fn GetUICommandProperty();
    fn SetUICommandProperty();
    fn InvalidateUICommand();
    fn FlushPendingInvalidations();
    fn SetModes();
}
pub trait IUIImageImpl: Sized {
    fn GetBitmap();
}
pub trait IUIImageFromBitmapImpl: Sized {
    fn CreateImage();
}
pub trait IUIRibbonImpl: Sized {
    fn GetHeight();
    fn LoadSettingsFromStream();
    fn SaveSettingsToStream();
}
pub trait IUISimplePropertySetImpl: Sized {
    fn GetValue();
}
