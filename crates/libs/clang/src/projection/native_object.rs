use super::*;

impl Projector<'_, '_> {
    pub(super) fn native_object(
        &self,
        snapshot_id: usize,
        observation: &RecordObservation,
        name: &TokenStream,
        inline_record: bool,
    ) -> Result<Body, Error> {
        let snapshot = self.resolution.snapshots[snapshot_id];
        let definition = observation.definition.as_ref().unwrap();
        if inline_record
            || snapshot.records[observation.record.0].owner.is_some()
            || !definition.fields.is_empty()
            || !definition.methods.is_empty()
            || !definition.enums.is_empty()
            || definition.semantics.non_trivial_destructor
            || snapshot.records.iter().any(|record| {
                record
                    .owner
                    .is_some_and(|owner| owner.record == observation.record)
            })
        {
            return Err(Error(format!(
                "native object `{name}` requires only public nonvirtual interface bases"
            )));
        }
        if observation.attributes.iter().any(|attribute| {
            !matches!(
                attribute.kind,
                AttributeKind::Uuid(_)
                    | AttributeKind::NoVTable
                    | AttributeKind::DefaultTypeVisibility
                    | AttributeKind::MaxFieldAlignment { .. }
                    | AttributeKind::Sal(_)
                    | AttributeKind::Deprecated(_)
            )
        }) {
            return Err(Error(format!(
                "unsupported native object attributes on `{name}`"
            )));
        }
        let pointer = if snapshot.target == Target::X86 { 4 } else { 8 };
        let packing = resolve::packing(&observation.attributes)
            .1
            .map(|bits| u64::from(bits / 8));
        let mut fields = Vec::new();
        for (position, base) in definition.bases.iter().enumerate() {
            let reference = self
                .resolution
                .group(RecordRef {
                    snapshot: snapshot_id,
                    record: base.record,
                })
                .definition
                .ok_or_else(|| Error(format!("native object `{name}` has an incomplete base")))?;
            let base_definition = self.resolution.snapshots[reference.snapshot].observations
                [reference.observation.0]
                .definition
                .as_ref()
                .unwrap();
            if base.offset_bits != position as u64 * pointer * 8
                || base_definition.layout
                    != (Layout {
                        size: pointer,
                        alignment: pointer,
                    })
            {
                return Err(Error(format!("unsupported native base layout on `{name}`")));
            }
            let field = ident(&format!("base{position}"))?;
            let ty = write_type(
                self.namespace,
                &self.interface_type(snapshot_id, base.record),
            );
            fields.push(quote! { #[base] #field: #ty, });
        }
        let layout = Layout {
            size: definition.bases.len() as u64 * pointer,
            alignment: packing.map_or(pointer, |packing| pointer.min(packing)),
        };
        if definition.layout != layout {
            return Err(Error(format!(
                "unsupported native object layout on `{name}`"
            )));
        }
        let packed = packing.map(|packing| {
            let packing = proc_macro2::Literal::u64_unsuffixed(packing);
            quote! { #[packed(#packing)] }
        });
        let guid = resolve::uuid(&observation.attributes).map(|uuid| {
            let uuid = values::write_guid(uuid);
            quote! { #[guid(#uuid)] }
        });
        let attributes = annotations::write(&observation.attributes)?;
        Ok(Body {
            abi: None,
            attributes: quote! { #attributes #packed #guid },
            declaration: quote! { struct #name { #(#fields)* } },
        })
    }
}
