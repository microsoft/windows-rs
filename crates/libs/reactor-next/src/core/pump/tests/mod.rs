//! Behavioral contract tests for [`super::Pump`], split by the surface each
//! group of tests exercises. Shared fixtures and helper functions live in
//! [`support`].

mod support;

mod background_tasks;
mod components_turn_ordering;
mod context_propagation;
mod events_controlled_feedback;
mod imperative_references;
mod keyed_fragments;
mod lifecycle_effects;
mod mount_update_publication;
mod properties_native_failure;
mod slots;
mod virtualization_realization;
