//! Grid — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    g: &Xaml::Grid,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::GridRows, PropValue::GridLengths(rows)) => Some((|| {
            let defs = g.get_RowDefinitions()?;
            defs.cast::<windows_collections::IVector<Xaml::RowDefinition>>()?
                .Clear()?;
            for r in rows {
                let rd = Xaml::RowDefinition::new()?;
                rd.cast::<Xaml::IRowDefinition>()?
                    .put_Height(super::super::to_xaml_gridlength(*r)?)?;
                defs.cast::<windows_collections::IVector<Xaml::RowDefinition>>()?
                    .Append(&rd)?;
            }
            Ok(())
        })()),
        (Prop::GridColumns, PropValue::GridLengths(cols)) => Some((|| {
            let defs = g.get_ColumnDefinitions()?;
            defs.cast::<windows_collections::IVector<Xaml::ColumnDefinition>>()?
                .Clear()?;
            for c in cols {
                let cd = Xaml::ColumnDefinition::new()?;
                cd.cast::<Xaml::IColumnDefinition>()?
                    .put_Width(super::super::to_xaml_gridlength(*c)?)?;
                defs.cast::<windows_collections::IVector<Xaml::ColumnDefinition>>()?
                    .Append(&cd)?;
            }
            Ok(())
        })()),
        (Prop::GridRowSpacing, PropValue::F64(v)) => Some(g.put_RowSpacing(*v)),
        (Prop::GridRowSpacing, PropValue::Unset) => Some(g.put_RowSpacing(0.0)),
        (Prop::GridColumnSpacing, PropValue::F64(v)) => Some(g.put_ColumnSpacing(*v)),
        (Prop::GridColumnSpacing, PropValue::Unset) => Some(g.put_ColumnSpacing(0.0)),
        (Prop::Background, PropValue::Brush(br)) => Some((|| {
            g.cast::<Xaml::IPanel>()?
                .put_Background(&super::super::brush_of(br)?)
        })()),
        (Prop::Background, PropValue::Unset) => {
            Some((|| g.cast::<Xaml::IPanel>()?.put_Background(None))())
        }
        _ => None,
    }
}
