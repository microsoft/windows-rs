#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn ILocalCategoriesStatics();
    fn ILocalLocation();
    fn ILocalLocation2();
    fn ILocalLocationFinderResult();
    fn ILocalLocationFinderStatics();
    fn ILocalLocationHoursOfOperationItem();
    fn ILocalLocationRatingInfo();
    fn IPlaceInfoHelperStatics();
    fn LocalCategories();
    fn LocalLocation();
    fn LocalLocationFinder();
    fn LocalLocationFinderResult();
    fn LocalLocationFinderStatus();
    fn LocalLocationHoursOfOperationItem();
    fn LocalLocationRatingInfo();
    fn PlaceInfoHelper();
}
