
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
#ifndef __windows2Eapplicationmodel2Elockscreen_h__
#define __windows2Eapplicationmodel2Elockscreen_h__
#ifndef __windows2Eapplicationmodel2Elockscreen_p_h__
#define __windows2Eapplicationmodel2Elockscreen_p_h__


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

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace LockScreen {
                interface ILockApplicationHost;
            } /* LockScreen */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost ABI::Windows::ApplicationModel::LockScreen::ILockApplicationHost

#endif // ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace LockScreen {
                interface ILockApplicationHostStatics;
            } /* LockScreen */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostStatics ABI::Windows::ApplicationModel::LockScreen::ILockApplicationHostStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace LockScreen {
                interface ILockScreenBadge;
            } /* LockScreen */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge ABI::Windows::ApplicationModel::LockScreen::ILockScreenBadge

#endif // ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace LockScreen {
                interface ILockScreenInfo;
            } /* LockScreen */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo ABI::Windows::ApplicationModel::LockScreen::ILockScreenInfo

#endif // ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingDeferral_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace LockScreen {
                interface ILockScreenUnlockingDeferral;
            } /* LockScreen */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingDeferral ABI::Windows::ApplicationModel::LockScreen::ILockScreenUnlockingDeferral

#endif // ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace LockScreen {
                interface ILockScreenUnlockingEventArgs;
            } /* LockScreen */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgs ABI::Windows::ApplicationModel::LockScreen::ILockScreenUnlockingEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgs_FWD_DEFINED__

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
        namespace ApplicationModel {
            namespace LockScreen {
                class LockScreenBadge;
            } /* LockScreen */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8d38f924-154d-5705-8f0b-ed61353f6ce2"))
IIterator<ABI::Windows::ApplicationModel::LockScreen::LockScreenBadge*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::LockScreen::LockScreenBadge*, ABI::Windows::ApplicationModel::LockScreen::ILockScreenBadge*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.LockScreen.LockScreenBadge>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::LockScreen::LockScreenBadge*> __FIIterator_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_t;
#define __FIIterator_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6e82dedc-b74e-503a-b00b-9c6f47f12a0f"))
IIterable<ABI::Windows::ApplicationModel::LockScreen::LockScreenBadge*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::LockScreen::LockScreenBadge*, ABI::Windows::ApplicationModel::LockScreen::ILockScreenBadge*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.LockScreen.LockScreenBadge>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::LockScreen::LockScreenBadge*> __FIIterable_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_t;
#define __FIIterable_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


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


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a0734995-94c2-50c2-88a2-d070fcd1d338"))
IVectorView<ABI::Windows::ApplicationModel::LockScreen::LockScreenBadge*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::LockScreen::LockScreenBadge*, ABI::Windows::ApplicationModel::LockScreen::ILockScreenBadge*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.LockScreen.LockScreenBadge>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::LockScreen::LockScreenBadge*> __FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_t;
#define __FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIReference_1_UINT32_USE
#define DEF___FIReference_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("513ef3af-e784-5325-a91e-97c2b8111cf3"))
IReference<UINT32> : IReference_impl<UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<UINT32> __FIReference_1_UINT32_t;
#define __FIReference_1_UINT32 ABI::Windows::Foundation::__FIReference_1_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_UINT32_USE */


namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace LockScreen {
                class LockApplicationHost;
            } /* LockScreen */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace LockScreen {
                class LockScreenUnlockingEventArgs;
            } /* LockScreen */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockApplicationHost_Windows__CApplicationModel__CLockScreen__CLockScreenUnlockingEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockApplicationHost_Windows__CApplicationModel__CLockScreen__CLockScreenUnlockingEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("002e5776-8a5b-5b93-8c6c-9c4c8788f5b4"))
ITypedEventHandler<ABI::Windows::ApplicationModel::LockScreen::LockApplicationHost*, ABI::Windows::ApplicationModel::LockScreen::LockScreenUnlockingEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::LockScreen::LockApplicationHost*, ABI::Windows::ApplicationModel::LockScreen::ILockApplicationHost*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::LockScreen::LockScreenUnlockingEventArgs*, ABI::Windows::ApplicationModel::LockScreen::ILockScreenUnlockingEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.LockScreen.LockApplicationHost, Windows.ApplicationModel.LockScreen.LockScreenUnlockingEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::LockScreen::LockApplicationHost*, ABI::Windows::ApplicationModel::LockScreen::LockScreenUnlockingEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockApplicationHost_Windows__CApplicationModel__CLockScreen__CLockScreenUnlockingEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockApplicationHost_Windows__CApplicationModel__CLockScreen__CLockScreenUnlockingEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockApplicationHost_Windows__CApplicationModel__CLockScreen__CLockScreenUnlockingEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockApplicationHost_Windows__CApplicationModel__CLockScreen__CLockScreenUnlockingEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace LockScreen {
                class LockScreenInfo;
            } /* LockScreen */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockScreenInfo_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockScreenInfo_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8be9e6a3-f88a-5429-8da3-676b7d4f1a5b"))
ITypedEventHandler<ABI::Windows::ApplicationModel::LockScreen::LockScreenInfo*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::LockScreen::LockScreenInfo*, ABI::Windows::ApplicationModel::LockScreen::ILockScreenInfo*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.LockScreen.LockScreenInfo, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::LockScreen::LockScreenInfo*, IInspectable*> __FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockScreenInfo_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockScreenInfo_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockScreenInfo_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockScreenInfo_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct DateTime DateTime;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IPropertyValue;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIPropertyValue ABI::Windows::Foundation::IPropertyValue

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IRandomAccessStream;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream ABI::Windows::Storage::Streams::IRandomAccessStream

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace LockScreen {
                class LockScreenUnlockingDeferral;
            } /* LockScreen */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Interface Windows.ApplicationModel.LockScreen.ILockApplicationHost
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.LockScreen.LockApplicationHost
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_LockScreen_ILockApplicationHost[] = L"Windows.ApplicationModel.LockScreen.ILockApplicationHost";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace LockScreen {
                MIDL_INTERFACE("38ee31ad-d94f-4e7c-81fa-4f4436506281")
                ILockApplicationHost : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE RequestUnlock(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Unlocking(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockApplicationHost_Windows__CApplicationModel__CLockScreen__CLockScreenUnlockingEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Unlocking(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILockApplicationHost = __uuidof(ILockApplicationHost);
            } /* LockScreen */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.LockScreen.ILockApplicationHostStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.LockScreen.LockApplicationHost
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_LockScreen_ILockApplicationHostStatics[] = L"Windows.ApplicationModel.LockScreen.ILockApplicationHostStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace LockScreen {
                MIDL_INTERFACE("f48fab8e-23d7-4e63-96a1-666ff52d3b2c")
                ILockApplicationHostStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForCurrentView(
                        ABI::Windows::ApplicationModel::LockScreen::ILockApplicationHost** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILockApplicationHostStatics = __uuidof(ILockApplicationHostStatics);
            } /* LockScreen */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.LockScreen.ILockScreenBadge
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.LockScreen.LockScreenBadge
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_LockScreen_ILockScreenBadge[] = L"Windows.ApplicationModel.LockScreen.ILockScreenBadge";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace LockScreen {
                MIDL_INTERFACE("e95105d9-2bff-4db0-9b4f-3824778b9c9a")
                ILockScreenBadge : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Logo(
                        ABI::Windows::Storage::Streams::IRandomAccessStream** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Glyph(
                        ABI::Windows::Storage::Streams::IRandomAccessStream** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Number(
                        __FIReference_1_UINT32** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AutomationName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE LaunchApp(void) = 0;
                };

                MIDL_CONST_ID IID& IID_ILockScreenBadge = __uuidof(ILockScreenBadge);
            } /* LockScreen */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.LockScreen.ILockScreenInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.LockScreen.LockScreenInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_LockScreen_ILockScreenInfo[] = L"Windows.ApplicationModel.LockScreen.ILockScreenInfo";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace LockScreen {
                MIDL_INTERFACE("f59aa65c-9711-4dc9-a630-95b6cb8cdad0")
                ILockScreenInfo : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_LockScreenImageChanged(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockScreenInfo_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_LockScreenImageChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LockScreenImage(
                        ABI::Windows::Storage::Streams::IRandomAccessStream** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_BadgesChanged(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockScreenInfo_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_BadgesChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Badges(
                        __FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_DetailTextChanged(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockScreenInfo_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_DetailTextChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DetailText(
                        __FIVectorView_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_AlarmIconChanged(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockScreenInfo_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_AlarmIconChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AlarmIcon(
                        ABI::Windows::Storage::Streams::IRandomAccessStream** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILockScreenInfo = __uuidof(ILockScreenInfo);
            } /* LockScreen */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.LockScreen.ILockScreenUnlockingDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.LockScreen.LockScreenUnlockingDeferral
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_LockScreen_ILockScreenUnlockingDeferral[] = L"Windows.ApplicationModel.LockScreen.ILockScreenUnlockingDeferral";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace LockScreen {
                MIDL_INTERFACE("7e7d1ad6-5203-43e7-9bd6-7c3947d1e3fe")
                ILockScreenUnlockingDeferral : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Complete(void) = 0;
                };

                MIDL_CONST_ID IID& IID_ILockScreenUnlockingDeferral = __uuidof(ILockScreenUnlockingDeferral);
            } /* LockScreen */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingDeferral;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.LockScreen.ILockScreenUnlockingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.LockScreen.LockScreenUnlockingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_LockScreen_ILockScreenUnlockingEventArgs[] = L"Windows.ApplicationModel.LockScreen.ILockScreenUnlockingEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace LockScreen {
                MIDL_INTERFACE("44e6c007-75fb-4abb-9f8b-824748900c71")
                ILockScreenUnlockingEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                        ABI::Windows::ApplicationModel::LockScreen::ILockScreenUnlockingDeferral** deferral
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Deadline(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILockScreenUnlockingEventArgs = __uuidof(ILockScreenUnlockingEventArgs);
            } /* LockScreen */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.LockScreen.LockApplicationHost
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.LockScreen.ILockApplicationHostStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.LockScreen.ILockApplicationHost ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_LockScreen_LockApplicationHost_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_LockScreen_LockApplicationHost_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_LockScreen_LockApplicationHost[] = L"Windows.ApplicationModel.LockScreen.LockApplicationHost";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.LockScreen.LockScreenBadge
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.LockScreen.ILockScreenBadge ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_LockScreen_LockScreenBadge_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_LockScreen_LockScreenBadge_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_LockScreen_LockScreenBadge[] = L"Windows.ApplicationModel.LockScreen.LockScreenBadge";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.LockScreen.LockScreenInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.LockScreen.ILockScreenInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_LockScreen_LockScreenInfo_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_LockScreen_LockScreenInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_LockScreen_LockScreenInfo[] = L"Windows.ApplicationModel.LockScreen.LockScreenInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.LockScreen.LockScreenUnlockingDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.LockScreen.ILockScreenUnlockingDeferral ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_LockScreen_LockScreenUnlockingDeferral_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_LockScreen_LockScreenUnlockingDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_LockScreen_LockScreenUnlockingDeferral[] = L"Windows.ApplicationModel.LockScreen.LockScreenUnlockingDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.LockScreen.LockScreenUnlockingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.LockScreen.ILockScreenUnlockingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_LockScreen_LockScreenUnlockingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_LockScreen_LockScreenUnlockingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_LockScreen_LockScreenUnlockingEventArgs[] = L"Windows.ApplicationModel.LockScreen.LockScreenUnlockingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost;

#endif // ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostStatics __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge;

#endif // ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo;

#endif // ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingDeferral __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingDeferral;

#endif // ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgs __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgs_FWD_DEFINED__

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge __FIIterator_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge;

typedef struct __FIIterator_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadgeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge* This,
        __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadgeVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadgeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge __FIIterable_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge;

typedef struct __FIIterable_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadgeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge* This,
        __FIIterator_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadgeVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadgeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge __FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadgeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge* This,
        __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadgeVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadgeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIReference_1_UINT32_INTERFACE_DEFINED__)
#define ____FIReference_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIReference_1_UINT32 __FIReference_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_UINT32;

typedef struct __FIReference_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_UINT32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_UINT32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_UINT32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_UINT32* This,
        UINT32* result);

    END_INTERFACE
} __FIReference_1_UINT32Vtbl;

interface __FIReference_1_UINT32
{
    CONST_VTBL struct __FIReference_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_UINT32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_UINT32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_UINT32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_UINT32_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_UINT32_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockApplicationHost_Windows__CApplicationModel__CLockScreen__CLockScreenUnlockingEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockApplicationHost_Windows__CApplicationModel__CLockScreen__CLockScreenUnlockingEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockApplicationHost_Windows__CApplicationModel__CLockScreen__CLockScreenUnlockingEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockApplicationHost_Windows__CApplicationModel__CLockScreen__CLockScreenUnlockingEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockApplicationHost_Windows__CApplicationModel__CLockScreen__CLockScreenUnlockingEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockApplicationHost_Windows__CApplicationModel__CLockScreen__CLockScreenUnlockingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockApplicationHost_Windows__CApplicationModel__CLockScreen__CLockScreenUnlockingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockApplicationHost_Windows__CApplicationModel__CLockScreen__CLockScreenUnlockingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockApplicationHost_Windows__CApplicationModel__CLockScreen__CLockScreenUnlockingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockApplicationHost_Windows__CApplicationModel__CLockScreen__CLockScreenUnlockingEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost* sender,
        __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockApplicationHost_Windows__CApplicationModel__CLockScreen__CLockScreenUnlockingEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockApplicationHost_Windows__CApplicationModel__CLockScreen__CLockScreenUnlockingEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockApplicationHost_Windows__CApplicationModel__CLockScreen__CLockScreenUnlockingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockApplicationHost_Windows__CApplicationModel__CLockScreen__CLockScreenUnlockingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockApplicationHost_Windows__CApplicationModel__CLockScreen__CLockScreenUnlockingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockApplicationHost_Windows__CApplicationModel__CLockScreen__CLockScreenUnlockingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockApplicationHost_Windows__CApplicationModel__CLockScreen__CLockScreenUnlockingEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockApplicationHost_Windows__CApplicationModel__CLockScreen__CLockScreenUnlockingEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockScreenInfo_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockScreenInfo_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockScreenInfo_IInspectable __FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockScreenInfo_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockScreenInfo_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockScreenInfo_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockScreenInfo_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockScreenInfo_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockScreenInfo_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockScreenInfo_IInspectable* This,
        __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockScreenInfo_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockScreenInfo_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockScreenInfo_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockScreenInfo_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockScreenInfo_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockScreenInfo_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockScreenInfo_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockScreenInfo_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__

/*
 *
 * Interface Windows.ApplicationModel.LockScreen.ILockApplicationHost
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.LockScreen.LockApplicationHost
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_LockScreen_ILockApplicationHost[] = L"Windows.ApplicationModel.LockScreen.ILockApplicationHost";
typedef struct __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RequestUnlock)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost* This);
    HRESULT (STDMETHODCALLTYPE* add_Unlocking)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockApplicationHost_Windows__CApplicationModel__CLockScreen__CLockScreenUnlockingEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Unlocking)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostVtbl;

interface __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost_RequestUnlock(This) \
    ((This)->lpVtbl->RequestUnlock(This))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost_add_Unlocking(This, handler, token) \
    ((This)->lpVtbl->add_Unlocking(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost_remove_Unlocking(This, token) \
    ((This)->lpVtbl->remove_Unlocking(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.LockScreen.ILockApplicationHostStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.LockScreen.LockApplicationHost
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_LockScreen_ILockApplicationHostStatics[] = L"Windows.ApplicationModel.LockScreen.ILockApplicationHostStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentView)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostStatics* This,
        __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHost** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostStatics_GetForCurrentView(This, result) \
    ((This)->lpVtbl->GetForCurrentView(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockApplicationHostStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.LockScreen.ILockScreenBadge
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.LockScreen.LockScreenBadge
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_LockScreen_ILockScreenBadge[] = L"Windows.ApplicationModel.LockScreen.ILockScreenBadge";
typedef struct __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadgeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Logo)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream** value);
    HRESULT (STDMETHODCALLTYPE* get_Glyph)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream** value);
    HRESULT (STDMETHODCALLTYPE* get_Number)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge* This,
        __FIReference_1_UINT32** value);
    HRESULT (STDMETHODCALLTYPE* get_AutomationName)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* LaunchApp)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge* This);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadgeVtbl;

interface __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadgeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge_get_Logo(This, value) \
    ((This)->lpVtbl->get_Logo(This, value))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge_get_Glyph(This, value) \
    ((This)->lpVtbl->get_Glyph(This, value))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge_get_Number(This, value) \
    ((This)->lpVtbl->get_Number(This, value))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge_get_AutomationName(This, value) \
    ((This)->lpVtbl->get_AutomationName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge_LaunchApp(This) \
    ((This)->lpVtbl->LaunchApp(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenBadge_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.LockScreen.ILockScreenInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.LockScreen.LockScreenInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_LockScreen_ILockScreenInfo[] = L"Windows.ApplicationModel.LockScreen.ILockScreenInfo";
typedef struct __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_LockScreenImageChanged)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockScreenInfo_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_LockScreenImageChanged)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_LockScreenImage)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream** value);
    HRESULT (STDMETHODCALLTYPE* add_BadgesChanged)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockScreenInfo_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_BadgesChanged)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_Badges)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo* This,
        __FIVectorView_1_Windows__CApplicationModel__CLockScreen__CLockScreenBadge** value);
    HRESULT (STDMETHODCALLTYPE* add_DetailTextChanged)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockScreenInfo_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_DetailTextChanged)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_DetailText)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* add_AlarmIconChanged)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CLockScreen__CLockScreenInfo_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_AlarmIconChanged)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_AlarmIcon)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfoVtbl;

interface __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo_add_LockScreenImageChanged(This, handler, token) \
    ((This)->lpVtbl->add_LockScreenImageChanged(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo_remove_LockScreenImageChanged(This, token) \
    ((This)->lpVtbl->remove_LockScreenImageChanged(This, token))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo_get_LockScreenImage(This, value) \
    ((This)->lpVtbl->get_LockScreenImage(This, value))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo_add_BadgesChanged(This, handler, token) \
    ((This)->lpVtbl->add_BadgesChanged(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo_remove_BadgesChanged(This, token) \
    ((This)->lpVtbl->remove_BadgesChanged(This, token))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo_get_Badges(This, value) \
    ((This)->lpVtbl->get_Badges(This, value))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo_add_DetailTextChanged(This, handler, token) \
    ((This)->lpVtbl->add_DetailTextChanged(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo_remove_DetailTextChanged(This, token) \
    ((This)->lpVtbl->remove_DetailTextChanged(This, token))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo_get_DetailText(This, value) \
    ((This)->lpVtbl->get_DetailText(This, value))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo_add_AlarmIconChanged(This, handler, token) \
    ((This)->lpVtbl->add_AlarmIconChanged(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo_remove_AlarmIconChanged(This, token) \
    ((This)->lpVtbl->remove_AlarmIconChanged(This, token))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo_get_AlarmIcon(This, value) \
    ((This)->lpVtbl->get_AlarmIcon(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.LockScreen.ILockScreenUnlockingDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.LockScreen.LockScreenUnlockingDeferral
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_LockScreen_ILockScreenUnlockingDeferral[] = L"Windows.ApplicationModel.LockScreen.ILockScreenUnlockingDeferral";
typedef struct __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingDeferralVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingDeferral* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingDeferral* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingDeferral* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingDeferral* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingDeferral* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingDeferral* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Complete)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingDeferral* This);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingDeferralVtbl;

interface __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingDeferral
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingDeferralVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingDeferral_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingDeferral_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingDeferral_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingDeferral_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingDeferral_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingDeferral_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingDeferral_Complete(This) \
    ((This)->lpVtbl->Complete(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingDeferral;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.LockScreen.ILockScreenUnlockingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.LockScreen.LockScreenUnlockingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_LockScreen_ILockScreenUnlockingEventArgs[] = L"Windows.ApplicationModel.LockScreen.ILockScreenUnlockingEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingDeferral** deferral);
    HRESULT (STDMETHODCALLTYPE* get_Deadline)(__x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgs_GetDeferral(This, deferral) \
    ((This)->lpVtbl->GetDeferral(This, deferral))

#define __x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgs_get_Deadline(This, value) \
    ((This)->lpVtbl->get_Deadline(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CLockScreen_CILockScreenUnlockingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.LockScreen.LockApplicationHost
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.LockScreen.ILockApplicationHostStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.LockScreen.ILockApplicationHost ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_LockScreen_LockApplicationHost_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_LockScreen_LockApplicationHost_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_LockScreen_LockApplicationHost[] = L"Windows.ApplicationModel.LockScreen.LockApplicationHost";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.LockScreen.LockScreenBadge
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.LockScreen.ILockScreenBadge ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_LockScreen_LockScreenBadge_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_LockScreen_LockScreenBadge_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_LockScreen_LockScreenBadge[] = L"Windows.ApplicationModel.LockScreen.LockScreenBadge";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.LockScreen.LockScreenInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.LockScreen.ILockScreenInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_LockScreen_LockScreenInfo_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_LockScreen_LockScreenInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_LockScreen_LockScreenInfo[] = L"Windows.ApplicationModel.LockScreen.LockScreenInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.LockScreen.LockScreenUnlockingDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.LockScreen.ILockScreenUnlockingDeferral ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_LockScreen_LockScreenUnlockingDeferral_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_LockScreen_LockScreenUnlockingDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_LockScreen_LockScreenUnlockingDeferral[] = L"Windows.ApplicationModel.LockScreen.LockScreenUnlockingDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.LockScreen.LockScreenUnlockingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.LockScreen.ILockScreenUnlockingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_LockScreen_LockScreenUnlockingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_LockScreen_LockScreenUnlockingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_LockScreen_LockScreenUnlockingEventArgs[] = L"Windows.ApplicationModel.LockScreen.LockScreenUnlockingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eapplicationmodel2Elockscreen_p_h__

#endif // __windows2Eapplicationmodel2Elockscreen_h__
