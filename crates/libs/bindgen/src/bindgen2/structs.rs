use super::*;

pub fn gen(def: TypeDef, gen: &Gen) -> TokenStream {
    if gen.reader.type_def_is_contract(def) {
        return quote! {};
    }

    if !gen.sys {
        if let Some(replacement) = replacements::gen(gen.reader.type_def_type_name(def)) {
            return replacement;
        }
    }

    if gen.reader.type_def_is_handle(def) {
        return handles::gen(def, gen);
    }

    // gen_struct_with_name(def, def.name(), &Cfg::new(), gen)
    
    " ".into()
}
