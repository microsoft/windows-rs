pub trait IEnumWIA_DEV_CAPSImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
pub trait IEnumWIA_DEV_INFOImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
pub trait IEnumWIA_FORMAT_INFOImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
pub trait IEnumWiaItemImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
pub trait IEnumWiaItem2Impl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
pub trait IWiaAppErrorHandlerImpl: Sized {
    fn GetWindow();
    fn ReportStatus();
}
pub trait IWiaDataCallbackImpl: Sized {
    fn BandedDataCallback();
}
pub trait IWiaDataTransferImpl: Sized {
    fn idtGetData();
    fn idtGetBandedData();
    fn idtQueryGetData();
    fn idtEnumWIA_FORMAT_INFO();
    fn idtGetExtendedTransferInfo();
}
pub trait IWiaDevMgrImpl: Sized {
    fn EnumDeviceInfo();
    fn CreateDevice();
    fn SelectDeviceDlg();
    fn SelectDeviceDlgID();
    fn GetImageDlg();
    fn RegisterEventCallbackProgram();
    fn RegisterEventCallbackInterface();
    fn RegisterEventCallbackCLSID();
    fn AddDeviceDlg();
}
pub trait IWiaDevMgr2Impl: Sized {
    fn EnumDeviceInfo();
    fn CreateDevice();
    fn SelectDeviceDlg();
    fn SelectDeviceDlgID();
    fn RegisterEventCallbackInterface();
    fn RegisterEventCallbackProgram();
    fn RegisterEventCallbackCLSID();
    fn GetImageDlg();
}
pub trait IWiaDrvItemImpl: Sized {
    fn GetItemFlags();
    fn GetDeviceSpecContext();
    fn GetFullItemName();
    fn GetItemName();
    fn AddItemToFolder();
    fn UnlinkItemTree();
    fn RemoveItemFromFolder();
    fn FindItemByName();
    fn FindChildItemByName();
    fn GetParentItem();
    fn GetFirstChildItem();
    fn GetNextSiblingItem();
    fn DumpItemData();
}
pub trait IWiaErrorHandlerImpl: Sized {
    fn ReportStatus();
    fn GetStatusDescription();
}
pub trait IWiaEventCallbackImpl: Sized {
    fn ImageEventCallback();
}
pub trait IWiaImageFilterImpl: Sized {
    fn InitializeFilter();
    fn SetNewCallback();
    fn FilterPreviewImage();
    fn ApplyProperties();
}
pub trait IWiaItemImpl: Sized {
    fn GetItemType();
    fn AnalyzeItem();
    fn EnumChildItems();
    fn DeleteItem();
    fn CreateChildItem();
    fn EnumRegisterEventInfo();
    fn FindItemByName();
    fn DeviceDlg();
    fn DeviceCommand();
    fn GetRootItem();
    fn EnumDeviceCapabilities();
    fn DumpItemData();
    fn DumpDrvItemData();
    fn DumpTreeItemData();
    fn Diagnostic();
}
pub trait IWiaItem2Impl: Sized {
    fn CreateChildItem();
    fn DeleteItem();
    fn EnumChildItems();
    fn FindItemByName();
    fn GetItemCategory();
    fn GetItemType();
    fn DeviceDlg();
    fn DeviceCommand();
    fn EnumDeviceCapabilities();
    fn CheckExtension();
    fn GetExtension();
    fn GetParentItem();
    fn GetRootItem();
    fn GetPreviewComponent();
    fn EnumRegisterEventInfo();
    fn Diagnostic();
}
pub trait IWiaItemExtrasImpl: Sized {
    fn GetExtendedErrorInfo();
    fn Escape();
    fn CancelPendingIO();
}
pub trait IWiaLogImpl: Sized {
    fn InitializeLog();
    fn hResult();
    fn Log();
}
pub trait IWiaLogExImpl: Sized {
    fn InitializeLogEx();
    fn hResult();
    fn Log();
    fn hResultEx();
    fn LogEx();
}
pub trait IWiaMiniDrvImpl: Sized {
    fn drvInitializeWia();
    fn drvAcquireItemData();
    fn drvInitItemProperties();
    fn drvValidateItemProperties();
    fn drvWriteItemProperties();
    fn drvReadItemProperties();
    fn drvLockWiaDevice();
    fn drvUnLockWiaDevice();
    fn drvAnalyzeItem();
    fn drvGetDeviceErrorStr();
    fn drvDeviceCommand();
    fn drvGetCapabilities();
    fn drvDeleteItem();
    fn drvFreeDrvItemContext();
    fn drvGetWiaFormatInfo();
    fn drvNotifyPnpEvent();
    fn drvUnInitializeWia();
}
pub trait IWiaMiniDrvCallBackImpl: Sized {
    fn MiniDrvCallback();
}
pub trait IWiaMiniDrvTransferCallbackImpl: Sized {
    fn GetNextStream();
    fn SendMessage();
}
pub trait IWiaNotifyDevMgrImpl: Sized {
    fn NewDeviceArrival();
}
pub trait IWiaPreviewImpl: Sized {
    fn GetNewPreview();
    fn UpdatePreview();
    fn DetectRegions();
    fn Clear();
}
pub trait IWiaPropertyStorageImpl: Sized {
    fn ReadMultiple();
    fn WriteMultiple();
    fn DeleteMultiple();
    fn ReadPropertyNames();
    fn WritePropertyNames();
    fn DeletePropertyNames();
    fn Commit();
    fn Revert();
    fn Enum();
    fn SetTimes();
    fn SetClass();
    fn Stat();
    fn GetPropertyAttributes();
    fn GetCount();
    fn GetPropertyStream();
    fn SetPropertyStream();
}
pub trait IWiaSegmentationFilterImpl: Sized {
    fn DetectRegions();
}
pub trait IWiaTransferImpl: Sized {
    fn Download();
    fn Upload();
    fn Cancel();
    fn EnumWIA_FORMAT_INFO();
}
pub trait IWiaTransferCallbackImpl: Sized {
    fn TransferCallback();
    fn GetNextStream();
}
pub trait IWiaUIExtensionImpl: Sized {
    fn DeviceDialog();
    fn GetDeviceIcon();
    fn GetDeviceBitmapLogo();
}
pub trait IWiaUIExtension2Impl: Sized {
    fn DeviceDialog();
    fn GetDeviceIcon();
}
pub trait IWiaVideoImpl: Sized {
    fn PreviewVisible();
    fn SetPreviewVisible();
    fn ImagesDirectory();
    fn SetImagesDirectory();
    fn CreateVideoByWiaDevID();
    fn CreateVideoByDevNum();
    fn CreateVideoByName();
    fn DestroyVideo();
    fn Play();
    fn Pause();
    fn TakePicture();
    fn ResizeVideo();
    fn GetCurrentState();
}
