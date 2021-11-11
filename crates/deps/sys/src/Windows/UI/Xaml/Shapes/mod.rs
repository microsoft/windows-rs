#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn Ellipse();
    fn IEllipse();
    fn ILine();
    fn ILineStatics();
    fn IPath();
    fn IPathFactory();
    fn IPathStatics();
    fn IPolygon();
    fn IPolygonStatics();
    fn IPolyline();
    fn IPolylineStatics();
    fn IRectangle();
    fn IRectangleStatics();
    fn IShape();
    fn IShape2();
    fn IShapeFactory();
    fn IShapeStatics();
    fn Line();
    fn Path();
    fn Polygon();
    fn Polyline();
    fn Rectangle();
    fn Shape();
}
