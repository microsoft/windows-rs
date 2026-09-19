use super::bindings as native;
use crate::{
    Adapter, Mutation, ObjectId, ObjectType, Property, PropertyId, PropertyValue, Realization,
    RelationContract, RelationId, relation_contracts,
};
use std::collections::{HashMap, HashSet};
use windows_core::{HSTRING, IInspectable, Interface};

#[derive(Clone)]
enum Handle {
    TextBlock(native::TextBlock),
    Border(native::Border),
    Grid(native::Grid),
    StackPanel(native::StackPanel),
    TreeView(native::TreeView),
    TreeNode(NativeTreeNode),
    ListView(native::ListView),
    Data(IInspectable),
}

#[derive(Clone)]
struct NativeTreeNode {
    value: native::TreeViewNode,
    text: HSTRING,
    content: Option<ObjectId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WinUiError {
    DuplicateObject(ObjectId),
    MissingObject(ObjectId),
    InvalidObject(ObjectId),
    InvalidRelation(ObjectId, RelationId),
    InvalidMutation(RelationId),
    StateMismatch(ObjectId),
    MissingContract(ObjectType, RelationId),
    IndexOverflow(usize),
    ChildNotFound(ObjectId),
    StillOwned(ObjectId),
    Native(windows_core::Error),
}

impl From<windows_core::Error> for WinUiError {
    fn from(value: windows_core::Error) -> Self {
        Self::Native(value)
    }
}

#[derive(Default)]
pub struct WinUiAdapter {
    handles: HashMap<ObjectId, Handle>,
    owners: HashMap<ObjectId, (ObjectId, RelationId)>,
    tree_template: Option<native::DataTemplate>,
}

pub struct NativeWindow(native::Window);

impl NativeWindow {
    pub fn close(&self) -> Result<(), WinUiError> {
        self.0.Close().map_err(Into::into)
    }
}

impl WinUiAdapter {
    pub fn open_window(&self, root: ObjectId) -> Result<NativeWindow, WinUiError> {
        let window = native::Window::new()?;
        window.SetContent(&self.ui_element(root)?)?;
        window.Activate()?;
        Ok(NativeWindow(window))
    }

    pub fn validate_graph(&self, graph: &crate::RetainedGraph) -> Result<(), WinUiError> {
        let Some(root) = graph.root() else {
            return Ok(());
        };
        let mut pending = vec![root];
        let mut visited = HashSet::new();
        while let Some(object) = pending.pop() {
            if !visited.insert(object) {
                continue;
            }
            let kind = graph
                .kind(object)
                .ok_or(WinUiError::MissingObject(object))?;
            if self.kind(object)? != kind {
                return Err(WinUiError::StateMismatch(object));
            }
            if let Handle::TextBlock(value) = self
                .handles
                .get(&object)
                .ok_or(WinUiError::MissingObject(object))?
            {
                let expected = graph
                    .properties(object)
                    .unwrap_or_default()
                    .iter()
                    .find_map(|property| match (&property.id, &property.value) {
                        (PropertyId::Text, PropertyValue::String(value)) => Some(value.as_ref()),
                        _ => None,
                    })
                    .unwrap_or_default();
                if value.Text()? != expected {
                    return Err(WinUiError::StateMismatch(object));
                }
            }
            for contract in relation_contracts(kind) {
                match contract.cardinality {
                    crate::Cardinality::One => {
                        if let Some(child) = graph.child(object, contract.id) {
                            if self.owners.get(&child) != Some(&(object, contract.id)) {
                                return Err(WinUiError::StateMismatch(object));
                            }
                            pending.push(child);
                        }
                    }
                    crate::Cardinality::Many => {
                        let children = graph.children(object, contract.id).unwrap_or_default();
                        for child in children {
                            if self.owners.get(child) != Some(&(object, contract.id)) {
                                return Err(WinUiError::StateMismatch(object));
                            }
                            pending.push(*child);
                        }
                        self.validate_native_order(object, contract, children)?;
                    }
                }
            }
        }
        if visited.len() != self.handles.len() {
            return Err(WinUiError::StateMismatch(root));
        }
        Ok(())
    }

    fn validate_native_order(
        &self,
        parent: ObjectId,
        contract: &RelationContract,
        children: &[ObjectId],
    ) -> Result<(), WinUiError> {
        let matches = match contract.realization {
            Realization::Owned => {
                let values = self.panel_children(parent)?;
                if values.Size()? as usize != children.len() {
                    false
                } else {
                    children.iter().enumerate().all(|(index, child)| {
                        values.GetAt(index as u32).ok().as_ref()
                            == self.ui_element(*child).ok().as_ref()
                    })
                }
            }
            Realization::Structural => {
                let values = self.tree_nodes(parent)?;
                if values.Size()? as usize != children.len() {
                    false
                } else {
                    children.iter().enumerate().all(|(index, child)| {
                        values.GetAt(index as u32).ok().as_ref()
                            == self.tree_node(*child).ok().as_ref()
                    })
                }
            }
            Realization::Container => {
                let values = self.items(parent)?;
                if values.Size()? as usize != children.len() {
                    false
                } else {
                    children.iter().enumerate().all(|(index, child)| {
                        values.GetAt(index as u32).ok().as_ref()
                            == self.inspectable(*child).ok().as_ref()
                    })
                }
            }
        };
        if matches {
            Ok(())
        } else {
            Err(WinUiError::StateMismatch(parent))
        }
    }

    fn create(&mut self, object: ObjectId, kind: ObjectType) -> Result<(), WinUiError> {
        if self.handles.contains_key(&object) {
            return Err(WinUiError::DuplicateObject(object));
        }
        let handle = match kind {
            ObjectType::TextBlock => Handle::TextBlock(native::TextBlock::new()?),
            ObjectType::Border => Handle::Border(native::Border::new()?),
            ObjectType::Grid => Handle::Grid(native::Grid::new()?),
            ObjectType::StackPanel => Handle::StackPanel(native::StackPanel::new()?),
            ObjectType::TreeView => {
                let tree = native::TreeView::new()?;
                let template = self.tree_template()?;
                tree.cast::<native::ITreeView2>()?
                    .SetItemTemplate(&template)?;
                Handle::TreeView(tree)
            }
            ObjectType::TreeNode => Handle::TreeNode(NativeTreeNode {
                value: native::TreeViewNode::new()?,
                text: HSTRING::new(),
                content: None,
            }),
            ObjectType::ListView => Handle::ListView(native::ListView::new()?),
            ObjectType::DataItem => {
                let value: IInspectable =
                    windows_reference::IReference::from(HSTRING::new()).into();
                Handle::Data(value)
            }
        };
        self.handles.insert(object, handle);
        Ok(())
    }

    fn set_properties(
        &mut self,
        object: ObjectId,
        set: &[Property],
        clear: &[PropertyId],
    ) -> Result<(), WinUiError> {
        for property in clear {
            if *property == PropertyId::Text
                && matches!(self.handles.get(&object), Some(Handle::Data(_)))
            {
                self.set_data_text(object, "")?;
                continue;
            }
            match (self.handle(object)?, property) {
                (Handle::TextBlock(value), PropertyId::Text) => value.SetText("")?,
                (Handle::TreeNode(value), PropertyId::Text) => {
                    value.text = HSTRING::new();
                    if value.content.is_none() {
                        let content: IInspectable =
                            windows_reference::IReference::from(value.text.clone()).into();
                        value.value.SetContent(&content)?;
                    }
                }
                (Handle::TreeNode(value), PropertyId::Expanded) => {
                    value.value.SetIsExpanded(false)?;
                }
                _ => return Err(WinUiError::InvalidObject(object)),
            }
        }
        for property in set {
            if property.id == PropertyId::Text
                && matches!(self.handles.get(&object), Some(Handle::Data(_)))
                && let PropertyValue::String(text) = &property.value
            {
                self.set_data_text(object, text)?;
                continue;
            }
            match (self.handle(object)?, property.id, &property.value) {
                (Handle::TextBlock(value), PropertyId::Text, PropertyValue::String(text)) => {
                    value.SetText(text)?;
                }
                (Handle::TreeNode(value), PropertyId::Text, PropertyValue::String(text)) => {
                    value.text = HSTRING::from(text.as_ref());
                    if value.content.is_none() {
                        let content: IInspectable =
                            windows_reference::IReference::from(value.text.clone()).into();
                        value.value.SetContent(&content)?;
                    }
                }
                (Handle::TreeNode(value), PropertyId::Expanded, PropertyValue::Bool(expanded)) => {
                    value.value.SetIsExpanded(*expanded)?;
                }
                _ => return Err(WinUiError::InvalidObject(object)),
            }
        }
        Ok(())
    }

    fn attach(
        &mut self,
        parent: ObjectId,
        relation: RelationId,
        child: ObjectId,
    ) -> Result<(), WinUiError> {
        if self.owners.contains_key(&child) {
            return Err(WinUiError::StillOwned(child));
        }
        let child_element = self.ui_element(child)?;
        match (self.handle(parent)?, relation) {
            (Handle::Border(parent), RelationId::Content) => parent.SetChild(&child_element)?,
            (Handle::TreeNode(parent), RelationId::Content) => {
                parent.value.SetContent(&child_element)?;
                parent.content = Some(child);
            }
            _ => return Err(WinUiError::InvalidRelation(parent, relation)),
        }
        self.owners.insert(child, (parent, relation));
        Ok(())
    }

    fn detach(
        &mut self,
        parent: ObjectId,
        relation: RelationId,
        child: ObjectId,
    ) -> Result<(), WinUiError> {
        if self.owners.get(&child) != Some(&(parent, relation)) {
            return Err(WinUiError::ChildNotFound(child));
        }
        match (self.handle(parent)?, relation) {
            (Handle::Border(parent), RelationId::Content) => {
                parent.SetChild(None::<&native::UIElement>)?;
            }
            (Handle::TreeNode(parent), RelationId::Content) => {
                let content: IInspectable =
                    windows_reference::IReference::from(parent.text.clone()).into();
                parent.value.SetContent(&content)?;
                parent.content = None;
            }
            _ => return Err(WinUiError::InvalidRelation(parent, relation)),
        }
        self.owners.remove(&child);
        Ok(())
    }

    fn insert(
        &mut self,
        parent: ObjectId,
        relation: RelationId,
        child: ObjectId,
        index: usize,
    ) -> Result<(), WinUiError> {
        if self.owners.contains_key(&child) {
            return Err(WinUiError::StillOwned(child));
        }
        match relation_contract(self.kind(parent)?, relation)?.realization {
            Realization::Owned => {
                self.panel_children(parent)?
                    .InsertAt(index32(index)?, &self.ui_element(child)?)?;
            }
            Realization::Structural => {
                self.tree_nodes(parent)?
                    .InsertAt(index32(index)?, &self.tree_node(child)?)?;
            }
            Realization::Container => {
                self.items(parent)?
                    .InsertAt(index32(index)?, &self.inspectable(child)?)?;
            }
        }
        self.owners.insert(child, (parent, relation));
        Ok(())
    }

    fn remove(
        &mut self,
        parent: ObjectId,
        relation: RelationId,
        child: ObjectId,
        index: usize,
    ) -> Result<(), WinUiError> {
        if self.owners.get(&child) != Some(&(parent, relation)) {
            return Err(WinUiError::ChildNotFound(child));
        }
        match relation_contract(self.kind(parent)?, relation)?.realization {
            Realization::Owned => {
                let values = self.panel_children(parent)?;
                if values.GetAt(index32(index)?)? != self.ui_element(child)? {
                    return Err(WinUiError::ChildNotFound(child));
                }
                values.RemoveAt(index32(index)?)?;
            }
            Realization::Structural => {
                let values = self.tree_nodes(parent)?;
                if values.GetAt(index32(index)?)? != self.tree_node(child)? {
                    return Err(WinUiError::ChildNotFound(child));
                }
                values.RemoveAt(index32(index)?)?;
            }
            Realization::Container => {
                let values = self.items(parent)?;
                if values.GetAt(index32(index)?)? != self.inspectable(child)? {
                    return Err(WinUiError::ChildNotFound(child));
                }
                values.RemoveAt(index32(index)?)?;
            }
        }
        self.owners.remove(&child);
        Ok(())
    }

    fn reorder(
        &mut self,
        parent: ObjectId,
        relation: RelationId,
        moves: &[crate::Move],
        children: &[ObjectId],
    ) -> Result<(), WinUiError> {
        match relation_contract(self.kind(parent)?, relation)?.realization {
            Realization::Owned => {
                let values = self.panel_children(parent)?;
                let movable = values.cast::<native::IUIElementCollection>()?;
                for movement in moves {
                    let child = self.ui_element(movement.child)?;
                    let mut from = 0;
                    if !values.IndexOf(&child, &mut from)? {
                        return Err(WinUiError::ChildNotFound(movement.child));
                    }
                    let target = if let Some(before) = movement.before {
                        let before = self.ui_element(before)?;
                        let mut target = 0;
                        if !values.IndexOf(&before, &mut target)? {
                            return Err(WinUiError::ChildNotFound(movement.child));
                        }
                        if from < target { target - 1 } else { target }
                    } else {
                        values.Size()?.checked_sub(1).unwrap()
                    };
                    movable.Move(from, target)?;
                }
            }
            Realization::Structural => {
                let tree = self.tree_owner(parent)?;
                let template = self.tree_template()?;
                let tree2 = tree.cast::<native::ITreeView2>()?;
                tree2.SetItemTemplate(None::<&native::DataTemplate>)?;
                tree.cast::<native::IUIElement>()?.UpdateLayout()?;
                let values = self.tree_nodes(parent)?;
                for movement in moves {
                    let child = self.tree_node(movement.child)?;
                    let mut previous = 0;
                    if !values.IndexOf(&child, &mut previous)? {
                        return Err(WinUiError::ChildNotFound(movement.child));
                    }
                    values.RemoveAt(previous)?;
                    let index = if let Some(before) = movement.before {
                        let before = self.tree_node(before)?;
                        let mut index = 0;
                        if !values.IndexOf(&before, &mut index)? {
                            return Err(WinUiError::ChildNotFound(movement.child));
                        }
                        index
                    } else {
                        values.Size()?
                    };
                    values.InsertAt(index, &child)?;
                }
                tree2.SetItemTemplate(&template)?;
            }
            Realization::Container => {
                let values = self.items(parent)?;
                for movement in moves {
                    let child = self.inspectable(movement.child)?;
                    let mut previous = 0;
                    if !values.IndexOf(&child, &mut previous)? {
                        return Err(WinUiError::ChildNotFound(movement.child));
                    }
                    values.RemoveAt(previous)?;
                    let index = if let Some(before) = movement.before {
                        let before = self.inspectable(before)?;
                        let mut index = 0;
                        if !values.IndexOf(&before, &mut index)? {
                            return Err(WinUiError::ChildNotFound(movement.child));
                        }
                        index
                    } else {
                        values.Size()?
                    };
                    values.InsertAt(index, &child)?;
                }
            }
        }
        debug_assert_eq!(
            children.len(),
            self.owners
                .values()
                .filter(|owner| **owner == (parent, relation))
                .count()
        );
        Ok(())
    }

    fn handle(&mut self, object: ObjectId) -> Result<&mut Handle, WinUiError> {
        self.handles
            .get_mut(&object)
            .ok_or(WinUiError::MissingObject(object))
    }

    fn kind(&self, object: ObjectId) -> Result<ObjectType, WinUiError> {
        Ok(
            match self
                .handles
                .get(&object)
                .ok_or(WinUiError::MissingObject(object))?
            {
                Handle::TextBlock(_) => ObjectType::TextBlock,
                Handle::Border(_) => ObjectType::Border,
                Handle::Grid(_) => ObjectType::Grid,
                Handle::StackPanel(_) => ObjectType::StackPanel,
                Handle::TreeView(_) => ObjectType::TreeView,
                Handle::TreeNode(_) => ObjectType::TreeNode,
                Handle::ListView(_) => ObjectType::ListView,
                Handle::Data(_) => ObjectType::DataItem,
            },
        )
    }

    fn ui_element(&self, object: ObjectId) -> Result<native::UIElement, WinUiError> {
        match self
            .handles
            .get(&object)
            .ok_or(WinUiError::MissingObject(object))?
        {
            Handle::TextBlock(value) => Ok(value.cast()?),
            Handle::Border(value) => Ok(value.cast()?),
            Handle::Grid(value) => Ok(value.cast()?),
            Handle::StackPanel(value) => Ok(value.cast()?),
            Handle::TreeView(value) => Ok(value.cast()?),
            Handle::ListView(value) => Ok(value.cast()?),
            Handle::TreeNode(_) | Handle::Data(_) => Err(WinUiError::InvalidObject(object)),
        }
    }

    fn inspectable(&self, object: ObjectId) -> Result<IInspectable, WinUiError> {
        match self
            .handles
            .get(&object)
            .ok_or(WinUiError::MissingObject(object))?
        {
            Handle::Data(value) => Ok(value.clone()),
            _ => Err(WinUiError::InvalidObject(object)),
        }
    }

    fn tree_node(&self, object: ObjectId) -> Result<native::TreeViewNode, WinUiError> {
        match self
            .handles
            .get(&object)
            .ok_or(WinUiError::MissingObject(object))?
        {
            Handle::TreeNode(value) => Ok(value.value.clone()),
            _ => Err(WinUiError::InvalidObject(object)),
        }
    }

    fn panel_children(&self, object: ObjectId) -> Result<native::UIElementCollection, WinUiError> {
        match self
            .handles
            .get(&object)
            .ok_or(WinUiError::MissingObject(object))?
        {
            Handle::Grid(value) => Ok(value.cast::<native::IPanel>()?.Children()?),
            Handle::StackPanel(value) => Ok(value.cast::<native::IPanel>()?.Children()?),
            _ => Err(WinUiError::InvalidObject(object)),
        }
    }

    fn tree_nodes(
        &self,
        object: ObjectId,
    ) -> Result<windows_collections::IVector<native::TreeViewNode>, WinUiError> {
        match self
            .handles
            .get(&object)
            .ok_or(WinUiError::MissingObject(object))?
        {
            Handle::TreeView(value) => Ok(value.RootNodes()?),
            Handle::TreeNode(value) => Ok(value.value.Children()?),
            _ => Err(WinUiError::InvalidObject(object)),
        }
    }

    fn tree_owner(&self, object: ObjectId) -> Result<native::TreeView, WinUiError> {
        let mut current = object;
        loop {
            match self
                .handles
                .get(&current)
                .ok_or(WinUiError::MissingObject(current))?
            {
                Handle::TreeView(tree) => return Ok(tree.clone()),
                Handle::TreeNode(_) => {
                    current = self
                        .owners
                        .get(&current)
                        .map(|(parent, _)| *parent)
                        .ok_or(WinUiError::StillOwned(current))?;
                }
                _ => return Err(WinUiError::InvalidObject(current)),
            }
        }
    }

    fn items(&self, object: ObjectId) -> Result<native::ItemCollection, WinUiError> {
        match self
            .handles
            .get(&object)
            .ok_or(WinUiError::MissingObject(object))?
        {
            Handle::ListView(value) => Ok(value.cast::<native::IItemsControl>()?.Items()?),
            _ => Err(WinUiError::InvalidObject(object)),
        }
    }

    fn set_data_text(&mut self, object: ObjectId, text: &str) -> Result<(), WinUiError> {
        let previous = self.inspectable(object)?;
        let next: IInspectable = windows_reference::IReference::from(HSTRING::from(text)).into();
        if let Some((parent, relation)) = self.owners.get(&object).copied() {
            if relation != RelationId::Items {
                return Err(WinUiError::InvalidRelation(parent, relation));
            }
            let items = self.items(parent)?;
            let mut index = 0;
            if !items.IndexOf(&previous, &mut index)? {
                return Err(WinUiError::ChildNotFound(object));
            }
            items.SetAt(index, &next)?;
        }
        let Handle::Data(current) = self.handle(object)? else {
            return Err(WinUiError::InvalidObject(object));
        };
        *current = next;
        Ok(())
    }

    fn tree_template(&mut self) -> Result<native::DataTemplate, WinUiError> {
        if let Some(template) = &self.tree_template {
            return Ok(template.clone());
        }
        let template = native::XamlReader::Load(
            "<DataTemplate xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation'>\
             <ContentPresenter Content='{Binding Content}'/>\
             </DataTemplate>",
        )?
        .cast::<native::DataTemplate>()?;
        self.tree_template = Some(template.clone());
        Ok(template)
    }
}

impl Adapter for WinUiAdapter {
    type Error = WinUiError;

    fn validate(&self, _mutations: &[Mutation]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn apply(&mut self, mutations: &[Mutation]) -> Result<(), Self::Error> {
        for mutation in mutations {
            match mutation {
                Mutation::Create { object, kind } => self.create(*object, *kind)?,
                Mutation::SetProperties { object, set, clear } => {
                    self.set_properties(*object, set, clear)?;
                }
                Mutation::Attach {
                    parent,
                    relation,
                    child,
                } => self.attach(*parent, *relation, *child)?,
                Mutation::Detach {
                    parent,
                    relation,
                    child,
                } => self.detach(*parent, *relation, *child)?,
                Mutation::Insert {
                    parent,
                    relation,
                    child,
                    index,
                } => self.insert(*parent, *relation, *child, *index)?,
                Mutation::Remove {
                    parent,
                    relation,
                    child,
                    index,
                } => self.remove(*parent, *relation, *child, *index)?,
                Mutation::Reorder {
                    parent,
                    relation,
                    moves,
                    children,
                } => self.reorder(*parent, *relation, moves, children)?,
                Mutation::Destroy { object } => {
                    if self.owners.contains_key(object) {
                        return Err(WinUiError::StillOwned(*object));
                    }
                    self.handles
                        .remove(object)
                        .ok_or(WinUiError::MissingObject(*object))?;
                }
            }
        }
        Ok(())
    }
}

fn relation_contract(
    kind: ObjectType,
    relation: RelationId,
) -> Result<&'static RelationContract, WinUiError> {
    relation_contracts(kind)
        .iter()
        .find(|contract| contract.id == relation)
        .ok_or(WinUiError::MissingContract(kind, relation))
}

fn index32(index: usize) -> Result<u32, WinUiError> {
    index
        .try_into()
        .map_err(|_| WinUiError::IndexOverflow(index))
}
