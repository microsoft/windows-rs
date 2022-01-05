pub trait IImageListImpl: Sized {
    fn Add();
    fn ReplaceIcon();
    fn SetOverlayImage();
    fn Replace();
    fn AddMasked();
    fn Draw();
    fn Remove();
    fn GetIcon();
    fn GetImageInfo();
    fn Copy();
    fn Merge();
    fn Clone();
    fn GetImageRect();
    fn GetIconSize();
    fn SetIconSize();
    fn GetImageCount();
    fn SetImageCount();
    fn SetBkColor();
    fn GetBkColor();
    fn BeginDrag();
    fn EndDrag();
    fn DragEnter();
    fn DragLeave();
    fn DragMove();
    fn SetDragCursorImage();
    fn DragShowNolock();
    fn GetDragImage();
    fn GetItemFlags();
    fn GetOverlayImage();
}
pub trait IImageList2Impl: Sized + IImageListImpl {
    fn Resize();
    fn GetOriginalSize();
    fn SetOriginalSize();
    fn SetCallback();
    fn GetCallback();
    fn ForceImagePresent();
    fn DiscardImages();
    fn PreloadImages();
    fn GetStatistics();
    fn Initialize();
    fn Replace2();
    fn ReplaceFromImageList();
}
