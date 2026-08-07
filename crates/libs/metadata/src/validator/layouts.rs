use super::*;

pub(super) fn validate(context: &mut Context<'_>) {
    let mut classes = HashMap::new();
    for layout in context.index.class_layouts() {
        let parent = layout.parent();
        if let Some(previous) = classes.insert(parent.row_id(), layout) {
            context.duplicate(
                layout.row_id(),
                previous.row_id(),
                format!(
                    "duplicate class layout for `{}.{}`",
                    parent.namespace(),
                    parent.name()
                ),
            );
        }

        let packing = layout.packing_size();
        if packing != 0 && (packing > 128 || !packing.is_power_of_two()) {
            context.invalid(
                layout.row_id(),
                Some(parent.row_id()),
                format!(
                    "class layout for `{}.{}` has invalid packing size {packing}",
                    parent.namespace(),
                    parent.name()
                ),
            );
        }

        let flags = parent.flags();
        if !flags.contains(crate::TypeAttributes::SequentialLayout)
            && !flags.contains(crate::TypeAttributes::ExplicitLayout)
        {
            context.invalid(
                layout.row_id(),
                Some(parent.row_id()),
                format!(
                    "class layout for `{}.{}` requires sequential or explicit layout",
                    parent.namespace(),
                    parent.name()
                ),
            );
        }
    }

    let mut fields = HashMap::new();
    for layout in context.index.field_layouts() {
        let field = layout.field();
        if let Some(previous) = fields.insert(field.row_id(), layout) {
            context.duplicate(
                layout.row_id(),
                previous.row_id(),
                format!("duplicate field layout for `{}`", field.name()),
            );
        }

        let parent = field.parent();
        if !parent
            .flags()
            .contains(crate::TypeAttributes::ExplicitLayout)
        {
            context.invalid(
                layout.row_id(),
                Some(field.row_id()),
                format!(
                    "field layout for `{}.{}.{}` requires explicit layout",
                    parent.namespace(),
                    parent.name(),
                    field.name()
                ),
            );
        }
    }
}
