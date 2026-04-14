
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
#ifndef __windows2Egaming2Einput2Epreview_h__
#define __windows2Egaming2Einput2Epreview_h__
#ifndef __windows2Egaming2Einput2Epreview_p_h__
#define __windows2Egaming2Einput2Epreview_p_h__


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
#if !defined(WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION)
#define WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION 0x20000
#endif // defined(WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION)

#if !defined(WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION)
#define WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION 0x70000
#endif // defined(WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Gaming.Input.h"
#include "Windows.Gaming.Input.Custom.h"
#include "Windows.System.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGaming_CInput_CPreview_CIGameControllerProviderInfoStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CPreview_CIGameControllerProviderInfoStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Preview {
                    interface IGameControllerProviderInfoStatics;
                } /* Preview */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CPreview_CIGameControllerProviderInfoStatics ABI::Windows::Gaming::Input::Preview::IGameControllerProviderInfoStatics

#endif // ____x_ABI_CWindows_CGaming_CInput_CPreview_CIGameControllerProviderInfoStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Preview {
                    interface ILegacyGipGameControllerProvider;
                } /* Preview */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider ABI::Windows::Gaming::Input::Preview::ILegacyGipGameControllerProvider

#endif // ____x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Preview {
                    interface ILegacyGipGameControllerProviderStatics;
                } /* Preview */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics ABI::Windows::Gaming::Input::Preview::ILegacyGipGameControllerProviderStatics

#endif // ____x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions

#ifndef DEF___FIIterator_1_HSTRING_USE
#define DEF___FIIterator_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8c304ebb-6615-50a4-8829-879ecd443236"))
IIterator<HSTRING> : IIterator_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<HSTRING> __FIIterator_1_HSTRING_t;
#define __FIIterator_1_HSTRING ABI::Windows::Foundation::Collections::__FIIterator_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_HSTRING_USE */



#ifndef DEF___FIIterable_1_HSTRING_USE
#define DEF___FIIterable_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e2fcc7c1-3bfc-5a0b-b2b0-72e769d1cb7e"))
IIterable<HSTRING> : IIterable_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<HSTRING> __FIIterable_1_HSTRING_t;
#define __FIIterable_1_HSTRING ABI::Windows::Foundation::Collections::__FIIterable_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_HSTRING_USE */


namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Preview {
                    typedef enum RemappingButtonCategory : int RemappingButtonCategory;
                } /* Preview */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

#if WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000

#ifndef DEF___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_USE
#define DEF___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bfb7672c-b8eb-5a83-bb57-c37ebcc053e6"))
IKeyValuePair<enum ABI::Windows::Gaming::Input::Preview::RemappingButtonCategory, IInspectable*> : IKeyValuePair_impl<enum ABI::Windows::Gaming::Input::Preview::RemappingButtonCategory, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<Windows.Gaming.Input.Preview.RemappingButtonCategory, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<enum ABI::Windows::Gaming::Input::Preview::RemappingButtonCategory, IInspectable*> __FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_t;
#define __FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_USE */

#endif // WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000

#if WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000

#ifndef DEF___FIIterator_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2f396834-b86a-5cd3-9e25-92d36931f0e2"))
IIterator<__FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable*> : IIterator_impl<__FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<Windows.Gaming.Input.Preview.RemappingButtonCategory, Object>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable*> __FIIterator_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_t;
#define __FIIterator_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_USE */

#endif // WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000

#if WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000

#ifndef DEF___FIIterable_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("da1c35a5-5993-56ce-9d1d-4f1e22433034"))
IIterable<__FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable*> : IIterable_impl<__FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<Windows.Gaming.Input.Preview.RemappingButtonCategory, Object>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable*> __FIIterable_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_t;
#define __FIIterable_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_USE */

#endif // WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000

#if WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000

#ifndef DEF___FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_USE
#define DEF___FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a73dd129-3e15-5afc-ba2b-18ecabbdb6fb"))
IMapView<enum ABI::Windows::Gaming::Input::Preview::RemappingButtonCategory, IInspectable*> : IMapView_impl<enum ABI::Windows::Gaming::Input::Preview::RemappingButtonCategory, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<Windows.Gaming.Input.Preview.RemappingButtonCategory, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<enum ABI::Windows::Gaming::Input::Preview::RemappingButtonCategory, IInspectable*> __FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_t;
#define __FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable ABI::Windows::Foundation::Collections::__FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_USE */

#endif // WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000


#ifndef DEF___FIVectorView_1_HSTRING_USE
#define DEF___FIVectorView_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2f13c006-a03a-5f69-b090-75a43e33423e"))
IVectorView<HSTRING> : IVectorView_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<HSTRING> __FIVectorView_1_HSTRING_t;
#define __FIVectorView_1_HSTRING ABI::Windows::Foundation::Collections::__FIVectorView_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_HSTRING_USE */


#ifndef ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    interface IGameControllerProvider;
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider ABI::Windows::Gaming::Input::Custom::IGameControllerProvider

#endif // ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIGameController_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIGameController_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                interface IGameController;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CIGameController ABI::Windows::Gaming::Input::IGameController

#endif // ____x_ABI_CWindows_CGaming_CInput_CIGameController_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace System {
            class User;
        } /* System */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            interface IUser;
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CIUser ABI::Windows::System::IUser

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Preview {
                    typedef enum DeviceCommand : int DeviceCommand;
                } /* Preview */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Preview {
                    typedef enum GameControllerBatteryChargingState : int GameControllerBatteryChargingState;
                } /* Preview */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Preview {
                    typedef enum GameControllerBatteryKind : int GameControllerBatteryKind;
                } /* Preview */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Preview {
                    typedef enum GameControllerBatteryLevel : int GameControllerBatteryLevel;
                } /* Preview */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Preview {
                    typedef enum GameControllerFirmwareCorruptReason : int GameControllerFirmwareCorruptReason;
                } /* Preview */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Preview {
                    typedef enum HeadsetOperation : int HeadsetOperation;
                } /* Preview */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Preview {
                    class LegacyGipGameControllerProvider;
                } /* Preview */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Gaming.Input.Preview.DeviceCommand
 *
 * Introduced to Windows.Gaming.Input.GamingInputPreviewContract in version 2.0
 *
 */
#if WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Preview {
                    enum DeviceCommand : int
                    {
                        DeviceCommand_Reset = 0,
                    };
                } /* Preview */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Gaming.Input.Preview.GameControllerBatteryChargingState
 *
 * Introduced to Windows.Gaming.Input.GamingInputPreviewContract in version 2.0
 *
 */
#if WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Preview {
                    enum GameControllerBatteryChargingState : int
                    {
                        GameControllerBatteryChargingState_Unknown = 0,
                        GameControllerBatteryChargingState_Inactive = 1,
                        GameControllerBatteryChargingState_Active = 2,
                        GameControllerBatteryChargingState_Error = 3,
                    };
                } /* Preview */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Gaming.Input.Preview.GameControllerBatteryKind
 *
 * Introduced to Windows.Gaming.Input.GamingInputPreviewContract in version 2.0
 *
 */
#if WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Preview {
                    enum GameControllerBatteryKind : int
                    {
                        GameControllerBatteryKind_Unknown = 0,
                        GameControllerBatteryKind_None = 1,
                        GameControllerBatteryKind_Standard = 2,
                        GameControllerBatteryKind_Rechargeable = 3,
                    };
                } /* Preview */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Gaming.Input.Preview.GameControllerBatteryLevel
 *
 * Introduced to Windows.Gaming.Input.GamingInputPreviewContract in version 2.0
 *
 */
#if WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Preview {
                    enum GameControllerBatteryLevel : int
                    {
                        GameControllerBatteryLevel_Unknown = 0,
                        GameControllerBatteryLevel_Critical = 1,
                        GameControllerBatteryLevel_Low = 2,
                        GameControllerBatteryLevel_Medium = 3,
                        GameControllerBatteryLevel_Full = 4,
                    };
                } /* Preview */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Gaming.Input.Preview.GameControllerFirmwareCorruptReason
 *
 * Introduced to Windows.Gaming.Input.GamingInputPreviewContract in version 2.0
 *
 */
#if WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Preview {
                    enum GameControllerFirmwareCorruptReason : int
                    {
                        GameControllerFirmwareCorruptReason_Unknown = 0,
                        GameControllerFirmwareCorruptReason_NotCorrupt = 1,
                        GameControllerFirmwareCorruptReason_TwoUpCorrupt = 2,
                        GameControllerFirmwareCorruptReason_AppCorrupt = 3,
                        GameControllerFirmwareCorruptReason_RadioCorrupt = 4,
                        GameControllerFirmwareCorruptReason_EepromCorrupt = 5,
                        GameControllerFirmwareCorruptReason_SafeToUpdate = 6,
                    };
                } /* Preview */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Gaming.Input.Preview.HeadsetLevel
 *
 * Introduced to Windows.Gaming.Input.GamingInputPreviewContract in version 2.0
 *
 */
#if WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Preview {
                    enum HeadsetLevel : int
                    {
                        HeadsetLevel_Off = 0,
                        HeadsetLevel_Low = 1,
                        HeadsetLevel_Medium = 2,
                        HeadsetLevel_High = 3,
                    };
                } /* Preview */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Gaming.Input.Preview.HeadsetOperation
 *
 * Introduced to Windows.Gaming.Input.GamingInputPreviewContract in version 2.0
 *
 */
#if WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Preview {
                    enum HeadsetOperation : int
                    {
                        HeadsetOperation_Geq = 0,
                        HeadsetOperation_BassBoostGain = 1,
                        HeadsetOperation_SmartMute = 2,
                        HeadsetOperation_SideTone = 3,
                        HeadsetOperation_MuteLedBrightness = 4,
                        HeadsetOperation_SwapMixAndVolumeDials = 5,
                    };
                } /* Preview */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Gaming.Input.Preview.RemappingButtonCategory
 *
 * Introduced to Windows.Gaming.Input.GamingInputPreviewContract in version 2.0
 *
 */
#if WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Preview {
                    enum RemappingButtonCategory : int
                    {
                        RemappingButtonCategory_ButtonSettings = 0,
                        RemappingButtonCategory_AnalogSettings = 1,
                        RemappingButtonCategory_VibrationSettings = 2,
                        RemappingButtonCategory_ShareShortPress = 3,
                        RemappingButtonCategory_ShareShortPressMetaData = 4,
                        RemappingButtonCategory_ShareShortPressMetaDataDisplay = 5,
                        RemappingButtonCategory_ShareLongPress = 6,
                        RemappingButtonCategory_ShareLongPressMetaData = 7,
                        RemappingButtonCategory_ShareLongPressMetaDataDisplay = 8,
                        RemappingButtonCategory_ShareDoublePress = 9,
                        RemappingButtonCategory_ShareDoublePressMetaData = 10,
                        RemappingButtonCategory_ShareDoublePressMetaDataDisplay = 11,
                    };
                } /* Preview */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Gaming.Input.Preview.HeadsetGeqGains
 *
 * Introduced to Windows.Gaming.Input.GamingInputPreviewContract in version 2.0
 *
 */
#if WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Preview {
                    struct HeadsetGeqGains
                    {
                        INT32 band1Gain;
                        INT32 band2Gain;
                        INT32 band3Gain;
                        INT32 band4Gain;
                        INT32 band5Gain;
                    };
                } /* Preview */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Gaming.Input.Preview.IGameControllerProviderInfoStatics
 *
 * Introduced to Windows.Gaming.Input.GamingInputPreviewContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.Preview.GameControllerProviderInfo
 *
 */
#if WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CPreview_CIGameControllerProviderInfoStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CPreview_CIGameControllerProviderInfoStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_Preview_IGameControllerProviderInfoStatics[] = L"Windows.Gaming.Input.Preview.IGameControllerProviderInfoStatics";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Preview {
                    MIDL_INTERFACE("0be1e6c5-d9bd-44ee-8362-488b2e464bfb")
                    IGameControllerProviderInfoStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetParentProviderId(
                            ABI::Windows::Gaming::Input::Custom::IGameControllerProvider* provider,
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetProviderId(
                            ABI::Windows::Gaming::Input::Custom::IGameControllerProvider* provider,
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGameControllerProviderInfoStatics = __uuidof(IGameControllerProviderInfoStatics);
                } /* Preview */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CPreview_CIGameControllerProviderInfoStatics;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CPreview_CIGameControllerProviderInfoStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.Input.Preview.ILegacyGipGameControllerProvider
 *
 * Introduced to Windows.Gaming.Input.GamingInputPreviewContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.Preview.LegacyGipGameControllerProvider
 *
 */
#if WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_Preview_ILegacyGipGameControllerProvider[] = L"Windows.Gaming.Input.Preview.ILegacyGipGameControllerProvider";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Preview {
                    MIDL_INTERFACE("2da3ed52-ffd9-43e2-825c-1d2790e04d14")
                    ILegacyGipGameControllerProvider : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_BatteryChargingState(
                            ABI::Windows::Gaming::Input::Preview::GameControllerBatteryChargingState* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_BatteryKind(
                            ABI::Windows::Gaming::Input::Preview::GameControllerBatteryKind* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_BatteryLevel(
                            ABI::Windows::Gaming::Input::Preview::GameControllerBatteryLevel* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeviceFirmwareCorruptionState(
                            ABI::Windows::Gaming::Input::Preview::GameControllerFirmwareCorruptReason* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsFirmwareCorrupted(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE IsInterfaceSupported(
                            GUID interfaceId,
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsSyntheticDevice(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PreferredTypes(
                            __FIVectorView_1_HSTRING** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ExecuteCommand(
                            ABI::Windows::Gaming::Input::Preview::DeviceCommand command
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetHomeLedIntensity(
                            BYTE intensity
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetExtendedDeviceInfo(
                            UINT32* bufferLength,
                            BYTE** buffer
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetHeadsetOperation(
                            ABI::Windows::Gaming::Input::Preview::HeadsetOperation operation,
                            UINT32 bufferLength,
                            BYTE* buffer
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetHeadsetOperation(
                            ABI::Windows::Gaming::Input::Preview::HeadsetOperation operation,
                            UINT32* bufferLength,
                            BYTE** buffer
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AppCompatVersion(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetStandardControllerButtonRemapping(
                            ABI::Windows::System::IUser* user,
                            boolean previous,
                            __FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable* remapping
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetStandardControllerButtonRemapping(
                            ABI::Windows::System::IUser* user,
                            boolean previous,
                            __FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable** remapping
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILegacyGipGameControllerProvider = __uuidof(ILegacyGipGameControllerProvider);
                } /* Preview */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Gaming.Input.Preview.ILegacyGipGameControllerProviderStatics
 *
 * Introduced to Windows.Gaming.Input.GamingInputPreviewContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.Preview.LegacyGipGameControllerProvider
 *
 */
#if WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_Preview_ILegacyGipGameControllerProviderStatics[] = L"Windows.Gaming.Input.Preview.ILegacyGipGameControllerProviderStatics";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Preview {
                    MIDL_INTERFACE("d40dda17-b1f4-499a-874c-7095aac15291")
                    ILegacyGipGameControllerProviderStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE FromGameController(
                            ABI::Windows::Gaming::Input::IGameController* controller,
                            ABI::Windows::Gaming::Input::Preview::ILegacyGipGameControllerProvider** legacyGipGameControllerProvider
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE FromGameControllerProvider(
                            ABI::Windows::Gaming::Input::Custom::IGameControllerProvider* provider,
                            ABI::Windows::Gaming::Input::Preview::ILegacyGipGameControllerProvider** legacyGipGameControllerProvider
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE PairPilotToCopilot(
                            ABI::Windows::System::IUser* user,
                            HSTRING pilotControllerProviderId,
                            HSTRING copilotControllerProviderId
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ClearPairing(
                            ABI::Windows::System::IUser* user,
                            HSTRING controllerProviderId
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE IsPilot(
                            ABI::Windows::System::IUser* user,
                            HSTRING controllerProviderId,
                            HSTRING* copilotControllerProviderId
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE IsCopilot(
                            ABI::Windows::System::IUser* user,
                            HSTRING controllerProviderId,
                            HSTRING* pilotControllerProviderId
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILegacyGipGameControllerProviderStatics = __uuidof(ILegacyGipGameControllerProviderStatics);
                } /* Preview */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Gaming.Input.Preview.GameControllerProviderInfo
 *
 * Introduced to Windows.Gaming.Input.GamingInputPreviewContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Gaming.Input.Preview.IGameControllerProviderInfoStatics interface starting with version 1.0 of the Windows.Gaming.Input.GamingInputPreviewContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_Preview_GameControllerProviderInfo_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_Preview_GameControllerProviderInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_Preview_GameControllerProviderInfo[] = L"Windows.Gaming.Input.Preview.GameControllerProviderInfo";
#endif
#endif // WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.Input.Preview.LegacyGipGameControllerProvider
 *
 * Introduced to Windows.Gaming.Input.GamingInputPreviewContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Gaming.Input.Preview.ILegacyGipGameControllerProviderStatics interface starting with version 2.0 of the Windows.Gaming.Input.GamingInputPreviewContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Input.Preview.ILegacyGipGameControllerProvider ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_Preview_LegacyGipGameControllerProvider_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_Preview_LegacyGipGameControllerProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_Preview_LegacyGipGameControllerProvider[] = L"Windows.Gaming.Input.Preview.LegacyGipGameControllerProvider";
#endif
#endif // WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGaming_CInput_CPreview_CIGameControllerProviderInfoStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CPreview_CIGameControllerProviderInfoStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CPreview_CIGameControllerProviderInfoStatics __x_ABI_CWindows_CGaming_CInput_CPreview_CIGameControllerProviderInfoStatics;

#endif // ____x_ABI_CWindows_CGaming_CInput_CPreview_CIGameControllerProviderInfoStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider;

#endif // ____x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics;

#endif // ____x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if !defined(____FIIterator_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterator_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterator_1_HSTRING __FIIterator_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_HSTRING;

typedef struct __FIIterator_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_HSTRING* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_HSTRING* This,
        UINT32 itemsLength,
        HSTRING* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_HSTRINGVtbl;

interface __FIIterator_1_HSTRING
{
    CONST_VTBL struct __FIIterator_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_HSTRING_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_HSTRING_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_HSTRING_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_HSTRING_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIIterable_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterable_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterable_1_HSTRING __FIIterable_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_HSTRING;

typedef struct __FIIterable_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_HSTRING* This,
        __FIIterator_1_HSTRING** result);

    END_INTERFACE
} __FIIterable_1_HSTRINGVtbl;

interface __FIIterable_1_HSTRING
{
    CONST_VTBL struct __FIIterable_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_HSTRING_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_HSTRING_INTERFACE_DEFINED__

typedef enum __x_ABI_CWindows_CGaming_CInput_CPreview_CRemappingButtonCategory __x_ABI_CWindows_CGaming_CInput_CPreview_CRemappingButtonCategory;

#if WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000
#if !defined(____FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable __FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable;

typedef struct __FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable* This,
        enum __x_ABI_CWindows_CGaming_CInput_CPreview_CRemappingButtonCategory* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable* This,
        IInspectable** result);

    END_INTERFACE
} __FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectableVtbl;

interface __FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable
{
    CONST_VTBL struct __FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000

#if WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000
#if !defined(____FIIterator_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable __FIIterator_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable;

typedef struct __FIIterator_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable* This,
        __FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectableVtbl;

interface __FIIterator_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000

#if WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000
#if !defined(____FIIterable_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable __FIIterable_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable;

typedef struct __FIIterable_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable* This,
        __FIIterator_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectableVtbl;

interface __FIIterable_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000

typedef interface __FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable __FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable;

#if WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000
#if !defined(____FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_INTERFACE_DEFINED__)
#define ____FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_INTERFACE_DEFINED__

typedef interface __FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable __FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable;

typedef struct __FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable* This,
        enum __x_ABI_CWindows_CGaming_CInput_CPreview_CRemappingButtonCategory key,
        IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable* This,
        enum __x_ABI_CWindows_CGaming_CInput_CPreview_CRemappingButtonCategory key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable* This,
        __FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable** first,
        __FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable** second);

    END_INTERFACE
} __FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectableVtbl;

interface __FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable
{
    CONST_VTBL struct __FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000

#if !defined(____FIVectorView_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIVectorView_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_HSTRING __FIVectorView_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_HSTRING;

typedef struct __FIVectorView_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_HSTRING* This,
        UINT32 index,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_HSTRING* This,
        HSTRING value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_HSTRING* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        HSTRING* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_HSTRINGVtbl;

interface __FIVectorView_1_HSTRING
{
    CONST_VTBL struct __FIVectorView_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_HSTRING_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_HSTRING_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_HSTRING_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_HSTRING_INTERFACE_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider;

#endif // ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIGameController_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIGameController_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CIGameController __x_ABI_CWindows_CGaming_CInput_CIGameController;

#endif // ____x_ABI_CWindows_CGaming_CInput_CIGameController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIUser __x_ABI_CWindows_CSystem_CIUser;

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CGaming_CInput_CPreview_CDeviceCommand __x_ABI_CWindows_CGaming_CInput_CPreview_CDeviceCommand;

typedef enum __x_ABI_CWindows_CGaming_CInput_CPreview_CGameControllerBatteryChargingState __x_ABI_CWindows_CGaming_CInput_CPreview_CGameControllerBatteryChargingState;

typedef enum __x_ABI_CWindows_CGaming_CInput_CPreview_CGameControllerBatteryKind __x_ABI_CWindows_CGaming_CInput_CPreview_CGameControllerBatteryKind;

typedef enum __x_ABI_CWindows_CGaming_CInput_CPreview_CGameControllerBatteryLevel __x_ABI_CWindows_CGaming_CInput_CPreview_CGameControllerBatteryLevel;

typedef enum __x_ABI_CWindows_CGaming_CInput_CPreview_CGameControllerFirmwareCorruptReason __x_ABI_CWindows_CGaming_CInput_CPreview_CGameControllerFirmwareCorruptReason;

typedef enum __x_ABI_CWindows_CGaming_CInput_CPreview_CHeadsetOperation __x_ABI_CWindows_CGaming_CInput_CPreview_CHeadsetOperation;

/*
 *
 * Struct Windows.Gaming.Input.Preview.DeviceCommand
 *
 * Introduced to Windows.Gaming.Input.GamingInputPreviewContract in version 2.0
 *
 */
#if WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CGaming_CInput_CPreview_CDeviceCommand
{
    DeviceCommand_Reset = 0,
};
#endif // WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Gaming.Input.Preview.GameControllerBatteryChargingState
 *
 * Introduced to Windows.Gaming.Input.GamingInputPreviewContract in version 2.0
 *
 */
#if WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CGaming_CInput_CPreview_CGameControllerBatteryChargingState
{
    GameControllerBatteryChargingState_Unknown = 0,
    GameControllerBatteryChargingState_Inactive = 1,
    GameControllerBatteryChargingState_Active = 2,
    GameControllerBatteryChargingState_Error = 3,
};
#endif // WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Gaming.Input.Preview.GameControllerBatteryKind
 *
 * Introduced to Windows.Gaming.Input.GamingInputPreviewContract in version 2.0
 *
 */
#if WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CGaming_CInput_CPreview_CGameControllerBatteryKind
{
    GameControllerBatteryKind_Unknown = 0,
    GameControllerBatteryKind_None = 1,
    GameControllerBatteryKind_Standard = 2,
    GameControllerBatteryKind_Rechargeable = 3,
};
#endif // WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Gaming.Input.Preview.GameControllerBatteryLevel
 *
 * Introduced to Windows.Gaming.Input.GamingInputPreviewContract in version 2.0
 *
 */
#if WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CGaming_CInput_CPreview_CGameControllerBatteryLevel
{
    GameControllerBatteryLevel_Unknown = 0,
    GameControllerBatteryLevel_Critical = 1,
    GameControllerBatteryLevel_Low = 2,
    GameControllerBatteryLevel_Medium = 3,
    GameControllerBatteryLevel_Full = 4,
};
#endif // WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Gaming.Input.Preview.GameControllerFirmwareCorruptReason
 *
 * Introduced to Windows.Gaming.Input.GamingInputPreviewContract in version 2.0
 *
 */
#if WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CGaming_CInput_CPreview_CGameControllerFirmwareCorruptReason
{
    GameControllerFirmwareCorruptReason_Unknown = 0,
    GameControllerFirmwareCorruptReason_NotCorrupt = 1,
    GameControllerFirmwareCorruptReason_TwoUpCorrupt = 2,
    GameControllerFirmwareCorruptReason_AppCorrupt = 3,
    GameControllerFirmwareCorruptReason_RadioCorrupt = 4,
    GameControllerFirmwareCorruptReason_EepromCorrupt = 5,
    GameControllerFirmwareCorruptReason_SafeToUpdate = 6,
};
#endif // WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Gaming.Input.Preview.HeadsetLevel
 *
 * Introduced to Windows.Gaming.Input.GamingInputPreviewContract in version 2.0
 *
 */
#if WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CGaming_CInput_CPreview_CHeadsetLevel
{
    HeadsetLevel_Off = 0,
    HeadsetLevel_Low = 1,
    HeadsetLevel_Medium = 2,
    HeadsetLevel_High = 3,
};
#endif // WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Gaming.Input.Preview.HeadsetOperation
 *
 * Introduced to Windows.Gaming.Input.GamingInputPreviewContract in version 2.0
 *
 */
#if WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CGaming_CInput_CPreview_CHeadsetOperation
{
    HeadsetOperation_Geq = 0,
    HeadsetOperation_BassBoostGain = 1,
    HeadsetOperation_SmartMute = 2,
    HeadsetOperation_SideTone = 3,
    HeadsetOperation_MuteLedBrightness = 4,
    HeadsetOperation_SwapMixAndVolumeDials = 5,
};
#endif // WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Gaming.Input.Preview.RemappingButtonCategory
 *
 * Introduced to Windows.Gaming.Input.GamingInputPreviewContract in version 2.0
 *
 */
#if WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CGaming_CInput_CPreview_CRemappingButtonCategory
{
    RemappingButtonCategory_ButtonSettings = 0,
    RemappingButtonCategory_AnalogSettings = 1,
    RemappingButtonCategory_VibrationSettings = 2,
    RemappingButtonCategory_ShareShortPress = 3,
    RemappingButtonCategory_ShareShortPressMetaData = 4,
    RemappingButtonCategory_ShareShortPressMetaDataDisplay = 5,
    RemappingButtonCategory_ShareLongPress = 6,
    RemappingButtonCategory_ShareLongPressMetaData = 7,
    RemappingButtonCategory_ShareLongPressMetaDataDisplay = 8,
    RemappingButtonCategory_ShareDoublePress = 9,
    RemappingButtonCategory_ShareDoublePressMetaData = 10,
    RemappingButtonCategory_ShareDoublePressMetaDataDisplay = 11,
};
#endif // WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Gaming.Input.Preview.HeadsetGeqGains
 *
 * Introduced to Windows.Gaming.Input.GamingInputPreviewContract in version 2.0
 *
 */
#if WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000
struct __x_ABI_CWindows_CGaming_CInput_CPreview_CHeadsetGeqGains
{
    INT32 band1Gain;
    INT32 band2Gain;
    INT32 band3Gain;
    INT32 band4Gain;
    INT32 band5Gain;
};
#endif // WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Gaming.Input.Preview.IGameControllerProviderInfoStatics
 *
 * Introduced to Windows.Gaming.Input.GamingInputPreviewContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.Preview.GameControllerProviderInfo
 *
 */
#if WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CPreview_CIGameControllerProviderInfoStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CPreview_CIGameControllerProviderInfoStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_Preview_IGameControllerProviderInfoStatics[] = L"Windows.Gaming.Input.Preview.IGameControllerProviderInfoStatics";
typedef struct __x_ABI_CWindows_CGaming_CInput_CPreview_CIGameControllerProviderInfoStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CPreview_CIGameControllerProviderInfoStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CPreview_CIGameControllerProviderInfoStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CPreview_CIGameControllerProviderInfoStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CPreview_CIGameControllerProviderInfoStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CPreview_CIGameControllerProviderInfoStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CPreview_CIGameControllerProviderInfoStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetParentProviderId)(__x_ABI_CWindows_CGaming_CInput_CPreview_CIGameControllerProviderInfoStatics* This,
        __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider* provider,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* GetProviderId)(__x_ABI_CWindows_CGaming_CInput_CPreview_CIGameControllerProviderInfoStatics* This,
        __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider* provider,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CPreview_CIGameControllerProviderInfoStaticsVtbl;

interface __x_ABI_CWindows_CGaming_CInput_CPreview_CIGameControllerProviderInfoStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CPreview_CIGameControllerProviderInfoStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CIGameControllerProviderInfoStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CIGameControllerProviderInfoStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CIGameControllerProviderInfoStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CIGameControllerProviderInfoStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CIGameControllerProviderInfoStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CIGameControllerProviderInfoStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CIGameControllerProviderInfoStatics_GetParentProviderId(This, provider, value) \
    ((This)->lpVtbl->GetParentProviderId(This, provider, value))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CIGameControllerProviderInfoStatics_GetProviderId(This, provider, value) \
    ((This)->lpVtbl->GetProviderId(This, provider, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CPreview_CIGameControllerProviderInfoStatics;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CPreview_CIGameControllerProviderInfoStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.Input.Preview.ILegacyGipGameControllerProvider
 *
 * Introduced to Windows.Gaming.Input.GamingInputPreviewContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.Preview.LegacyGipGameControllerProvider
 *
 */
#if WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_Preview_ILegacyGipGameControllerProvider[] = L"Windows.Gaming.Input.Preview.ILegacyGipGameControllerProvider";
typedef struct __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BatteryChargingState)(__x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider* This,
        enum __x_ABI_CWindows_CGaming_CInput_CPreview_CGameControllerBatteryChargingState* value);
    HRESULT (STDMETHODCALLTYPE* get_BatteryKind)(__x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider* This,
        enum __x_ABI_CWindows_CGaming_CInput_CPreview_CGameControllerBatteryKind* value);
    HRESULT (STDMETHODCALLTYPE* get_BatteryLevel)(__x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider* This,
        enum __x_ABI_CWindows_CGaming_CInput_CPreview_CGameControllerBatteryLevel* value);
    HRESULT (STDMETHODCALLTYPE* GetDeviceFirmwareCorruptionState)(__x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider* This,
        enum __x_ABI_CWindows_CGaming_CInput_CPreview_CGameControllerFirmwareCorruptReason* value);
    HRESULT (STDMETHODCALLTYPE* get_IsFirmwareCorrupted)(__x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* IsInterfaceSupported)(__x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider* This,
        GUID interfaceId,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsSyntheticDevice)(__x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_PreferredTypes)(__x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* ExecuteCommand)(__x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider* This,
        enum __x_ABI_CWindows_CGaming_CInput_CPreview_CDeviceCommand command);
    HRESULT (STDMETHODCALLTYPE* SetHomeLedIntensity)(__x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider* This,
        BYTE intensity);
    HRESULT (STDMETHODCALLTYPE* GetExtendedDeviceInfo)(__x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider* This,
        UINT32* bufferLength,
        BYTE** buffer);
    HRESULT (STDMETHODCALLTYPE* SetHeadsetOperation)(__x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider* This,
        enum __x_ABI_CWindows_CGaming_CInput_CPreview_CHeadsetOperation operation,
        UINT32 bufferLength,
        BYTE* buffer);
    HRESULT (STDMETHODCALLTYPE* GetHeadsetOperation)(__x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider* This,
        enum __x_ABI_CWindows_CGaming_CInput_CPreview_CHeadsetOperation operation,
        UINT32* bufferLength,
        BYTE** buffer);
    HRESULT (STDMETHODCALLTYPE* get_AppCompatVersion)(__x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* SetStandardControllerButtonRemapping)(__x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        boolean previous,
        __FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable* remapping);
    HRESULT (STDMETHODCALLTYPE* GetStandardControllerButtonRemapping)(__x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        boolean previous,
        __FIMapView_2_Windows__CGaming__CInput__CPreview__CRemappingButtonCategory_IInspectable** remapping);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderVtbl;

interface __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider_get_BatteryChargingState(This, value) \
    ((This)->lpVtbl->get_BatteryChargingState(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider_get_BatteryKind(This, value) \
    ((This)->lpVtbl->get_BatteryKind(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider_get_BatteryLevel(This, value) \
    ((This)->lpVtbl->get_BatteryLevel(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider_GetDeviceFirmwareCorruptionState(This, value) \
    ((This)->lpVtbl->GetDeviceFirmwareCorruptionState(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider_get_IsFirmwareCorrupted(This, value) \
    ((This)->lpVtbl->get_IsFirmwareCorrupted(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider_IsInterfaceSupported(This, interfaceId, value) \
    ((This)->lpVtbl->IsInterfaceSupported(This, interfaceId, value))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider_get_IsSyntheticDevice(This, value) \
    ((This)->lpVtbl->get_IsSyntheticDevice(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider_get_PreferredTypes(This, value) \
    ((This)->lpVtbl->get_PreferredTypes(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider_ExecuteCommand(This, command) \
    ((This)->lpVtbl->ExecuteCommand(This, command))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider_SetHomeLedIntensity(This, intensity) \
    ((This)->lpVtbl->SetHomeLedIntensity(This, intensity))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider_GetExtendedDeviceInfo(This, bufferLength, buffer) \
    ((This)->lpVtbl->GetExtendedDeviceInfo(This, bufferLength, buffer))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider_SetHeadsetOperation(This, operation, bufferLength, buffer) \
    ((This)->lpVtbl->SetHeadsetOperation(This, operation, bufferLength, buffer))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider_GetHeadsetOperation(This, operation, bufferLength, buffer) \
    ((This)->lpVtbl->GetHeadsetOperation(This, operation, bufferLength, buffer))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider_get_AppCompatVersion(This, value) \
    ((This)->lpVtbl->get_AppCompatVersion(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider_SetStandardControllerButtonRemapping(This, user, previous, remapping) \
    ((This)->lpVtbl->SetStandardControllerButtonRemapping(This, user, previous, remapping))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider_GetStandardControllerButtonRemapping(This, user, previous, remapping) \
    ((This)->lpVtbl->GetStandardControllerButtonRemapping(This, user, previous, remapping))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Gaming.Input.Preview.ILegacyGipGameControllerProviderStatics
 *
 * Introduced to Windows.Gaming.Input.GamingInputPreviewContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.Preview.LegacyGipGameControllerProvider
 *
 */
#if WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_Preview_ILegacyGipGameControllerProviderStatics[] = L"Windows.Gaming.Input.Preview.ILegacyGipGameControllerProviderStatics";
typedef struct __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FromGameController)(__x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics* This,
        __x_ABI_CWindows_CGaming_CInput_CIGameController* controller,
        __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider** legacyGipGameControllerProvider);
    HRESULT (STDMETHODCALLTYPE* FromGameControllerProvider)(__x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics* This,
        __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider* provider,
        __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProvider** legacyGipGameControllerProvider);
    HRESULT (STDMETHODCALLTYPE* PairPilotToCopilot)(__x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING pilotControllerProviderId,
        HSTRING copilotControllerProviderId);
    HRESULT (STDMETHODCALLTYPE* ClearPairing)(__x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING controllerProviderId);
    HRESULT (STDMETHODCALLTYPE* IsPilot)(__x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING controllerProviderId,
        HSTRING* copilotControllerProviderId);
    HRESULT (STDMETHODCALLTYPE* IsCopilot)(__x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING controllerProviderId,
        HSTRING* pilotControllerProviderId);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStaticsVtbl;

interface __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics_FromGameController(This, controller, legacyGipGameControllerProvider) \
    ((This)->lpVtbl->FromGameController(This, controller, legacyGipGameControllerProvider))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics_FromGameControllerProvider(This, provider, legacyGipGameControllerProvider) \
    ((This)->lpVtbl->FromGameControllerProvider(This, provider, legacyGipGameControllerProvider))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics_PairPilotToCopilot(This, user, pilotControllerProviderId, copilotControllerProviderId) \
    ((This)->lpVtbl->PairPilotToCopilot(This, user, pilotControllerProviderId, copilotControllerProviderId))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics_ClearPairing(This, user, controllerProviderId) \
    ((This)->lpVtbl->ClearPairing(This, user, controllerProviderId))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics_IsPilot(This, user, controllerProviderId, copilotControllerProviderId) \
    ((This)->lpVtbl->IsPilot(This, user, controllerProviderId, copilotControllerProviderId))

#define __x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics_IsCopilot(This, user, controllerProviderId, pilotControllerProviderId) \
    ((This)->lpVtbl->IsCopilot(This, user, controllerProviderId, pilotControllerProviderId))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CPreview_CILegacyGipGameControllerProviderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Gaming.Input.Preview.GameControllerProviderInfo
 *
 * Introduced to Windows.Gaming.Input.GamingInputPreviewContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Gaming.Input.Preview.IGameControllerProviderInfoStatics interface starting with version 1.0 of the Windows.Gaming.Input.GamingInputPreviewContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_Preview_GameControllerProviderInfo_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_Preview_GameControllerProviderInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_Preview_GameControllerProviderInfo[] = L"Windows.Gaming.Input.Preview.GameControllerProviderInfo";
#endif
#endif // WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.Input.Preview.LegacyGipGameControllerProvider
 *
 * Introduced to Windows.Gaming.Input.GamingInputPreviewContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Gaming.Input.Preview.ILegacyGipGameControllerProviderStatics interface starting with version 2.0 of the Windows.Gaming.Input.GamingInputPreviewContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Input.Preview.ILegacyGipGameControllerProvider ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_Preview_LegacyGipGameControllerProvider_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_Preview_LegacyGipGameControllerProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_Preview_LegacyGipGameControllerProvider[] = L"Windows.Gaming.Input.Preview.LegacyGipGameControllerProvider";
#endif
#endif // WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION >= 0x20000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Egaming2Einput2Epreview_p_h__

#endif // __windows2Egaming2Einput2Epreview_h__
