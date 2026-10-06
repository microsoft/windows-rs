#![doc = include_str!("../readme.md")]

#[cfg(doctest)]
#[doc = include_str!("../../../../docs/crates/windows-reactor.md")]
mod guide {}

#[cfg(any(test, feature = "test"))]
#[path = "test_support/adapter.rs"]
mod adapter;
mod component;
mod declaration;
mod generated;
mod ir;
#[cfg(not(feature = "test"))]
mod native;
#[cfg(feature = "test")]
pub mod native;
mod reconcile;
mod reference;
mod sealed {
    pub trait PayloadCallback<T> {}
    pub trait Sealed {}
    pub trait UnitCallback {}
}
mod window;

use std::any::{Any, TypeId};
use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::fmt;
use std::marker::PhantomData;
use std::mem::size_of;
use std::path::Path;
use std::rc::{Rc, Weak};
#[cfg(test)]
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, Weak as SyncWeak};
use std::time::Duration;

pub(crate) use sealed::Sealed;

#[cfg(any(test, feature = "test"))]
pub use adapter::*;
#[cfg(not(any(test, feature = "test")))]
use component::*;
#[cfg(any(test, feature = "test"))]
pub use component::*;
pub use component::{
    CancellationToken, Component, ComponentCompletion, ComponentContext, ComponentNode,
    ComponentSender, ComponentTask, ComponentTimer, Context, EffectKey, LocalSender, ViewContext,
    WindowHandle, component, provide,
};
#[cfg(not(any(test, feature = "test")))]
pub use declaration::generated_declarations::*;
#[cfg(not(any(test, feature = "test")))]
use declaration::*;
#[cfg(any(test, feature = "test"))]
pub use declaration::*;
pub use declaration::{
    AcceleratorKey, AcceleratorModifiers, Brush, ButtonStyle, Callback, CharacterEventInfo, Color,
    CommandBarCommand, CommandBarFlyout, CommandBarFlyoutExt, ContentDialogExt,
    ContentDialogResult, CornerRadius, DragDropAction, DragDropOperation, DragDropPolicy, DragKind,
    DroppedData, DroppedStorageItem, ElementFocusState, EncodedImage, ExitTransition, Flyout,
    FlyoutExt, FlyoutPlacement, FocusEventInfo, FontWeight, GridLength, Icon, ImageSource,
    InputModifiers, IntoPayloadCallback, IntoUnitCallback, IntoViews, Key, KeyAccelerator,
    KeyAccelerators, KeyEventInfo, KeyedView, Menu, MenuExt, MenuItem, NavigationViewDisplayMode,
    PhysicalKeyStatus, PointerEventInfo, ResourceOverrides, ResourceValue, RichText,
    RichTextHyperlink, RichTextInline, RichTextParagraph, RichTextRun, RoutedCallback, ThemeBrush,
    ThemeTransition, Thickness, Tooltip, TooltipExt, TooltipPlacement, View, VirtualKey,
    VirtualSource, keyed,
};
#[cfg(not(any(test, feature = "test")))]
use generated::*;
#[cfg(any(test, feature = "test"))]
pub use generated::*;
#[cfg(not(any(test, feature = "test")))]
use ir::*;
#[cfg(any(test, feature = "test"))]
pub use ir::*;
pub use native::{App, AppCallback, AppContext, AppProxy, ScreenPoint, WindowTitleBarHeight};
#[cfg(feature = "test")]
pub use native::{
    LiveRenderingSubscription, LiveTickSubscription, bring_live_virtual_index,
    live_virtual_shell_counts, schedule_live_test_exit, subscribe_live_interval,
    subscribe_live_rendering, subscribe_live_tick,
};
#[cfg(not(any(test, feature = "test")))]
use reconcile::*;
#[cfg(any(test, feature = "test"))]
pub use reconcile::*;
#[cfg(not(any(test, feature = "test")))]
use reference::*;
#[cfg(any(test, feature = "test"))]
pub use reference::*;
pub use reference::{
    AnyElement, CompatibleElementRef, CompositionHostEvent, ElementObservation, ElementRef,
    FocusError, IntegrationError, ReferenceElement, SwapChainPanelBinding, SwapChainPanelEvent,
};
pub use window::*;
pub use windows_time::{DateTime, TimeSpan};

#[cfg(test)]
mod tests;
