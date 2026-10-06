use super::*;

impl ImperativeRequest {
    pub(crate) fn object(&self) -> ObjectId {
        match self {
            Self::Focus { object, .. }
            | Self::InitializeWebView2 { object, .. }
            | Self::ObserveSwapChainPanel { object, .. }
            | Self::RequestSwapChainPanelFrame { object, .. }
            | Self::SetSwapChain { object, .. }
            | Self::SetNativeImageSource { object, .. }
            | Self::ObserveImageScale { object, .. }
            | Self::ObserveCompositionHost { object, .. }
            | Self::RevokeObservation { object, .. }
            | Self::SetCompositionChildVisual { object, .. } => *object,
        }
    }
}
