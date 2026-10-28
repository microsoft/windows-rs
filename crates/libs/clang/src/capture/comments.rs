use super::*;

type Token = (CXTokenKind, String, CXSourceLocation, u32);

fn expansion(location: CXSourceLocation) -> (CXFile, u32) {
    let mut file = std::ptr::null_mut();
    let mut offset = 0;
    unsafe {
        clang_getExpansionLocation(
            location,
            &raw mut file,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            &raw mut offset,
        );
    }
    (file, offset)
}

/// Tokens of `owner`'s extent, or `None` when the extent spans more than one expansion file.
fn tokens(owner: CXCursor) -> Option<Vec<Token>> {
    let unit = unsafe { clang_Cursor_getTranslationUnit(owner) };
    let extent = unsafe { clang_getCursorExtent(owner) };
    let source_file = expansion(unsafe { clang_getRangeStart(extent) }).0;
    let mut tokens = std::ptr::null_mut();
    let mut count = 0;
    unsafe {
        clang_tokenize(unit, extent, &raw mut tokens, &raw mut count);
    }
    let mut owned = Vec::new();
    let mut same_file = true;
    for index in 0..count as usize {
        let token = unsafe { *tokens.add(index) };
        let location = unsafe { clang_getTokenLocation(unit, token) };
        let (file, offset) = expansion(location);
        same_file &= file == source_file;
        let kind = unsafe { clang_getTokenKind(token) };
        owned.push((
            kind,
            if matches!(kind, CXToken_Punctuation | CXToken_Comment) {
                text(unsafe { clang_getTokenSpelling(unit, token) })
            } else {
                String::new()
            },
            location,
            offset,
        ));
    }
    unsafe {
        clang_disposeTokens(unit, tokens, count);
    }
    same_file.then_some(owned)
}

impl Capture<'_> {
    /// Comments between the start of a method declaration and its name.
    pub(super) fn method_comments(&mut self, method: CXCursor) -> Vec<Comment> {
        let Some(owned) = tokens(method) else {
            return Vec::new();
        };
        let name = expansion(unsafe { clang_getCursorLocation(method) }).1;
        owned
            .iter()
            .take_while(|(_, _, _, offset)| *offset < name)
            .filter(|(kind, ..)| *kind == CXToken_Comment)
            .map(|(_, text, location, _)| Comment {
                text: text.clone(),
                location: self.location_at(*location),
            })
            .collect()
    }

    pub(super) fn parameter_comments(&mut self, owner: CXCursor, parameters: &mut [Parameter]) {
        if parameters.is_empty() {
            return;
        }
        let cursors: Vec<_> = children(owner)
            .into_iter()
            .filter(|cursor| cursor.kind == CXCursor_ParmDecl)
            .collect();
        if cursors.len() != parameters.len() {
            return;
        }
        let spans: Vec<_> = cursors
            .iter()
            .map(|cursor| unsafe {
                let range = clang_getCursorExtent(*cursor);
                let start = self.location_at(clang_getRangeStart(range));
                let end = self.location_at(clang_getRangeEnd(range));
                (start, end)
            })
            .collect();
        let file = spans[0].0.file;
        if spans
            .iter()
            .any(|(start, end)| start.file != file || end.file != file)
            || spans
                .windows(2)
                .any(|pair| pair[0].1.offset >= pair[1].0.offset)
        {
            return;
        }
        let source_file =
            expansion(unsafe { clang_getRangeStart(clang_getCursorExtent(cursors[0])) }).0;
        if expansion(unsafe { clang_getRangeStart(clang_getCursorExtent(owner)) }).0 != source_file
        {
            return;
        }
        let Some(owned) = tokens(owner) else {
            return;
        };
        // Select the innermost parentheses enclosing every compiler-owned parameter span.
        let mut stack = Vec::new();
        let mut open = None;
        for (index, (kind, text, _, offset)) in owned.iter().enumerate() {
            if *kind != CXToken_Punctuation {
                continue;
            }
            if text == "(" {
                stack.push(index);
            } else if text == ")"
                && let Some(start) = stack.pop()
                && owned[start].3 < spans[0].0.offset
                && spans.last().unwrap().1.offset <= *offset
            {
                open = Some(start);
                break;
            }
        }
        let Some(mut boundary) = open else {
            return;
        };
        for (index, parameter) in parameters.iter_mut().enumerate() {
            if index != 0 {
                let previous = spans[index - 1].1.offset;
                let Some(next) = owned.iter().position(|(kind, _, _, offset)| {
                    *offset >= previous && *kind != CXToken_Comment
                }) else {
                    return;
                };
                if owned[next].1 != "," || owned[next].3 >= spans[index].0.offset {
                    return;
                }
                boundary = next;
            }
            for (kind, text, location, offset) in &owned[boundary + 1..] {
                if *kind != CXToken_Comment || *offset >= spans[index].0.offset {
                    break;
                }
                parameter.comments.push(Comment {
                    text: text.clone(),
                    location: self.location_at(*location),
                });
            }
        }
    }
}
