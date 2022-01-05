pub trait IDMOQualityControlImpl: Sized {
    fn SetNow();
    fn SetStatus();
    fn GetStatus();
}
pub trait IDMOVideoOutputOptimizationsImpl: Sized {
    fn QueryOperationModePreferences();
    fn SetOperationMode();
    fn GetCurrentOperationMode();
    fn GetCurrentSampleRequirements();
}
pub trait IEnumDMOImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IMediaBufferImpl: Sized {
    fn SetLength();
    fn GetMaxLength();
    fn GetBufferAndLength();
}
pub trait IMediaObjectImpl: Sized {
    fn GetStreamCount();
    fn GetInputStreamInfo();
    fn GetOutputStreamInfo();
    fn GetInputType();
    fn GetOutputType();
    fn SetInputType();
    fn SetOutputType();
    fn GetInputCurrentType();
    fn GetOutputCurrentType();
    fn GetInputSizeInfo();
    fn GetOutputSizeInfo();
    fn GetInputMaxLatency();
    fn SetInputMaxLatency();
    fn Flush();
    fn Discontinuity();
    fn AllocateStreamingResources();
    fn FreeStreamingResources();
    fn GetInputStatus();
    fn ProcessInput();
    fn ProcessOutput();
    fn Lock();
}
pub trait IMediaObjectInPlaceImpl: Sized {
    fn Process();
    fn Clone();
    fn GetLatency();
}
