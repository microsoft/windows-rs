#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct Ellipse(i32);
pub struct IEllipse(pub *mut ::core::ffi::c_void);
pub struct ILine(pub *mut ::core::ffi::c_void);
pub struct ILineStatics(pub *mut ::core::ffi::c_void);
pub struct IPath(pub *mut ::core::ffi::c_void);
pub struct IPathFactory(pub *mut ::core::ffi::c_void);
pub struct IPathStatics(pub *mut ::core::ffi::c_void);
pub struct IPolygon(pub *mut ::core::ffi::c_void);
pub struct IPolygonStatics(pub *mut ::core::ffi::c_void);
pub struct IPolyline(pub *mut ::core::ffi::c_void);
pub struct IPolylineStatics(pub *mut ::core::ffi::c_void);
pub struct IRectangle(pub *mut ::core::ffi::c_void);
pub struct IRectangleStatics(pub *mut ::core::ffi::c_void);
pub struct IShape(pub *mut ::core::ffi::c_void);
pub struct IShape2(pub *mut ::core::ffi::c_void);
pub struct IShapeFactory(pub *mut ::core::ffi::c_void);
pub struct IShapeStatics(pub *mut ::core::ffi::c_void);
pub struct Line(i32);
pub struct Path(i32);
pub struct Polygon(i32);
pub struct Polyline(i32);
pub struct Rectangle(i32);
pub struct Shape(i32);
