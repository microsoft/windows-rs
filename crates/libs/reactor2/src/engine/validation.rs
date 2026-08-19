use super::*;

impl<R: NativeRuntime> Engine<R> {
    pub(crate) fn validate_application_root(
        &self,
        root: NodeId,
    ) -> Result<Option<Vec<NodeId>>, EngineError> {
        fn applications<R: NativeRuntime>(engine: &Engine<R>, id: NodeId, found: &mut Vec<NodeId>) {
            let Some(node) = engine.arena.get(id) else {
                return;
            };
            if matches!(node.kind, NodeKind::Application) {
                found.push(id);
            }
            for child in &node.children {
                applications(engine, *child, found);
            }
        }

        #[derive(Clone, Copy, PartialEq, Eq)]
        enum Scope {
            OutsideApplication,
            WindowList,
            WindowContent,
        }

        fn validate<R: NativeRuntime>(
            engine: &Engine<R>,
            id: NodeId,
            scope: Scope,
            windows: &mut Vec<NodeId>,
        ) -> Result<(), EngineError> {
            let node = engine.arena.get(id).ok_or(EngineError::InvalidNode(id))?;
            match node.kind {
                NodeKind::Application => {
                    if scope != Scope::OutsideApplication {
                        return Err(EngineError::InvalidApplicationTree(
                            "Application must be the single logical root",
                        ));
                    }
                    for child in &node.children {
                        validate(engine, *child, Scope::WindowList, windows)?;
                    }
                }
                NodeKind::Window => {
                    if scope == Scope::OutsideApplication {
                        return Err(EngineError::InvalidApplicationTree(
                            "Window must be owned by Application",
                        ));
                    }
                    if scope != Scope::WindowList {
                        return Err(EngineError::InvalidApplicationTree(
                            "Window may only appear in an application or owned-window slot",
                        ));
                    }
                    if node.children.len() != 2 {
                        return Err(EngineError::InvalidApplicationTree(
                            "Window must own content and an owned-window slot",
                        ));
                    }
                    let count = engine.projected_native_roots(node.children[0]).len();
                    if count != 1 {
                        return Err(EngineError::WindowContentNativeRootCount {
                            window: id,
                            count,
                        });
                    }
                    windows.push(id);
                    validate(engine, node.children[0], Scope::WindowContent, windows)?;
                    validate(engine, node.children[1], Scope::WindowList, windows)?;
                }
                _ => {
                    if node.kind.is_native() && scope != Scope::WindowContent {
                        return Err(EngineError::InvalidApplicationTree(
                            "native controls must be owned by a Window",
                        ));
                    }
                    for child in &node.children {
                        validate(engine, *child, scope, windows)?;
                    }
                }
            }
            Ok(())
        }

        let mut found = Vec::new();
        applications(self, root, &mut found);
        let Some(application) = found.first().copied() else {
            return Ok(None);
        };
        if found.len() != 1 {
            return Err(EngineError::InvalidApplicationTree(
                "the Reactor root must contain exactly one Application",
            ));
        }
        fn validate_window_slot<R: NativeRuntime>(
            engine: &Engine<R>,
            roots: &[NodeId],
        ) -> Result<(), EngineError> {
            fn collect<R: NativeRuntime>(
                engine: &Engine<R>,
                id: NodeId,
                windows: &mut Vec<NodeId>,
            ) {
                let node = engine.arena.get(id).unwrap();
                if matches!(node.kind, NodeKind::Window) {
                    windows.push(id);
                } else {
                    for child in &node.children {
                        collect(engine, *child, windows);
                    }
                }
            }

            let mut windows = Vec::new();
            for root in roots {
                collect(engine, *root, &mut windows);
            }
            if windows.len() > 1 {
                let mut keys = BTreeSet::new();
                for window in &windows {
                    let Some(key) = engine
                        .arena
                        .get(*window)
                        .and_then(|node| node.mounted.as_ref())
                        .and_then(|mounted| mounted.key)
                    else {
                        return Err(EngineError::InvalidApplicationTree(
                            "sibling windows must have unique keys",
                        ));
                    };
                    if !keys.insert(key) {
                        return Err(EngineError::DuplicateSiblingKey { key });
                    }
                }
            }
            for window in windows {
                let owned_root = engine.arena.get(window).unwrap().children[1];
                validate_window_slot(engine, &[owned_root])?;
            }
            Ok(())
        }

        let mut windows = Vec::new();
        validate(self, root, Scope::OutsideApplication, &mut windows)?;
        let application_children = &self.arena.get(application).unwrap().children;
        validate_window_slot(self, application_children)?;
        Ok(Some(windows))
    }

    pub(crate) fn sync_window_content_roots(
        &mut self,
        windows: &[NodeId],
    ) -> Result<(), EngineError> {
        for window in windows {
            let (branch, current) = {
                let node = self
                    .arena
                    .get(*window)
                    .ok_or(EngineError::InvalidNode(*window))?;
                let current = match node.mounted.as_ref().map(|mounted| &mounted.kind) {
                    Some(MountedKind::Window(window)) => window.content,
                    _ => return Err(EngineError::InvalidNode(*window)),
                };
                (node.children[0], current)
            };
            let content = self.single_projected_native_root(branch).ok_or(
                EngineError::WindowContentNativeRootCount {
                    window: *window,
                    count: self.projected_native_root_count(branch),
                },
            )?;
            if content != current {
                self.set_window_content(*window, content)?;
                let node = self.arena.get_mut(*window).unwrap();
                let Some(MountedKind::Window(window)) =
                    node.mounted.as_mut().map(|mounted| &mut mounted.kind)
                else {
                    unreachable!()
                };
                window.content = content;
            }
        }
        Ok(())
    }
}
