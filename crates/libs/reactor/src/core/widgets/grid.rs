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
        let mut out = Vec::with_capacity(4);
        if !self.rows.is_empty() {
            out.push(Binding::Prop(
                Prop::GridRows,
                PropValue::GridLengths(self.rows.clone()),
            ));
        }
        if !self.columns.is_empty() {
            out.push(Binding::Prop(
                Prop::GridColumns,
                PropValue::GridLengths(self.columns.clone()),
            ));
        }
        if let Some(v) = self.row_spacing {
            out.push(Binding::Prop(Prop::GridRowSpacing, PropValue::F64(v)));
        }
        if let Some(v) = self.column_spacing {
            out.push(Binding::Prop(Prop::GridColumnSpacing, PropValue::F64(v)));
        }
        out
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
