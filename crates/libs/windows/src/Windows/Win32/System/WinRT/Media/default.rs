impl ::core::cmp::PartialEq for IAudioFrameNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioFrameNative {}
impl ::core::fmt::Debug for IAudioFrameNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioFrameNative").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAudioFrameNativeFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioFrameNativeFactory {}
impl ::core::fmt::Debug for IAudioFrameNativeFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioFrameNativeFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IVideoFrameNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVideoFrameNative {}
impl ::core::fmt::Debug for IVideoFrameNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVideoFrameNative").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IVideoFrameNativeFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVideoFrameNativeFactory {}
impl ::core::fmt::Debug for IVideoFrameNativeFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVideoFrameNativeFactory").field(&self.0).finish()
    }
}
