#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CAccessiblityWinSAT();
    fn CInitiateWinSAT();
    fn CProvideWinSATVisuals();
    fn CQueryAllWinSAT();
    fn CQueryOEMWinSATCustomization();
    fn CQueryWinSAT();
    fn IAccessibleWinSAT();
    fn IInitiateWinSATAssessment();
    fn IProvideWinSATAssessmentInfo();
    fn IProvideWinSATResultsInfo();
    fn IProvideWinSATVisuals();
    fn IQueryAllWinSATAssessments();
    fn IQueryOEMWinSATCustomization();
    fn IQueryRecentWinSATAssessment();
    fn IWinSATInitiateEvents();
    fn WINSAT_ASSESSMENT_STATE();
    fn WINSAT_ASSESSMENT_TYPE();
    fn WINSAT_BITMAP_SIZE();
    fn WINSAT_OEM_DATA_TYPE();
}
