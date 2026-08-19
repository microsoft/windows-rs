use super::*;

impl<R: NativeRuntime> Engine<R> {
    pub(super) fn realize<F>(
        &mut self,
        host: NodeId,
        index: usize,
        lease: u64,
        rows: &mut F,
    ) -> Result<(), EngineError>
    where
        F: RowFactory<R> + ?Sized,
    {
        let NodeKind::VirtualHost { realized } = &self
            .arena
            .get(host)
            .ok_or(EngineError::InvalidNode(host))?
            .kind
        else {
            return Err(EngineError::InvalidNode(host));
        };
        if let Some(realized) = realized.get(&index) {
            if realized.lease == lease {
                return Ok(());
            }
            let root = realized.root;
            self.remove_subtree(root)?;
        }

        let key = rows.key(self, host, index)?;
        let row = self
            .parked_virtual_rows
            .get_mut(&host)
            .and_then(|parked| parked.remove(&key))
            .map(Ok)
            .unwrap_or_else(|| rows.mount(self, host, index))?;
        if self
            .arena
            .get(row)
            .ok_or(EngineError::InvalidNode(row))?
            .parent
            .is_some()
        {
            return Err(EngineError::RowRootAlreadyParented(row));
        }
        let position = {
            let NodeKind::VirtualHost { realized } = &self
                .arena
                .get(host)
                .ok_or(EngineError::InvalidNode(host))?
                .kind
            else {
                return Err(EngineError::InvalidNode(host));
            };
            realized.keys().filter(|current| **current < index).count()
        };
        let NodeKind::VirtualHost { realized } = &mut self
            .arena
            .get_mut(host)
            .ok_or(EngineError::InvalidNode(host))?
            .kind
        else {
            return Err(EngineError::InvalidNode(host));
        };
        realized.insert(
            index,
            RealizedRow {
                lease,
                key,
                root: row,
            },
        );
        self.attach_at(host, row, position)?;
        Ok(())
    }

    pub(crate) fn take_virtual_row_roots(
        &mut self,
        host: NodeId,
    ) -> Result<Vec<(u64, NodeId)>, EngineError> {
        let parked = self
            .parked_virtual_rows
            .remove(&host)
            .map(BTreeMap::into_iter)
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
        let rows = {
            let node = self.arena.get(host).ok_or(EngineError::InvalidNode(host))?;
            let NodeKind::VirtualHost { realized } = &node.kind else {
                return Err(EngineError::InvalidNode(host));
            };
            realized
                .iter()
                .map(|(index, row)| (*index, row.clone()))
                .collect::<Vec<_>>()
        };
        // Preserve parked roots across source changes only until WinUI realizes a replacement
        // window. After that, retaining both sets would accumulate rows across scroll cycles.
        let mut roots = if rows.is_empty() {
            parked
        } else {
            for (_, root) in parked {
                self.remove_subtree(root)?;
            }
            Vec::with_capacity(rows.len())
        };
        for (index, row) in &rows {
            let roots = self.projected_native_roots(row.root);
            if roots.len() != 1 {
                return Err(EngineError::VirtualRowNativeRootCount {
                    host,
                    index: *index,
                    count: roots.len(),
                });
            }
            self.pending.push(Command::Detach {
                parent: host,
                child: roots[0],
            });
        }
        let node = self.arena.get_mut(host).unwrap();
        let NodeKind::VirtualHost { realized } = &mut node.kind else {
            unreachable!()
        };
        realized.clear();
        let empty = self.virtual_empty.get(&host).copied();
        node.children.retain(|child| Some(*child) == empty);
        for (_, row) in &rows {
            self.arena.get_mut(row.root).unwrap().parent = None;
        }
        roots.extend(rows.into_iter().map(|(_, row)| (row.key, row.root)));
        Ok(roots)
    }

    pub(crate) fn park_virtual_rows(
        &mut self,
        host: NodeId,
        rows: impl IntoIterator<Item = (u64, NodeId)>,
    ) -> Result<(), EngineError> {
        if !matches!(
            self.arena.get(host).map(|node| &node.kind),
            Some(NodeKind::VirtualHost { .. })
        ) {
            return Err(EngineError::InvalidNode(host));
        }
        let mut parked = BTreeMap::new();
        for (key, root) in rows {
            if self
                .arena
                .get(root)
                .ok_or(EngineError::InvalidNode(root))?
                .parent
                .is_some()
            {
                return Err(EngineError::RowRootAlreadyParented(root));
            }
            parked.insert(key, root);
        }
        if !parked.is_empty() {
            self.parked_virtual_rows.insert(host, parked);
        }
        Ok(())
    }

    pub(crate) fn parked_virtual_rows(&self, host: NodeId) -> Vec<(u64, NodeId)> {
        self.parked_virtual_rows
            .get(&host)
            .into_iter()
            .flat_map(|rows| rows.iter().map(|(key, root)| (*key, *root)))
            .collect()
    }

    pub(crate) fn virtual_empty(&self, host: NodeId) -> Option<NodeId> {
        self.virtual_empty.get(&host).copied()
    }

    pub(crate) fn attach_virtual_empty(
        &mut self,
        host: NodeId,
        root: NodeId,
    ) -> Result<(), EngineError> {
        if self.virtual_empty.contains_key(&host) {
            return Err(EngineError::ParentConflict {
                child: root,
                parent: host,
            });
        }
        self.virtual_empty.insert(host, root);
        let index = self.arena.get(host).unwrap().children.len();
        if let Err(error) = self.attach_at(host, root, index) {
            self.virtual_empty.remove(&host);
            return Err(error);
        }
        Ok(())
    }

    pub(super) fn recycle(
        &mut self,
        host: NodeId,
        index: usize,
        lease: u64,
    ) -> Result<(), EngineError> {
        let row = {
            let node = self
                .arena
                .get_mut(host)
                .ok_or(EngineError::InvalidNode(host))?;
            let NodeKind::VirtualHost { realized } = &mut node.kind else {
                return Err(EngineError::InvalidNode(host));
            };
            if realized.get(&index).is_some_and(|row| row.lease == lease) {
                realized.remove(&index).map(|row| row.root)
            } else {
                None
            }
        };
        if let Some(row) = row {
            self.remove_subtree(row)?;
        }
        Ok(())
    }
}
