#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CGD_ARRAY_NODE();
    fn CGD_BINARY_PROPERTY();
    fn CGD_DATE_PROPERTY();
    fn CGD_DEFAULT();
    fn CGD_STRING_PROPERTY();
    fn CGD_UNKNOWN_PROPERTY();
    fn CLSID_ContactAggregationManager();
    fn CONTACT_AGGREGATION_BLOB();
    fn CONTACT_AGGREGATION_COLLECTION_OPTIONS();
    fn CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS();
    fn Contact();
    fn ContactManager();
    fn IContact();
    fn IContactAggregationAggregate();
    fn IContactAggregationAggregateCollection();
    fn IContactAggregationContact();
    fn IContactAggregationContactCollection();
    fn IContactAggregationGroup();
    fn IContactAggregationGroupCollection();
    fn IContactAggregationLink();
    fn IContactAggregationLinkCollection();
    fn IContactAggregationManager();
    fn IContactAggregationServerPerson();
    fn IContactAggregationServerPersonCollection();
    fn IContactCollection();
    fn IContactManager();
    fn IContactProperties();
    fn IContactPropertyCollection();
}
