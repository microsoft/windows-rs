#[cfg(feature = "implement_exclusive")]
pub trait IEllipseImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ILineImpl: Sized {
    fn X1(&self) -> ::windows::core::Result<f64>;
    fn SetX1(&self, value: f64) -> ::windows::core::Result<()>;
    fn Y1(&self) -> ::windows::core::Result<f64>;
    fn SetY1(&self, value: f64) -> ::windows::core::Result<()>;
    fn X2(&self) -> ::windows::core::Result<f64>;
    fn SetX2(&self, value: f64) -> ::windows::core::Result<()>;
    fn Y2(&self) -> ::windows::core::Result<f64>;
    fn SetY2(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineStaticsImpl: Sized {
    fn X1Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn Y1Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn X2Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn Y2Property(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPathImpl: Sized {
    fn Data(&self) -> ::windows::core::Result<super::Media::Geometry>;
    fn SetData(&self, value: &::core::option::Option<super::Media::Geometry>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPathFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Path>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPathStaticsImpl: Sized {
    fn DataProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPolygonImpl: Sized {
    fn FillRule(&self) -> ::windows::core::Result<super::Media::FillRule>;
    fn SetFillRule(&self, value: super::Media::FillRule) -> ::windows::core::Result<()>;
    fn Points(&self) -> ::windows::core::Result<super::Media::PointCollection>;
    fn SetPoints(&self, value: &::core::option::Option<super::Media::PointCollection>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPolygonStaticsImpl: Sized {
    fn FillRuleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PointsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPolylineImpl: Sized {
    fn FillRule(&self) -> ::windows::core::Result<super::Media::FillRule>;
    fn SetFillRule(&self, value: super::Media::FillRule) -> ::windows::core::Result<()>;
    fn Points(&self) -> ::windows::core::Result<super::Media::PointCollection>;
    fn SetPoints(&self, value: &::core::option::Option<super::Media::PointCollection>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPolylineStaticsImpl: Sized {
    fn FillRuleProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn PointsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRectangleImpl: Sized {
    fn RadiusX(&self) -> ::windows::core::Result<f64>;
    fn SetRadiusX(&self, value: f64) -> ::windows::core::Result<()>;
    fn RadiusY(&self) -> ::windows::core::Result<f64>;
    fn SetRadiusY(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRectangleStaticsImpl: Sized {
    fn RadiusXProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn RadiusYProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IShapeImpl: Sized {
    fn Fill(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetFill(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn Stroke(&self) -> ::windows::core::Result<super::Media::Brush>;
    fn SetStroke(&self, value: &::core::option::Option<super::Media::Brush>) -> ::windows::core::Result<()>;
    fn StrokeMiterLimit(&self) -> ::windows::core::Result<f64>;
    fn SetStrokeMiterLimit(&self, value: f64) -> ::windows::core::Result<()>;
    fn StrokeThickness(&self) -> ::windows::core::Result<f64>;
    fn SetStrokeThickness(&self, value: f64) -> ::windows::core::Result<()>;
    fn StrokeStartLineCap(&self) -> ::windows::core::Result<super::Media::PenLineCap>;
    fn SetStrokeStartLineCap(&self, value: super::Media::PenLineCap) -> ::windows::core::Result<()>;
    fn StrokeEndLineCap(&self) -> ::windows::core::Result<super::Media::PenLineCap>;
    fn SetStrokeEndLineCap(&self, value: super::Media::PenLineCap) -> ::windows::core::Result<()>;
    fn StrokeLineJoin(&self) -> ::windows::core::Result<super::Media::PenLineJoin>;
    fn SetStrokeLineJoin(&self, value: super::Media::PenLineJoin) -> ::windows::core::Result<()>;
    fn StrokeDashOffset(&self) -> ::windows::core::Result<f64>;
    fn SetStrokeDashOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn StrokeDashCap(&self) -> ::windows::core::Result<super::Media::PenLineCap>;
    fn SetStrokeDashCap(&self, value: super::Media::PenLineCap) -> ::windows::core::Result<()>;
    fn StrokeDashArray(&self) -> ::windows::core::Result<super::Media::DoubleCollection>;
    fn SetStrokeDashArray(&self, value: &::core::option::Option<super::Media::DoubleCollection>) -> ::windows::core::Result<()>;
    fn Stretch(&self) -> ::windows::core::Result<super::Media::Stretch>;
    fn SetStretch(&self, value: super::Media::Stretch) -> ::windows::core::Result<()>;
    fn GeometryTransform(&self) -> ::windows::core::Result<super::Media::Transform>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IShape2Impl: Sized {
    fn GetAlphaMask(&self) -> ::windows::core::Result<super::super::Composition::CompositionBrush>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IShapeFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Shape>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IShapeStaticsImpl: Sized {
    fn FillProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StrokeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StrokeMiterLimitProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StrokeThicknessProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StrokeStartLineCapProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StrokeEndLineCapProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StrokeLineJoinProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StrokeDashOffsetProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StrokeDashCapProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StrokeDashArrayProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn StretchProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
