pub trait IPhotoAcquireImpl: Sized {
    fn CreatePhotoSource();
    fn Acquire();
    fn EnumResults();
}
pub trait IPhotoAcquireDeviceSelectionDialogImpl: Sized {
    fn SetTitle();
    fn SetSubmitButtonText();
    fn DoModal();
}
pub trait IPhotoAcquireItemImpl: Sized {
    fn GetItemName();
    fn GetThumbnail();
    fn GetProperty();
    fn SetProperty();
    fn GetStream();
    fn CanDelete();
    fn Delete();
    fn GetSubItemCount();
    fn GetSubItemAt();
}
pub trait IPhotoAcquireOptionsDialogImpl: Sized {
    fn Initialize();
    fn Create();
    fn Destroy();
    fn DoModal();
    fn SaveData();
}
pub trait IPhotoAcquirePluginImpl: Sized {
    fn Initialize();
    fn ProcessItem();
    fn TransferComplete();
    fn DisplayConfigureDialog();
}
pub trait IPhotoAcquireProgressCBImpl: Sized {
    fn Cancelled();
    fn StartEnumeration();
    fn FoundItem();
    fn EndEnumeration();
    fn StartTransfer();
    fn StartItemTransfer();
    fn DirectoryCreated();
    fn UpdateTransferPercent();
    fn EndItemTransfer();
    fn EndTransfer();
    fn StartDelete();
    fn StartItemDelete();
    fn UpdateDeletePercent();
    fn EndItemDelete();
    fn EndDelete();
    fn EndSession();
    fn GetDeleteAfterAcquire();
    fn ErrorAdvise();
    fn GetUserInput();
}
pub trait IPhotoAcquireSettingsImpl: Sized {
    fn InitializeFromRegistry();
    fn SetFlags();
    fn SetOutputFilenameTemplate();
    fn SetSequencePaddingWidth();
    fn SetSequenceZeroPadding();
    fn SetGroupTag();
    fn SetAcquisitionTime();
    fn GetFlags();
    fn GetOutputFilenameTemplate();
    fn GetSequencePaddingWidth();
    fn GetSequenceZeroPadding();
    fn GetGroupTag();
    fn GetAcquisitionTime();
}
pub trait IPhotoAcquireSourceImpl: Sized {
    fn GetFriendlyName();
    fn GetDeviceIcons();
    fn InitializeItemList();
    fn GetItemCount();
    fn GetItemAt();
    fn GetPhotoAcquireSettings();
    fn GetDeviceId();
    fn BindToObject();
}
pub trait IPhotoProgressActionCBImpl: Sized {
    fn DoAction();
}
pub trait IPhotoProgressDialogImpl: Sized {
    fn Create();
    fn GetWindow();
    fn Destroy();
    fn SetTitle();
    fn ShowCheckbox();
    fn SetCheckboxText();
    fn SetCheckboxCheck();
    fn SetCheckboxTooltip();
    fn IsCheckboxChecked();
    fn SetCaption();
    fn SetImage();
    fn SetPercentComplete();
    fn SetProgressText();
    fn SetActionLinkCallback();
    fn SetActionLinkText();
    fn ShowActionLink();
    fn IsCancelled();
    fn GetUserInput();
}
pub trait IUserInputStringImpl: Sized {
    fn GetSubmitButtonText();
    fn GetPrompt();
    fn GetStringId();
    fn GetStringType();
    fn GetTooltipText();
    fn GetMaxLength();
    fn GetDefault();
    fn GetMruCount();
    fn GetMruEntryAt();
    fn GetImage();
}
