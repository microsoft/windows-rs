#![doc = include_str!("../readme.md")]

mod application;
mod component;
mod declaration;
mod generated;
mod ir;
mod native;
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

use application::ApplicationCommand;
pub use application::{
    Application, ApplicationContext, ApplicationView, ApplicationViewContext, NotifyIcon,
};
pub(crate) use sealed::Sealed;

use component::*;
pub use component::{
    CancellationToken, Component, ComponentCompletion, ComponentContext, ComponentNode,
    ComponentSender, ComponentTask, ComponentTimer, Context, EffectKey, LocalSender, ViewContext,
    WindowHandle, component, provide,
};
pub use declaration::generated_declarations::*;
use declaration::*;
pub use declaration::{
    AcceleratorKey, AcceleratorModifiers, Brush, ButtonStyle, Callback, CharacterEventInfo, Color,
    CommandBarCommand, CommandBarFlyout, CommandBarFlyoutExt, ContentDialogExt,
    ContentDialogResult, CornerRadius, DragDropAction, DragDropOperation, DragDropPolicy, DragKind,
    DroppedData, DroppedStorageItem, ElementFocusState, EncodedImage, Flyout, FlyoutExt,
    FlyoutPlacement, FocusEventInfo, FontWeight, GridLength, Icon, ImageSource, InputModifiers,
    IntoPayloadCallback, IntoUnitCallback, IntoViews, Key, KeyAccelerator, KeyAccelerators,
    KeyEventInfo, KeyedView, Menu, MenuExt, MenuItem, NavigationViewDisplayMode, PhysicalKeyStatus,
    PointerCallback, PointerEventInfo, ResourceOverrides, ResourceValue, RichText,
    RichTextHyperlink, RichTextInline, RichTextParagraph, RichTextRun, RoutedCallback, ThemeBrush,
    ThemeTransition, Thickness, Tooltip, TooltipExt, TooltipPlacement, View, VirtualKey,
    VirtualSource, keyed,
};
use generated::*;
use ir::*;
pub use native::{App, ScreenPoint, WindowTitleBarHeight};
use native::{
    ComponentServices, SharedWinUiAdapter, TimerRegistration, UiCallback, WindowServices,
};
use reconcile::*;
use reference::*;
pub use reference::{
    AnyElement, AutomationNotification, AutomationNotificationKind,
    AutomationNotificationProcessing, CompatibleElementRef, CompositionHostEvent,
    ElementObservation, ElementRef, FocusError, IntegrationError, ReferenceElement,
    ScrollViewMetrics, SwapChainPanelBinding, SwapChainPanelEvent,
};
pub use window::*;
pub use windows_time::{DateTime, TimeSpan};

#[cfg(test)]
mod tests;
