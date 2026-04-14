
#pragma warning( disable: 4049 )  /* more than 64k source lines */

/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 500
#endif

/* verify that the <rpcsal.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCSAL_H_VERSION__
#define __REQUIRED_RPCSAL_H_VERSION__ 100
#endif

#include <rpc.h>
#include <rpcndr.h>

#ifndef __RPCNDR_H_VERSION__
#error this stub requires an updated version of <rpcndr.h>
#endif /* __RPCNDR_H_VERSION__ */

#ifndef COM_NO_WINDOWS_H
#include <windows.h>
#include <ole2.h>
#endif /*COM_NO_WINDOWS_H*/
#ifndef __windows2Eui2Examl2Ecore2Edirect_h__
#define __windows2Eui2Examl2Ecore2Edirect_h__
#ifndef __windows2Eui2Examl2Ecore2Edirect_p_h__
#define __windows2Eui2Examl2Ecore2Edirect_p_h__


#pragma once

//
// Deprecated attribute support
//

#pragma push_macro("DEPRECATED")
#undef DEPRECATED

#if !defined(DISABLE_WINRT_DEPRECATION)
#if defined(__cplusplus)
#if __cplusplus >= 201402
#define DEPRECATED(x) [[deprecated(x)]]
#define DEPRECATEDENUMERATOR(x) [[deprecated(x)]]
#elif defined(_MSC_VER)
#if _MSC_VER >= 1900
#define DEPRECATED(x) [[deprecated(x)]]
#define DEPRECATEDENUMERATOR(x) [[deprecated(x)]]
#else
#define DEPRECATED(x) __declspec(deprecated(x))
#define DEPRECATEDENUMERATOR(x)
#endif // _MSC_VER >= 1900
#else // Not Standard C++ or MSVC, ignore the construct.
#define DEPRECATED(x)
#define DEPRECATEDENUMERATOR(x)
#endif  // C++ deprecation
#else // C - disable deprecation
#define DEPRECATED(x)
#define DEPRECATEDENUMERATOR(x)
#endif
#else // Deprecation is disabled
#define DEPRECATED(x)
#define DEPRECATEDENUMERATOR(x)
#endif  /* DEPRECATED */

// Disable Deprecation for this header, MIDL verifies that cross-type access is acceptable
#ifdef __clang__
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#else
#pragma warning(push)
#pragma warning(disable: 4996)
#endif

// Ensure that the setting of the /ns_prefix command line switch is consistent for all headers.
// If you get an error from the compiler indicating "warning C4005: 'CHECK_NS_PREFIX_STATE': macro redefinition", this
// indicates that you have included two different headers with different settings for the /ns_prefix MIDL command line switch
#if !defined(DISABLE_NS_PREFIX_CHECKS)
#define CHECK_NS_PREFIX_STATE "always"
#endif // !defined(DISABLE_NS_PREFIX_CHECKS)


#pragma push_macro("MIDL_CONST_ID")
#undef MIDL_CONST_ID
#define MIDL_CONST_ID const __declspec(selectany)


//  API Contract Inclusion Definitions
#if !defined(SPECIFIC_API_CONTRACT_DEFINITIONS)
#if !defined(WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION)
#define WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION 0x40000
#endif // defined(WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION)

#if !defined(WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION)
#define WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION 0x130000
#endif // defined(WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION)

#if !defined(WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION)
#define WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION 0x50000
#endif // defined(WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.UI.h"
#include "Windows.UI.Xaml.h"
#include "Windows.UI.Xaml.Media.h"
#include "Windows.UI.Xaml.Media.Media3D.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Core {
                    namespace Direct {
                        interface IXamlDirect;
                    } /* Direct */
                } /* Core */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect ABI::Windows::UI::Xaml::Core::Direct::IXamlDirect

#endif // ____x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Core {
                    namespace Direct {
                        interface IXamlDirectObject;
                    } /* Direct */
                } /* Core */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject

#endif // ____x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Core {
                    namespace Direct {
                        interface IXamlDirectStatics;
                    } /* Direct */
                } /* Core */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectStatics ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct DateTime DateTime;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct Point Point;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct Rect Rect;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct Size Size;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct TimeSpan TimeSpan;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            typedef struct Color Color;
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                typedef struct CornerRadius CornerRadius;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                typedef struct Duration Duration;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                typedef struct GridLength GridLength;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    typedef struct Matrix Matrix;
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Media3D {
                        typedef struct Matrix3D Matrix3D;
                    } /* Media3D */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                typedef struct Thickness Thickness;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Core {
                    namespace Direct {
                        typedef enum XamlEventIndex : int XamlEventIndex;
                    } /* Direct */
                } /* Core */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Core {
                    namespace Direct {
                        typedef enum XamlPropertyIndex : int XamlPropertyIndex;
                    } /* Direct */
                } /* Core */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Core {
                    namespace Direct {
                        typedef enum XamlTypeIndex : int XamlTypeIndex;
                    } /* Direct */
                } /* Core */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Core {
                    namespace Direct {
                        class XamlDirect;
                    } /* Direct */
                } /* Core */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.UI.Xaml.Core.Direct.XamlEventIndex
 *
 * Introduced to Windows.UI.Xaml.Core.Direct.XamlDirectContract in version 1.0
 *
 */
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Core {
                    namespace Direct {
                        enum XamlEventIndex : int
                        {
                            XamlEventIndex_FrameworkElement_DataContextChanged = 16,
                            XamlEventIndex_FrameworkElement_SizeChanged = 17,
                            XamlEventIndex_FrameworkElement_LayoutUpdated = 18,
                            XamlEventIndex_UIElement_KeyUp = 22,
                            XamlEventIndex_UIElement_KeyDown = 23,
                            XamlEventIndex_UIElement_GotFocus = 24,
                            XamlEventIndex_UIElement_LostFocus = 25,
                            XamlEventIndex_UIElement_DragStarting = 26,
                            XamlEventIndex_UIElement_DropCompleted = 27,
                            XamlEventIndex_UIElement_CharacterReceived = 28,
                            XamlEventIndex_UIElement_DragEnter = 29,
                            XamlEventIndex_UIElement_DragLeave = 30,
                            XamlEventIndex_UIElement_DragOver = 31,
                            XamlEventIndex_UIElement_Drop = 32,
                            XamlEventIndex_UIElement_PointerPressed = 38,
                            XamlEventIndex_UIElement_PointerMoved = 39,
                            XamlEventIndex_UIElement_PointerReleased = 40,
                            XamlEventIndex_UIElement_PointerEntered = 41,
                            XamlEventIndex_UIElement_PointerExited = 42,
                            XamlEventIndex_UIElement_PointerCaptureLost = 43,
                            XamlEventIndex_UIElement_PointerCanceled = 44,
                            XamlEventIndex_UIElement_PointerWheelChanged = 45,
                            XamlEventIndex_UIElement_Tapped = 46,
                            XamlEventIndex_UIElement_DoubleTapped = 47,
                            XamlEventIndex_UIElement_Holding = 48,
                            XamlEventIndex_UIElement_ContextRequested = 49,
                            XamlEventIndex_UIElement_ContextCanceled = 50,
                            XamlEventIndex_UIElement_RightTapped = 51,
                            XamlEventIndex_UIElement_ManipulationStarting = 52,
                            XamlEventIndex_UIElement_ManipulationInertiaStarting = 53,
                            XamlEventIndex_UIElement_ManipulationStarted = 54,
                            XamlEventIndex_UIElement_ManipulationDelta = 55,
                            XamlEventIndex_UIElement_ManipulationCompleted = 56,
                            XamlEventIndex_UIElement_ProcessKeyboardAccelerators = 60,
                            XamlEventIndex_UIElement_GettingFocus = 61,
                            XamlEventIndex_UIElement_LosingFocus = 62,
                            XamlEventIndex_UIElement_NoFocusCandidateFound = 63,
                            XamlEventIndex_UIElement_PreviewKeyDown = 64,
                            XamlEventIndex_UIElement_PreviewKeyUp = 65,
                            XamlEventIndex_UIElement_BringIntoViewRequested = 66,
                            XamlEventIndex_AppBar_Opening = 109,
                            XamlEventIndex_AppBar_Opened = 110,
                            XamlEventIndex_AppBar_Closing = 111,
                            XamlEventIndex_AppBar_Closed = 112,
                            XamlEventIndex_AutoSuggestBox_SuggestionChosen = 113,
                            XamlEventIndex_AutoSuggestBox_TextChanged = 114,
                            XamlEventIndex_AutoSuggestBox_QuerySubmitted = 115,
                            XamlEventIndex_CalendarDatePicker_CalendarViewDayItemChanging = 116,
                            XamlEventIndex_CalendarDatePicker_DateChanged = 117,
                            XamlEventIndex_CalendarDatePicker_Opened = 118,
                            XamlEventIndex_CalendarDatePicker_Closed = 119,
                            XamlEventIndex_CalendarView_CalendarViewDayItemChanging = 120,
                            XamlEventIndex_CalendarView_SelectedDatesChanged = 121,
                            XamlEventIndex_ComboBox_DropDownClosed = 122,
                            XamlEventIndex_ComboBox_DropDownOpened = 123,
                            XamlEventIndex_CommandBar_DynamicOverflowItemsChanging = 124,
                            XamlEventIndex_ContentDialog_Closing = 126,
                            XamlEventIndex_ContentDialog_Closed = 127,
                            XamlEventIndex_ContentDialog_Opened = 128,
                            XamlEventIndex_ContentDialog_PrimaryButtonClick = 129,
                            XamlEventIndex_ContentDialog_SecondaryButtonClick = 130,
                            XamlEventIndex_ContentDialog_CloseButtonClick = 131,
                            XamlEventIndex_Control_FocusEngaged = 132,
                            XamlEventIndex_Control_FocusDisengaged = 133,
                            XamlEventIndex_DatePicker_DateChanged = 135,
                            XamlEventIndex_Frame_Navigated = 136,
                            XamlEventIndex_Frame_Navigating = 137,
                            XamlEventIndex_Frame_NavigationFailed = 138,
                            XamlEventIndex_Frame_NavigationStopped = 139,
                            XamlEventIndex_Hub_SectionHeaderClick = 143,
                            XamlEventIndex_Hub_SectionsInViewChanged = 144,
                            XamlEventIndex_ItemsPresenter_HorizontalSnapPointsChanged = 148,
                            XamlEventIndex_ItemsPresenter_VerticalSnapPointsChanged = 149,
                            XamlEventIndex_ListViewBase_ItemClick = 150,
                            XamlEventIndex_ListViewBase_DragItemsStarting = 151,
                            XamlEventIndex_ListViewBase_DragItemsCompleted = 152,
                            XamlEventIndex_ListViewBase_ContainerContentChanging = 153,
                            XamlEventIndex_ListViewBase_ChoosingItemContainer = 154,
                            XamlEventIndex_ListViewBase_ChoosingGroupHeaderContainer = 155,
                            XamlEventIndex_MediaTransportControls_ThumbnailRequested = 167,
                            XamlEventIndex_MenuFlyoutItem_Click = 168,
                            XamlEventIndex_RichEditBox_TextChanging = 177,
                            XamlEventIndex_ScrollViewer_ViewChanging = 192,
                            XamlEventIndex_ScrollViewer_ViewChanged = 193,
                            XamlEventIndex_ScrollViewer_DirectManipulationStarted = 194,
                            XamlEventIndex_ScrollViewer_DirectManipulationCompleted = 195,
                            XamlEventIndex_SearchBox_QueryChanged = 196,
                            XamlEventIndex_SearchBox_SuggestionsRequested = 197,
                            XamlEventIndex_SearchBox_QuerySubmitted = 198,
                            XamlEventIndex_SearchBox_ResultSuggestionChosen = 199,
                            XamlEventIndex_SearchBox_PrepareForFocusOnKeyboardInput = 200,
                            XamlEventIndex_SemanticZoom_ViewChangeStarted = 201,
                            XamlEventIndex_SemanticZoom_ViewChangeCompleted = 202,
                            XamlEventIndex_SettingsFlyout_BackClick = 203,
                            XamlEventIndex_StackPanel_HorizontalSnapPointsChanged = 208,
                            XamlEventIndex_StackPanel_VerticalSnapPointsChanged = 209,
                            XamlEventIndex_TimePicker_TimeChanged = 227,
                            XamlEventIndex_ToggleSwitch_Toggled = 228,
                            XamlEventIndex_ToolTip_Closed = 229,
                            XamlEventIndex_ToolTip_Opened = 230,
                            XamlEventIndex_VirtualizingStackPanel_CleanUpVirtualizedItemEvent = 231,
                            XamlEventIndex_WebView_SeparateProcessLost = 232,
                            XamlEventIndex_WebView_LoadCompleted = 233,
                            XamlEventIndex_WebView_ScriptNotify = 234,
                            XamlEventIndex_WebView_NavigationFailed = 235,
                            XamlEventIndex_WebView_NavigationStarting = 236,
                            XamlEventIndex_WebView_ContentLoading = 237,
                            XamlEventIndex_WebView_DOMContentLoaded = 238,
                            XamlEventIndex_WebView_NavigationCompleted = 239,
                            XamlEventIndex_WebView_FrameNavigationStarting = 240,
                            XamlEventIndex_WebView_FrameContentLoading = 241,
                            XamlEventIndex_WebView_FrameDOMContentLoaded = 242,
                            XamlEventIndex_WebView_FrameNavigationCompleted = 243,
                            XamlEventIndex_WebView_LongRunningScriptDetected = 244,
                            XamlEventIndex_WebView_UnsafeContentWarningDisplaying = 245,
                            XamlEventIndex_WebView_UnviewableContentIdentified = 246,
                            XamlEventIndex_WebView_ContainsFullScreenElementChanged = 247,
                            XamlEventIndex_WebView_UnsupportedUriSchemeIdentified = 248,
                            XamlEventIndex_WebView_NewWindowRequested = 249,
                            XamlEventIndex_WebView_PermissionRequested = 250,
                            XamlEventIndex_ButtonBase_Click = 256,
                            XamlEventIndex_CarouselPanel_HorizontalSnapPointsChanged = 257,
                            XamlEventIndex_CarouselPanel_VerticalSnapPointsChanged = 258,
                            XamlEventIndex_OrientedVirtualizingPanel_HorizontalSnapPointsChanged = 263,
                            XamlEventIndex_OrientedVirtualizingPanel_VerticalSnapPointsChanged = 264,
                            XamlEventIndex_RangeBase_ValueChanged = 267,
                            XamlEventIndex_ScrollBar_Scroll = 268,
                            XamlEventIndex_Selector_SelectionChanged = 269,
                            XamlEventIndex_Thumb_DragStarted = 270,
                            XamlEventIndex_Thumb_DragDelta = 271,
                            XamlEventIndex_Thumb_DragCompleted = 272,
                            XamlEventIndex_ToggleButton_Checked = 273,
                            XamlEventIndex_ToggleButton_Unchecked = 274,
                            XamlEventIndex_ToggleButton_Indeterminate = 275,
                            XamlEventIndex_WebView_WebResourceRequested = 283,
                            XamlEventIndex_ScrollViewer_AnchorRequested = 291,
                            XamlEventIndex_DatePicker_SelectedDateChanged = 322,
                            XamlEventIndex_TimePicker_SelectedTimeChanged = 323,
                        };
                    } /* Direct */
                } /* Core */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Core.Direct.XamlPropertyIndex
 *
 * Introduced to Windows.UI.Xaml.Core.Direct.XamlDirectContract in version 1.0
 *
 */
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Core {
                    namespace Direct {
                        enum XamlPropertyIndex : int
                        {
                            XamlPropertyIndex_AutomationProperties_AcceleratorKey = 5,
                            XamlPropertyIndex_AutomationProperties_AccessibilityView = 6,
                            XamlPropertyIndex_AutomationProperties_AccessKey = 7,
                            XamlPropertyIndex_AutomationProperties_AutomationId = 8,
                            XamlPropertyIndex_AutomationProperties_ControlledPeers = 9,
                            XamlPropertyIndex_AutomationProperties_HelpText = 10,
                            XamlPropertyIndex_AutomationProperties_IsRequiredForForm = 11,
                            XamlPropertyIndex_AutomationProperties_ItemStatus = 12,
                            XamlPropertyIndex_AutomationProperties_ItemType = 13,
                            XamlPropertyIndex_AutomationProperties_LabeledBy = 14,
                            XamlPropertyIndex_AutomationProperties_LiveSetting = 15,
                            XamlPropertyIndex_AutomationProperties_Name = 16,
                            XamlPropertyIndex_ToolTipService_Placement = 24,
                            XamlPropertyIndex_ToolTipService_PlacementTarget = 25,
                            XamlPropertyIndex_ToolTipService_ToolTip = 26,
                            XamlPropertyIndex_Typography_AnnotationAlternates = 28,
                            XamlPropertyIndex_Typography_Capitals = 29,
                            XamlPropertyIndex_Typography_CapitalSpacing = 30,
                            XamlPropertyIndex_Typography_CaseSensitiveForms = 31,
                            XamlPropertyIndex_Typography_ContextualAlternates = 32,
                            XamlPropertyIndex_Typography_ContextualLigatures = 33,
                            XamlPropertyIndex_Typography_ContextualSwashes = 34,
                            XamlPropertyIndex_Typography_DiscretionaryLigatures = 35,
                            XamlPropertyIndex_Typography_EastAsianExpertForms = 36,
                            XamlPropertyIndex_Typography_EastAsianLanguage = 37,
                            XamlPropertyIndex_Typography_EastAsianWidths = 38,
                            XamlPropertyIndex_Typography_Fraction = 39,
                            XamlPropertyIndex_Typography_HistoricalForms = 40,
                            XamlPropertyIndex_Typography_HistoricalLigatures = 41,
                            XamlPropertyIndex_Typography_Kerning = 42,
                            XamlPropertyIndex_Typography_MathematicalGreek = 43,
                            XamlPropertyIndex_Typography_NumeralAlignment = 44,
                            XamlPropertyIndex_Typography_NumeralStyle = 45,
                            XamlPropertyIndex_Typography_SlashedZero = 46,
                            XamlPropertyIndex_Typography_StandardLigatures = 47,
                            XamlPropertyIndex_Typography_StandardSwashes = 48,
                            XamlPropertyIndex_Typography_StylisticAlternates = 49,
                            XamlPropertyIndex_Typography_StylisticSet1 = 50,
                            XamlPropertyIndex_Typography_StylisticSet10 = 51,
                            XamlPropertyIndex_Typography_StylisticSet11 = 52,
                            XamlPropertyIndex_Typography_StylisticSet12 = 53,
                            XamlPropertyIndex_Typography_StylisticSet13 = 54,
                            XamlPropertyIndex_Typography_StylisticSet14 = 55,
                            XamlPropertyIndex_Typography_StylisticSet15 = 56,
                            XamlPropertyIndex_Typography_StylisticSet16 = 57,
                            XamlPropertyIndex_Typography_StylisticSet17 = 58,
                            XamlPropertyIndex_Typography_StylisticSet18 = 59,
                            XamlPropertyIndex_Typography_StylisticSet19 = 60,
                            XamlPropertyIndex_Typography_StylisticSet2 = 61,
                            XamlPropertyIndex_Typography_StylisticSet20 = 62,
                            XamlPropertyIndex_Typography_StylisticSet3 = 63,
                            XamlPropertyIndex_Typography_StylisticSet4 = 64,
                            XamlPropertyIndex_Typography_StylisticSet5 = 65,
                            XamlPropertyIndex_Typography_StylisticSet6 = 66,
                            XamlPropertyIndex_Typography_StylisticSet7 = 67,
                            XamlPropertyIndex_Typography_StylisticSet8 = 68,
                            XamlPropertyIndex_Typography_StylisticSet9 = 69,
                            XamlPropertyIndex_Typography_Variants = 70,
                            XamlPropertyIndex_AutomationPeer_EventsSource = 75,
                            XamlPropertyIndex_AutoSuggestBoxSuggestionChosenEventArgs_SelectedItem = 76,
                            XamlPropertyIndex_AutoSuggestBoxTextChangedEventArgs_Reason = 77,
                            XamlPropertyIndex_Brush_Opacity = 78,
                            XamlPropertyIndex_Brush_RelativeTransform = 79,
                            XamlPropertyIndex_Brush_Transform = 80,
                            XamlPropertyIndex_CollectionViewSource_IsSourceGrouped = 81,
                            XamlPropertyIndex_CollectionViewSource_ItemsPath = 82,
                            XamlPropertyIndex_CollectionViewSource_Source = 83,
                            XamlPropertyIndex_CollectionViewSource_View = 84,
                            XamlPropertyIndex_ColorKeyFrame_KeyTime = 90,
                            XamlPropertyIndex_ColorKeyFrame_Value = 91,
                            XamlPropertyIndex_ColumnDefinition_ActualWidth = 92,
                            XamlPropertyIndex_ColumnDefinition_MaxWidth = 93,
                            XamlPropertyIndex_ColumnDefinition_MinWidth = 94,
                            XamlPropertyIndex_ColumnDefinition_Width = 95,
                            XamlPropertyIndex_ComboBoxTemplateSettings_DropDownClosedHeight = 96,
                            XamlPropertyIndex_ComboBoxTemplateSettings_DropDownOffset = 97,
                            XamlPropertyIndex_ComboBoxTemplateSettings_DropDownOpenedHeight = 98,
                            XamlPropertyIndex_ComboBoxTemplateSettings_SelectedItemDirection = 99,
                            XamlPropertyIndex_DoubleKeyFrame_KeyTime = 107,
                            XamlPropertyIndex_DoubleKeyFrame_Value = 108,
                            XamlPropertyIndex_EasingFunctionBase_EasingMode = 111,
                            XamlPropertyIndex_FlyoutBase_AttachedFlyout = 114,
                            XamlPropertyIndex_FlyoutBase_Placement = 115,
                            XamlPropertyIndex_Geometry_Bounds = 118,
                            XamlPropertyIndex_Geometry_Transform = 119,
                            XamlPropertyIndex_GradientStop_Color = 120,
                            XamlPropertyIndex_GradientStop_Offset = 121,
                            XamlPropertyIndex_GroupStyle_ContainerStyle = 124,
                            XamlPropertyIndex_GroupStyle_ContainerStyleSelector = 125,
                            XamlPropertyIndex_GroupStyle_HeaderContainerStyle = 126,
                            XamlPropertyIndex_GroupStyle_HeaderTemplate = 127,
                            XamlPropertyIndex_GroupStyle_HeaderTemplateSelector = 128,
                            XamlPropertyIndex_GroupStyle_HidesIfEmpty = 129,
                            XamlPropertyIndex_GroupStyle_Panel = 130,
                            XamlPropertyIndex_InertiaExpansionBehavior_DesiredDeceleration = 144,
                            XamlPropertyIndex_InertiaExpansionBehavior_DesiredExpansion = 145,
                            XamlPropertyIndex_InertiaRotationBehavior_DesiredDeceleration = 146,
                            XamlPropertyIndex_InertiaRotationBehavior_DesiredRotation = 147,
                            XamlPropertyIndex_InertiaTranslationBehavior_DesiredDeceleration = 148,
                            XamlPropertyIndex_InertiaTranslationBehavior_DesiredDisplacement = 149,
                            XamlPropertyIndex_InputScope_Names = 150,
                            XamlPropertyIndex_InputScopeName_NameValue = 151,
                            XamlPropertyIndex_KeySpline_ControlPoint1 = 153,
                            XamlPropertyIndex_KeySpline_ControlPoint2 = 154,
                            XamlPropertyIndex_ManipulationPivot_Center = 159,
                            XamlPropertyIndex_ManipulationPivot_Radius = 160,
                            XamlPropertyIndex_ObjectKeyFrame_KeyTime = 183,
                            XamlPropertyIndex_ObjectKeyFrame_Value = 184,
                            XamlPropertyIndex_PageStackEntry_SourcePageType = 185,
                            XamlPropertyIndex_PathFigure_IsClosed = 192,
                            XamlPropertyIndex_PathFigure_IsFilled = 193,
                            XamlPropertyIndex_PathFigure_Segments = 194,
                            XamlPropertyIndex_PathFigure_StartPoint = 195,
                            XamlPropertyIndex_Pointer_IsInContact = 199,
                            XamlPropertyIndex_Pointer_IsInRange = 200,
                            XamlPropertyIndex_Pointer_PointerDeviceType = 201,
                            XamlPropertyIndex_Pointer_PointerId = 202,
                            XamlPropertyIndex_PointKeyFrame_KeyTime = 205,
                            XamlPropertyIndex_PointKeyFrame_Value = 206,
                            XamlPropertyIndex_PrintDocument_DocumentSource = 209,
                            XamlPropertyIndex_ProgressBarTemplateSettings_ContainerAnimationEndPosition = 211,
                            XamlPropertyIndex_ProgressBarTemplateSettings_ContainerAnimationStartPosition = 212,
                            XamlPropertyIndex_ProgressBarTemplateSettings_EllipseAnimationEndPosition = 213,
                            XamlPropertyIndex_ProgressBarTemplateSettings_EllipseAnimationWellPosition = 214,
                            XamlPropertyIndex_ProgressBarTemplateSettings_EllipseDiameter = 215,
                            XamlPropertyIndex_ProgressBarTemplateSettings_EllipseOffset = 216,
                            XamlPropertyIndex_ProgressBarTemplateSettings_IndicatorLengthDelta = 217,
                            XamlPropertyIndex_ProgressRingTemplateSettings_EllipseDiameter = 218,
                            XamlPropertyIndex_ProgressRingTemplateSettings_EllipseOffset = 219,
                            XamlPropertyIndex_ProgressRingTemplateSettings_MaxSideLength = 220,
                            XamlPropertyIndex_PropertyPath_Path = 221,
                            XamlPropertyIndex_RowDefinition_ActualHeight = 226,
                            XamlPropertyIndex_RowDefinition_Height = 227,
                            XamlPropertyIndex_RowDefinition_MaxHeight = 228,
                            XamlPropertyIndex_RowDefinition_MinHeight = 229,
                            XamlPropertyIndex_SetterBase_IsSealed = 233,
                            XamlPropertyIndex_SettingsFlyoutTemplateSettings_BorderBrush = 234,
                            XamlPropertyIndex_SettingsFlyoutTemplateSettings_BorderThickness = 235,
                            XamlPropertyIndex_SettingsFlyoutTemplateSettings_ContentTransitions = 236,
                            XamlPropertyIndex_SettingsFlyoutTemplateSettings_HeaderBackground = 237,
                            XamlPropertyIndex_SettingsFlyoutTemplateSettings_HeaderForeground = 238,
                            XamlPropertyIndex_SettingsFlyoutTemplateSettings_IconSource = 239,
                            XamlPropertyIndex_Style_BasedOn = 244,
                            XamlPropertyIndex_Style_IsSealed = 245,
                            XamlPropertyIndex_Style_Setters = 246,
                            XamlPropertyIndex_Style_TargetType = 247,
                            XamlPropertyIndex_TextElement_CharacterSpacing = 249,
                            XamlPropertyIndex_TextElement_FontFamily = 250,
                            XamlPropertyIndex_TextElement_FontSize = 251,
                            XamlPropertyIndex_TextElement_FontStretch = 252,
                            XamlPropertyIndex_TextElement_FontStyle = 253,
                            XamlPropertyIndex_TextElement_FontWeight = 254,
                            XamlPropertyIndex_TextElement_Foreground = 255,
                            XamlPropertyIndex_TextElement_IsTextScaleFactorEnabled = 256,
                            XamlPropertyIndex_TextElement_Language = 257,
                            XamlPropertyIndex_Timeline_AutoReverse = 263,
                            XamlPropertyIndex_Timeline_BeginTime = 264,
                            XamlPropertyIndex_Timeline_Duration = 265,
                            XamlPropertyIndex_Timeline_FillBehavior = 266,
                            XamlPropertyIndex_Timeline_RepeatBehavior = 267,
                            XamlPropertyIndex_Timeline_SpeedRatio = 268,
                            XamlPropertyIndex_TimelineMarker_Text = 269,
                            XamlPropertyIndex_TimelineMarker_Time = 270,
                            XamlPropertyIndex_TimelineMarker_Type = 271,
                            XamlPropertyIndex_ToggleSwitchTemplateSettings_CurtainCurrentToOffOffset = 273,
                            XamlPropertyIndex_ToggleSwitchTemplateSettings_CurtainCurrentToOnOffset = 274,
                            XamlPropertyIndex_ToggleSwitchTemplateSettings_CurtainOffToOnOffset = 275,
                            XamlPropertyIndex_ToggleSwitchTemplateSettings_CurtainOnToOffOffset = 276,
                            XamlPropertyIndex_ToggleSwitchTemplateSettings_KnobCurrentToOffOffset = 277,
                            XamlPropertyIndex_ToggleSwitchTemplateSettings_KnobCurrentToOnOffset = 278,
                            XamlPropertyIndex_ToggleSwitchTemplateSettings_KnobOffToOnOffset = 279,
                            XamlPropertyIndex_ToggleSwitchTemplateSettings_KnobOnToOffOffset = 280,
                            XamlPropertyIndex_ToolTipTemplateSettings_FromHorizontalOffset = 281,
                            XamlPropertyIndex_ToolTipTemplateSettings_FromVerticalOffset = 282,
                            XamlPropertyIndex_UIElement_AllowDrop = 292,
                            XamlPropertyIndex_UIElement_CacheMode = 293,
                            XamlPropertyIndex_UIElement_Clip = 295,
                            XamlPropertyIndex_UIElement_CompositeMode = 296,
                            XamlPropertyIndex_UIElement_IsDoubleTapEnabled = 297,
                            XamlPropertyIndex_UIElement_IsHitTestVisible = 298,
                            XamlPropertyIndex_UIElement_IsHoldingEnabled = 299,
                            XamlPropertyIndex_UIElement_IsRightTapEnabled = 300,
                            XamlPropertyIndex_UIElement_IsTapEnabled = 301,
                            XamlPropertyIndex_UIElement_ManipulationMode = 302,
                            XamlPropertyIndex_UIElement_Opacity = 303,
                            XamlPropertyIndex_UIElement_PointerCaptures = 304,
                            XamlPropertyIndex_UIElement_Projection = 305,
                            XamlPropertyIndex_UIElement_RenderSize = 306,
                            XamlPropertyIndex_UIElement_RenderTransform = 307,
                            XamlPropertyIndex_UIElement_RenderTransformOrigin = 308,
                            XamlPropertyIndex_UIElement_Transitions = 309,
                            XamlPropertyIndex_UIElement_UseLayoutRounding = 311,
                            XamlPropertyIndex_UIElement_Visibility = 312,
                            XamlPropertyIndex_VisualState_Storyboard = 322,
                            XamlPropertyIndex_VisualStateGroup_States = 323,
                            XamlPropertyIndex_VisualStateGroup_Transitions = 324,
                            XamlPropertyIndex_VisualStateManager_CustomVisualStateManager = 325,
                            XamlPropertyIndex_VisualStateManager_VisualStateGroups = 326,
                            XamlPropertyIndex_VisualTransition_From = 327,
                            XamlPropertyIndex_VisualTransition_GeneratedDuration = 328,
                            XamlPropertyIndex_VisualTransition_GeneratedEasingFunction = 329,
                            XamlPropertyIndex_VisualTransition_Storyboard = 330,
                            XamlPropertyIndex_VisualTransition_To = 331,
                            XamlPropertyIndex_ArcSegment_IsLargeArc = 332,
                            XamlPropertyIndex_ArcSegment_Point = 333,
                            XamlPropertyIndex_ArcSegment_RotationAngle = 334,
                            XamlPropertyIndex_ArcSegment_Size = 335,
                            XamlPropertyIndex_ArcSegment_SweepDirection = 336,
                            XamlPropertyIndex_BackEase_Amplitude = 337,
                            XamlPropertyIndex_BeginStoryboard_Storyboard = 338,
                            XamlPropertyIndex_BezierSegment_Point1 = 339,
                            XamlPropertyIndex_BezierSegment_Point2 = 340,
                            XamlPropertyIndex_BezierSegment_Point3 = 341,
                            XamlPropertyIndex_BitmapSource_PixelHeight = 342,
                            XamlPropertyIndex_BitmapSource_PixelWidth = 343,
                            XamlPropertyIndex_Block_LineHeight = 344,
                            XamlPropertyIndex_Block_LineStackingStrategy = 345,
                            XamlPropertyIndex_Block_Margin = 346,
                            XamlPropertyIndex_Block_TextAlignment = 347,
                            XamlPropertyIndex_BounceEase_Bounces = 348,
                            XamlPropertyIndex_BounceEase_Bounciness = 349,
                            XamlPropertyIndex_ColorAnimation_By = 350,
                            XamlPropertyIndex_ColorAnimation_EasingFunction = 351,
                            XamlPropertyIndex_ColorAnimation_EnableDependentAnimation = 352,
                            XamlPropertyIndex_ColorAnimation_From = 353,
                            XamlPropertyIndex_ColorAnimation_To = 354,
                            XamlPropertyIndex_ColorAnimationUsingKeyFrames_EnableDependentAnimation = 355,
                            XamlPropertyIndex_ColorAnimationUsingKeyFrames_KeyFrames = 356,
                            XamlPropertyIndex_ContentThemeTransition_HorizontalOffset = 357,
                            XamlPropertyIndex_ContentThemeTransition_VerticalOffset = 358,
                            XamlPropertyIndex_ControlTemplate_TargetType = 359,
                            XamlPropertyIndex_DispatcherTimer_Interval = 362,
                            XamlPropertyIndex_DoubleAnimation_By = 363,
                            XamlPropertyIndex_DoubleAnimation_EasingFunction = 364,
                            XamlPropertyIndex_DoubleAnimation_EnableDependentAnimation = 365,
                            XamlPropertyIndex_DoubleAnimation_From = 366,
                            XamlPropertyIndex_DoubleAnimation_To = 367,
                            XamlPropertyIndex_DoubleAnimationUsingKeyFrames_EnableDependentAnimation = 368,
                            XamlPropertyIndex_DoubleAnimationUsingKeyFrames_KeyFrames = 369,
                            XamlPropertyIndex_EasingColorKeyFrame_EasingFunction = 372,
                            XamlPropertyIndex_EasingDoubleKeyFrame_EasingFunction = 373,
                            XamlPropertyIndex_EasingPointKeyFrame_EasingFunction = 374,
                            XamlPropertyIndex_EdgeUIThemeTransition_Edge = 375,
                            XamlPropertyIndex_ElasticEase_Oscillations = 376,
                            XamlPropertyIndex_ElasticEase_Springiness = 377,
                            XamlPropertyIndex_EllipseGeometry_Center = 378,
                            XamlPropertyIndex_EllipseGeometry_RadiusX = 379,
                            XamlPropertyIndex_EllipseGeometry_RadiusY = 380,
                            XamlPropertyIndex_EntranceThemeTransition_FromHorizontalOffset = 381,
                            XamlPropertyIndex_EntranceThemeTransition_FromVerticalOffset = 382,
                            XamlPropertyIndex_EntranceThemeTransition_IsStaggeringEnabled = 383,
                            XamlPropertyIndex_EventTrigger_Actions = 384,
                            XamlPropertyIndex_EventTrigger_RoutedEvent = 385,
                            XamlPropertyIndex_ExponentialEase_Exponent = 386,
                            XamlPropertyIndex_Flyout_Content = 387,
                            XamlPropertyIndex_Flyout_FlyoutPresenterStyle = 388,
                            XamlPropertyIndex_FrameworkElement_ActualHeight = 389,
                            XamlPropertyIndex_FrameworkElement_ActualWidth = 390,
                            XamlPropertyIndex_FrameworkElement_DataContext = 391,
                            XamlPropertyIndex_FrameworkElement_FlowDirection = 392,
                            XamlPropertyIndex_FrameworkElement_Height = 393,
                            XamlPropertyIndex_FrameworkElement_HorizontalAlignment = 394,
                            XamlPropertyIndex_FrameworkElement_Language = 396,
                            XamlPropertyIndex_FrameworkElement_Margin = 397,
                            XamlPropertyIndex_FrameworkElement_MaxHeight = 398,
                            XamlPropertyIndex_FrameworkElement_MaxWidth = 399,
                            XamlPropertyIndex_FrameworkElement_MinHeight = 400,
                            XamlPropertyIndex_FrameworkElement_MinWidth = 401,
                            XamlPropertyIndex_FrameworkElement_Parent = 402,
                            XamlPropertyIndex_FrameworkElement_RequestedTheme = 403,
                            XamlPropertyIndex_FrameworkElement_Resources = 404,
                            XamlPropertyIndex_FrameworkElement_Style = 405,
                            XamlPropertyIndex_FrameworkElement_Tag = 406,
                            XamlPropertyIndex_FrameworkElement_Triggers = 407,
                            XamlPropertyIndex_FrameworkElement_VerticalAlignment = 408,
                            XamlPropertyIndex_FrameworkElement_Width = 409,
                            XamlPropertyIndex_FrameworkElementAutomationPeer_Owner = 410,
                            XamlPropertyIndex_GeometryGroup_Children = 411,
                            XamlPropertyIndex_GeometryGroup_FillRule = 412,
                            XamlPropertyIndex_GradientBrush_ColorInterpolationMode = 413,
                            XamlPropertyIndex_GradientBrush_GradientStops = 414,
                            XamlPropertyIndex_GradientBrush_MappingMode = 415,
                            XamlPropertyIndex_GradientBrush_SpreadMethod = 416,
                            XamlPropertyIndex_GridViewItemTemplateSettings_DragItemsCount = 417,
                            XamlPropertyIndex_ItemAutomationPeer_Item = 419,
                            XamlPropertyIndex_ItemAutomationPeer_ItemsControlAutomationPeer = 420,
                            XamlPropertyIndex_LineGeometry_EndPoint = 422,
                            XamlPropertyIndex_LineGeometry_StartPoint = 423,
                            XamlPropertyIndex_LineSegment_Point = 424,
                            XamlPropertyIndex_ListViewItemTemplateSettings_DragItemsCount = 425,
                            XamlPropertyIndex_Matrix3DProjection_ProjectionMatrix = 426,
                            XamlPropertyIndex_MenuFlyout_Items = 427,
                            XamlPropertyIndex_MenuFlyout_MenuFlyoutPresenterStyle = 428,
                            XamlPropertyIndex_ObjectAnimationUsingKeyFrames_EnableDependentAnimation = 429,
                            XamlPropertyIndex_ObjectAnimationUsingKeyFrames_KeyFrames = 430,
                            XamlPropertyIndex_PaneThemeTransition_Edge = 431,
                            XamlPropertyIndex_PathGeometry_Figures = 432,
                            XamlPropertyIndex_PathGeometry_FillRule = 433,
                            XamlPropertyIndex_PlaneProjection_CenterOfRotationX = 434,
                            XamlPropertyIndex_PlaneProjection_CenterOfRotationY = 435,
                            XamlPropertyIndex_PlaneProjection_CenterOfRotationZ = 436,
                            XamlPropertyIndex_PlaneProjection_GlobalOffsetX = 437,
                            XamlPropertyIndex_PlaneProjection_GlobalOffsetY = 438,
                            XamlPropertyIndex_PlaneProjection_GlobalOffsetZ = 439,
                            XamlPropertyIndex_PlaneProjection_LocalOffsetX = 440,
                            XamlPropertyIndex_PlaneProjection_LocalOffsetY = 441,
                            XamlPropertyIndex_PlaneProjection_LocalOffsetZ = 442,
                            XamlPropertyIndex_PlaneProjection_ProjectionMatrix = 443,
                            XamlPropertyIndex_PlaneProjection_RotationX = 444,
                            XamlPropertyIndex_PlaneProjection_RotationY = 445,
                            XamlPropertyIndex_PlaneProjection_RotationZ = 446,
                            XamlPropertyIndex_PointAnimation_By = 447,
                            XamlPropertyIndex_PointAnimation_EasingFunction = 448,
                            XamlPropertyIndex_PointAnimation_EnableDependentAnimation = 449,
                            XamlPropertyIndex_PointAnimation_From = 450,
                            XamlPropertyIndex_PointAnimation_To = 451,
                            XamlPropertyIndex_PointAnimationUsingKeyFrames_EnableDependentAnimation = 452,
                            XamlPropertyIndex_PointAnimationUsingKeyFrames_KeyFrames = 453,
                            XamlPropertyIndex_PolyBezierSegment_Points = 456,
                            XamlPropertyIndex_PolyLineSegment_Points = 457,
                            XamlPropertyIndex_PolyQuadraticBezierSegment_Points = 458,
                            XamlPropertyIndex_PopupThemeTransition_FromHorizontalOffset = 459,
                            XamlPropertyIndex_PopupThemeTransition_FromVerticalOffset = 460,
                            XamlPropertyIndex_PowerEase_Power = 461,
                            XamlPropertyIndex_QuadraticBezierSegment_Point1 = 466,
                            XamlPropertyIndex_QuadraticBezierSegment_Point2 = 467,
                            XamlPropertyIndex_RectangleGeometry_Rect = 470,
                            XamlPropertyIndex_RelativeSource_Mode = 471,
                            XamlPropertyIndex_RenderTargetBitmap_PixelHeight = 472,
                            XamlPropertyIndex_RenderTargetBitmap_PixelWidth = 473,
                            XamlPropertyIndex_Setter_Property = 474,
                            XamlPropertyIndex_Setter_Value = 475,
                            XamlPropertyIndex_SolidColorBrush_Color = 476,
                            XamlPropertyIndex_SplineColorKeyFrame_KeySpline = 477,
                            XamlPropertyIndex_SplineDoubleKeyFrame_KeySpline = 478,
                            XamlPropertyIndex_SplinePointKeyFrame_KeySpline = 479,
                            XamlPropertyIndex_TileBrush_AlignmentX = 483,
                            XamlPropertyIndex_TileBrush_AlignmentY = 484,
                            XamlPropertyIndex_TileBrush_Stretch = 485,
                            XamlPropertyIndex_Binding_Converter = 487,
                            XamlPropertyIndex_Binding_ConverterLanguage = 488,
                            XamlPropertyIndex_Binding_ConverterParameter = 489,
                            XamlPropertyIndex_Binding_ElementName = 490,
                            XamlPropertyIndex_Binding_FallbackValue = 491,
                            XamlPropertyIndex_Binding_Mode = 492,
                            XamlPropertyIndex_Binding_Path = 493,
                            XamlPropertyIndex_Binding_RelativeSource = 494,
                            XamlPropertyIndex_Binding_Source = 495,
                            XamlPropertyIndex_Binding_TargetNullValue = 496,
                            XamlPropertyIndex_Binding_UpdateSourceTrigger = 497,
                            XamlPropertyIndex_BitmapImage_CreateOptions = 498,
                            XamlPropertyIndex_BitmapImage_DecodePixelHeight = 499,
                            XamlPropertyIndex_BitmapImage_DecodePixelType = 500,
                            XamlPropertyIndex_BitmapImage_DecodePixelWidth = 501,
                            XamlPropertyIndex_BitmapImage_UriSource = 502,
                            XamlPropertyIndex_Border_Background = 503,
                            XamlPropertyIndex_Border_BorderBrush = 504,
                            XamlPropertyIndex_Border_BorderThickness = 505,
                            XamlPropertyIndex_Border_Child = 506,
                            XamlPropertyIndex_Border_ChildTransitions = 507,
                            XamlPropertyIndex_Border_CornerRadius = 508,
                            XamlPropertyIndex_Border_Padding = 509,
                            XamlPropertyIndex_CaptureElement_Source = 510,
                            XamlPropertyIndex_CaptureElement_Stretch = 511,
                            XamlPropertyIndex_CompositeTransform_CenterX = 514,
                            XamlPropertyIndex_CompositeTransform_CenterY = 515,
                            XamlPropertyIndex_CompositeTransform_Rotation = 516,
                            XamlPropertyIndex_CompositeTransform_ScaleX = 517,
                            XamlPropertyIndex_CompositeTransform_ScaleY = 518,
                            XamlPropertyIndex_CompositeTransform_SkewX = 519,
                            XamlPropertyIndex_CompositeTransform_SkewY = 520,
                            XamlPropertyIndex_CompositeTransform_TranslateX = 521,
                            XamlPropertyIndex_CompositeTransform_TranslateY = 522,
                            XamlPropertyIndex_ContentPresenter_CharacterSpacing = 523,
                            XamlPropertyIndex_ContentPresenter_Content = 524,
                            XamlPropertyIndex_ContentPresenter_ContentTemplate = 525,
                            XamlPropertyIndex_ContentPresenter_ContentTemplateSelector = 526,
                            XamlPropertyIndex_ContentPresenter_ContentTransitions = 527,
                            XamlPropertyIndex_ContentPresenter_FontFamily = 528,
                            XamlPropertyIndex_ContentPresenter_FontSize = 529,
                            XamlPropertyIndex_ContentPresenter_FontStretch = 530,
                            XamlPropertyIndex_ContentPresenter_FontStyle = 531,
                            XamlPropertyIndex_ContentPresenter_FontWeight = 532,
                            XamlPropertyIndex_ContentPresenter_Foreground = 533,
                            XamlPropertyIndex_ContentPresenter_IsTextScaleFactorEnabled = 534,
                            XamlPropertyIndex_ContentPresenter_LineStackingStrategy = 535,
                            XamlPropertyIndex_ContentPresenter_MaxLines = 536,
                            XamlPropertyIndex_ContentPresenter_OpticalMarginAlignment = 537,
                            XamlPropertyIndex_ContentPresenter_TextLineBounds = 539,
                            XamlPropertyIndex_ContentPresenter_TextWrapping = 540,
                            XamlPropertyIndex_Control_Background = 541,
                            XamlPropertyIndex_Control_BorderBrush = 542,
                            XamlPropertyIndex_Control_BorderThickness = 543,
                            XamlPropertyIndex_Control_CharacterSpacing = 544,
                            XamlPropertyIndex_Control_FocusState = 546,
                            XamlPropertyIndex_Control_FontFamily = 547,
                            XamlPropertyIndex_Control_FontSize = 548,
                            XamlPropertyIndex_Control_FontStretch = 549,
                            XamlPropertyIndex_Control_FontStyle = 550,
                            XamlPropertyIndex_Control_FontWeight = 551,
                            XamlPropertyIndex_Control_Foreground = 552,
                            XamlPropertyIndex_Control_HorizontalContentAlignment = 553,
                            XamlPropertyIndex_Control_IsEnabled = 554,
                            XamlPropertyIndex_Control_IsTabStop = 555,
                            XamlPropertyIndex_Control_IsTextScaleFactorEnabled = 556,
                            XamlPropertyIndex_Control_Padding = 557,
                            XamlPropertyIndex_Control_TabIndex = 558,
                            XamlPropertyIndex_Control_TabNavigation = 559,
                            XamlPropertyIndex_Control_Template = 560,
                            XamlPropertyIndex_Control_VerticalContentAlignment = 561,
                            XamlPropertyIndex_DragItemThemeAnimation_TargetName = 565,
                            XamlPropertyIndex_DragOverThemeAnimation_Direction = 566,
                            XamlPropertyIndex_DragOverThemeAnimation_TargetName = 567,
                            XamlPropertyIndex_DragOverThemeAnimation_ToOffset = 568,
                            XamlPropertyIndex_DropTargetItemThemeAnimation_TargetName = 569,
                            XamlPropertyIndex_FadeInThemeAnimation_TargetName = 570,
                            XamlPropertyIndex_FadeOutThemeAnimation_TargetName = 571,
                            XamlPropertyIndex_Glyphs_Fill = 574,
                            XamlPropertyIndex_Glyphs_FontRenderingEmSize = 575,
                            XamlPropertyIndex_Glyphs_FontUri = 576,
                            XamlPropertyIndex_Glyphs_Indices = 577,
                            XamlPropertyIndex_Glyphs_OriginX = 578,
                            XamlPropertyIndex_Glyphs_OriginY = 579,
                            XamlPropertyIndex_Glyphs_StyleSimulations = 580,
                            XamlPropertyIndex_Glyphs_UnicodeString = 581,
                            XamlPropertyIndex_IconElement_Foreground = 584,
                            XamlPropertyIndex_Image_NineGrid = 586,
                            XamlPropertyIndex_Image_PlayToSource = 587,
                            XamlPropertyIndex_Image_Source = 588,
                            XamlPropertyIndex_Image_Stretch = 589,
                            XamlPropertyIndex_ImageBrush_ImageSource = 591,
                            XamlPropertyIndex_InlineUIContainer_Child = 592,
                            XamlPropertyIndex_ItemsPresenter_Footer = 594,
                            XamlPropertyIndex_ItemsPresenter_FooterTemplate = 595,
                            XamlPropertyIndex_ItemsPresenter_FooterTransitions = 596,
                            XamlPropertyIndex_ItemsPresenter_Header = 597,
                            XamlPropertyIndex_ItemsPresenter_HeaderTemplate = 598,
                            XamlPropertyIndex_ItemsPresenter_HeaderTransitions = 599,
                            XamlPropertyIndex_ItemsPresenter_Padding = 601,
                            XamlPropertyIndex_LinearGradientBrush_EndPoint = 602,
                            XamlPropertyIndex_LinearGradientBrush_StartPoint = 603,
                            XamlPropertyIndex_MatrixTransform_Matrix = 604,
                            XamlPropertyIndex_MediaElement_ActualStereo3DVideoPackingMode = 605,
                            XamlPropertyIndex_MediaElement_AreTransportControlsEnabled = 606,
                            XamlPropertyIndex_MediaElement_AspectRatioHeight = 607,
                            XamlPropertyIndex_MediaElement_AspectRatioWidth = 608,
                            XamlPropertyIndex_MediaElement_AudioCategory = 609,
                            XamlPropertyIndex_MediaElement_AudioDeviceType = 610,
                            XamlPropertyIndex_MediaElement_AudioStreamCount = 611,
                            XamlPropertyIndex_MediaElement_AudioStreamIndex = 612,
                            XamlPropertyIndex_MediaElement_AutoPlay = 613,
                            XamlPropertyIndex_MediaElement_Balance = 614,
                            XamlPropertyIndex_MediaElement_BufferingProgress = 615,
                            XamlPropertyIndex_MediaElement_CanPause = 616,
                            XamlPropertyIndex_MediaElement_CanSeek = 617,
                            XamlPropertyIndex_MediaElement_CurrentState = 618,
                            XamlPropertyIndex_MediaElement_DefaultPlaybackRate = 619,
                            XamlPropertyIndex_MediaElement_DownloadProgress = 620,
                            XamlPropertyIndex_MediaElement_DownloadProgressOffset = 621,
                            XamlPropertyIndex_MediaElement_IsAudioOnly = 623,
                            XamlPropertyIndex_MediaElement_IsFullWindow = 624,
                            XamlPropertyIndex_MediaElement_IsLooping = 625,
                            XamlPropertyIndex_MediaElement_IsMuted = 626,
                            XamlPropertyIndex_MediaElement_IsStereo3DVideo = 627,
                            XamlPropertyIndex_MediaElement_Markers = 628,
                            XamlPropertyIndex_MediaElement_NaturalDuration = 629,
                            XamlPropertyIndex_MediaElement_NaturalVideoHeight = 630,
                            XamlPropertyIndex_MediaElement_NaturalVideoWidth = 631,
                            XamlPropertyIndex_MediaElement_PlaybackRate = 632,
                            XamlPropertyIndex_MediaElement_PlayToPreferredSourceUri = 633,
                            XamlPropertyIndex_MediaElement_PlayToSource = 634,
                            XamlPropertyIndex_MediaElement_Position = 635,
                            XamlPropertyIndex_MediaElement_PosterSource = 636,
                            XamlPropertyIndex_MediaElement_ProtectionManager = 637,
                            XamlPropertyIndex_MediaElement_RealTimePlayback = 638,
                            XamlPropertyIndex_MediaElement_Source = 639,
                            XamlPropertyIndex_MediaElement_Stereo3DVideoPackingMode = 640,
                            XamlPropertyIndex_MediaElement_Stereo3DVideoRenderMode = 641,
                            XamlPropertyIndex_MediaElement_Stretch = 642,
                            XamlPropertyIndex_MediaElement_TransportControls = 643,
                            XamlPropertyIndex_MediaElement_Volume = 644,
                            XamlPropertyIndex_Panel_Background = 647,
                            XamlPropertyIndex_Panel_Children = 648,
                            XamlPropertyIndex_Panel_ChildrenTransitions = 649,
                            XamlPropertyIndex_Panel_IsItemsHost = 651,
                            XamlPropertyIndex_Paragraph_Inlines = 652,
                            XamlPropertyIndex_Paragraph_TextIndent = 653,
                            XamlPropertyIndex_PointerDownThemeAnimation_TargetName = 660,
                            XamlPropertyIndex_PointerUpThemeAnimation_TargetName = 662,
                            XamlPropertyIndex_PopInThemeAnimation_FromHorizontalOffset = 664,
                            XamlPropertyIndex_PopInThemeAnimation_FromVerticalOffset = 665,
                            XamlPropertyIndex_PopInThemeAnimation_TargetName = 666,
                            XamlPropertyIndex_PopOutThemeAnimation_TargetName = 667,
                            XamlPropertyIndex_Popup_Child = 668,
                            XamlPropertyIndex_Popup_ChildTransitions = 669,
                            XamlPropertyIndex_Popup_HorizontalOffset = 670,
                            XamlPropertyIndex_Popup_IsLightDismissEnabled = 673,
                            XamlPropertyIndex_Popup_IsOpen = 674,
                            XamlPropertyIndex_Popup_VerticalOffset = 676,
                            XamlPropertyIndex_RepositionThemeAnimation_FromHorizontalOffset = 683,
                            XamlPropertyIndex_RepositionThemeAnimation_FromVerticalOffset = 684,
                            XamlPropertyIndex_RepositionThemeAnimation_TargetName = 685,
                            XamlPropertyIndex_ResourceDictionary_MergedDictionaries = 687,
                            XamlPropertyIndex_ResourceDictionary_Source = 688,
                            XamlPropertyIndex_ResourceDictionary_ThemeDictionaries = 689,
                            XamlPropertyIndex_RichTextBlock_Blocks = 691,
                            XamlPropertyIndex_RichTextBlock_CharacterSpacing = 692,
                            XamlPropertyIndex_RichTextBlock_FontFamily = 693,
                            XamlPropertyIndex_RichTextBlock_FontSize = 694,
                            XamlPropertyIndex_RichTextBlock_FontStretch = 695,
                            XamlPropertyIndex_RichTextBlock_FontStyle = 696,
                            XamlPropertyIndex_RichTextBlock_FontWeight = 697,
                            XamlPropertyIndex_RichTextBlock_Foreground = 698,
                            XamlPropertyIndex_RichTextBlock_HasOverflowContent = 699,
                            XamlPropertyIndex_RichTextBlock_IsColorFontEnabled = 700,
                            XamlPropertyIndex_RichTextBlock_IsTextScaleFactorEnabled = 701,
                            XamlPropertyIndex_RichTextBlock_IsTextSelectionEnabled = 702,
                            XamlPropertyIndex_RichTextBlock_LineHeight = 703,
                            XamlPropertyIndex_RichTextBlock_LineStackingStrategy = 704,
                            XamlPropertyIndex_RichTextBlock_MaxLines = 705,
                            XamlPropertyIndex_RichTextBlock_OpticalMarginAlignment = 706,
                            XamlPropertyIndex_RichTextBlock_OverflowContentTarget = 707,
                            XamlPropertyIndex_RichTextBlock_Padding = 708,
                            XamlPropertyIndex_RichTextBlock_SelectedText = 709,
                            XamlPropertyIndex_RichTextBlock_SelectionHighlightColor = 710,
                            XamlPropertyIndex_RichTextBlock_TextAlignment = 711,
                            XamlPropertyIndex_RichTextBlock_TextIndent = 712,
                            XamlPropertyIndex_RichTextBlock_TextLineBounds = 713,
                            XamlPropertyIndex_RichTextBlock_TextReadingOrder = 714,
                            XamlPropertyIndex_RichTextBlock_TextTrimming = 715,
                            XamlPropertyIndex_RichTextBlock_TextWrapping = 716,
                            XamlPropertyIndex_RichTextBlockOverflow_HasOverflowContent = 717,
                            XamlPropertyIndex_RichTextBlockOverflow_MaxLines = 718,
                            XamlPropertyIndex_RichTextBlockOverflow_OverflowContentTarget = 719,
                            XamlPropertyIndex_RichTextBlockOverflow_Padding = 720,
                            XamlPropertyIndex_RotateTransform_Angle = 721,
                            XamlPropertyIndex_RotateTransform_CenterX = 722,
                            XamlPropertyIndex_RotateTransform_CenterY = 723,
                            XamlPropertyIndex_Run_FlowDirection = 725,
                            XamlPropertyIndex_Run_Text = 726,
                            XamlPropertyIndex_ScaleTransform_CenterX = 727,
                            XamlPropertyIndex_ScaleTransform_CenterY = 728,
                            XamlPropertyIndex_ScaleTransform_ScaleX = 729,
                            XamlPropertyIndex_ScaleTransform_ScaleY = 730,
                            XamlPropertyIndex_SetterBaseCollection_IsSealed = 732,
                            XamlPropertyIndex_Shape_Fill = 733,
                            XamlPropertyIndex_Shape_GeometryTransform = 734,
                            XamlPropertyIndex_Shape_Stretch = 735,
                            XamlPropertyIndex_Shape_Stroke = 736,
                            XamlPropertyIndex_Shape_StrokeDashArray = 737,
                            XamlPropertyIndex_Shape_StrokeDashCap = 738,
                            XamlPropertyIndex_Shape_StrokeDashOffset = 739,
                            XamlPropertyIndex_Shape_StrokeEndLineCap = 740,
                            XamlPropertyIndex_Shape_StrokeLineJoin = 741,
                            XamlPropertyIndex_Shape_StrokeMiterLimit = 742,
                            XamlPropertyIndex_Shape_StrokeStartLineCap = 743,
                            XamlPropertyIndex_Shape_StrokeThickness = 744,
                            XamlPropertyIndex_SkewTransform_AngleX = 745,
                            XamlPropertyIndex_SkewTransform_AngleY = 746,
                            XamlPropertyIndex_SkewTransform_CenterX = 747,
                            XamlPropertyIndex_SkewTransform_CenterY = 748,
                            XamlPropertyIndex_Span_Inlines = 749,
                            XamlPropertyIndex_SplitCloseThemeAnimation_ClosedLength = 750,
                            XamlPropertyIndex_SplitCloseThemeAnimation_ClosedTarget = 751,
                            XamlPropertyIndex_SplitCloseThemeAnimation_ClosedTargetName = 752,
                            XamlPropertyIndex_SplitCloseThemeAnimation_ContentTarget = 753,
                            XamlPropertyIndex_SplitCloseThemeAnimation_ContentTargetName = 754,
                            XamlPropertyIndex_SplitCloseThemeAnimation_ContentTranslationDirection = 755,
                            XamlPropertyIndex_SplitCloseThemeAnimation_ContentTranslationOffset = 756,
                            XamlPropertyIndex_SplitCloseThemeAnimation_OffsetFromCenter = 757,
                            XamlPropertyIndex_SplitCloseThemeAnimation_OpenedLength = 758,
                            XamlPropertyIndex_SplitCloseThemeAnimation_OpenedTarget = 759,
                            XamlPropertyIndex_SplitCloseThemeAnimation_OpenedTargetName = 760,
                            XamlPropertyIndex_SplitOpenThemeAnimation_ClosedLength = 761,
                            XamlPropertyIndex_SplitOpenThemeAnimation_ClosedTarget = 762,
                            XamlPropertyIndex_SplitOpenThemeAnimation_ClosedTargetName = 763,
                            XamlPropertyIndex_SplitOpenThemeAnimation_ContentTarget = 764,
                            XamlPropertyIndex_SplitOpenThemeAnimation_ContentTargetName = 765,
                            XamlPropertyIndex_SplitOpenThemeAnimation_ContentTranslationDirection = 766,
                            XamlPropertyIndex_SplitOpenThemeAnimation_ContentTranslationOffset = 767,
                            XamlPropertyIndex_SplitOpenThemeAnimation_OffsetFromCenter = 768,
                            XamlPropertyIndex_SplitOpenThemeAnimation_OpenedLength = 769,
                            XamlPropertyIndex_SplitOpenThemeAnimation_OpenedTarget = 770,
                            XamlPropertyIndex_SplitOpenThemeAnimation_OpenedTargetName = 771,
                            XamlPropertyIndex_Storyboard_Children = 772,
                            XamlPropertyIndex_Storyboard_TargetName = 774,
                            XamlPropertyIndex_Storyboard_TargetProperty = 775,
                            XamlPropertyIndex_SwipeBackThemeAnimation_FromHorizontalOffset = 776,
                            XamlPropertyIndex_SwipeBackThemeAnimation_FromVerticalOffset = 777,
                            XamlPropertyIndex_SwipeBackThemeAnimation_TargetName = 778,
                            XamlPropertyIndex_SwipeHintThemeAnimation_TargetName = 779,
                            XamlPropertyIndex_SwipeHintThemeAnimation_ToHorizontalOffset = 780,
                            XamlPropertyIndex_SwipeHintThemeAnimation_ToVerticalOffset = 781,
                            XamlPropertyIndex_TextBlock_CharacterSpacing = 782,
                            XamlPropertyIndex_TextBlock_FontFamily = 783,
                            XamlPropertyIndex_TextBlock_FontSize = 784,
                            XamlPropertyIndex_TextBlock_FontStretch = 785,
                            XamlPropertyIndex_TextBlock_FontStyle = 786,
                            XamlPropertyIndex_TextBlock_FontWeight = 787,
                            XamlPropertyIndex_TextBlock_Foreground = 788,
                            XamlPropertyIndex_TextBlock_Inlines = 789,
                            XamlPropertyIndex_TextBlock_IsColorFontEnabled = 790,
                            XamlPropertyIndex_TextBlock_IsTextScaleFactorEnabled = 791,
                            XamlPropertyIndex_TextBlock_IsTextSelectionEnabled = 792,
                            XamlPropertyIndex_TextBlock_LineHeight = 793,
                            XamlPropertyIndex_TextBlock_LineStackingStrategy = 794,
                            XamlPropertyIndex_TextBlock_MaxLines = 795,
                            XamlPropertyIndex_TextBlock_OpticalMarginAlignment = 796,
                            XamlPropertyIndex_TextBlock_Padding = 797,
                            XamlPropertyIndex_TextBlock_SelectedText = 798,
                            XamlPropertyIndex_TextBlock_SelectionHighlightColor = 799,
                            XamlPropertyIndex_TextBlock_Text = 800,
                            XamlPropertyIndex_TextBlock_TextAlignment = 801,
                            XamlPropertyIndex_TextBlock_TextDecorations = 802,
                            XamlPropertyIndex_TextBlock_TextLineBounds = 803,
                            XamlPropertyIndex_TextBlock_TextReadingOrder = 804,
                            XamlPropertyIndex_TextBlock_TextTrimming = 805,
                            XamlPropertyIndex_TextBlock_TextWrapping = 806,
                            XamlPropertyIndex_TransformGroup_Children = 811,
                            XamlPropertyIndex_TransformGroup_Value = 812,
                            XamlPropertyIndex_TranslateTransform_X = 814,
                            XamlPropertyIndex_TranslateTransform_Y = 815,
                            XamlPropertyIndex_Viewbox_Child = 819,
                            XamlPropertyIndex_Viewbox_Stretch = 820,
                            XamlPropertyIndex_Viewbox_StretchDirection = 821,
                            XamlPropertyIndex_WebViewBrush_SourceName = 825,
                            XamlPropertyIndex_AppBarSeparator_IsCompact = 826,
                            XamlPropertyIndex_BitmapIcon_UriSource = 827,
                            XamlPropertyIndex_Canvas_Left = 828,
                            XamlPropertyIndex_Canvas_Top = 829,
                            XamlPropertyIndex_Canvas_ZIndex = 830,
                            XamlPropertyIndex_ContentControl_Content = 832,
                            XamlPropertyIndex_ContentControl_ContentTemplate = 833,
                            XamlPropertyIndex_ContentControl_ContentTemplateSelector = 834,
                            XamlPropertyIndex_ContentControl_ContentTransitions = 835,
                            XamlPropertyIndex_DatePicker_CalendarIdentifier = 837,
                            XamlPropertyIndex_DatePicker_Date = 838,
                            XamlPropertyIndex_DatePicker_DayFormat = 839,
                            XamlPropertyIndex_DatePicker_DayVisible = 840,
                            XamlPropertyIndex_DatePicker_Header = 841,
                            XamlPropertyIndex_DatePicker_HeaderTemplate = 842,
                            XamlPropertyIndex_DatePicker_MaxYear = 843,
                            XamlPropertyIndex_DatePicker_MinYear = 844,
                            XamlPropertyIndex_DatePicker_MonthFormat = 845,
                            XamlPropertyIndex_DatePicker_MonthVisible = 846,
                            XamlPropertyIndex_DatePicker_Orientation = 847,
                            XamlPropertyIndex_DatePicker_YearFormat = 848,
                            XamlPropertyIndex_DatePicker_YearVisible = 849,
                            XamlPropertyIndex_FontIcon_FontFamily = 851,
                            XamlPropertyIndex_FontIcon_FontSize = 852,
                            XamlPropertyIndex_FontIcon_FontStyle = 853,
                            XamlPropertyIndex_FontIcon_FontWeight = 854,
                            XamlPropertyIndex_FontIcon_Glyph = 855,
                            XamlPropertyIndex_FontIcon_IsTextScaleFactorEnabled = 856,
                            XamlPropertyIndex_Grid_Column = 857,
                            XamlPropertyIndex_Grid_ColumnDefinitions = 858,
                            XamlPropertyIndex_Grid_ColumnSpan = 859,
                            XamlPropertyIndex_Grid_Row = 860,
                            XamlPropertyIndex_Grid_RowDefinitions = 861,
                            XamlPropertyIndex_Grid_RowSpan = 862,
                            XamlPropertyIndex_Hub_DefaultSectionIndex = 863,
                            XamlPropertyIndex_Hub_Header = 864,
                            XamlPropertyIndex_Hub_HeaderTemplate = 865,
                            XamlPropertyIndex_Hub_IsActiveView = 866,
                            XamlPropertyIndex_Hub_IsZoomedInView = 867,
                            XamlPropertyIndex_Hub_Orientation = 868,
                            XamlPropertyIndex_Hub_SectionHeaders = 869,
                            XamlPropertyIndex_Hub_Sections = 870,
                            XamlPropertyIndex_Hub_SectionsInView = 871,
                            XamlPropertyIndex_Hub_SemanticZoomOwner = 872,
                            XamlPropertyIndex_HubSection_ContentTemplate = 873,
                            XamlPropertyIndex_HubSection_Header = 874,
                            XamlPropertyIndex_HubSection_HeaderTemplate = 875,
                            XamlPropertyIndex_HubSection_IsHeaderInteractive = 876,
                            XamlPropertyIndex_Hyperlink_NavigateUri = 877,
                            XamlPropertyIndex_ItemsControl_DisplayMemberPath = 879,
                            XamlPropertyIndex_ItemsControl_GroupStyle = 880,
                            XamlPropertyIndex_ItemsControl_GroupStyleSelector = 881,
                            XamlPropertyIndex_ItemsControl_IsGrouping = 882,
                            XamlPropertyIndex_ItemsControl_ItemContainerStyle = 884,
                            XamlPropertyIndex_ItemsControl_ItemContainerStyleSelector = 885,
                            XamlPropertyIndex_ItemsControl_ItemContainerTransitions = 886,
                            XamlPropertyIndex_ItemsControl_Items = 887,
                            XamlPropertyIndex_ItemsControl_ItemsPanel = 889,
                            XamlPropertyIndex_ItemsControl_ItemsSource = 890,
                            XamlPropertyIndex_ItemsControl_ItemTemplate = 891,
                            XamlPropertyIndex_ItemsControl_ItemTemplateSelector = 892,
                            XamlPropertyIndex_Line_X1 = 893,
                            XamlPropertyIndex_Line_X2 = 894,
                            XamlPropertyIndex_Line_Y1 = 895,
                            XamlPropertyIndex_Line_Y2 = 896,
                            XamlPropertyIndex_MediaTransportControls_IsFastForwardButtonVisible = 898,
                            XamlPropertyIndex_MediaTransportControls_IsFastRewindButtonVisible = 900,
                            XamlPropertyIndex_MediaTransportControls_IsFullWindowButtonVisible = 902,
                            XamlPropertyIndex_MediaTransportControls_IsPlaybackRateButtonVisible = 904,
                            XamlPropertyIndex_MediaTransportControls_IsSeekBarVisible = 905,
                            XamlPropertyIndex_MediaTransportControls_IsStopButtonVisible = 908,
                            XamlPropertyIndex_MediaTransportControls_IsVolumeButtonVisible = 910,
                            XamlPropertyIndex_MediaTransportControls_IsZoomButtonVisible = 912,
                            XamlPropertyIndex_PasswordBox_Header = 913,
                            XamlPropertyIndex_PasswordBox_HeaderTemplate = 914,
                            XamlPropertyIndex_PasswordBox_IsPasswordRevealButtonEnabled = 915,
                            XamlPropertyIndex_PasswordBox_MaxLength = 916,
                            XamlPropertyIndex_PasswordBox_Password = 917,
                            XamlPropertyIndex_PasswordBox_PasswordChar = 918,
                            XamlPropertyIndex_PasswordBox_PlaceholderText = 919,
                            XamlPropertyIndex_PasswordBox_PreventKeyboardDisplayOnProgrammaticFocus = 920,
                            XamlPropertyIndex_PasswordBox_SelectionHighlightColor = 921,
                            XamlPropertyIndex_Path_Data = 922,
                            XamlPropertyIndex_PathIcon_Data = 923,
                            XamlPropertyIndex_Polygon_FillRule = 924,
                            XamlPropertyIndex_Polygon_Points = 925,
                            XamlPropertyIndex_Polyline_FillRule = 926,
                            XamlPropertyIndex_Polyline_Points = 927,
                            XamlPropertyIndex_ProgressRing_IsActive = 928,
                            XamlPropertyIndex_ProgressRing_TemplateSettings = 929,
                            XamlPropertyIndex_RangeBase_LargeChange = 930,
                            XamlPropertyIndex_RangeBase_Maximum = 931,
                            XamlPropertyIndex_RangeBase_Minimum = 932,
                            XamlPropertyIndex_RangeBase_SmallChange = 933,
                            XamlPropertyIndex_RangeBase_Value = 934,
                            XamlPropertyIndex_Rectangle_RadiusX = 935,
                            XamlPropertyIndex_Rectangle_RadiusY = 936,
                            XamlPropertyIndex_RichEditBox_AcceptsReturn = 937,
                            XamlPropertyIndex_RichEditBox_Header = 938,
                            XamlPropertyIndex_RichEditBox_HeaderTemplate = 939,
                            XamlPropertyIndex_RichEditBox_InputScope = 940,
                            XamlPropertyIndex_RichEditBox_IsColorFontEnabled = 941,
                            XamlPropertyIndex_RichEditBox_IsReadOnly = 942,
                            XamlPropertyIndex_RichEditBox_IsSpellCheckEnabled = 943,
                            XamlPropertyIndex_RichEditBox_IsTextPredictionEnabled = 944,
                            XamlPropertyIndex_RichEditBox_PlaceholderText = 945,
                            XamlPropertyIndex_RichEditBox_PreventKeyboardDisplayOnProgrammaticFocus = 946,
                            XamlPropertyIndex_RichEditBox_SelectionHighlightColor = 947,
                            XamlPropertyIndex_RichEditBox_TextAlignment = 948,
                            XamlPropertyIndex_RichEditBox_TextWrapping = 949,
                            XamlPropertyIndex_SearchBox_ChooseSuggestionOnEnter = 950,
                            XamlPropertyIndex_SearchBox_FocusOnKeyboardInput = 951,
                            XamlPropertyIndex_SearchBox_PlaceholderText = 952,
                            XamlPropertyIndex_SearchBox_QueryText = 953,
                            XamlPropertyIndex_SearchBox_SearchHistoryContext = 954,
                            XamlPropertyIndex_SearchBox_SearchHistoryEnabled = 955,
                            XamlPropertyIndex_SemanticZoom_CanChangeViews = 956,
                            XamlPropertyIndex_SemanticZoom_IsZoomedInViewActive = 957,
                            XamlPropertyIndex_SemanticZoom_IsZoomOutButtonEnabled = 958,
                            XamlPropertyIndex_SemanticZoom_ZoomedInView = 959,
                            XamlPropertyIndex_SemanticZoom_ZoomedOutView = 960,
                            XamlPropertyIndex_StackPanel_AreScrollSnapPointsRegular = 961,
                            XamlPropertyIndex_StackPanel_Orientation = 962,
                            XamlPropertyIndex_SymbolIcon_Symbol = 963,
                            XamlPropertyIndex_TextBox_AcceptsReturn = 964,
                            XamlPropertyIndex_TextBox_Header = 965,
                            XamlPropertyIndex_TextBox_HeaderTemplate = 966,
                            XamlPropertyIndex_TextBox_InputScope = 967,
                            XamlPropertyIndex_TextBox_IsColorFontEnabled = 968,
                            XamlPropertyIndex_TextBox_IsReadOnly = 971,
                            XamlPropertyIndex_TextBox_IsSpellCheckEnabled = 972,
                            XamlPropertyIndex_TextBox_IsTextPredictionEnabled = 973,
                            XamlPropertyIndex_TextBox_MaxLength = 974,
                            XamlPropertyIndex_TextBox_PlaceholderText = 975,
                            XamlPropertyIndex_TextBox_PreventKeyboardDisplayOnProgrammaticFocus = 976,
                            XamlPropertyIndex_TextBox_SelectedText = 977,
                            XamlPropertyIndex_TextBox_SelectionHighlightColor = 978,
                            XamlPropertyIndex_TextBox_SelectionLength = 979,
                            XamlPropertyIndex_TextBox_SelectionStart = 980,
                            XamlPropertyIndex_TextBox_Text = 981,
                            XamlPropertyIndex_TextBox_TextAlignment = 982,
                            XamlPropertyIndex_TextBox_TextWrapping = 983,
                            XamlPropertyIndex_Thumb_IsDragging = 984,
                            XamlPropertyIndex_TickBar_Fill = 985,
                            XamlPropertyIndex_TimePicker_ClockIdentifier = 986,
                            XamlPropertyIndex_TimePicker_Header = 987,
                            XamlPropertyIndex_TimePicker_HeaderTemplate = 988,
                            XamlPropertyIndex_TimePicker_MinuteIncrement = 989,
                            XamlPropertyIndex_TimePicker_Time = 990,
                            XamlPropertyIndex_ToggleSwitch_Header = 991,
                            XamlPropertyIndex_ToggleSwitch_HeaderTemplate = 992,
                            XamlPropertyIndex_ToggleSwitch_IsOn = 993,
                            XamlPropertyIndex_ToggleSwitch_OffContent = 994,
                            XamlPropertyIndex_ToggleSwitch_OffContentTemplate = 995,
                            XamlPropertyIndex_ToggleSwitch_OnContent = 996,
                            XamlPropertyIndex_ToggleSwitch_OnContentTemplate = 997,
                            XamlPropertyIndex_ToggleSwitch_TemplateSettings = 998,
                            XamlPropertyIndex_UserControl_Content = 999,
                            XamlPropertyIndex_VariableSizedWrapGrid_ColumnSpan = 1000,
                            XamlPropertyIndex_VariableSizedWrapGrid_HorizontalChildrenAlignment = 1001,
                            XamlPropertyIndex_VariableSizedWrapGrid_ItemHeight = 1002,
                            XamlPropertyIndex_VariableSizedWrapGrid_ItemWidth = 1003,
                            XamlPropertyIndex_VariableSizedWrapGrid_MaximumRowsOrColumns = 1004,
                            XamlPropertyIndex_VariableSizedWrapGrid_Orientation = 1005,
                            XamlPropertyIndex_VariableSizedWrapGrid_RowSpan = 1006,
                            XamlPropertyIndex_VariableSizedWrapGrid_VerticalChildrenAlignment = 1007,
                            XamlPropertyIndex_WebView_AllowedScriptNotifyUris = 1008,
                            XamlPropertyIndex_WebView_CanGoBack = 1009,
                            XamlPropertyIndex_WebView_CanGoForward = 1010,
                            XamlPropertyIndex_WebView_ContainsFullScreenElement = 1011,
                            XamlPropertyIndex_WebView_DataTransferPackage = 1012,
                            XamlPropertyIndex_WebView_DefaultBackgroundColor = 1013,
                            XamlPropertyIndex_WebView_DocumentTitle = 1014,
                            XamlPropertyIndex_WebView_Source = 1015,
                            XamlPropertyIndex_AppBar_ClosedDisplayMode = 1016,
                            XamlPropertyIndex_AppBar_IsOpen = 1017,
                            XamlPropertyIndex_AppBar_IsSticky = 1018,
                            XamlPropertyIndex_AutoSuggestBox_AutoMaximizeSuggestionArea = 1019,
                            XamlPropertyIndex_AutoSuggestBox_Header = 1020,
                            XamlPropertyIndex_AutoSuggestBox_IsSuggestionListOpen = 1021,
                            XamlPropertyIndex_AutoSuggestBox_MaxSuggestionListHeight = 1022,
                            XamlPropertyIndex_AutoSuggestBox_PlaceholderText = 1023,
                            XamlPropertyIndex_AutoSuggestBox_Text = 1024,
                            XamlPropertyIndex_AutoSuggestBox_TextBoxStyle = 1025,
                            XamlPropertyIndex_AutoSuggestBox_TextMemberPath = 1026,
                            XamlPropertyIndex_AutoSuggestBox_UpdateTextOnSelect = 1027,
                            XamlPropertyIndex_ButtonBase_ClickMode = 1029,
                            XamlPropertyIndex_ButtonBase_Command = 1030,
                            XamlPropertyIndex_ButtonBase_CommandParameter = 1031,
                            XamlPropertyIndex_ButtonBase_IsPointerOver = 1032,
                            XamlPropertyIndex_ButtonBase_IsPressed = 1033,
                            XamlPropertyIndex_ContentDialog_FullSizeDesired = 1034,
                            XamlPropertyIndex_ContentDialog_IsPrimaryButtonEnabled = 1035,
                            XamlPropertyIndex_ContentDialog_IsSecondaryButtonEnabled = 1036,
                            XamlPropertyIndex_ContentDialog_PrimaryButtonCommand = 1037,
                            XamlPropertyIndex_ContentDialog_PrimaryButtonCommandParameter = 1038,
                            XamlPropertyIndex_ContentDialog_PrimaryButtonText = 1039,
                            XamlPropertyIndex_ContentDialog_SecondaryButtonCommand = 1040,
                            XamlPropertyIndex_ContentDialog_SecondaryButtonCommandParameter = 1041,
                            XamlPropertyIndex_ContentDialog_SecondaryButtonText = 1042,
                            XamlPropertyIndex_ContentDialog_Title = 1043,
                            XamlPropertyIndex_ContentDialog_TitleTemplate = 1044,
                            XamlPropertyIndex_Frame_BackStack = 1045,
                            XamlPropertyIndex_Frame_BackStackDepth = 1046,
                            XamlPropertyIndex_Frame_CacheSize = 1047,
                            XamlPropertyIndex_Frame_CanGoBack = 1048,
                            XamlPropertyIndex_Frame_CanGoForward = 1049,
                            XamlPropertyIndex_Frame_CurrentSourcePageType = 1050,
                            XamlPropertyIndex_Frame_ForwardStack = 1051,
                            XamlPropertyIndex_Frame_SourcePageType = 1052,
                            XamlPropertyIndex_GridViewItemPresenter_CheckBrush = 1053,
                            XamlPropertyIndex_GridViewItemPresenter_CheckHintBrush = 1054,
                            XamlPropertyIndex_GridViewItemPresenter_CheckSelectingBrush = 1055,
                            XamlPropertyIndex_GridViewItemPresenter_ContentMargin = 1056,
                            XamlPropertyIndex_GridViewItemPresenter_DisabledOpacity = 1057,
                            XamlPropertyIndex_GridViewItemPresenter_DragBackground = 1058,
                            XamlPropertyIndex_GridViewItemPresenter_DragForeground = 1059,
                            XamlPropertyIndex_GridViewItemPresenter_DragOpacity = 1060,
                            XamlPropertyIndex_GridViewItemPresenter_FocusBorderBrush = 1061,
                            XamlPropertyIndex_GridViewItemPresenter_GridViewItemPresenterHorizontalContentAlignment = 1062,
                            XamlPropertyIndex_GridViewItemPresenter_GridViewItemPresenterPadding = 1063,
                            XamlPropertyIndex_GridViewItemPresenter_PlaceholderBackground = 1064,
                            XamlPropertyIndex_GridViewItemPresenter_PointerOverBackground = 1065,
                            XamlPropertyIndex_GridViewItemPresenter_PointerOverBackgroundMargin = 1066,
                            XamlPropertyIndex_GridViewItemPresenter_ReorderHintOffset = 1067,
                            XamlPropertyIndex_GridViewItemPresenter_SelectedBackground = 1068,
                            XamlPropertyIndex_GridViewItemPresenter_SelectedBorderThickness = 1069,
                            XamlPropertyIndex_GridViewItemPresenter_SelectedForeground = 1070,
                            XamlPropertyIndex_GridViewItemPresenter_SelectedPointerOverBackground = 1071,
                            XamlPropertyIndex_GridViewItemPresenter_SelectedPointerOverBorderBrush = 1072,
                            XamlPropertyIndex_GridViewItemPresenter_SelectionCheckMarkVisualEnabled = 1073,
                            XamlPropertyIndex_GridViewItemPresenter_GridViewItemPresenterVerticalContentAlignment = 1074,
                            XamlPropertyIndex_ItemsStackPanel_CacheLength = 1076,
                            XamlPropertyIndex_ItemsStackPanel_GroupHeaderPlacement = 1077,
                            XamlPropertyIndex_ItemsStackPanel_GroupPadding = 1078,
                            XamlPropertyIndex_ItemsStackPanel_ItemsUpdatingScrollMode = 1079,
                            XamlPropertyIndex_ItemsStackPanel_Orientation = 1080,
                            XamlPropertyIndex_ItemsWrapGrid_CacheLength = 1081,
                            XamlPropertyIndex_ItemsWrapGrid_GroupHeaderPlacement = 1082,
                            XamlPropertyIndex_ItemsWrapGrid_GroupPadding = 1083,
                            XamlPropertyIndex_ItemsWrapGrid_ItemHeight = 1084,
                            XamlPropertyIndex_ItemsWrapGrid_ItemWidth = 1085,
                            XamlPropertyIndex_ItemsWrapGrid_MaximumRowsOrColumns = 1086,
                            XamlPropertyIndex_ItemsWrapGrid_Orientation = 1087,
                            XamlPropertyIndex_ListViewItemPresenter_CheckBrush = 1088,
                            XamlPropertyIndex_ListViewItemPresenter_CheckHintBrush = 1089,
                            XamlPropertyIndex_ListViewItemPresenter_CheckSelectingBrush = 1090,
                            XamlPropertyIndex_ListViewItemPresenter_ContentMargin = 1091,
                            XamlPropertyIndex_ListViewItemPresenter_DisabledOpacity = 1092,
                            XamlPropertyIndex_ListViewItemPresenter_DragBackground = 1093,
                            XamlPropertyIndex_ListViewItemPresenter_DragForeground = 1094,
                            XamlPropertyIndex_ListViewItemPresenter_DragOpacity = 1095,
                            XamlPropertyIndex_ListViewItemPresenter_FocusBorderBrush = 1096,
                            XamlPropertyIndex_ListViewItemPresenter_ListViewItemPresenterHorizontalContentAlignment = 1097,
                            XamlPropertyIndex_ListViewItemPresenter_ListViewItemPresenterPadding = 1098,
                            XamlPropertyIndex_ListViewItemPresenter_PlaceholderBackground = 1099,
                            XamlPropertyIndex_ListViewItemPresenter_PointerOverBackground = 1100,
                            XamlPropertyIndex_ListViewItemPresenter_PointerOverBackgroundMargin = 1101,
                            XamlPropertyIndex_ListViewItemPresenter_ReorderHintOffset = 1102,
                            XamlPropertyIndex_ListViewItemPresenter_SelectedBackground = 1103,
                            XamlPropertyIndex_ListViewItemPresenter_SelectedBorderThickness = 1104,
                            XamlPropertyIndex_ListViewItemPresenter_SelectedForeground = 1105,
                            XamlPropertyIndex_ListViewItemPresenter_SelectedPointerOverBackground = 1106,
                            XamlPropertyIndex_ListViewItemPresenter_SelectedPointerOverBorderBrush = 1107,
                            XamlPropertyIndex_ListViewItemPresenter_SelectionCheckMarkVisualEnabled = 1108,
                            XamlPropertyIndex_ListViewItemPresenter_ListViewItemPresenterVerticalContentAlignment = 1109,
                            XamlPropertyIndex_MenuFlyoutItem_Command = 1110,
                            XamlPropertyIndex_MenuFlyoutItem_CommandParameter = 1111,
                            XamlPropertyIndex_MenuFlyoutItem_Text = 1112,
                            XamlPropertyIndex_Page_BottomAppBar = 1114,
                            XamlPropertyIndex_Page_Frame = 1115,
                            XamlPropertyIndex_Page_NavigationCacheMode = 1116,
                            XamlPropertyIndex_Page_TopAppBar = 1117,
                            XamlPropertyIndex_ProgressBar_IsIndeterminate = 1118,
                            XamlPropertyIndex_ProgressBar_ShowError = 1119,
                            XamlPropertyIndex_ProgressBar_ShowPaused = 1120,
                            XamlPropertyIndex_ProgressBar_TemplateSettings = 1121,
                            XamlPropertyIndex_ScrollBar_IndicatorMode = 1122,
                            XamlPropertyIndex_ScrollBar_Orientation = 1123,
                            XamlPropertyIndex_ScrollBar_ViewportSize = 1124,
                            XamlPropertyIndex_Selector_IsSynchronizedWithCurrentItem = 1126,
                            XamlPropertyIndex_Selector_SelectedIndex = 1127,
                            XamlPropertyIndex_Selector_SelectedItem = 1128,
                            XamlPropertyIndex_Selector_SelectedValue = 1129,
                            XamlPropertyIndex_Selector_SelectedValuePath = 1130,
                            XamlPropertyIndex_SelectorItem_IsSelected = 1131,
                            XamlPropertyIndex_SettingsFlyout_HeaderBackground = 1132,
                            XamlPropertyIndex_SettingsFlyout_HeaderForeground = 1133,
                            XamlPropertyIndex_SettingsFlyout_IconSource = 1134,
                            XamlPropertyIndex_SettingsFlyout_TemplateSettings = 1135,
                            XamlPropertyIndex_SettingsFlyout_Title = 1136,
                            XamlPropertyIndex_Slider_Header = 1137,
                            XamlPropertyIndex_Slider_HeaderTemplate = 1138,
                            XamlPropertyIndex_Slider_IntermediateValue = 1139,
                            XamlPropertyIndex_Slider_IsDirectionReversed = 1140,
                            XamlPropertyIndex_Slider_IsThumbToolTipEnabled = 1141,
                            XamlPropertyIndex_Slider_Orientation = 1142,
                            XamlPropertyIndex_Slider_SnapsTo = 1143,
                            XamlPropertyIndex_Slider_StepFrequency = 1144,
                            XamlPropertyIndex_Slider_ThumbToolTipValueConverter = 1145,
                            XamlPropertyIndex_Slider_TickFrequency = 1146,
                            XamlPropertyIndex_Slider_TickPlacement = 1147,
                            XamlPropertyIndex_SwapChainPanel_CompositionScaleX = 1148,
                            XamlPropertyIndex_SwapChainPanel_CompositionScaleY = 1149,
                            XamlPropertyIndex_ToolTip_HorizontalOffset = 1150,
                            XamlPropertyIndex_ToolTip_IsOpen = 1151,
                            XamlPropertyIndex_ToolTip_Placement = 1152,
                            XamlPropertyIndex_ToolTip_PlacementTarget = 1153,
                            XamlPropertyIndex_ToolTip_TemplateSettings = 1154,
                            XamlPropertyIndex_ToolTip_VerticalOffset = 1155,
                            XamlPropertyIndex_Button_Flyout = 1156,
                            XamlPropertyIndex_ComboBox_Header = 1157,
                            XamlPropertyIndex_ComboBox_HeaderTemplate = 1158,
                            XamlPropertyIndex_ComboBox_IsDropDownOpen = 1159,
                            XamlPropertyIndex_ComboBox_IsEditable = 1160,
                            XamlPropertyIndex_ComboBox_IsSelectionBoxHighlighted = 1161,
                            XamlPropertyIndex_ComboBox_MaxDropDownHeight = 1162,
                            XamlPropertyIndex_ComboBox_PlaceholderText = 1163,
                            XamlPropertyIndex_ComboBox_SelectionBoxItem = 1164,
                            XamlPropertyIndex_ComboBox_SelectionBoxItemTemplate = 1165,
                            XamlPropertyIndex_ComboBox_TemplateSettings = 1166,
                            XamlPropertyIndex_CommandBar_PrimaryCommands = 1167,
                            XamlPropertyIndex_CommandBar_SecondaryCommands = 1168,
                            XamlPropertyIndex_FlipView_UseTouchAnimationsForAllNavigation = 1169,
                            XamlPropertyIndex_HyperlinkButton_NavigateUri = 1170,
                            XamlPropertyIndex_ListBox_SelectedItems = 1171,
                            XamlPropertyIndex_ListBox_SelectionMode = 1172,
                            XamlPropertyIndex_ListViewBase_CanDragItems = 1173,
                            XamlPropertyIndex_ListViewBase_CanReorderItems = 1174,
                            XamlPropertyIndex_ListViewBase_DataFetchSize = 1175,
                            XamlPropertyIndex_ListViewBase_Footer = 1176,
                            XamlPropertyIndex_ListViewBase_FooterTemplate = 1177,
                            XamlPropertyIndex_ListViewBase_FooterTransitions = 1178,
                            XamlPropertyIndex_ListViewBase_Header = 1179,
                            XamlPropertyIndex_ListViewBase_HeaderTemplate = 1180,
                            XamlPropertyIndex_ListViewBase_HeaderTransitions = 1181,
                            XamlPropertyIndex_ListViewBase_IncrementalLoadingThreshold = 1182,
                            XamlPropertyIndex_ListViewBase_IncrementalLoadingTrigger = 1183,
                            XamlPropertyIndex_ListViewBase_IsActiveView = 1184,
                            XamlPropertyIndex_ListViewBase_IsItemClickEnabled = 1185,
                            XamlPropertyIndex_ListViewBase_IsSwipeEnabled = 1186,
                            XamlPropertyIndex_ListViewBase_IsZoomedInView = 1187,
                            XamlPropertyIndex_ListViewBase_ReorderMode = 1188,
                            XamlPropertyIndex_ListViewBase_SelectedItems = 1189,
                            XamlPropertyIndex_ListViewBase_SelectionMode = 1190,
                            XamlPropertyIndex_ListViewBase_SemanticZoomOwner = 1191,
                            XamlPropertyIndex_ListViewBase_ShowsScrollingPlaceholders = 1192,
                            XamlPropertyIndex_RepeatButton_Delay = 1193,
                            XamlPropertyIndex_RepeatButton_Interval = 1194,
                            XamlPropertyIndex_ScrollViewer_BringIntoViewOnFocusChange = 1195,
                            XamlPropertyIndex_ScrollViewer_ComputedHorizontalScrollBarVisibility = 1196,
                            XamlPropertyIndex_ScrollViewer_ComputedVerticalScrollBarVisibility = 1197,
                            XamlPropertyIndex_ScrollViewer_ExtentHeight = 1198,
                            XamlPropertyIndex_ScrollViewer_ExtentWidth = 1199,
                            XamlPropertyIndex_ScrollViewer_HorizontalOffset = 1200,
                            XamlPropertyIndex_ScrollViewer_HorizontalScrollBarVisibility = 1201,
                            XamlPropertyIndex_ScrollViewer_HorizontalScrollMode = 1202,
                            XamlPropertyIndex_ScrollViewer_HorizontalSnapPointsAlignment = 1203,
                            XamlPropertyIndex_ScrollViewer_HorizontalSnapPointsType = 1204,
                            XamlPropertyIndex_ScrollViewer_IsDeferredScrollingEnabled = 1205,
                            XamlPropertyIndex_ScrollViewer_IsHorizontalRailEnabled = 1206,
                            XamlPropertyIndex_ScrollViewer_IsHorizontalScrollChainingEnabled = 1207,
                            XamlPropertyIndex_ScrollViewer_IsScrollInertiaEnabled = 1208,
                            XamlPropertyIndex_ScrollViewer_IsVerticalRailEnabled = 1209,
                            XamlPropertyIndex_ScrollViewer_IsVerticalScrollChainingEnabled = 1210,
                            XamlPropertyIndex_ScrollViewer_IsZoomChainingEnabled = 1211,
                            XamlPropertyIndex_ScrollViewer_IsZoomInertiaEnabled = 1212,
                            XamlPropertyIndex_ScrollViewer_LeftHeader = 1213,
                            XamlPropertyIndex_ScrollViewer_MaxZoomFactor = 1214,
                            XamlPropertyIndex_ScrollViewer_MinZoomFactor = 1215,
                            XamlPropertyIndex_ScrollViewer_ScrollableHeight = 1216,
                            XamlPropertyIndex_ScrollViewer_ScrollableWidth = 1217,
                            XamlPropertyIndex_ScrollViewer_TopHeader = 1218,
                            XamlPropertyIndex_ScrollViewer_TopLeftHeader = 1219,
                            XamlPropertyIndex_ScrollViewer_VerticalOffset = 1220,
                            XamlPropertyIndex_ScrollViewer_VerticalScrollBarVisibility = 1221,
                            XamlPropertyIndex_ScrollViewer_VerticalScrollMode = 1222,
                            XamlPropertyIndex_ScrollViewer_VerticalSnapPointsAlignment = 1223,
                            XamlPropertyIndex_ScrollViewer_VerticalSnapPointsType = 1224,
                            XamlPropertyIndex_ScrollViewer_ViewportHeight = 1225,
                            XamlPropertyIndex_ScrollViewer_ViewportWidth = 1226,
                            XamlPropertyIndex_ScrollViewer_ZoomFactor = 1227,
                            XamlPropertyIndex_ScrollViewer_ZoomMode = 1228,
                            XamlPropertyIndex_ScrollViewer_ZoomSnapPoints = 1229,
                            XamlPropertyIndex_ScrollViewer_ZoomSnapPointsType = 1230,
                            XamlPropertyIndex_ToggleButton_IsChecked = 1231,
                            XamlPropertyIndex_ToggleButton_IsThreeState = 1232,
                            XamlPropertyIndex_ToggleMenuFlyoutItem_IsChecked = 1233,
                            XamlPropertyIndex_VirtualizingStackPanel_AreScrollSnapPointsRegular = 1234,
                            XamlPropertyIndex_VirtualizingStackPanel_IsVirtualizing = 1236,
                            XamlPropertyIndex_VirtualizingStackPanel_Orientation = 1237,
                            XamlPropertyIndex_VirtualizingStackPanel_VirtualizationMode = 1238,
                            XamlPropertyIndex_WrapGrid_HorizontalChildrenAlignment = 1239,
                            XamlPropertyIndex_WrapGrid_ItemHeight = 1240,
                            XamlPropertyIndex_WrapGrid_ItemWidth = 1241,
                            XamlPropertyIndex_WrapGrid_MaximumRowsOrColumns = 1242,
                            XamlPropertyIndex_WrapGrid_Orientation = 1243,
                            XamlPropertyIndex_WrapGrid_VerticalChildrenAlignment = 1244,
                            XamlPropertyIndex_AppBarButton_Icon = 1245,
                            XamlPropertyIndex_AppBarButton_IsCompact = 1246,
                            XamlPropertyIndex_AppBarButton_Label = 1247,
                            XamlPropertyIndex_AppBarToggleButton_Icon = 1248,
                            XamlPropertyIndex_AppBarToggleButton_IsCompact = 1249,
                            XamlPropertyIndex_AppBarToggleButton_Label = 1250,
                            XamlPropertyIndex_GridViewItem_TemplateSettings = 1251,
                            XamlPropertyIndex_ListViewItem_TemplateSettings = 1252,
                            XamlPropertyIndex_RadioButton_GroupName = 1253,
                            XamlPropertyIndex_Glyphs_ColorFontPaletteIndex = 1267,
                            XamlPropertyIndex_Glyphs_IsColorFontEnabled = 1268,
                            XamlPropertyIndex_CalendarViewTemplateSettings_HasMoreContentAfter = 1274,
                            XamlPropertyIndex_CalendarViewTemplateSettings_HasMoreContentBefore = 1275,
                            XamlPropertyIndex_CalendarViewTemplateSettings_HasMoreViews = 1276,
                            XamlPropertyIndex_CalendarViewTemplateSettings_HeaderText = 1277,
                            XamlPropertyIndex_CalendarViewTemplateSettings_WeekDay1 = 1280,
                            XamlPropertyIndex_CalendarViewTemplateSettings_WeekDay2 = 1281,
                            XamlPropertyIndex_CalendarViewTemplateSettings_WeekDay3 = 1282,
                            XamlPropertyIndex_CalendarViewTemplateSettings_WeekDay4 = 1283,
                            XamlPropertyIndex_CalendarViewTemplateSettings_WeekDay5 = 1284,
                            XamlPropertyIndex_CalendarViewTemplateSettings_WeekDay6 = 1285,
                            XamlPropertyIndex_CalendarViewTemplateSettings_WeekDay7 = 1286,
                            XamlPropertyIndex_CalendarView_CalendarIdentifier = 1291,
                            XamlPropertyIndex_CalendarView_DayOfWeekFormat = 1299,
                            XamlPropertyIndex_CalendarView_DisplayMode = 1302,
                            XamlPropertyIndex_CalendarView_FirstDayOfWeek = 1303,
                            XamlPropertyIndex_CalendarView_IsOutOfScopeEnabled = 1317,
                            XamlPropertyIndex_CalendarView_IsTodayHighlighted = 1318,
                            XamlPropertyIndex_CalendarView_MaxDate = 1320,
                            XamlPropertyIndex_CalendarView_MinDate = 1321,
                            XamlPropertyIndex_CalendarView_NumberOfWeeksInView = 1327,
                            XamlPropertyIndex_CalendarView_SelectedDates = 1333,
                            XamlPropertyIndex_CalendarView_SelectionMode = 1335,
                            XamlPropertyIndex_CalendarView_TemplateSettings = 1336,
                            XamlPropertyIndex_CalendarViewDayItem_Date = 1339,
                            XamlPropertyIndex_CalendarViewDayItem_IsBlackout = 1340,
                            XamlPropertyIndex_MediaTransportControls_IsFastForwardEnabled = 1382,
                            XamlPropertyIndex_MediaTransportControls_IsFastRewindEnabled = 1383,
                            XamlPropertyIndex_MediaTransportControls_IsFullWindowEnabled = 1384,
                            XamlPropertyIndex_MediaTransportControls_IsPlaybackRateEnabled = 1385,
                            XamlPropertyIndex_MediaTransportControls_IsSeekEnabled = 1386,
                            XamlPropertyIndex_MediaTransportControls_IsStopEnabled = 1387,
                            XamlPropertyIndex_MediaTransportControls_IsVolumeEnabled = 1388,
                            XamlPropertyIndex_MediaTransportControls_IsZoomEnabled = 1389,
                            XamlPropertyIndex_ContentPresenter_LineHeight = 1425,
                            XamlPropertyIndex_CalendarViewTemplateSettings_MinViewWidth = 1435,
                            XamlPropertyIndex_ListViewBase_SelectedRanges = 1459,
                            XamlPropertyIndex_SplitViewTemplateSettings_CompactPaneGridLength = 1462,
                            XamlPropertyIndex_SplitViewTemplateSettings_NegativeOpenPaneLength = 1463,
                            XamlPropertyIndex_SplitViewTemplateSettings_NegativeOpenPaneLengthMinusCompactLength = 1464,
                            XamlPropertyIndex_SplitViewTemplateSettings_OpenPaneGridLength = 1465,
                            XamlPropertyIndex_SplitViewTemplateSettings_OpenPaneLengthMinusCompactLength = 1466,
                            XamlPropertyIndex_SplitView_CompactPaneLength = 1467,
                            XamlPropertyIndex_SplitView_Content = 1468,
                            XamlPropertyIndex_SplitView_DisplayMode = 1469,
                            XamlPropertyIndex_SplitView_IsPaneOpen = 1470,
                            XamlPropertyIndex_SplitView_OpenPaneLength = 1471,
                            XamlPropertyIndex_SplitView_Pane = 1472,
                            XamlPropertyIndex_SplitView_PanePlacement = 1473,
                            XamlPropertyIndex_SplitView_TemplateSettings = 1474,
                            XamlPropertyIndex_UIElement_Transform3D = 1475,
                            XamlPropertyIndex_CompositeTransform3D_CenterX = 1476,
                            XamlPropertyIndex_CompositeTransform3D_CenterY = 1478,
                            XamlPropertyIndex_CompositeTransform3D_CenterZ = 1480,
                            XamlPropertyIndex_CompositeTransform3D_RotationX = 1482,
                            XamlPropertyIndex_CompositeTransform3D_RotationY = 1484,
                            XamlPropertyIndex_CompositeTransform3D_RotationZ = 1486,
                            XamlPropertyIndex_CompositeTransform3D_ScaleX = 1488,
                            XamlPropertyIndex_CompositeTransform3D_ScaleY = 1490,
                            XamlPropertyIndex_CompositeTransform3D_ScaleZ = 1492,
                            XamlPropertyIndex_CompositeTransform3D_TranslateX = 1494,
                            XamlPropertyIndex_CompositeTransform3D_TranslateY = 1496,
                            XamlPropertyIndex_CompositeTransform3D_TranslateZ = 1498,
                            XamlPropertyIndex_PerspectiveTransform3D_Depth = 1500,
                            XamlPropertyIndex_PerspectiveTransform3D_OffsetX = 1501,
                            XamlPropertyIndex_PerspectiveTransform3D_OffsetY = 1502,
                            XamlPropertyIndex_RelativePanel_Above = 1508,
                            XamlPropertyIndex_RelativePanel_AlignBottomWith = 1509,
                            XamlPropertyIndex_RelativePanel_AlignLeftWith = 1510,
                            XamlPropertyIndex_RelativePanel_AlignRightWith = 1515,
                            XamlPropertyIndex_RelativePanel_AlignTopWith = 1516,
                            XamlPropertyIndex_RelativePanel_Below = 1517,
                            XamlPropertyIndex_RelativePanel_LeftOf = 1520,
                            XamlPropertyIndex_RelativePanel_RightOf = 1521,
                            XamlPropertyIndex_SplitViewTemplateSettings_OpenPaneLength = 1524,
                            XamlPropertyIndex_PasswordBox_PasswordRevealMode = 1527,
                            XamlPropertyIndex_SplitView_PaneBackground = 1528,
                            XamlPropertyIndex_ItemsStackPanel_AreStickyGroupHeadersEnabled = 1529,
                            XamlPropertyIndex_ItemsWrapGrid_AreStickyGroupHeadersEnabled = 1530,
                            XamlPropertyIndex_MenuFlyoutSubItem_Items = 1531,
                            XamlPropertyIndex_MenuFlyoutSubItem_Text = 1532,
                            XamlPropertyIndex_UIElement_CanDrag = 1534,
                            XamlPropertyIndex_DataTemplate_ExtensionInstance = 1535,
                            XamlPropertyIndex_RelativePanel_AlignHorizontalCenterWith = 1552,
                            XamlPropertyIndex_RelativePanel_AlignVerticalCenterWith = 1553,
                            XamlPropertyIndex_TargetPropertyPath_Path = 1555,
                            XamlPropertyIndex_TargetPropertyPath_Target = 1556,
                            XamlPropertyIndex_VisualState_Setters = 1558,
                            XamlPropertyIndex_VisualState_StateTriggers = 1559,
                            XamlPropertyIndex_AdaptiveTrigger_MinWindowHeight = 1560,
                            XamlPropertyIndex_AdaptiveTrigger_MinWindowWidth = 1561,
                            XamlPropertyIndex_Setter_Target = 1562,
                            XamlPropertyIndex_CalendarView_BlackoutForeground = 1565,
                            XamlPropertyIndex_CalendarView_CalendarItemBackground = 1566,
                            XamlPropertyIndex_CalendarView_CalendarItemBorderBrush = 1567,
                            XamlPropertyIndex_CalendarView_CalendarItemBorderThickness = 1568,
                            XamlPropertyIndex_CalendarView_CalendarItemForeground = 1569,
                            XamlPropertyIndex_CalendarView_CalendarViewDayItemStyle = 1570,
                            XamlPropertyIndex_CalendarView_DayItemFontFamily = 1571,
                            XamlPropertyIndex_CalendarView_DayItemFontSize = 1572,
                            XamlPropertyIndex_CalendarView_DayItemFontStyle = 1573,
                            XamlPropertyIndex_CalendarView_DayItemFontWeight = 1574,
                            XamlPropertyIndex_CalendarView_FirstOfMonthLabelFontFamily = 1575,
                            XamlPropertyIndex_CalendarView_FirstOfMonthLabelFontSize = 1576,
                            XamlPropertyIndex_CalendarView_FirstOfMonthLabelFontStyle = 1577,
                            XamlPropertyIndex_CalendarView_FirstOfMonthLabelFontWeight = 1578,
                            XamlPropertyIndex_CalendarView_FirstOfYearDecadeLabelFontFamily = 1579,
                            XamlPropertyIndex_CalendarView_FirstOfYearDecadeLabelFontSize = 1580,
                            XamlPropertyIndex_CalendarView_FirstOfYearDecadeLabelFontStyle = 1581,
                            XamlPropertyIndex_CalendarView_FirstOfYearDecadeLabelFontWeight = 1582,
                            XamlPropertyIndex_CalendarView_FocusBorderBrush = 1583,
                            XamlPropertyIndex_CalendarView_HorizontalDayItemAlignment = 1584,
                            XamlPropertyIndex_CalendarView_HorizontalFirstOfMonthLabelAlignment = 1585,
                            XamlPropertyIndex_CalendarView_HoverBorderBrush = 1586,
                            XamlPropertyIndex_CalendarView_MonthYearItemFontFamily = 1588,
                            XamlPropertyIndex_CalendarView_MonthYearItemFontSize = 1589,
                            XamlPropertyIndex_CalendarView_MonthYearItemFontStyle = 1590,
                            XamlPropertyIndex_CalendarView_MonthYearItemFontWeight = 1591,
                            XamlPropertyIndex_CalendarView_OutOfScopeBackground = 1592,
                            XamlPropertyIndex_CalendarView_OutOfScopeForeground = 1593,
                            XamlPropertyIndex_CalendarView_PressedBorderBrush = 1594,
                            XamlPropertyIndex_CalendarView_PressedForeground = 1595,
                            XamlPropertyIndex_CalendarView_SelectedBorderBrush = 1596,
                            XamlPropertyIndex_CalendarView_SelectedForeground = 1597,
                            XamlPropertyIndex_CalendarView_SelectedHoverBorderBrush = 1598,
                            XamlPropertyIndex_CalendarView_SelectedPressedBorderBrush = 1599,
                            XamlPropertyIndex_CalendarView_TodayFontWeight = 1600,
                            XamlPropertyIndex_CalendarView_TodayForeground = 1601,
                            XamlPropertyIndex_CalendarView_VerticalDayItemAlignment = 1602,
                            XamlPropertyIndex_CalendarView_VerticalFirstOfMonthLabelAlignment = 1603,
                            XamlPropertyIndex_MediaTransportControls_IsCompact = 1605,
                            XamlPropertyIndex_RelativePanel_AlignBottomWithPanel = 1606,
                            XamlPropertyIndex_RelativePanel_AlignHorizontalCenterWithPanel = 1607,
                            XamlPropertyIndex_RelativePanel_AlignLeftWithPanel = 1608,
                            XamlPropertyIndex_RelativePanel_AlignRightWithPanel = 1609,
                            XamlPropertyIndex_RelativePanel_AlignTopWithPanel = 1610,
                            XamlPropertyIndex_RelativePanel_AlignVerticalCenterWithPanel = 1611,
                            XamlPropertyIndex_ListViewBase_IsMultiSelectCheckBoxEnabled = 1612,
                            XamlPropertyIndex_AutomationProperties_Level = 1614,
                            XamlPropertyIndex_AutomationProperties_PositionInSet = 1615,
                            XamlPropertyIndex_AutomationProperties_SizeOfSet = 1616,
                            XamlPropertyIndex_ListViewItemPresenter_CheckBoxBrush = 1617,
                            XamlPropertyIndex_ListViewItemPresenter_CheckMode = 1618,
                            XamlPropertyIndex_ListViewItemPresenter_PressedBackground = 1620,
                            XamlPropertyIndex_ListViewItemPresenter_SelectedPressedBackground = 1621,
                            XamlPropertyIndex_Control_IsTemplateFocusTarget = 1623,
                            XamlPropertyIndex_Control_UseSystemFocusVisuals = 1624,
                            XamlPropertyIndex_ListViewItemPresenter_FocusSecondaryBorderBrush = 1628,
                            XamlPropertyIndex_ListViewItemPresenter_PointerOverForeground = 1630,
                            XamlPropertyIndex_FontIcon_MirroredWhenRightToLeft = 1631,
                            XamlPropertyIndex_CalendarViewTemplateSettings_CenterX = 1632,
                            XamlPropertyIndex_CalendarViewTemplateSettings_CenterY = 1633,
                            XamlPropertyIndex_CalendarViewTemplateSettings_ClipRect = 1634,
                            XamlPropertyIndex_PasswordBox_TextReadingOrder = 1650,
                            XamlPropertyIndex_RichEditBox_TextReadingOrder = 1651,
                            XamlPropertyIndex_TextBox_TextReadingOrder = 1652,
                            XamlPropertyIndex_WebView_ExecutionMode = 1653,
                            XamlPropertyIndex_WebView_DeferredPermissionRequests = 1655,
                            XamlPropertyIndex_WebView_Settings = 1656,
                            XamlPropertyIndex_RichEditBox_DesiredCandidateWindowAlignment = 1660,
                            XamlPropertyIndex_TextBox_DesiredCandidateWindowAlignment = 1662,
                            XamlPropertyIndex_CalendarDatePicker_CalendarIdentifier = 1663,
                            XamlPropertyIndex_CalendarDatePicker_CalendarViewStyle = 1664,
                            XamlPropertyIndex_CalendarDatePicker_Date = 1665,
                            XamlPropertyIndex_CalendarDatePicker_DateFormat = 1666,
                            XamlPropertyIndex_CalendarDatePicker_DayOfWeekFormat = 1667,
                            XamlPropertyIndex_CalendarDatePicker_DisplayMode = 1668,
                            XamlPropertyIndex_CalendarDatePicker_FirstDayOfWeek = 1669,
                            XamlPropertyIndex_CalendarDatePicker_Header = 1670,
                            XamlPropertyIndex_CalendarDatePicker_HeaderTemplate = 1671,
                            XamlPropertyIndex_CalendarDatePicker_IsCalendarOpen = 1672,
                            XamlPropertyIndex_CalendarDatePicker_IsGroupLabelVisible = 1673,
                            XamlPropertyIndex_CalendarDatePicker_IsOutOfScopeEnabled = 1674,
                            XamlPropertyIndex_CalendarDatePicker_IsTodayHighlighted = 1675,
                            XamlPropertyIndex_CalendarDatePicker_MaxDate = 1676,
                            XamlPropertyIndex_CalendarDatePicker_MinDate = 1677,
                            XamlPropertyIndex_CalendarDatePicker_PlaceholderText = 1678,
                            XamlPropertyIndex_CalendarView_IsGroupLabelVisible = 1679,
                            XamlPropertyIndex_ContentPresenter_Background = 1680,
                            XamlPropertyIndex_ContentPresenter_BorderBrush = 1681,
                            XamlPropertyIndex_ContentPresenter_BorderThickness = 1682,
                            XamlPropertyIndex_ContentPresenter_CornerRadius = 1683,
                            XamlPropertyIndex_ContentPresenter_Padding = 1684,
                            XamlPropertyIndex_Grid_BorderBrush = 1685,
                            XamlPropertyIndex_Grid_BorderThickness = 1686,
                            XamlPropertyIndex_Grid_CornerRadius = 1687,
                            XamlPropertyIndex_Grid_Padding = 1688,
                            XamlPropertyIndex_RelativePanel_BorderBrush = 1689,
                            XamlPropertyIndex_RelativePanel_BorderThickness = 1690,
                            XamlPropertyIndex_RelativePanel_CornerRadius = 1691,
                            XamlPropertyIndex_RelativePanel_Padding = 1692,
                            XamlPropertyIndex_StackPanel_BorderBrush = 1693,
                            XamlPropertyIndex_StackPanel_BorderThickness = 1694,
                            XamlPropertyIndex_StackPanel_CornerRadius = 1695,
                            XamlPropertyIndex_StackPanel_Padding = 1696,
                            XamlPropertyIndex_PasswordBox_InputScope = 1697,
                            XamlPropertyIndex_MediaTransportControlsHelper_DropoutOrder = 1698,
                            XamlPropertyIndex_AutoSuggestBoxQuerySubmittedEventArgs_ChosenSuggestion = 1699,
                            XamlPropertyIndex_AutoSuggestBoxQuerySubmittedEventArgs_QueryText = 1700,
                            XamlPropertyIndex_AutoSuggestBox_QueryIcon = 1701,
                            XamlPropertyIndex_StateTrigger_IsActive = 1702,
                            XamlPropertyIndex_ContentPresenter_HorizontalContentAlignment = 1703,
                            XamlPropertyIndex_ContentPresenter_VerticalContentAlignment = 1704,
                            XamlPropertyIndex_AppBarTemplateSettings_ClipRect = 1705,
                            XamlPropertyIndex_AppBarTemplateSettings_CompactRootMargin = 1706,
                            XamlPropertyIndex_AppBarTemplateSettings_CompactVerticalDelta = 1707,
                            XamlPropertyIndex_AppBarTemplateSettings_HiddenRootMargin = 1708,
                            XamlPropertyIndex_AppBarTemplateSettings_HiddenVerticalDelta = 1709,
                            XamlPropertyIndex_AppBarTemplateSettings_MinimalRootMargin = 1710,
                            XamlPropertyIndex_AppBarTemplateSettings_MinimalVerticalDelta = 1711,
                            XamlPropertyIndex_CommandBarTemplateSettings_ContentHeight = 1712,
                            XamlPropertyIndex_CommandBarTemplateSettings_NegativeOverflowContentHeight = 1713,
                            XamlPropertyIndex_CommandBarTemplateSettings_OverflowContentClipRect = 1714,
                            XamlPropertyIndex_CommandBarTemplateSettings_OverflowContentHeight = 1715,
                            XamlPropertyIndex_CommandBarTemplateSettings_OverflowContentHorizontalOffset = 1716,
                            XamlPropertyIndex_CommandBarTemplateSettings_OverflowContentMaxHeight = 1717,
                            XamlPropertyIndex_CommandBarTemplateSettings_OverflowContentMinWidth = 1718,
                            XamlPropertyIndex_AppBar_TemplateSettings = 1719,
                            XamlPropertyIndex_CommandBar_CommandBarOverflowPresenterStyle = 1720,
                            XamlPropertyIndex_CommandBar_CommandBarTemplateSettings = 1721,
                            XamlPropertyIndex_DrillInThemeAnimation_EntranceTarget = 1722,
                            XamlPropertyIndex_DrillInThemeAnimation_EntranceTargetName = 1723,
                            XamlPropertyIndex_DrillInThemeAnimation_ExitTarget = 1724,
                            XamlPropertyIndex_DrillInThemeAnimation_ExitTargetName = 1725,
                            XamlPropertyIndex_DrillOutThemeAnimation_EntranceTarget = 1726,
                            XamlPropertyIndex_DrillOutThemeAnimation_EntranceTargetName = 1727,
                            XamlPropertyIndex_DrillOutThemeAnimation_ExitTarget = 1728,
                            XamlPropertyIndex_DrillOutThemeAnimation_ExitTargetName = 1729,
                            XamlPropertyIndex_XamlBindingHelper_DataTemplateComponent = 1730,
                            XamlPropertyIndex_AutomationProperties_Annotations = 1732,
                            XamlPropertyIndex_AutomationAnnotation_Element = 1733,
                            XamlPropertyIndex_AutomationAnnotation_Type = 1734,
                            XamlPropertyIndex_AutomationPeerAnnotation_Peer = 1735,
                            XamlPropertyIndex_AutomationPeerAnnotation_Type = 1736,
                            XamlPropertyIndex_Hyperlink_UnderlineStyle = 1741,
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
                            XamlPropertyIndex_CalendarView_DisabledForeground = 1742,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
                            XamlPropertyIndex_CalendarView_TodayBackground = 1743,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
                            XamlPropertyIndex_CalendarView_TodayBlackoutBackground = 1744,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
                            XamlPropertyIndex_CalendarView_TodaySelectedInnerBorderBrush = 1747,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
                            XamlPropertyIndex_Control_IsFocusEngaged = 1749,
                            XamlPropertyIndex_Control_IsFocusEngagementEnabled = 1752,
                            XamlPropertyIndex_RichEditBox_ClipboardCopyFormat = 1754,
                            XamlPropertyIndex_CommandBarTemplateSettings_OverflowContentMaxWidth = 1757,
                            XamlPropertyIndex_ComboBoxTemplateSettings_DropDownContentMinWidth = 1758,
                            XamlPropertyIndex_MenuFlyoutPresenterTemplateSettings_FlyoutContentMinWidth = 1762,
                            XamlPropertyIndex_MenuFlyoutPresenter_TemplateSettings = 1763,
                            XamlPropertyIndex_AutomationProperties_LandmarkType = 1766,
                            XamlPropertyIndex_AutomationProperties_LocalizedLandmarkType = 1767,
                            XamlPropertyIndex_RepositionThemeTransition_IsStaggeringEnabled = 1769,
                            XamlPropertyIndex_ListBox_SingleSelectionFollowsFocus = 1770,
                            XamlPropertyIndex_ListViewBase_SingleSelectionFollowsFocus = 1771,
                            XamlPropertyIndex_BitmapImage_AutoPlay = 1773,
                            XamlPropertyIndex_BitmapImage_IsAnimatedBitmap = 1774,
                            XamlPropertyIndex_BitmapImage_IsPlaying = 1775,
                            XamlPropertyIndex_AutomationProperties_FullDescription = 1776,
                            XamlPropertyIndex_AutomationProperties_IsDataValidForForm = 1777,
                            XamlPropertyIndex_AutomationProperties_IsPeripheral = 1778,
                            XamlPropertyIndex_AutomationProperties_LocalizedControlType = 1779,
                            XamlPropertyIndex_FlyoutBase_AllowFocusOnInteraction = 1780,
                            XamlPropertyIndex_TextElement_AllowFocusOnInteraction = 1781,
                            XamlPropertyIndex_FrameworkElement_AllowFocusOnInteraction = 1782,
                            XamlPropertyIndex_Control_RequiresPointer = 1783,
                            XamlPropertyIndex_UIElement_ContextFlyout = 1785,
                            XamlPropertyIndex_TextElement_AccessKey = 1786,
                            XamlPropertyIndex_UIElement_AccessKeyScopeOwner = 1787,
                            XamlPropertyIndex_UIElement_IsAccessKeyScope = 1788,
                            XamlPropertyIndex_AutomationProperties_DescribedBy = 1790,
                            XamlPropertyIndex_UIElement_AccessKey = 1803,
                            XamlPropertyIndex_Control_XYFocusDown = 1804,
                            XamlPropertyIndex_Control_XYFocusLeft = 1805,
                            XamlPropertyIndex_Control_XYFocusRight = 1806,
                            XamlPropertyIndex_Control_XYFocusUp = 1807,
                            XamlPropertyIndex_Hyperlink_XYFocusDown = 1808,
                            XamlPropertyIndex_Hyperlink_XYFocusLeft = 1809,
                            XamlPropertyIndex_Hyperlink_XYFocusRight = 1810,
                            XamlPropertyIndex_Hyperlink_XYFocusUp = 1811,
                            XamlPropertyIndex_WebView_XYFocusDown = 1812,
                            XamlPropertyIndex_WebView_XYFocusLeft = 1813,
                            XamlPropertyIndex_WebView_XYFocusRight = 1814,
                            XamlPropertyIndex_WebView_XYFocusUp = 1815,
                            XamlPropertyIndex_CommandBarTemplateSettings_EffectiveOverflowButtonVisibility = 1816,
                            XamlPropertyIndex_AppBarSeparator_IsInOverflow = 1817,
                            XamlPropertyIndex_CommandBar_DefaultLabelPosition = 1818,
                            XamlPropertyIndex_CommandBar_IsDynamicOverflowEnabled = 1819,
                            XamlPropertyIndex_CommandBar_OverflowButtonVisibility = 1820,
                            XamlPropertyIndex_AppBarButton_IsInOverflow = 1821,
                            XamlPropertyIndex_AppBarButton_LabelPosition = 1822,
                            XamlPropertyIndex_AppBarToggleButton_IsInOverflow = 1823,
                            XamlPropertyIndex_AppBarToggleButton_LabelPosition = 1824,
                            XamlPropertyIndex_FlyoutBase_LightDismissOverlayMode = 1825,
                            XamlPropertyIndex_Popup_LightDismissOverlayMode = 1827,
                            XamlPropertyIndex_CalendarDatePicker_LightDismissOverlayMode = 1829,
                            XamlPropertyIndex_DatePicker_LightDismissOverlayMode = 1830,
                            XamlPropertyIndex_SplitView_LightDismissOverlayMode = 1831,
                            XamlPropertyIndex_TimePicker_LightDismissOverlayMode = 1832,
                            XamlPropertyIndex_AppBar_LightDismissOverlayMode = 1833,
                            XamlPropertyIndex_AutoSuggestBox_LightDismissOverlayMode = 1834,
                            XamlPropertyIndex_ComboBox_LightDismissOverlayMode = 1835,
                            XamlPropertyIndex_AppBarSeparator_DynamicOverflowOrder = 1836,
                            XamlPropertyIndex_AppBarButton_DynamicOverflowOrder = 1837,
                            XamlPropertyIndex_AppBarToggleButton_DynamicOverflowOrder = 1838,
                            XamlPropertyIndex_FrameworkElement_FocusVisualMargin = 1839,
                            XamlPropertyIndex_FrameworkElement_FocusVisualPrimaryBrush = 1840,
                            XamlPropertyIndex_FrameworkElement_FocusVisualPrimaryThickness = 1841,
                            XamlPropertyIndex_FrameworkElement_FocusVisualSecondaryBrush = 1842,
                            XamlPropertyIndex_FrameworkElement_FocusVisualSecondaryThickness = 1843,
                            XamlPropertyIndex_FlyoutBase_AllowFocusWhenDisabled = 1846,
                            XamlPropertyIndex_FrameworkElement_AllowFocusWhenDisabled = 1847,
                            XamlPropertyIndex_ComboBox_IsTextSearchEnabled = 1848,
                            XamlPropertyIndex_TextElement_ExitDisplayModeOnAccessKeyInvoked = 1849,
                            XamlPropertyIndex_UIElement_ExitDisplayModeOnAccessKeyInvoked = 1850,
                            XamlPropertyIndex_MediaPlayerPresenter_IsFullWindow = 1851,
                            XamlPropertyIndex_MediaPlayerPresenter_MediaPlayer = 1852,
                            XamlPropertyIndex_MediaPlayerPresenter_Stretch = 1853,
                            XamlPropertyIndex_MediaPlayerElement_AreTransportControlsEnabled = 1854,
                            XamlPropertyIndex_MediaPlayerElement_AutoPlay = 1855,
                            XamlPropertyIndex_MediaPlayerElement_IsFullWindow = 1856,
                            XamlPropertyIndex_MediaPlayerElement_MediaPlayer = 1857,
                            XamlPropertyIndex_MediaPlayerElement_PosterSource = 1858,
                            XamlPropertyIndex_MediaPlayerElement_Source = 1859,
                            XamlPropertyIndex_MediaPlayerElement_Stretch = 1860,
                            XamlPropertyIndex_MediaPlayerElement_TransportControls = 1861,
                            XamlPropertyIndex_MediaTransportControls_FastPlayFallbackBehaviour = 1862,
                            XamlPropertyIndex_MediaTransportControls_IsNextTrackButtonVisible = 1863,
                            XamlPropertyIndex_MediaTransportControls_IsPreviousTrackButtonVisible = 1864,
                            XamlPropertyIndex_MediaTransportControls_IsSkipBackwardButtonVisible = 1865,
                            XamlPropertyIndex_MediaTransportControls_IsSkipBackwardEnabled = 1866,
                            XamlPropertyIndex_MediaTransportControls_IsSkipForwardButtonVisible = 1867,
                            XamlPropertyIndex_MediaTransportControls_IsSkipForwardEnabled = 1868,
                            XamlPropertyIndex_FlyoutBase_ElementSoundMode = 1869,
                            XamlPropertyIndex_Control_ElementSoundMode = 1870,
                            XamlPropertyIndex_Hyperlink_ElementSoundMode = 1871,
                            XamlPropertyIndex_AutomationProperties_FlowsFrom = 1876,
                            XamlPropertyIndex_AutomationProperties_FlowsTo = 1877,
                            XamlPropertyIndex_TextElement_TextDecorations = 1879,
                            XamlPropertyIndex_RichTextBlock_TextDecorations = 1881,
                            XamlPropertyIndex_Control_DefaultStyleResourceUri = 1882,
                            XamlPropertyIndex_ContentDialog_PrimaryButtonStyle = 1884,
                            XamlPropertyIndex_ContentDialog_SecondaryButtonStyle = 1885,
                            XamlPropertyIndex_TextElement_KeyTipHorizontalOffset = 1890,
                            XamlPropertyIndex_TextElement_KeyTipPlacementMode = 1891,
                            XamlPropertyIndex_TextElement_KeyTipVerticalOffset = 1892,
                            XamlPropertyIndex_UIElement_KeyTipHorizontalOffset = 1893,
                            XamlPropertyIndex_UIElement_KeyTipPlacementMode = 1894,
                            XamlPropertyIndex_UIElement_KeyTipVerticalOffset = 1895,
                            XamlPropertyIndex_FlyoutBase_OverlayInputPassThroughElement = 1896,
                            XamlPropertyIndex_UIElement_XYFocusKeyboardNavigation = 1897,
                            XamlPropertyIndex_AutomationProperties_Culture = 1898,
                            XamlPropertyIndex_UIElement_XYFocusDownNavigationStrategy = 1918,
                            XamlPropertyIndex_UIElement_XYFocusLeftNavigationStrategy = 1919,
                            XamlPropertyIndex_UIElement_XYFocusRightNavigationStrategy = 1920,
                            XamlPropertyIndex_UIElement_XYFocusUpNavigationStrategy = 1921,
                            XamlPropertyIndex_Hyperlink_XYFocusDownNavigationStrategy = 1922,
                            XamlPropertyIndex_Hyperlink_XYFocusLeftNavigationStrategy = 1923,
                            XamlPropertyIndex_Hyperlink_XYFocusRightNavigationStrategy = 1924,
                            XamlPropertyIndex_Hyperlink_XYFocusUpNavigationStrategy = 1925,
                            XamlPropertyIndex_TextElement_AccessKeyScopeOwner = 1926,
                            XamlPropertyIndex_TextElement_IsAccessKeyScope = 1927,
                            XamlPropertyIndex_Hyperlink_FocusState = 1934,
                            XamlPropertyIndex_ContentDialog_CloseButtonCommand = 1936,
                            XamlPropertyIndex_ContentDialog_CloseButtonCommandParameter = 1937,
                            XamlPropertyIndex_ContentDialog_CloseButtonStyle = 1938,
                            XamlPropertyIndex_ContentDialog_CloseButtonText = 1939,
                            XamlPropertyIndex_ContentDialog_DefaultButton = 1940,
                            XamlPropertyIndex_RichEditBox_SelectionHighlightColorWhenNotFocused = 1941,
                            XamlPropertyIndex_TextBox_SelectionHighlightColorWhenNotFocused = 1942,
                            XamlPropertyIndex_SvgImageSource_RasterizePixelHeight = 1948,
                            XamlPropertyIndex_SvgImageSource_RasterizePixelWidth = 1949,
                            XamlPropertyIndex_SvgImageSource_UriSource = 1950,
                            XamlPropertyIndex_LoadedImageSurface_DecodedPhysicalSize = 1955,
                            XamlPropertyIndex_LoadedImageSurface_DecodedSize = 1956,
                            XamlPropertyIndex_LoadedImageSurface_NaturalSize = 1957,
                            XamlPropertyIndex_ComboBox_SelectionChangedTrigger = 1958,
                            XamlPropertyIndex_XamlCompositionBrushBase_FallbackColor = 1960,
                            XamlPropertyIndex_UIElement_Lights = 1962,
                            XamlPropertyIndex_MenuFlyoutItem_Icon = 1963,
                            XamlPropertyIndex_MenuFlyoutSubItem_Icon = 1964,
                            XamlPropertyIndex_BitmapIcon_ShowAsMonochrome = 1965,
                            XamlPropertyIndex_UIElement_HighContrastAdjustment = 1967,
                            XamlPropertyIndex_RichEditBox_MaxLength = 1968,
                            XamlPropertyIndex_UIElement_TabFocusNavigation = 1969,
                            XamlPropertyIndex_Control_IsTemplateKeyTipTarget = 1970,
                            XamlPropertyIndex_Hyperlink_IsTabStop = 1972,
                            XamlPropertyIndex_Hyperlink_TabIndex = 1973,
                            XamlPropertyIndex_MediaTransportControls_IsRepeatButtonVisible = 1974,
                            XamlPropertyIndex_MediaTransportControls_IsRepeatEnabled = 1975,
                            XamlPropertyIndex_MediaTransportControls_ShowAndHideAutomatically = 1976,
                            XamlPropertyIndex_RichEditBox_DisabledFormattingAccelerators = 1977,
                            XamlPropertyIndex_RichEditBox_CharacterCasing = 1978,
                            XamlPropertyIndex_TextBox_CharacterCasing = 1979,
                            XamlPropertyIndex_RichTextBlock_IsTextTrimmed = 1980,
                            XamlPropertyIndex_RichTextBlockOverflow_IsTextTrimmed = 1981,
                            XamlPropertyIndex_TextBlock_IsTextTrimmed = 1982,
                            XamlPropertyIndex_TextHighlighter_Background = 1985,
                            XamlPropertyIndex_TextHighlighter_Foreground = 1986,
                            XamlPropertyIndex_TextHighlighter_Ranges = 1987,
                            XamlPropertyIndex_RichTextBlock_TextHighlighters = 1988,
                            XamlPropertyIndex_TextBlock_TextHighlighters = 1989,
                            XamlPropertyIndex_FrameworkElement_ActualTheme = 1992,
                            XamlPropertyIndex_Grid_ColumnSpacing = 1993,
                            XamlPropertyIndex_Grid_RowSpacing = 1994,
                            XamlPropertyIndex_StackPanel_Spacing = 1995,
                            XamlPropertyIndex_Block_HorizontalTextAlignment = 1996,
                            XamlPropertyIndex_RichTextBlock_HorizontalTextAlignment = 1997,
                            XamlPropertyIndex_TextBlock_HorizontalTextAlignment = 1998,
                            XamlPropertyIndex_RichEditBox_HorizontalTextAlignment = 1999,
                            XamlPropertyIndex_TextBox_HorizontalTextAlignment = 2000,
                            XamlPropertyIndex_TextBox_PlaceholderForeground = 2001,
                            XamlPropertyIndex_ComboBox_PlaceholderForeground = 2002,
                            XamlPropertyIndex_KeyboardAccelerator_IsEnabled = 2003,
                            XamlPropertyIndex_KeyboardAccelerator_Key = 2004,
                            XamlPropertyIndex_KeyboardAccelerator_Modifiers = 2005,
                            XamlPropertyIndex_KeyboardAccelerator_ScopeOwner = 2006,
                            XamlPropertyIndex_UIElement_KeyboardAccelerators = 2007,
                            XamlPropertyIndex_ListViewItemPresenter_RevealBackground = 2009,
                            XamlPropertyIndex_ListViewItemPresenter_RevealBackgroundShowsAboveContent = 2010,
                            XamlPropertyIndex_ListViewItemPresenter_RevealBorderBrush = 2011,
                            XamlPropertyIndex_ListViewItemPresenter_RevealBorderThickness = 2012,
                            XamlPropertyIndex_UIElement_KeyTipTarget = 2014,
                            XamlPropertyIndex_AppBarButtonTemplateSettings_KeyboardAcceleratorTextMinWidth = 2015,
                            XamlPropertyIndex_AppBarToggleButtonTemplateSettings_KeyboardAcceleratorTextMinWidth = 2016,
                            XamlPropertyIndex_MenuFlyoutItemTemplateSettings_KeyboardAcceleratorTextMinWidth = 2017,
                            XamlPropertyIndex_MenuFlyoutItem_TemplateSettings = 2019,
                            XamlPropertyIndex_AppBarButton_TemplateSettings = 2021,
                            XamlPropertyIndex_AppBarToggleButton_TemplateSettings = 2023,
                            XamlPropertyIndex_UIElement_KeyboardAcceleratorPlacementMode = 2028,
                            XamlPropertyIndex_MediaTransportControls_IsCompactOverlayButtonVisible = 2032,
                            XamlPropertyIndex_MediaTransportControls_IsCompactOverlayEnabled = 2033,
                            XamlPropertyIndex_UIElement_KeyboardAcceleratorPlacementTarget = 2061,
                            XamlPropertyIndex_UIElement_CenterPoint = 2062,
                            XamlPropertyIndex_UIElement_Rotation = 2063,
                            XamlPropertyIndex_UIElement_RotationAxis = 2064,
                            XamlPropertyIndex_UIElement_Scale = 2065,
                            XamlPropertyIndex_UIElement_TransformMatrix = 2066,
                            XamlPropertyIndex_UIElement_Translation = 2067,
                            XamlPropertyIndex_TextBox_HandwritingView = 2068,
                            XamlPropertyIndex_AutomationProperties_HeadingLevel = 2069,
                            XamlPropertyIndex_TextBox_IsHandwritingViewEnabled = 2076,
                            XamlPropertyIndex_RichEditBox_ContentLinkProviders = 2078,
                            XamlPropertyIndex_RichEditBox_ContentLinkBackgroundColor = 2079,
                            XamlPropertyIndex_RichEditBox_ContentLinkForegroundColor = 2080,
                            XamlPropertyIndex_HandwritingView_AreCandidatesEnabled = 2081,
                            XamlPropertyIndex_HandwritingView_IsOpen = 2082,
                            XamlPropertyIndex_HandwritingView_PlacementTarget = 2084,
                            XamlPropertyIndex_HandwritingView_PlacementAlignment = 2085,
                            XamlPropertyIndex_RichEditBox_HandwritingView = 2086,
                            XamlPropertyIndex_RichEditBox_IsHandwritingViewEnabled = 2087,
                            XamlPropertyIndex_MenuFlyoutItem_KeyboardAcceleratorTextOverride = 2090,
                            XamlPropertyIndex_AppBarButton_KeyboardAcceleratorTextOverride = 2091,
                            XamlPropertyIndex_AppBarToggleButton_KeyboardAcceleratorTextOverride = 2092,
                            XamlPropertyIndex_ContentLink_Background = 2093,
                            XamlPropertyIndex_ContentLink_Cursor = 2094,
                            XamlPropertyIndex_ContentLink_ElementSoundMode = 2095,
                            XamlPropertyIndex_ContentLink_FocusState = 2096,
                            XamlPropertyIndex_ContentLink_IsTabStop = 2097,
                            XamlPropertyIndex_ContentLink_TabIndex = 2098,
                            XamlPropertyIndex_ContentLink_XYFocusDown = 2099,
                            XamlPropertyIndex_ContentLink_XYFocusDownNavigationStrategy = 2100,
                            XamlPropertyIndex_ContentLink_XYFocusLeft = 2101,
                            XamlPropertyIndex_ContentLink_XYFocusLeftNavigationStrategy = 2102,
                            XamlPropertyIndex_ContentLink_XYFocusRight = 2103,
                            XamlPropertyIndex_ContentLink_XYFocusRightNavigationStrategy = 2104,
                            XamlPropertyIndex_ContentLink_XYFocusUp = 2105,
                            XamlPropertyIndex_ContentLink_XYFocusUpNavigationStrategy = 2106,
                            XamlPropertyIndex_IconSource_Foreground = 2112,
                            XamlPropertyIndex_BitmapIconSource_ShowAsMonochrome = 2113,
                            XamlPropertyIndex_BitmapIconSource_UriSource = 2114,
                            XamlPropertyIndex_FontIconSource_FontFamily = 2115,
                            XamlPropertyIndex_FontIconSource_FontSize = 2116,
                            XamlPropertyIndex_FontIconSource_FontStyle = 2117,
                            XamlPropertyIndex_FontIconSource_FontWeight = 2118,
                            XamlPropertyIndex_FontIconSource_Glyph = 2119,
                            XamlPropertyIndex_FontIconSource_IsTextScaleFactorEnabled = 2120,
                            XamlPropertyIndex_FontIconSource_MirroredWhenRightToLeft = 2121,
                            XamlPropertyIndex_PathIconSource_Data = 2122,
                            XamlPropertyIndex_SymbolIconSource_Symbol = 2123,
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
                            XamlPropertyIndex_UIElement_Shadow = 2130,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
                            XamlPropertyIndex_IconSourceElement_IconSource = 2131,
                            XamlPropertyIndex_PasswordBox_CanPasteClipboardContent = 2137,
                            XamlPropertyIndex_TextBox_CanPasteClipboardContent = 2138,
                            XamlPropertyIndex_TextBox_CanRedo = 2139,
                            XamlPropertyIndex_TextBox_CanUndo = 2140,
                            XamlPropertyIndex_FlyoutBase_ShowMode = 2141,
                            XamlPropertyIndex_FlyoutBase_Target = 2142,
                            XamlPropertyIndex_Control_CornerRadius = 2143,
                            XamlPropertyIndex_AutomationProperties_IsDialog = 2149,
                            XamlPropertyIndex_AppBarElementContainer_DynamicOverflowOrder = 2150,
                            XamlPropertyIndex_AppBarElementContainer_IsCompact = 2151,
                            XamlPropertyIndex_AppBarElementContainer_IsInOverflow = 2152,
                            XamlPropertyIndex_ScrollContentPresenter_CanContentRenderOutsideBounds = 2157,
                            XamlPropertyIndex_ScrollViewer_CanContentRenderOutsideBounds = 2158,
                            XamlPropertyIndex_RichEditBox_SelectionFlyout = 2159,
                            XamlPropertyIndex_TextBox_SelectionFlyout = 2160,
                            XamlPropertyIndex_Border_BackgroundSizing = 2161,
                            XamlPropertyIndex_ContentPresenter_BackgroundSizing = 2162,
                            XamlPropertyIndex_Control_BackgroundSizing = 2163,
                            XamlPropertyIndex_Grid_BackgroundSizing = 2164,
                            XamlPropertyIndex_RelativePanel_BackgroundSizing = 2165,
                            XamlPropertyIndex_StackPanel_BackgroundSizing = 2166,
                            XamlPropertyIndex_ScrollViewer_HorizontalAnchorRatio = 2170,
                            XamlPropertyIndex_ScrollViewer_VerticalAnchorRatio = 2171,
                            XamlPropertyIndex_ComboBox_Text = 2208,
                            XamlPropertyIndex_TextBox_Description = 2217,
                            XamlPropertyIndex_ToolTip_PlacementRect = 2218,
                            XamlPropertyIndex_RichTextBlock_SelectionFlyout = 2219,
                            XamlPropertyIndex_TextBlock_SelectionFlyout = 2220,
                            XamlPropertyIndex_PasswordBox_SelectionFlyout = 2221,
                            XamlPropertyIndex_Border_BackgroundTransition = 2222,
                            XamlPropertyIndex_ContentPresenter_BackgroundTransition = 2223,
                            XamlPropertyIndex_Panel_BackgroundTransition = 2224,
                            XamlPropertyIndex_ColorPaletteResources_Accent = 2227,
                            XamlPropertyIndex_ColorPaletteResources_AltHigh = 2228,
                            XamlPropertyIndex_ColorPaletteResources_AltLow = 2229,
                            XamlPropertyIndex_ColorPaletteResources_AltMedium = 2230,
                            XamlPropertyIndex_ColorPaletteResources_AltMediumHigh = 2231,
                            XamlPropertyIndex_ColorPaletteResources_AltMediumLow = 2232,
                            XamlPropertyIndex_ColorPaletteResources_BaseHigh = 2233,
                            XamlPropertyIndex_ColorPaletteResources_BaseLow = 2234,
                            XamlPropertyIndex_ColorPaletteResources_BaseMedium = 2235,
                            XamlPropertyIndex_ColorPaletteResources_BaseMediumHigh = 2236,
                            XamlPropertyIndex_ColorPaletteResources_BaseMediumLow = 2237,
                            XamlPropertyIndex_ColorPaletteResources_ChromeAltLow = 2238,
                            XamlPropertyIndex_ColorPaletteResources_ChromeBlackHigh = 2239,
                            XamlPropertyIndex_ColorPaletteResources_ChromeBlackLow = 2240,
                            XamlPropertyIndex_ColorPaletteResources_ChromeBlackMedium = 2241,
                            XamlPropertyIndex_ColorPaletteResources_ChromeBlackMediumLow = 2242,
                            XamlPropertyIndex_ColorPaletteResources_ChromeDisabledHigh = 2243,
                            XamlPropertyIndex_ColorPaletteResources_ChromeDisabledLow = 2244,
                            XamlPropertyIndex_ColorPaletteResources_ChromeGray = 2245,
                            XamlPropertyIndex_ColorPaletteResources_ChromeHigh = 2246,
                            XamlPropertyIndex_ColorPaletteResources_ChromeLow = 2247,
                            XamlPropertyIndex_ColorPaletteResources_ChromeMedium = 2248,
                            XamlPropertyIndex_ColorPaletteResources_ChromeMediumLow = 2249,
                            XamlPropertyIndex_ColorPaletteResources_ChromeWhite = 2250,
                            XamlPropertyIndex_ColorPaletteResources_ErrorText = 2252,
                            XamlPropertyIndex_ColorPaletteResources_ListLow = 2253,
                            XamlPropertyIndex_ColorPaletteResources_ListMedium = 2254,
                            XamlPropertyIndex_UIElement_TranslationTransition = 2255,
                            XamlPropertyIndex_UIElement_OpacityTransition = 2256,
                            XamlPropertyIndex_UIElement_RotationTransition = 2257,
                            XamlPropertyIndex_UIElement_ScaleTransition = 2258,
                            XamlPropertyIndex_BrushTransition_Duration = 2261,
                            XamlPropertyIndex_ScalarTransition_Duration = 2262,
                            XamlPropertyIndex_Vector3Transition_Duration = 2263,
                            XamlPropertyIndex_Vector3Transition_Components = 2266,
                            XamlPropertyIndex_FlyoutBase_IsOpen = 2267,
                            XamlPropertyIndex_StandardUICommand_Kind = 2275,
                            XamlPropertyIndex_UIElement_CanBeScrollAnchor = 2276,
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
                            XamlPropertyIndex_ThemeShadow_Receivers = 2279,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
                            XamlPropertyIndex_ScrollContentPresenter_SizesContentToTemplatedParent = 2280,
                            XamlPropertyIndex_ComboBox_TextBoxStyle = 2281,
                            XamlPropertyIndex_Frame_IsNavigationStackEnabled = 2282,
                            XamlPropertyIndex_RichEditBox_ProofingMenuFlyout = 2283,
                            XamlPropertyIndex_TextBox_ProofingMenuFlyout = 2284,
                            XamlPropertyIndex_ScrollViewer_ReduceViewportForCoreInputViewOcclusions = 2295,
                            XamlPropertyIndex_FlyoutBase_AreOpenCloseAnimationsEnabled = 2296,
                            XamlPropertyIndex_FlyoutBase_InputDevicePrefersPrimaryCommands = 2297,
                            XamlPropertyIndex_CalendarDatePicker_Description = 2300,
                            XamlPropertyIndex_PasswordBox_Description = 2308,
                            XamlPropertyIndex_RichEditBox_Description = 2316,
                            XamlPropertyIndex_AutoSuggestBox_Description = 2331,
                            XamlPropertyIndex_ComboBox_Description = 2339,
                            XamlPropertyIndex_XamlUICommand_AccessKey = 2347,
                            XamlPropertyIndex_XamlUICommand_Command = 2348,
                            XamlPropertyIndex_XamlUICommand_Description = 2349,
                            XamlPropertyIndex_XamlUICommand_IconSource = 2350,
                            XamlPropertyIndex_XamlUICommand_KeyboardAccelerators = 2351,
                            XamlPropertyIndex_XamlUICommand_Label = 2352,
                            XamlPropertyIndex_DatePicker_SelectedDate = 2355,
                            XamlPropertyIndex_TimePicker_SelectedTime = 2356,
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
                            XamlPropertyIndex_AppBarTemplateSettings_NegativeCompactVerticalDelta = 2367,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
                            XamlPropertyIndex_AppBarTemplateSettings_NegativeHiddenVerticalDelta = 2368,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
                            XamlPropertyIndex_AppBarTemplateSettings_NegativeMinimalVerticalDelta = 2369,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
                            XamlPropertyIndex_FlyoutBase_ShouldConstrainToRootBounds = 2378,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
                            XamlPropertyIndex_Popup_ShouldConstrainToRootBounds = 2379,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
                            XamlPropertyIndex_FlyoutPresenter_IsDefaultShadowEnabled = 2380,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
                            XamlPropertyIndex_MenuFlyoutPresenter_IsDefaultShadowEnabled = 2381,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
                            XamlPropertyIndex_UIElement_ActualOffset = 2382,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
                            XamlPropertyIndex_UIElement_ActualSize = 2383,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
                            XamlPropertyIndex_CommandBarTemplateSettings_OverflowContentCompactYTranslation = 2384,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
                            XamlPropertyIndex_CommandBarTemplateSettings_OverflowContentHiddenYTranslation = 2385,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
                            XamlPropertyIndex_CommandBarTemplateSettings_OverflowContentMinimalYTranslation = 2386,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x30000
                            XamlPropertyIndex_HandwritingView_IsCommandBarOpen = 2395,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x30000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x30000
                            XamlPropertyIndex_HandwritingView_IsSwitchToKeyboardEnabled = 2396,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x30000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
                            XamlPropertyIndex_ListViewItemPresenter_SelectionIndicatorVisualEnabled = 2399,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
                            XamlPropertyIndex_ListViewItemPresenter_SelectionIndicatorBrush = 2400,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
                            XamlPropertyIndex_ListViewItemPresenter_SelectionIndicatorMode = 2401,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
                            XamlPropertyIndex_ListViewItemPresenter_SelectionIndicatorPointerOverBrush = 2402,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
                            XamlPropertyIndex_ListViewItemPresenter_SelectionIndicatorPressedBrush = 2403,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
                            XamlPropertyIndex_ListViewItemPresenter_SelectedBorderBrush = 2410,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
                            XamlPropertyIndex_ListViewItemPresenter_SelectedInnerBorderBrush = 2411,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
                            XamlPropertyIndex_ListViewItemPresenter_CheckBoxCornerRadius = 2412,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
                            XamlPropertyIndex_ListViewItemPresenter_SelectionIndicatorCornerRadius = 2413,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
                            XamlPropertyIndex_ListViewItemPresenter_SelectedDisabledBorderBrush = 2414,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
                            XamlPropertyIndex_ListViewItemPresenter_SelectedPressedBorderBrush = 2415,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
                            XamlPropertyIndex_ListViewItemPresenter_SelectedDisabledBackground = 2416,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
                            XamlPropertyIndex_ListViewItemPresenter_PointerOverBorderBrush = 2417,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
                            XamlPropertyIndex_ListViewItemPresenter_CheckBoxPointerOverBrush = 2418,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
                            XamlPropertyIndex_ListViewItemPresenter_CheckBoxPressedBrush = 2419,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
                            XamlPropertyIndex_ListViewItemPresenter_CheckDisabledBrush = 2420,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
                            XamlPropertyIndex_ListViewItemPresenter_CheckPressedBrush = 2421,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
                            XamlPropertyIndex_ListViewItemPresenter_CheckBoxBorderBrush = 2422,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
                            XamlPropertyIndex_ListViewItemPresenter_CheckBoxDisabledBorderBrush = 2423,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
                            XamlPropertyIndex_ListViewItemPresenter_CheckBoxPressedBorderBrush = 2424,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
                            XamlPropertyIndex_ListViewItemPresenter_CheckBoxDisabledBrush = 2425,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
                            XamlPropertyIndex_ListViewItemPresenter_CheckBoxSelectedBrush = 2426,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
                            XamlPropertyIndex_ListViewItemPresenter_CheckBoxSelectedDisabledBrush = 2427,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
                            XamlPropertyIndex_ListViewItemPresenter_CheckBoxSelectedPointerOverBrush = 2428,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
                            XamlPropertyIndex_ListViewItemPresenter_CheckBoxSelectedPressedBrush = 2429,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
                            XamlPropertyIndex_ListViewItemPresenter_CheckBoxPointerOverBorderBrush = 2430,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
                            XamlPropertyIndex_ListViewItemPresenter_SelectionIndicatorDisabledBrush = 2431,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
                            XamlPropertyIndex_CalendarView_BlackoutBackground = 2432,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
                            XamlPropertyIndex_CalendarView_BlackoutStrikethroughBrush = 2433,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
                            XamlPropertyIndex_CalendarView_CalendarItemCornerRadius = 2434,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
                            XamlPropertyIndex_CalendarView_CalendarItemDisabledBackground = 2435,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
                            XamlPropertyIndex_CalendarView_CalendarItemHoverBackground = 2436,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
                            XamlPropertyIndex_CalendarView_CalendarItemPressedBackground = 2437,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
                            XamlPropertyIndex_CalendarView_DayItemMargin = 2438,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
                            XamlPropertyIndex_CalendarView_FirstOfMonthLabelMargin = 2439,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
                            XamlPropertyIndex_CalendarView_FirstOfYearDecadeLabelMargin = 2440,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
                            XamlPropertyIndex_CalendarView_MonthYearItemMargin = 2441,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
                            XamlPropertyIndex_CalendarView_OutOfScopeHoverForeground = 2442,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
                            XamlPropertyIndex_CalendarView_OutOfScopePressedForeground = 2443,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
                            XamlPropertyIndex_CalendarView_SelectedDisabledBorderBrush = 2444,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
                            XamlPropertyIndex_CalendarView_SelectedDisabledForeground = 2445,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
                            XamlPropertyIndex_CalendarView_SelectedHoverForeground = 2446,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
                            XamlPropertyIndex_CalendarView_SelectedPressedForeground = 2447,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
                            XamlPropertyIndex_CalendarView_TodayBlackoutForeground = 2448,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
                            XamlPropertyIndex_CalendarView_TodayDisabledBackground = 2449,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
                            XamlPropertyIndex_CalendarView_TodayHoverBackground = 2450,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
                            XamlPropertyIndex_CalendarView_TodayPressedBackground = 2451,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
                            XamlPropertyIndex_Popup_ActualPlacement = 2452,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
                            XamlPropertyIndex_Popup_DesiredPlacement = 2453,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
                            XamlPropertyIndex_Popup_PlacementTarget = 2454,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
                            XamlPropertyIndex_AutomationProperties_AutomationControlType = 2455,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
                        };
                    } /* Direct */
                } /* Core */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Core.Direct.XamlTypeIndex
 *
 * Introduced to Windows.UI.Xaml.Core.Direct.XamlDirectContract in version 1.0
 *
 */
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Core {
                    namespace Direct {
                        enum XamlTypeIndex : int
                        {
                            XamlTypeIndex_AutoSuggestBoxSuggestionChosenEventArgs = 34,
                            XamlTypeIndex_AutoSuggestBoxTextChangedEventArgs = 35,
                            XamlTypeIndex_CollectionViewSource = 41,
                            XamlTypeIndex_ColumnDefinition = 44,
                            XamlTypeIndex_GradientStop = 64,
                            XamlTypeIndex_InputScope = 74,
                            XamlTypeIndex_InputScopeName = 75,
                            XamlTypeIndex_KeySpline = 78,
                            XamlTypeIndex_PathFigure = 93,
                            XamlTypeIndex_PrintDocument = 100,
                            XamlTypeIndex_RowDefinition = 106,
                            XamlTypeIndex_Style = 114,
                            XamlTypeIndex_TimelineMarker = 126,
                            XamlTypeIndex_VisualState = 137,
                            XamlTypeIndex_VisualStateGroup = 138,
                            XamlTypeIndex_VisualStateManager = 139,
                            XamlTypeIndex_VisualTransition = 140,
                            XamlTypeIndex_AddDeleteThemeTransition = 177,
                            XamlTypeIndex_ArcSegment = 178,
                            XamlTypeIndex_BackEase = 179,
                            XamlTypeIndex_BeginStoryboard = 180,
                            XamlTypeIndex_BezierSegment = 181,
                            XamlTypeIndex_BindingBase = 182,
                            XamlTypeIndex_BitmapCache = 183,
                            XamlTypeIndex_BounceEase = 186,
                            XamlTypeIndex_CircleEase = 187,
                            XamlTypeIndex_ColorAnimation = 188,
                            XamlTypeIndex_ColorAnimationUsingKeyFrames = 189,
                            XamlTypeIndex_ContentThemeTransition = 190,
                            XamlTypeIndex_ControlTemplate = 191,
                            XamlTypeIndex_CubicEase = 192,
                            XamlTypeIndex_DataTemplate = 194,
                            XamlTypeIndex_DiscreteColorKeyFrame = 195,
                            XamlTypeIndex_DiscreteDoubleKeyFrame = 196,
                            XamlTypeIndex_DiscreteObjectKeyFrame = 197,
                            XamlTypeIndex_DiscretePointKeyFrame = 198,
                            XamlTypeIndex_DoubleAnimation = 200,
                            XamlTypeIndex_DoubleAnimationUsingKeyFrames = 201,
                            XamlTypeIndex_EasingColorKeyFrame = 204,
                            XamlTypeIndex_EasingDoubleKeyFrame = 205,
                            XamlTypeIndex_EasingPointKeyFrame = 206,
                            XamlTypeIndex_EdgeUIThemeTransition = 207,
                            XamlTypeIndex_ElasticEase = 208,
                            XamlTypeIndex_EllipseGeometry = 209,
                            XamlTypeIndex_EntranceThemeTransition = 210,
                            XamlTypeIndex_EventTrigger = 211,
                            XamlTypeIndex_ExponentialEase = 212,
                            XamlTypeIndex_Flyout = 213,
                            XamlTypeIndex_GeometryGroup = 216,
                            XamlTypeIndex_ItemsPanelTemplate = 227,
                            XamlTypeIndex_LinearColorKeyFrame = 230,
                            XamlTypeIndex_LinearDoubleKeyFrame = 231,
                            XamlTypeIndex_LinearPointKeyFrame = 232,
                            XamlTypeIndex_LineGeometry = 233,
                            XamlTypeIndex_LineSegment = 234,
                            XamlTypeIndex_Matrix3DProjection = 236,
                            XamlTypeIndex_MenuFlyout = 238,
                            XamlTypeIndex_ObjectAnimationUsingKeyFrames = 240,
                            XamlTypeIndex_PaneThemeTransition = 241,
                            XamlTypeIndex_PathGeometry = 243,
                            XamlTypeIndex_PlaneProjection = 244,
                            XamlTypeIndex_PointAnimation = 245,
                            XamlTypeIndex_PointAnimationUsingKeyFrames = 246,
                            XamlTypeIndex_PolyBezierSegment = 248,
                            XamlTypeIndex_PolyLineSegment = 249,
                            XamlTypeIndex_PolyQuadraticBezierSegment = 250,
                            XamlTypeIndex_PopupThemeTransition = 251,
                            XamlTypeIndex_PowerEase = 252,
                            XamlTypeIndex_QuadraticBezierSegment = 254,
                            XamlTypeIndex_QuadraticEase = 255,
                            XamlTypeIndex_QuarticEase = 256,
                            XamlTypeIndex_QuinticEase = 257,
                            XamlTypeIndex_RectangleGeometry = 258,
                            XamlTypeIndex_RelativeSource = 259,
                            XamlTypeIndex_RenderTargetBitmap = 260,
                            XamlTypeIndex_ReorderThemeTransition = 261,
                            XamlTypeIndex_RepositionThemeTransition = 262,
                            XamlTypeIndex_Setter = 263,
                            XamlTypeIndex_SineEase = 264,
                            XamlTypeIndex_SolidColorBrush = 265,
                            XamlTypeIndex_SplineColorKeyFrame = 266,
                            XamlTypeIndex_SplineDoubleKeyFrame = 267,
                            XamlTypeIndex_SplinePointKeyFrame = 268,
                            XamlTypeIndex_BitmapImage = 285,
                            XamlTypeIndex_Border = 286,
                            XamlTypeIndex_CaptureElement = 288,
                            XamlTypeIndex_CompositeTransform = 295,
                            XamlTypeIndex_ContentPresenter = 296,
                            XamlTypeIndex_DragItemThemeAnimation = 302,
                            XamlTypeIndex_DragOverThemeAnimation = 303,
                            XamlTypeIndex_DropTargetItemThemeAnimation = 304,
                            XamlTypeIndex_FadeInThemeAnimation = 306,
                            XamlTypeIndex_FadeOutThemeAnimation = 307,
                            XamlTypeIndex_Glyphs = 312,
                            XamlTypeIndex_Image = 326,
                            XamlTypeIndex_ImageBrush = 328,
                            XamlTypeIndex_InlineUIContainer = 329,
                            XamlTypeIndex_ItemsPresenter = 332,
                            XamlTypeIndex_LinearGradientBrush = 334,
                            XamlTypeIndex_LineBreak = 335,
                            XamlTypeIndex_MatrixTransform = 340,
                            XamlTypeIndex_MediaElement = 342,
                            XamlTypeIndex_Paragraph = 349,
                            XamlTypeIndex_PointerDownThemeAnimation = 357,
                            XamlTypeIndex_PointerUpThemeAnimation = 359,
                            XamlTypeIndex_PopInThemeAnimation = 361,
                            XamlTypeIndex_PopOutThemeAnimation = 362,
                            XamlTypeIndex_Popup = 363,
                            XamlTypeIndex_RepositionThemeAnimation = 370,
                            XamlTypeIndex_ResourceDictionary = 371,
                            XamlTypeIndex_RichTextBlock = 374,
                            XamlTypeIndex_RichTextBlockOverflow = 376,
                            XamlTypeIndex_RotateTransform = 378,
                            XamlTypeIndex_Run = 380,
                            XamlTypeIndex_ScaleTransform = 381,
                            XamlTypeIndex_SkewTransform = 389,
                            XamlTypeIndex_Span = 390,
                            XamlTypeIndex_SplitCloseThemeAnimation = 391,
                            XamlTypeIndex_SplitOpenThemeAnimation = 392,
                            XamlTypeIndex_Storyboard = 393,
                            XamlTypeIndex_SwipeBackThemeAnimation = 394,
                            XamlTypeIndex_SwipeHintThemeAnimation = 395,
                            XamlTypeIndex_TextBlock = 396,
                            XamlTypeIndex_TransformGroup = 411,
                            XamlTypeIndex_TranslateTransform = 413,
                            XamlTypeIndex_Viewbox = 417,
                            XamlTypeIndex_WebViewBrush = 423,
                            XamlTypeIndex_AppBarSeparator = 427,
                            XamlTypeIndex_BitmapIcon = 429,
                            XamlTypeIndex_Bold = 430,
                            XamlTypeIndex_Canvas = 432,
                            XamlTypeIndex_ContentControl = 435,
                            XamlTypeIndex_DatePicker = 436,
                            XamlTypeIndex_DependencyObjectCollection = 437,
                            XamlTypeIndex_Ellipse = 438,
                            XamlTypeIndex_FontIcon = 440,
                            XamlTypeIndex_Grid = 442,
                            XamlTypeIndex_Hub = 445,
                            XamlTypeIndex_HubSection = 446,
                            XamlTypeIndex_Hyperlink = 447,
                            XamlTypeIndex_Italic = 449,
                            XamlTypeIndex_ItemsControl = 451,
                            XamlTypeIndex_Line = 452,
                            XamlTypeIndex_MediaTransportControls = 458,
                            XamlTypeIndex_PasswordBox = 462,
                            XamlTypeIndex_Path = 463,
                            XamlTypeIndex_PathIcon = 464,
                            XamlTypeIndex_Polygon = 465,
                            XamlTypeIndex_Polyline = 466,
                            XamlTypeIndex_ProgressRing = 468,
                            XamlTypeIndex_Rectangle = 470,
                            XamlTypeIndex_RichEditBox = 473,
                            XamlTypeIndex_ScrollContentPresenter = 476,
                            XamlTypeIndex_SearchBox = 477,
                            XamlTypeIndex_SemanticZoom = 479,
                            XamlTypeIndex_StackPanel = 481,
                            XamlTypeIndex_SymbolIcon = 482,
                            XamlTypeIndex_TextBox = 483,
                            XamlTypeIndex_Thumb = 485,
                            XamlTypeIndex_TickBar = 486,
                            XamlTypeIndex_TimePicker = 487,
                            XamlTypeIndex_ToggleSwitch = 489,
                            XamlTypeIndex_Underline = 490,
                            XamlTypeIndex_UserControl = 491,
                            XamlTypeIndex_VariableSizedWrapGrid = 492,
                            XamlTypeIndex_WebView = 494,
                            XamlTypeIndex_AppBar = 495,
                            XamlTypeIndex_AutoSuggestBox = 499,
                            XamlTypeIndex_CarouselPanel = 502,
                            XamlTypeIndex_ContentDialog = 506,
                            XamlTypeIndex_FlyoutPresenter = 508,
                            XamlTypeIndex_Frame = 509,
                            XamlTypeIndex_GridViewItemPresenter = 511,
                            XamlTypeIndex_GroupItem = 512,
                            XamlTypeIndex_ItemsStackPanel = 514,
                            XamlTypeIndex_ItemsWrapGrid = 515,
                            XamlTypeIndex_ListViewItemPresenter = 520,
                            XamlTypeIndex_MenuFlyoutItem = 521,
                            XamlTypeIndex_MenuFlyoutPresenter = 522,
                            XamlTypeIndex_MenuFlyoutSeparator = 523,
                            XamlTypeIndex_Page = 525,
                            XamlTypeIndex_ProgressBar = 528,
                            XamlTypeIndex_ScrollBar = 530,
                            XamlTypeIndex_SettingsFlyout = 533,
                            XamlTypeIndex_Slider = 534,
                            XamlTypeIndex_SwapChainBackgroundPanel = 535,
                            XamlTypeIndex_SwapChainPanel = 536,
                            XamlTypeIndex_ToolTip = 538,
                            XamlTypeIndex_Button = 540,
                            XamlTypeIndex_ComboBoxItem = 541,
                            XamlTypeIndex_CommandBar = 542,
                            XamlTypeIndex_FlipViewItem = 543,
                            XamlTypeIndex_GridViewHeaderItem = 545,
                            XamlTypeIndex_HyperlinkButton = 546,
                            XamlTypeIndex_ListBoxItem = 547,
                            XamlTypeIndex_ListViewHeaderItem = 550,
                            XamlTypeIndex_RepeatButton = 551,
                            XamlTypeIndex_ScrollViewer = 552,
                            XamlTypeIndex_ToggleButton = 553,
                            XamlTypeIndex_ToggleMenuFlyoutItem = 554,
                            XamlTypeIndex_VirtualizingStackPanel = 555,
                            XamlTypeIndex_WrapGrid = 556,
                            XamlTypeIndex_AppBarButton = 557,
                            XamlTypeIndex_AppBarToggleButton = 558,
                            XamlTypeIndex_CheckBox = 559,
                            XamlTypeIndex_GridViewItem = 560,
                            XamlTypeIndex_ListViewItem = 561,
                            XamlTypeIndex_RadioButton = 562,
                            XamlTypeIndex_Binding = 564,
                            XamlTypeIndex_ComboBox = 566,
                            XamlTypeIndex_FlipView = 567,
                            XamlTypeIndex_ListBox = 568,
                            XamlTypeIndex_GridView = 570,
                            XamlTypeIndex_ListView = 571,
                            XamlTypeIndex_CalendarView = 707,
                            XamlTypeIndex_CalendarViewDayItem = 709,
                            XamlTypeIndex_CalendarPanel = 723,
                            XamlTypeIndex_SplitView = 728,
                            XamlTypeIndex_CompositeTransform3D = 732,
                            XamlTypeIndex_PerspectiveTransform3D = 733,
                            XamlTypeIndex_RelativePanel = 744,
                            XamlTypeIndex_InkCanvas = 748,
                            XamlTypeIndex_MenuFlyoutSubItem = 749,
                            XamlTypeIndex_AdaptiveTrigger = 757,
                            XamlTypeIndex_SoftwareBitmapSource = 761,
                            XamlTypeIndex_StateTrigger = 767,
                            XamlTypeIndex_CalendarDatePicker = 774,
                            XamlTypeIndex_AutoSuggestBoxQuerySubmittedEventArgs = 778,
                            XamlTypeIndex_CommandBarOverflowPresenter = 781,
                            XamlTypeIndex_DrillInThemeAnimation = 782,
                            XamlTypeIndex_DrillOutThemeAnimation = 783,
                            XamlTypeIndex_AutomationAnnotation = 789,
                            XamlTypeIndex_AutomationPeerAnnotation = 790,
                            XamlTypeIndex_MediaPlayerPresenter = 828,
                            XamlTypeIndex_MediaPlayerElement = 829,
                            XamlTypeIndex_XamlLight = 855,
                            XamlTypeIndex_SvgImageSource = 860,
                            XamlTypeIndex_KeyboardAccelerator = 897,
                            XamlTypeIndex_HandwritingView = 920,
                            XamlTypeIndex_ContentLink = 925,
                            XamlTypeIndex_BitmapIconSource = 929,
                            XamlTypeIndex_FontIconSource = 930,
                            XamlTypeIndex_PathIconSource = 931,
                            XamlTypeIndex_SymbolIconSource = 933,
                            XamlTypeIndex_IconSourceElement = 939,
                            XamlTypeIndex_AppBarElementContainer = 945,
                            XamlTypeIndex_ColorPaletteResources = 952,
                            XamlTypeIndex_StandardUICommand = 961,
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
                            XamlTypeIndex_ThemeShadow = 964,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
                            XamlTypeIndex_XamlUICommand = 969,
                        };
                    } /* Direct */
                } /* Core */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Core.Direct.IXamlDirect
 *
 * Introduced to Windows.UI.Xaml.Core.Direct.XamlDirectContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Core.Direct.XamlDirect
 *
 */
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Core_Direct_IXamlDirect[] = L"Windows.UI.Xaml.Core.Direct.IXamlDirect";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Core {
                    namespace Direct {
                        MIDL_INTERFACE("5ffa1295-add2-590f-a051-70989b866ade")
                        IXamlDirect : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE GetObject(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                IInspectable** result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetXamlDirectObject(
                                IInspectable* object,
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject** result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                                ABI::Windows::UI::Xaml::Core::Direct::XamlTypeIndex typeIndex,
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject** result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SetObjectProperty(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                IInspectable* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SetXamlDirectObjectProperty(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SetBooleanProperty(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                boolean value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SetDoubleProperty(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                DOUBLE value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SetInt32Property(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                INT32 value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SetStringProperty(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                HSTRING value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SetDateTimeProperty(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                ABI::Windows::Foundation::DateTime value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SetPointProperty(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                ABI::Windows::Foundation::Point value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SetRectProperty(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                ABI::Windows::Foundation::Rect value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SetSizeProperty(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                ABI::Windows::Foundation::Size value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SetTimeSpanProperty(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                ABI::Windows::Foundation::TimeSpan value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SetColorProperty(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                ABI::Windows::UI::Color value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SetCornerRadiusProperty(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                ABI::Windows::UI::Xaml::CornerRadius value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SetDurationProperty(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                ABI::Windows::UI::Xaml::Duration value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SetGridLengthProperty(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                ABI::Windows::UI::Xaml::GridLength value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SetThicknessProperty(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                ABI::Windows::UI::Xaml::Thickness value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SetMatrixProperty(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                ABI::Windows::UI::Xaml::Media::Matrix value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SetMatrix3DProperty(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                ABI::Windows::UI::Xaml::Media::Media3D::Matrix3D value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SetEnumProperty(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                UINT32 value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetObjectProperty(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                IInspectable** result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetXamlDirectObjectProperty(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject** result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetBooleanProperty(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                boolean* result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetDoubleProperty(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                DOUBLE* result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetInt32Property(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                INT32* result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetStringProperty(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                HSTRING* result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetDateTimeProperty(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                ABI::Windows::Foundation::DateTime* result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetPointProperty(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                ABI::Windows::Foundation::Point* result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetRectProperty(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                ABI::Windows::Foundation::Rect* result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetSizeProperty(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                ABI::Windows::Foundation::Size* result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetTimeSpanProperty(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                ABI::Windows::Foundation::TimeSpan* result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetColorProperty(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                ABI::Windows::UI::Color* result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetCornerRadiusProperty(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                ABI::Windows::UI::Xaml::CornerRadius* result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetDurationProperty(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                ABI::Windows::UI::Xaml::Duration* result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetGridLengthProperty(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                ABI::Windows::UI::Xaml::GridLength* result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetThicknessProperty(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                ABI::Windows::UI::Xaml::Thickness* result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetMatrixProperty(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                ABI::Windows::UI::Xaml::Media::Matrix* result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetMatrix3DProperty(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                ABI::Windows::UI::Xaml::Media::Media3D::Matrix3D* result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetEnumProperty(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex,
                                UINT32* result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE ClearProperty(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlPropertyIndex propertyIndex
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetCollectionCount(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                UINT32* result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetXamlDirectObjectFromCollectionAt(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                UINT32 index,
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject** result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE AddToCollection(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE InsertIntoCollectionAt(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                UINT32 index,
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE RemoveFromCollection(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* value,
                                boolean* result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE RemoveFromCollectionAt(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                UINT32 index
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE ClearCollection(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE AddEventHandler(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlEventIndex eventIndex,
                                IInspectable* handler
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE AddEventHandler_HandledEventsToo(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlEventIndex eventIndex,
                                IInspectable* handler,
                                boolean handledEventsToo
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE RemoveEventHandler(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirectObject* xamlDirectObject,
                                ABI::Windows::UI::Xaml::Core::Direct::XamlEventIndex eventIndex,
                                IInspectable* handler
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IXamlDirect = __uuidof(IXamlDirect);
                    } /* Direct */
                } /* Core */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Core.Direct.IXamlDirectObject
 *
 * Introduced to Windows.UI.Xaml.Core.Direct.XamlDirectContract in version 1.0
 *
 */
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Core_Direct_IXamlDirectObject[] = L"Windows.UI.Xaml.Core.Direct.IXamlDirectObject";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Core {
                    namespace Direct {
                        MIDL_INTERFACE("10614a82-cee4-4645-ba25-d071ce778355")
                        IXamlDirectObject : public IInspectable
                        {
                        public:
                        };

                        MIDL_CONST_ID IID& IID_IXamlDirectObject = __uuidof(IXamlDirectObject);
                    } /* Direct */
                } /* Core */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Core.Direct.IXamlDirectStatics
 *
 * Introduced to Windows.UI.Xaml.Core.Direct.XamlDirectContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Core.Direct.XamlDirect
 *
 */
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Core_Direct_IXamlDirectStatics[] = L"Windows.UI.Xaml.Core.Direct.IXamlDirectStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Core {
                    namespace Direct {
                        MIDL_INTERFACE("321887cc-14e4-5c6f-878d-fbb604ad7d17")
                        IXamlDirectStatics : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE GetDefault(
                                ABI::Windows::UI::Xaml::Core::Direct::IXamlDirect** result
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IXamlDirectStatics = __uuidof(IXamlDirectStatics);
                    } /* Direct */
                } /* Core */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Core.Direct.XamlDirect
 *
 * Introduced to Windows.UI.Xaml.Core.Direct.XamlDirectContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Core.Direct.IXamlDirectStatics interface starting with version 1.0 of the Windows.UI.Xaml.Core.Direct.XamlDirectContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Core.Direct.IXamlDirect ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Core_Direct_XamlDirect_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Core_Direct_XamlDirect_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Core_Direct_XamlDirect[] = L"Windows.UI.Xaml.Core.Direct.XamlDirect";
#endif
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect;

#endif // ____x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject;

#endif // ____x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectStatics __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

typedef struct __x_ABI_CWindows_CFoundation_CPoint __x_ABI_CWindows_CFoundation_CPoint;

typedef struct __x_ABI_CWindows_CFoundation_CRect __x_ABI_CWindows_CFoundation_CRect;

typedef struct __x_ABI_CWindows_CFoundation_CSize __x_ABI_CWindows_CFoundation_CSize;

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

typedef struct __x_ABI_CWindows_CUI_CColor __x_ABI_CWindows_CUI_CColor;

typedef struct __x_ABI_CWindows_CUI_CXaml_CCornerRadius __x_ABI_CWindows_CUI_CXaml_CCornerRadius;

typedef struct __x_ABI_CWindows_CUI_CXaml_CDuration __x_ABI_CWindows_CUI_CXaml_CDuration;

typedef struct __x_ABI_CWindows_CUI_CXaml_CGridLength __x_ABI_CWindows_CUI_CXaml_CGridLength;

typedef struct __x_ABI_CWindows_CUI_CXaml_CMedia_CMatrix __x_ABI_CWindows_CUI_CXaml_CMedia_CMatrix;

typedef struct __x_ABI_CWindows_CUI_CXaml_CMedia_CMedia3D_CMatrix3D __x_ABI_CWindows_CUI_CXaml_CMedia_CMedia3D_CMatrix3D;

typedef struct __x_ABI_CWindows_CUI_CXaml_CThickness __x_ABI_CWindows_CUI_CXaml_CThickness;

typedef enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlEventIndex __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlEventIndex;

typedef enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex;

typedef enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlTypeIndex __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlTypeIndex;

/*
 *
 * Struct Windows.UI.Xaml.Core.Direct.XamlEventIndex
 *
 * Introduced to Windows.UI.Xaml.Core.Direct.XamlDirectContract in version 1.0
 *
 */
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlEventIndex
{
    XamlEventIndex_FrameworkElement_DataContextChanged = 16,
    XamlEventIndex_FrameworkElement_SizeChanged = 17,
    XamlEventIndex_FrameworkElement_LayoutUpdated = 18,
    XamlEventIndex_UIElement_KeyUp = 22,
    XamlEventIndex_UIElement_KeyDown = 23,
    XamlEventIndex_UIElement_GotFocus = 24,
    XamlEventIndex_UIElement_LostFocus = 25,
    XamlEventIndex_UIElement_DragStarting = 26,
    XamlEventIndex_UIElement_DropCompleted = 27,
    XamlEventIndex_UIElement_CharacterReceived = 28,
    XamlEventIndex_UIElement_DragEnter = 29,
    XamlEventIndex_UIElement_DragLeave = 30,
    XamlEventIndex_UIElement_DragOver = 31,
    XamlEventIndex_UIElement_Drop = 32,
    XamlEventIndex_UIElement_PointerPressed = 38,
    XamlEventIndex_UIElement_PointerMoved = 39,
    XamlEventIndex_UIElement_PointerReleased = 40,
    XamlEventIndex_UIElement_PointerEntered = 41,
    XamlEventIndex_UIElement_PointerExited = 42,
    XamlEventIndex_UIElement_PointerCaptureLost = 43,
    XamlEventIndex_UIElement_PointerCanceled = 44,
    XamlEventIndex_UIElement_PointerWheelChanged = 45,
    XamlEventIndex_UIElement_Tapped = 46,
    XamlEventIndex_UIElement_DoubleTapped = 47,
    XamlEventIndex_UIElement_Holding = 48,
    XamlEventIndex_UIElement_ContextRequested = 49,
    XamlEventIndex_UIElement_ContextCanceled = 50,
    XamlEventIndex_UIElement_RightTapped = 51,
    XamlEventIndex_UIElement_ManipulationStarting = 52,
    XamlEventIndex_UIElement_ManipulationInertiaStarting = 53,
    XamlEventIndex_UIElement_ManipulationStarted = 54,
    XamlEventIndex_UIElement_ManipulationDelta = 55,
    XamlEventIndex_UIElement_ManipulationCompleted = 56,
    XamlEventIndex_UIElement_ProcessKeyboardAccelerators = 60,
    XamlEventIndex_UIElement_GettingFocus = 61,
    XamlEventIndex_UIElement_LosingFocus = 62,
    XamlEventIndex_UIElement_NoFocusCandidateFound = 63,
    XamlEventIndex_UIElement_PreviewKeyDown = 64,
    XamlEventIndex_UIElement_PreviewKeyUp = 65,
    XamlEventIndex_UIElement_BringIntoViewRequested = 66,
    XamlEventIndex_AppBar_Opening = 109,
    XamlEventIndex_AppBar_Opened = 110,
    XamlEventIndex_AppBar_Closing = 111,
    XamlEventIndex_AppBar_Closed = 112,
    XamlEventIndex_AutoSuggestBox_SuggestionChosen = 113,
    XamlEventIndex_AutoSuggestBox_TextChanged = 114,
    XamlEventIndex_AutoSuggestBox_QuerySubmitted = 115,
    XamlEventIndex_CalendarDatePicker_CalendarViewDayItemChanging = 116,
    XamlEventIndex_CalendarDatePicker_DateChanged = 117,
    XamlEventIndex_CalendarDatePicker_Opened = 118,
    XamlEventIndex_CalendarDatePicker_Closed = 119,
    XamlEventIndex_CalendarView_CalendarViewDayItemChanging = 120,
    XamlEventIndex_CalendarView_SelectedDatesChanged = 121,
    XamlEventIndex_ComboBox_DropDownClosed = 122,
    XamlEventIndex_ComboBox_DropDownOpened = 123,
    XamlEventIndex_CommandBar_DynamicOverflowItemsChanging = 124,
    XamlEventIndex_ContentDialog_Closing = 126,
    XamlEventIndex_ContentDialog_Closed = 127,
    XamlEventIndex_ContentDialog_Opened = 128,
    XamlEventIndex_ContentDialog_PrimaryButtonClick = 129,
    XamlEventIndex_ContentDialog_SecondaryButtonClick = 130,
    XamlEventIndex_ContentDialog_CloseButtonClick = 131,
    XamlEventIndex_Control_FocusEngaged = 132,
    XamlEventIndex_Control_FocusDisengaged = 133,
    XamlEventIndex_DatePicker_DateChanged = 135,
    XamlEventIndex_Frame_Navigated = 136,
    XamlEventIndex_Frame_Navigating = 137,
    XamlEventIndex_Frame_NavigationFailed = 138,
    XamlEventIndex_Frame_NavigationStopped = 139,
    XamlEventIndex_Hub_SectionHeaderClick = 143,
    XamlEventIndex_Hub_SectionsInViewChanged = 144,
    XamlEventIndex_ItemsPresenter_HorizontalSnapPointsChanged = 148,
    XamlEventIndex_ItemsPresenter_VerticalSnapPointsChanged = 149,
    XamlEventIndex_ListViewBase_ItemClick = 150,
    XamlEventIndex_ListViewBase_DragItemsStarting = 151,
    XamlEventIndex_ListViewBase_DragItemsCompleted = 152,
    XamlEventIndex_ListViewBase_ContainerContentChanging = 153,
    XamlEventIndex_ListViewBase_ChoosingItemContainer = 154,
    XamlEventIndex_ListViewBase_ChoosingGroupHeaderContainer = 155,
    XamlEventIndex_MediaTransportControls_ThumbnailRequested = 167,
    XamlEventIndex_MenuFlyoutItem_Click = 168,
    XamlEventIndex_RichEditBox_TextChanging = 177,
    XamlEventIndex_ScrollViewer_ViewChanging = 192,
    XamlEventIndex_ScrollViewer_ViewChanged = 193,
    XamlEventIndex_ScrollViewer_DirectManipulationStarted = 194,
    XamlEventIndex_ScrollViewer_DirectManipulationCompleted = 195,
    XamlEventIndex_SearchBox_QueryChanged = 196,
    XamlEventIndex_SearchBox_SuggestionsRequested = 197,
    XamlEventIndex_SearchBox_QuerySubmitted = 198,
    XamlEventIndex_SearchBox_ResultSuggestionChosen = 199,
    XamlEventIndex_SearchBox_PrepareForFocusOnKeyboardInput = 200,
    XamlEventIndex_SemanticZoom_ViewChangeStarted = 201,
    XamlEventIndex_SemanticZoom_ViewChangeCompleted = 202,
    XamlEventIndex_SettingsFlyout_BackClick = 203,
    XamlEventIndex_StackPanel_HorizontalSnapPointsChanged = 208,
    XamlEventIndex_StackPanel_VerticalSnapPointsChanged = 209,
    XamlEventIndex_TimePicker_TimeChanged = 227,
    XamlEventIndex_ToggleSwitch_Toggled = 228,
    XamlEventIndex_ToolTip_Closed = 229,
    XamlEventIndex_ToolTip_Opened = 230,
    XamlEventIndex_VirtualizingStackPanel_CleanUpVirtualizedItemEvent = 231,
    XamlEventIndex_WebView_SeparateProcessLost = 232,
    XamlEventIndex_WebView_LoadCompleted = 233,
    XamlEventIndex_WebView_ScriptNotify = 234,
    XamlEventIndex_WebView_NavigationFailed = 235,
    XamlEventIndex_WebView_NavigationStarting = 236,
    XamlEventIndex_WebView_ContentLoading = 237,
    XamlEventIndex_WebView_DOMContentLoaded = 238,
    XamlEventIndex_WebView_NavigationCompleted = 239,
    XamlEventIndex_WebView_FrameNavigationStarting = 240,
    XamlEventIndex_WebView_FrameContentLoading = 241,
    XamlEventIndex_WebView_FrameDOMContentLoaded = 242,
    XamlEventIndex_WebView_FrameNavigationCompleted = 243,
    XamlEventIndex_WebView_LongRunningScriptDetected = 244,
    XamlEventIndex_WebView_UnsafeContentWarningDisplaying = 245,
    XamlEventIndex_WebView_UnviewableContentIdentified = 246,
    XamlEventIndex_WebView_ContainsFullScreenElementChanged = 247,
    XamlEventIndex_WebView_UnsupportedUriSchemeIdentified = 248,
    XamlEventIndex_WebView_NewWindowRequested = 249,
    XamlEventIndex_WebView_PermissionRequested = 250,
    XamlEventIndex_ButtonBase_Click = 256,
    XamlEventIndex_CarouselPanel_HorizontalSnapPointsChanged = 257,
    XamlEventIndex_CarouselPanel_VerticalSnapPointsChanged = 258,
    XamlEventIndex_OrientedVirtualizingPanel_HorizontalSnapPointsChanged = 263,
    XamlEventIndex_OrientedVirtualizingPanel_VerticalSnapPointsChanged = 264,
    XamlEventIndex_RangeBase_ValueChanged = 267,
    XamlEventIndex_ScrollBar_Scroll = 268,
    XamlEventIndex_Selector_SelectionChanged = 269,
    XamlEventIndex_Thumb_DragStarted = 270,
    XamlEventIndex_Thumb_DragDelta = 271,
    XamlEventIndex_Thumb_DragCompleted = 272,
    XamlEventIndex_ToggleButton_Checked = 273,
    XamlEventIndex_ToggleButton_Unchecked = 274,
    XamlEventIndex_ToggleButton_Indeterminate = 275,
    XamlEventIndex_WebView_WebResourceRequested = 283,
    XamlEventIndex_ScrollViewer_AnchorRequested = 291,
    XamlEventIndex_DatePicker_SelectedDateChanged = 322,
    XamlEventIndex_TimePicker_SelectedTimeChanged = 323,
};
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Core.Direct.XamlPropertyIndex
 *
 * Introduced to Windows.UI.Xaml.Core.Direct.XamlDirectContract in version 1.0
 *
 */
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex
{
    XamlPropertyIndex_AutomationProperties_AcceleratorKey = 5,
    XamlPropertyIndex_AutomationProperties_AccessibilityView = 6,
    XamlPropertyIndex_AutomationProperties_AccessKey = 7,
    XamlPropertyIndex_AutomationProperties_AutomationId = 8,
    XamlPropertyIndex_AutomationProperties_ControlledPeers = 9,
    XamlPropertyIndex_AutomationProperties_HelpText = 10,
    XamlPropertyIndex_AutomationProperties_IsRequiredForForm = 11,
    XamlPropertyIndex_AutomationProperties_ItemStatus = 12,
    XamlPropertyIndex_AutomationProperties_ItemType = 13,
    XamlPropertyIndex_AutomationProperties_LabeledBy = 14,
    XamlPropertyIndex_AutomationProperties_LiveSetting = 15,
    XamlPropertyIndex_AutomationProperties_Name = 16,
    XamlPropertyIndex_ToolTipService_Placement = 24,
    XamlPropertyIndex_ToolTipService_PlacementTarget = 25,
    XamlPropertyIndex_ToolTipService_ToolTip = 26,
    XamlPropertyIndex_Typography_AnnotationAlternates = 28,
    XamlPropertyIndex_Typography_Capitals = 29,
    XamlPropertyIndex_Typography_CapitalSpacing = 30,
    XamlPropertyIndex_Typography_CaseSensitiveForms = 31,
    XamlPropertyIndex_Typography_ContextualAlternates = 32,
    XamlPropertyIndex_Typography_ContextualLigatures = 33,
    XamlPropertyIndex_Typography_ContextualSwashes = 34,
    XamlPropertyIndex_Typography_DiscretionaryLigatures = 35,
    XamlPropertyIndex_Typography_EastAsianExpertForms = 36,
    XamlPropertyIndex_Typography_EastAsianLanguage = 37,
    XamlPropertyIndex_Typography_EastAsianWidths = 38,
    XamlPropertyIndex_Typography_Fraction = 39,
    XamlPropertyIndex_Typography_HistoricalForms = 40,
    XamlPropertyIndex_Typography_HistoricalLigatures = 41,
    XamlPropertyIndex_Typography_Kerning = 42,
    XamlPropertyIndex_Typography_MathematicalGreek = 43,
    XamlPropertyIndex_Typography_NumeralAlignment = 44,
    XamlPropertyIndex_Typography_NumeralStyle = 45,
    XamlPropertyIndex_Typography_SlashedZero = 46,
    XamlPropertyIndex_Typography_StandardLigatures = 47,
    XamlPropertyIndex_Typography_StandardSwashes = 48,
    XamlPropertyIndex_Typography_StylisticAlternates = 49,
    XamlPropertyIndex_Typography_StylisticSet1 = 50,
    XamlPropertyIndex_Typography_StylisticSet10 = 51,
    XamlPropertyIndex_Typography_StylisticSet11 = 52,
    XamlPropertyIndex_Typography_StylisticSet12 = 53,
    XamlPropertyIndex_Typography_StylisticSet13 = 54,
    XamlPropertyIndex_Typography_StylisticSet14 = 55,
    XamlPropertyIndex_Typography_StylisticSet15 = 56,
    XamlPropertyIndex_Typography_StylisticSet16 = 57,
    XamlPropertyIndex_Typography_StylisticSet17 = 58,
    XamlPropertyIndex_Typography_StylisticSet18 = 59,
    XamlPropertyIndex_Typography_StylisticSet19 = 60,
    XamlPropertyIndex_Typography_StylisticSet2 = 61,
    XamlPropertyIndex_Typography_StylisticSet20 = 62,
    XamlPropertyIndex_Typography_StylisticSet3 = 63,
    XamlPropertyIndex_Typography_StylisticSet4 = 64,
    XamlPropertyIndex_Typography_StylisticSet5 = 65,
    XamlPropertyIndex_Typography_StylisticSet6 = 66,
    XamlPropertyIndex_Typography_StylisticSet7 = 67,
    XamlPropertyIndex_Typography_StylisticSet8 = 68,
    XamlPropertyIndex_Typography_StylisticSet9 = 69,
    XamlPropertyIndex_Typography_Variants = 70,
    XamlPropertyIndex_AutomationPeer_EventsSource = 75,
    XamlPropertyIndex_AutoSuggestBoxSuggestionChosenEventArgs_SelectedItem = 76,
    XamlPropertyIndex_AutoSuggestBoxTextChangedEventArgs_Reason = 77,
    XamlPropertyIndex_Brush_Opacity = 78,
    XamlPropertyIndex_Brush_RelativeTransform = 79,
    XamlPropertyIndex_Brush_Transform = 80,
    XamlPropertyIndex_CollectionViewSource_IsSourceGrouped = 81,
    XamlPropertyIndex_CollectionViewSource_ItemsPath = 82,
    XamlPropertyIndex_CollectionViewSource_Source = 83,
    XamlPropertyIndex_CollectionViewSource_View = 84,
    XamlPropertyIndex_ColorKeyFrame_KeyTime = 90,
    XamlPropertyIndex_ColorKeyFrame_Value = 91,
    XamlPropertyIndex_ColumnDefinition_ActualWidth = 92,
    XamlPropertyIndex_ColumnDefinition_MaxWidth = 93,
    XamlPropertyIndex_ColumnDefinition_MinWidth = 94,
    XamlPropertyIndex_ColumnDefinition_Width = 95,
    XamlPropertyIndex_ComboBoxTemplateSettings_DropDownClosedHeight = 96,
    XamlPropertyIndex_ComboBoxTemplateSettings_DropDownOffset = 97,
    XamlPropertyIndex_ComboBoxTemplateSettings_DropDownOpenedHeight = 98,
    XamlPropertyIndex_ComboBoxTemplateSettings_SelectedItemDirection = 99,
    XamlPropertyIndex_DoubleKeyFrame_KeyTime = 107,
    XamlPropertyIndex_DoubleKeyFrame_Value = 108,
    XamlPropertyIndex_EasingFunctionBase_EasingMode = 111,
    XamlPropertyIndex_FlyoutBase_AttachedFlyout = 114,
    XamlPropertyIndex_FlyoutBase_Placement = 115,
    XamlPropertyIndex_Geometry_Bounds = 118,
    XamlPropertyIndex_Geometry_Transform = 119,
    XamlPropertyIndex_GradientStop_Color = 120,
    XamlPropertyIndex_GradientStop_Offset = 121,
    XamlPropertyIndex_GroupStyle_ContainerStyle = 124,
    XamlPropertyIndex_GroupStyle_ContainerStyleSelector = 125,
    XamlPropertyIndex_GroupStyle_HeaderContainerStyle = 126,
    XamlPropertyIndex_GroupStyle_HeaderTemplate = 127,
    XamlPropertyIndex_GroupStyle_HeaderTemplateSelector = 128,
    XamlPropertyIndex_GroupStyle_HidesIfEmpty = 129,
    XamlPropertyIndex_GroupStyle_Panel = 130,
    XamlPropertyIndex_InertiaExpansionBehavior_DesiredDeceleration = 144,
    XamlPropertyIndex_InertiaExpansionBehavior_DesiredExpansion = 145,
    XamlPropertyIndex_InertiaRotationBehavior_DesiredDeceleration = 146,
    XamlPropertyIndex_InertiaRotationBehavior_DesiredRotation = 147,
    XamlPropertyIndex_InertiaTranslationBehavior_DesiredDeceleration = 148,
    XamlPropertyIndex_InertiaTranslationBehavior_DesiredDisplacement = 149,
    XamlPropertyIndex_InputScope_Names = 150,
    XamlPropertyIndex_InputScopeName_NameValue = 151,
    XamlPropertyIndex_KeySpline_ControlPoint1 = 153,
    XamlPropertyIndex_KeySpline_ControlPoint2 = 154,
    XamlPropertyIndex_ManipulationPivot_Center = 159,
    XamlPropertyIndex_ManipulationPivot_Radius = 160,
    XamlPropertyIndex_ObjectKeyFrame_KeyTime = 183,
    XamlPropertyIndex_ObjectKeyFrame_Value = 184,
    XamlPropertyIndex_PageStackEntry_SourcePageType = 185,
    XamlPropertyIndex_PathFigure_IsClosed = 192,
    XamlPropertyIndex_PathFigure_IsFilled = 193,
    XamlPropertyIndex_PathFigure_Segments = 194,
    XamlPropertyIndex_PathFigure_StartPoint = 195,
    XamlPropertyIndex_Pointer_IsInContact = 199,
    XamlPropertyIndex_Pointer_IsInRange = 200,
    XamlPropertyIndex_Pointer_PointerDeviceType = 201,
    XamlPropertyIndex_Pointer_PointerId = 202,
    XamlPropertyIndex_PointKeyFrame_KeyTime = 205,
    XamlPropertyIndex_PointKeyFrame_Value = 206,
    XamlPropertyIndex_PrintDocument_DocumentSource = 209,
    XamlPropertyIndex_ProgressBarTemplateSettings_ContainerAnimationEndPosition = 211,
    XamlPropertyIndex_ProgressBarTemplateSettings_ContainerAnimationStartPosition = 212,
    XamlPropertyIndex_ProgressBarTemplateSettings_EllipseAnimationEndPosition = 213,
    XamlPropertyIndex_ProgressBarTemplateSettings_EllipseAnimationWellPosition = 214,
    XamlPropertyIndex_ProgressBarTemplateSettings_EllipseDiameter = 215,
    XamlPropertyIndex_ProgressBarTemplateSettings_EllipseOffset = 216,
    XamlPropertyIndex_ProgressBarTemplateSettings_IndicatorLengthDelta = 217,
    XamlPropertyIndex_ProgressRingTemplateSettings_EllipseDiameter = 218,
    XamlPropertyIndex_ProgressRingTemplateSettings_EllipseOffset = 219,
    XamlPropertyIndex_ProgressRingTemplateSettings_MaxSideLength = 220,
    XamlPropertyIndex_PropertyPath_Path = 221,
    XamlPropertyIndex_RowDefinition_ActualHeight = 226,
    XamlPropertyIndex_RowDefinition_Height = 227,
    XamlPropertyIndex_RowDefinition_MaxHeight = 228,
    XamlPropertyIndex_RowDefinition_MinHeight = 229,
    XamlPropertyIndex_SetterBase_IsSealed = 233,
    XamlPropertyIndex_SettingsFlyoutTemplateSettings_BorderBrush = 234,
    XamlPropertyIndex_SettingsFlyoutTemplateSettings_BorderThickness = 235,
    XamlPropertyIndex_SettingsFlyoutTemplateSettings_ContentTransitions = 236,
    XamlPropertyIndex_SettingsFlyoutTemplateSettings_HeaderBackground = 237,
    XamlPropertyIndex_SettingsFlyoutTemplateSettings_HeaderForeground = 238,
    XamlPropertyIndex_SettingsFlyoutTemplateSettings_IconSource = 239,
    XamlPropertyIndex_Style_BasedOn = 244,
    XamlPropertyIndex_Style_IsSealed = 245,
    XamlPropertyIndex_Style_Setters = 246,
    XamlPropertyIndex_Style_TargetType = 247,
    XamlPropertyIndex_TextElement_CharacterSpacing = 249,
    XamlPropertyIndex_TextElement_FontFamily = 250,
    XamlPropertyIndex_TextElement_FontSize = 251,
    XamlPropertyIndex_TextElement_FontStretch = 252,
    XamlPropertyIndex_TextElement_FontStyle = 253,
    XamlPropertyIndex_TextElement_FontWeight = 254,
    XamlPropertyIndex_TextElement_Foreground = 255,
    XamlPropertyIndex_TextElement_IsTextScaleFactorEnabled = 256,
    XamlPropertyIndex_TextElement_Language = 257,
    XamlPropertyIndex_Timeline_AutoReverse = 263,
    XamlPropertyIndex_Timeline_BeginTime = 264,
    XamlPropertyIndex_Timeline_Duration = 265,
    XamlPropertyIndex_Timeline_FillBehavior = 266,
    XamlPropertyIndex_Timeline_RepeatBehavior = 267,
    XamlPropertyIndex_Timeline_SpeedRatio = 268,
    XamlPropertyIndex_TimelineMarker_Text = 269,
    XamlPropertyIndex_TimelineMarker_Time = 270,
    XamlPropertyIndex_TimelineMarker_Type = 271,
    XamlPropertyIndex_ToggleSwitchTemplateSettings_CurtainCurrentToOffOffset = 273,
    XamlPropertyIndex_ToggleSwitchTemplateSettings_CurtainCurrentToOnOffset = 274,
    XamlPropertyIndex_ToggleSwitchTemplateSettings_CurtainOffToOnOffset = 275,
    XamlPropertyIndex_ToggleSwitchTemplateSettings_CurtainOnToOffOffset = 276,
    XamlPropertyIndex_ToggleSwitchTemplateSettings_KnobCurrentToOffOffset = 277,
    XamlPropertyIndex_ToggleSwitchTemplateSettings_KnobCurrentToOnOffset = 278,
    XamlPropertyIndex_ToggleSwitchTemplateSettings_KnobOffToOnOffset = 279,
    XamlPropertyIndex_ToggleSwitchTemplateSettings_KnobOnToOffOffset = 280,
    XamlPropertyIndex_ToolTipTemplateSettings_FromHorizontalOffset = 281,
    XamlPropertyIndex_ToolTipTemplateSettings_FromVerticalOffset = 282,
    XamlPropertyIndex_UIElement_AllowDrop = 292,
    XamlPropertyIndex_UIElement_CacheMode = 293,
    XamlPropertyIndex_UIElement_Clip = 295,
    XamlPropertyIndex_UIElement_CompositeMode = 296,
    XamlPropertyIndex_UIElement_IsDoubleTapEnabled = 297,
    XamlPropertyIndex_UIElement_IsHitTestVisible = 298,
    XamlPropertyIndex_UIElement_IsHoldingEnabled = 299,
    XamlPropertyIndex_UIElement_IsRightTapEnabled = 300,
    XamlPropertyIndex_UIElement_IsTapEnabled = 301,
    XamlPropertyIndex_UIElement_ManipulationMode = 302,
    XamlPropertyIndex_UIElement_Opacity = 303,
    XamlPropertyIndex_UIElement_PointerCaptures = 304,
    XamlPropertyIndex_UIElement_Projection = 305,
    XamlPropertyIndex_UIElement_RenderSize = 306,
    XamlPropertyIndex_UIElement_RenderTransform = 307,
    XamlPropertyIndex_UIElement_RenderTransformOrigin = 308,
    XamlPropertyIndex_UIElement_Transitions = 309,
    XamlPropertyIndex_UIElement_UseLayoutRounding = 311,
    XamlPropertyIndex_UIElement_Visibility = 312,
    XamlPropertyIndex_VisualState_Storyboard = 322,
    XamlPropertyIndex_VisualStateGroup_States = 323,
    XamlPropertyIndex_VisualStateGroup_Transitions = 324,
    XamlPropertyIndex_VisualStateManager_CustomVisualStateManager = 325,
    XamlPropertyIndex_VisualStateManager_VisualStateGroups = 326,
    XamlPropertyIndex_VisualTransition_From = 327,
    XamlPropertyIndex_VisualTransition_GeneratedDuration = 328,
    XamlPropertyIndex_VisualTransition_GeneratedEasingFunction = 329,
    XamlPropertyIndex_VisualTransition_Storyboard = 330,
    XamlPropertyIndex_VisualTransition_To = 331,
    XamlPropertyIndex_ArcSegment_IsLargeArc = 332,
    XamlPropertyIndex_ArcSegment_Point = 333,
    XamlPropertyIndex_ArcSegment_RotationAngle = 334,
    XamlPropertyIndex_ArcSegment_Size = 335,
    XamlPropertyIndex_ArcSegment_SweepDirection = 336,
    XamlPropertyIndex_BackEase_Amplitude = 337,
    XamlPropertyIndex_BeginStoryboard_Storyboard = 338,
    XamlPropertyIndex_BezierSegment_Point1 = 339,
    XamlPropertyIndex_BezierSegment_Point2 = 340,
    XamlPropertyIndex_BezierSegment_Point3 = 341,
    XamlPropertyIndex_BitmapSource_PixelHeight = 342,
    XamlPropertyIndex_BitmapSource_PixelWidth = 343,
    XamlPropertyIndex_Block_LineHeight = 344,
    XamlPropertyIndex_Block_LineStackingStrategy = 345,
    XamlPropertyIndex_Block_Margin = 346,
    XamlPropertyIndex_Block_TextAlignment = 347,
    XamlPropertyIndex_BounceEase_Bounces = 348,
    XamlPropertyIndex_BounceEase_Bounciness = 349,
    XamlPropertyIndex_ColorAnimation_By = 350,
    XamlPropertyIndex_ColorAnimation_EasingFunction = 351,
    XamlPropertyIndex_ColorAnimation_EnableDependentAnimation = 352,
    XamlPropertyIndex_ColorAnimation_From = 353,
    XamlPropertyIndex_ColorAnimation_To = 354,
    XamlPropertyIndex_ColorAnimationUsingKeyFrames_EnableDependentAnimation = 355,
    XamlPropertyIndex_ColorAnimationUsingKeyFrames_KeyFrames = 356,
    XamlPropertyIndex_ContentThemeTransition_HorizontalOffset = 357,
    XamlPropertyIndex_ContentThemeTransition_VerticalOffset = 358,
    XamlPropertyIndex_ControlTemplate_TargetType = 359,
    XamlPropertyIndex_DispatcherTimer_Interval = 362,
    XamlPropertyIndex_DoubleAnimation_By = 363,
    XamlPropertyIndex_DoubleAnimation_EasingFunction = 364,
    XamlPropertyIndex_DoubleAnimation_EnableDependentAnimation = 365,
    XamlPropertyIndex_DoubleAnimation_From = 366,
    XamlPropertyIndex_DoubleAnimation_To = 367,
    XamlPropertyIndex_DoubleAnimationUsingKeyFrames_EnableDependentAnimation = 368,
    XamlPropertyIndex_DoubleAnimationUsingKeyFrames_KeyFrames = 369,
    XamlPropertyIndex_EasingColorKeyFrame_EasingFunction = 372,
    XamlPropertyIndex_EasingDoubleKeyFrame_EasingFunction = 373,
    XamlPropertyIndex_EasingPointKeyFrame_EasingFunction = 374,
    XamlPropertyIndex_EdgeUIThemeTransition_Edge = 375,
    XamlPropertyIndex_ElasticEase_Oscillations = 376,
    XamlPropertyIndex_ElasticEase_Springiness = 377,
    XamlPropertyIndex_EllipseGeometry_Center = 378,
    XamlPropertyIndex_EllipseGeometry_RadiusX = 379,
    XamlPropertyIndex_EllipseGeometry_RadiusY = 380,
    XamlPropertyIndex_EntranceThemeTransition_FromHorizontalOffset = 381,
    XamlPropertyIndex_EntranceThemeTransition_FromVerticalOffset = 382,
    XamlPropertyIndex_EntranceThemeTransition_IsStaggeringEnabled = 383,
    XamlPropertyIndex_EventTrigger_Actions = 384,
    XamlPropertyIndex_EventTrigger_RoutedEvent = 385,
    XamlPropertyIndex_ExponentialEase_Exponent = 386,
    XamlPropertyIndex_Flyout_Content = 387,
    XamlPropertyIndex_Flyout_FlyoutPresenterStyle = 388,
    XamlPropertyIndex_FrameworkElement_ActualHeight = 389,
    XamlPropertyIndex_FrameworkElement_ActualWidth = 390,
    XamlPropertyIndex_FrameworkElement_DataContext = 391,
    XamlPropertyIndex_FrameworkElement_FlowDirection = 392,
    XamlPropertyIndex_FrameworkElement_Height = 393,
    XamlPropertyIndex_FrameworkElement_HorizontalAlignment = 394,
    XamlPropertyIndex_FrameworkElement_Language = 396,
    XamlPropertyIndex_FrameworkElement_Margin = 397,
    XamlPropertyIndex_FrameworkElement_MaxHeight = 398,
    XamlPropertyIndex_FrameworkElement_MaxWidth = 399,
    XamlPropertyIndex_FrameworkElement_MinHeight = 400,
    XamlPropertyIndex_FrameworkElement_MinWidth = 401,
    XamlPropertyIndex_FrameworkElement_Parent = 402,
    XamlPropertyIndex_FrameworkElement_RequestedTheme = 403,
    XamlPropertyIndex_FrameworkElement_Resources = 404,
    XamlPropertyIndex_FrameworkElement_Style = 405,
    XamlPropertyIndex_FrameworkElement_Tag = 406,
    XamlPropertyIndex_FrameworkElement_Triggers = 407,
    XamlPropertyIndex_FrameworkElement_VerticalAlignment = 408,
    XamlPropertyIndex_FrameworkElement_Width = 409,
    XamlPropertyIndex_FrameworkElementAutomationPeer_Owner = 410,
    XamlPropertyIndex_GeometryGroup_Children = 411,
    XamlPropertyIndex_GeometryGroup_FillRule = 412,
    XamlPropertyIndex_GradientBrush_ColorInterpolationMode = 413,
    XamlPropertyIndex_GradientBrush_GradientStops = 414,
    XamlPropertyIndex_GradientBrush_MappingMode = 415,
    XamlPropertyIndex_GradientBrush_SpreadMethod = 416,
    XamlPropertyIndex_GridViewItemTemplateSettings_DragItemsCount = 417,
    XamlPropertyIndex_ItemAutomationPeer_Item = 419,
    XamlPropertyIndex_ItemAutomationPeer_ItemsControlAutomationPeer = 420,
    XamlPropertyIndex_LineGeometry_EndPoint = 422,
    XamlPropertyIndex_LineGeometry_StartPoint = 423,
    XamlPropertyIndex_LineSegment_Point = 424,
    XamlPropertyIndex_ListViewItemTemplateSettings_DragItemsCount = 425,
    XamlPropertyIndex_Matrix3DProjection_ProjectionMatrix = 426,
    XamlPropertyIndex_MenuFlyout_Items = 427,
    XamlPropertyIndex_MenuFlyout_MenuFlyoutPresenterStyle = 428,
    XamlPropertyIndex_ObjectAnimationUsingKeyFrames_EnableDependentAnimation = 429,
    XamlPropertyIndex_ObjectAnimationUsingKeyFrames_KeyFrames = 430,
    XamlPropertyIndex_PaneThemeTransition_Edge = 431,
    XamlPropertyIndex_PathGeometry_Figures = 432,
    XamlPropertyIndex_PathGeometry_FillRule = 433,
    XamlPropertyIndex_PlaneProjection_CenterOfRotationX = 434,
    XamlPropertyIndex_PlaneProjection_CenterOfRotationY = 435,
    XamlPropertyIndex_PlaneProjection_CenterOfRotationZ = 436,
    XamlPropertyIndex_PlaneProjection_GlobalOffsetX = 437,
    XamlPropertyIndex_PlaneProjection_GlobalOffsetY = 438,
    XamlPropertyIndex_PlaneProjection_GlobalOffsetZ = 439,
    XamlPropertyIndex_PlaneProjection_LocalOffsetX = 440,
    XamlPropertyIndex_PlaneProjection_LocalOffsetY = 441,
    XamlPropertyIndex_PlaneProjection_LocalOffsetZ = 442,
    XamlPropertyIndex_PlaneProjection_ProjectionMatrix = 443,
    XamlPropertyIndex_PlaneProjection_RotationX = 444,
    XamlPropertyIndex_PlaneProjection_RotationY = 445,
    XamlPropertyIndex_PlaneProjection_RotationZ = 446,
    XamlPropertyIndex_PointAnimation_By = 447,
    XamlPropertyIndex_PointAnimation_EasingFunction = 448,
    XamlPropertyIndex_PointAnimation_EnableDependentAnimation = 449,
    XamlPropertyIndex_PointAnimation_From = 450,
    XamlPropertyIndex_PointAnimation_To = 451,
    XamlPropertyIndex_PointAnimationUsingKeyFrames_EnableDependentAnimation = 452,
    XamlPropertyIndex_PointAnimationUsingKeyFrames_KeyFrames = 453,
    XamlPropertyIndex_PolyBezierSegment_Points = 456,
    XamlPropertyIndex_PolyLineSegment_Points = 457,
    XamlPropertyIndex_PolyQuadraticBezierSegment_Points = 458,
    XamlPropertyIndex_PopupThemeTransition_FromHorizontalOffset = 459,
    XamlPropertyIndex_PopupThemeTransition_FromVerticalOffset = 460,
    XamlPropertyIndex_PowerEase_Power = 461,
    XamlPropertyIndex_QuadraticBezierSegment_Point1 = 466,
    XamlPropertyIndex_QuadraticBezierSegment_Point2 = 467,
    XamlPropertyIndex_RectangleGeometry_Rect = 470,
    XamlPropertyIndex_RelativeSource_Mode = 471,
    XamlPropertyIndex_RenderTargetBitmap_PixelHeight = 472,
    XamlPropertyIndex_RenderTargetBitmap_PixelWidth = 473,
    XamlPropertyIndex_Setter_Property = 474,
    XamlPropertyIndex_Setter_Value = 475,
    XamlPropertyIndex_SolidColorBrush_Color = 476,
    XamlPropertyIndex_SplineColorKeyFrame_KeySpline = 477,
    XamlPropertyIndex_SplineDoubleKeyFrame_KeySpline = 478,
    XamlPropertyIndex_SplinePointKeyFrame_KeySpline = 479,
    XamlPropertyIndex_TileBrush_AlignmentX = 483,
    XamlPropertyIndex_TileBrush_AlignmentY = 484,
    XamlPropertyIndex_TileBrush_Stretch = 485,
    XamlPropertyIndex_Binding_Converter = 487,
    XamlPropertyIndex_Binding_ConverterLanguage = 488,
    XamlPropertyIndex_Binding_ConverterParameter = 489,
    XamlPropertyIndex_Binding_ElementName = 490,
    XamlPropertyIndex_Binding_FallbackValue = 491,
    XamlPropertyIndex_Binding_Mode = 492,
    XamlPropertyIndex_Binding_Path = 493,
    XamlPropertyIndex_Binding_RelativeSource = 494,
    XamlPropertyIndex_Binding_Source = 495,
    XamlPropertyIndex_Binding_TargetNullValue = 496,
    XamlPropertyIndex_Binding_UpdateSourceTrigger = 497,
    XamlPropertyIndex_BitmapImage_CreateOptions = 498,
    XamlPropertyIndex_BitmapImage_DecodePixelHeight = 499,
    XamlPropertyIndex_BitmapImage_DecodePixelType = 500,
    XamlPropertyIndex_BitmapImage_DecodePixelWidth = 501,
    XamlPropertyIndex_BitmapImage_UriSource = 502,
    XamlPropertyIndex_Border_Background = 503,
    XamlPropertyIndex_Border_BorderBrush = 504,
    XamlPropertyIndex_Border_BorderThickness = 505,
    XamlPropertyIndex_Border_Child = 506,
    XamlPropertyIndex_Border_ChildTransitions = 507,
    XamlPropertyIndex_Border_CornerRadius = 508,
    XamlPropertyIndex_Border_Padding = 509,
    XamlPropertyIndex_CaptureElement_Source = 510,
    XamlPropertyIndex_CaptureElement_Stretch = 511,
    XamlPropertyIndex_CompositeTransform_CenterX = 514,
    XamlPropertyIndex_CompositeTransform_CenterY = 515,
    XamlPropertyIndex_CompositeTransform_Rotation = 516,
    XamlPropertyIndex_CompositeTransform_ScaleX = 517,
    XamlPropertyIndex_CompositeTransform_ScaleY = 518,
    XamlPropertyIndex_CompositeTransform_SkewX = 519,
    XamlPropertyIndex_CompositeTransform_SkewY = 520,
    XamlPropertyIndex_CompositeTransform_TranslateX = 521,
    XamlPropertyIndex_CompositeTransform_TranslateY = 522,
    XamlPropertyIndex_ContentPresenter_CharacterSpacing = 523,
    XamlPropertyIndex_ContentPresenter_Content = 524,
    XamlPropertyIndex_ContentPresenter_ContentTemplate = 525,
    XamlPropertyIndex_ContentPresenter_ContentTemplateSelector = 526,
    XamlPropertyIndex_ContentPresenter_ContentTransitions = 527,
    XamlPropertyIndex_ContentPresenter_FontFamily = 528,
    XamlPropertyIndex_ContentPresenter_FontSize = 529,
    XamlPropertyIndex_ContentPresenter_FontStretch = 530,
    XamlPropertyIndex_ContentPresenter_FontStyle = 531,
    XamlPropertyIndex_ContentPresenter_FontWeight = 532,
    XamlPropertyIndex_ContentPresenter_Foreground = 533,
    XamlPropertyIndex_ContentPresenter_IsTextScaleFactorEnabled = 534,
    XamlPropertyIndex_ContentPresenter_LineStackingStrategy = 535,
    XamlPropertyIndex_ContentPresenter_MaxLines = 536,
    XamlPropertyIndex_ContentPresenter_OpticalMarginAlignment = 537,
    XamlPropertyIndex_ContentPresenter_TextLineBounds = 539,
    XamlPropertyIndex_ContentPresenter_TextWrapping = 540,
    XamlPropertyIndex_Control_Background = 541,
    XamlPropertyIndex_Control_BorderBrush = 542,
    XamlPropertyIndex_Control_BorderThickness = 543,
    XamlPropertyIndex_Control_CharacterSpacing = 544,
    XamlPropertyIndex_Control_FocusState = 546,
    XamlPropertyIndex_Control_FontFamily = 547,
    XamlPropertyIndex_Control_FontSize = 548,
    XamlPropertyIndex_Control_FontStretch = 549,
    XamlPropertyIndex_Control_FontStyle = 550,
    XamlPropertyIndex_Control_FontWeight = 551,
    XamlPropertyIndex_Control_Foreground = 552,
    XamlPropertyIndex_Control_HorizontalContentAlignment = 553,
    XamlPropertyIndex_Control_IsEnabled = 554,
    XamlPropertyIndex_Control_IsTabStop = 555,
    XamlPropertyIndex_Control_IsTextScaleFactorEnabled = 556,
    XamlPropertyIndex_Control_Padding = 557,
    XamlPropertyIndex_Control_TabIndex = 558,
    XamlPropertyIndex_Control_TabNavigation = 559,
    XamlPropertyIndex_Control_Template = 560,
    XamlPropertyIndex_Control_VerticalContentAlignment = 561,
    XamlPropertyIndex_DragItemThemeAnimation_TargetName = 565,
    XamlPropertyIndex_DragOverThemeAnimation_Direction = 566,
    XamlPropertyIndex_DragOverThemeAnimation_TargetName = 567,
    XamlPropertyIndex_DragOverThemeAnimation_ToOffset = 568,
    XamlPropertyIndex_DropTargetItemThemeAnimation_TargetName = 569,
    XamlPropertyIndex_FadeInThemeAnimation_TargetName = 570,
    XamlPropertyIndex_FadeOutThemeAnimation_TargetName = 571,
    XamlPropertyIndex_Glyphs_Fill = 574,
    XamlPropertyIndex_Glyphs_FontRenderingEmSize = 575,
    XamlPropertyIndex_Glyphs_FontUri = 576,
    XamlPropertyIndex_Glyphs_Indices = 577,
    XamlPropertyIndex_Glyphs_OriginX = 578,
    XamlPropertyIndex_Glyphs_OriginY = 579,
    XamlPropertyIndex_Glyphs_StyleSimulations = 580,
    XamlPropertyIndex_Glyphs_UnicodeString = 581,
    XamlPropertyIndex_IconElement_Foreground = 584,
    XamlPropertyIndex_Image_NineGrid = 586,
    XamlPropertyIndex_Image_PlayToSource = 587,
    XamlPropertyIndex_Image_Source = 588,
    XamlPropertyIndex_Image_Stretch = 589,
    XamlPropertyIndex_ImageBrush_ImageSource = 591,
    XamlPropertyIndex_InlineUIContainer_Child = 592,
    XamlPropertyIndex_ItemsPresenter_Footer = 594,
    XamlPropertyIndex_ItemsPresenter_FooterTemplate = 595,
    XamlPropertyIndex_ItemsPresenter_FooterTransitions = 596,
    XamlPropertyIndex_ItemsPresenter_Header = 597,
    XamlPropertyIndex_ItemsPresenter_HeaderTemplate = 598,
    XamlPropertyIndex_ItemsPresenter_HeaderTransitions = 599,
    XamlPropertyIndex_ItemsPresenter_Padding = 601,
    XamlPropertyIndex_LinearGradientBrush_EndPoint = 602,
    XamlPropertyIndex_LinearGradientBrush_StartPoint = 603,
    XamlPropertyIndex_MatrixTransform_Matrix = 604,
    XamlPropertyIndex_MediaElement_ActualStereo3DVideoPackingMode = 605,
    XamlPropertyIndex_MediaElement_AreTransportControlsEnabled = 606,
    XamlPropertyIndex_MediaElement_AspectRatioHeight = 607,
    XamlPropertyIndex_MediaElement_AspectRatioWidth = 608,
    XamlPropertyIndex_MediaElement_AudioCategory = 609,
    XamlPropertyIndex_MediaElement_AudioDeviceType = 610,
    XamlPropertyIndex_MediaElement_AudioStreamCount = 611,
    XamlPropertyIndex_MediaElement_AudioStreamIndex = 612,
    XamlPropertyIndex_MediaElement_AutoPlay = 613,
    XamlPropertyIndex_MediaElement_Balance = 614,
    XamlPropertyIndex_MediaElement_BufferingProgress = 615,
    XamlPropertyIndex_MediaElement_CanPause = 616,
    XamlPropertyIndex_MediaElement_CanSeek = 617,
    XamlPropertyIndex_MediaElement_CurrentState = 618,
    XamlPropertyIndex_MediaElement_DefaultPlaybackRate = 619,
    XamlPropertyIndex_MediaElement_DownloadProgress = 620,
    XamlPropertyIndex_MediaElement_DownloadProgressOffset = 621,
    XamlPropertyIndex_MediaElement_IsAudioOnly = 623,
    XamlPropertyIndex_MediaElement_IsFullWindow = 624,
    XamlPropertyIndex_MediaElement_IsLooping = 625,
    XamlPropertyIndex_MediaElement_IsMuted = 626,
    XamlPropertyIndex_MediaElement_IsStereo3DVideo = 627,
    XamlPropertyIndex_MediaElement_Markers = 628,
    XamlPropertyIndex_MediaElement_NaturalDuration = 629,
    XamlPropertyIndex_MediaElement_NaturalVideoHeight = 630,
    XamlPropertyIndex_MediaElement_NaturalVideoWidth = 631,
    XamlPropertyIndex_MediaElement_PlaybackRate = 632,
    XamlPropertyIndex_MediaElement_PlayToPreferredSourceUri = 633,
    XamlPropertyIndex_MediaElement_PlayToSource = 634,
    XamlPropertyIndex_MediaElement_Position = 635,
    XamlPropertyIndex_MediaElement_PosterSource = 636,
    XamlPropertyIndex_MediaElement_ProtectionManager = 637,
    XamlPropertyIndex_MediaElement_RealTimePlayback = 638,
    XamlPropertyIndex_MediaElement_Source = 639,
    XamlPropertyIndex_MediaElement_Stereo3DVideoPackingMode = 640,
    XamlPropertyIndex_MediaElement_Stereo3DVideoRenderMode = 641,
    XamlPropertyIndex_MediaElement_Stretch = 642,
    XamlPropertyIndex_MediaElement_TransportControls = 643,
    XamlPropertyIndex_MediaElement_Volume = 644,
    XamlPropertyIndex_Panel_Background = 647,
    XamlPropertyIndex_Panel_Children = 648,
    XamlPropertyIndex_Panel_ChildrenTransitions = 649,
    XamlPropertyIndex_Panel_IsItemsHost = 651,
    XamlPropertyIndex_Paragraph_Inlines = 652,
    XamlPropertyIndex_Paragraph_TextIndent = 653,
    XamlPropertyIndex_PointerDownThemeAnimation_TargetName = 660,
    XamlPropertyIndex_PointerUpThemeAnimation_TargetName = 662,
    XamlPropertyIndex_PopInThemeAnimation_FromHorizontalOffset = 664,
    XamlPropertyIndex_PopInThemeAnimation_FromVerticalOffset = 665,
    XamlPropertyIndex_PopInThemeAnimation_TargetName = 666,
    XamlPropertyIndex_PopOutThemeAnimation_TargetName = 667,
    XamlPropertyIndex_Popup_Child = 668,
    XamlPropertyIndex_Popup_ChildTransitions = 669,
    XamlPropertyIndex_Popup_HorizontalOffset = 670,
    XamlPropertyIndex_Popup_IsLightDismissEnabled = 673,
    XamlPropertyIndex_Popup_IsOpen = 674,
    XamlPropertyIndex_Popup_VerticalOffset = 676,
    XamlPropertyIndex_RepositionThemeAnimation_FromHorizontalOffset = 683,
    XamlPropertyIndex_RepositionThemeAnimation_FromVerticalOffset = 684,
    XamlPropertyIndex_RepositionThemeAnimation_TargetName = 685,
    XamlPropertyIndex_ResourceDictionary_MergedDictionaries = 687,
    XamlPropertyIndex_ResourceDictionary_Source = 688,
    XamlPropertyIndex_ResourceDictionary_ThemeDictionaries = 689,
    XamlPropertyIndex_RichTextBlock_Blocks = 691,
    XamlPropertyIndex_RichTextBlock_CharacterSpacing = 692,
    XamlPropertyIndex_RichTextBlock_FontFamily = 693,
    XamlPropertyIndex_RichTextBlock_FontSize = 694,
    XamlPropertyIndex_RichTextBlock_FontStretch = 695,
    XamlPropertyIndex_RichTextBlock_FontStyle = 696,
    XamlPropertyIndex_RichTextBlock_FontWeight = 697,
    XamlPropertyIndex_RichTextBlock_Foreground = 698,
    XamlPropertyIndex_RichTextBlock_HasOverflowContent = 699,
    XamlPropertyIndex_RichTextBlock_IsColorFontEnabled = 700,
    XamlPropertyIndex_RichTextBlock_IsTextScaleFactorEnabled = 701,
    XamlPropertyIndex_RichTextBlock_IsTextSelectionEnabled = 702,
    XamlPropertyIndex_RichTextBlock_LineHeight = 703,
    XamlPropertyIndex_RichTextBlock_LineStackingStrategy = 704,
    XamlPropertyIndex_RichTextBlock_MaxLines = 705,
    XamlPropertyIndex_RichTextBlock_OpticalMarginAlignment = 706,
    XamlPropertyIndex_RichTextBlock_OverflowContentTarget = 707,
    XamlPropertyIndex_RichTextBlock_Padding = 708,
    XamlPropertyIndex_RichTextBlock_SelectedText = 709,
    XamlPropertyIndex_RichTextBlock_SelectionHighlightColor = 710,
    XamlPropertyIndex_RichTextBlock_TextAlignment = 711,
    XamlPropertyIndex_RichTextBlock_TextIndent = 712,
    XamlPropertyIndex_RichTextBlock_TextLineBounds = 713,
    XamlPropertyIndex_RichTextBlock_TextReadingOrder = 714,
    XamlPropertyIndex_RichTextBlock_TextTrimming = 715,
    XamlPropertyIndex_RichTextBlock_TextWrapping = 716,
    XamlPropertyIndex_RichTextBlockOverflow_HasOverflowContent = 717,
    XamlPropertyIndex_RichTextBlockOverflow_MaxLines = 718,
    XamlPropertyIndex_RichTextBlockOverflow_OverflowContentTarget = 719,
    XamlPropertyIndex_RichTextBlockOverflow_Padding = 720,
    XamlPropertyIndex_RotateTransform_Angle = 721,
    XamlPropertyIndex_RotateTransform_CenterX = 722,
    XamlPropertyIndex_RotateTransform_CenterY = 723,
    XamlPropertyIndex_Run_FlowDirection = 725,
    XamlPropertyIndex_Run_Text = 726,
    XamlPropertyIndex_ScaleTransform_CenterX = 727,
    XamlPropertyIndex_ScaleTransform_CenterY = 728,
    XamlPropertyIndex_ScaleTransform_ScaleX = 729,
    XamlPropertyIndex_ScaleTransform_ScaleY = 730,
    XamlPropertyIndex_SetterBaseCollection_IsSealed = 732,
    XamlPropertyIndex_Shape_Fill = 733,
    XamlPropertyIndex_Shape_GeometryTransform = 734,
    XamlPropertyIndex_Shape_Stretch = 735,
    XamlPropertyIndex_Shape_Stroke = 736,
    XamlPropertyIndex_Shape_StrokeDashArray = 737,
    XamlPropertyIndex_Shape_StrokeDashCap = 738,
    XamlPropertyIndex_Shape_StrokeDashOffset = 739,
    XamlPropertyIndex_Shape_StrokeEndLineCap = 740,
    XamlPropertyIndex_Shape_StrokeLineJoin = 741,
    XamlPropertyIndex_Shape_StrokeMiterLimit = 742,
    XamlPropertyIndex_Shape_StrokeStartLineCap = 743,
    XamlPropertyIndex_Shape_StrokeThickness = 744,
    XamlPropertyIndex_SkewTransform_AngleX = 745,
    XamlPropertyIndex_SkewTransform_AngleY = 746,
    XamlPropertyIndex_SkewTransform_CenterX = 747,
    XamlPropertyIndex_SkewTransform_CenterY = 748,
    XamlPropertyIndex_Span_Inlines = 749,
    XamlPropertyIndex_SplitCloseThemeAnimation_ClosedLength = 750,
    XamlPropertyIndex_SplitCloseThemeAnimation_ClosedTarget = 751,
    XamlPropertyIndex_SplitCloseThemeAnimation_ClosedTargetName = 752,
    XamlPropertyIndex_SplitCloseThemeAnimation_ContentTarget = 753,
    XamlPropertyIndex_SplitCloseThemeAnimation_ContentTargetName = 754,
    XamlPropertyIndex_SplitCloseThemeAnimation_ContentTranslationDirection = 755,
    XamlPropertyIndex_SplitCloseThemeAnimation_ContentTranslationOffset = 756,
    XamlPropertyIndex_SplitCloseThemeAnimation_OffsetFromCenter = 757,
    XamlPropertyIndex_SplitCloseThemeAnimation_OpenedLength = 758,
    XamlPropertyIndex_SplitCloseThemeAnimation_OpenedTarget = 759,
    XamlPropertyIndex_SplitCloseThemeAnimation_OpenedTargetName = 760,
    XamlPropertyIndex_SplitOpenThemeAnimation_ClosedLength = 761,
    XamlPropertyIndex_SplitOpenThemeAnimation_ClosedTarget = 762,
    XamlPropertyIndex_SplitOpenThemeAnimation_ClosedTargetName = 763,
    XamlPropertyIndex_SplitOpenThemeAnimation_ContentTarget = 764,
    XamlPropertyIndex_SplitOpenThemeAnimation_ContentTargetName = 765,
    XamlPropertyIndex_SplitOpenThemeAnimation_ContentTranslationDirection = 766,
    XamlPropertyIndex_SplitOpenThemeAnimation_ContentTranslationOffset = 767,
    XamlPropertyIndex_SplitOpenThemeAnimation_OffsetFromCenter = 768,
    XamlPropertyIndex_SplitOpenThemeAnimation_OpenedLength = 769,
    XamlPropertyIndex_SplitOpenThemeAnimation_OpenedTarget = 770,
    XamlPropertyIndex_SplitOpenThemeAnimation_OpenedTargetName = 771,
    XamlPropertyIndex_Storyboard_Children = 772,
    XamlPropertyIndex_Storyboard_TargetName = 774,
    XamlPropertyIndex_Storyboard_TargetProperty = 775,
    XamlPropertyIndex_SwipeBackThemeAnimation_FromHorizontalOffset = 776,
    XamlPropertyIndex_SwipeBackThemeAnimation_FromVerticalOffset = 777,
    XamlPropertyIndex_SwipeBackThemeAnimation_TargetName = 778,
    XamlPropertyIndex_SwipeHintThemeAnimation_TargetName = 779,
    XamlPropertyIndex_SwipeHintThemeAnimation_ToHorizontalOffset = 780,
    XamlPropertyIndex_SwipeHintThemeAnimation_ToVerticalOffset = 781,
    XamlPropertyIndex_TextBlock_CharacterSpacing = 782,
    XamlPropertyIndex_TextBlock_FontFamily = 783,
    XamlPropertyIndex_TextBlock_FontSize = 784,
    XamlPropertyIndex_TextBlock_FontStretch = 785,
    XamlPropertyIndex_TextBlock_FontStyle = 786,
    XamlPropertyIndex_TextBlock_FontWeight = 787,
    XamlPropertyIndex_TextBlock_Foreground = 788,
    XamlPropertyIndex_TextBlock_Inlines = 789,
    XamlPropertyIndex_TextBlock_IsColorFontEnabled = 790,
    XamlPropertyIndex_TextBlock_IsTextScaleFactorEnabled = 791,
    XamlPropertyIndex_TextBlock_IsTextSelectionEnabled = 792,
    XamlPropertyIndex_TextBlock_LineHeight = 793,
    XamlPropertyIndex_TextBlock_LineStackingStrategy = 794,
    XamlPropertyIndex_TextBlock_MaxLines = 795,
    XamlPropertyIndex_TextBlock_OpticalMarginAlignment = 796,
    XamlPropertyIndex_TextBlock_Padding = 797,
    XamlPropertyIndex_TextBlock_SelectedText = 798,
    XamlPropertyIndex_TextBlock_SelectionHighlightColor = 799,
    XamlPropertyIndex_TextBlock_Text = 800,
    XamlPropertyIndex_TextBlock_TextAlignment = 801,
    XamlPropertyIndex_TextBlock_TextDecorations = 802,
    XamlPropertyIndex_TextBlock_TextLineBounds = 803,
    XamlPropertyIndex_TextBlock_TextReadingOrder = 804,
    XamlPropertyIndex_TextBlock_TextTrimming = 805,
    XamlPropertyIndex_TextBlock_TextWrapping = 806,
    XamlPropertyIndex_TransformGroup_Children = 811,
    XamlPropertyIndex_TransformGroup_Value = 812,
    XamlPropertyIndex_TranslateTransform_X = 814,
    XamlPropertyIndex_TranslateTransform_Y = 815,
    XamlPropertyIndex_Viewbox_Child = 819,
    XamlPropertyIndex_Viewbox_Stretch = 820,
    XamlPropertyIndex_Viewbox_StretchDirection = 821,
    XamlPropertyIndex_WebViewBrush_SourceName = 825,
    XamlPropertyIndex_AppBarSeparator_IsCompact = 826,
    XamlPropertyIndex_BitmapIcon_UriSource = 827,
    XamlPropertyIndex_Canvas_Left = 828,
    XamlPropertyIndex_Canvas_Top = 829,
    XamlPropertyIndex_Canvas_ZIndex = 830,
    XamlPropertyIndex_ContentControl_Content = 832,
    XamlPropertyIndex_ContentControl_ContentTemplate = 833,
    XamlPropertyIndex_ContentControl_ContentTemplateSelector = 834,
    XamlPropertyIndex_ContentControl_ContentTransitions = 835,
    XamlPropertyIndex_DatePicker_CalendarIdentifier = 837,
    XamlPropertyIndex_DatePicker_Date = 838,
    XamlPropertyIndex_DatePicker_DayFormat = 839,
    XamlPropertyIndex_DatePicker_DayVisible = 840,
    XamlPropertyIndex_DatePicker_Header = 841,
    XamlPropertyIndex_DatePicker_HeaderTemplate = 842,
    XamlPropertyIndex_DatePicker_MaxYear = 843,
    XamlPropertyIndex_DatePicker_MinYear = 844,
    XamlPropertyIndex_DatePicker_MonthFormat = 845,
    XamlPropertyIndex_DatePicker_MonthVisible = 846,
    XamlPropertyIndex_DatePicker_Orientation = 847,
    XamlPropertyIndex_DatePicker_YearFormat = 848,
    XamlPropertyIndex_DatePicker_YearVisible = 849,
    XamlPropertyIndex_FontIcon_FontFamily = 851,
    XamlPropertyIndex_FontIcon_FontSize = 852,
    XamlPropertyIndex_FontIcon_FontStyle = 853,
    XamlPropertyIndex_FontIcon_FontWeight = 854,
    XamlPropertyIndex_FontIcon_Glyph = 855,
    XamlPropertyIndex_FontIcon_IsTextScaleFactorEnabled = 856,
    XamlPropertyIndex_Grid_Column = 857,
    XamlPropertyIndex_Grid_ColumnDefinitions = 858,
    XamlPropertyIndex_Grid_ColumnSpan = 859,
    XamlPropertyIndex_Grid_Row = 860,
    XamlPropertyIndex_Grid_RowDefinitions = 861,
    XamlPropertyIndex_Grid_RowSpan = 862,
    XamlPropertyIndex_Hub_DefaultSectionIndex = 863,
    XamlPropertyIndex_Hub_Header = 864,
    XamlPropertyIndex_Hub_HeaderTemplate = 865,
    XamlPropertyIndex_Hub_IsActiveView = 866,
    XamlPropertyIndex_Hub_IsZoomedInView = 867,
    XamlPropertyIndex_Hub_Orientation = 868,
    XamlPropertyIndex_Hub_SectionHeaders = 869,
    XamlPropertyIndex_Hub_Sections = 870,
    XamlPropertyIndex_Hub_SectionsInView = 871,
    XamlPropertyIndex_Hub_SemanticZoomOwner = 872,
    XamlPropertyIndex_HubSection_ContentTemplate = 873,
    XamlPropertyIndex_HubSection_Header = 874,
    XamlPropertyIndex_HubSection_HeaderTemplate = 875,
    XamlPropertyIndex_HubSection_IsHeaderInteractive = 876,
    XamlPropertyIndex_Hyperlink_NavigateUri = 877,
    XamlPropertyIndex_ItemsControl_DisplayMemberPath = 879,
    XamlPropertyIndex_ItemsControl_GroupStyle = 880,
    XamlPropertyIndex_ItemsControl_GroupStyleSelector = 881,
    XamlPropertyIndex_ItemsControl_IsGrouping = 882,
    XamlPropertyIndex_ItemsControl_ItemContainerStyle = 884,
    XamlPropertyIndex_ItemsControl_ItemContainerStyleSelector = 885,
    XamlPropertyIndex_ItemsControl_ItemContainerTransitions = 886,
    XamlPropertyIndex_ItemsControl_Items = 887,
    XamlPropertyIndex_ItemsControl_ItemsPanel = 889,
    XamlPropertyIndex_ItemsControl_ItemsSource = 890,
    XamlPropertyIndex_ItemsControl_ItemTemplate = 891,
    XamlPropertyIndex_ItemsControl_ItemTemplateSelector = 892,
    XamlPropertyIndex_Line_X1 = 893,
    XamlPropertyIndex_Line_X2 = 894,
    XamlPropertyIndex_Line_Y1 = 895,
    XamlPropertyIndex_Line_Y2 = 896,
    XamlPropertyIndex_MediaTransportControls_IsFastForwardButtonVisible = 898,
    XamlPropertyIndex_MediaTransportControls_IsFastRewindButtonVisible = 900,
    XamlPropertyIndex_MediaTransportControls_IsFullWindowButtonVisible = 902,
    XamlPropertyIndex_MediaTransportControls_IsPlaybackRateButtonVisible = 904,
    XamlPropertyIndex_MediaTransportControls_IsSeekBarVisible = 905,
    XamlPropertyIndex_MediaTransportControls_IsStopButtonVisible = 908,
    XamlPropertyIndex_MediaTransportControls_IsVolumeButtonVisible = 910,
    XamlPropertyIndex_MediaTransportControls_IsZoomButtonVisible = 912,
    XamlPropertyIndex_PasswordBox_Header = 913,
    XamlPropertyIndex_PasswordBox_HeaderTemplate = 914,
    XamlPropertyIndex_PasswordBox_IsPasswordRevealButtonEnabled = 915,
    XamlPropertyIndex_PasswordBox_MaxLength = 916,
    XamlPropertyIndex_PasswordBox_Password = 917,
    XamlPropertyIndex_PasswordBox_PasswordChar = 918,
    XamlPropertyIndex_PasswordBox_PlaceholderText = 919,
    XamlPropertyIndex_PasswordBox_PreventKeyboardDisplayOnProgrammaticFocus = 920,
    XamlPropertyIndex_PasswordBox_SelectionHighlightColor = 921,
    XamlPropertyIndex_Path_Data = 922,
    XamlPropertyIndex_PathIcon_Data = 923,
    XamlPropertyIndex_Polygon_FillRule = 924,
    XamlPropertyIndex_Polygon_Points = 925,
    XamlPropertyIndex_Polyline_FillRule = 926,
    XamlPropertyIndex_Polyline_Points = 927,
    XamlPropertyIndex_ProgressRing_IsActive = 928,
    XamlPropertyIndex_ProgressRing_TemplateSettings = 929,
    XamlPropertyIndex_RangeBase_LargeChange = 930,
    XamlPropertyIndex_RangeBase_Maximum = 931,
    XamlPropertyIndex_RangeBase_Minimum = 932,
    XamlPropertyIndex_RangeBase_SmallChange = 933,
    XamlPropertyIndex_RangeBase_Value = 934,
    XamlPropertyIndex_Rectangle_RadiusX = 935,
    XamlPropertyIndex_Rectangle_RadiusY = 936,
    XamlPropertyIndex_RichEditBox_AcceptsReturn = 937,
    XamlPropertyIndex_RichEditBox_Header = 938,
    XamlPropertyIndex_RichEditBox_HeaderTemplate = 939,
    XamlPropertyIndex_RichEditBox_InputScope = 940,
    XamlPropertyIndex_RichEditBox_IsColorFontEnabled = 941,
    XamlPropertyIndex_RichEditBox_IsReadOnly = 942,
    XamlPropertyIndex_RichEditBox_IsSpellCheckEnabled = 943,
    XamlPropertyIndex_RichEditBox_IsTextPredictionEnabled = 944,
    XamlPropertyIndex_RichEditBox_PlaceholderText = 945,
    XamlPropertyIndex_RichEditBox_PreventKeyboardDisplayOnProgrammaticFocus = 946,
    XamlPropertyIndex_RichEditBox_SelectionHighlightColor = 947,
    XamlPropertyIndex_RichEditBox_TextAlignment = 948,
    XamlPropertyIndex_RichEditBox_TextWrapping = 949,
    XamlPropertyIndex_SearchBox_ChooseSuggestionOnEnter = 950,
    XamlPropertyIndex_SearchBox_FocusOnKeyboardInput = 951,
    XamlPropertyIndex_SearchBox_PlaceholderText = 952,
    XamlPropertyIndex_SearchBox_QueryText = 953,
    XamlPropertyIndex_SearchBox_SearchHistoryContext = 954,
    XamlPropertyIndex_SearchBox_SearchHistoryEnabled = 955,
    XamlPropertyIndex_SemanticZoom_CanChangeViews = 956,
    XamlPropertyIndex_SemanticZoom_IsZoomedInViewActive = 957,
    XamlPropertyIndex_SemanticZoom_IsZoomOutButtonEnabled = 958,
    XamlPropertyIndex_SemanticZoom_ZoomedInView = 959,
    XamlPropertyIndex_SemanticZoom_ZoomedOutView = 960,
    XamlPropertyIndex_StackPanel_AreScrollSnapPointsRegular = 961,
    XamlPropertyIndex_StackPanel_Orientation = 962,
    XamlPropertyIndex_SymbolIcon_Symbol = 963,
    XamlPropertyIndex_TextBox_AcceptsReturn = 964,
    XamlPropertyIndex_TextBox_Header = 965,
    XamlPropertyIndex_TextBox_HeaderTemplate = 966,
    XamlPropertyIndex_TextBox_InputScope = 967,
    XamlPropertyIndex_TextBox_IsColorFontEnabled = 968,
    XamlPropertyIndex_TextBox_IsReadOnly = 971,
    XamlPropertyIndex_TextBox_IsSpellCheckEnabled = 972,
    XamlPropertyIndex_TextBox_IsTextPredictionEnabled = 973,
    XamlPropertyIndex_TextBox_MaxLength = 974,
    XamlPropertyIndex_TextBox_PlaceholderText = 975,
    XamlPropertyIndex_TextBox_PreventKeyboardDisplayOnProgrammaticFocus = 976,
    XamlPropertyIndex_TextBox_SelectedText = 977,
    XamlPropertyIndex_TextBox_SelectionHighlightColor = 978,
    XamlPropertyIndex_TextBox_SelectionLength = 979,
    XamlPropertyIndex_TextBox_SelectionStart = 980,
    XamlPropertyIndex_TextBox_Text = 981,
    XamlPropertyIndex_TextBox_TextAlignment = 982,
    XamlPropertyIndex_TextBox_TextWrapping = 983,
    XamlPropertyIndex_Thumb_IsDragging = 984,
    XamlPropertyIndex_TickBar_Fill = 985,
    XamlPropertyIndex_TimePicker_ClockIdentifier = 986,
    XamlPropertyIndex_TimePicker_Header = 987,
    XamlPropertyIndex_TimePicker_HeaderTemplate = 988,
    XamlPropertyIndex_TimePicker_MinuteIncrement = 989,
    XamlPropertyIndex_TimePicker_Time = 990,
    XamlPropertyIndex_ToggleSwitch_Header = 991,
    XamlPropertyIndex_ToggleSwitch_HeaderTemplate = 992,
    XamlPropertyIndex_ToggleSwitch_IsOn = 993,
    XamlPropertyIndex_ToggleSwitch_OffContent = 994,
    XamlPropertyIndex_ToggleSwitch_OffContentTemplate = 995,
    XamlPropertyIndex_ToggleSwitch_OnContent = 996,
    XamlPropertyIndex_ToggleSwitch_OnContentTemplate = 997,
    XamlPropertyIndex_ToggleSwitch_TemplateSettings = 998,
    XamlPropertyIndex_UserControl_Content = 999,
    XamlPropertyIndex_VariableSizedWrapGrid_ColumnSpan = 1000,
    XamlPropertyIndex_VariableSizedWrapGrid_HorizontalChildrenAlignment = 1001,
    XamlPropertyIndex_VariableSizedWrapGrid_ItemHeight = 1002,
    XamlPropertyIndex_VariableSizedWrapGrid_ItemWidth = 1003,
    XamlPropertyIndex_VariableSizedWrapGrid_MaximumRowsOrColumns = 1004,
    XamlPropertyIndex_VariableSizedWrapGrid_Orientation = 1005,
    XamlPropertyIndex_VariableSizedWrapGrid_RowSpan = 1006,
    XamlPropertyIndex_VariableSizedWrapGrid_VerticalChildrenAlignment = 1007,
    XamlPropertyIndex_WebView_AllowedScriptNotifyUris = 1008,
    XamlPropertyIndex_WebView_CanGoBack = 1009,
    XamlPropertyIndex_WebView_CanGoForward = 1010,
    XamlPropertyIndex_WebView_ContainsFullScreenElement = 1011,
    XamlPropertyIndex_WebView_DataTransferPackage = 1012,
    XamlPropertyIndex_WebView_DefaultBackgroundColor = 1013,
    XamlPropertyIndex_WebView_DocumentTitle = 1014,
    XamlPropertyIndex_WebView_Source = 1015,
    XamlPropertyIndex_AppBar_ClosedDisplayMode = 1016,
    XamlPropertyIndex_AppBar_IsOpen = 1017,
    XamlPropertyIndex_AppBar_IsSticky = 1018,
    XamlPropertyIndex_AutoSuggestBox_AutoMaximizeSuggestionArea = 1019,
    XamlPropertyIndex_AutoSuggestBox_Header = 1020,
    XamlPropertyIndex_AutoSuggestBox_IsSuggestionListOpen = 1021,
    XamlPropertyIndex_AutoSuggestBox_MaxSuggestionListHeight = 1022,
    XamlPropertyIndex_AutoSuggestBox_PlaceholderText = 1023,
    XamlPropertyIndex_AutoSuggestBox_Text = 1024,
    XamlPropertyIndex_AutoSuggestBox_TextBoxStyle = 1025,
    XamlPropertyIndex_AutoSuggestBox_TextMemberPath = 1026,
    XamlPropertyIndex_AutoSuggestBox_UpdateTextOnSelect = 1027,
    XamlPropertyIndex_ButtonBase_ClickMode = 1029,
    XamlPropertyIndex_ButtonBase_Command = 1030,
    XamlPropertyIndex_ButtonBase_CommandParameter = 1031,
    XamlPropertyIndex_ButtonBase_IsPointerOver = 1032,
    XamlPropertyIndex_ButtonBase_IsPressed = 1033,
    XamlPropertyIndex_ContentDialog_FullSizeDesired = 1034,
    XamlPropertyIndex_ContentDialog_IsPrimaryButtonEnabled = 1035,
    XamlPropertyIndex_ContentDialog_IsSecondaryButtonEnabled = 1036,
    XamlPropertyIndex_ContentDialog_PrimaryButtonCommand = 1037,
    XamlPropertyIndex_ContentDialog_PrimaryButtonCommandParameter = 1038,
    XamlPropertyIndex_ContentDialog_PrimaryButtonText = 1039,
    XamlPropertyIndex_ContentDialog_SecondaryButtonCommand = 1040,
    XamlPropertyIndex_ContentDialog_SecondaryButtonCommandParameter = 1041,
    XamlPropertyIndex_ContentDialog_SecondaryButtonText = 1042,
    XamlPropertyIndex_ContentDialog_Title = 1043,
    XamlPropertyIndex_ContentDialog_TitleTemplate = 1044,
    XamlPropertyIndex_Frame_BackStack = 1045,
    XamlPropertyIndex_Frame_BackStackDepth = 1046,
    XamlPropertyIndex_Frame_CacheSize = 1047,
    XamlPropertyIndex_Frame_CanGoBack = 1048,
    XamlPropertyIndex_Frame_CanGoForward = 1049,
    XamlPropertyIndex_Frame_CurrentSourcePageType = 1050,
    XamlPropertyIndex_Frame_ForwardStack = 1051,
    XamlPropertyIndex_Frame_SourcePageType = 1052,
    XamlPropertyIndex_GridViewItemPresenter_CheckBrush = 1053,
    XamlPropertyIndex_GridViewItemPresenter_CheckHintBrush = 1054,
    XamlPropertyIndex_GridViewItemPresenter_CheckSelectingBrush = 1055,
    XamlPropertyIndex_GridViewItemPresenter_ContentMargin = 1056,
    XamlPropertyIndex_GridViewItemPresenter_DisabledOpacity = 1057,
    XamlPropertyIndex_GridViewItemPresenter_DragBackground = 1058,
    XamlPropertyIndex_GridViewItemPresenter_DragForeground = 1059,
    XamlPropertyIndex_GridViewItemPresenter_DragOpacity = 1060,
    XamlPropertyIndex_GridViewItemPresenter_FocusBorderBrush = 1061,
    XamlPropertyIndex_GridViewItemPresenter_GridViewItemPresenterHorizontalContentAlignment = 1062,
    XamlPropertyIndex_GridViewItemPresenter_GridViewItemPresenterPadding = 1063,
    XamlPropertyIndex_GridViewItemPresenter_PlaceholderBackground = 1064,
    XamlPropertyIndex_GridViewItemPresenter_PointerOverBackground = 1065,
    XamlPropertyIndex_GridViewItemPresenter_PointerOverBackgroundMargin = 1066,
    XamlPropertyIndex_GridViewItemPresenter_ReorderHintOffset = 1067,
    XamlPropertyIndex_GridViewItemPresenter_SelectedBackground = 1068,
    XamlPropertyIndex_GridViewItemPresenter_SelectedBorderThickness = 1069,
    XamlPropertyIndex_GridViewItemPresenter_SelectedForeground = 1070,
    XamlPropertyIndex_GridViewItemPresenter_SelectedPointerOverBackground = 1071,
    XamlPropertyIndex_GridViewItemPresenter_SelectedPointerOverBorderBrush = 1072,
    XamlPropertyIndex_GridViewItemPresenter_SelectionCheckMarkVisualEnabled = 1073,
    XamlPropertyIndex_GridViewItemPresenter_GridViewItemPresenterVerticalContentAlignment = 1074,
    XamlPropertyIndex_ItemsStackPanel_CacheLength = 1076,
    XamlPropertyIndex_ItemsStackPanel_GroupHeaderPlacement = 1077,
    XamlPropertyIndex_ItemsStackPanel_GroupPadding = 1078,
    XamlPropertyIndex_ItemsStackPanel_ItemsUpdatingScrollMode = 1079,
    XamlPropertyIndex_ItemsStackPanel_Orientation = 1080,
    XamlPropertyIndex_ItemsWrapGrid_CacheLength = 1081,
    XamlPropertyIndex_ItemsWrapGrid_GroupHeaderPlacement = 1082,
    XamlPropertyIndex_ItemsWrapGrid_GroupPadding = 1083,
    XamlPropertyIndex_ItemsWrapGrid_ItemHeight = 1084,
    XamlPropertyIndex_ItemsWrapGrid_ItemWidth = 1085,
    XamlPropertyIndex_ItemsWrapGrid_MaximumRowsOrColumns = 1086,
    XamlPropertyIndex_ItemsWrapGrid_Orientation = 1087,
    XamlPropertyIndex_ListViewItemPresenter_CheckBrush = 1088,
    XamlPropertyIndex_ListViewItemPresenter_CheckHintBrush = 1089,
    XamlPropertyIndex_ListViewItemPresenter_CheckSelectingBrush = 1090,
    XamlPropertyIndex_ListViewItemPresenter_ContentMargin = 1091,
    XamlPropertyIndex_ListViewItemPresenter_DisabledOpacity = 1092,
    XamlPropertyIndex_ListViewItemPresenter_DragBackground = 1093,
    XamlPropertyIndex_ListViewItemPresenter_DragForeground = 1094,
    XamlPropertyIndex_ListViewItemPresenter_DragOpacity = 1095,
    XamlPropertyIndex_ListViewItemPresenter_FocusBorderBrush = 1096,
    XamlPropertyIndex_ListViewItemPresenter_ListViewItemPresenterHorizontalContentAlignment = 1097,
    XamlPropertyIndex_ListViewItemPresenter_ListViewItemPresenterPadding = 1098,
    XamlPropertyIndex_ListViewItemPresenter_PlaceholderBackground = 1099,
    XamlPropertyIndex_ListViewItemPresenter_PointerOverBackground = 1100,
    XamlPropertyIndex_ListViewItemPresenter_PointerOverBackgroundMargin = 1101,
    XamlPropertyIndex_ListViewItemPresenter_ReorderHintOffset = 1102,
    XamlPropertyIndex_ListViewItemPresenter_SelectedBackground = 1103,
    XamlPropertyIndex_ListViewItemPresenter_SelectedBorderThickness = 1104,
    XamlPropertyIndex_ListViewItemPresenter_SelectedForeground = 1105,
    XamlPropertyIndex_ListViewItemPresenter_SelectedPointerOverBackground = 1106,
    XamlPropertyIndex_ListViewItemPresenter_SelectedPointerOverBorderBrush = 1107,
    XamlPropertyIndex_ListViewItemPresenter_SelectionCheckMarkVisualEnabled = 1108,
    XamlPropertyIndex_ListViewItemPresenter_ListViewItemPresenterVerticalContentAlignment = 1109,
    XamlPropertyIndex_MenuFlyoutItem_Command = 1110,
    XamlPropertyIndex_MenuFlyoutItem_CommandParameter = 1111,
    XamlPropertyIndex_MenuFlyoutItem_Text = 1112,
    XamlPropertyIndex_Page_BottomAppBar = 1114,
    XamlPropertyIndex_Page_Frame = 1115,
    XamlPropertyIndex_Page_NavigationCacheMode = 1116,
    XamlPropertyIndex_Page_TopAppBar = 1117,
    XamlPropertyIndex_ProgressBar_IsIndeterminate = 1118,
    XamlPropertyIndex_ProgressBar_ShowError = 1119,
    XamlPropertyIndex_ProgressBar_ShowPaused = 1120,
    XamlPropertyIndex_ProgressBar_TemplateSettings = 1121,
    XamlPropertyIndex_ScrollBar_IndicatorMode = 1122,
    XamlPropertyIndex_ScrollBar_Orientation = 1123,
    XamlPropertyIndex_ScrollBar_ViewportSize = 1124,
    XamlPropertyIndex_Selector_IsSynchronizedWithCurrentItem = 1126,
    XamlPropertyIndex_Selector_SelectedIndex = 1127,
    XamlPropertyIndex_Selector_SelectedItem = 1128,
    XamlPropertyIndex_Selector_SelectedValue = 1129,
    XamlPropertyIndex_Selector_SelectedValuePath = 1130,
    XamlPropertyIndex_SelectorItem_IsSelected = 1131,
    XamlPropertyIndex_SettingsFlyout_HeaderBackground = 1132,
    XamlPropertyIndex_SettingsFlyout_HeaderForeground = 1133,
    XamlPropertyIndex_SettingsFlyout_IconSource = 1134,
    XamlPropertyIndex_SettingsFlyout_TemplateSettings = 1135,
    XamlPropertyIndex_SettingsFlyout_Title = 1136,
    XamlPropertyIndex_Slider_Header = 1137,
    XamlPropertyIndex_Slider_HeaderTemplate = 1138,
    XamlPropertyIndex_Slider_IntermediateValue = 1139,
    XamlPropertyIndex_Slider_IsDirectionReversed = 1140,
    XamlPropertyIndex_Slider_IsThumbToolTipEnabled = 1141,
    XamlPropertyIndex_Slider_Orientation = 1142,
    XamlPropertyIndex_Slider_SnapsTo = 1143,
    XamlPropertyIndex_Slider_StepFrequency = 1144,
    XamlPropertyIndex_Slider_ThumbToolTipValueConverter = 1145,
    XamlPropertyIndex_Slider_TickFrequency = 1146,
    XamlPropertyIndex_Slider_TickPlacement = 1147,
    XamlPropertyIndex_SwapChainPanel_CompositionScaleX = 1148,
    XamlPropertyIndex_SwapChainPanel_CompositionScaleY = 1149,
    XamlPropertyIndex_ToolTip_HorizontalOffset = 1150,
    XamlPropertyIndex_ToolTip_IsOpen = 1151,
    XamlPropertyIndex_ToolTip_Placement = 1152,
    XamlPropertyIndex_ToolTip_PlacementTarget = 1153,
    XamlPropertyIndex_ToolTip_TemplateSettings = 1154,
    XamlPropertyIndex_ToolTip_VerticalOffset = 1155,
    XamlPropertyIndex_Button_Flyout = 1156,
    XamlPropertyIndex_ComboBox_Header = 1157,
    XamlPropertyIndex_ComboBox_HeaderTemplate = 1158,
    XamlPropertyIndex_ComboBox_IsDropDownOpen = 1159,
    XamlPropertyIndex_ComboBox_IsEditable = 1160,
    XamlPropertyIndex_ComboBox_IsSelectionBoxHighlighted = 1161,
    XamlPropertyIndex_ComboBox_MaxDropDownHeight = 1162,
    XamlPropertyIndex_ComboBox_PlaceholderText = 1163,
    XamlPropertyIndex_ComboBox_SelectionBoxItem = 1164,
    XamlPropertyIndex_ComboBox_SelectionBoxItemTemplate = 1165,
    XamlPropertyIndex_ComboBox_TemplateSettings = 1166,
    XamlPropertyIndex_CommandBar_PrimaryCommands = 1167,
    XamlPropertyIndex_CommandBar_SecondaryCommands = 1168,
    XamlPropertyIndex_FlipView_UseTouchAnimationsForAllNavigation = 1169,
    XamlPropertyIndex_HyperlinkButton_NavigateUri = 1170,
    XamlPropertyIndex_ListBox_SelectedItems = 1171,
    XamlPropertyIndex_ListBox_SelectionMode = 1172,
    XamlPropertyIndex_ListViewBase_CanDragItems = 1173,
    XamlPropertyIndex_ListViewBase_CanReorderItems = 1174,
    XamlPropertyIndex_ListViewBase_DataFetchSize = 1175,
    XamlPropertyIndex_ListViewBase_Footer = 1176,
    XamlPropertyIndex_ListViewBase_FooterTemplate = 1177,
    XamlPropertyIndex_ListViewBase_FooterTransitions = 1178,
    XamlPropertyIndex_ListViewBase_Header = 1179,
    XamlPropertyIndex_ListViewBase_HeaderTemplate = 1180,
    XamlPropertyIndex_ListViewBase_HeaderTransitions = 1181,
    XamlPropertyIndex_ListViewBase_IncrementalLoadingThreshold = 1182,
    XamlPropertyIndex_ListViewBase_IncrementalLoadingTrigger = 1183,
    XamlPropertyIndex_ListViewBase_IsActiveView = 1184,
    XamlPropertyIndex_ListViewBase_IsItemClickEnabled = 1185,
    XamlPropertyIndex_ListViewBase_IsSwipeEnabled = 1186,
    XamlPropertyIndex_ListViewBase_IsZoomedInView = 1187,
    XamlPropertyIndex_ListViewBase_ReorderMode = 1188,
    XamlPropertyIndex_ListViewBase_SelectedItems = 1189,
    XamlPropertyIndex_ListViewBase_SelectionMode = 1190,
    XamlPropertyIndex_ListViewBase_SemanticZoomOwner = 1191,
    XamlPropertyIndex_ListViewBase_ShowsScrollingPlaceholders = 1192,
    XamlPropertyIndex_RepeatButton_Delay = 1193,
    XamlPropertyIndex_RepeatButton_Interval = 1194,
    XamlPropertyIndex_ScrollViewer_BringIntoViewOnFocusChange = 1195,
    XamlPropertyIndex_ScrollViewer_ComputedHorizontalScrollBarVisibility = 1196,
    XamlPropertyIndex_ScrollViewer_ComputedVerticalScrollBarVisibility = 1197,
    XamlPropertyIndex_ScrollViewer_ExtentHeight = 1198,
    XamlPropertyIndex_ScrollViewer_ExtentWidth = 1199,
    XamlPropertyIndex_ScrollViewer_HorizontalOffset = 1200,
    XamlPropertyIndex_ScrollViewer_HorizontalScrollBarVisibility = 1201,
    XamlPropertyIndex_ScrollViewer_HorizontalScrollMode = 1202,
    XamlPropertyIndex_ScrollViewer_HorizontalSnapPointsAlignment = 1203,
    XamlPropertyIndex_ScrollViewer_HorizontalSnapPointsType = 1204,
    XamlPropertyIndex_ScrollViewer_IsDeferredScrollingEnabled = 1205,
    XamlPropertyIndex_ScrollViewer_IsHorizontalRailEnabled = 1206,
    XamlPropertyIndex_ScrollViewer_IsHorizontalScrollChainingEnabled = 1207,
    XamlPropertyIndex_ScrollViewer_IsScrollInertiaEnabled = 1208,
    XamlPropertyIndex_ScrollViewer_IsVerticalRailEnabled = 1209,
    XamlPropertyIndex_ScrollViewer_IsVerticalScrollChainingEnabled = 1210,
    XamlPropertyIndex_ScrollViewer_IsZoomChainingEnabled = 1211,
    XamlPropertyIndex_ScrollViewer_IsZoomInertiaEnabled = 1212,
    XamlPropertyIndex_ScrollViewer_LeftHeader = 1213,
    XamlPropertyIndex_ScrollViewer_MaxZoomFactor = 1214,
    XamlPropertyIndex_ScrollViewer_MinZoomFactor = 1215,
    XamlPropertyIndex_ScrollViewer_ScrollableHeight = 1216,
    XamlPropertyIndex_ScrollViewer_ScrollableWidth = 1217,
    XamlPropertyIndex_ScrollViewer_TopHeader = 1218,
    XamlPropertyIndex_ScrollViewer_TopLeftHeader = 1219,
    XamlPropertyIndex_ScrollViewer_VerticalOffset = 1220,
    XamlPropertyIndex_ScrollViewer_VerticalScrollBarVisibility = 1221,
    XamlPropertyIndex_ScrollViewer_VerticalScrollMode = 1222,
    XamlPropertyIndex_ScrollViewer_VerticalSnapPointsAlignment = 1223,
    XamlPropertyIndex_ScrollViewer_VerticalSnapPointsType = 1224,
    XamlPropertyIndex_ScrollViewer_ViewportHeight = 1225,
    XamlPropertyIndex_ScrollViewer_ViewportWidth = 1226,
    XamlPropertyIndex_ScrollViewer_ZoomFactor = 1227,
    XamlPropertyIndex_ScrollViewer_ZoomMode = 1228,
    XamlPropertyIndex_ScrollViewer_ZoomSnapPoints = 1229,
    XamlPropertyIndex_ScrollViewer_ZoomSnapPointsType = 1230,
    XamlPropertyIndex_ToggleButton_IsChecked = 1231,
    XamlPropertyIndex_ToggleButton_IsThreeState = 1232,
    XamlPropertyIndex_ToggleMenuFlyoutItem_IsChecked = 1233,
    XamlPropertyIndex_VirtualizingStackPanel_AreScrollSnapPointsRegular = 1234,
    XamlPropertyIndex_VirtualizingStackPanel_IsVirtualizing = 1236,
    XamlPropertyIndex_VirtualizingStackPanel_Orientation = 1237,
    XamlPropertyIndex_VirtualizingStackPanel_VirtualizationMode = 1238,
    XamlPropertyIndex_WrapGrid_HorizontalChildrenAlignment = 1239,
    XamlPropertyIndex_WrapGrid_ItemHeight = 1240,
    XamlPropertyIndex_WrapGrid_ItemWidth = 1241,
    XamlPropertyIndex_WrapGrid_MaximumRowsOrColumns = 1242,
    XamlPropertyIndex_WrapGrid_Orientation = 1243,
    XamlPropertyIndex_WrapGrid_VerticalChildrenAlignment = 1244,
    XamlPropertyIndex_AppBarButton_Icon = 1245,
    XamlPropertyIndex_AppBarButton_IsCompact = 1246,
    XamlPropertyIndex_AppBarButton_Label = 1247,
    XamlPropertyIndex_AppBarToggleButton_Icon = 1248,
    XamlPropertyIndex_AppBarToggleButton_IsCompact = 1249,
    XamlPropertyIndex_AppBarToggleButton_Label = 1250,
    XamlPropertyIndex_GridViewItem_TemplateSettings = 1251,
    XamlPropertyIndex_ListViewItem_TemplateSettings = 1252,
    XamlPropertyIndex_RadioButton_GroupName = 1253,
    XamlPropertyIndex_Glyphs_ColorFontPaletteIndex = 1267,
    XamlPropertyIndex_Glyphs_IsColorFontEnabled = 1268,
    XamlPropertyIndex_CalendarViewTemplateSettings_HasMoreContentAfter = 1274,
    XamlPropertyIndex_CalendarViewTemplateSettings_HasMoreContentBefore = 1275,
    XamlPropertyIndex_CalendarViewTemplateSettings_HasMoreViews = 1276,
    XamlPropertyIndex_CalendarViewTemplateSettings_HeaderText = 1277,
    XamlPropertyIndex_CalendarViewTemplateSettings_WeekDay1 = 1280,
    XamlPropertyIndex_CalendarViewTemplateSettings_WeekDay2 = 1281,
    XamlPropertyIndex_CalendarViewTemplateSettings_WeekDay3 = 1282,
    XamlPropertyIndex_CalendarViewTemplateSettings_WeekDay4 = 1283,
    XamlPropertyIndex_CalendarViewTemplateSettings_WeekDay5 = 1284,
    XamlPropertyIndex_CalendarViewTemplateSettings_WeekDay6 = 1285,
    XamlPropertyIndex_CalendarViewTemplateSettings_WeekDay7 = 1286,
    XamlPropertyIndex_CalendarView_CalendarIdentifier = 1291,
    XamlPropertyIndex_CalendarView_DayOfWeekFormat = 1299,
    XamlPropertyIndex_CalendarView_DisplayMode = 1302,
    XamlPropertyIndex_CalendarView_FirstDayOfWeek = 1303,
    XamlPropertyIndex_CalendarView_IsOutOfScopeEnabled = 1317,
    XamlPropertyIndex_CalendarView_IsTodayHighlighted = 1318,
    XamlPropertyIndex_CalendarView_MaxDate = 1320,
    XamlPropertyIndex_CalendarView_MinDate = 1321,
    XamlPropertyIndex_CalendarView_NumberOfWeeksInView = 1327,
    XamlPropertyIndex_CalendarView_SelectedDates = 1333,
    XamlPropertyIndex_CalendarView_SelectionMode = 1335,
    XamlPropertyIndex_CalendarView_TemplateSettings = 1336,
    XamlPropertyIndex_CalendarViewDayItem_Date = 1339,
    XamlPropertyIndex_CalendarViewDayItem_IsBlackout = 1340,
    XamlPropertyIndex_MediaTransportControls_IsFastForwardEnabled = 1382,
    XamlPropertyIndex_MediaTransportControls_IsFastRewindEnabled = 1383,
    XamlPropertyIndex_MediaTransportControls_IsFullWindowEnabled = 1384,
    XamlPropertyIndex_MediaTransportControls_IsPlaybackRateEnabled = 1385,
    XamlPropertyIndex_MediaTransportControls_IsSeekEnabled = 1386,
    XamlPropertyIndex_MediaTransportControls_IsStopEnabled = 1387,
    XamlPropertyIndex_MediaTransportControls_IsVolumeEnabled = 1388,
    XamlPropertyIndex_MediaTransportControls_IsZoomEnabled = 1389,
    XamlPropertyIndex_ContentPresenter_LineHeight = 1425,
    XamlPropertyIndex_CalendarViewTemplateSettings_MinViewWidth = 1435,
    XamlPropertyIndex_ListViewBase_SelectedRanges = 1459,
    XamlPropertyIndex_SplitViewTemplateSettings_CompactPaneGridLength = 1462,
    XamlPropertyIndex_SplitViewTemplateSettings_NegativeOpenPaneLength = 1463,
    XamlPropertyIndex_SplitViewTemplateSettings_NegativeOpenPaneLengthMinusCompactLength = 1464,
    XamlPropertyIndex_SplitViewTemplateSettings_OpenPaneGridLength = 1465,
    XamlPropertyIndex_SplitViewTemplateSettings_OpenPaneLengthMinusCompactLength = 1466,
    XamlPropertyIndex_SplitView_CompactPaneLength = 1467,
    XamlPropertyIndex_SplitView_Content = 1468,
    XamlPropertyIndex_SplitView_DisplayMode = 1469,
    XamlPropertyIndex_SplitView_IsPaneOpen = 1470,
    XamlPropertyIndex_SplitView_OpenPaneLength = 1471,
    XamlPropertyIndex_SplitView_Pane = 1472,
    XamlPropertyIndex_SplitView_PanePlacement = 1473,
    XamlPropertyIndex_SplitView_TemplateSettings = 1474,
    XamlPropertyIndex_UIElement_Transform3D = 1475,
    XamlPropertyIndex_CompositeTransform3D_CenterX = 1476,
    XamlPropertyIndex_CompositeTransform3D_CenterY = 1478,
    XamlPropertyIndex_CompositeTransform3D_CenterZ = 1480,
    XamlPropertyIndex_CompositeTransform3D_RotationX = 1482,
    XamlPropertyIndex_CompositeTransform3D_RotationY = 1484,
    XamlPropertyIndex_CompositeTransform3D_RotationZ = 1486,
    XamlPropertyIndex_CompositeTransform3D_ScaleX = 1488,
    XamlPropertyIndex_CompositeTransform3D_ScaleY = 1490,
    XamlPropertyIndex_CompositeTransform3D_ScaleZ = 1492,
    XamlPropertyIndex_CompositeTransform3D_TranslateX = 1494,
    XamlPropertyIndex_CompositeTransform3D_TranslateY = 1496,
    XamlPropertyIndex_CompositeTransform3D_TranslateZ = 1498,
    XamlPropertyIndex_PerspectiveTransform3D_Depth = 1500,
    XamlPropertyIndex_PerspectiveTransform3D_OffsetX = 1501,
    XamlPropertyIndex_PerspectiveTransform3D_OffsetY = 1502,
    XamlPropertyIndex_RelativePanel_Above = 1508,
    XamlPropertyIndex_RelativePanel_AlignBottomWith = 1509,
    XamlPropertyIndex_RelativePanel_AlignLeftWith = 1510,
    XamlPropertyIndex_RelativePanel_AlignRightWith = 1515,
    XamlPropertyIndex_RelativePanel_AlignTopWith = 1516,
    XamlPropertyIndex_RelativePanel_Below = 1517,
    XamlPropertyIndex_RelativePanel_LeftOf = 1520,
    XamlPropertyIndex_RelativePanel_RightOf = 1521,
    XamlPropertyIndex_SplitViewTemplateSettings_OpenPaneLength = 1524,
    XamlPropertyIndex_PasswordBox_PasswordRevealMode = 1527,
    XamlPropertyIndex_SplitView_PaneBackground = 1528,
    XamlPropertyIndex_ItemsStackPanel_AreStickyGroupHeadersEnabled = 1529,
    XamlPropertyIndex_ItemsWrapGrid_AreStickyGroupHeadersEnabled = 1530,
    XamlPropertyIndex_MenuFlyoutSubItem_Items = 1531,
    XamlPropertyIndex_MenuFlyoutSubItem_Text = 1532,
    XamlPropertyIndex_UIElement_CanDrag = 1534,
    XamlPropertyIndex_DataTemplate_ExtensionInstance = 1535,
    XamlPropertyIndex_RelativePanel_AlignHorizontalCenterWith = 1552,
    XamlPropertyIndex_RelativePanel_AlignVerticalCenterWith = 1553,
    XamlPropertyIndex_TargetPropertyPath_Path = 1555,
    XamlPropertyIndex_TargetPropertyPath_Target = 1556,
    XamlPropertyIndex_VisualState_Setters = 1558,
    XamlPropertyIndex_VisualState_StateTriggers = 1559,
    XamlPropertyIndex_AdaptiveTrigger_MinWindowHeight = 1560,
    XamlPropertyIndex_AdaptiveTrigger_MinWindowWidth = 1561,
    XamlPropertyIndex_Setter_Target = 1562,
    XamlPropertyIndex_CalendarView_BlackoutForeground = 1565,
    XamlPropertyIndex_CalendarView_CalendarItemBackground = 1566,
    XamlPropertyIndex_CalendarView_CalendarItemBorderBrush = 1567,
    XamlPropertyIndex_CalendarView_CalendarItemBorderThickness = 1568,
    XamlPropertyIndex_CalendarView_CalendarItemForeground = 1569,
    XamlPropertyIndex_CalendarView_CalendarViewDayItemStyle = 1570,
    XamlPropertyIndex_CalendarView_DayItemFontFamily = 1571,
    XamlPropertyIndex_CalendarView_DayItemFontSize = 1572,
    XamlPropertyIndex_CalendarView_DayItemFontStyle = 1573,
    XamlPropertyIndex_CalendarView_DayItemFontWeight = 1574,
    XamlPropertyIndex_CalendarView_FirstOfMonthLabelFontFamily = 1575,
    XamlPropertyIndex_CalendarView_FirstOfMonthLabelFontSize = 1576,
    XamlPropertyIndex_CalendarView_FirstOfMonthLabelFontStyle = 1577,
    XamlPropertyIndex_CalendarView_FirstOfMonthLabelFontWeight = 1578,
    XamlPropertyIndex_CalendarView_FirstOfYearDecadeLabelFontFamily = 1579,
    XamlPropertyIndex_CalendarView_FirstOfYearDecadeLabelFontSize = 1580,
    XamlPropertyIndex_CalendarView_FirstOfYearDecadeLabelFontStyle = 1581,
    XamlPropertyIndex_CalendarView_FirstOfYearDecadeLabelFontWeight = 1582,
    XamlPropertyIndex_CalendarView_FocusBorderBrush = 1583,
    XamlPropertyIndex_CalendarView_HorizontalDayItemAlignment = 1584,
    XamlPropertyIndex_CalendarView_HorizontalFirstOfMonthLabelAlignment = 1585,
    XamlPropertyIndex_CalendarView_HoverBorderBrush = 1586,
    XamlPropertyIndex_CalendarView_MonthYearItemFontFamily = 1588,
    XamlPropertyIndex_CalendarView_MonthYearItemFontSize = 1589,
    XamlPropertyIndex_CalendarView_MonthYearItemFontStyle = 1590,
    XamlPropertyIndex_CalendarView_MonthYearItemFontWeight = 1591,
    XamlPropertyIndex_CalendarView_OutOfScopeBackground = 1592,
    XamlPropertyIndex_CalendarView_OutOfScopeForeground = 1593,
    XamlPropertyIndex_CalendarView_PressedBorderBrush = 1594,
    XamlPropertyIndex_CalendarView_PressedForeground = 1595,
    XamlPropertyIndex_CalendarView_SelectedBorderBrush = 1596,
    XamlPropertyIndex_CalendarView_SelectedForeground = 1597,
    XamlPropertyIndex_CalendarView_SelectedHoverBorderBrush = 1598,
    XamlPropertyIndex_CalendarView_SelectedPressedBorderBrush = 1599,
    XamlPropertyIndex_CalendarView_TodayFontWeight = 1600,
    XamlPropertyIndex_CalendarView_TodayForeground = 1601,
    XamlPropertyIndex_CalendarView_VerticalDayItemAlignment = 1602,
    XamlPropertyIndex_CalendarView_VerticalFirstOfMonthLabelAlignment = 1603,
    XamlPropertyIndex_MediaTransportControls_IsCompact = 1605,
    XamlPropertyIndex_RelativePanel_AlignBottomWithPanel = 1606,
    XamlPropertyIndex_RelativePanel_AlignHorizontalCenterWithPanel = 1607,
    XamlPropertyIndex_RelativePanel_AlignLeftWithPanel = 1608,
    XamlPropertyIndex_RelativePanel_AlignRightWithPanel = 1609,
    XamlPropertyIndex_RelativePanel_AlignTopWithPanel = 1610,
    XamlPropertyIndex_RelativePanel_AlignVerticalCenterWithPanel = 1611,
    XamlPropertyIndex_ListViewBase_IsMultiSelectCheckBoxEnabled = 1612,
    XamlPropertyIndex_AutomationProperties_Level = 1614,
    XamlPropertyIndex_AutomationProperties_PositionInSet = 1615,
    XamlPropertyIndex_AutomationProperties_SizeOfSet = 1616,
    XamlPropertyIndex_ListViewItemPresenter_CheckBoxBrush = 1617,
    XamlPropertyIndex_ListViewItemPresenter_CheckMode = 1618,
    XamlPropertyIndex_ListViewItemPresenter_PressedBackground = 1620,
    XamlPropertyIndex_ListViewItemPresenter_SelectedPressedBackground = 1621,
    XamlPropertyIndex_Control_IsTemplateFocusTarget = 1623,
    XamlPropertyIndex_Control_UseSystemFocusVisuals = 1624,
    XamlPropertyIndex_ListViewItemPresenter_FocusSecondaryBorderBrush = 1628,
    XamlPropertyIndex_ListViewItemPresenter_PointerOverForeground = 1630,
    XamlPropertyIndex_FontIcon_MirroredWhenRightToLeft = 1631,
    XamlPropertyIndex_CalendarViewTemplateSettings_CenterX = 1632,
    XamlPropertyIndex_CalendarViewTemplateSettings_CenterY = 1633,
    XamlPropertyIndex_CalendarViewTemplateSettings_ClipRect = 1634,
    XamlPropertyIndex_PasswordBox_TextReadingOrder = 1650,
    XamlPropertyIndex_RichEditBox_TextReadingOrder = 1651,
    XamlPropertyIndex_TextBox_TextReadingOrder = 1652,
    XamlPropertyIndex_WebView_ExecutionMode = 1653,
    XamlPropertyIndex_WebView_DeferredPermissionRequests = 1655,
    XamlPropertyIndex_WebView_Settings = 1656,
    XamlPropertyIndex_RichEditBox_DesiredCandidateWindowAlignment = 1660,
    XamlPropertyIndex_TextBox_DesiredCandidateWindowAlignment = 1662,
    XamlPropertyIndex_CalendarDatePicker_CalendarIdentifier = 1663,
    XamlPropertyIndex_CalendarDatePicker_CalendarViewStyle = 1664,
    XamlPropertyIndex_CalendarDatePicker_Date = 1665,
    XamlPropertyIndex_CalendarDatePicker_DateFormat = 1666,
    XamlPropertyIndex_CalendarDatePicker_DayOfWeekFormat = 1667,
    XamlPropertyIndex_CalendarDatePicker_DisplayMode = 1668,
    XamlPropertyIndex_CalendarDatePicker_FirstDayOfWeek = 1669,
    XamlPropertyIndex_CalendarDatePicker_Header = 1670,
    XamlPropertyIndex_CalendarDatePicker_HeaderTemplate = 1671,
    XamlPropertyIndex_CalendarDatePicker_IsCalendarOpen = 1672,
    XamlPropertyIndex_CalendarDatePicker_IsGroupLabelVisible = 1673,
    XamlPropertyIndex_CalendarDatePicker_IsOutOfScopeEnabled = 1674,
    XamlPropertyIndex_CalendarDatePicker_IsTodayHighlighted = 1675,
    XamlPropertyIndex_CalendarDatePicker_MaxDate = 1676,
    XamlPropertyIndex_CalendarDatePicker_MinDate = 1677,
    XamlPropertyIndex_CalendarDatePicker_PlaceholderText = 1678,
    XamlPropertyIndex_CalendarView_IsGroupLabelVisible = 1679,
    XamlPropertyIndex_ContentPresenter_Background = 1680,
    XamlPropertyIndex_ContentPresenter_BorderBrush = 1681,
    XamlPropertyIndex_ContentPresenter_BorderThickness = 1682,
    XamlPropertyIndex_ContentPresenter_CornerRadius = 1683,
    XamlPropertyIndex_ContentPresenter_Padding = 1684,
    XamlPropertyIndex_Grid_BorderBrush = 1685,
    XamlPropertyIndex_Grid_BorderThickness = 1686,
    XamlPropertyIndex_Grid_CornerRadius = 1687,
    XamlPropertyIndex_Grid_Padding = 1688,
    XamlPropertyIndex_RelativePanel_BorderBrush = 1689,
    XamlPropertyIndex_RelativePanel_BorderThickness = 1690,
    XamlPropertyIndex_RelativePanel_CornerRadius = 1691,
    XamlPropertyIndex_RelativePanel_Padding = 1692,
    XamlPropertyIndex_StackPanel_BorderBrush = 1693,
    XamlPropertyIndex_StackPanel_BorderThickness = 1694,
    XamlPropertyIndex_StackPanel_CornerRadius = 1695,
    XamlPropertyIndex_StackPanel_Padding = 1696,
    XamlPropertyIndex_PasswordBox_InputScope = 1697,
    XamlPropertyIndex_MediaTransportControlsHelper_DropoutOrder = 1698,
    XamlPropertyIndex_AutoSuggestBoxQuerySubmittedEventArgs_ChosenSuggestion = 1699,
    XamlPropertyIndex_AutoSuggestBoxQuerySubmittedEventArgs_QueryText = 1700,
    XamlPropertyIndex_AutoSuggestBox_QueryIcon = 1701,
    XamlPropertyIndex_StateTrigger_IsActive = 1702,
    XamlPropertyIndex_ContentPresenter_HorizontalContentAlignment = 1703,
    XamlPropertyIndex_ContentPresenter_VerticalContentAlignment = 1704,
    XamlPropertyIndex_AppBarTemplateSettings_ClipRect = 1705,
    XamlPropertyIndex_AppBarTemplateSettings_CompactRootMargin = 1706,
    XamlPropertyIndex_AppBarTemplateSettings_CompactVerticalDelta = 1707,
    XamlPropertyIndex_AppBarTemplateSettings_HiddenRootMargin = 1708,
    XamlPropertyIndex_AppBarTemplateSettings_HiddenVerticalDelta = 1709,
    XamlPropertyIndex_AppBarTemplateSettings_MinimalRootMargin = 1710,
    XamlPropertyIndex_AppBarTemplateSettings_MinimalVerticalDelta = 1711,
    XamlPropertyIndex_CommandBarTemplateSettings_ContentHeight = 1712,
    XamlPropertyIndex_CommandBarTemplateSettings_NegativeOverflowContentHeight = 1713,
    XamlPropertyIndex_CommandBarTemplateSettings_OverflowContentClipRect = 1714,
    XamlPropertyIndex_CommandBarTemplateSettings_OverflowContentHeight = 1715,
    XamlPropertyIndex_CommandBarTemplateSettings_OverflowContentHorizontalOffset = 1716,
    XamlPropertyIndex_CommandBarTemplateSettings_OverflowContentMaxHeight = 1717,
    XamlPropertyIndex_CommandBarTemplateSettings_OverflowContentMinWidth = 1718,
    XamlPropertyIndex_AppBar_TemplateSettings = 1719,
    XamlPropertyIndex_CommandBar_CommandBarOverflowPresenterStyle = 1720,
    XamlPropertyIndex_CommandBar_CommandBarTemplateSettings = 1721,
    XamlPropertyIndex_DrillInThemeAnimation_EntranceTarget = 1722,
    XamlPropertyIndex_DrillInThemeAnimation_EntranceTargetName = 1723,
    XamlPropertyIndex_DrillInThemeAnimation_ExitTarget = 1724,
    XamlPropertyIndex_DrillInThemeAnimation_ExitTargetName = 1725,
    XamlPropertyIndex_DrillOutThemeAnimation_EntranceTarget = 1726,
    XamlPropertyIndex_DrillOutThemeAnimation_EntranceTargetName = 1727,
    XamlPropertyIndex_DrillOutThemeAnimation_ExitTarget = 1728,
    XamlPropertyIndex_DrillOutThemeAnimation_ExitTargetName = 1729,
    XamlPropertyIndex_XamlBindingHelper_DataTemplateComponent = 1730,
    XamlPropertyIndex_AutomationProperties_Annotations = 1732,
    XamlPropertyIndex_AutomationAnnotation_Element = 1733,
    XamlPropertyIndex_AutomationAnnotation_Type = 1734,
    XamlPropertyIndex_AutomationPeerAnnotation_Peer = 1735,
    XamlPropertyIndex_AutomationPeerAnnotation_Type = 1736,
    XamlPropertyIndex_Hyperlink_UnderlineStyle = 1741,
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
    XamlPropertyIndex_CalendarView_DisabledForeground = 1742,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
    XamlPropertyIndex_CalendarView_TodayBackground = 1743,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
    XamlPropertyIndex_CalendarView_TodayBlackoutBackground = 1744,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
    XamlPropertyIndex_CalendarView_TodaySelectedInnerBorderBrush = 1747,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
    XamlPropertyIndex_Control_IsFocusEngaged = 1749,
    XamlPropertyIndex_Control_IsFocusEngagementEnabled = 1752,
    XamlPropertyIndex_RichEditBox_ClipboardCopyFormat = 1754,
    XamlPropertyIndex_CommandBarTemplateSettings_OverflowContentMaxWidth = 1757,
    XamlPropertyIndex_ComboBoxTemplateSettings_DropDownContentMinWidth = 1758,
    XamlPropertyIndex_MenuFlyoutPresenterTemplateSettings_FlyoutContentMinWidth = 1762,
    XamlPropertyIndex_MenuFlyoutPresenter_TemplateSettings = 1763,
    XamlPropertyIndex_AutomationProperties_LandmarkType = 1766,
    XamlPropertyIndex_AutomationProperties_LocalizedLandmarkType = 1767,
    XamlPropertyIndex_RepositionThemeTransition_IsStaggeringEnabled = 1769,
    XamlPropertyIndex_ListBox_SingleSelectionFollowsFocus = 1770,
    XamlPropertyIndex_ListViewBase_SingleSelectionFollowsFocus = 1771,
    XamlPropertyIndex_BitmapImage_AutoPlay = 1773,
    XamlPropertyIndex_BitmapImage_IsAnimatedBitmap = 1774,
    XamlPropertyIndex_BitmapImage_IsPlaying = 1775,
    XamlPropertyIndex_AutomationProperties_FullDescription = 1776,
    XamlPropertyIndex_AutomationProperties_IsDataValidForForm = 1777,
    XamlPropertyIndex_AutomationProperties_IsPeripheral = 1778,
    XamlPropertyIndex_AutomationProperties_LocalizedControlType = 1779,
    XamlPropertyIndex_FlyoutBase_AllowFocusOnInteraction = 1780,
    XamlPropertyIndex_TextElement_AllowFocusOnInteraction = 1781,
    XamlPropertyIndex_FrameworkElement_AllowFocusOnInteraction = 1782,
    XamlPropertyIndex_Control_RequiresPointer = 1783,
    XamlPropertyIndex_UIElement_ContextFlyout = 1785,
    XamlPropertyIndex_TextElement_AccessKey = 1786,
    XamlPropertyIndex_UIElement_AccessKeyScopeOwner = 1787,
    XamlPropertyIndex_UIElement_IsAccessKeyScope = 1788,
    XamlPropertyIndex_AutomationProperties_DescribedBy = 1790,
    XamlPropertyIndex_UIElement_AccessKey = 1803,
    XamlPropertyIndex_Control_XYFocusDown = 1804,
    XamlPropertyIndex_Control_XYFocusLeft = 1805,
    XamlPropertyIndex_Control_XYFocusRight = 1806,
    XamlPropertyIndex_Control_XYFocusUp = 1807,
    XamlPropertyIndex_Hyperlink_XYFocusDown = 1808,
    XamlPropertyIndex_Hyperlink_XYFocusLeft = 1809,
    XamlPropertyIndex_Hyperlink_XYFocusRight = 1810,
    XamlPropertyIndex_Hyperlink_XYFocusUp = 1811,
    XamlPropertyIndex_WebView_XYFocusDown = 1812,
    XamlPropertyIndex_WebView_XYFocusLeft = 1813,
    XamlPropertyIndex_WebView_XYFocusRight = 1814,
    XamlPropertyIndex_WebView_XYFocusUp = 1815,
    XamlPropertyIndex_CommandBarTemplateSettings_EffectiveOverflowButtonVisibility = 1816,
    XamlPropertyIndex_AppBarSeparator_IsInOverflow = 1817,
    XamlPropertyIndex_CommandBar_DefaultLabelPosition = 1818,
    XamlPropertyIndex_CommandBar_IsDynamicOverflowEnabled = 1819,
    XamlPropertyIndex_CommandBar_OverflowButtonVisibility = 1820,
    XamlPropertyIndex_AppBarButton_IsInOverflow = 1821,
    XamlPropertyIndex_AppBarButton_LabelPosition = 1822,
    XamlPropertyIndex_AppBarToggleButton_IsInOverflow = 1823,
    XamlPropertyIndex_AppBarToggleButton_LabelPosition = 1824,
    XamlPropertyIndex_FlyoutBase_LightDismissOverlayMode = 1825,
    XamlPropertyIndex_Popup_LightDismissOverlayMode = 1827,
    XamlPropertyIndex_CalendarDatePicker_LightDismissOverlayMode = 1829,
    XamlPropertyIndex_DatePicker_LightDismissOverlayMode = 1830,
    XamlPropertyIndex_SplitView_LightDismissOverlayMode = 1831,
    XamlPropertyIndex_TimePicker_LightDismissOverlayMode = 1832,
    XamlPropertyIndex_AppBar_LightDismissOverlayMode = 1833,
    XamlPropertyIndex_AutoSuggestBox_LightDismissOverlayMode = 1834,
    XamlPropertyIndex_ComboBox_LightDismissOverlayMode = 1835,
    XamlPropertyIndex_AppBarSeparator_DynamicOverflowOrder = 1836,
    XamlPropertyIndex_AppBarButton_DynamicOverflowOrder = 1837,
    XamlPropertyIndex_AppBarToggleButton_DynamicOverflowOrder = 1838,
    XamlPropertyIndex_FrameworkElement_FocusVisualMargin = 1839,
    XamlPropertyIndex_FrameworkElement_FocusVisualPrimaryBrush = 1840,
    XamlPropertyIndex_FrameworkElement_FocusVisualPrimaryThickness = 1841,
    XamlPropertyIndex_FrameworkElement_FocusVisualSecondaryBrush = 1842,
    XamlPropertyIndex_FrameworkElement_FocusVisualSecondaryThickness = 1843,
    XamlPropertyIndex_FlyoutBase_AllowFocusWhenDisabled = 1846,
    XamlPropertyIndex_FrameworkElement_AllowFocusWhenDisabled = 1847,
    XamlPropertyIndex_ComboBox_IsTextSearchEnabled = 1848,
    XamlPropertyIndex_TextElement_ExitDisplayModeOnAccessKeyInvoked = 1849,
    XamlPropertyIndex_UIElement_ExitDisplayModeOnAccessKeyInvoked = 1850,
    XamlPropertyIndex_MediaPlayerPresenter_IsFullWindow = 1851,
    XamlPropertyIndex_MediaPlayerPresenter_MediaPlayer = 1852,
    XamlPropertyIndex_MediaPlayerPresenter_Stretch = 1853,
    XamlPropertyIndex_MediaPlayerElement_AreTransportControlsEnabled = 1854,
    XamlPropertyIndex_MediaPlayerElement_AutoPlay = 1855,
    XamlPropertyIndex_MediaPlayerElement_IsFullWindow = 1856,
    XamlPropertyIndex_MediaPlayerElement_MediaPlayer = 1857,
    XamlPropertyIndex_MediaPlayerElement_PosterSource = 1858,
    XamlPropertyIndex_MediaPlayerElement_Source = 1859,
    XamlPropertyIndex_MediaPlayerElement_Stretch = 1860,
    XamlPropertyIndex_MediaPlayerElement_TransportControls = 1861,
    XamlPropertyIndex_MediaTransportControls_FastPlayFallbackBehaviour = 1862,
    XamlPropertyIndex_MediaTransportControls_IsNextTrackButtonVisible = 1863,
    XamlPropertyIndex_MediaTransportControls_IsPreviousTrackButtonVisible = 1864,
    XamlPropertyIndex_MediaTransportControls_IsSkipBackwardButtonVisible = 1865,
    XamlPropertyIndex_MediaTransportControls_IsSkipBackwardEnabled = 1866,
    XamlPropertyIndex_MediaTransportControls_IsSkipForwardButtonVisible = 1867,
    XamlPropertyIndex_MediaTransportControls_IsSkipForwardEnabled = 1868,
    XamlPropertyIndex_FlyoutBase_ElementSoundMode = 1869,
    XamlPropertyIndex_Control_ElementSoundMode = 1870,
    XamlPropertyIndex_Hyperlink_ElementSoundMode = 1871,
    XamlPropertyIndex_AutomationProperties_FlowsFrom = 1876,
    XamlPropertyIndex_AutomationProperties_FlowsTo = 1877,
    XamlPropertyIndex_TextElement_TextDecorations = 1879,
    XamlPropertyIndex_RichTextBlock_TextDecorations = 1881,
    XamlPropertyIndex_Control_DefaultStyleResourceUri = 1882,
    XamlPropertyIndex_ContentDialog_PrimaryButtonStyle = 1884,
    XamlPropertyIndex_ContentDialog_SecondaryButtonStyle = 1885,
    XamlPropertyIndex_TextElement_KeyTipHorizontalOffset = 1890,
    XamlPropertyIndex_TextElement_KeyTipPlacementMode = 1891,
    XamlPropertyIndex_TextElement_KeyTipVerticalOffset = 1892,
    XamlPropertyIndex_UIElement_KeyTipHorizontalOffset = 1893,
    XamlPropertyIndex_UIElement_KeyTipPlacementMode = 1894,
    XamlPropertyIndex_UIElement_KeyTipVerticalOffset = 1895,
    XamlPropertyIndex_FlyoutBase_OverlayInputPassThroughElement = 1896,
    XamlPropertyIndex_UIElement_XYFocusKeyboardNavigation = 1897,
    XamlPropertyIndex_AutomationProperties_Culture = 1898,
    XamlPropertyIndex_UIElement_XYFocusDownNavigationStrategy = 1918,
    XamlPropertyIndex_UIElement_XYFocusLeftNavigationStrategy = 1919,
    XamlPropertyIndex_UIElement_XYFocusRightNavigationStrategy = 1920,
    XamlPropertyIndex_UIElement_XYFocusUpNavigationStrategy = 1921,
    XamlPropertyIndex_Hyperlink_XYFocusDownNavigationStrategy = 1922,
    XamlPropertyIndex_Hyperlink_XYFocusLeftNavigationStrategy = 1923,
    XamlPropertyIndex_Hyperlink_XYFocusRightNavigationStrategy = 1924,
    XamlPropertyIndex_Hyperlink_XYFocusUpNavigationStrategy = 1925,
    XamlPropertyIndex_TextElement_AccessKeyScopeOwner = 1926,
    XamlPropertyIndex_TextElement_IsAccessKeyScope = 1927,
    XamlPropertyIndex_Hyperlink_FocusState = 1934,
    XamlPropertyIndex_ContentDialog_CloseButtonCommand = 1936,
    XamlPropertyIndex_ContentDialog_CloseButtonCommandParameter = 1937,
    XamlPropertyIndex_ContentDialog_CloseButtonStyle = 1938,
    XamlPropertyIndex_ContentDialog_CloseButtonText = 1939,
    XamlPropertyIndex_ContentDialog_DefaultButton = 1940,
    XamlPropertyIndex_RichEditBox_SelectionHighlightColorWhenNotFocused = 1941,
    XamlPropertyIndex_TextBox_SelectionHighlightColorWhenNotFocused = 1942,
    XamlPropertyIndex_SvgImageSource_RasterizePixelHeight = 1948,
    XamlPropertyIndex_SvgImageSource_RasterizePixelWidth = 1949,
    XamlPropertyIndex_SvgImageSource_UriSource = 1950,
    XamlPropertyIndex_LoadedImageSurface_DecodedPhysicalSize = 1955,
    XamlPropertyIndex_LoadedImageSurface_DecodedSize = 1956,
    XamlPropertyIndex_LoadedImageSurface_NaturalSize = 1957,
    XamlPropertyIndex_ComboBox_SelectionChangedTrigger = 1958,
    XamlPropertyIndex_XamlCompositionBrushBase_FallbackColor = 1960,
    XamlPropertyIndex_UIElement_Lights = 1962,
    XamlPropertyIndex_MenuFlyoutItem_Icon = 1963,
    XamlPropertyIndex_MenuFlyoutSubItem_Icon = 1964,
    XamlPropertyIndex_BitmapIcon_ShowAsMonochrome = 1965,
    XamlPropertyIndex_UIElement_HighContrastAdjustment = 1967,
    XamlPropertyIndex_RichEditBox_MaxLength = 1968,
    XamlPropertyIndex_UIElement_TabFocusNavigation = 1969,
    XamlPropertyIndex_Control_IsTemplateKeyTipTarget = 1970,
    XamlPropertyIndex_Hyperlink_IsTabStop = 1972,
    XamlPropertyIndex_Hyperlink_TabIndex = 1973,
    XamlPropertyIndex_MediaTransportControls_IsRepeatButtonVisible = 1974,
    XamlPropertyIndex_MediaTransportControls_IsRepeatEnabled = 1975,
    XamlPropertyIndex_MediaTransportControls_ShowAndHideAutomatically = 1976,
    XamlPropertyIndex_RichEditBox_DisabledFormattingAccelerators = 1977,
    XamlPropertyIndex_RichEditBox_CharacterCasing = 1978,
    XamlPropertyIndex_TextBox_CharacterCasing = 1979,
    XamlPropertyIndex_RichTextBlock_IsTextTrimmed = 1980,
    XamlPropertyIndex_RichTextBlockOverflow_IsTextTrimmed = 1981,
    XamlPropertyIndex_TextBlock_IsTextTrimmed = 1982,
    XamlPropertyIndex_TextHighlighter_Background = 1985,
    XamlPropertyIndex_TextHighlighter_Foreground = 1986,
    XamlPropertyIndex_TextHighlighter_Ranges = 1987,
    XamlPropertyIndex_RichTextBlock_TextHighlighters = 1988,
    XamlPropertyIndex_TextBlock_TextHighlighters = 1989,
    XamlPropertyIndex_FrameworkElement_ActualTheme = 1992,
    XamlPropertyIndex_Grid_ColumnSpacing = 1993,
    XamlPropertyIndex_Grid_RowSpacing = 1994,
    XamlPropertyIndex_StackPanel_Spacing = 1995,
    XamlPropertyIndex_Block_HorizontalTextAlignment = 1996,
    XamlPropertyIndex_RichTextBlock_HorizontalTextAlignment = 1997,
    XamlPropertyIndex_TextBlock_HorizontalTextAlignment = 1998,
    XamlPropertyIndex_RichEditBox_HorizontalTextAlignment = 1999,
    XamlPropertyIndex_TextBox_HorizontalTextAlignment = 2000,
    XamlPropertyIndex_TextBox_PlaceholderForeground = 2001,
    XamlPropertyIndex_ComboBox_PlaceholderForeground = 2002,
    XamlPropertyIndex_KeyboardAccelerator_IsEnabled = 2003,
    XamlPropertyIndex_KeyboardAccelerator_Key = 2004,
    XamlPropertyIndex_KeyboardAccelerator_Modifiers = 2005,
    XamlPropertyIndex_KeyboardAccelerator_ScopeOwner = 2006,
    XamlPropertyIndex_UIElement_KeyboardAccelerators = 2007,
    XamlPropertyIndex_ListViewItemPresenter_RevealBackground = 2009,
    XamlPropertyIndex_ListViewItemPresenter_RevealBackgroundShowsAboveContent = 2010,
    XamlPropertyIndex_ListViewItemPresenter_RevealBorderBrush = 2011,
    XamlPropertyIndex_ListViewItemPresenter_RevealBorderThickness = 2012,
    XamlPropertyIndex_UIElement_KeyTipTarget = 2014,
    XamlPropertyIndex_AppBarButtonTemplateSettings_KeyboardAcceleratorTextMinWidth = 2015,
    XamlPropertyIndex_AppBarToggleButtonTemplateSettings_KeyboardAcceleratorTextMinWidth = 2016,
    XamlPropertyIndex_MenuFlyoutItemTemplateSettings_KeyboardAcceleratorTextMinWidth = 2017,
    XamlPropertyIndex_MenuFlyoutItem_TemplateSettings = 2019,
    XamlPropertyIndex_AppBarButton_TemplateSettings = 2021,
    XamlPropertyIndex_AppBarToggleButton_TemplateSettings = 2023,
    XamlPropertyIndex_UIElement_KeyboardAcceleratorPlacementMode = 2028,
    XamlPropertyIndex_MediaTransportControls_IsCompactOverlayButtonVisible = 2032,
    XamlPropertyIndex_MediaTransportControls_IsCompactOverlayEnabled = 2033,
    XamlPropertyIndex_UIElement_KeyboardAcceleratorPlacementTarget = 2061,
    XamlPropertyIndex_UIElement_CenterPoint = 2062,
    XamlPropertyIndex_UIElement_Rotation = 2063,
    XamlPropertyIndex_UIElement_RotationAxis = 2064,
    XamlPropertyIndex_UIElement_Scale = 2065,
    XamlPropertyIndex_UIElement_TransformMatrix = 2066,
    XamlPropertyIndex_UIElement_Translation = 2067,
    XamlPropertyIndex_TextBox_HandwritingView = 2068,
    XamlPropertyIndex_AutomationProperties_HeadingLevel = 2069,
    XamlPropertyIndex_TextBox_IsHandwritingViewEnabled = 2076,
    XamlPropertyIndex_RichEditBox_ContentLinkProviders = 2078,
    XamlPropertyIndex_RichEditBox_ContentLinkBackgroundColor = 2079,
    XamlPropertyIndex_RichEditBox_ContentLinkForegroundColor = 2080,
    XamlPropertyIndex_HandwritingView_AreCandidatesEnabled = 2081,
    XamlPropertyIndex_HandwritingView_IsOpen = 2082,
    XamlPropertyIndex_HandwritingView_PlacementTarget = 2084,
    XamlPropertyIndex_HandwritingView_PlacementAlignment = 2085,
    XamlPropertyIndex_RichEditBox_HandwritingView = 2086,
    XamlPropertyIndex_RichEditBox_IsHandwritingViewEnabled = 2087,
    XamlPropertyIndex_MenuFlyoutItem_KeyboardAcceleratorTextOverride = 2090,
    XamlPropertyIndex_AppBarButton_KeyboardAcceleratorTextOverride = 2091,
    XamlPropertyIndex_AppBarToggleButton_KeyboardAcceleratorTextOverride = 2092,
    XamlPropertyIndex_ContentLink_Background = 2093,
    XamlPropertyIndex_ContentLink_Cursor = 2094,
    XamlPropertyIndex_ContentLink_ElementSoundMode = 2095,
    XamlPropertyIndex_ContentLink_FocusState = 2096,
    XamlPropertyIndex_ContentLink_IsTabStop = 2097,
    XamlPropertyIndex_ContentLink_TabIndex = 2098,
    XamlPropertyIndex_ContentLink_XYFocusDown = 2099,
    XamlPropertyIndex_ContentLink_XYFocusDownNavigationStrategy = 2100,
    XamlPropertyIndex_ContentLink_XYFocusLeft = 2101,
    XamlPropertyIndex_ContentLink_XYFocusLeftNavigationStrategy = 2102,
    XamlPropertyIndex_ContentLink_XYFocusRight = 2103,
    XamlPropertyIndex_ContentLink_XYFocusRightNavigationStrategy = 2104,
    XamlPropertyIndex_ContentLink_XYFocusUp = 2105,
    XamlPropertyIndex_ContentLink_XYFocusUpNavigationStrategy = 2106,
    XamlPropertyIndex_IconSource_Foreground = 2112,
    XamlPropertyIndex_BitmapIconSource_ShowAsMonochrome = 2113,
    XamlPropertyIndex_BitmapIconSource_UriSource = 2114,
    XamlPropertyIndex_FontIconSource_FontFamily = 2115,
    XamlPropertyIndex_FontIconSource_FontSize = 2116,
    XamlPropertyIndex_FontIconSource_FontStyle = 2117,
    XamlPropertyIndex_FontIconSource_FontWeight = 2118,
    XamlPropertyIndex_FontIconSource_Glyph = 2119,
    XamlPropertyIndex_FontIconSource_IsTextScaleFactorEnabled = 2120,
    XamlPropertyIndex_FontIconSource_MirroredWhenRightToLeft = 2121,
    XamlPropertyIndex_PathIconSource_Data = 2122,
    XamlPropertyIndex_SymbolIconSource_Symbol = 2123,
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
    XamlPropertyIndex_UIElement_Shadow = 2130,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
    XamlPropertyIndex_IconSourceElement_IconSource = 2131,
    XamlPropertyIndex_PasswordBox_CanPasteClipboardContent = 2137,
    XamlPropertyIndex_TextBox_CanPasteClipboardContent = 2138,
    XamlPropertyIndex_TextBox_CanRedo = 2139,
    XamlPropertyIndex_TextBox_CanUndo = 2140,
    XamlPropertyIndex_FlyoutBase_ShowMode = 2141,
    XamlPropertyIndex_FlyoutBase_Target = 2142,
    XamlPropertyIndex_Control_CornerRadius = 2143,
    XamlPropertyIndex_AutomationProperties_IsDialog = 2149,
    XamlPropertyIndex_AppBarElementContainer_DynamicOverflowOrder = 2150,
    XamlPropertyIndex_AppBarElementContainer_IsCompact = 2151,
    XamlPropertyIndex_AppBarElementContainer_IsInOverflow = 2152,
    XamlPropertyIndex_ScrollContentPresenter_CanContentRenderOutsideBounds = 2157,
    XamlPropertyIndex_ScrollViewer_CanContentRenderOutsideBounds = 2158,
    XamlPropertyIndex_RichEditBox_SelectionFlyout = 2159,
    XamlPropertyIndex_TextBox_SelectionFlyout = 2160,
    XamlPropertyIndex_Border_BackgroundSizing = 2161,
    XamlPropertyIndex_ContentPresenter_BackgroundSizing = 2162,
    XamlPropertyIndex_Control_BackgroundSizing = 2163,
    XamlPropertyIndex_Grid_BackgroundSizing = 2164,
    XamlPropertyIndex_RelativePanel_BackgroundSizing = 2165,
    XamlPropertyIndex_StackPanel_BackgroundSizing = 2166,
    XamlPropertyIndex_ScrollViewer_HorizontalAnchorRatio = 2170,
    XamlPropertyIndex_ScrollViewer_VerticalAnchorRatio = 2171,
    XamlPropertyIndex_ComboBox_Text = 2208,
    XamlPropertyIndex_TextBox_Description = 2217,
    XamlPropertyIndex_ToolTip_PlacementRect = 2218,
    XamlPropertyIndex_RichTextBlock_SelectionFlyout = 2219,
    XamlPropertyIndex_TextBlock_SelectionFlyout = 2220,
    XamlPropertyIndex_PasswordBox_SelectionFlyout = 2221,
    XamlPropertyIndex_Border_BackgroundTransition = 2222,
    XamlPropertyIndex_ContentPresenter_BackgroundTransition = 2223,
    XamlPropertyIndex_Panel_BackgroundTransition = 2224,
    XamlPropertyIndex_ColorPaletteResources_Accent = 2227,
    XamlPropertyIndex_ColorPaletteResources_AltHigh = 2228,
    XamlPropertyIndex_ColorPaletteResources_AltLow = 2229,
    XamlPropertyIndex_ColorPaletteResources_AltMedium = 2230,
    XamlPropertyIndex_ColorPaletteResources_AltMediumHigh = 2231,
    XamlPropertyIndex_ColorPaletteResources_AltMediumLow = 2232,
    XamlPropertyIndex_ColorPaletteResources_BaseHigh = 2233,
    XamlPropertyIndex_ColorPaletteResources_BaseLow = 2234,
    XamlPropertyIndex_ColorPaletteResources_BaseMedium = 2235,
    XamlPropertyIndex_ColorPaletteResources_BaseMediumHigh = 2236,
    XamlPropertyIndex_ColorPaletteResources_BaseMediumLow = 2237,
    XamlPropertyIndex_ColorPaletteResources_ChromeAltLow = 2238,
    XamlPropertyIndex_ColorPaletteResources_ChromeBlackHigh = 2239,
    XamlPropertyIndex_ColorPaletteResources_ChromeBlackLow = 2240,
    XamlPropertyIndex_ColorPaletteResources_ChromeBlackMedium = 2241,
    XamlPropertyIndex_ColorPaletteResources_ChromeBlackMediumLow = 2242,
    XamlPropertyIndex_ColorPaletteResources_ChromeDisabledHigh = 2243,
    XamlPropertyIndex_ColorPaletteResources_ChromeDisabledLow = 2244,
    XamlPropertyIndex_ColorPaletteResources_ChromeGray = 2245,
    XamlPropertyIndex_ColorPaletteResources_ChromeHigh = 2246,
    XamlPropertyIndex_ColorPaletteResources_ChromeLow = 2247,
    XamlPropertyIndex_ColorPaletteResources_ChromeMedium = 2248,
    XamlPropertyIndex_ColorPaletteResources_ChromeMediumLow = 2249,
    XamlPropertyIndex_ColorPaletteResources_ChromeWhite = 2250,
    XamlPropertyIndex_ColorPaletteResources_ErrorText = 2252,
    XamlPropertyIndex_ColorPaletteResources_ListLow = 2253,
    XamlPropertyIndex_ColorPaletteResources_ListMedium = 2254,
    XamlPropertyIndex_UIElement_TranslationTransition = 2255,
    XamlPropertyIndex_UIElement_OpacityTransition = 2256,
    XamlPropertyIndex_UIElement_RotationTransition = 2257,
    XamlPropertyIndex_UIElement_ScaleTransition = 2258,
    XamlPropertyIndex_BrushTransition_Duration = 2261,
    XamlPropertyIndex_ScalarTransition_Duration = 2262,
    XamlPropertyIndex_Vector3Transition_Duration = 2263,
    XamlPropertyIndex_Vector3Transition_Components = 2266,
    XamlPropertyIndex_FlyoutBase_IsOpen = 2267,
    XamlPropertyIndex_StandardUICommand_Kind = 2275,
    XamlPropertyIndex_UIElement_CanBeScrollAnchor = 2276,
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
    XamlPropertyIndex_ThemeShadow_Receivers = 2279,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
    XamlPropertyIndex_ScrollContentPresenter_SizesContentToTemplatedParent = 2280,
    XamlPropertyIndex_ComboBox_TextBoxStyle = 2281,
    XamlPropertyIndex_Frame_IsNavigationStackEnabled = 2282,
    XamlPropertyIndex_RichEditBox_ProofingMenuFlyout = 2283,
    XamlPropertyIndex_TextBox_ProofingMenuFlyout = 2284,
    XamlPropertyIndex_ScrollViewer_ReduceViewportForCoreInputViewOcclusions = 2295,
    XamlPropertyIndex_FlyoutBase_AreOpenCloseAnimationsEnabled = 2296,
    XamlPropertyIndex_FlyoutBase_InputDevicePrefersPrimaryCommands = 2297,
    XamlPropertyIndex_CalendarDatePicker_Description = 2300,
    XamlPropertyIndex_PasswordBox_Description = 2308,
    XamlPropertyIndex_RichEditBox_Description = 2316,
    XamlPropertyIndex_AutoSuggestBox_Description = 2331,
    XamlPropertyIndex_ComboBox_Description = 2339,
    XamlPropertyIndex_XamlUICommand_AccessKey = 2347,
    XamlPropertyIndex_XamlUICommand_Command = 2348,
    XamlPropertyIndex_XamlUICommand_Description = 2349,
    XamlPropertyIndex_XamlUICommand_IconSource = 2350,
    XamlPropertyIndex_XamlUICommand_KeyboardAccelerators = 2351,
    XamlPropertyIndex_XamlUICommand_Label = 2352,
    XamlPropertyIndex_DatePicker_SelectedDate = 2355,
    XamlPropertyIndex_TimePicker_SelectedTime = 2356,
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
    XamlPropertyIndex_AppBarTemplateSettings_NegativeCompactVerticalDelta = 2367,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
    XamlPropertyIndex_AppBarTemplateSettings_NegativeHiddenVerticalDelta = 2368,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
    XamlPropertyIndex_AppBarTemplateSettings_NegativeMinimalVerticalDelta = 2369,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
    XamlPropertyIndex_FlyoutBase_ShouldConstrainToRootBounds = 2378,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
    XamlPropertyIndex_Popup_ShouldConstrainToRootBounds = 2379,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
    XamlPropertyIndex_FlyoutPresenter_IsDefaultShadowEnabled = 2380,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
    XamlPropertyIndex_MenuFlyoutPresenter_IsDefaultShadowEnabled = 2381,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
    XamlPropertyIndex_UIElement_ActualOffset = 2382,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
    XamlPropertyIndex_UIElement_ActualSize = 2383,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
    XamlPropertyIndex_CommandBarTemplateSettings_OverflowContentCompactYTranslation = 2384,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
    XamlPropertyIndex_CommandBarTemplateSettings_OverflowContentHiddenYTranslation = 2385,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
    XamlPropertyIndex_CommandBarTemplateSettings_OverflowContentMinimalYTranslation = 2386,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x30000
    XamlPropertyIndex_HandwritingView_IsCommandBarOpen = 2395,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x30000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x30000
    XamlPropertyIndex_HandwritingView_IsSwitchToKeyboardEnabled = 2396,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x30000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
    XamlPropertyIndex_ListViewItemPresenter_SelectionIndicatorVisualEnabled = 2399,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
    XamlPropertyIndex_ListViewItemPresenter_SelectionIndicatorBrush = 2400,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
    XamlPropertyIndex_ListViewItemPresenter_SelectionIndicatorMode = 2401,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
    XamlPropertyIndex_ListViewItemPresenter_SelectionIndicatorPointerOverBrush = 2402,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
    XamlPropertyIndex_ListViewItemPresenter_SelectionIndicatorPressedBrush = 2403,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
    XamlPropertyIndex_ListViewItemPresenter_SelectedBorderBrush = 2410,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
    XamlPropertyIndex_ListViewItemPresenter_SelectedInnerBorderBrush = 2411,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
    XamlPropertyIndex_ListViewItemPresenter_CheckBoxCornerRadius = 2412,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
    XamlPropertyIndex_ListViewItemPresenter_SelectionIndicatorCornerRadius = 2413,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
    XamlPropertyIndex_ListViewItemPresenter_SelectedDisabledBorderBrush = 2414,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
    XamlPropertyIndex_ListViewItemPresenter_SelectedPressedBorderBrush = 2415,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
    XamlPropertyIndex_ListViewItemPresenter_SelectedDisabledBackground = 2416,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
    XamlPropertyIndex_ListViewItemPresenter_PointerOverBorderBrush = 2417,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
    XamlPropertyIndex_ListViewItemPresenter_CheckBoxPointerOverBrush = 2418,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
    XamlPropertyIndex_ListViewItemPresenter_CheckBoxPressedBrush = 2419,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
    XamlPropertyIndex_ListViewItemPresenter_CheckDisabledBrush = 2420,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
    XamlPropertyIndex_ListViewItemPresenter_CheckPressedBrush = 2421,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
    XamlPropertyIndex_ListViewItemPresenter_CheckBoxBorderBrush = 2422,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
    XamlPropertyIndex_ListViewItemPresenter_CheckBoxDisabledBorderBrush = 2423,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
    XamlPropertyIndex_ListViewItemPresenter_CheckBoxPressedBorderBrush = 2424,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
    XamlPropertyIndex_ListViewItemPresenter_CheckBoxDisabledBrush = 2425,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
    XamlPropertyIndex_ListViewItemPresenter_CheckBoxSelectedBrush = 2426,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
    XamlPropertyIndex_ListViewItemPresenter_CheckBoxSelectedDisabledBrush = 2427,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
    XamlPropertyIndex_ListViewItemPresenter_CheckBoxSelectedPointerOverBrush = 2428,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
    XamlPropertyIndex_ListViewItemPresenter_CheckBoxSelectedPressedBrush = 2429,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
    XamlPropertyIndex_ListViewItemPresenter_CheckBoxPointerOverBorderBrush = 2430,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
    XamlPropertyIndex_ListViewItemPresenter_SelectionIndicatorDisabledBrush = 2431,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x40000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
    XamlPropertyIndex_CalendarView_BlackoutBackground = 2432,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
    XamlPropertyIndex_CalendarView_BlackoutStrikethroughBrush = 2433,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
    XamlPropertyIndex_CalendarView_CalendarItemCornerRadius = 2434,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
    XamlPropertyIndex_CalendarView_CalendarItemDisabledBackground = 2435,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
    XamlPropertyIndex_CalendarView_CalendarItemHoverBackground = 2436,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
    XamlPropertyIndex_CalendarView_CalendarItemPressedBackground = 2437,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
    XamlPropertyIndex_CalendarView_DayItemMargin = 2438,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
    XamlPropertyIndex_CalendarView_FirstOfMonthLabelMargin = 2439,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
    XamlPropertyIndex_CalendarView_FirstOfYearDecadeLabelMargin = 2440,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
    XamlPropertyIndex_CalendarView_MonthYearItemMargin = 2441,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
    XamlPropertyIndex_CalendarView_OutOfScopeHoverForeground = 2442,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
    XamlPropertyIndex_CalendarView_OutOfScopePressedForeground = 2443,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
    XamlPropertyIndex_CalendarView_SelectedDisabledBorderBrush = 2444,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
    XamlPropertyIndex_CalendarView_SelectedDisabledForeground = 2445,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
    XamlPropertyIndex_CalendarView_SelectedHoverForeground = 2446,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
    XamlPropertyIndex_CalendarView_SelectedPressedForeground = 2447,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
    XamlPropertyIndex_CalendarView_TodayBlackoutForeground = 2448,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
    XamlPropertyIndex_CalendarView_TodayDisabledBackground = 2449,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
    XamlPropertyIndex_CalendarView_TodayHoverBackground = 2450,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
    XamlPropertyIndex_CalendarView_TodayPressedBackground = 2451,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
    XamlPropertyIndex_Popup_ActualPlacement = 2452,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
    XamlPropertyIndex_Popup_DesiredPlacement = 2453,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
    XamlPropertyIndex_Popup_PlacementTarget = 2454,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
    XamlPropertyIndex_AutomationProperties_AutomationControlType = 2455,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x50000
};
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Core.Direct.XamlTypeIndex
 *
 * Introduced to Windows.UI.Xaml.Core.Direct.XamlDirectContract in version 1.0
 *
 */
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlTypeIndex
{
    XamlTypeIndex_AutoSuggestBoxSuggestionChosenEventArgs = 34,
    XamlTypeIndex_AutoSuggestBoxTextChangedEventArgs = 35,
    XamlTypeIndex_CollectionViewSource = 41,
    XamlTypeIndex_ColumnDefinition = 44,
    XamlTypeIndex_GradientStop = 64,
    XamlTypeIndex_InputScope = 74,
    XamlTypeIndex_InputScopeName = 75,
    XamlTypeIndex_KeySpline = 78,
    XamlTypeIndex_PathFigure = 93,
    XamlTypeIndex_PrintDocument = 100,
    XamlTypeIndex_RowDefinition = 106,
    XamlTypeIndex_Style = 114,
    XamlTypeIndex_TimelineMarker = 126,
    XamlTypeIndex_VisualState = 137,
    XamlTypeIndex_VisualStateGroup = 138,
    XamlTypeIndex_VisualStateManager = 139,
    XamlTypeIndex_VisualTransition = 140,
    XamlTypeIndex_AddDeleteThemeTransition = 177,
    XamlTypeIndex_ArcSegment = 178,
    XamlTypeIndex_BackEase = 179,
    XamlTypeIndex_BeginStoryboard = 180,
    XamlTypeIndex_BezierSegment = 181,
    XamlTypeIndex_BindingBase = 182,
    XamlTypeIndex_BitmapCache = 183,
    XamlTypeIndex_BounceEase = 186,
    XamlTypeIndex_CircleEase = 187,
    XamlTypeIndex_ColorAnimation = 188,
    XamlTypeIndex_ColorAnimationUsingKeyFrames = 189,
    XamlTypeIndex_ContentThemeTransition = 190,
    XamlTypeIndex_ControlTemplate = 191,
    XamlTypeIndex_CubicEase = 192,
    XamlTypeIndex_DataTemplate = 194,
    XamlTypeIndex_DiscreteColorKeyFrame = 195,
    XamlTypeIndex_DiscreteDoubleKeyFrame = 196,
    XamlTypeIndex_DiscreteObjectKeyFrame = 197,
    XamlTypeIndex_DiscretePointKeyFrame = 198,
    XamlTypeIndex_DoubleAnimation = 200,
    XamlTypeIndex_DoubleAnimationUsingKeyFrames = 201,
    XamlTypeIndex_EasingColorKeyFrame = 204,
    XamlTypeIndex_EasingDoubleKeyFrame = 205,
    XamlTypeIndex_EasingPointKeyFrame = 206,
    XamlTypeIndex_EdgeUIThemeTransition = 207,
    XamlTypeIndex_ElasticEase = 208,
    XamlTypeIndex_EllipseGeometry = 209,
    XamlTypeIndex_EntranceThemeTransition = 210,
    XamlTypeIndex_EventTrigger = 211,
    XamlTypeIndex_ExponentialEase = 212,
    XamlTypeIndex_Flyout = 213,
    XamlTypeIndex_GeometryGroup = 216,
    XamlTypeIndex_ItemsPanelTemplate = 227,
    XamlTypeIndex_LinearColorKeyFrame = 230,
    XamlTypeIndex_LinearDoubleKeyFrame = 231,
    XamlTypeIndex_LinearPointKeyFrame = 232,
    XamlTypeIndex_LineGeometry = 233,
    XamlTypeIndex_LineSegment = 234,
    XamlTypeIndex_Matrix3DProjection = 236,
    XamlTypeIndex_MenuFlyout = 238,
    XamlTypeIndex_ObjectAnimationUsingKeyFrames = 240,
    XamlTypeIndex_PaneThemeTransition = 241,
    XamlTypeIndex_PathGeometry = 243,
    XamlTypeIndex_PlaneProjection = 244,
    XamlTypeIndex_PointAnimation = 245,
    XamlTypeIndex_PointAnimationUsingKeyFrames = 246,
    XamlTypeIndex_PolyBezierSegment = 248,
    XamlTypeIndex_PolyLineSegment = 249,
    XamlTypeIndex_PolyQuadraticBezierSegment = 250,
    XamlTypeIndex_PopupThemeTransition = 251,
    XamlTypeIndex_PowerEase = 252,
    XamlTypeIndex_QuadraticBezierSegment = 254,
    XamlTypeIndex_QuadraticEase = 255,
    XamlTypeIndex_QuarticEase = 256,
    XamlTypeIndex_QuinticEase = 257,
    XamlTypeIndex_RectangleGeometry = 258,
    XamlTypeIndex_RelativeSource = 259,
    XamlTypeIndex_RenderTargetBitmap = 260,
    XamlTypeIndex_ReorderThemeTransition = 261,
    XamlTypeIndex_RepositionThemeTransition = 262,
    XamlTypeIndex_Setter = 263,
    XamlTypeIndex_SineEase = 264,
    XamlTypeIndex_SolidColorBrush = 265,
    XamlTypeIndex_SplineColorKeyFrame = 266,
    XamlTypeIndex_SplineDoubleKeyFrame = 267,
    XamlTypeIndex_SplinePointKeyFrame = 268,
    XamlTypeIndex_BitmapImage = 285,
    XamlTypeIndex_Border = 286,
    XamlTypeIndex_CaptureElement = 288,
    XamlTypeIndex_CompositeTransform = 295,
    XamlTypeIndex_ContentPresenter = 296,
    XamlTypeIndex_DragItemThemeAnimation = 302,
    XamlTypeIndex_DragOverThemeAnimation = 303,
    XamlTypeIndex_DropTargetItemThemeAnimation = 304,
    XamlTypeIndex_FadeInThemeAnimation = 306,
    XamlTypeIndex_FadeOutThemeAnimation = 307,
    XamlTypeIndex_Glyphs = 312,
    XamlTypeIndex_Image = 326,
    XamlTypeIndex_ImageBrush = 328,
    XamlTypeIndex_InlineUIContainer = 329,
    XamlTypeIndex_ItemsPresenter = 332,
    XamlTypeIndex_LinearGradientBrush = 334,
    XamlTypeIndex_LineBreak = 335,
    XamlTypeIndex_MatrixTransform = 340,
    XamlTypeIndex_MediaElement = 342,
    XamlTypeIndex_Paragraph = 349,
    XamlTypeIndex_PointerDownThemeAnimation = 357,
    XamlTypeIndex_PointerUpThemeAnimation = 359,
    XamlTypeIndex_PopInThemeAnimation = 361,
    XamlTypeIndex_PopOutThemeAnimation = 362,
    XamlTypeIndex_Popup = 363,
    XamlTypeIndex_RepositionThemeAnimation = 370,
    XamlTypeIndex_ResourceDictionary = 371,
    XamlTypeIndex_RichTextBlock = 374,
    XamlTypeIndex_RichTextBlockOverflow = 376,
    XamlTypeIndex_RotateTransform = 378,
    XamlTypeIndex_Run = 380,
    XamlTypeIndex_ScaleTransform = 381,
    XamlTypeIndex_SkewTransform = 389,
    XamlTypeIndex_Span = 390,
    XamlTypeIndex_SplitCloseThemeAnimation = 391,
    XamlTypeIndex_SplitOpenThemeAnimation = 392,
    XamlTypeIndex_Storyboard = 393,
    XamlTypeIndex_SwipeBackThemeAnimation = 394,
    XamlTypeIndex_SwipeHintThemeAnimation = 395,
    XamlTypeIndex_TextBlock = 396,
    XamlTypeIndex_TransformGroup = 411,
    XamlTypeIndex_TranslateTransform = 413,
    XamlTypeIndex_Viewbox = 417,
    XamlTypeIndex_WebViewBrush = 423,
    XamlTypeIndex_AppBarSeparator = 427,
    XamlTypeIndex_BitmapIcon = 429,
    XamlTypeIndex_Bold = 430,
    XamlTypeIndex_Canvas = 432,
    XamlTypeIndex_ContentControl = 435,
    XamlTypeIndex_DatePicker = 436,
    XamlTypeIndex_DependencyObjectCollection = 437,
    XamlTypeIndex_Ellipse = 438,
    XamlTypeIndex_FontIcon = 440,
    XamlTypeIndex_Grid = 442,
    XamlTypeIndex_Hub = 445,
    XamlTypeIndex_HubSection = 446,
    XamlTypeIndex_Hyperlink = 447,
    XamlTypeIndex_Italic = 449,
    XamlTypeIndex_ItemsControl = 451,
    XamlTypeIndex_Line = 452,
    XamlTypeIndex_MediaTransportControls = 458,
    XamlTypeIndex_PasswordBox = 462,
    XamlTypeIndex_Path = 463,
    XamlTypeIndex_PathIcon = 464,
    XamlTypeIndex_Polygon = 465,
    XamlTypeIndex_Polyline = 466,
    XamlTypeIndex_ProgressRing = 468,
    XamlTypeIndex_Rectangle = 470,
    XamlTypeIndex_RichEditBox = 473,
    XamlTypeIndex_ScrollContentPresenter = 476,
    XamlTypeIndex_SearchBox = 477,
    XamlTypeIndex_SemanticZoom = 479,
    XamlTypeIndex_StackPanel = 481,
    XamlTypeIndex_SymbolIcon = 482,
    XamlTypeIndex_TextBox = 483,
    XamlTypeIndex_Thumb = 485,
    XamlTypeIndex_TickBar = 486,
    XamlTypeIndex_TimePicker = 487,
    XamlTypeIndex_ToggleSwitch = 489,
    XamlTypeIndex_Underline = 490,
    XamlTypeIndex_UserControl = 491,
    XamlTypeIndex_VariableSizedWrapGrid = 492,
    XamlTypeIndex_WebView = 494,
    XamlTypeIndex_AppBar = 495,
    XamlTypeIndex_AutoSuggestBox = 499,
    XamlTypeIndex_CarouselPanel = 502,
    XamlTypeIndex_ContentDialog = 506,
    XamlTypeIndex_FlyoutPresenter = 508,
    XamlTypeIndex_Frame = 509,
    XamlTypeIndex_GridViewItemPresenter = 511,
    XamlTypeIndex_GroupItem = 512,
    XamlTypeIndex_ItemsStackPanel = 514,
    XamlTypeIndex_ItemsWrapGrid = 515,
    XamlTypeIndex_ListViewItemPresenter = 520,
    XamlTypeIndex_MenuFlyoutItem = 521,
    XamlTypeIndex_MenuFlyoutPresenter = 522,
    XamlTypeIndex_MenuFlyoutSeparator = 523,
    XamlTypeIndex_Page = 525,
    XamlTypeIndex_ProgressBar = 528,
    XamlTypeIndex_ScrollBar = 530,
    XamlTypeIndex_SettingsFlyout = 533,
    XamlTypeIndex_Slider = 534,
    XamlTypeIndex_SwapChainBackgroundPanel = 535,
    XamlTypeIndex_SwapChainPanel = 536,
    XamlTypeIndex_ToolTip = 538,
    XamlTypeIndex_Button = 540,
    XamlTypeIndex_ComboBoxItem = 541,
    XamlTypeIndex_CommandBar = 542,
    XamlTypeIndex_FlipViewItem = 543,
    XamlTypeIndex_GridViewHeaderItem = 545,
    XamlTypeIndex_HyperlinkButton = 546,
    XamlTypeIndex_ListBoxItem = 547,
    XamlTypeIndex_ListViewHeaderItem = 550,
    XamlTypeIndex_RepeatButton = 551,
    XamlTypeIndex_ScrollViewer = 552,
    XamlTypeIndex_ToggleButton = 553,
    XamlTypeIndex_ToggleMenuFlyoutItem = 554,
    XamlTypeIndex_VirtualizingStackPanel = 555,
    XamlTypeIndex_WrapGrid = 556,
    XamlTypeIndex_AppBarButton = 557,
    XamlTypeIndex_AppBarToggleButton = 558,
    XamlTypeIndex_CheckBox = 559,
    XamlTypeIndex_GridViewItem = 560,
    XamlTypeIndex_ListViewItem = 561,
    XamlTypeIndex_RadioButton = 562,
    XamlTypeIndex_Binding = 564,
    XamlTypeIndex_ComboBox = 566,
    XamlTypeIndex_FlipView = 567,
    XamlTypeIndex_ListBox = 568,
    XamlTypeIndex_GridView = 570,
    XamlTypeIndex_ListView = 571,
    XamlTypeIndex_CalendarView = 707,
    XamlTypeIndex_CalendarViewDayItem = 709,
    XamlTypeIndex_CalendarPanel = 723,
    XamlTypeIndex_SplitView = 728,
    XamlTypeIndex_CompositeTransform3D = 732,
    XamlTypeIndex_PerspectiveTransform3D = 733,
    XamlTypeIndex_RelativePanel = 744,
    XamlTypeIndex_InkCanvas = 748,
    XamlTypeIndex_MenuFlyoutSubItem = 749,
    XamlTypeIndex_AdaptiveTrigger = 757,
    XamlTypeIndex_SoftwareBitmapSource = 761,
    XamlTypeIndex_StateTrigger = 767,
    XamlTypeIndex_CalendarDatePicker = 774,
    XamlTypeIndex_AutoSuggestBoxQuerySubmittedEventArgs = 778,
    XamlTypeIndex_CommandBarOverflowPresenter = 781,
    XamlTypeIndex_DrillInThemeAnimation = 782,
    XamlTypeIndex_DrillOutThemeAnimation = 783,
    XamlTypeIndex_AutomationAnnotation = 789,
    XamlTypeIndex_AutomationPeerAnnotation = 790,
    XamlTypeIndex_MediaPlayerPresenter = 828,
    XamlTypeIndex_MediaPlayerElement = 829,
    XamlTypeIndex_XamlLight = 855,
    XamlTypeIndex_SvgImageSource = 860,
    XamlTypeIndex_KeyboardAccelerator = 897,
    XamlTypeIndex_HandwritingView = 920,
    XamlTypeIndex_ContentLink = 925,
    XamlTypeIndex_BitmapIconSource = 929,
    XamlTypeIndex_FontIconSource = 930,
    XamlTypeIndex_PathIconSource = 931,
    XamlTypeIndex_SymbolIconSource = 933,
    XamlTypeIndex_IconSourceElement = 939,
    XamlTypeIndex_AppBarElementContainer = 945,
    XamlTypeIndex_ColorPaletteResources = 952,
    XamlTypeIndex_StandardUICommand = 961,
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
    XamlTypeIndex_ThemeShadow = 964,
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x20000
    XamlTypeIndex_XamlUICommand = 969,
};
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Core.Direct.IXamlDirect
 *
 * Introduced to Windows.UI.Xaml.Core.Direct.XamlDirectContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Core.Direct.XamlDirect
 *
 */
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Core_Direct_IXamlDirect[] = L"Windows.UI.Xaml.Core.Direct.IXamlDirect";
typedef struct __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetObject)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* GetXamlDirectObject)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        IInspectable* object,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject** result);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlTypeIndex typeIndex,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject** result);
    HRESULT (STDMETHODCALLTYPE* SetObjectProperty)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        IInspectable* value);
    HRESULT (STDMETHODCALLTYPE* SetXamlDirectObjectProperty)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* value);
    HRESULT (STDMETHODCALLTYPE* SetBooleanProperty)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* SetDoubleProperty)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* SetInt32Property)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* SetStringProperty)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* SetDateTimeProperty)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        struct __x_ABI_CWindows_CFoundation_CDateTime value);
    HRESULT (STDMETHODCALLTYPE* SetPointProperty)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        struct __x_ABI_CWindows_CFoundation_CPoint value);
    HRESULT (STDMETHODCALLTYPE* SetRectProperty)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        struct __x_ABI_CWindows_CFoundation_CRect value);
    HRESULT (STDMETHODCALLTYPE* SetSizeProperty)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        struct __x_ABI_CWindows_CFoundation_CSize value);
    HRESULT (STDMETHODCALLTYPE* SetTimeSpanProperty)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* SetColorProperty)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        struct __x_ABI_CWindows_CUI_CColor value);
    HRESULT (STDMETHODCALLTYPE* SetCornerRadiusProperty)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        struct __x_ABI_CWindows_CUI_CXaml_CCornerRadius value);
    HRESULT (STDMETHODCALLTYPE* SetDurationProperty)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        struct __x_ABI_CWindows_CUI_CXaml_CDuration value);
    HRESULT (STDMETHODCALLTYPE* SetGridLengthProperty)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        struct __x_ABI_CWindows_CUI_CXaml_CGridLength value);
    HRESULT (STDMETHODCALLTYPE* SetThicknessProperty)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        struct __x_ABI_CWindows_CUI_CXaml_CThickness value);
    HRESULT (STDMETHODCALLTYPE* SetMatrixProperty)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        struct __x_ABI_CWindows_CUI_CXaml_CMedia_CMatrix value);
    HRESULT (STDMETHODCALLTYPE* SetMatrix3DProperty)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        struct __x_ABI_CWindows_CUI_CXaml_CMedia_CMedia3D_CMatrix3D value);
    HRESULT (STDMETHODCALLTYPE* SetEnumProperty)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* GetObjectProperty)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* GetXamlDirectObjectProperty)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject** result);
    HRESULT (STDMETHODCALLTYPE* GetBooleanProperty)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetDoubleProperty)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        DOUBLE* result);
    HRESULT (STDMETHODCALLTYPE* GetInt32Property)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        INT32* result);
    HRESULT (STDMETHODCALLTYPE* GetStringProperty)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* GetDateTimeProperty)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        struct __x_ABI_CWindows_CFoundation_CDateTime* result);
    HRESULT (STDMETHODCALLTYPE* GetPointProperty)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        struct __x_ABI_CWindows_CFoundation_CPoint* result);
    HRESULT (STDMETHODCALLTYPE* GetRectProperty)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        struct __x_ABI_CWindows_CFoundation_CRect* result);
    HRESULT (STDMETHODCALLTYPE* GetSizeProperty)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        struct __x_ABI_CWindows_CFoundation_CSize* result);
    HRESULT (STDMETHODCALLTYPE* GetTimeSpanProperty)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* result);
    HRESULT (STDMETHODCALLTYPE* GetColorProperty)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        struct __x_ABI_CWindows_CUI_CColor* result);
    HRESULT (STDMETHODCALLTYPE* GetCornerRadiusProperty)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        struct __x_ABI_CWindows_CUI_CXaml_CCornerRadius* result);
    HRESULT (STDMETHODCALLTYPE* GetDurationProperty)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        struct __x_ABI_CWindows_CUI_CXaml_CDuration* result);
    HRESULT (STDMETHODCALLTYPE* GetGridLengthProperty)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        struct __x_ABI_CWindows_CUI_CXaml_CGridLength* result);
    HRESULT (STDMETHODCALLTYPE* GetThicknessProperty)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        struct __x_ABI_CWindows_CUI_CXaml_CThickness* result);
    HRESULT (STDMETHODCALLTYPE* GetMatrixProperty)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        struct __x_ABI_CWindows_CUI_CXaml_CMedia_CMatrix* result);
    HRESULT (STDMETHODCALLTYPE* GetMatrix3DProperty)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        struct __x_ABI_CWindows_CUI_CXaml_CMedia_CMedia3D_CMatrix3D* result);
    HRESULT (STDMETHODCALLTYPE* GetEnumProperty)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ClearProperty)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlPropertyIndex propertyIndex);
    HRESULT (STDMETHODCALLTYPE* GetCollectionCount)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetXamlDirectObjectFromCollectionAt)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        UINT32 index,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject** result);
    HRESULT (STDMETHODCALLTYPE* AddToCollection)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* value);
    HRESULT (STDMETHODCALLTYPE* InsertIntoCollectionAt)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        UINT32 index,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* value);
    HRESULT (STDMETHODCALLTYPE* RemoveFromCollection)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* value,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* RemoveFromCollectionAt)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* ClearCollection)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject);
    HRESULT (STDMETHODCALLTYPE* AddEventHandler)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlEventIndex eventIndex,
        IInspectable* handler);
    HRESULT (STDMETHODCALLTYPE* AddEventHandler_HandledEventsToo)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlEventIndex eventIndex,
        IInspectable* handler,
        boolean handledEventsToo);
    HRESULT (STDMETHODCALLTYPE* RemoveEventHandler)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* xamlDirectObject,
        enum __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CXamlEventIndex eventIndex,
        IInspectable* handler);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_GetObject(This, xamlDirectObject, result) \
    ((This)->lpVtbl->GetObject(This, xamlDirectObject, result))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_GetXamlDirectObject(This, object, result) \
    ((This)->lpVtbl->GetXamlDirectObject(This, object, result))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_CreateInstance(This, typeIndex, result) \
    ((This)->lpVtbl->CreateInstance(This, typeIndex, result))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_SetObjectProperty(This, xamlDirectObject, propertyIndex, value) \
    ((This)->lpVtbl->SetObjectProperty(This, xamlDirectObject, propertyIndex, value))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_SetXamlDirectObjectProperty(This, xamlDirectObject, propertyIndex, value) \
    ((This)->lpVtbl->SetXamlDirectObjectProperty(This, xamlDirectObject, propertyIndex, value))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_SetBooleanProperty(This, xamlDirectObject, propertyIndex, value) \
    ((This)->lpVtbl->SetBooleanProperty(This, xamlDirectObject, propertyIndex, value))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_SetDoubleProperty(This, xamlDirectObject, propertyIndex, value) \
    ((This)->lpVtbl->SetDoubleProperty(This, xamlDirectObject, propertyIndex, value))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_SetInt32Property(This, xamlDirectObject, propertyIndex, value) \
    ((This)->lpVtbl->SetInt32Property(This, xamlDirectObject, propertyIndex, value))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_SetStringProperty(This, xamlDirectObject, propertyIndex, value) \
    ((This)->lpVtbl->SetStringProperty(This, xamlDirectObject, propertyIndex, value))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_SetDateTimeProperty(This, xamlDirectObject, propertyIndex, value) \
    ((This)->lpVtbl->SetDateTimeProperty(This, xamlDirectObject, propertyIndex, value))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_SetPointProperty(This, xamlDirectObject, propertyIndex, value) \
    ((This)->lpVtbl->SetPointProperty(This, xamlDirectObject, propertyIndex, value))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_SetRectProperty(This, xamlDirectObject, propertyIndex, value) \
    ((This)->lpVtbl->SetRectProperty(This, xamlDirectObject, propertyIndex, value))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_SetSizeProperty(This, xamlDirectObject, propertyIndex, value) \
    ((This)->lpVtbl->SetSizeProperty(This, xamlDirectObject, propertyIndex, value))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_SetTimeSpanProperty(This, xamlDirectObject, propertyIndex, value) \
    ((This)->lpVtbl->SetTimeSpanProperty(This, xamlDirectObject, propertyIndex, value))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_SetColorProperty(This, xamlDirectObject, propertyIndex, value) \
    ((This)->lpVtbl->SetColorProperty(This, xamlDirectObject, propertyIndex, value))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_SetCornerRadiusProperty(This, xamlDirectObject, propertyIndex, value) \
    ((This)->lpVtbl->SetCornerRadiusProperty(This, xamlDirectObject, propertyIndex, value))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_SetDurationProperty(This, xamlDirectObject, propertyIndex, value) \
    ((This)->lpVtbl->SetDurationProperty(This, xamlDirectObject, propertyIndex, value))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_SetGridLengthProperty(This, xamlDirectObject, propertyIndex, value) \
    ((This)->lpVtbl->SetGridLengthProperty(This, xamlDirectObject, propertyIndex, value))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_SetThicknessProperty(This, xamlDirectObject, propertyIndex, value) \
    ((This)->lpVtbl->SetThicknessProperty(This, xamlDirectObject, propertyIndex, value))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_SetMatrixProperty(This, xamlDirectObject, propertyIndex, value) \
    ((This)->lpVtbl->SetMatrixProperty(This, xamlDirectObject, propertyIndex, value))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_SetMatrix3DProperty(This, xamlDirectObject, propertyIndex, value) \
    ((This)->lpVtbl->SetMatrix3DProperty(This, xamlDirectObject, propertyIndex, value))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_SetEnumProperty(This, xamlDirectObject, propertyIndex, value) \
    ((This)->lpVtbl->SetEnumProperty(This, xamlDirectObject, propertyIndex, value))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_GetObjectProperty(This, xamlDirectObject, propertyIndex, result) \
    ((This)->lpVtbl->GetObjectProperty(This, xamlDirectObject, propertyIndex, result))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_GetXamlDirectObjectProperty(This, xamlDirectObject, propertyIndex, result) \
    ((This)->lpVtbl->GetXamlDirectObjectProperty(This, xamlDirectObject, propertyIndex, result))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_GetBooleanProperty(This, xamlDirectObject, propertyIndex, result) \
    ((This)->lpVtbl->GetBooleanProperty(This, xamlDirectObject, propertyIndex, result))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_GetDoubleProperty(This, xamlDirectObject, propertyIndex, result) \
    ((This)->lpVtbl->GetDoubleProperty(This, xamlDirectObject, propertyIndex, result))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_GetInt32Property(This, xamlDirectObject, propertyIndex, result) \
    ((This)->lpVtbl->GetInt32Property(This, xamlDirectObject, propertyIndex, result))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_GetStringProperty(This, xamlDirectObject, propertyIndex, result) \
    ((This)->lpVtbl->GetStringProperty(This, xamlDirectObject, propertyIndex, result))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_GetDateTimeProperty(This, xamlDirectObject, propertyIndex, result) \
    ((This)->lpVtbl->GetDateTimeProperty(This, xamlDirectObject, propertyIndex, result))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_GetPointProperty(This, xamlDirectObject, propertyIndex, result) \
    ((This)->lpVtbl->GetPointProperty(This, xamlDirectObject, propertyIndex, result))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_GetRectProperty(This, xamlDirectObject, propertyIndex, result) \
    ((This)->lpVtbl->GetRectProperty(This, xamlDirectObject, propertyIndex, result))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_GetSizeProperty(This, xamlDirectObject, propertyIndex, result) \
    ((This)->lpVtbl->GetSizeProperty(This, xamlDirectObject, propertyIndex, result))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_GetTimeSpanProperty(This, xamlDirectObject, propertyIndex, result) \
    ((This)->lpVtbl->GetTimeSpanProperty(This, xamlDirectObject, propertyIndex, result))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_GetColorProperty(This, xamlDirectObject, propertyIndex, result) \
    ((This)->lpVtbl->GetColorProperty(This, xamlDirectObject, propertyIndex, result))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_GetCornerRadiusProperty(This, xamlDirectObject, propertyIndex, result) \
    ((This)->lpVtbl->GetCornerRadiusProperty(This, xamlDirectObject, propertyIndex, result))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_GetDurationProperty(This, xamlDirectObject, propertyIndex, result) \
    ((This)->lpVtbl->GetDurationProperty(This, xamlDirectObject, propertyIndex, result))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_GetGridLengthProperty(This, xamlDirectObject, propertyIndex, result) \
    ((This)->lpVtbl->GetGridLengthProperty(This, xamlDirectObject, propertyIndex, result))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_GetThicknessProperty(This, xamlDirectObject, propertyIndex, result) \
    ((This)->lpVtbl->GetThicknessProperty(This, xamlDirectObject, propertyIndex, result))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_GetMatrixProperty(This, xamlDirectObject, propertyIndex, result) \
    ((This)->lpVtbl->GetMatrixProperty(This, xamlDirectObject, propertyIndex, result))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_GetMatrix3DProperty(This, xamlDirectObject, propertyIndex, result) \
    ((This)->lpVtbl->GetMatrix3DProperty(This, xamlDirectObject, propertyIndex, result))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_GetEnumProperty(This, xamlDirectObject, propertyIndex, result) \
    ((This)->lpVtbl->GetEnumProperty(This, xamlDirectObject, propertyIndex, result))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_ClearProperty(This, xamlDirectObject, propertyIndex) \
    ((This)->lpVtbl->ClearProperty(This, xamlDirectObject, propertyIndex))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_GetCollectionCount(This, xamlDirectObject, result) \
    ((This)->lpVtbl->GetCollectionCount(This, xamlDirectObject, result))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_GetXamlDirectObjectFromCollectionAt(This, xamlDirectObject, index, result) \
    ((This)->lpVtbl->GetXamlDirectObjectFromCollectionAt(This, xamlDirectObject, index, result))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_AddToCollection(This, xamlDirectObject, value) \
    ((This)->lpVtbl->AddToCollection(This, xamlDirectObject, value))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_InsertIntoCollectionAt(This, xamlDirectObject, index, value) \
    ((This)->lpVtbl->InsertIntoCollectionAt(This, xamlDirectObject, index, value))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_RemoveFromCollection(This, xamlDirectObject, value, result) \
    ((This)->lpVtbl->RemoveFromCollection(This, xamlDirectObject, value, result))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_RemoveFromCollectionAt(This, xamlDirectObject, index) \
    ((This)->lpVtbl->RemoveFromCollectionAt(This, xamlDirectObject, index))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_ClearCollection(This, xamlDirectObject) \
    ((This)->lpVtbl->ClearCollection(This, xamlDirectObject))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_AddEventHandler(This, xamlDirectObject, eventIndex, handler) \
    ((This)->lpVtbl->AddEventHandler(This, xamlDirectObject, eventIndex, handler))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_AddEventHandler_HandledEventsToo(This, xamlDirectObject, eventIndex, handler, handledEventsToo) \
    ((This)->lpVtbl->AddEventHandler_HandledEventsToo(This, xamlDirectObject, eventIndex, handler, handledEventsToo))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_RemoveEventHandler(This, xamlDirectObject, eventIndex, handler) \
    ((This)->lpVtbl->RemoveEventHandler(This, xamlDirectObject, eventIndex, handler))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Core.Direct.IXamlDirectObject
 *
 * Introduced to Windows.UI.Xaml.Core.Direct.XamlDirectContract in version 1.0
 *
 */
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Core_Direct_IXamlDirectObject[] = L"Windows.UI.Xaml.Core.Direct.IXamlDirectObject";
typedef struct __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObjectVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObjectVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObjectVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectObject_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Core.Direct.IXamlDirectStatics
 *
 * Introduced to Windows.UI.Xaml.Core.Direct.XamlDirectContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Core.Direct.XamlDirect
 *
 */
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Core_Direct_IXamlDirectStatics[] = L"Windows.UI.Xaml.Core.Direct.IXamlDirectStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirect** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectStatics_GetDefault(This, result) \
    ((This)->lpVtbl->GetDefault(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CCore_CDirect_CIXamlDirectStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Core.Direct.XamlDirect
 *
 * Introduced to Windows.UI.Xaml.Core.Direct.XamlDirectContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Core.Direct.IXamlDirectStatics interface starting with version 1.0 of the Windows.UI.Xaml.Core.Direct.XamlDirectContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Core.Direct.IXamlDirect ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Core_Direct_XamlDirect_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Core_Direct_XamlDirect_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Core_Direct_XamlDirect[] = L"Windows.UI.Xaml.Core.Direct.XamlDirect";
#endif
#endif // WINDOWS_UI_XAML_CORE_DIRECT_XAMLDIRECTCONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eui2Examl2Ecore2Edirect_p_h__

#endif // __windows2Eui2Examl2Ecore2Edirect_h__
