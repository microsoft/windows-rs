#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CEventClass();
    fn CEventPublisher();
    fn CEventSubscription();
    fn CEventSystem();
    fn COMEVENTSYSCHANGEINFO();
    fn EOC_ChangeType();
    fn EventObjectChange();
    fn EventObjectChange2();
    fn IDontSupportEventSubscription();
    fn IEnumEventObject();
    fn IEventClass();
    fn IEventClass2();
    fn IEventControl();
    fn IEventObjectChange();
    fn IEventObjectChange2();
    fn IEventObjectCollection();
    fn IEventProperty();
    fn IEventPublisher();
    fn IEventSubscription();
    fn IEventSystem();
    fn IFiringControl();
    fn IMultiInterfaceEventControl();
    fn IMultiInterfacePublisherFilter();
    fn IPublisherFilter();
}
