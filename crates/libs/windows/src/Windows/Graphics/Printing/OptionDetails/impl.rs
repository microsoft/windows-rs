#[cfg(feature = "implement_exclusive")]
pub trait IPrintBindingOptionDetailsImpl: Sized {
    fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintBorderingOptionDetailsImpl: Sized {
    fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintCollationOptionDetailsImpl: Sized {
    fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintColorModeOptionDetailsImpl: Sized {
    fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintCopiesOptionDetailsImpl: Sized {
    fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintCustomItemDetailsImpl: Sized {
    fn ItemId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetItemDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ItemDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintCustomItemListOptionDetailsImpl: Sized + IPrintCustomOptionDetailsImpl + IPrintItemListOptionDetailsImpl + IPrintOptionDetailsImpl {
    fn AddItem(&self, itemid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintCustomItemListOptionDetails2Impl: Sized {
    fn AddItem(&self, itemid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING, description: &::windows::core::HSTRING, icon: &::core::option::Option<super::super::super::Storage::Streams::IRandomAccessStreamWithContentType>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintCustomItemListOptionDetails3Impl: Sized {
    fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
pub trait IPrintCustomOptionDetailsImpl: Sized + IPrintOptionDetailsImpl {
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintCustomTextOptionDetailsImpl: Sized + IPrintCustomOptionDetailsImpl + IPrintOptionDetailsImpl {
    fn SetMaxCharacters(&self, value: u32) -> ::windows::core::Result<()>;
    fn MaxCharacters(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintCustomTextOptionDetails2Impl: Sized {
    fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintCustomToggleOptionDetailsImpl: Sized {
    fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintDuplexOptionDetailsImpl: Sized {
    fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintHolePunchOptionDetailsImpl: Sized {
    fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
pub trait IPrintItemListOptionDetailsImpl: Sized + IPrintOptionDetailsImpl {
    fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintMediaSizeOptionDetailsImpl: Sized {
    fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintMediaTypeOptionDetailsImpl: Sized {
    fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
pub trait IPrintNumberOptionDetailsImpl: Sized + IPrintOptionDetailsImpl {
    fn MinValue(&self) -> ::windows::core::Result<u32>;
    fn MaxValue(&self) -> ::windows::core::Result<u32>;
}
pub trait IPrintOptionDetailsImpl: Sized {
    fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OptionType(&self) -> ::windows::core::Result<PrintOptionType>;
    fn SetErrorText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()>;
    fn State(&self) -> ::windows::core::Result<PrintOptionStates>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn TrySetValue(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintOrientationOptionDetailsImpl: Sized {
    fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintPageRangeOptionDetailsImpl: Sized {
    fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintQualityOptionDetailsImpl: Sized {
    fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintStapleOptionDetailsImpl: Sized {
    fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTaskOptionChangedEventArgsImpl: Sized {
    fn OptionId(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTaskOptionDetailsImpl: Sized {
    fn Options(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, IPrintOptionDetails>>;
    fn CreateItemListOption(&self, optionid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<PrintCustomItemListOptionDetails>;
    fn CreateTextOption(&self, optionid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<PrintCustomTextOptionDetails>;
    fn OptionChanged(&self, eventhandler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PrintTaskOptionDetails, PrintTaskOptionChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveOptionChanged(&self, eventcookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BeginValidation(&self, eventhandler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PrintTaskOptionDetails, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveBeginValidation(&self, eventcookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTaskOptionDetails2Impl: Sized {
    fn CreateToggleOption(&self, optionid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<PrintCustomToggleOptionDetails>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTaskOptionDetailsStaticImpl: Sized {
    fn GetFromPrintTaskOptions(&self, printtaskoptions: &::core::option::Option<super::PrintTaskOptions>) -> ::windows::core::Result<PrintTaskOptionDetails>;
}
pub trait IPrintTextOptionDetailsImpl: Sized + IPrintOptionDetailsImpl {
    fn MaxCharacters(&self) -> ::windows::core::Result<u32>;
}
