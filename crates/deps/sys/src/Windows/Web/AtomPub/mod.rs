#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn AtomPubClient();
    fn IAtomPubClient();
    fn IAtomPubClientFactory();
    fn IResourceCollection();
    fn IServiceDocument();
    fn IWorkspace();
    fn ResourceCollection();
    fn ServiceDocument();
    fn Workspace();
}
