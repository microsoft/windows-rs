use std::any::{Any, TypeId};
use std::rc::Rc;

use super::*;

pub trait ComponentObject: 'static {
    fn render(&self, cx: &mut RenderCx) -> Element;

    fn props_as_any(&self) -> &dyn Any;

    fn component_type_id(&self) -> TypeId;

    fn is_equivalent(&self, other: &dyn ComponentObject) -> bool;

    fn should_update(&self, other: &dyn ComponentObject) -> bool;

    fn invoke_appeared(&self, cx: &mut RenderCx);

    fn invoke_disappeared(&self, cx: &mut RenderCx);

    fn has_on_appeared(&self) -> bool;

    fn has_on_disappeared(&self) -> bool;
}

struct ComponentCell<C, P>
where
    C: Component<P>,
    P: Clone + PartialEq + 'static,
{
    component: C,
    props: P,
}

impl<C, P> ComponentObject for ComponentCell<C, P>
where
    C: Component<P>,
    P: Clone + PartialEq + 'static,
{
    fn render(&self, cx: &mut RenderCx) -> Element {
        self.component.render(&self.props, cx)
    }

    fn props_as_any(&self) -> &dyn Any {
        &self.props
    }

    fn component_type_id(&self) -> TypeId {
        TypeId::of::<C>()
    }

    fn is_equivalent(&self, other: &dyn ComponentObject) -> bool {
        if self.component_type_id() != other.component_type_id() {
            return false;
        }
        let Some(other_props) = other.props_as_any().downcast_ref::<P>() else {
            return false;
        };
        &self.props == other_props
    }

    fn should_update(&self, other: &dyn ComponentObject) -> bool {
        if self.component_type_id() != other.component_type_id() {
            return true;
        }
        let Some(other_props) = other.props_as_any().downcast_ref::<P>() else {
            return true;
        };

        self.component.should_update(&self.props, other_props)
    }

    fn invoke_appeared(&self, cx: &mut RenderCx) {
        self.component.on_appeared(&self.props, cx);
    }

    fn invoke_disappeared(&self, cx: &mut RenderCx) {
        self.component.on_disappeared(&self.props, cx);
    }

    fn has_on_appeared(&self) -> bool {
        self.component.has_on_appeared()
    }

    fn has_on_disappeared(&self) -> bool {
        self.component.has_on_disappeared()
    }
}

/// Erased [`Component`]+props pair carried inside [`Element::Component`].
pub struct ComponentElement {
    pub key: Option<String>,
    pub memoised: bool,
    pub obj: Rc<dyn ComponentObject>,
}

impl Clone for ComponentElement {
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            memoised: self.memoised,
            obj: Rc::clone(&self.obj),
        }
    }
}

impl std::fmt::Debug for ComponentElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComponentElement")
            .field("key", &self.key)
            .field("memoised", &self.memoised)
            .field("component_type_id", &self.obj.component_type_id())
            .finish()
    }
}

impl PartialEq for ComponentElement {
    fn eq(&self, other: &Self) -> bool {
        if self.key != other.key || self.memoised != other.memoised {
            return false;
        }
        if self.memoised {
            self.obj.is_equivalent(&*other.obj)
        } else {
            !self.obj.should_update(&*other.obj)
        }
    }
}

/// Wrap a [`Component`] + props into an [`Element`] for embedding in a tree.
pub fn component<C, P>(component: C, props: P) -> Element
where
    C: Component<P>,
    P: Clone + PartialEq + 'static,
{
    let cell = ComponentCell { component, props };
    Element::Component(ComponentElement {
        key: None,
        memoised: false,
        obj: Rc::new(cell),
    })
}

/// Like [`component()`](crate::core::component_element::component()), but treat the component as memoised: the host
/// reuses the previous render output when props compare equal.
pub fn memo<C, P>(component: C, props: P) -> Element
where
    C: Component<P>,
    P: Clone + PartialEq + 'static,
{
    let cell = ComponentCell { component, props };
    Element::Component(ComponentElement {
        key: None,
        memoised: true,
        obj: Rc::new(cell),
    })
}
