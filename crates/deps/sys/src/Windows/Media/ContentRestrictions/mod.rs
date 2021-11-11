#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn ContentAccessRestrictionLevel();
    fn ContentRestrictionsBrowsePolicy();
    fn IContentRestrictionsBrowsePolicy();
    fn IRatedContentDescription();
    fn IRatedContentDescriptionFactory();
    fn IRatedContentRestrictions();
    fn IRatedContentRestrictionsFactory();
    fn RatedContentCategory();
    fn RatedContentDescription();
    fn RatedContentRestrictions();
}
