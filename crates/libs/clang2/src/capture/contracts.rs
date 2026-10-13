use super::*;

mod source;

pub(super) struct Index<'a> {
    unit: CXTranslationUnit,
    bridge: Option<&'a layout::Bridge>,
    macros: BTreeMap<(String, u32), Vec<Expansion>>,
    family_files: BTreeSet<String>,
    family_names: BTreeSet<String>,
    files: std::cell::RefCell<BTreeMap<String, Vec<Token>>>,
    records: Vec<CXCursor>,
}

struct Expansion {
    name: String,
    definition: CXCursor,
    location: CXSourceLocation,
    end: u32,
}

#[derive(Clone)]
struct Token {
    text: String,
    kind: i32,
    location: Location,
    end: u32,
}

impl<'a> Index<'a> {
    pub fn new(unit: CXTranslationUnit, bridge: Option<&'a layout::Bridge>) -> Result<Self> {
        let mut macros = BTreeMap::<_, Vec<Expansion>>::new();
        let children = children(unsafe { clang_getTranslationUnitCursor(unit) });
        let definitions = children
            .iter()
            .copied()
            .filter(|cursor| unsafe { clang_getCursorKind(*cursor) } == CXCursor_MacroDefinition)
            .collect::<Vec<_>>();
        let family_files = definitions
            .iter()
            .filter(|cursor| {
                matches!(
                    text(unsafe { clang_getCursorSpelling(**cursor) }).as_str(),
                    "_SAL_VERSION" | "_SAL1_Source_" | "_SAL2_Source_" | "__allowed"
                )
            })
            .map(|cursor| locations(*cursor).1.file)
            .collect::<BTreeSet<_>>();
        let family_names = definitions
            .iter()
            .filter(|cursor| family_files.contains(&locations(**cursor).1.file))
            .map(|cursor| text(unsafe { clang_getCursorSpelling(*cursor) }))
            .collect();
        for cursor in children {
            if unsafe { clang_getCursorKind(cursor) } != CXCursor_MacroExpansion {
                continue;
            }
            let definition = unsafe { clang_getCursorReferenced(cursor) };
            if unsafe { clang_Cursor_isNull(definition) } != 0 {
                continue;
            }
            let location = locations(cursor).0;
            let mut end = 0;
            unsafe {
                clang_getExpansionLocation(
                    clang_getRangeEnd(clang_getCursorExtent(cursor)),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    &raw mut end,
                );
            }
            let expansion = Expansion {
                name: text(unsafe { clang_getCursorSpelling(cursor) }),
                definition,
                location: unsafe { clang_getCursorLocation(cursor) },
                end,
            };
            macros
                .entry((location.file, location.offset))
                .or_default()
                .push(expansion);
        }
        let mut pending = vec![unsafe { clang_getTranslationUnitCursor(unit) }];
        let mut records = Vec::new();
        while let Some(cursor) = pending.pop() {
            let kind = unsafe { clang_getCursorKind(cursor) };
            if matches!(kind, CXCursor_StructDecl | CXCursor_ClassDecl)
                && unsafe { clang_isCursorDefinition(cursor) } != 0
            {
                records.push(cursor);
            }
            if matches!(
                kind,
                CXCursor_TranslationUnit
                    | CXCursor_Namespace
                    | CXCursor_LinkageSpec
                    | CXCursor_UnexposedDecl
                    | CXCursor_StructDecl
                    | CXCursor_ClassDecl
            ) {
                pending.extend(super::children(cursor));
            }
        }
        let mut index = Self {
            unit,
            bridge,
            macros,
            family_files,
            family_names,
            files: Default::default(),
            records,
        };
        if bridge.is_none() {
            loop {
                let mut indirect = BTreeSet::new();
                for definition in &definitions {
                    let name = text(unsafe { clang_getCursorSpelling(*definition) });
                    if !index.family_names.contains(&name)
                        && index
                            .tokenize(unsafe { clang_getCursorExtent(*definition) })
                            .iter()
                            .skip(1)
                            .any(|token| index.family_names.contains(&token.text))
                    {
                        indirect.insert(name);
                    }
                }
                if indirect.is_empty() {
                    break;
                }
                index.family_names.extend(indirect);
            }
        }
        Ok(index)
    }

    pub fn function(
        &self,
        cursor: CXCursor,
        params: &mut [NativeParameter],
    ) -> Result<Vec<AnnotationEvidence>> {
        let tokens = self.declaration_tokens(cursor)?;
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
        let mut annotations = self.annotations(&tokens[..name], &names, cursor)?;
        for annotation in &mut annotations {
            if annotation.source == "SAL" {
                let payload = annotation
                    .resolution
                    .as_ref()
                    .map_or(annotation.payload.as_str(), |value| value.payload.as_str());
                if payload != "_Check_return_" {
                    return Err(error(cursor, "unsupported return SAL contract"));
                }
                annotation.source = "SAL-return".into();
            }
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
            let expansion = self.expansion(&key, cursor)?;
            let resolution = expansion
                .map(|expansion| self.resolve(expansion, cursor))
                .transpose()?
                .flatten();
            let (source, payload, arguments, end) = if resolution.is_some() {
                let expansion = expansion.unwrap();
                if expansion.name != token.text {
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
                let original = tokens[position..end]
                    .iter()
                    .map(|token| token.text.as_str())
                    .collect::<Vec<_>>()
                    .join(" ");
                let arguments = tokens[position + 1..end]
                    .iter()
                    .map(|token| token.text.clone())
                    .collect::<Vec<_>>();
                ("SAL", original, arguments, end)
            } else if token.kind == CXToken_Comment
                && token
                    .text
                    .strip_prefix("/*")
                    .is_some_and(|body| body.trim_start().starts_with('['))
            {
                if midl(&token.text).is_none() {
                    return Err(error(cursor, "malformed consumed MIDL contract"));
                }
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
                resolution: resolution.and_then(|mut value| {
                    let expansion = expansion.unwrap();
                    if value.payload == expansion.name {
                        return None;
                    }
                    let suffix = tokens[position + 1..end]
                        .iter()
                        .map(|token| token.text.as_str())
                        .collect::<Vec<_>>()
                        .join(" ");
                    if !suffix.is_empty() {
                        value.payload.push(' ');
                        value.payload.push_str(&suffix);
                    }
                    Some(value)
                }),
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
            let payload = annotation
                .resolution
                .as_ref()
                .map_or(annotation.payload.as_str(), |value| value.payload.as_str());
            let name = payload.split_whitespace().next().unwrap();
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
                    let content = payload
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
