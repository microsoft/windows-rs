use super::*;

pub fn format_idl(source: &str) -> Result<String> {
    let mut printer = Printer::new();
    printer.idl_file(&IdlFile::parse_str(source)?);
    Ok(printer.out)
    // TODO: workaround until we have complete formatting support
    //Ok(source.to_string())
}

#[derive(Default)]
struct Printer {
    out: String,
    indent: usize,
    newline: bool,
}

impl Printer {
    fn new() -> Self {
        Self::default()
    }

    fn word(&mut self, value: &str) {
        if self.newline {
            self.newline = false;
            self.out.push('\n');
            for _ in 0..self.indent {
                self.out.push_str("    ");
            }
        }

        self.out.push_str(value);
    }

    fn newline(&mut self) {
        self.newline = true;
    }

    fn idl_file(&mut self, file: &IdlFile) {
        for reference in &file.references {
            self.item_use(reference);
        }

        for module in &file.modules {
            self.idl_module(module);
        }
    }

    fn idl_module(&mut self, module: &IdlModule) {
        self.word("mod ");
        self.ident(&module.ident);
        self.word(" {");
        self.newline();
        self.indent += 1;

        for member in &module.members {
            self.idl_module_member(member);
            self.newline();
        }

        self.indent -= 1;
        self.newline();
        self.word("}");
    }

    fn idl_module_member(&mut self, member: &IdlModuleMember) {
        match member {
            IdlModuleMember::Module(member) => self.idl_module(member),
            IdlModuleMember::Interface(member) => self.idl_interface(member),
            IdlModuleMember::Struct(member) => self.idl_struct(member),
            IdlModuleMember::Enum(member) => self.idl_enum(member),
            IdlModuleMember::Class(member) => self.idl_class(member),
        }
    }

    fn idl_interface(&mut self, member: &IdlInterface) {
        self.attrs(&member.attributes);
        self.word("interface ");
        self.ident(&member.ident);
        self.word(" {");
        self.newline();
        self.indent += 1;

        for method in &member.methods {
            self.trait_item_method(method);
            self.word(";");
            self.newline();
        }

        self.indent -= 1;
        self.newline();
        self.word("}");
    }

    fn attrs(&mut self, attrs: &[syn::Attribute]) {
        for attr in attrs {
            self.attr(attr);
        }
    }

    fn attr(&mut self, attr: &syn::Attribute) {
        self.word("#[");
        self.path(&attr.path);
        self.word("]");
        self.newline();
    }

    fn idl_struct(&mut self, member: &IdlStruct) {
        self.attrs(&member.item.attrs);

        self.word("struct ");
        self.ident(&member.item.ident);
        self.word(" {");
        self.newline();
        self.indent += 1;

        if let syn::Fields::Named(fields) = &member.item.fields {
            for field in &fields.named {
                self.field(field);
                self.word(",");
                self.newline();
            }
        }

        self.indent -= 1;
        self.newline();
        self.word("}");
    }

    fn idl_enum(&mut self, member: &IdlEnum) {
        self.attrs(&member.item.attrs);

        self.word("enum ");
        self.ident(&member.item.ident);
        self.word(" {");
        self.newline();
        self.indent += 1;

        for variant in &member.item.variants {
            self.ident(&variant.ident);
            if let Some((_, expr)) = &variant.discriminant {
                self.word(" = ");
                self.expr(expr);
            }
            self.word(",");
            self.newline();
        }

        self.indent -= 1;
        self.newline();
        self.word("}");
    }

    fn idl_class(&mut self, _member: &IdlClass) {}

    fn trait_item_method(&mut self, method: &syn::TraitItemMethod) {
        self.signature(&method.sig);
    }

    fn signature(&mut self, signature: &syn::Signature) {
        self.word("fn ");
        self.ident(&signature.ident);
        self.word("(");

        let mut first = true;
        for input in &signature.inputs {
            if first {
                first = false;
            } else {
                self.word(", ");
            }
            self.fn_arg(input);
        }

        self.word(")");

        if let syn::ReturnType::Type(_, ty) = &signature.output {
            self.word(" -> ");
            self.ty(ty);
        }
    }

    fn fn_arg(&mut self, fn_arg: &syn::FnArg) {
        if let syn::FnArg::Typed(pat_type) = fn_arg {
            self.pat_type(pat_type);
        }
    }

    fn pat_type(&mut self, pat_type: &syn::PatType) {
        self.pat(&pat_type.pat);
        self.word(": ");
        self.ty(&pat_type.ty);
    }

    fn pat(&mut self, pat: &syn::Pat) {
        match pat {
            syn::Pat::Ident(pat_ident) => self.pat_ident(pat_ident),
            _ => todo!("{:?}", pat),
        }
    }

    fn pat_ident(&mut self, pat_ident: &syn::PatIdent) {
        self.ident(&pat_ident.ident);
    }

    fn field(&mut self, field: &syn::Field) {
        if let Some(ref ident) = field.ident {
            self.ident(ident);
            self.word(": ");
            self.ty(&field.ty);
        }
    }

    fn ty(&mut self, ty: &syn::Type) {
        match ty {
            syn::Type::Path(ty) => self.type_path(ty),
            syn::Type::Ptr(ptr) => self.type_ptr(ptr),
            syn::Type::Array(array) => self.type_array(array),
            _ => todo!("{:?}", ty),
        }
    }

    fn type_array(&mut self, array: &syn::TypeArray) {
        self.word("[");
        self.ty(&array.elem);
        self.word("; ");
        self.expr(&array.len);
        self.word("]");
    }

    fn expr(&mut self, expr: &syn::Expr) {
        match expr {
            syn::Expr::Lit(lit) => self.expr_lit(lit),
            syn::Expr::Unary(unary) => self.expr_unary(unary),
            _ => todo!("{:?}", expr),
        }
    }

    fn expr_unary(&mut self, unary: &syn::ExprUnary) {
        self.word("-");
        self.expr(&unary.expr);
    }

    fn expr_lit(&mut self, expr: &syn::ExprLit) {
        self.lit(&expr.lit);
    }

    fn lit(&mut self, lit: &syn::Lit) {
        match lit {
            syn::Lit::Int(lit) => self.lit_int(lit),
            syn::Lit::Str(lit) => self.lit_str(lit),
            _ => _ = dbg!(lit),
        }
    }

    fn lit_str(&mut self, lit: &syn::LitStr) {
        self.word("\"");
        self.word(&lit.value());
        self.word("\"");
    }

    fn lit_int(&mut self, lit: &syn::LitInt) {
        self.word(&lit.token().to_string());
    }

    fn type_ptr(&mut self, ptr: &syn::TypePtr) {
        if ptr.mutability.is_some() {
            self.word("*mut ");
        } else {
            self.word("*const ");
        }
        self.ty(&ptr.elem);
    }

    fn type_path(&mut self, ty: &syn::TypePath) {
        self.path(&ty.path);
    }

    fn path(&mut self, path: &syn::Path) {
        let mut first = true;
        for segment in &path.segments {
            if first {
                first = false;
            } else {
                self.word("::");
            }
            self.path_segment(segment);
        }
    }

    pub fn path_segment(&mut self, segment: &syn::PathSegment) {
        self.ident(&segment.ident);
    }

    fn item_use(&mut self, item: &syn::ItemUse) {
        self.word("use ");
        self.use_tree(&item.tree);
        self.word(";");
        self.newline();
    }

    fn use_tree(&mut self, use_tree: &syn::UseTree) {
        match use_tree {
            syn::UseTree::Path(use_path) => self.use_path(use_path),
            syn::UseTree::Name(use_name) => self.use_name(use_name),
            _ => {}
        }
    }

    fn use_path(&mut self, use_path: &syn::UsePath) {
        self.ident(&use_path.ident);
        self.word("::");
        self.use_tree(&use_path.tree);
    }

    fn use_name(&mut self, use_name: &syn::UseName) {
        self.ident(&use_name.ident);
    }

    pub fn ident(&mut self, ident: &syn::Ident) {
        self.word(&ident.to_string());
    }
}
