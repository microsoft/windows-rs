use super::*;

#[derive(Debug)]
pub struct Const {
    pub name: String,
    pub ty: Option<metadata::Type>,
    pub value: metadata::Value,
}

impl Const {
    /// Parse object-like macros whose token body has a fixed constant shape.
    pub fn parse(cursor: Cursor, parser: &mut Parser<'_>) -> Result<Option<Self>, Error> {
        if cursor.is_macro_builtin() {
            return Ok(None);
        }
        // Source-adjacency catches libclang misreports that leak macro parameters as types.
        if cursor.is_macro_function_like()
            || parser.tu.macro_is_function_like_by_source(cursor.extent())
        {
            return Ok(None);
        }

        let name = cursor.name();
        if name.is_empty() || name.starts_with('_') {
            return Ok(None);
        }

        let tokens = parser.tu.tokenize(cursor.extent());
        let body: Vec<_> = tokens.into_iter().skip(1).collect();

        let Some(value) = parse_body(&body, parser.namespace, parser.ref_map, parser.header_names)
        else {
            return Ok(None);
        };

        Ok(Some(Self {
            name,
            ty: None,
            value,
        }))
    }

    /// Parse file-scope floating-point `const` variables that flat metadata would otherwise lose.
    pub fn parse_var_decl(cursor: &Cursor) -> Option<Self> {
        let name = cursor.name();
        if name.is_empty() || name.starts_with('_') {
            return None;
        }
        let ty = cursor.ty();
        if !ty.is_const() {
            return None;
        }
        let value = match ty.canonical_type().kind() {
            CXType_Float => metadata::Value::F32(cursor.evaluate_double()? as f32),
            CXType_Double | CXType_LongDouble => metadata::Value::F64(cursor.evaluate_double()?),
            _ => return None,
        };
        Some(Self {
            name,
            ty: None,
            value,
        })
    }

    pub fn write(&self, namespace: &str) -> Result<TokenStream, Error> {
        let name = write_ident(&self.name);
        let value_ty = self.value.ty();
        let ty = self.ty.as_ref().unwrap_or(&value_ty);
        let value = write_typed_value(namespace, ty, &self.value);
        let ty = write_type(namespace, ty);
        match &self.value {
            metadata::Value::Utf8(_) => {
                let attr = native_encoding_attr("ansi");
                Ok(quote! {
                    #attr
                    const #name: #ty = #value;
                })
            }
            metadata::Value::Utf16(_) => {
                let attr = native_encoding_attr("utf-16");
                Ok(quote! {
                    #attr
                    const #name: #ty = #value;
                })
            }
            _ => Ok(quote! { const #name: #ty = #value; }),
        }
    }
}

/// A GUID constant from a forward-declared C++ class with `__declspec(uuid(...))`.
/// MIDL uses this for COM server activation CLSIDs.
#[derive(Debug)]
pub struct GuidConst {
    pub name: String,
    pub uuid: String,
}

impl GuidConst {
    pub fn write(&self) -> Result<TokenStream, Error> {
        let name = write_ident(&self.name);
        let lit_str = uuid_to_u128_literal(&self.uuid);
        let lit = syn::LitInt::new(&lit_str, Span::call_site());
        Ok(quote! { const #name: GUID = #lit; })
    }
}

/// A `PROPERTYKEY`/`DEVPROPKEY` macro constant: GUID in `#[guid]`, `pid` as the value.
#[derive(Debug)]
pub struct PropertyKeyConst {
    pub name: String,
    pub ty: String,
    pub uuid: String,
    pub pid: u32,
}

impl PropertyKeyConst {
    pub fn write(&self) -> Result<TokenStream, Error> {
        let name = write_ident(&self.name);
        let ty = write_ident(&self.ty);
        let guid = syn::LitInt::new(&uuid_to_u128_literal(&self.uuid), Span::call_site());
        let pid = syn::LitInt::new(&format!("{}u32", self.pid), Span::call_site());
        Ok(quote! {
            #[guid(#guid)]
            const #name: #ty = #pid;
        })
    }
}

impl Const {
    /// Evaluate macro expressions by injecting per-name enum probes into a synthetic TU.
    /// Independent probes and `CXTranslationUnit_KeepGoing` let one bad macro fail without
    /// hiding the rest; size/signedness probes recover the C integer type clang otherwise drops.
    pub fn evaluate_macros(
        input: &str,
        names: &[String],
        index: &Index,
        args: &[&str],
    ) -> Result<Vec<Self>, Error> {
        if names.is_empty() {
            return Ok(vec![]);
        }

        // Put the synthetic file beside the header; include by basename so relative includes work.
        let input_basename = std::path::Path::new(input)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(input);
        let prefix = format!("#include \"{input_basename}\"\n{NARG_PROLOGUE}");

        let synthetic = format!("{input}.__rdl_eval__.cpp");

        Self::evaluate_names(&prefix, &synthetic, names, index, args)
    }

    /// Evaluate macro names from in-memory source by embedding it into the synthetic TU.
    pub fn evaluate_macros_str(
        content: &str,
        names: &[String],
        index: &Index,
        args: &[&str],
    ) -> Result<Vec<Self>, Error> {
        if names.is_empty() {
            return Ok(vec![]);
        }

        // There is no on-disk directory context, so relative includes may not resolve.
        let prefix = format!("{content}\n{NARG_PROLOGUE}");
        const SYNTHETIC: &str = "__rdl_input_str_eval__.cpp";

        Self::evaluate_names(&prefix, SYNTHETIC, names, index, args)
    }

    /// Evaluate names in batches, retrying swallowed probe enums in smaller batches.
    /// Validity comes from recovered enum values, not diagnostics; unbalanced delimiters can
    /// consume following probes, so missing names are requeued until the poison macro is isolated.
    fn evaluate_names(
        prefix: &str,
        synthetic: &str,
        names: &[String],
        index: &Index,
        args: &[&str],
    ) -> Result<Vec<Self>, Error> {
        let eval_args = with_unlimited_errors(args);
        let mut results = vec![];
        // Swallowed probes are split until a singleton poison macro can be dropped.
        let mut queue: Vec<Vec<String>> = vec![names.to_vec()];
        while let Some(batch) = queue.pop() {
            let mut source = String::from(prefix);
            for name in &batch {
                // The name came from libclang, but validate before writing generated C++.
                if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                    continue;
                }
                source.push_str(&eval_probe(name));
            }
            let tu =
                index.parse_unsaved(synthetic, &source, &eval_args, CXTranslationUnit_KeepGoing)?;
            let (consts, present) = collect_eval_results(&tu);
            results.extend(consts);

            let missing: Vec<String> = batch
                .iter()
                .filter(|n| n.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'))
                .filter(|n| !present.contains(n.as_str()))
                .cloned()
                .collect();
            if !missing.is_empty() && batch.len() > 1 {
                let retry = if missing.len() == batch.len() {
                    &batch
                } else {
                    &missing
                };
                if retry.len() > 1 {
                    let mid = retry.len() / 2;
                    queue.push(retry[mid..].to_vec());
                    queue.push(retry[..mid].to_vec());
                }
            }
        }
        Ok(results)
    }
}

/// Disable clang's error cap so bad probe enums do not abort the TU before later valid macros.
fn with_unlimited_errors<'a>(args: &[&'a str]) -> Vec<&'a str> {
    let mut out = args.to_vec();
    out.push("-ferror-limit=0");
    out
}

/// Counts top-level comma-separated macro-expansion results for the shape gate.
/// This rejects GUID initializer lists before the C comma operator can fold them to an integer.
const NARG_PROLOGUE: &str = "\
#define __RDL_NARG(...) __RDL_NARG_(__VA_ARGS__,20,19,18,17,16,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0)\n\
#define __RDL_NARG_(_1,_2,_3,_4,_5,_6,_7,_8,_9,_10,_11,_12,_13,_14,_15,_16,_17,_18,_19,_20,N,...) N\n";

/// Emits independent enum probes for one macro: value, integer gate, comma-count gate,
/// width, and signedness. `& 0` rejects pointer/string/floating expressions, while
/// `__RDL_NARG` catches post-expansion comma lists such as GUID initializers.
/// Validity is read from recovered enum values, not diagnostics.
fn eval_probe(name: &str) -> String {
    format!(
        "constexpr auto __rdl_eval_{name} = ({name});\n\
         enum {{ __rdl_ok_{name} = (({name}) & 0) + 1 }};\n\
         enum {{ __rdl_nc_{name} = __RDL_NARG({name}) }};\n\
         enum {{ __rdl_sz_{name} = sizeof({name}) }};\n\
         enum {{ __rdl_sg_{name} = ((({name}) * 0 - 1) < 0) ? 1 : 2 }};\n"
    )
}

/// Collect kept eval probes and the names whose gating probes were fully parsed.
/// Missing gates mean a preceding macro swallowed later enum declarations, so the caller
/// retries those names; failed gates are real rejects and are not retried.
fn collect_eval_results(tu: &TranslationUnit) -> (Vec<Const>, HashSet<String>) {
    let mut evals: Vec<(String, u64, i64, Option<metadata::Type>)> = vec![];
    let mut eval_seen: HashSet<String> = HashSet::new();
    let mut ok_seen: HashSet<String> = HashSet::new();
    let mut nc_seen: HashSet<String> = HashSet::new();
    let mut type_ok: HashSet<String> = HashSet::new();
    let mut shape_ok: HashSet<String> = HashSet::new();
    let mut sizes: HashMap<String, i64> = HashMap::new();
    let mut signs: HashMap<String, i64> = HashMap::new();
    for child in tu.cursor().children() {
        if !child.is_from_main_file() {
            continue;
        }
        if child.kind() == CXCursor_VarDecl
            && let Some(original_name) = child.name().strip_prefix("__rdl_eval_")
        {
            eval_seen.insert(original_name.to_string());
            if let Some((unsigned, signed)) = child.evaluate_integer() {
                let ty = child.ty();
                let semantic = pointer_sized_abi(&ty.ty().name());
                evals.push((original_name.to_string(), unsigned, signed, semantic));
            }
            continue;
        }
        if child.kind() != CXCursor_EnumDecl {
            continue;
        }
        for constant in child.children() {
            if constant.kind() != CXCursor_EnumConstantDecl {
                continue;
            }
            let const_name = constant.name();
            if let Some(original_name) = const_name.strip_prefix("__rdl_ok_") {
                ok_seen.insert(original_name.to_string());
                if constant.enum_value() == 1 {
                    type_ok.insert(original_name.to_string());
                }
            } else if let Some(original_name) = const_name.strip_prefix("__rdl_nc_") {
                nc_seen.insert(original_name.to_string());
                if constant.enum_value() == 1 {
                    shape_ok.insert(original_name.to_string());
                }
            } else if let Some(original_name) = const_name.strip_prefix("__rdl_sz_") {
                sizes.insert(original_name.to_string(), constant.enum_value());
            } else if let Some(original_name) = const_name.strip_prefix("__rdl_sg_") {
                signs.insert(original_name.to_string(), constant.enum_value());
            }
        }
    }

    // Missing gating probes mean the candidate was swallowed and must be retried.
    let present: HashSet<String> = eval_seen
        .iter()
        .filter(|n| ok_seen.contains(*n) && nc_seen.contains(*n))
        .cloned()
        .collect();

    let kept = evals
        .into_iter()
        .filter(|(name, _, _, _)| type_ok.contains(name) && shape_ok.contains(name))
        .map(|(name, unsigned, signed, ty)| {
            let value = eval_integer_value(
                unsigned,
                signed,
                sizes.get(&name).copied(),
                signs.get(&name).copied(),
            );
            Const { name, ty, value }
        })
        .collect();

    (kept, present)
}

/// Type an evaluated integer from size/signedness probes, with a value-based fallback.
fn eval_integer_value(
    unsigned: u64,
    signed: i64,
    sz: Option<i64>,
    sg: Option<i64>,
) -> metadata::Value {
    match (sz, sg) {
        (Some(4), Some(1)) => metadata::Value::I32(signed as i32),
        (Some(4), Some(2)) => metadata::Value::U32(unsigned as u32),
        (Some(8), Some(1)) => metadata::Value::I64(signed),
        (Some(8), Some(2)) => metadata::Value::U64(unsigned),
        _ => {
            if signed >= 0 {
                if let Ok(v) = u32::try_from(unsigned) {
                    metadata::Value::U32(v)
                } else {
                    metadata::Value::U64(unsigned)
                }
            } else if let Ok(v) = i32::try_from(signed) {
                metadata::Value::I32(v)
            } else {
                metadata::Value::I64(signed)
            }
        }
    }
}

/// Parse fixed literal/cast macro bodies; nested handle casts fall through to `parse_nested_cast`.
fn parse_body(
    body: &[(CXTokenKind, String)],
    namespace: &str,
    ref_map: &HashMap<String, String>,
    header_names: Option<&HashMap<String, String>>,
) -> Option<metadata::Value> {
    match body {
        [(CXToken_Literal, lit)] => parse_literal(lit, false),
        [(CXToken_Punctuation, minus), (CXToken_Literal, lit)] if minus == "-" => {
            parse_literal(lit, true)
        }
        [
            (CXToken_Punctuation, lp),
            (CXToken_Literal, lit),
            (CXToken_Punctuation, rp),
        ] if lp == "(" && rp == ")" => parse_literal(lit, false),
        [
            (CXToken_Punctuation, lp1),
            (CXToken_Punctuation, lp2),
            (CXToken_Identifier, ty),
            (CXToken_Punctuation, rp1),
            (CXToken_Literal, lit),
            (CXToken_Punctuation, rp2),
        ] if lp1 == "(" && lp2 == "(" && rp1 == ")" && rp2 == ")" => {
            parse_named_cast(namespace, ref_map, header_names, ty, lit, false)
        }
        [
            (CXToken_Punctuation, lp1),
            (CXToken_Punctuation, lp2),
            (CXToken_Identifier, ty),
            (CXToken_Punctuation, rp1),
            (CXToken_Punctuation, minus),
            (CXToken_Literal, lit),
            (CXToken_Punctuation, rp2),
        ] if lp1 == "(" && lp2 == "(" && rp1 == ")" && minus == "-" && rp2 == ")" => {
            parse_named_cast(namespace, ref_map, header_names, ty, lit, true)
        }
        [
            (CXToken_Punctuation, lp),
            (CXToken_Identifier, ty),
            (CXToken_Punctuation, rp),
            (CXToken_Literal, lit),
        ] if lp == "(" && rp == ")" => {
            parse_named_cast(namespace, ref_map, header_names, ty, lit, false)
        }
        [
            (CXToken_Punctuation, lp),
            (CXToken_Identifier, ty),
            (CXToken_Punctuation, rp),
            (CXToken_Punctuation, minus),
            (CXToken_Literal, lit),
        ] if lp == "(" && rp == ")" && minus == "-" => {
            parse_named_cast(namespace, ref_map, header_names, ty, lit, true)
        }
        [
            (CXToken_Punctuation, lp1),
            (CXToken_Identifier, ty),
            (CXToken_Punctuation, rp1),
            (CXToken_Punctuation, lp2),
            (CXToken_Punctuation, not),
            (CXToken_Literal, lit),
            (CXToken_Punctuation, rp2),
        ] if lp1 == "(" && rp1 == ")" && lp2 == "(" && not == "~" && rp2 == ")" => {
            parse_named_complement(namespace, ref_map, header_names, ty, lit)
        }
        [
            (CXToken_Punctuation, lp1),
            (CXToken_Punctuation, lp2),
            (CXToken_Keyword, kw),
            (CXToken_Punctuation, rp1),
            (CXToken_Literal, lit),
            (CXToken_Punctuation, rp2),
        ] if lp1 == "(" && lp2 == "(" && rp1 == ")" && rp2 == ")" => {
            parse_keyword_cast(kw, lit, false)
        }
        [
            (CXToken_Punctuation, lp1),
            (CXToken_Punctuation, lp2),
            (CXToken_Keyword, kw),
            (CXToken_Punctuation, rp1),
            (CXToken_Punctuation, minus),
            (CXToken_Literal, lit),
            (CXToken_Punctuation, rp2),
        ] if lp1 == "(" && lp2 == "(" && rp1 == ")" && minus == "-" && rp2 == ")" => {
            parse_keyword_cast(kw, lit, true)
        }
        [
            (CXToken_Punctuation, lp),
            (CXToken_Keyword, kw),
            (CXToken_Punctuation, rp),
            (CXToken_Literal, lit),
        ] if lp == "(" && rp == ")" => parse_keyword_cast(kw, lit, false),
        [
            (CXToken_Punctuation, lp),
            (CXToken_Keyword, kw),
            (CXToken_Punctuation, rp),
            (CXToken_Punctuation, minus),
            (CXToken_Literal, lit),
        ] if lp == "(" && rp == ")" && minus == "-" => parse_keyword_cast(kw, lit, true),
        // Preserve SDK error-code wrapper casts such as `_HRESULT_TYPEDEF_`.
        [
            (CXToken_Identifier, w),
            (CXToken_Punctuation, lp),
            (CXToken_Literal, lit),
            (CXToken_Punctuation, rp),
        ] if lp == "(" && rp == ")" && cast_wrapper_macro(w).is_some() => parse_named_cast(
            namespace,
            ref_map,
            header_names,
            cast_wrapper_macro(w)?,
            lit,
            false,
        ),
        [
            (CXToken_Identifier, w),
            (CXToken_Punctuation, lp),
            (CXToken_Punctuation, minus),
            (CXToken_Literal, lit),
            (CXToken_Punctuation, rp),
        ] if lp == "(" && rp == ")" && minus == "-" && cast_wrapper_macro(w).is_some() => {
            parse_named_cast(
                namespace,
                ref_map,
                header_names,
                cast_wrapper_macro(w)?,
                lit,
                true,
            )
        }
        // Ordinal resources are pointer-valued macros; parse them here so evaluation does
        // not drop them.
        [
            (CXToken_Identifier, w),
            (CXToken_Punctuation, lp),
            (CXToken_Literal, lit),
            (CXToken_Punctuation, rp),
        ] if lp == "(" && rp == ")" && makeintresource_macro(w).is_some() => {
            let (digits, _suffix) = split_int_suffix(lit);
            let raw: u64 = parse_int_digits(digits)?;
            Some(metadata::Value::EnumValue(
                metadata::TypeName::named(namespace, makeintresource_macro(w)?),
                Box::new(metadata::Value::I32(raw as i32)),
            ))
        }
        // `MAKEINTRESOURCEW(-2)` truncates to `WORD` before widening to a pointer; keep the
        // zero-extended 16-bit ordinal so resource APIs do not treat it as a string pointer.
        [
            (CXToken_Identifier, w),
            (CXToken_Punctuation, lp),
            (CXToken_Punctuation, minus),
            (CXToken_Literal, lit),
            (CXToken_Punctuation, rp),
        ] if lp == "(" && rp == ")" && minus == "-" && makeintresource_macro(w).is_some() => {
            let (digits, _suffix) = split_int_suffix(lit);
            let raw: u64 = parse_int_digits(digits)?;
            Some(metadata::Value::EnumValue(
                metadata::TypeName::named(namespace, makeintresource_macro(w)?),
                Box::new(metadata::Value::I32((raw as u16).wrapping_neg() as i32)),
            ))
        }
        // Inline char-pointer sentinels are not named aliases, so match their token shape here.
        [
            (CXToken_Punctuation, lp1),
            (CXToken_Punctuation, lp2),
            (CXToken_Identifier, ptr_ty),
            (CXToken_Punctuation, star),
            (CXToken_Punctuation, rp1),
            (CXToken_Punctuation, lp3),
            (CXToken_Identifier, inner),
            (CXToken_Punctuation, rp2),
            (CXToken_Punctuation, minus),
            (CXToken_Literal, lit),
            (CXToken_Punctuation, rp3),
        ] if lp1 == "("
            && lp2 == "("
            && star == "*"
            && rp1 == ")"
            && lp3 == "("
            && rp2 == ")"
            && minus == "-"
            && rp3 == ")"
            && char_pointer_target(ptr_ty).is_some() =>
        {
            let (digits, _suffix) = split_int_suffix(lit);
            let raw: u64 = parse_int_digits(digits)?;
            Some(metadata::Value::EnumValue(
                metadata::TypeName::named(namespace, char_pointer_target(ptr_ty)?),
                Box::new(inner_scalar_value(inner, raw, true)),
            ))
        }
        _ => parse_nested_cast(body, namespace, ref_map, header_names),
    }
}

/// Build the pseudo-attribute the RDL reader maps to `NativeEncodingAttribute`.
fn native_encoding_attr(encoding: &str) -> TokenStream {
    quote! { #[encoding(#encoding)] }
}

/// Parse a C integer, float, or string literal into a `metadata::Value`.
fn parse_literal(lit: &str, negate: bool) -> Option<metadata::Value> {
    if lit.starts_with("L\"") {
        if negate {
            return None;
        }
        let inner = lit.strip_prefix("L\"")?.strip_suffix('"')?;
        return Some(metadata::Value::Utf16(decode_c_wide_string(inner)?));
    }

    if lit.starts_with('"') {
        if negate {
            return None;
        }
        let inner = lit.strip_prefix('"')?.strip_suffix('"')?;
        return Some(metadata::Value::Utf8(decode_c_narrow_string(inner)?));
    }

    let (digits, suffix) = split_int_suffix(lit);
    let Some(raw) = parse_int_digits(digits) else {
        // Not an integer - try a floating-point literal.
        return parse_float_literal(lit, negate);
    };

    integer_value(raw, int_literal_is_decimal(digits), suffix, negate)
}

/// Classify literal base for C typing: only decimal needs a `U` suffix to become unsigned.
fn int_literal_is_decimal(digits: &str) -> bool {
    if digits.len() >= 2 {
        let prefix = &digits[..2];
        if prefix.eq_ignore_ascii_case("0x") || prefix.eq_ignore_ascii_case("0b") {
            return false;
        }
    }
    !(digits.len() > 1 && digits.starts_with('0'))
}

/// Parse decimal floating literals; hex floats are not represented.
/// A decimal point or exponent is required so hex integers like `0xF` stay integer literals.
fn parse_float_literal(lit: &str, negate: bool) -> Option<metadata::Value> {
    let lower = lit.to_ascii_lowercase();
    if lower.starts_with("0x") {
        return None;
    }

    let (body, is_f32) = if let Some(body) = lower.strip_suffix('f') {
        (body, true)
    } else if let Some(body) = lower.strip_suffix('l') {
        (body, false)
    } else {
        (lower.as_str(), false)
    };

    if !body.contains('.') && !body.contains('e') {
        return None;
    }

    if is_f32 {
        let value = body.parse::<f32>().ok()?;
        Some(metadata::Value::F32(if negate { -value } else { value }))
    } else {
        let value = body.parse::<f64>().ok()?;
        Some(metadata::Value::F64(if negate { -value } else { value }))
    }
}

/// Decode a narrow-string literal as bytes, then keep it only if those bytes are valid UTF-8.
/// Raw byte arrays have no exact `String` representation and are omitted.
fn decode_c_narrow_string(inner: &str) -> Option<String> {
    let mut bytes = Vec::with_capacity(inner.len());
    let mut chars = inner.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '\\' {
            let mut buf = [0u8; 4];
            bytes.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
            continue;
        }
        match chars.next()? {
            '\\' => bytes.push(b'\\'),
            '"' => bytes.push(b'"'),
            '\'' => bytes.push(b'\''),
            '?' => bytes.push(b'?'),
            'n' => bytes.push(b'\n'),
            'r' => bytes.push(b'\r'),
            't' => bytes.push(b'\t'),
            'a' => bytes.push(0x07),
            'b' => bytes.push(0x08),
            'f' => bytes.push(0x0c),
            'v' => bytes.push(0x0b),
            'x' => {
                let (value, count) = take_radix(&mut chars, 16, usize::MAX);
                if count == 0 {
                    return None;
                }
                bytes.push(value as u8);
            }
            o @ '0'..='7' => {
                let mut value = o.to_digit(8)?;
                let (rest, count) = take_radix(&mut chars, 8, 2);
                value = value * 8u32.pow(count as u32) + rest;
                bytes.push(value as u8);
            }
            other => {
                bytes.push(b'\\');
                let mut buf = [0u8; 4];
                bytes.extend_from_slice(other.encode_utf8(&mut buf).as_bytes());
            }
        }
    }
    String::from_utf8(bytes).ok()
}

/// Decode a wide-string literal, dropping escapes that do not name valid Unicode scalars.
fn decode_c_wide_string(inner: &str) -> Option<String> {
    let mut out = String::with_capacity(inner.len());
    let mut chars = inner.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        let scalar = match chars.next()? {
            '\\' => '\\',
            '"' => '"',
            '\'' => '\'',
            '?' => '?',
            'n' => '\n',
            'r' => '\r',
            't' => '\t',
            'a' => '\u{07}',
            'b' => '\u{08}',
            'f' => '\u{0c}',
            'v' => '\u{0b}',
            'x' | 'u' | 'U' => {
                let (value, count) = take_radix(&mut chars, 16, usize::MAX);
                if count == 0 {
                    return None;
                }
                char::from_u32(value)?
            }
            o @ '0'..='7' => {
                let mut value = o.to_digit(8)?;
                let (rest, count) = take_radix(&mut chars, 8, 2);
                value = value * 8u32.pow(count as u32) + rest;
                char::from_u32(value)?
            }
            other => {
                out.push('\\');
                other
            }
        };
        out.push(scalar);
    }
    Some(out)
}

/// Consume up to `max` leading digits of `radix`.
fn take_radix(
    chars: &mut std::iter::Peekable<std::str::Chars>,
    radix: u32,
    max: usize,
) -> (u32, usize) {
    let mut value = 0;
    let mut count = 0;
    while count < max {
        match chars.peek().and_then(|c| c.to_digit(radix)) {
            Some(digit) => {
                value = value * radix + digit;
                chars.next();
                count += 1;
            }
            None => break,
        }
    }
    (value, count)
}

/// Select the C11 integer literal type under Windows LLP64 widths.
/// Decimal literals use signed candidates unless `U`-suffixed; non-decimal literals may
/// take unsigned candidates to fit the magnitude.
fn c_integer_constant_type(
    is_decimal: bool,
    has_u: bool,
    has_l: bool,
    has_ll: bool,
    raw: u64,
) -> metadata::Type {
    use metadata::Type::{I32, I64, U32, U64};
    let candidates: &[metadata::Type] = if has_u {
        if has_ll { &[U64] } else { &[U32, U64] }
    } else if has_ll {
        if is_decimal { &[I64] } else { &[I64, U64] }
    } else if has_l || is_decimal {
        if is_decimal {
            &[I32, I64]
        } else {
            &[I32, U32, I64, U64]
        }
    } else {
        &[I32, U32, I64, U64]
    };
    for ty in candidates {
        let ty = ty.clone();
        let fits = match ty {
            I32 => raw <= i32::MAX as u64,
            U32 => raw <= u32::MAX as u64,
            I64 => raw <= i64::MAX as u64,
            _ => true,
        };
        if fits {
            return ty;
        }
    }
    U64
}

/// Parse a C integer literal using C11 typing; unary minus applies within that type.
fn integer_value(
    raw: u64,
    is_decimal: bool,
    suffix: &str,
    negate: bool,
) -> Option<metadata::Value> {
    let suffix = suffix.to_ascii_uppercase();
    let has_u = suffix.contains('U');
    let has_ll = suffix.contains("LL");
    let has_l = suffix.contains('L');
    let ty = c_integer_constant_type(is_decimal, has_u, has_l, has_ll, raw);
    Some(if negate {
        match ty {
            metadata::Type::I32 => metadata::Value::I32((raw as i32).wrapping_neg()),
            metadata::Type::I64 => metadata::Value::I64((raw as i64).wrapping_neg()),
            metadata::Type::U32 => metadata::Value::U32((raw as u32).wrapping_neg()),
            metadata::Type::U64 => metadata::Value::U64(raw.wrapping_neg()),
            _ => return None,
        }
    } else {
        match ty {
            metadata::Type::I32 => metadata::Value::I32(raw as i32),
            metadata::Type::U32 => metadata::Value::U32(raw as u32),
            metadata::Type::I64 => metadata::Value::I64(raw as i64),
            metadata::Type::U64 => metadata::Value::U64(raw),
            _ => return None,
        }
    })
}

/// Parse a builtin-keyword integer cast; explicit casts govern width and signedness.
/// Multi-token keywords fall through to batch evaluation.
fn parse_keyword_cast(kw: &str, lit: &str, negate: bool) -> Option<metadata::Value> {
    let ty = keyword_scalar(kw)?;
    let (digits, _suffix) = split_int_suffix(lit);
    let raw: u64 = parse_int_digits(digits)?;
    scalar_value(&ty, raw, negate)
}

/// Map SDK error-code typedef wrapper macros to their hidden cast type.
/// This preserves `HRESULT`/`DWORD` instead of flattening constants to integers.
fn cast_wrapper_macro(name: &str) -> Option<&'static str> {
    Some(match name {
        "_HRESULT_TYPEDEF_" | "_ASF_HRESULT_TYPEDEF_" => "HRESULT",
        "_NDIS_ERROR_TYPEDEF_" => "DWORD",
        _ => return None,
    })
}

/// Map a `MAKEINTRESOURCE` macro to the string-pointer type that carries its ordinal.
/// The scrape is ANSI-default, but bare `MAKEINTRESOURCE` is treated as wide to match
/// the reference metadata.
fn makeintresource_macro(name: &str) -> Option<&'static str> {
    Some(match name {
        "MAKEINTRESOURCE" | "MAKEINTRESOURCEW" => "PWSTR",
        "MAKEINTRESOURCEA" => "PSTR",
        _ => return None,
    })
}

/// Map inline char-pointer sentinel casts to canonical string-pointer types.
fn char_pointer_target(name: &str) -> Option<&'static str> {
    Some(match name {
        "OLECHAR" | "WCHAR" | "wchar_t" => "PWSTR",
        "CHAR" | "char" => "PSTR",
        _ => return None,
    })
}

/// Map a single C builtin integer keyword under LLP64; multi-token spellings fall through.
fn keyword_scalar(name: &str) -> Option<metadata::Type> {
    Some(match name {
        "char" => metadata::Type::I8,
        "short" => metadata::Type::I16,
        "int" | "long" => metadata::Type::I32,
        "unsigned" => metadata::Type::U32,
        _ => return None,
    })
}

fn parse_named_cast(
    namespace: &str,
    ref_map: &HashMap<String, String>,
    header_names: Option<&HashMap<String, String>>,
    type_name: &str,
    lit: &str,
    negate: bool,
) -> Option<metadata::Value> {
    let (digits, _suffix) = split_int_suffix(lit);
    let raw: u64 = parse_int_digits(digits)?;

    // Collapsed scalar typedefs have no emitted name; preserved seed scalars/enums stay named.
    if !ref_map.contains_key(type_name)
        && let Some(ty) = fundamental_scalar(type_name)
    {
        return scalar_value(&ty, raw, negate);
    }

    // Pointer-sized typedefs are collapsed aliases, so sentinels use native-int primitives.
    if !ref_map.contains_key(type_name)
        && let Some(ty) = pointer_sized_abi(type_name)
    {
        return scalar_value(&ty, raw, negate);
    }

    // Void-pointer aliases are collapsed too; keeping the alias name would dangle.
    if !ref_map.contains_key(type_name) && void_pointer_alias(type_name).is_some() {
        return scalar_value(&metadata::Type::USize, raw, negate);
    }

    // String-pointer aliases normalize because redundant `LP*` alias definitions are suppressed.
    let type_name = string_alias_canonical(type_name).unwrap_or(type_name);

    // Token casts have no cursor in per-header mode; `header_names` locates the type partition.
    let ns = header_names
        .and_then(|m| m.get(type_name))
        .or_else(|| ref_map.get(type_name))
        .map_or(namespace, |s| s.as_str());
    let v = if negate {
        (raw as i64).wrapping_neg()
    } else {
        raw as i64
    };
    Some(metadata::Value::EnumValue(
        metadata::TypeName::named(ns, type_name),
        Box::new(metadata::Value::I64(v)),
    ))
}

/// Parse a named cast of a complemented integer literal such as `(SOCKET)(~0)`.
fn parse_named_complement(
    namespace: &str,
    ref_map: &HashMap<String, String>,
    header_names: Option<&HashMap<String, String>>,
    type_name: &str,
    lit: &str,
) -> Option<metadata::Value> {
    let (digits, suffix) = split_int_suffix(lit);
    let raw = parse_int_digits(digits)?;
    let value = integer_complement_value(raw, int_literal_is_decimal(digits), suffix)?;

    if !ref_map.contains_key(type_name)
        && let Some(ty) = fundamental_scalar(type_name).or_else(|| pointer_sized_abi(type_name))
    {
        return cast_integer_value(&ty, &value);
    }

    let ns = header_names
        .and_then(|m| m.get(type_name))
        .or_else(|| ref_map.get(type_name))
        .map_or(namespace, |s| s.as_str());
    Some(metadata::Value::EnumValue(
        metadata::TypeName::named(ns, type_name),
        Box::new(value),
    ))
}

/// Apply integer promotion and complement within the literal's C11 candidate type.
fn integer_complement_value(raw: u64, is_decimal: bool, suffix: &str) -> Option<metadata::Value> {
    let suffix = suffix.to_ascii_uppercase();
    let ty = c_integer_constant_type(
        is_decimal,
        suffix.contains('U'),
        suffix.contains('L'),
        suffix.contains("LL"),
        raw,
    );
    Some(match ty {
        metadata::Type::I32 => metadata::Value::I32(!(raw as i32)),
        metadata::Type::U32 => metadata::Value::U32(!(raw as u32)),
        metadata::Type::I64 => metadata::Value::I64(!(raw as i64)),
        metadata::Type::U64 => metadata::Value::U64(!raw),
        _ => return None,
    })
}

/// Cast an integer value to a collapsed scalar typedef.
fn cast_integer_value(ty: &metadata::Type, value: &metadata::Value) -> Option<metadata::Value> {
    let (signed, unsigned) = match value {
        metadata::Value::I32(value) => (Some(*value as i64), None),
        metadata::Value::I64(value) => (Some(*value), None),
        metadata::Value::U32(value) => (None, Some(*value as u64)),
        metadata::Value::U64(value) => (None, Some(*value)),
        _ => return None,
    };
    let raw = signed.map_or_else(|| unsigned.unwrap(), |value| value as u64);
    scalar_value(ty, raw, false)
}

/// Parse nested casts used by handle/pointer constants such as `INVALID_HANDLE_VALUE`.
/// The innermost scalar cast supplies the bit pattern; reading the outer pointer-sized cast
/// would zero-extend values that the SDK intends to sign-extend.
fn parse_nested_cast(
    body: &[(CXTokenKind, String)],
    namespace: &str,
    ref_map: &HashMap<String, String>,
    header_names: Option<&HashMap<String, String>>,
) -> Option<metadata::Value> {
    let mut casts: Vec<&str> = Vec::new();
    let mut negate = false;
    let mut literal: Option<&str> = None;

    for (i, (kind, tok)) in body.iter().enumerate() {
        match *kind {
            CXToken_Punctuation => match tok.as_str() {
                "(" | ")" => {}
                "-" if literal.is_none() && !negate => negate = true,
                _ => return None,
            },
            CXToken_Identifier => {
                if literal.is_some() {
                    return None;
                }
                // Reject function-like macros so batch evaluation can compute them.
                if !matches!(body.get(i + 1), Some((CXToken_Punctuation, p)) if p == ")") {
                    return None;
                }
                casts.push(tok);
            }
            CXToken_Literal => {
                if literal.is_some() {
                    return None;
                }
                literal = Some(tok);
            }
            _ => return None,
        }
    }

    let lit = literal?;
    if casts.len() < 2 {
        return None;
    }
    let outer = casts[0];
    let inner = casts[casts.len() - 1];
    if fundamental_scalar(outer).is_some() {
        return None;
    }
    // Normalize suppressed string-pointer aliases, as in `parse_named_cast`.
    let outer = string_alias_canonical(outer).unwrap_or(outer);

    // Void-pointer aliases have no emitted name, and arithmetic inner expressions are outside
    // this token parser; drop them rather than dangling on the collapsed alias.
    if void_pointer_alias(outer).is_some() {
        return None;
    }

    let (digits, _suffix) = split_int_suffix(lit);
    let raw: u64 = parse_int_digits(digits)?;
    let inner_value = inner_scalar_value(inner, raw, negate);

    let ns = header_names
        .and_then(|m| m.get(outer))
        .or_else(|| ref_map.get(outer))
        .map_or(namespace, |s| s.as_str());
    Some(metadata::Value::EnumValue(
        metadata::TypeName::named(ns, outer),
        Box::new(inner_value),
    ))
}
/// Read the innermost scalar cast of a nested handle constant.
/// Unknown integer casts use `i64`, the widest safe reinterpretation.
fn inner_scalar_value(inner: &str, raw: u64, negate: bool) -> metadata::Value {
    if let Some(ty) = fundamental_scalar(inner)
        && let Some(value) = scalar_value(&ty, raw, negate)
    {
        return value;
    }
    let v = if negate {
        (raw as i64).wrapping_neg()
    } else {
        raw as i64
    };
    metadata::Value::I64(v)
}

/// Build a fixed-width scalar value, applying negation as wrapping two's-complement.
fn scalar_value(ty: &metadata::Type, raw: u64, negate: bool) -> Option<metadata::Value> {
    let signed = if negate {
        (raw as i64).wrapping_neg()
    } else {
        raw as i64
    };
    Some(match ty {
        metadata::Type::U8 => metadata::Value::U8(signed as u8),
        metadata::Type::U16 => metadata::Value::U16(signed as u16),
        metadata::Type::U32 => metadata::Value::U32(signed as u32),
        metadata::Type::U64 => metadata::Value::U64(signed as u64),
        metadata::Type::I8 => metadata::Value::I8(signed as i8),
        metadata::Type::I16 => metadata::Value::I16(signed as i16),
        metadata::Type::I32 => metadata::Value::I32(signed as i32),
        metadata::Type::I64 => metadata::Value::I64(signed),
        metadata::Type::USize => metadata::Value::USize(signed as u64),
        metadata::Type::ISize => metadata::Value::ISize(signed),
        _ => return None,
    })
}

/// Split a C integer literal after the last digit or `x`/`X` prefix marker.
fn split_int_suffix(lit: &str) -> (&str, &str) {
    let suffix_start = lit
        .rfind(|c: char| c.is_ascii_hexdigit() || c == 'x' || c == 'X')
        .map_or(lit.len(), |i| i + 1);
    (&lit[..suffix_start], &lit[suffix_start..])
}

/// Parse C integer digits, with invalid octal falling back to decimal.
fn parse_int_digits(digits: &str) -> Option<u64> {
    if let Some(hex) = digits
        .strip_prefix("0x")
        .or_else(|| digits.strip_prefix("0X"))
    {
        u64::from_str_radix(hex, 16).ok()
    } else if let Some(bin) = digits
        .strip_prefix("0b")
        .or_else(|| digits.strip_prefix("0B"))
    {
        u64::from_str_radix(bin, 2).ok()
    } else if digits.len() > 1 && digits.starts_with('0') {
        u64::from_str_radix(&digits[1..], 8)
            .or_else(|_| digits.parse::<u64>())
            .ok()
    } else {
        digits.parse::<u64>().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn narrow_decodes_standard_escapes() {
        assert_eq!(decode_c_narrow_string("!<arch>\\n").unwrap(), "!<arch>\n");
        assert_eq!(
            decode_c_narrow_string("Software\\\\Microsoft\\\\OID").unwrap(),
            "Software\\Microsoft\\OID"
        );
        assert_eq!(decode_c_narrow_string("M\\0\\0\\0").unwrap(), "M\0\0\0");
        assert_eq!(decode_c_narrow_string("a\\tb").unwrap(), "a\tb");
    }

    #[test]
    fn narrow_octal_uses_digit_count_not_value() {
        // The trailing octal digits are positional, not the numeric value 1.
        assert_eq!(decode_c_narrow_string("\\101").unwrap(), "A");
    }

    #[test]
    fn narrow_ascii_hex_bytes_are_kept() {
        assert_eq!(
            decode_c_narrow_string("\\x20\\x30\\x10").unwrap(),
            " 0\u{10}"
        );
    }

    #[test]
    fn narrow_non_utf8_byte_array_is_omitted() {
        // Raw GUID byte spellings are not UTF-8.
        assert_eq!(decode_c_narrow_string("\\xaa\\x31\\x28"), None);
    }

    #[test]
    fn wide_decodes_standard_escapes() {
        assert_eq!(decode_c_wide_string("line\\n").unwrap(), "line\n");
        assert_eq!(decode_c_wide_string("path\\\\here").unwrap(), "path\\here");
        assert_eq!(decode_c_wide_string("\\x41\\x42").unwrap(), "AB");
    }

    #[test]
    fn int_digits_respect_radix_prefix() {
        assert_eq!(parse_int_digits("0x1F"), Some(31));
        assert_eq!(parse_int_digits("0b1010"), Some(10));
        assert_eq!(parse_int_digits("010"), Some(8));
        assert_eq!(parse_int_digits("0"), Some(0));
        assert_eq!(parse_int_digits("42"), Some(42));
        // Invalid octal falls back to decimal.
        assert_eq!(parse_int_digits("08"), Some(8));
    }

    #[test]
    fn integer_literals_take_c11_types() {
        use metadata::Value::{I32, I64, U32, U64};
        let v = |raw, decimal, suffix| integer_value(raw, decimal, suffix, false).unwrap();

        // Bare decimal widens to signed i64 rather than u32.
        assert_eq!(v(42, true, ""), I32(42));
        assert_eq!(v(2147483648, true, ""), I64(2147483648));
        // Hex may become unsigned after signed overflow at the same width.
        assert_eq!(v(31, false, ""), I32(31));
        assert_eq!(v(2147483648, false, ""), U32(2147483648));
        assert_eq!(v(4294967295, false, ""), U32(4294967295));
        assert_eq!(v(100, true, "U"), U32(100));
        assert_eq!(v(4000000000, true, "U"), U32(4000000000));
        assert_eq!(v(4294967296, true, "LL"), I64(4294967296));
        assert_eq!(
            v(18446744073709551615, true, "ULL"),
            U64(18446744073709551615)
        );
        assert_eq!(integer_value(5, true, "", true).unwrap(), I32(-5));
    }

    #[test]
    fn eval_values_use_probe_types() {
        use metadata::Value::{I32, I64, U32, U64};
        assert_eq!(eval_integer_value(1, 1, Some(4), Some(1)), I32(1));
        assert_eq!(eval_integer_value(1, 1, Some(4), Some(2)), U32(1));
        assert_eq!(eval_integer_value(1, 1, Some(8), Some(1)), I64(1));
        assert_eq!(eval_integer_value(1, 1, Some(8), Some(2)), U64(1));
        assert_eq!(
            eval_integer_value(u64::MAX, -1, Some(8), Some(2)),
            U64(u64::MAX)
        );
        assert_eq!(eval_integer_value(100, 100, None, None), U32(100));
        assert_eq!(eval_integer_value(u64::MAX, -1, None, None), I32(-1));
        assert_eq!(eval_integer_value(1, 1, Some(0), Some(0)), U32(1));
    }
}
