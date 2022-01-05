pub trait IDCompositionAffineTransform2DEffectImpl: Sized + IDCompositionFilterEffectImpl + IDCompositionEffectImpl {
    fn SetInterpolationMode();
    fn SetBorderMode();
    fn SetTransformMatrix();
    fn SetTransformMatrixElement();
    fn SetTransformMatrixElement();
    fn SetSharpness();
    fn SetSharpness();
}
pub trait IDCompositionAnimationImpl: Sized {
    fn Reset();
    fn SetAbsoluteBeginTime();
    fn AddCubic();
    fn AddSinusoidal();
    fn AddRepeat();
    fn End();
}
pub trait IDCompositionArithmeticCompositeEffectImpl: Sized + IDCompositionFilterEffectImpl + IDCompositionEffectImpl {
    fn SetCoefficients();
    fn SetClampOutput();
    fn SetCoefficient1();
    fn SetCoefficient1();
    fn SetCoefficient2();
    fn SetCoefficient2();
    fn SetCoefficient3();
    fn SetCoefficient3();
    fn SetCoefficient4();
    fn SetCoefficient4();
}
pub trait IDCompositionBlendEffectImpl: Sized + IDCompositionFilterEffectImpl + IDCompositionEffectImpl {
    fn SetMode();
}
pub trait IDCompositionBrightnessEffectImpl: Sized + IDCompositionFilterEffectImpl + IDCompositionEffectImpl {
    fn SetWhitePoint();
    fn SetBlackPoint();
    fn SetWhitePointX();
    fn SetWhitePointX();
    fn SetWhitePointY();
    fn SetWhitePointY();
    fn SetBlackPointX();
    fn SetBlackPointX();
    fn SetBlackPointY();
    fn SetBlackPointY();
}
pub trait IDCompositionClipImpl: Sized {}
pub trait IDCompositionColorMatrixEffectImpl: Sized + IDCompositionFilterEffectImpl + IDCompositionEffectImpl {
    fn SetMatrix();
    fn SetMatrixElement();
    fn SetMatrixElement();
    fn SetAlphaMode();
    fn SetClampOutput();
}
pub trait IDCompositionCompositeEffectImpl: Sized + IDCompositionFilterEffectImpl + IDCompositionEffectImpl {
    fn SetMode();
}
pub trait IDCompositionDelegatedInkTrailImpl: Sized {
    fn AddTrailPoints();
    fn AddTrailPointsWithPrediction();
    fn RemoveTrailPoints();
    fn StartNewTrail();
}
pub trait IDCompositionDesktopDeviceImpl: Sized + IDCompositionDevice2Impl {
    fn CreateTargetForHwnd();
    fn CreateSurfaceFromHandle();
    fn CreateSurfaceFromHwnd();
}
pub trait IDCompositionDeviceImpl: Sized {
    fn Commit();
    fn WaitForCommitCompletion();
    fn GetFrameStatistics();
    fn CreateTargetForHwnd();
    fn CreateVisual();
    fn CreateSurface();
    fn CreateVirtualSurface();
    fn CreateSurfaceFromHandle();
    fn CreateSurfaceFromHwnd();
    fn CreateTranslateTransform();
    fn CreateScaleTransform();
    fn CreateRotateTransform();
    fn CreateSkewTransform();
    fn CreateMatrixTransform();
    fn CreateTransformGroup();
    fn CreateTranslateTransform3D();
    fn CreateScaleTransform3D();
    fn CreateRotateTransform3D();
    fn CreateMatrixTransform3D();
    fn CreateTransform3DGroup();
    fn CreateEffectGroup();
    fn CreateRectangleClip();
    fn CreateAnimation();
    fn CheckDeviceState();
}
pub trait IDCompositionDevice2Impl: Sized {
    fn Commit();
    fn WaitForCommitCompletion();
    fn GetFrameStatistics();
    fn CreateVisual();
    fn CreateSurfaceFactory();
    fn CreateSurface();
    fn CreateVirtualSurface();
    fn CreateTranslateTransform();
    fn CreateScaleTransform();
    fn CreateRotateTransform();
    fn CreateSkewTransform();
    fn CreateMatrixTransform();
    fn CreateTransformGroup();
    fn CreateTranslateTransform3D();
    fn CreateScaleTransform3D();
    fn CreateRotateTransform3D();
    fn CreateMatrixTransform3D();
    fn CreateTransform3DGroup();
    fn CreateEffectGroup();
    fn CreateRectangleClip();
    fn CreateAnimation();
}
pub trait IDCompositionDevice3Impl: Sized + IDCompositionDevice2Impl {
    fn CreateGaussianBlurEffect();
    fn CreateBrightnessEffect();
    fn CreateColorMatrixEffect();
    fn CreateShadowEffect();
    fn CreateHueRotationEffect();
    fn CreateSaturationEffect();
    fn CreateTurbulenceEffect();
    fn CreateLinearTransferEffect();
    fn CreateTableTransferEffect();
    fn CreateCompositeEffect();
    fn CreateBlendEffect();
    fn CreateArithmeticCompositeEffect();
    fn CreateAffineTransform2DEffect();
}
pub trait IDCompositionDeviceDebugImpl: Sized {
    fn EnableDebugCounters();
    fn DisableDebugCounters();
}
pub trait IDCompositionEffectImpl: Sized {}
pub trait IDCompositionEffectGroupImpl: Sized + IDCompositionEffectImpl {
    fn SetOpacity();
    fn SetOpacity();
    fn SetTransform3D();
}
pub trait IDCompositionFilterEffectImpl: Sized + IDCompositionEffectImpl {
    fn SetInput();
}
pub trait IDCompositionGaussianBlurEffectImpl: Sized + IDCompositionFilterEffectImpl + IDCompositionEffectImpl {
    fn SetStandardDeviation();
    fn SetStandardDeviation();
    fn SetBorderMode();
}
pub trait IDCompositionHueRotationEffectImpl: Sized + IDCompositionFilterEffectImpl + IDCompositionEffectImpl {
    fn SetAngle();
    fn SetAngle();
}
pub trait IDCompositionInkTrailDeviceImpl: Sized {
    fn CreateDelegatedInkTrail();
    fn CreateDelegatedInkTrailForSwapChain();
}
pub trait IDCompositionLinearTransferEffectImpl: Sized + IDCompositionFilterEffectImpl + IDCompositionEffectImpl {
    fn SetRedYIntercept();
    fn SetRedYIntercept();
    fn SetRedSlope();
    fn SetRedSlope();
    fn SetRedDisable();
    fn SetGreenYIntercept();
    fn SetGreenYIntercept();
    fn SetGreenSlope();
    fn SetGreenSlope();
    fn SetGreenDisable();
    fn SetBlueYIntercept();
    fn SetBlueYIntercept();
    fn SetBlueSlope();
    fn SetBlueSlope();
    fn SetBlueDisable();
    fn SetAlphaYIntercept();
    fn SetAlphaYIntercept();
    fn SetAlphaSlope();
    fn SetAlphaSlope();
    fn SetAlphaDisable();
    fn SetClampOutput();
}
pub trait IDCompositionMatrixTransformImpl: Sized + IDCompositionTransformImpl + IDCompositionTransform3DImpl + IDCompositionEffectImpl {
    fn SetMatrix();
    fn SetMatrixElement();
    fn SetMatrixElement();
}
pub trait IDCompositionMatrixTransform3DImpl: Sized + IDCompositionTransform3DImpl + IDCompositionEffectImpl {
    fn SetMatrix();
    fn SetMatrixElement();
    fn SetMatrixElement();
}
pub trait IDCompositionRectangleClipImpl: Sized + IDCompositionClipImpl {
    fn SetLeft();
    fn SetLeft();
    fn SetTop();
    fn SetTop();
    fn SetRight();
    fn SetRight();
    fn SetBottom();
    fn SetBottom();
    fn SetTopLeftRadiusX();
    fn SetTopLeftRadiusX();
    fn SetTopLeftRadiusY();
    fn SetTopLeftRadiusY();
    fn SetTopRightRadiusX();
    fn SetTopRightRadiusX();
    fn SetTopRightRadiusY();
    fn SetTopRightRadiusY();
    fn SetBottomLeftRadiusX();
    fn SetBottomLeftRadiusX();
    fn SetBottomLeftRadiusY();
    fn SetBottomLeftRadiusY();
    fn SetBottomRightRadiusX();
    fn SetBottomRightRadiusX();
    fn SetBottomRightRadiusY();
    fn SetBottomRightRadiusY();
}
pub trait IDCompositionRotateTransformImpl: Sized + IDCompositionTransformImpl + IDCompositionTransform3DImpl + IDCompositionEffectImpl {
    fn SetAngle();
    fn SetAngle();
    fn SetCenterX();
    fn SetCenterX();
    fn SetCenterY();
    fn SetCenterY();
}
pub trait IDCompositionRotateTransform3DImpl: Sized + IDCompositionTransform3DImpl + IDCompositionEffectImpl {
    fn SetAngle();
    fn SetAngle();
    fn SetAxisX();
    fn SetAxisX();
    fn SetAxisY();
    fn SetAxisY();
    fn SetAxisZ();
    fn SetAxisZ();
    fn SetCenterX();
    fn SetCenterX();
    fn SetCenterY();
    fn SetCenterY();
    fn SetCenterZ();
    fn SetCenterZ();
}
pub trait IDCompositionSaturationEffectImpl: Sized + IDCompositionFilterEffectImpl + IDCompositionEffectImpl {
    fn SetSaturation();
    fn SetSaturation();
}
pub trait IDCompositionScaleTransformImpl: Sized + IDCompositionTransformImpl + IDCompositionTransform3DImpl + IDCompositionEffectImpl {
    fn SetScaleX();
    fn SetScaleX();
    fn SetScaleY();
    fn SetScaleY();
    fn SetCenterX();
    fn SetCenterX();
    fn SetCenterY();
    fn SetCenterY();
}
pub trait IDCompositionScaleTransform3DImpl: Sized + IDCompositionTransform3DImpl + IDCompositionEffectImpl {
    fn SetScaleX();
    fn SetScaleX();
    fn SetScaleY();
    fn SetScaleY();
    fn SetScaleZ();
    fn SetScaleZ();
    fn SetCenterX();
    fn SetCenterX();
    fn SetCenterY();
    fn SetCenterY();
    fn SetCenterZ();
    fn SetCenterZ();
}
pub trait IDCompositionShadowEffectImpl: Sized + IDCompositionFilterEffectImpl + IDCompositionEffectImpl {
    fn SetStandardDeviation();
    fn SetStandardDeviation();
    fn SetColor();
    fn SetRed();
    fn SetRed();
    fn SetGreen();
    fn SetGreen();
    fn SetBlue();
    fn SetBlue();
    fn SetAlpha();
    fn SetAlpha();
}
pub trait IDCompositionSkewTransformImpl: Sized + IDCompositionTransformImpl + IDCompositionTransform3DImpl + IDCompositionEffectImpl {
    fn SetAngleX();
    fn SetAngleX();
    fn SetAngleY();
    fn SetAngleY();
    fn SetCenterX();
    fn SetCenterX();
    fn SetCenterY();
    fn SetCenterY();
}
pub trait IDCompositionSurfaceImpl: Sized {
    fn BeginDraw();
    fn EndDraw();
    fn SuspendDraw();
    fn ResumeDraw();
    fn Scroll();
}
pub trait IDCompositionSurfaceFactoryImpl: Sized {
    fn CreateSurface();
    fn CreateVirtualSurface();
}
pub trait IDCompositionTableTransferEffectImpl: Sized + IDCompositionFilterEffectImpl + IDCompositionEffectImpl {
    fn SetRedTable();
    fn SetGreenTable();
    fn SetBlueTable();
    fn SetAlphaTable();
    fn SetRedDisable();
    fn SetGreenDisable();
    fn SetBlueDisable();
    fn SetAlphaDisable();
    fn SetClampOutput();
    fn SetRedTableValue();
    fn SetRedTableValue();
    fn SetGreenTableValue();
    fn SetGreenTableValue();
    fn SetBlueTableValue();
    fn SetBlueTableValue();
    fn SetAlphaTableValue();
    fn SetAlphaTableValue();
}
pub trait IDCompositionTargetImpl: Sized {
    fn SetRoot();
}
pub trait IDCompositionTransformImpl: Sized + IDCompositionTransform3DImpl + IDCompositionEffectImpl {}
pub trait IDCompositionTransform3DImpl: Sized + IDCompositionEffectImpl {}
pub trait IDCompositionTranslateTransformImpl: Sized + IDCompositionTransformImpl + IDCompositionTransform3DImpl + IDCompositionEffectImpl {
    fn SetOffsetX();
    fn SetOffsetX();
    fn SetOffsetY();
    fn SetOffsetY();
}
pub trait IDCompositionTranslateTransform3DImpl: Sized + IDCompositionTransform3DImpl + IDCompositionEffectImpl {
    fn SetOffsetX();
    fn SetOffsetX();
    fn SetOffsetY();
    fn SetOffsetY();
    fn SetOffsetZ();
    fn SetOffsetZ();
}
pub trait IDCompositionTurbulenceEffectImpl: Sized + IDCompositionFilterEffectImpl + IDCompositionEffectImpl {
    fn SetOffset();
    fn SetBaseFrequency();
    fn SetSize();
    fn SetNumOctaves();
    fn SetSeed();
    fn SetNoise();
    fn SetStitchable();
}
pub trait IDCompositionVirtualSurfaceImpl: Sized + IDCompositionSurfaceImpl {
    fn Resize();
    fn Trim();
}
pub trait IDCompositionVisualImpl: Sized {
    fn SetOffsetX();
    fn SetOffsetX();
    fn SetOffsetY();
    fn SetOffsetY();
    fn SetTransform();
    fn SetTransform();
    fn SetTransformParent();
    fn SetEffect();
    fn SetBitmapInterpolationMode();
    fn SetBorderMode();
    fn SetClip();
    fn SetClip();
    fn SetContent();
    fn AddVisual();
    fn RemoveVisual();
    fn RemoveAllVisuals();
    fn SetCompositeMode();
}
pub trait IDCompositionVisual2Impl: Sized + IDCompositionVisualImpl {
    fn SetOpacityMode();
    fn SetBackFaceVisibility();
}
pub trait IDCompositionVisual3Impl: Sized + IDCompositionVisualDebugImpl + IDCompositionVisual2Impl + IDCompositionVisualImpl {
    fn SetDepthMode();
    fn SetOffsetZ();
    fn SetOffsetZ();
    fn SetOpacity();
    fn SetOpacity();
    fn SetTransform();
    fn SetTransform();
    fn SetVisible();
}
pub trait IDCompositionVisualDebugImpl: Sized + IDCompositionVisual2Impl + IDCompositionVisualImpl {
    fn EnableHeatMap();
    fn DisableHeatMap();
    fn EnableRedrawRegions();
    fn DisableRedrawRegions();
}
