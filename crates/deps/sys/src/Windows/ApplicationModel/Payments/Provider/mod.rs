#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IPaymentAppCanMakePaymentTriggerDetails();
    fn IPaymentAppManager();
    fn IPaymentAppManagerStatics();
    fn IPaymentTransaction();
    fn IPaymentTransactionAcceptResult();
    fn IPaymentTransactionStatics();
    fn PaymentAppCanMakePaymentTriggerDetails();
    fn PaymentAppManager();
    fn PaymentTransaction();
    fn PaymentTransactionAcceptResult();
}
