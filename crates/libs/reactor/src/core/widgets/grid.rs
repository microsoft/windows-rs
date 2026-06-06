use super::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Grid {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub rows: Vec<GridLength>,
    pub columns: Vec<GridLength>,
    pub row_spacing: Option<f64>,
    pub column_spacing: Option<f64>,
    pub children: Vec<Element>,
}

impl Widget for Grid {
    widget_header!(ControlKind::Grid);
    fn bindings(&self) -> PropBindings {
        crate::core::generated_bindings::grid_bindings(self)
    }
    fn children(&self) -> Children<'_> {
        Children::Keyed(&self.children)
    }
}

impl Grid {
    pub fn rows<I: IntoIterator<Item = GridLength>>(mut self, it: I) -> Self {
        self.rows = it.into_iter().collect();
        self
    }

    pub fn columns<I: IntoIterator<Item = GridLength>>(mut self, it: I) -> Self {
        self.columns = it.into_iter().collect();
        self
    }

    pub fn row_spacing(mut self, v: f64) -> Self {
        self.row_spacing = Some(v);
        self
    }

    pub fn column_spacing(mut self, v: f64) -> Self {
        self.column_spacing = Some(v);
        self
    }
}

pub fn grid(children: impl crate::core::into_elements::IntoElements) -> Grid {
    Grid {
        children: children.into_elements(),
        ..Grid::default()
    }
}
