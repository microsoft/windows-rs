pub trait IMILBitmapEffectImpl: Sized {
    fn GetOutput();
    fn GetParentEffect();
    fn SetInputSource();
}
pub trait IMILBitmapEffectConnectionsImpl: Sized {
    fn GetInputConnector();
    fn GetOutputConnector();
}
pub trait IMILBitmapEffectConnectionsInfoImpl: Sized {
    fn GetNumberInputs();
    fn GetNumberOutputs();
    fn GetInputConnectorInfo();
    fn GetOutputConnectorInfo();
}
pub trait IMILBitmapEffectConnectorImpl: Sized + IMILBitmapEffectConnectorInfoImpl {
    fn IsConnected();
    fn GetBitmapEffect();
}
pub trait IMILBitmapEffectConnectorInfoImpl: Sized {
    fn GetIndex();
    fn GetOptimalFormat();
    fn GetNumberFormats();
    fn GetFormat();
}
pub trait IMILBitmapEffectEventsImpl: Sized {
    fn PropertyChange();
    fn DirtyRegion();
}
pub trait IMILBitmapEffectFactoryImpl: Sized {
    fn CreateEffect();
    fn CreateContext();
    fn CreateEffectOuter();
}
pub trait IMILBitmapEffectGroupImpl: Sized {
    fn GetInteriorInputConnector();
    fn GetInteriorOutputConnector();
    fn Add();
}
pub trait IMILBitmapEffectGroupImplImpl: Sized {
    fn Preprocess();
    fn GetNumberChildren();
    fn GetChildren();
}
pub trait IMILBitmapEffectImplImpl: Sized {
    fn IsInPlaceModificationAllowed();
    fn SetParentEffect();
    fn GetInputSource();
    fn GetInputSourceBounds();
    fn GetInputBitmapSource();
    fn GetOutputBitmapSource();
    fn Initialize();
}
pub trait IMILBitmapEffectInputConnectorImpl: Sized + IMILBitmapEffectConnectorImpl + IMILBitmapEffectConnectorInfoImpl {
    fn ConnectTo();
    fn GetConnection();
}
pub trait IMILBitmapEffectInteriorInputConnectorImpl: Sized {
    fn GetInputConnector();
}
pub trait IMILBitmapEffectInteriorOutputConnectorImpl: Sized {
    fn GetOutputConnector();
}
pub trait IMILBitmapEffectOutputConnectorImpl: Sized + IMILBitmapEffectConnectorImpl + IMILBitmapEffectConnectorInfoImpl {
    fn GetNumberConnections();
    fn GetConnection();
}
pub trait IMILBitmapEffectOutputConnectorImplImpl: Sized {
    fn AddBackLink();
    fn RemoveBackLink();
}
pub trait IMILBitmapEffectPrimitiveImpl: Sized {
    fn GetOutput();
    fn TransformPoint();
    fn TransformRect();
    fn HasAffineTransform();
    fn HasInverseTransform();
    fn GetAffineMatrix();
}
pub trait IMILBitmapEffectPrimitiveImplImpl: Sized {
    fn IsDirty();
    fn IsVolatile();
}
pub trait IMILBitmapEffectRenderContextImpl: Sized {
    fn SetOutputPixelFormat();
    fn GetOutputPixelFormat();
    fn SetUseSoftwareRenderer();
    fn SetInitialTransform();
    fn GetFinalTransform();
    fn SetOutputDPI();
    fn GetOutputDPI();
    fn SetRegionOfInterest();
}
pub trait IMILBitmapEffectRenderContextImplImpl: Sized {
    fn GetUseSoftwareRenderer();
    fn GetTransform();
    fn UpdateTransform();
    fn GetOutputBounds();
    fn UpdateOutputBounds();
}
pub trait IMILBitmapEffectsImpl: Sized {
    fn _NewEnum();
    fn Parent();
    fn Item();
    fn Count();
}
