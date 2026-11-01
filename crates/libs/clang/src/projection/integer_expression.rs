use super::*;
use windows_metadata::{IntegerExpression, IntegerLiteral, IntegerReference};

pub(super) fn project(value: &sal::Expression) -> Option<IntegerExpression> {
    project_with(value, &|index, dereference| {
        let index = index.try_into().ok()?;
        Some(if dereference {
            IntegerReference::Dereference(index)
        } else {
            IntegerReference::Parameter(index)
        })
    })
}

fn project_with(
    value: &sal::Expression,
    reference: &impl Fn(usize, bool) -> Option<IntegerReference>,
) -> Option<IntegerExpression> {
    Some(match value {
        sal::Expression::Constant(value) => IntegerExpression::Literal(IntegerLiteral::I32(*value)),
        sal::Expression::Parameter(index) => {
            IntegerExpression::Reference(reference(*index, false)?)
        }
        sal::Expression::Dereference(index) => {
            IntegerExpression::Reference(reference(*index, true)?)
        }
        sal::Expression::Add(left, right) => IntegerExpression::Add(
            Box::new(project_with(left, reference)?),
            Box::new(project_with(right, reference)?),
        ),
        sal::Expression::Subtract(left, right) => IntegerExpression::Subtract(
            Box::new(project_with(left, reference)?),
            Box::new(project_with(right, reference)?),
        ),
        sal::Expression::Multiply(left, right) => IntegerExpression::Multiply(
            Box::new(project_with(left, reference)?),
            Box::new(project_with(right, reference)?),
        ),
    })
}

fn underlying(snapshot: &Snapshot, mut ty: TypeId) -> TypeId {
    while let TypeKind::Alias(id) = snapshot.types[ty.0].kind {
        ty = snapshot.aliases[id.0].observations[0].canonical;
    }
    ty
}

pub(super) fn integer(snapshot: &Snapshot, ty: TypeId) -> bool {
    matches!(snapshot.types[underlying(snapshot, ty).0].kind,
        TypeKind::Scalar(ref value) if value.signed_integer().is_some())
}

/// Integer value ranges constrain argument values, not the projected signature, and metadata does
/// not publish them.
pub(super) fn range(value: &sal::Annotation<'_>) -> bool {
    value.unscoped().is_some_and(|value| {
        matches!(
            (value.version.as_str(), value.name.as_str()),
            ("2", "_In_range_" | "_Deref_in_range_" | "_Deref_out_range_")
                | ("1.2", "__RPC__in_range")
        )
    })
}

pub(super) fn field_expression(
    expression: &sal::Expression,
    fields: &[Field],
    snapshot: &Snapshot,
) -> Option<IntegerExpression> {
    project_with(expression, &|index, dereference| {
        let field = fields.get(index)?;
        (!dereference && field.bit_width.is_none() && integer(snapshot, field.canonical))
            .then(|| IntegerReference::Field(field.name.clone()))
    })
}
