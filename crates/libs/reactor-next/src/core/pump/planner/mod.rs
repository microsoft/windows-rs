//! Pure planning logic for reconciling and mounting `Element`/`View` trees.
//!
//! These modules never touch [`NativeRuntime`], apply commands, build receipts,
//! schedule effects, or mutate a published [`Pump`]; they only compute what
//! should change. `topology` holds tree-shape helpers shared by both planning
//! kinds; `element` plans plain element trees; `view` plans view/component
//! trees and depends on both `element` and `topology`.

mod element;
mod topology;
mod view;
