#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn ApiInformation();
    fn AttributeTargets();
    fn CompositionType();
    fn DeprecationType();
    fn FeatureStage();
    fn GCPressureAmount();
    fn IApiInformationStatics();
    fn MarshalingType();
    fn Platform();
    fn ThreadingModel();
}
