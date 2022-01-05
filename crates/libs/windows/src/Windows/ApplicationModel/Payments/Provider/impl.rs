#[cfg(feature = "implement_exclusive")]
pub trait IPaymentAppCanMakePaymentTriggerDetailsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<super::PaymentRequest>;
    fn ReportCanMakePaymentResult(&self, value: &::core::option::Option<super::PaymentCanMakePaymentResult>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentAppManagerImpl: Sized {
    fn RegisterAsync(&self, supportedpaymentmethodids: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn UnregisterAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentAppManagerStaticsImpl: Sized {
    fn Current(&self) -> ::windows::core::Result<PaymentAppManager>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentTransactionImpl: Sized {
    fn PaymentRequest(&self) -> ::windows::core::Result<super::PaymentRequest>;
    fn PayerEmail(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPayerEmail(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PayerName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPayerName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PayerPhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPayerPhoneNumber(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UpdateShippingAddressAsync(&self, shippingaddress: &::core::option::Option<super::PaymentAddress>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::PaymentRequestChangedResult>>;
    fn UpdateSelectedShippingOptionAsync(&self, selectedshippingoption: &::core::option::Option<super::PaymentShippingOption>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::PaymentRequestChangedResult>>;
    fn AcceptAsync(&self, paymenttoken: &::core::option::Option<super::PaymentToken>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<PaymentTransactionAcceptResult>>;
    fn Reject(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentTransactionAcceptResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<super::PaymentRequestCompletionStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaymentTransactionStaticsImpl: Sized {
    fn FromIdAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<PaymentTransaction>>;
}
