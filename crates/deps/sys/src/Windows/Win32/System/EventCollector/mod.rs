#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn EC_CREATE_NEW();
    fn EC_OPEN_ALWAYS();
    fn EC_OPEN_EXISTING();
    fn EC_READ_ACCESS();
    fn EC_SUBSCRIPTION_CONFIGURATION_MODE();
    fn EC_SUBSCRIPTION_CONTENT_FORMAT();
    fn EC_SUBSCRIPTION_CREDENTIALS_TYPE();
    fn EC_SUBSCRIPTION_DELIVERY_MODE();
    fn EC_SUBSCRIPTION_PROPERTY_ID();
    fn EC_SUBSCRIPTION_RUNTIME_STATUS_ACTIVE_STATUS();
    fn EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID();
    fn EC_SUBSCRIPTION_TYPE();
    fn EC_VARIANT();
    fn EC_VARIANT_TYPE();
    fn EC_VARIANT_TYPE_ARRAY();
    fn EC_VARIANT_TYPE_MASK();
    fn EC_WRITE_ACCESS();
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
