//! Behavioral contract tests for [`super::Pump`], split by the surface each
//! group of tests exercises. Shared fixtures and helper functions live in
//! [`support`].

mod support;

mod background_tasks;
mod breadcrumb_bar_properties;
mod components_turn_ordering;
mod content_dialogs;
mod context_propagation;
mod events_controlled_feedback;
mod exit_retirement;
mod failure_policy;
mod flyouts;
mod grid_properties;
mod hyperlink_button_properties;
mod image_properties;
mod imperative_references;
mod keyed_fragments;
mod lifecycle_effects;
mod menus;
mod model_reconciliation;
mod mount_update_publication;
mod pointer_events;
mod properties_native_failure;
mod resource_overrides;
mod rich_text;
mod scrolling_properties;
mod slots;
mod tooltips;
mod tree_views;
mod virtualization_realization;
mod visual_properties;
mod window_observations;
mod window_requests;
mod window_title_bars;
mod window_titles;
mod window_visuals;
