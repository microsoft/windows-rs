pub trait ICloneViewHelperImpl: Sized {
    fn GetConnectedIDs();
    fn GetActiveTopology();
    fn SetActiveTopology();
    fn Commit();
}
pub trait IViewHelperImpl: Sized {
    fn GetConnectedIDs();
    fn GetActiveTopology();
    fn SetActiveTopology();
    fn Commit();
    fn SetConfiguration();
    fn GetProceedOnNewConfiguration();
}
