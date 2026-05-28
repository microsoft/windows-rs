#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct InnerConstraints {
    pub min_width: Option<f64>,
    pub min_height: Option<f64>,
    pub max_width: Option<f64>,
    pub max_height: Option<f64>,
}
