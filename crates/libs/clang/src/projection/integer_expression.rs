use super::*;
use windows_metadata::{IntegerExpression, IntegerLiteral, IntegerReference};

pub(super) fn parse(
    text: &str,
    parameters: &[Parameter],
    snapshot: &Snapshot,
) -> Option<IntegerExpression> {
    let names: Vec<_> = parameters.iter().map(|value| value.name.as_str()).collect();
    let value = project(&sal::Expression::parse(text, &names)?)?;
    for reference in value.references()? {
        let (IntegerReference::Parameter(index) | IntegerReference::Dereference(index)) = reference
        else {
            return None;
        };
        let mut ty = underlying(snapshot, parameters.get(*index as usize)?.written);
        if matches!(reference, IntegerReference::Dereference(_)) {
            let TypeKind::Pointer(inner) = snapshot.types[ty.0].kind else {
                return None;
            };
            ty = underlying(snapshot, inner);
        }
        if !integer(snapshot, ty) {
            return None;
        }
    }
    value.is_valid().then_some(value)
}

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

pub(super) fn range(
    value: &sal::Annotation<'_>,
    parameter: &Parameter,
    parameters: &[Parameter],
    snapshot: &Snapshot,
) -> Option<windows_metadata::IntegerRange> {
    use windows_metadata::BufferPhase;
    let value = value.unscoped()?;
    let (phase, indirect) = match (value.version.as_str(), value.name.as_str()) {
        ("2", "_In_range_") | ("1.2", "__RPC__in_range") => (BufferPhase::Entry, false),
        ("2", "_Deref_in_range_") => (BufferPhase::Entry, true),
        ("2", "_Deref_out_range_") => (BufferPhase::Success, true),
        _ => return None,
    };
    let mut ty = underlying(snapshot, parameter.written);
    if indirect {
        let TypeKind::Pointer(inner) = snapshot.types[ty.0].kind else {
            return None;
        };
        ty = inner;
    }
    if !integer(snapshot, ty) {
        return None;
    }
    bounds(value, phase, indirect, |text| {
        parse(text, parameters, snapshot)
    })
}

pub(super) fn field_range(
    value: &sal::Annotation<'_>,
    field: &Field,
    fields: &[Field],
    snapshot: &Snapshot,
) -> Option<windows_metadata::IntegerRange> {
    let value = value.unscoped()?;
    if value.version != "2"
        || value.name != "_Field_range_"
        || field.bit_width.is_some()
        || !integer(snapshot, field.canonical)
    {
        return None;
    }
    let names: Vec<_> = fields.iter().map(|field| field.name.as_str()).collect();
    bounds(value, windows_metadata::BufferPhase::Value, false, |text| {
        field_expression(&sal::Expression::parse(text, &names)?, fields, snapshot)
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

pub(super) fn return_range(
    value: &sal::Annotation<'_>,
    result: TypeId,
    parameters: &[Parameter],
    snapshot: &Snapshot,
) -> Option<windows_metadata::IntegerRange> {
    let value = value.unscoped()?;
    if value.version != "2" || value.name != "_Ret_range_" || !integer(snapshot, result) {
        return None;
    }
    bounds(
        value,
        windows_metadata::BufferPhase::Success,
        false,
        |text| parse(text, parameters, snapshot),
    )
}

fn bounds(
    value: &SalAnnotation,
    phase: windows_metadata::BufferPhase,
    indirect: bool,
    parse: impl Fn(&str) -> Option<IntegerExpression>,
) -> Option<windows_metadata::IntegerRange> {
    use windows_metadata::{IntegerBound, IntegerRange};
    let (lower, upper) = value.expanded.split_once(',')?;
    let bound = |text, exclusive| {
        Some(IntegerBound {
            expression: parse(text)?,
            exclusive,
        })
    };
    let mut result = IntegerRange {
        phase,
        indirect,
        lower: None,
        upper: None,
    };
    match lower.trim() {
        ">" | ">=" => result.lower = bound(upper, lower.trim() == ">"),
        "<" | "<=" => result.upper = bound(upper, lower.trim() == "<"),
        _ => {
            result.lower = Some(bound(lower, false)?);
            result.upper = Some(bound(upper, false)?);
        }
    }
    result.is_valid().then_some(result)
}
