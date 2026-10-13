use super::*;

pub(super) struct Index {
    unit: CXTranslationUnit,
    macros: BTreeMap<(String, u32), String>,
}

struct Token {
    text: String,
    kind: i32,
    location: Location,
}

impl Index {
    pub fn new(unit: CXTranslationUnit) -> Result<Self> {
        let mut macros = BTreeMap::new();
        for cursor in children(unsafe { clang_getTranslationUnitCursor(unit) }) {
            if unsafe { clang_getCursorKind(cursor) } != CXCursor_MacroExpansion {
                continue;
            }
            let definition = unsafe { clang_getCursorReferenced(cursor) };
            if unsafe { clang_Cursor_isNull(definition) } != 0 {
                continue;
            }
            let location = locations(definition).1;
            let filename = std::path::Path::new(&location.file)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("");
            if matches!(
                filename,
                "sal.h"
                    | "rpcsal.h"
                    | "specstrings.h"
                    | "specstrings_strict.h"
                    | "specstrings_supp.h"
                    | "concurrencysal.h"
            ) {
                let location = locations(cursor).0;
                let name = text(unsafe { clang_getCursorSpelling(cursor) });
                if macros
                    .insert((location.file, location.offset), name)
                    .is_some()
                {
                    return Err(error(cursor, "ambiguous annotation macro expansion"));
                }
            }
        }
        Ok(Self { unit, macros })
    }

    pub fn function(
        &self,
        cursor: CXCursor,
        params: &mut [NativeParameter],
    ) -> Result<Vec<AnnotationEvidence>> {
        let mut native = std::ptr::null_mut();
        let mut count = 0;
        unsafe {
            let range = clang_getCursorExtent(cursor);
            let physical = |location| {
                let mut file = std::ptr::null_mut();
                let mut offset = 0;
                clang_getExpansionLocation(
                    location,
                    &raw mut file,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    &raw mut offset,
                );
                clang_getLocationForOffset(self.unit, file, offset)
            };
            clang_tokenize(
                self.unit,
                clang_getRange(
                    physical(clang_getRangeStart(range)),
                    physical(clang_getRangeEnd(range)),
                ),
                &raw mut native,
                &raw mut count,
            );
        };
        let tokens = if count == 0 {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(native, count as usize) }
                .iter()
                .map(|token| {
                    let mut file = std::ptr::null_mut();
                    let mut line = 0;
                    let mut column = 0;
                    let mut offset = 0;
                    unsafe {
                        clang_getSpellingLocation(
                            clang_getTokenLocation(self.unit, *token),
                            &raw mut file,
                            &raw mut line,
                            &raw mut column,
                            &raw mut offset,
                        );
                    };
                    Token {
                        text: text(unsafe { clang_getTokenSpelling(self.unit, *token) }),
                        kind: unsafe { clang_getTokenKind(*token) },
                        location: Location {
                            file: text(unsafe { clang_getFileName(file) }),
                            line,
                            column,
                            offset,
                        },
                    }
                })
                .collect::<Vec<_>>()
        };
        if !native.is_null() {
            unsafe { clang_disposeTokens(self.unit, native, count) };
        }
        let location = locations(cursor).0;
        let symbol = text(unsafe { clang_getCursorSpelling(cursor) });
        let name = tokens
            .iter()
            .position(|token| token.text == symbol && token.location == location)
            .ok_or_else(|| {
                error(
                    cursor,
                    "callable contracts require a physical source name anchor",
                )
            })?;
        let open = name + 1;
        if tokens.get(open).is_none_or(|token| token.text != "(") {
            return Err(error(
                cursor,
                "callable contracts lack an anchored parameter list",
            ));
        }
        let mut depth = 0;
        let mut start = open + 1;
        let mut arguments = Vec::new();
        let mut close = None;
        for (position, token) in tokens.iter().enumerate().skip(start) {
            if token.kind == CXToken_Comment {
                continue;
            }
            match token.text.as_str() {
                "(" | "[" | "{" => depth += 1,
                ")" if depth == 0 => {
                    arguments.push(&tokens[start..position]);
                    close = Some(position);
                    break;
                }
                ")" | "]" | "}" => depth -= 1,
                "," if depth == 0 => {
                    arguments.push(&tokens[start..position]);
                    start = position + 1;
                }
                _ => {}
            }
            if depth < 0 {
                return Err(error(cursor, "unbalanced source parameter context"));
            }
        }
        if close.is_none() {
            return Err(error(cursor, "unterminated source parameter context"));
        }
        if unsafe { clang_Cursor_isVariadic(cursor) } != 0
            && arguments.last().is_some_and(|tokens| {
                tokens
                    .iter()
                    .filter(|token| token.kind != CXToken_Comment)
                    .map(|token| token.text.as_str())
                    .eq(["..."])
            })
        {
            arguments.pop();
        }
        if params.is_empty() && !arguments.is_empty() {
            if arguments.len() != 1
                || arguments[0]
                    .iter()
                    .filter(|token| token.kind != CXToken_Comment)
                    .any(|token| token.text != "void")
            {
                return Err(error(
                    cursor,
                    "zero-parameter source context disagrees with compiler",
                ));
            }
            arguments.clear();
        }
        if arguments.len() != params.len() {
            return Err(error(
                cursor,
                "source parameter positions disagree with compiler",
            ));
        }
        let names = params
            .iter()
            .map(|param| param.evidence.name.clone())
            .collect::<Vec<_>>();
        for (ordinal, (param, tokens)) in params.iter_mut().zip(arguments).enumerate() {
            let parameter = unsafe { clang_Cursor_getArgument(cursor, ordinal as u32) };
            let mut file = std::ptr::null_mut();
            let mut offset = 0;
            unsafe {
                clang_getExpansionLocation(
                    clang_getRangeStart(clang_getCursorExtent(parameter)),
                    &raw mut file,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    &raw mut offset,
                );
            };
            let filename = text(unsafe { clang_getFileName(file) });
            if !tokens
                .iter()
                .any(|token| token.location.file == filename && token.location.offset == offset)
            {
                return Err(error(
                    cursor,
                    "parameter location is outside its original source context",
                ));
            }
            let annotations = self.annotations(tokens, &names, cursor)?;
            param.evidence.contract = lower(&annotations, &names, cursor)?;
            param.evidence.contract.annotations = annotations;
            if param.evidence.ordinal != ordinal {
                return Err(error(cursor, "parameter ordinal mismatch"));
            }
        }
        for param in params.iter() {
            for extent in &param.evidence.contract.extents {
                let ty = &params[extent.parameter].canonical;
                let kind = if extent.dereference {
                    let NativeKind::Pointer(value) = &ty.kind else {
                        return Err(error(
                            cursor,
                            "dereferenced extent does not name a pointer parameter",
                        ));
                    };
                    &value.kind
                } else {
                    &ty.kind
                };
                if !matches!(
                    kind,
                    NativeKind::Scalar(
                        CXType_UChar
                            | CXType_SChar
                            | CXType_Short
                            | CXType_UShort
                            | CXType_Int
                            | CXType_UInt
                            | CXType_Long
                            | CXType_ULong
                            | CXType_LongLong
                            | CXType_ULongLong
                    )
                ) {
                    return Err(error(
                        cursor,
                        "buffer extent does not name an integer value",
                    ));
                }
                if !matches!(param.canonical.kind, NativeKind::Pointer(_)) {
                    return Err(error(
                        cursor,
                        "buffer contract requires a pointer parameter",
                    ));
                }
            }
        }
        let annotations = self.annotations(&tokens[..name], &names, cursor)?;
        if annotations
            .iter()
            .any(|annotation| annotation.source == "SAL")
        {
            return Err(error(
                cursor,
                "return SAL contracts require a supported return model",
            ));
        }
        Ok(annotations)
    }

    fn annotations(
        &self,
        tokens: &[Token],
        names: &[String],
        cursor: CXCursor,
    ) -> Result<Vec<AnnotationEvidence>> {
        let mut result: Vec<AnnotationEvidence> = Vec::new();
        let mut position = 0;
        while position < tokens.len() {
            let token = &tokens[position];
            let key = (token.location.file.clone(), token.location.offset);
            let (source, payload, arguments, end) = if let Some(name) = self.macros.get(&key) {
                if name != &token.text {
                    return Err(error(
                        cursor,
                        "annotation expansion/token identity disagrees",
                    ));
                }
                let mut end = position + 1;
                if tokens.get(end).is_some_and(|token| token.text == "(") {
                    let mut depth = 0;
                    loop {
                        let token = tokens
                            .get(end)
                            .ok_or_else(|| error(cursor, "unterminated annotation invocation"))?;
                        match token.text.as_str() {
                            "(" => depth += 1,
                            ")" => depth -= 1,
                            _ => {}
                        }
                        end += 1;
                        if depth == 0 {
                            break;
                        }
                    }
                }
                (
                    "SAL",
                    tokens[position..end]
                        .iter()
                        .map(|token| token.text.as_str())
                        .collect::<Vec<_>>()
                        .join(" "),
                    tokens[position + 1..end]
                        .iter()
                        .map(|token| token.text.clone())
                        .collect::<Vec<_>>(),
                    end,
                )
            } else if token.kind == CXToken_Comment && midl(&token.text).is_some() {
                ("MIDL", token.text.clone(), Vec::new(), position + 1)
            } else {
                position += 1;
                continue;
            };
            let mut bindings = BTreeSet::new();
            for identifier in arguments {
                let matches = names
                    .iter()
                    .enumerate()
                    .filter(|(_, name)| **name == identifier)
                    .collect::<Vec<_>>();
                if let [(ordinal, _)] = matches.as_slice() {
                    bindings.insert((identifier, *ordinal));
                } else if matches.len() > 1 {
                    return Err(error(
                        cursor,
                        "annotation identifier has ambiguous positional binding",
                    ));
                }
            }
            result.push(AnnotationEvidence {
                source: source.into(),
                ordinal: result
                    .iter()
                    .filter(|annotation| annotation.source == source)
                    .count(),
                payload,
                location: token.location.clone(),
                bindings: bindings.into_iter().collect(),
            });
            position = end;
        }
        Ok(result)
    }
}

fn midl(comment: &str) -> Option<Vec<&str>> {
    let mut body = comment.strip_prefix("/*")?.strip_suffix("*/")?.trim();
    if !body.starts_with('[') {
        return None;
    }
    let mut groups = Vec::new();
    while !body.is_empty() {
        let tail = body.strip_prefix('[')?;
        let end = tail.find(']')?;
        groups.push(tail[..end].trim());
        body = tail[end + 1..].trim();
    }
    Some(groups)
}

fn lower(
    annotations: &[AnnotationEvidence],
    names: &[String],
    cursor: CXCursor,
) -> Result<ParameterContract> {
    let mut result = ParameterContract::default();
    let mut sal = None;
    let mut midl_direction = None;
    for annotation in annotations {
        let direction = if annotation.source == "MIDL" {
            let mut input = false;
            let mut output = false;
            for group in midl(&annotation.payload).unwrap() {
                for tag in group.split(',').map(str::trim) {
                    match tag {
                        "in" => input = true,
                        "out" => output = true,
                        "optional" => result.optional = true,
                        "annotation" | "iid_is" | "local" => {}
                        _ => {
                            return Err(error(
                                cursor,
                                &format!("unsupported consumed MIDL contract `{tag}`"),
                            ));
                        }
                    }
                }
            }
            let direction = match (input, output) {
                (true, true) => Some(Direction::InOut),
                (true, false) => Some(Direction::In),
                (false, true) => Some(Direction::Out),
                _ => None,
            };
            if let Some(direction) = direction {
                if midl_direction.is_some_and(|previous| previous != direction) {
                    return Err(error(cursor, "MIDL direction contracts disagree"));
                }
                midl_direction = Some(direction);
            }
            continue;
        } else {
            let name = annotation.payload.split_whitespace().next().unwrap();
            match name {
                "_In_" => Direction::In,
                "_In_opt_" => {
                    result.optional = true;
                    Direction::In
                }
                "_Out_" => Direction::Out,
                "_Out_opt_" => {
                    result.optional = true;
                    Direction::Out
                }
                "_Inout_" => Direction::InOut,
                "_COM_Outptr_" => {
                    result.com_outptr = true;
                    Direction::Out
                }
                "_In_reads_bytes_" | "_Out_writes_bytes_to_" => {
                    let content = annotation
                        .payload
                        .split_once('(')
                        .unwrap()
                        .1
                        .strip_suffix(')')
                        .unwrap();
                    let expressions = content.split(',').map(str::trim).collect::<Vec<_>>();
                    let expected = if name == "_In_reads_bytes_" { 1 } else { 2 };
                    if expressions.len() != expected {
                        return Err(error(cursor, "buffer annotation arity disagrees"));
                    }
                    for (position, expression) in expressions.iter().enumerate() {
                        let (dereference, identifier) = expression
                            .strip_prefix('*')
                            .map_or((false, *expression), |value| (true, value.trim()));
                        let positions = names
                            .iter()
                            .enumerate()
                            .filter(|(_, name)| name.as_str() == identifier)
                            .map(|(ordinal, _)| ordinal)
                            .collect::<Vec<_>>();
                        let [parameter] = positions.as_slice() else {
                            return Err(error(
                                cursor,
                                &format!("unsupported or unbound buffer expression `{expression}`"),
                            ));
                        };
                        result.extents.push(BufferExtent {
                            phase: if position == 0 { "capacity" } else { "valid" }.into(),
                            units: "bytes".into(),
                            parameter: *parameter,
                            dereference,
                        });
                    }
                    if name == "_In_reads_bytes_" {
                        Direction::In
                    } else {
                        Direction::Out
                    }
                }
                _ => {
                    return Err(error(
                        cursor,
                        &format!("unsupported consumed SAL contract `{name}`"),
                    ));
                }
            }
        };
        if sal.is_some_and(|previous| previous != direction) {
            return Err(error(cursor, "SAL direction contracts disagree"));
        }
        sal = Some(direction);
    }
    if sal.is_some() && midl_direction.is_some() && sal != midl_direction {
        return Err(error(cursor, "SAL and MIDL directions disagree"));
    }
    result.direction = sal.or(midl_direction);
    Ok(result)
}
