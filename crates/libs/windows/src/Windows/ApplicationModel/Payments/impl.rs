#[cfg(feature = "implement_exclusive")]
pub trait IPaymentAddressImpl: Sized {
    fn Country(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCountry(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AddressLines(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn SetAddressLines(&self, value: &::core::option::Option<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>) -> ::windows::core::Result<()>;
    fn Region(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRegion(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn City(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCity(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DependentLocality(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDependentLocality(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PostalCode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPostalCode(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SortingCode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSortingCode(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LanguageCode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLanguageCode(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Organization(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetOrganization(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Recipient(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRecipient(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPhoneNumber(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentCanMakePaymentResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<PaymentCanMakePaymentResultStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentCanMakePaymentResultFactoryImpl: Sized {
    fn Create(&self, value: PaymentCanMakePaymentResultStatus) -> ::windows::core::Result<PaymentCanMakePaymentResult>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentCurrencyAmountImpl: Sized {
    fn Currency(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCurrency(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CurrencySystem(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCurrencySystem(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentCurrencyAmountFactoryImpl: Sized {
    fn Create(&self, value: &::windows::core::HSTRING, currency: &::windows::core::HSTRING) -> ::windows::core::Result<PaymentCurrencyAmount>;
    fn CreateWithCurrencySystem(&self, value: &::windows::core::HSTRING, currency: &::windows::core::HSTRING, currencysystem: &::windows::core::HSTRING) -> ::windows::core::Result<PaymentCurrencyAmount>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentDetailsImpl: Sized {
    fn Total(&self) -> ::windows::core::Result<PaymentItem>;
    fn SetTotal(&self, value: &::core::option::Option<PaymentItem>) -> ::windows::core::Result<()>;
    fn DisplayItems(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PaymentItem>>;
    fn SetDisplayItems(&self, value: &::core::option::Option<super::super::Foundation::Collections::IVectorView<PaymentItem>>) -> ::windows::core::Result<()>;
    fn ShippingOptions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PaymentShippingOption>>;
    fn SetShippingOptions(&self, value: &::core::option::Option<super::super::Foundation::Collections::IVectorView<PaymentShippingOption>>) -> ::windows::core::Result<()>;
    fn Modifiers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PaymentDetailsModifier>>;
    fn SetModifiers(&self, value: &::core::option::Option<super::super::Foundation::Collections::IVectorView<PaymentDetailsModifier>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentDetailsFactoryImpl: Sized {
    fn Create(&self, total: &::core::option::Option<PaymentItem>) -> ::windows::core::Result<PaymentDetails>;
    fn CreateWithDisplayItems(&self, total: &::core::option::Option<PaymentItem>, displayitems: &::core::option::Option<super::super::Foundation::Collections::IIterable<PaymentItem>>) -> ::windows::core::Result<PaymentDetails>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentDetailsModifierImpl: Sized {
    fn JsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SupportedMethodIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Total(&self) -> ::windows::core::Result<PaymentItem>;
    fn AdditionalDisplayItems(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PaymentItem>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentDetailsModifierFactoryImpl: Sized {
    fn Create(&self, supportedmethodids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, total: &::core::option::Option<PaymentItem>) -> ::windows::core::Result<PaymentDetailsModifier>;
    fn CreateWithAdditionalDisplayItems(&self, supportedmethodids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, total: &::core::option::Option<PaymentItem>, additionaldisplayitems: &::core::option::Option<super::super::Foundation::Collections::IIterable<PaymentItem>>) -> ::windows::core::Result<PaymentDetailsModifier>;
    fn CreateWithAdditionalDisplayItemsAndJsonData(&self, supportedmethodids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, total: &::core::option::Option<PaymentItem>, additionaldisplayitems: &::core::option::Option<super::super::Foundation::Collections::IIterable<PaymentItem>>, jsondata: &::windows::core::HSTRING) -> ::windows::core::Result<PaymentDetailsModifier>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentItemImpl: Sized {
    fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLabel(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Amount(&self) -> ::windows::core::Result<PaymentCurrencyAmount>;
    fn SetAmount(&self, value: &::core::option::Option<PaymentCurrencyAmount>) -> ::windows::core::Result<()>;
    fn Pending(&self) -> ::windows::core::Result<bool>;
    fn SetPending(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentItemFactoryImpl: Sized {
    fn Create(&self, label: &::windows::core::HSTRING, amount: &::core::option::Option<PaymentCurrencyAmount>) -> ::windows::core::Result<PaymentItem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentMediatorImpl: Sized {
    fn GetSupportedMethodIdsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
    fn SubmitPaymentRequestAsync(&self, paymentrequest: &::core::option::Option<PaymentRequest>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PaymentRequestSubmitResult>>;
    fn SubmitPaymentRequestWithChangeHandlerAsync(&self, paymentrequest: &::core::option::Option<PaymentRequest>, changehandler: &::core::option::Option<PaymentRequestChangedHandler>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PaymentRequestSubmitResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentMediator2Impl: Sized {
    fn CanMakePaymentAsync(&self, paymentrequest: &::core::option::Option<PaymentRequest>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PaymentCanMakePaymentResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentMerchantInfoImpl: Sized {
    fn PackageFullName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentMerchantInfoFactoryImpl: Sized {
    fn Create(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<PaymentMerchantInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentMethodDataImpl: Sized {
    fn SupportedMethodIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn JsonData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentMethodDataFactoryImpl: Sized {
    fn Create(&self, supportedmethodids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<PaymentMethodData>;
    fn CreateWithJsonData(&self, supportedmethodids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, jsondata: &::windows::core::HSTRING) -> ::windows::core::Result<PaymentMethodData>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentOptionsImpl: Sized {
    fn RequestPayerEmail(&self) -> ::windows::core::Result<PaymentOptionPresence>;
    fn SetRequestPayerEmail(&self, value: PaymentOptionPresence) -> ::windows::core::Result<()>;
    fn RequestPayerName(&self) -> ::windows::core::Result<PaymentOptionPresence>;
    fn SetRequestPayerName(&self, value: PaymentOptionPresence) -> ::windows::core::Result<()>;
    fn RequestPayerPhoneNumber(&self) -> ::windows::core::Result<PaymentOptionPresence>;
    fn SetRequestPayerPhoneNumber(&self, value: PaymentOptionPresence) -> ::windows::core::Result<()>;
    fn RequestShipping(&self) -> ::windows::core::Result<bool>;
    fn SetRequestShipping(&self, value: bool) -> ::windows::core::Result<()>;
    fn ShippingType(&self) -> ::windows::core::Result<PaymentShippingType>;
    fn SetShippingType(&self, value: PaymentShippingType) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentRequestImpl: Sized {
    fn MerchantInfo(&self) -> ::windows::core::Result<PaymentMerchantInfo>;
    fn Details(&self) -> ::windows::core::Result<PaymentDetails>;
    fn MethodData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PaymentMethodData>>;
    fn Options(&self) -> ::windows::core::Result<PaymentOptions>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentRequest2Impl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentRequestChangedArgsImpl: Sized {
    fn ChangeKind(&self) -> ::windows::core::Result<PaymentRequestChangeKind>;
    fn ShippingAddress(&self) -> ::windows::core::Result<PaymentAddress>;
    fn SelectedShippingOption(&self) -> ::windows::core::Result<PaymentShippingOption>;
    fn Acknowledge(&self, changeresult: &::core::option::Option<PaymentRequestChangedResult>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentRequestChangedResultImpl: Sized {
    fn ChangeAcceptedByMerchant(&self) -> ::windows::core::Result<bool>;
    fn SetChangeAcceptedByMerchant(&self, value: bool) -> ::windows::core::Result<()>;
    fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMessage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UpdatedPaymentDetails(&self) -> ::windows::core::Result<PaymentDetails>;
    fn SetUpdatedPaymentDetails(&self, value: &::core::option::Option<PaymentDetails>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentRequestChangedResultFactoryImpl: Sized {
    fn Create(&self, changeacceptedbymerchant: bool) -> ::windows::core::Result<PaymentRequestChangedResult>;
    fn CreateWithPaymentDetails(&self, changeacceptedbymerchant: bool, updatedpaymentdetails: &::core::option::Option<PaymentDetails>) -> ::windows::core::Result<PaymentRequestChangedResult>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentRequestFactoryImpl: Sized {
    fn Create(&self, details: &::core::option::Option<PaymentDetails>, methoddata: &::core::option::Option<super::super::Foundation::Collections::IIterable<PaymentMethodData>>) -> ::windows::core::Result<PaymentRequest>;
    fn CreateWithMerchantInfo(&self, details: &::core::option::Option<PaymentDetails>, methoddata: &::core::option::Option<super::super::Foundation::Collections::IIterable<PaymentMethodData>>, merchantinfo: &::core::option::Option<PaymentMerchantInfo>) -> ::windows::core::Result<PaymentRequest>;
    fn CreateWithMerchantInfoAndOptions(&self, details: &::core::option::Option<PaymentDetails>, methoddata: &::core::option::Option<super::super::Foundation::Collections::IIterable<PaymentMethodData>>, merchantinfo: &::core::option::Option<PaymentMerchantInfo>, options: &::core::option::Option<PaymentOptions>) -> ::windows::core::Result<PaymentRequest>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentRequestFactory2Impl: Sized {
    fn CreateWithMerchantInfoOptionsAndId(&self, details: &::core::option::Option<PaymentDetails>, methoddata: &::core::option::Option<super::super::Foundation::Collections::IIterable<PaymentMethodData>>, merchantinfo: &::core::option::Option<PaymentMerchantInfo>, options: &::core::option::Option<PaymentOptions>, id: &::windows::core::HSTRING) -> ::windows::core::Result<PaymentRequest>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentRequestSubmitResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<PaymentRequestStatus>;
    fn Response(&self) -> ::windows::core::Result<PaymentResponse>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentResponseImpl: Sized {
    fn PaymentToken(&self) -> ::windows::core::Result<PaymentToken>;
    fn ShippingOption(&self) -> ::windows::core::Result<PaymentShippingOption>;
    fn ShippingAddress(&self) -> ::windows::core::Result<PaymentAddress>;
    fn PayerEmail(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PayerName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PayerPhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CompleteAsync(&self, status: PaymentRequestCompletionStatus) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentShippingOptionImpl: Sized {
    fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLabel(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Amount(&self) -> ::windows::core::Result<PaymentCurrencyAmount>;
    fn SetAmount(&self, value: &::core::option::Option<PaymentCurrencyAmount>) -> ::windows::core::Result<()>;
    fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTag(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsSelected(&self) -> ::windows::core::Result<bool>;
    fn SetIsSelected(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentShippingOptionFactoryImpl: Sized {
    fn Create(&self, label: &::windows::core::HSTRING, amount: &::core::option::Option<PaymentCurrencyAmount>) -> ::windows::core::Result<PaymentShippingOption>;
    fn CreateWithSelected(&self, label: &::windows::core::HSTRING, amount: &::core::option::Option<PaymentCurrencyAmount>, selected: bool) -> ::windows::core::Result<PaymentShippingOption>;
    fn CreateWithSelectedAndTag(&self, label: &::windows::core::HSTRING, amount: &::core::option::Option<PaymentCurrencyAmount>, selected: bool, tag: &::windows::core::HSTRING) -> ::windows::core::Result<PaymentShippingOption>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentTokenImpl: Sized {
    fn PaymentMethodId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn JsonDetails(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentTokenFactoryImpl: Sized {
    fn Create(&self, paymentmethodid: &::windows::core::HSTRING) -> ::windows::core::Result<PaymentToken>;
    fn CreateWithJsonDetails(&self, paymentmethodid: &::windows::core::HSTRING, jsondetails: &::windows::core::HSTRING) -> ::windows::core::Result<PaymentToken>;
}
