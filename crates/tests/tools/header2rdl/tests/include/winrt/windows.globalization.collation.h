
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
#ifndef __windows2Eglobalization2Ecollation_h__
#define __windows2Eglobalization2Ecollation_h__
#ifndef __windows2Eglobalization2Ecollation_p_h__
#define __windows2Eglobalization2Ecollation_p_h__


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
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace Collation {
                interface ICharacterGrouping;
            } /* Collation */
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping ABI::Windows::Globalization::Collation::ICharacterGrouping

#endif // ____x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupings_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupings_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace Collation {
                interface ICharacterGroupings;
            } /* Collation */
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupings ABI::Windows::Globalization::Collation::ICharacterGroupings

#endif // ____x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace Collation {
                interface ICharacterGroupingsFactory;
            } /* Collation */
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsFactory ABI::Windows::Globalization::Collation::ICharacterGroupingsFactory

#endif // ____x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace Collation {
                class CharacterGrouping;
            } /* Collation */
        } /* Globalization */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CGlobalization__CCollation__CCharacterGrouping_USE
#define DEF___FIIterator_1_Windows__CGlobalization__CCollation__CCharacterGrouping_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("57e89bbc-9220-5df2-844b-ddfe6605db5f"))
IIterator<ABI::Windows::Globalization::Collation::CharacterGrouping*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Globalization::Collation::CharacterGrouping*, ABI::Windows::Globalization::Collation::ICharacterGrouping*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Globalization.Collation.CharacterGrouping>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Globalization::Collation::CharacterGrouping*> __FIIterator_1_Windows__CGlobalization__CCollation__CCharacterGrouping_t;
#define __FIIterator_1_Windows__CGlobalization__CCollation__CCharacterGrouping ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGlobalization__CCollation__CCharacterGrouping_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGlobalization__CCollation__CCharacterGrouping_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CGlobalization__CCollation__CCharacterGrouping_USE
#define DEF___FIIterable_1_Windows__CGlobalization__CCollation__CCharacterGrouping_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("82e3abf0-06e3-5609-ba39-c51eb2f5fae6"))
IIterable<ABI::Windows::Globalization::Collation::CharacterGrouping*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Globalization::Collation::CharacterGrouping*, ABI::Windows::Globalization::Collation::ICharacterGrouping*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Globalization.Collation.CharacterGrouping>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Globalization::Collation::CharacterGrouping*> __FIIterable_1_Windows__CGlobalization__CCollation__CCharacterGrouping_t;
#define __FIIterable_1_Windows__CGlobalization__CCollation__CCharacterGrouping ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGlobalization__CCollation__CCharacterGrouping_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGlobalization__CCollation__CCharacterGrouping_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CGlobalization__CCollation__CCharacterGrouping_USE
#define DEF___FIVectorView_1_Windows__CGlobalization__CCollation__CCharacterGrouping_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f7cf5a4a-2b7a-5bc9-a0c4-9dce07ff61c9"))
IVectorView<ABI::Windows::Globalization::Collation::CharacterGrouping*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Globalization::Collation::CharacterGrouping*, ABI::Windows::Globalization::Collation::ICharacterGrouping*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Globalization.Collation.CharacterGrouping>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Globalization::Collation::CharacterGrouping*> __FIVectorView_1_Windows__CGlobalization__CCollation__CCharacterGrouping_t;
#define __FIVectorView_1_Windows__CGlobalization__CCollation__CCharacterGrouping ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGlobalization__CCollation__CCharacterGrouping_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGlobalization__CCollation__CCharacterGrouping_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace Collation {
                class CharacterGroupings;
            } /* Collation */
        } /* Globalization */
    } /* Windows */
} /* ABI */

/*
 *
 * Interface Windows.Globalization.Collation.ICharacterGrouping
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.Collation.CharacterGrouping
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_Collation_ICharacterGrouping[] = L"Windows.Globalization.Collation.ICharacterGrouping";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace Collation {
                MIDL_INTERFACE("fae761bb-805d-4bb0-95bb-c1f7c3e8eb8e")
                ICharacterGrouping : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_First(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Label(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICharacterGrouping = __uuidof(ICharacterGrouping);
            } /* Collation */
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.Collation.ICharacterGroupings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.Collation.CharacterGroupings
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.Collections.IVectorView`1<Windows.Globalization.Collation.CharacterGrouping>
 *     Windows.Foundation.Collections.IIterable`1<Windows.Globalization.Collation.CharacterGrouping>
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_Collation_ICharacterGroupings[] = L"Windows.Globalization.Collation.ICharacterGroupings";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace Collation {
                MIDL_INTERFACE("b8d20a75-d4cf-4055-80e5-ce169c226496")
                ICharacterGroupings : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Lookup(
                        HSTRING text,
                        HSTRING* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICharacterGroupings = __uuidof(ICharacterGroupings);
            } /* Collation */
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupings;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.Collation.ICharacterGroupingsFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.Collation.CharacterGroupings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_Collation_ICharacterGroupingsFactory[] = L"Windows.Globalization.Collation.ICharacterGroupingsFactory";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace Collation {
                MIDL_INTERFACE("99ea9fd9-886d-4401-9f98-69c82d4c2f78")
                ICharacterGroupingsFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        HSTRING language,
                        ABI::Windows::Globalization::Collation::ICharacterGroupings** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICharacterGroupingsFactory = __uuidof(ICharacterGroupingsFactory);
            } /* Collation */
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsFactory;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Globalization.Collation.CharacterGrouping
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Globalization.Collation.ICharacterGrouping ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_Collation_CharacterGrouping_DEFINED
#define RUNTIMECLASS_Windows_Globalization_Collation_CharacterGrouping_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_Collation_CharacterGrouping[] = L"Windows.Globalization.Collation.CharacterGrouping";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.Collation.CharacterGroupings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Globalization.Collation.ICharacterGroupingsFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Globalization.Collation.ICharacterGroupings ** Default Interface **
 *    Windows.Foundation.Collections.IVectorView`1<Windows.Globalization.Collation.CharacterGrouping>
 *    Windows.Foundation.Collections.IIterable`1<Windows.Globalization.Collation.CharacterGrouping>
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_Collation_CharacterGroupings_DEFINED
#define RUNTIMECLASS_Windows_Globalization_Collation_CharacterGroupings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_Collation_CharacterGroupings[] = L"Windows.Globalization.Collation.CharacterGroupings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping;

#endif // ____x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupings_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupings_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupings __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupings;

#endif // ____x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsFactory __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsFactory;

#endif // ____x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CGlobalization__CCollation__CCharacterGrouping_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGlobalization__CCollation__CCharacterGrouping_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGlobalization__CCollation__CCharacterGrouping __FIIterator_1_Windows__CGlobalization__CCollation__CCharacterGrouping;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGlobalization__CCollation__CCharacterGrouping;

typedef struct __FIIterator_1_Windows__CGlobalization__CCollation__CCharacterGroupingVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGlobalization__CCollation__CCharacterGrouping* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGlobalization__CCollation__CCharacterGrouping* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGlobalization__CCollation__CCharacterGrouping* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGlobalization__CCollation__CCharacterGrouping* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGlobalization__CCollation__CCharacterGrouping* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGlobalization__CCollation__CCharacterGrouping* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGlobalization__CCollation__CCharacterGrouping* This,
        __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGlobalization__CCollation__CCharacterGrouping* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGlobalization__CCollation__CCharacterGrouping* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGlobalization__CCollation__CCharacterGrouping* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGlobalization__CCollation__CCharacterGroupingVtbl;

interface __FIIterator_1_Windows__CGlobalization__CCollation__CCharacterGrouping
{
    CONST_VTBL struct __FIIterator_1_Windows__CGlobalization__CCollation__CCharacterGroupingVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGlobalization__CCollation__CCharacterGrouping_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGlobalization__CCollation__CCharacterGrouping_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGlobalization__CCollation__CCharacterGrouping_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGlobalization__CCollation__CCharacterGrouping_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGlobalization__CCollation__CCharacterGrouping_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGlobalization__CCollation__CCharacterGrouping_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGlobalization__CCollation__CCharacterGrouping_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGlobalization__CCollation__CCharacterGrouping_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGlobalization__CCollation__CCharacterGrouping_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGlobalization__CCollation__CCharacterGrouping_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGlobalization__CCollation__CCharacterGrouping_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CGlobalization__CCollation__CCharacterGrouping_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGlobalization__CCollation__CCharacterGrouping_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGlobalization__CCollation__CCharacterGrouping __FIIterable_1_Windows__CGlobalization__CCollation__CCharacterGrouping;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGlobalization__CCollation__CCharacterGrouping;

typedef struct __FIIterable_1_Windows__CGlobalization__CCollation__CCharacterGroupingVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGlobalization__CCollation__CCharacterGrouping* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGlobalization__CCollation__CCharacterGrouping* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGlobalization__CCollation__CCharacterGrouping* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGlobalization__CCollation__CCharacterGrouping* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGlobalization__CCollation__CCharacterGrouping* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGlobalization__CCollation__CCharacterGrouping* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGlobalization__CCollation__CCharacterGrouping* This,
        __FIIterator_1_Windows__CGlobalization__CCollation__CCharacterGrouping** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGlobalization__CCollation__CCharacterGroupingVtbl;

interface __FIIterable_1_Windows__CGlobalization__CCollation__CCharacterGrouping
{
    CONST_VTBL struct __FIIterable_1_Windows__CGlobalization__CCollation__CCharacterGroupingVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGlobalization__CCollation__CCharacterGrouping_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGlobalization__CCollation__CCharacterGrouping_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGlobalization__CCollation__CCharacterGrouping_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGlobalization__CCollation__CCharacterGrouping_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGlobalization__CCollation__CCharacterGrouping_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGlobalization__CCollation__CCharacterGrouping_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGlobalization__CCollation__CCharacterGrouping_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGlobalization__CCollation__CCharacterGrouping_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CGlobalization__CCollation__CCharacterGrouping_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGlobalization__CCollation__CCharacterGrouping_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGlobalization__CCollation__CCharacterGrouping __FIVectorView_1_Windows__CGlobalization__CCollation__CCharacterGrouping;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGlobalization__CCollation__CCharacterGrouping;

typedef struct __FIVectorView_1_Windows__CGlobalization__CCollation__CCharacterGroupingVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGlobalization__CCollation__CCharacterGrouping* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGlobalization__CCollation__CCharacterGrouping* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGlobalization__CCollation__CCharacterGrouping* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGlobalization__CCollation__CCharacterGrouping* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGlobalization__CCollation__CCharacterGrouping* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGlobalization__CCollation__CCharacterGrouping* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGlobalization__CCollation__CCharacterGrouping* This,
        UINT32 index,
        __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGlobalization__CCollation__CCharacterGrouping* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGlobalization__CCollation__CCharacterGrouping* This,
        __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGlobalization__CCollation__CCharacterGrouping* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGlobalization__CCollation__CCharacterGroupingVtbl;

interface __FIVectorView_1_Windows__CGlobalization__CCollation__CCharacterGrouping
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGlobalization__CCollation__CCharacterGroupingVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGlobalization__CCollation__CCharacterGrouping_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGlobalization__CCollation__CCharacterGrouping_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGlobalization__CCollation__CCharacterGrouping_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGlobalization__CCollation__CCharacterGrouping_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGlobalization__CCollation__CCharacterGrouping_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGlobalization__CCollation__CCharacterGrouping_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGlobalization__CCollation__CCharacterGrouping_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGlobalization__CCollation__CCharacterGrouping_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGlobalization__CCollation__CCharacterGrouping_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGlobalization__CCollation__CCharacterGrouping_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGlobalization__CCollation__CCharacterGrouping_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.Collation.ICharacterGrouping
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.Collation.CharacterGrouping
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_Collation_ICharacterGrouping[] = L"Windows.Globalization.Collation.ICharacterGrouping";
typedef struct __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_First)(__x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Label)(__x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingVtbl;

interface __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping_get_First(This, value) \
    ((This)->lpVtbl->get_First(This, value))

#define __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping_get_Label(This, value) \
    ((This)->lpVtbl->get_Label(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CCollation_CICharacterGrouping_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.Collation.ICharacterGroupings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.Collation.CharacterGroupings
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.Collections.IVectorView`1<Windows.Globalization.Collation.CharacterGrouping>
 *     Windows.Foundation.Collections.IIterable`1<Windows.Globalization.Collation.CharacterGrouping>
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_Collation_ICharacterGroupings[] = L"Windows.Globalization.Collation.ICharacterGroupings";
typedef struct __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupings* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupings* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupings* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupings* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupings* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupings* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupings* This,
        HSTRING text,
        HSTRING* result);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsVtbl;

interface __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupings
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupings_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupings_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupings_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupings_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupings_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupings_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupings_Lookup(This, text, result) \
    ((This)->lpVtbl->Lookup(This, text, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupings;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.Collation.ICharacterGroupingsFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.Collation.CharacterGroupings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_Collation_ICharacterGroupingsFactory[] = L"Windows.Globalization.Collation.ICharacterGroupingsFactory";
typedef struct __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsFactory* This,
        HSTRING language,
        __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupings** result);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsFactoryVtbl;

interface __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsFactory_Create(This, language, result) \
    ((This)->lpVtbl->Create(This, language, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsFactory;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CCollation_CICharacterGroupingsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Globalization.Collation.CharacterGrouping
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Globalization.Collation.ICharacterGrouping ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_Collation_CharacterGrouping_DEFINED
#define RUNTIMECLASS_Windows_Globalization_Collation_CharacterGrouping_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_Collation_CharacterGrouping[] = L"Windows.Globalization.Collation.CharacterGrouping";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.Collation.CharacterGroupings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Globalization.Collation.ICharacterGroupingsFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Globalization.Collation.ICharacterGroupings ** Default Interface **
 *    Windows.Foundation.Collections.IVectorView`1<Windows.Globalization.Collation.CharacterGrouping>
 *    Windows.Foundation.Collections.IIterable`1<Windows.Globalization.Collation.CharacterGrouping>
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_Collation_CharacterGroupings_DEFINED
#define RUNTIMECLASS_Windows_Globalization_Collation_CharacterGroupings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_Collation_CharacterGroupings[] = L"Windows.Globalization.Collation.CharacterGroupings";
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
#endif // __windows2Eglobalization2Ecollation_p_h__

#endif // __windows2Eglobalization2Ecollation_h__
