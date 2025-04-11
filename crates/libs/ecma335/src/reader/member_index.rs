use super::*;

// pub struct ConstDef<'a> {
//     pub field: Field<'a>,
//     pub namespace: &'a str,
// }

// pub struct FnDef<'a> {
//     pub method: MethodDef<'a>,
//     pub namespace: &'a str,
// }

pub enum Member<'a> {
    Type(TypeDef<'a>),
    Fn(MethodDef<'a>),
    Const(Field<'a>),
}

type HashType<'a> = HashMap<&'a str, HashMap<&'a str, Vec<Member<'a>>>>;

pub struct MemberIndex<'a> {
    pub members: HashType<'a>,
}

impl<'a> MemberIndex<'a> {
    pub fn new(index: &'a Index) -> Self {
        let mut members: HashType = HashMap::new();

        for (namespace, name, ty) in index.iter() {
            insert(&mut members, namespace, name, Member::Type(ty));

            if !ty.flags().contains(TypeAttributes::WindowsRuntime) {
                match ty.category() {
                    TypeCategory::Class if name == "Apis" => {
                        for method in ty.methods() {
                            insert(&mut members, namespace, method.name(), Member::Fn(method));
                        }
                        for field in ty.fields() {
                            insert(&mut members, namespace, field.name(), Member::Const(field));
                        }
                    }
                    TypeCategory::Enum if !ty.has_attribute("ScopedEnumAttribute") => {
                        for field in ty.fields() {
                            if field.flags().contains(FieldAttributes::Literal) {
                                insert(&mut members, namespace, field.name(), Member::Const(field));
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        Self { members }
    }
}

fn insert<'a>(
    members: &mut HashType<'a>,
    namespace: &'a str,
    name: &'a str,
    member: Member<'a>,
) {
    members
        .entry(namespace)
        .or_default()
        .entry(name)
        .or_default()
        .push(member);
}
