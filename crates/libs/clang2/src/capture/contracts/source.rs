use super::*;

impl Index<'_> {
    pub(super) fn expansion(
        &self,
        key: &(String, u32),
        cursor: CXCursor,
    ) -> Result<Option<&Expansion>> {
        let Some(observations) = self.macros.get(key) else {
            return Ok(None);
        };
        let first = &observations[0];
        if observations.iter().any(|value| {
            value.name != first.name
                || value.end != first.end
                || unsafe { clang_equalLocations(value.location, first.location) } == 0
                || unsafe { clang_equalCursors(value.definition, first.definition) } == 0
        }) {
            return Err(error(
                cursor,
                "ambiguous consumed annotation macro expansion",
            ));
        }
        Ok(Some(first))
    }

    pub(super) fn resolve(
        &self,
        expansion: &Expansion,
        cursor: CXCursor,
    ) -> Result<Option<AnnotationResolution>> {
        if self.bridge.is_none() {
            if self
                .family_files
                .contains(&locations(expansion.definition).1.file)
            {
                return Ok(Some(AnnotationResolution {
                    payload: expansion.name.clone(),
                    definitions: Vec::new(),
                }));
            }
            if self.family_names.contains(&expansion.name) {
                return Err(error(
                    cursor,
                    "annotation wrapper requires compiler macro-history evidence",
                ));
            }
            return Ok(None);
        }
        let bridge = self.bridge.unwrap();
        let mut name = expansion.name.clone();
        let mut definitions = Vec::new();
        let mut visiting = BTreeSet::new();
        loop {
            if visiting.len() >= 64 || !visiting.insert(name.clone()) {
                return Err(error(
                    cursor,
                    "cyclic or excessive macro contract provenance",
                ));
            }
            let Some(definition) = bridge.macro_definition(self.unit, expansion.location, &name)?
            else {
                return Ok(None);
            };
            if definitions.is_empty() && definition.location != locations(expansion.definition).1 {
                return Err(error(
                    cursor,
                    "macro-history definition disagrees with preprocessing record",
                ));
            }
            if self.family_files.contains(&definition.location.file) {
                definitions.push(definition);
                return Ok(Some(AnnotationResolution {
                    payload: name,
                    definitions,
                }));
            }
            if !definition.function_like
                && definition.tokens.len() == 1
                && identifier(&definition.tokens[0])
            {
                name.clone_from(&definition.tokens[0]);
                definitions.push(definition);
                continue;
            }
            if self.contains_sal(expansion.location, &definition, &mut BTreeSet::new())? {
                return Err(error(
                    cursor,
                    "annotation wrapper transformation is unsupported",
                ));
            }
            return Ok(None);
        }
    }

    fn contains_sal(
        &self,
        location: CXSourceLocation,
        definition: &MacroDefinitionEvidence,
        visiting: &mut BTreeSet<String>,
    ) -> Result<bool> {
        if self.family_files.contains(&definition.location.file) {
            return Ok(true);
        }
        if !visiting.insert(definition.name.clone()) {
            return Ok(false);
        }
        if visiting.len() > 64 {
            return Err(Error::new(
                Stage::Capture,
                "macro contract dependency depth exceeds 64",
            ));
        }
        for token in &definition.tokens {
            if identifier(token)
                && let Some(value) = self
                    .bridge
                    .unwrap()
                    .macro_definition(self.unit, location, token)?
                && self.contains_sal(location, &value, visiting)?
            {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub(super) fn tokenize(&self, range: CXSourceRange) -> Vec<Token> {
        let mut native = std::ptr::null_mut();
        let mut count = 0;
        unsafe {
            clang_tokenize(self.unit, range, &raw mut native, &raw mut count);
        }
        let mut result = Vec::new();
        if !native.is_null() {
            for token in unsafe { std::slice::from_raw_parts(native, count as usize) } {
                let mut file = std::ptr::null_mut();
                let mut line = 0;
                let mut column = 0;
                let mut offset = 0;
                let mut end = 0;
                unsafe {
                    clang_getSpellingLocation(
                        clang_getTokenLocation(self.unit, *token),
                        &raw mut file,
                        &raw mut line,
                        &raw mut column,
                        &raw mut offset,
                    );
                    clang_getSpellingLocation(
                        clang_getRangeEnd(clang_getTokenExtent(self.unit, *token)),
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        &raw mut end,
                    );
                }
                result.push(Token {
                    text: text(unsafe { clang_getTokenSpelling(self.unit, *token) }),
                    kind: unsafe { clang_getTokenKind(*token) },
                    end,
                    location: Location {
                        file: text(unsafe { clang_getFileName(file) }),
                        line,
                        column,
                        offset,
                    },
                });
            }
            unsafe {
                clang_disposeTokens(self.unit, native, count);
            }
        }
        result
    }

    fn file_tokens(&self, filename: &str) -> Result<Vec<Token>> {
        if let Some(tokens) = self.files.borrow().get(filename) {
            return Ok(tokens.clone());
        }
        let name = CString::new(filename)
            .map_err(|_| Error::new(Stage::Capture, "source file contains NUL"))?;
        let file = unsafe { clang_getFile(self.unit, name.as_ptr()) };
        if file.is_null() {
            return Err(Error::new(
                Stage::Capture,
                "compiler source file unavailable",
            ));
        }
        let mut size = 0;
        let contents = unsafe { clang_getFileContents(self.unit, file, &raw mut size) };
        if contents.is_null() {
            return Err(Error::new(
                Stage::Capture,
                "compiler source buffer unavailable",
            ));
        }
        let size = u32::try_from(size).map_err(|_| {
            Error::new(
                Stage::Capture,
                "source buffer exceeds compiler offset range",
            )
        })?;
        let range = unsafe {
            clang_getRange(
                clang_getLocationForOffset(self.unit, file, 0),
                clang_getLocationForOffset(self.unit, file, size),
            )
        };
        let mut tokens = self.tokenize(range);
        let skipped = unsafe { clang_getSkippedRanges(self.unit, file) };
        if !skipped.is_null() {
            let list = unsafe { &*skipped };
            if list.count != 0 {
                let ranges =
                    unsafe { std::slice::from_raw_parts(list.ranges, list.count as usize) }
                        .iter()
                        .map(|range| {
                            let mut start = 0;
                            let mut end = 0;
                            unsafe {
                                clang_getExpansionLocation(
                                    clang_getRangeStart(*range),
                                    std::ptr::null_mut(),
                                    std::ptr::null_mut(),
                                    std::ptr::null_mut(),
                                    &raw mut start,
                                );
                                clang_getExpansionLocation(
                                    clang_getRangeEnd(*range),
                                    std::ptr::null_mut(),
                                    std::ptr::null_mut(),
                                    std::ptr::null_mut(),
                                    &raw mut end,
                                );
                            }
                            (start, end)
                        })
                        .collect::<Vec<_>>();
                tokens.retain(|token| {
                    !ranges.iter().any(|(start, end)| {
                        token.location.offset >= *start && token.location.offset < *end
                    })
                });
            }
            unsafe {
                clang_disposeSourceRangeList(skipped);
            }
        }
        self.files
            .borrow_mut()
            .insert(filename.into(), tokens.clone());
        Ok(tokens)
    }

    fn prefix_start(&self, tokens: &[Token], mut start: usize, cursor: CXCursor) -> Result<usize> {
        loop {
            if start == 0 {
                return Ok(start);
            }
            if tokens[start - 1].kind == CXToken_Comment {
                start -= 1;
                continue;
            }
            if start >= 2
                && tokens[start - 2].text == "extern"
                && matches!(tokens[start - 1].text.as_str(), "\"C\"" | "\"C++\"")
            {
                let parent = unsafe { clang_getCursorLexicalParent(cursor) };
                let mut offset = 0;
                unsafe {
                    clang_getExpansionLocation(
                        clang_getRangeStart(clang_getCursorExtent(parent)),
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        &raw mut offset,
                    );
                }
                if unsafe { clang_getCursorKind(parent) } == CXCursor_LinkageSpec
                    && offset == tokens[start - 2].location.offset
                {
                    start -= 2;
                    continue;
                }
            }
            let preceding = tokens[start - 1].end;
            let candidate = self.macros.iter().find(|((file, offset), expansions)| {
                file == &tokens[start - 1].location.file
                    && *offset < preceding
                    && expansions
                        .iter()
                        .any(|expansion| expansion.end == preceding)
            });
            if let Some((key @ (file, offset), _)) = candidate
                && self
                    .resolve(self.expansion(key, cursor)?.unwrap(), cursor)?
                    .is_some()
            {
                let anchor = tokens[..start]
                    .iter()
                    .position(|token| {
                        &token.location.file == file && token.location.offset == *offset
                    })
                    .ok_or_else(|| {
                        error(cursor, "annotation prefix has no physical token anchor")
                    })?;
                if self.directive(&tokens[anchor], cursor)? {
                    return Ok(start);
                }
                start = anchor;
                continue;
            }
            return Ok(start);
        }
    }

    fn directive(&self, token: &Token, cursor: CXCursor) -> Result<bool> {
        let filename = CString::new(token.location.file.as_str())
            .map_err(|_| error(cursor, "source filename contains NUL"))?;
        let file = unsafe { clang_getFile(self.unit, filename.as_ptr()) };
        let mut size = 0;
        let contents = unsafe { clang_getFileContents(self.unit, file, &raw mut size) };
        if contents.is_null() || token.location.offset as usize > size {
            return Err(error(
                cursor,
                "annotation prefix lacks its compiler source buffer",
            ));
        }
        let bytes = unsafe { std::slice::from_raw_parts(contents.cast::<u8>(), size) };
        let mut start = token.location.offset as usize;
        loop {
            start = bytes[..start]
                .iter()
                .rposition(|byte| *byte == b'\n')
                .map_or(0, |position| position + 1);
            if start == 0 {
                break;
            }
            let mut end = start - 1;
            if end > 0 && bytes[end - 1] == b'\r' {
                end -= 1;
            }
            if end == 0 || bytes[end - 1] != b'\\' {
                break;
            }
            start = end - 1;
        }
        Ok(bytes[start..token.location.offset as usize]
            .iter()
            .find(|byte| !byte.is_ascii_whitespace())
            .is_some_and(|byte| *byte == b'#'))
    }

    pub(super) fn declaration_tokens(&self, cursor: CXCursor) -> Result<Vec<Token>> {
        let range = unsafe { clang_getCursorExtent(cursor) };
        let mut file = std::ptr::null_mut();
        let mut start = 0;
        let mut end = 0;
        unsafe {
            clang_getExpansionLocation(
                clang_getRangeStart(range),
                &raw mut file,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &raw mut start,
            );
            clang_getExpansionLocation(
                clang_getRangeEnd(range),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &raw mut end,
            );
        }
        let filename = text(unsafe { clang_getFileName(file) });
        let tokens = self.file_tokens(&filename)?;
        let first = tokens
            .iter()
            .position(|token| token.location.offset >= start)
            .ok_or_else(|| error(cursor, "declaration lacks a physical source range"))?;
        let first = self.prefix_start(&tokens, first, cursor)?;
        Ok(tokens[first..]
            .iter()
            .take_while(|token| token.location.offset < end)
            .cloned()
            .collect())
    }

    pub fn interface(&self, cursor: CXCursor) -> Result<Vec<AnnotationEvidence>> {
        let location = locations(cursor).0;
        let tokens = self.file_tokens(&location.file)?;
        let markers = tokens
            .iter()
            .enumerate()
            .filter_map(|(position, token)| {
                if token.kind != CXToken_Comment {
                    return None;
                }
                let body = token.text.strip_prefix("/*")?.strip_suffix("*/")?.trim();
                let name = body.strip_prefix("interface ")?.trim();
                identifier(name).then_some((position, name))
            })
            .collect::<Vec<_>>();
        let name = text(unsafe { clang_getCursorSpelling(cursor) });
        if let Some((marker, marked_name)) = markers
            .iter()
            .rev()
            .find(|(position, _)| tokens[*position].location.offset < location.offset)
        {
            let end = markers
                .iter()
                .find(|(position, _)| position > marker)
                .map_or(u32::MAX, |(position, _)| tokens[*position].location.offset);
            let annotations = tokens[marker + 1..]
                .iter()
                .take_while(|token| token.kind == CXToken_Comment)
                .filter(|token| bracket_comment(&token.text))
                .collect::<Vec<_>>();
            if !annotations.is_empty() {
                if *marked_name != name
                    || markers
                        .iter()
                        .filter(|(_, candidate)| *candidate == name)
                        .count()
                        != 1
                {
                    return Err(error(
                        cursor,
                        "MIDL section has mismatched or duplicate interface identity",
                    ));
                }
                let mut candidates = Vec::new();
                for record in &self.records {
                    let observed = locations(*record).0;
                    if observed.file == location.file
                        && observed.offset > tokens[*marker].location.offset
                        && observed.offset < end
                        && text(unsafe { clang_getCursorSpelling(*record) }) == name
                    {
                        let canonical = unsafe { clang_getCanonicalCursor(*record) };
                        if !candidates.iter().any(
                            |candidate| unsafe { clang_equalCursors(*candidate, canonical) } != 0,
                        ) {
                            candidates.push(canonical);
                        }
                    }
                }
                if candidates.len() != 1
                    || unsafe {
                        clang_equalCursors(candidates[0], clang_getCanonicalCursor(cursor))
                    } == 0
                {
                    return Err(error(
                        cursor,
                        "MIDL section does not own one compiler interface identity",
                    ));
                }
                return annotations
                    .iter()
                    .enumerate()
                    .map(|(ordinal, token)| {
                        if midl(&token.text).is_none() {
                            return Err(error(cursor, "malformed MIDL section contract"));
                        }
                        Ok(AnnotationEvidence {
                            source: "MIDL-section".into(),
                            ordinal,
                            payload: format!("{}\n{}", tokens[*marker].text, token.text),
                            location: token.location.clone(),
                            bindings: Vec::new(),
                            resolution: None,
                        })
                    })
                    .collect();
            }
        }
        let declaration = self.declaration_tokens(cursor)?;
        let prefix = declaration
            .iter()
            .take_while(|token| token.kind == CXToken_Comment)
            .cloned()
            .collect::<Vec<_>>();
        self.annotations(&prefix, &[], cursor)
    }
}

fn identifier(name: &str) -> bool {
    let mut bytes = name.bytes();
    matches!(bytes.next(), Some(b'a'..=b'z' | b'A'..=b'Z' | b'_'))
        && bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
}

fn bracket_comment(comment: &str) -> bool {
    comment
        .strip_prefix("/*")
        .is_some_and(|body| body.trim_start().starts_with('['))
}
