#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct Ellipse(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEllipse(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILine(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILineStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPath(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPathFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPathStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPolygon(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPolygonStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPolyline(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPolylineStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRectangle(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRectangleStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShape(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShape2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShapeFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShapeStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Line(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Path(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Polygon(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Polyline(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Rectangle(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Shape(pub *mut ::core::ffi::c_void);
