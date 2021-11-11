#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IIndexedResourceCandidate();
    fn IIndexedResourceQualifier();
    fn IResourceIndexer();
    fn IResourceIndexerFactory();
    fn IResourceIndexerFactory2();
    fn IndexedResourceCandidate();
    fn IndexedResourceQualifier();
    fn IndexedResourceType();
    fn ResourceIndexer();
    fn ResourceIndexerContract();
}
