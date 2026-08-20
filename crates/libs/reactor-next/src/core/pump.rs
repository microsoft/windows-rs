use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PumpError {
    AlreadyMounted,
    KindChanged,
    NotMounted,
    StructureUnsupported,
    Tree(TreeError),
}

impl From<TreeError> for PumpError {
    fn from(value: TreeError) -> Self {
        Self::Tree(value)
    }
}

struct PropertyCommit {
    command: usize,
    node: NodeId,
    property: PropertyId,
    value: Option<PropertyValue>,
}

pub struct Pump<R> {
    tree: Tree,
    runtime: R,
    root: Option<NodeId>,
}

impl<R: NativeRuntime> Pump<R> {
    pub fn new(runtime: R) -> Self {
        Self {
            tree: Tree::new(),
            runtime,
            root: None,
        }
    }

    pub fn mount(&mut self, element: Element) -> Result<CommitReceipt, PumpError> {
        if self.root.is_some() {
            return Err(PumpError::AlreadyMounted);
        }
        let mut commands = Vec::new();
        let mut commits = Vec::new();
        let node = self.mount_element(None, None, element, &mut commands, &mut commits)?;

        let receipt = self.runtime.apply(&commands);
        let structural_failure = commands
            .iter()
            .enumerate()
            .any(|(index, command)| command.structural() && !receipt.applied(index));
        if structural_failure {
            let rollback = commands
                .iter()
                .enumerate()
                .rev()
                .filter(|(index, _)| receipt.applied(*index))
                .filter_map(|(_, command)| match command {
                    Command::Create { node, .. } => Some(Command::Destroy { node: *node }),
                    Command::InsertChild { parent, child, .. } => Some(Command::RemoveChild {
                        parent: *parent,
                        child: *child,
                    }),
                    _ => None,
                })
                .collect::<Vec<_>>();
            if !rollback.is_empty() {
                self.runtime.apply(&rollback);
            }
            self.tree.retire_subtree(node)?;
            return Ok(receipt);
        }
        self.commit_properties(&commits, &receipt)?;
        self.root = Some(node);
        Ok(receipt)
    }

    pub fn update(&mut self, element: Element) -> Result<CommitReceipt, PumpError> {
        let node = self.root.ok_or(PumpError::NotMounted)?;
        let parts = element.into_parts();
        if parts.structure != ElementStructure::None {
            return Err(PumpError::StructureUnsupported);
        }
        let NodeKind::Native(kind) = self.tree.kind(node)? else {
            return Err(PumpError::NotMounted);
        };
        if kind != parts.kind {
            return Err(PumpError::KindChanged);
        }

        let mut commands = Vec::new();
        let mut commits = Vec::new();
        let committed = &self.tree.native(node)?.committed;
        parts.props.visit_properties(&mut |property, value| {
            let changed = match &value {
                Some(value) => committed.get(&property) != Some(value),
                None => committed.contains_key(&property),
            };
            if !changed {
                return;
            }

            let command = commands.len();
            match &value {
                Some(value) => commands.push(Command::SetProperty {
                    node,
                    property,
                    value: value.clone(),
                }),
                None => commands.push(Command::ClearProperty { node, property }),
            }
            commits.push(PropertyCommit {
                command,
                node,
                property,
                value,
            });
        });

        self.tree.native_mut(node)?.desired = parts.props;
        if commands.is_empty() {
            return Ok(CommitReceipt {
                outcomes: Vec::new(),
            });
        }

        let receipt = self.runtime.apply(&commands);
        self.commit_properties(&commits, &receipt)?;
        Ok(receipt)
    }

    pub fn runtime(&self) -> &R {
        &self.runtime
    }

    pub fn runtime_mut(&mut self) -> &mut R {
        &mut self.runtime
    }

    pub fn root(&self) -> Option<NodeId> {
        self.root
    }

    fn commit_properties(
        &mut self,
        commits: &[PropertyCommit],
        receipt: &CommitReceipt,
    ) -> Result<(), PumpError> {
        for commit in commits {
            if receipt.applied(commit.command) {
                let committed = &mut self.tree.native_mut(commit.node)?.committed;
                if let Some(value) = &commit.value {
                    committed.insert(commit.property, value.clone());
                } else {
                    committed.remove(&commit.property);
                }
            }
        }
        Ok(())
    }

    fn mount_element(
        &mut self,
        parent: Option<NodeId>,
        key: Option<Key>,
        element: Element,
        commands: &mut Vec<Command>,
        commits: &mut Vec<PropertyCommit>,
    ) -> Result<NodeId, PumpError> {
        let parts = element.into_parts();
        let node = self
            .tree
            .insert_native(parent, parts.kind, key, parts.props.clone())?;
        commands.push(Command::Create {
            node,
            kind: parts.kind,
        });
        parts.props.visit_properties(&mut |property, value| {
            if let Some(value) = value {
                commits.push(PropertyCommit {
                    command: commands.len(),
                    node,
                    property,
                    value: Some(value.clone()),
                });
                commands.push(Command::SetProperty {
                    node,
                    property,
                    value,
                });
            }
        });

        match parts.structure {
            ElementStructure::None => {}
            ElementStructure::Content(content) => {
                if let Some(content) = content {
                    let child = self.mount_element(Some(node), None, content, commands, commits)?;
                    commands.push(Command::InsertChild {
                        parent: node,
                        child,
                        index: 0,
                    });
                }
            }
            ElementStructure::Children(children) => {
                for (index, child) in children.into_iter().enumerate() {
                    let (key, child) = child.into_parts();
                    let child =
                        self.mount_element(Some(node), Some(key), child, commands, commits)?;
                    commands.push(Command::InsertChild {
                        parent: node,
                        child,
                        index,
                    });
                }
            }
        }
        Ok(node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::native::*;

    #[test]
    fn mount_update_clear_and_no_change_follow_receipts() {
        let mut pump = Pump::new(RecordingRuntime::default());
        let mounted = pump.mount(TextBlock::new().text("first").into()).unwrap();
        let root = pump.root().unwrap();

        assert_eq!(mounted.outcomes.len(), 2);
        assert_eq!(
            pump.runtime()
                .node(root)
                .unwrap()
                .property(PropertyId::TextBlockText),
            Some(&PropertyValue::Str("first".into()))
        );

        let updated = pump.update(TextBlock::new().text("second").into()).unwrap();
        assert_eq!(updated.outcomes, [CommandOutcome::Applied]);
        assert_eq!(
            pump.runtime()
                .node(root)
                .unwrap()
                .property(PropertyId::TextBlockText),
            Some(&PropertyValue::Str("second".into()))
        );

        let batches = pump.runtime().batches();
        let unchanged = pump.update(TextBlock::new().text("second").into()).unwrap();
        assert!(unchanged.outcomes.is_empty());
        assert_eq!(pump.runtime().batches(), batches);

        let cleared = pump.update(TextBlock::new().into()).unwrap();
        assert_eq!(cleared.outcomes, [CommandOutcome::Applied]);
        assert!(
            pump.runtime()
                .node(root)
                .unwrap()
                .property(PropertyId::TextBlockText)
                .is_none()
        );
    }

    #[test]
    fn failed_property_is_not_committed_and_retries() {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(TextBlock::new().text("first").into()).unwrap();
        let root = pump.root().unwrap();
        pump.runtime_mut().fail_at(0);

        let failed = pump.update(TextBlock::new().text("second").into()).unwrap();
        assert_eq!(
            failed.outcomes,
            [CommandOutcome::Failed(RuntimeError::Injected)]
        );
        assert_eq!(
            pump.runtime()
                .node(root)
                .unwrap()
                .property(PropertyId::TextBlockText),
            Some(&PropertyValue::Str("first".into()))
        );

        let retried = pump.update(TextBlock::new().text("second").into()).unwrap();
        assert_eq!(retried.outcomes, [CommandOutcome::Applied]);
        assert_eq!(
            pump.runtime()
                .node(root)
                .unwrap()
                .property(PropertyId::TextBlockText),
            Some(&PropertyValue::Str("second".into()))
        );
    }

    #[test]
    fn failed_create_does_not_publish_a_root() {
        let mut runtime = RecordingRuntime::default();
        runtime.fail_at(0);
        let mut pump = Pump::new(runtime);

        let failed = pump.mount(TextBlock::new().text("first").into()).unwrap();

        assert_eq!(
            failed.outcomes,
            [
                CommandOutcome::Failed(RuntimeError::Injected),
                CommandOutcome::Skipped,
            ]
        );
        assert_eq!(pump.root(), None);
        assert!(pump.runtime().is_empty());
    }

    #[test]
    fn mounts_content_and_keyed_children_recursively() {
        let mut pump = Pump::new(RecordingRuntime::default());
        let tree = StackPanel::new()
            .child("text", TextBlock::new().text("value"))
            .child(
                "button",
                Button::new().content(TextBlock::new().text("increment")),
            );

        pump.mount(tree.into()).unwrap();

        let root = pump.root().unwrap();
        let children = pump.runtime().node(root).unwrap().children();
        assert_eq!(children.len(), 2);
        let button = children[1];
        assert_eq!(pump.runtime().node(button).unwrap().children().len(), 1);
    }

    #[test]
    fn structural_mount_failure_removes_created_nodes() {
        let mut runtime = RecordingRuntime::default();
        runtime.fail_at(1);
        let mut pump = Pump::new(runtime);
        let tree = StackPanel::new().child("text", TextBlock::new().text("value"));

        let failed = pump.mount(tree.into()).unwrap();

        assert!(matches!(
            failed.outcomes[1],
            CommandOutcome::Failed(RuntimeError::Injected)
        ));
        assert_eq!(pump.root(), None);
        assert!(pump.runtime().is_empty());
    }
}
