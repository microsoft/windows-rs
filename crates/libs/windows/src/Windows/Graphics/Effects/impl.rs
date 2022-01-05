pub trait IGraphicsEffectImpl: Sized + IGraphicsEffectSourceImpl {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
pub trait IGraphicsEffectSourceImpl: Sized {}
