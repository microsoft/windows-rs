use super::*;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum SourceAnnotation {
    Identifier(String),
    Comment(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ParameterAnnotations {
    pub name: String,
    pub attributes: Vec<String>,
    pub source: Vec<SourceAnnotation>,
}

impl ParameterAnnotations {
    pub(super) fn capture(
        cursor: CXCursor,
        parameters: &[CXCursor],
        macros: &MacroDefinitions,
    ) -> Vec<Self> {
        let mut params: Vec<_> = parameters.iter()
            .map(|&child| Self {
                name: cx_string(unsafe { clang_getCursorSpelling(child) }),
                attributes: cursor_children(child)
                    .into_iter()
                    .filter(|attribute| unsafe { clang_getCursorKind(*attribute) } == CXCursor_AnnotateAttr)
                    .map(|attribute| cx_string(unsafe { clang_getCursorSpelling(attribute) }))
                    .collect(),
                source: vec![],
            })
            .collect();
        let tokens = cursor_tokens(cursor);
        let cursor_name = cx_string(unsafe { clang_getCursorSpelling(cursor) });
        let name_index = tokens
            .iter()
            .position(|(kind, token)| *kind == CXToken_Identifier && token == &cursor_name);
        let Some(open) = tokens
            .iter()
            .enumerate()
            .skip(name_index.map_or(0, |index| index + 1))
            .find(|(_, (kind, token))| *kind == CXToken_Punctuation && token == "(")
            .map(|(index, _)| index)
        else {
            return params;
        };
        let mut index = 0;
        let mut depth = 1;
        for (kind, token) in &tokens[open + 1..] {
            match (*kind, token.as_str()) {
                (CXToken_Punctuation, "(") => depth += 1,
                (CXToken_Punctuation, ")") => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                (CXToken_Punctuation, ",") if depth == 1 => index += 1,
                (CXToken_Identifier, identifier)
                    if depth == 1
                        && index < params.len()
                        && (identifier.starts_with("_COM_Outptr_")
                            || (matches!(identifier, "IN" | "OUT" | "OPTIONAL")
                                && macros.contains_key(identifier))) =>
                {
                    params[index]
                        .source
                        .push(SourceAnnotation::Identifier(token.clone()));
                }
                (CXToken_Comment, _) if depth == 1 && index < params.len() => {
                    params[index]
                        .source
                        .push(SourceAnnotation::Comment(token.clone()));
                }
                _ => {}
            }
        }
        params
    }

    // This summary is the existing RDL projection policy, not a native compatibility key.
    pub(super) fn project_attributes(&self, void_double_pointer: bool) -> ParamAnnotation {
        let mut result = ParamAnnotation::default();
        for annotation in &self.attributes {
            if annotation.starts_with("_In_") || annotation.starts_with("_Inout_") {
                result.input = true;
            }
            if annotation.starts_with("_Out_")
                || annotation.starts_with("_Outptr_")
                || annotation.starts_with("_COM_Outptr_")
                || annotation.starts_with("_Inout_")
            {
                result.output = true;
            }
            if annotation.contains("_opt_")
                || (annotation.starts_with("_Outptr_") && annotation.contains("_result_maybenull_"))
            {
                result.optional = true;
            }
            if annotation == "_Reserved_" {
                result.reserved = true;
            }
            if annotation.starts_with("_COM_Outptr_") {
                result.com_out_ptr = void_double_pointer;
            }
            let sal_name = annotation
                .split_once('(')
                .map_or(annotation.as_str(), |value| value.0);
            if sal_name.contains("_z_") || sal_name.ends_with("_z") {
                result.null_terminated = true;
            }
            if result.size.is_none()
                && (annotation.contains("_reads_")
                    || annotation.contains("_writes_")
                    || annotation.contains("_updates_"))
                && let Some(argument) = annotation
                    .split_once('(')
                    .and_then(|(_, rest)| rest.strip_suffix(')'))
                    .and_then(|arguments| arguments.split(',').next())
            {
                let argument = argument.trim();
                let value = if let Some(value) = parse_sal_integer(argument) {
                    SalSizeValue::Constant(value)
                } else if is_c_identifier(argument) {
                    SalSizeValue::Parameter(argument.to_string())
                } else if let Some(argument) = argument.strip_prefix('*').map(str::trim)
                    && is_c_identifier(argument)
                {
                    SalSizeValue::IndirectParameter(argument.to_string())
                } else {
                    SalSizeValue::Expression(argument.to_string())
                };
                result.size = Some(SalSize {
                    bytes: annotation.contains("_bytes"),
                    value,
                });
            }
        }
        result
    }

    pub(super) fn project_source(&self, result: &mut ParamAnnotation, void_double_pointer: bool) {
        for annotation in &self.source {
            match annotation {
                SourceAnnotation::Identifier(identifier) => {
                    if identifier.starts_with("_COM_Outptr_") {
                        result.output = true;
                        result.com_out_ptr |= void_double_pointer;
                        result.optional |= identifier.contains("_opt_");
                    } else {
                        match identifier.as_str() {
                            "IN" => result.input = true,
                            "OUT" => result.output = true,
                            "OPTIONAL" => result.optional = true,
                            _ => unreachable!(),
                        }
                    }
                }
                SourceAnnotation::Comment(comment) => {
                    result.input |= comment.contains("[in]");
                    result.output |= comment.contains("[out]");
                    result.optional |= comment.contains("[optional]");
                    result.retval |= comment.contains("[retval]");
                    result.com_out_ptr |= comment.contains("[iid_is]") && result.output;
                }
            }
        }
    }
}
