#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn EcClose();
    fn EcDeleteSubscription();
    fn EcEnumNextSubscription();
    fn EcGetObjectArrayProperty();
    fn EcGetObjectArraySize();
    fn EcGetSubscriptionProperty();
    fn EcGetSubscriptionRunTimeStatus();
    fn EcInsertObjectArrayElement();
    fn EcOpenSubscription();
    fn EcOpenSubscriptionEnum();
    fn EcRemoveObjectArrayElement();
    fn EcRetrySubscription();
    fn EcSaveSubscription();
    fn EcSetObjectArrayProperty();
    fn EcSetSubscriptionProperty();
}
