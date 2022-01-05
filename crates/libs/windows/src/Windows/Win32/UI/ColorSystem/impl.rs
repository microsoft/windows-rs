pub trait IDeviceModelPlugInImpl: Sized {
    fn Initialize();
    fn GetNumChannels();
    fn DeviceToColorimetricColors();
    fn ColorimetricToDeviceColors();
    fn ColorimetricToDeviceColorsWithBlack();
    fn SetTransformDeviceModelInfo();
    fn GetPrimarySamples();
    fn GetGamutBoundaryMeshSize();
    fn GetGamutBoundaryMesh();
    fn GetNeutralAxisSize();
    fn GetNeutralAxis();
}
pub trait IGamutMapModelPlugInImpl: Sized {
    fn Initialize();
    fn SourceToDestinationAppearanceColors();
}
