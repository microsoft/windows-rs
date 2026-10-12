use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NativeVtable {
    pub size: u32,
    pub inherited: u32,
}

/// Absolute native slot and optional owner of the inherited declaration being overridden.
#[derive(Debug, Clone, PartialEq)]
pub struct NativeSlot {
    pub index: u32,
    pub reuse: Option<TypeName>,
}

#[derive(Debug)]
pub struct NativeSlotError(String);

impl std::fmt::Display for NativeSlotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for NativeSlotError {}

fn slot_attribute(attr: Attribute<'_>) -> bool {
    attr.namespace() == "Windows.Win32.Metadata"
        && matches!(
            attr.name(),
            "NativeVtableAttribute" | "NativeSlotAttribute" | "NativeSlotReuseAttribute"
        )
}

fn descriptor<'a>(
    row: impl HasAttributes<'a>,
    name: &str,
) -> Result<Option<Vec<Value>>, NativeSlotError> {
    let mut attributes = row
        .attributes()
        .filter(|attr| attr.namespace() == "Windows.Win32.Metadata" && attr.name() == name);
    let Some(attr) = attributes.next() else {
        return Ok(None);
    };
    if attributes.next().is_some() {
        return invalid("duplicate native slot descriptor");
    }
    let args = attr.value();
    if args.iter().any(|(name, _)| !name.is_empty()) {
        return invalid("native slot descriptor arguments must be positional");
    }
    Ok(Some(args.into_iter().map(|(_, value)| value).collect()))
}

impl TypeDef<'_> {
    pub fn native_vtable(&self) -> Result<Option<NativeVtable>, NativeSlotError> {
        let Some(args) = descriptor(*self, "NativeVtableAttribute")? else {
            return Ok(None);
        };
        let [Value::I32(size), Value::I32(inherited)] = args.as_slice() else {
            return invalid("invalid native vtable arguments");
        };
        if *inherited < 0 || size < inherited {
            return invalid("native vtable requires 0 <= inherited <= size");
        }
        Ok(Some(NativeVtable {
            size: *size as u32,
            inherited: *inherited as u32,
        }))
    }
}

impl MethodDef<'_> {
    pub fn native_slot(&self) -> Result<Option<NativeSlot>, NativeSlotError> {
        let slot = descriptor(*self, "NativeSlotAttribute")?;
        let reuse = descriptor(*self, "NativeSlotReuseAttribute")?;
        let Some(slot) = slot else {
            if reuse.is_some() {
                return invalid("native slot reuse requires a slot descriptor");
            }
            return Ok(None);
        };
        let [Value::I32(index)] = slot.as_slice() else {
            return invalid("invalid native slot arguments");
        };
        let Ok(index) = u32::try_from(*index) else {
            return invalid("native slot must be nonnegative");
        };
        let reuse = if let Some(args) = reuse {
            let [Value::TypeName(owner)] = args.as_slice() else {
                return invalid("native slot reuse requires a type-valued owner");
            };
            if owner.name.is_empty() || !owner.generics.is_empty() {
                return invalid("native slot reuse requires a nongeneric owner");
            }
            Some(owner.clone())
        } else {
            None
        };
        Ok(Some(NativeSlot { index, reuse }))
    }
}

impl Index {
    /// Checks one explicit native vtable model without deriving ABI order from method names.
    pub fn validate_native_slots(&self, reference: Option<&Self>) -> Result<(), NativeSlotError> {
        let mut interfaces = BTreeSet::new();
        for attr in self.all_attributes().filter(|attr| slot_attribute(*attr)) {
            match (attr.name(), attr.parent()) {
                ("NativeVtableAttribute", HasAttribute::TypeDef(owner)) => {
                    interfaces.insert(owner);
                }
                (
                    "NativeSlotAttribute" | "NativeSlotReuseAttribute",
                    HasAttribute::MethodDef(method),
                ) => {
                    interfaces.insert(method.parent_row::<TypeDef<'_>>(5));
                }
                _ => return invalid("native slot descriptor has an invalid owner"),
            }
        }
        for interface in interfaces {
            if interface.category() != TypeCategory::Interface
                || interface.flags().contains(TypeAttributes::WindowsRuntime)
                || interface.generic_params().next().is_some()
            {
                return invalid("native vtables require nongeneric native interfaces");
            }
            let Some(table) = interface.native_vtable()? else {
                return invalid("native slot requires a vtable model");
            };
            let base = native_base(interface)?;
            if base.is_none() && table.inherited != 0 {
                return invalid("native root interface cannot have inherited slots");
            }
            let arches = interface.arches();
            if let Some(base) = &base {
                for definition in self.slot_definitions(base, arches, reference) {
                    if let Some(base_table) = definition.native_vtable()?
                        && base_table.size != table.inherited
                    {
                        return invalid("native inherited count disagrees with the base vtable");
                    }
                }
            }
            let mut ancestors = Vec::new();
            self.slot_ancestors(
                interface,
                arches,
                reference,
                &mut Vec::new(),
                &mut ancestors,
            )?;
            let mut occupied = BTreeSet::new();
            let mut new_slots = 0;
            for method in interface.methods() {
                if method.arches() != 0 {
                    return invalid(
                        "native slot methods require interface-level architecture variants",
                    );
                }
                let Some(slot) = method.native_slot()? else {
                    return invalid("native vtable method is missing its slot descriptor");
                };
                let mut expected = MethodAttributes::Public
                    | MethodAttributes::HideBySig
                    | MethodAttributes::Abstract
                    | MethodAttributes::Virtual;
                if slot.reuse.is_none() {
                    expected |= MethodAttributes::NewSlot;
                }
                if method.flags().contains(MethodAttributes::SpecialName) {
                    expected |= MethodAttributes::SpecialName;
                }
                if method.flags() != expected
                    || method.impl_flags() != MethodImplAttributes::default()
                    || method.signature(&[]).flags != MethodCallAttributes::HASTHIS
                {
                    return invalid(
                        "native slot method flags disagree with its new/reuse contract",
                    );
                }
                if slot.index >= table.size || !occupied.insert(slot.index) {
                    return invalid("native slots must be unique and within the vtable");
                }
                if let Some(owner) = &slot.reuse {
                    if slot.index >= table.inherited {
                        return invalid("native reused slot must be in the inherited prefix");
                    }
                    if !ancestors.contains(owner) {
                        return invalid("native slot reuse owner is not an available ancestor");
                    }
                    for definition in self.slot_definitions(owner, arches, reference) {
                        if definition.native_vtable()?.is_some() {
                            let mut found = false;
                            for target in definition.methods() {
                                if target
                                    .native_slot()?
                                    .is_some_and(|target| target.index == slot.index)
                                {
                                    found = true;
                                }
                            }
                            if !found {
                                return invalid(
                                    "native slot reuse owner has no declaration at that slot",
                                );
                            }
                        }
                    }
                } else {
                    if slot.index < table.inherited {
                        return invalid("native new slot cannot occupy the inherited prefix");
                    }
                    new_slots += 1;
                }
            }
            if new_slots != table.size - table.inherited {
                return invalid("native new slots must cover the complete vtable suffix");
            }
        }
        Ok(())
    }

    fn slot_definitions<'a>(
        &'a self,
        name: &TypeName,
        arches: i32,
        reference: Option<&'a Self>,
    ) -> Vec<TypeDef<'a>> {
        self.get(&name.namespace, &name.name)
            .chain(
                reference
                    .into_iter()
                    .flat_map(|index| index.get(&name.namespace, &name.name)),
            )
            .filter(|definition| {
                arches == 0 || definition.arches() == 0 || arches & definition.arches() != 0
            })
            .collect()
    }

    fn slot_ancestors(
        &self,
        interface: TypeDef<'_>,
        arches: i32,
        reference: Option<&Self>,
        visiting: &mut Vec<TypeName>,
        ancestors: &mut Vec<TypeName>,
    ) -> Result<(), NativeSlotError> {
        let name = interface.full_name();
        if visiting.contains(&name) {
            return invalid("cyclic native interface inheritance");
        }
        visiting.push(name);
        if let Some(base) = native_base(interface)? {
            if !ancestors.contains(&base) {
                ancestors.push(base.clone());
            }
            let mut common: Option<Vec<TypeName>> = None;
            for definition in self.slot_definitions(&base, arches, reference) {
                if definition.category() != TypeCategory::Interface
                    || definition.flags().contains(TypeAttributes::WindowsRuntime)
                    || definition.generic_params().next().is_some()
                {
                    return invalid("native vtable base must be a nongeneric native interface");
                }
                let target_arches = if arches == 0 {
                    definition.arches()
                } else if definition.arches() == 0 {
                    arches
                } else {
                    arches & definition.arches()
                };
                let mut path = Vec::new();
                self.slot_ancestors(definition, target_arches, reference, visiting, &mut path)?;
                if let Some(common) = &mut common {
                    common.retain(|name| path.contains(name));
                } else {
                    common = Some(path);
                }
            }
            for ancestor in common.into_iter().flatten() {
                if !ancestors.contains(&ancestor) {
                    ancestors.push(ancestor);
                }
            }
        }
        visiting.pop();
        Ok(())
    }
}

fn native_base(interface: TypeDef<'_>) -> Result<Option<TypeName>, NativeSlotError> {
    let mut bases = interface.interface_impls();
    let Some(base) = bases.next() else {
        return Ok(None);
    };
    if bases.next().is_some() {
        return invalid("native vtables require single inheritance");
    }
    let Type::ClassName(name) = base.interface(&[]) else {
        return invalid("native vtable base must be an interface type");
    };
    if !name.generics.is_empty() {
        return invalid("native vtable base must be nongeneric");
    }
    Ok(Some(name))
}

fn invalid<T>(message: &str) -> Result<T, NativeSlotError> {
    Err(NativeSlotError(message.to_string()))
}
