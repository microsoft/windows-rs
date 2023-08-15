use super::*;

// TODO: should we use rustfmt in the short term (with pre/post)

#[derive(Default)]
pub struct Writer {
    out: String,
    indent: usize,
    newline: bool,
}

impl Writer {
    pub fn new(file: &rdl::File) -> Self {
        let mut writer = Self::default();
        writer.rdl_file(file);
        writer
    }

    pub fn into_string(mut self) -> String {
        self.out.push('\n');
        self.out
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

    fn rdl_file(&mut self, file: &rdl::File) {
        if file.winrt {
            self.word("#![winrt]\n");
        } else {
            self.word("#![win32]\n");
        }

        self.newline();

        for reference in &file.references {
            self.item_use(reference);
        }

        for module in &file.modules {
            self.rdl_module(module);
        }
    }

    fn rdl_module(&mut self, module: &rdl::Module) {
        self.word("mod ");
        self.word(module.name());
        self.word(" {");
        self.newline();
        self.indent += 1;

        for member in &module.members {
            self.rdl_module_member(member);
            self.newline();
        }

        self.indent -= 1;
        self.newline();
        self.word("}");
        self.newline();
    }

    fn rdl_module_member(&mut self, member: &rdl::ModuleMember) {
        match member {
            rdl::ModuleMember::Module(member) => self.rdl_module(member),
            rdl::ModuleMember::Interface(member) => self.rdl_interface(member),
            rdl::ModuleMember::Struct(member) => self.rdl_struct(member),
            rdl::ModuleMember::Enum(member) => self.rdl_enum(member),
            rdl::ModuleMember::Class(member) => self.rdl_class(member),
            rdl::ModuleMember::Constant(member) => self.rdl_constant(member),
            rdl::ModuleMember::Function(member) => self.rdl_function(member),
        }
    }

    fn rdl_class(&mut self, member: &rdl::Class) {
        self.attrs(&member.attributes);
        self.word("class ");
        self.word(&member.name);

        if !member.extends.is_empty() || member.base.is_some() {
            self.word(" : ");

            if let Some(path) = &member.base {
                self.word("class ");
                self.type_path(path);

                if !member.extends.is_empty() {
                    self.word(", ");
                }
            }

            let mut first = true;
            for path in &member.extends {
                if first {
                    first = false;
                } else {
                    self.word(", ");
                }
                self.type_path(path);
            }
        }

        self.word(";");
        self.newline();
    }

    fn rdl_interface(&mut self, member: &rdl::Interface) {
        self.attrs(&member.attributes);
        self.word("interface ");
        self.word(&member.name);

        if !member.generics.is_empty() {
            self.word("<");

            let mut first = true;
            for generic in &member.generics {
                if first {
                    first = false;
                } else {
                    self.word(", ");
                }
                self.word(generic);
            }

            self.word(">");
        }

        if !member.extends.is_empty() {
            self.word(" : ");

            let mut first = true;
            for path in &member.extends {
                if first {
                    first = false;
                } else {
                    self.word(", ");
                }
                self.type_path(path);
            }
        }

        self.word(" {");
        self.newline();
        self.indent += 1;

        for method in &member.methods {
            self.trait_item_fn(method);
            self.word(";");
            self.newline();
        }

        self.indent -= 1;
        self.newline();
        self.word("}");
    }

    fn rdl_constant(&mut self, member: &rdl::Constant) {
        self.item_const(&member.item);
    }

    fn rdl_function(&mut self, member: &rdl::Function) {
        self.trait_item_fn(&member.item);
        self.word(";");
        self.newline();
    }

    fn item_const(&mut self, item: &syn::ItemConst) {
        self.word("const ");
        self.ident(&item.ident);
        self.word(": ");
        self.ty(&item.ty);
        self.word(" = ");
        self.expr(&item.expr);
        self.word(";");
        self.newline();
    }

    fn attrs(&mut self, attrs: &[syn::Attribute]) {
        for attr in attrs {
            self.attr(attr);
        }
    }

    fn attr(&mut self, attr: &syn::Attribute) {
        self.word("#[");
        self.meta(&attr.meta);
        self.word("]");
        self.newline();
    }

    fn meta(&mut self, meta: &syn::Meta) {
        match meta {
            syn::Meta::Path(path) => self.path(path),
            syn::Meta::List(list) => self.meta_list(list),
            syn::Meta::NameValue(meta) => self.meta_name_value(meta),
        }
    }

    fn meta_list(&mut self, meta_list: &syn::MetaList) {
        self.path(&meta_list.path);
        self.word("(");
        self.word(&meta_list.tokens.to_string());
        self.word(")");
    }

    fn meta_name_value(&mut self, meta: &syn::MetaNameValue) {
        self.path(&meta.path);
        self.word(" = ");
        self.expr(&meta.value);
    }

    fn rdl_struct(&mut self, member: &rdl::Struct) {
        self.attrs(&member.attributes);

        self.word("struct ");
        self.word(&member.name);
        self.word(" {");
        self.newline();
        self.indent += 1;

        for field in &member.fields {
            self.word(&field.name);
            self.word(": ");
            self.ty(&field.ty);
            self.word(",");
            self.newline();
        }

        self.indent -= 1;
        self.newline();
        self.word("}");
    }

    fn rdl_enum(&mut self, member: &rdl::Enum) {
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

    fn trait_item_fn(&mut self, method: &syn::TraitItemFn) {
        self.attrs(&method.attrs);
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
            rest => unimplemented!("{rest:?}"),
        }
    }

    fn pat_ident(&mut self, pat_ident: &syn::PatIdent) {
        self.ident(&pat_ident.ident);
    }

    fn ty(&mut self, ty: &syn::Type) {
        match ty {
            syn::Type::Path(ty) => self.type_path(ty),
            syn::Type::Ptr(ptr) => self.type_ptr(ptr),
            syn::Type::Array(array) => self.type_array(array),
            rest => unimplemented!("{rest:?}"),
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
            rest => unimplemented!("{rest:?}"),
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

        if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
            self.word("<");

            let mut first = true;
            for arg in &args.args {
                if first {
                    first = false;
                } else {
                    self.word(", ");
                }
                self.generic_argument(arg);
            }

            self.word(">");
        }
    }

    fn generic_argument(&mut self, arg: &syn::GenericArgument) {
        match arg {
            syn::GenericArgument::Type(ty) => self.ty(ty),
            rest => unimplemented!("{rest:?}"),
        }
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
