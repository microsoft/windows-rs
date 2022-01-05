pub trait IMFDeviceTransformImpl: Sized {
    fn InitializeTransform();
    fn GetInputAvailableType();
    fn GetInputCurrentType();
    fn GetInputStreamAttributes();
    fn GetOutputAvailableType();
    fn GetOutputCurrentType();
    fn GetOutputStreamAttributes();
    fn GetStreamCount();
    fn GetStreamIDs();
    fn ProcessEvent();
    fn ProcessInput();
    fn ProcessMessage();
    fn ProcessOutput();
    fn SetInputStreamState();
    fn GetInputStreamState();
    fn SetOutputStreamState();
    fn GetOutputStreamState();
    fn GetInputStreamPreferredState();
    fn FlushInputStream();
    fn FlushOutputStream();
}
pub trait IMFDeviceTransformCallbackImpl: Sized {
    fn OnBufferSent();
}
