#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct Ellipse(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Ellipse {}
impl ::core::clone::Clone for Ellipse {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEllipse(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEllipse {}
impl ::core::clone::Clone for IEllipse {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILine(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILine {}
impl ::core::clone::Clone for ILine {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILineStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILineStatics {}
impl ::core::clone::Clone for ILineStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPath(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPath {}
impl ::core::clone::Clone for IPath {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPathFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPathFactory {}
impl ::core::clone::Clone for IPathFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPathStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPathStatics {}
impl ::core::clone::Clone for IPathStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPolygon(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPolygon {}
impl ::core::clone::Clone for IPolygon {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPolygonStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPolygonStatics {}
impl ::core::clone::Clone for IPolygonStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPolyline(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPolyline {}
impl ::core::clone::Clone for IPolyline {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPolylineStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPolylineStatics {}
impl ::core::clone::Clone for IPolylineStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRectangle(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRectangle {}
impl ::core::clone::Clone for IRectangle {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRectangleStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRectangleStatics {}
impl ::core::clone::Clone for IRectangleStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IShape(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IShape {}
impl ::core::clone::Clone for IShape {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IShape2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IShape2 {}
impl ::core::clone::Clone for IShape2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IShapeFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IShapeFactory {}
impl ::core::clone::Clone for IShapeFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IShapeStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IShapeStatics {}
impl ::core::clone::Clone for IShapeStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Line(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Line {}
impl ::core::clone::Clone for Line {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Path(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Path {}
impl ::core::clone::Clone for Path {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Polygon(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Polygon {}
impl ::core::clone::Clone for Polygon {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Polyline(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Polyline {}
impl ::core::clone::Clone for Polyline {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Rectangle(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Rectangle {}
impl ::core::clone::Clone for Rectangle {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Shape(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Shape {}
impl ::core::clone::Clone for Shape {
    fn clone(&self) -> Self {
        *self
    }
}
