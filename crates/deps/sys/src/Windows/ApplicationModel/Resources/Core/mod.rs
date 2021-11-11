#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn INamedResource();
    fn IResourceCandidate();
    fn IResourceCandidate2();
    fn IResourceCandidate3();
    fn IResourceContext();
    fn IResourceContextStatics();
    fn IResourceContextStatics2();
    fn IResourceContextStatics3();
    fn IResourceContextStatics4();
    fn IResourceManager();
    fn IResourceManager2();
    fn IResourceManagerStatics();
    fn IResourceMap();
    fn IResourceQualifier();
    fn NamedResource();
    fn ResourceCandidate();
    fn ResourceCandidateKind();
    fn ResourceCandidateVectorView();
    fn ResourceContext();
    fn ResourceContextLanguagesVectorView();
    fn ResourceLayoutInfo();
    fn ResourceManager();
    fn ResourceMap();
    fn ResourceMapIterator();
    fn ResourceMapMapView();
    fn ResourceMapMapViewIterator();
    fn ResourceQualifier();
    fn ResourceQualifierMapView();
    fn ResourceQualifierObservableMap();
    fn ResourceQualifierPersistence();
    fn ResourceQualifierVectorView();
}
