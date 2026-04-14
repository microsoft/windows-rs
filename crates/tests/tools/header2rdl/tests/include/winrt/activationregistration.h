/* Header file automatically generated from activationregistration.idl */
/*
 * File built with Microsoft(R) MIDLRT Compiler Engine Version 10.00.0231 
 */

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
#ifndef __activationregistration_h__
#define __activationregistration_h__
#ifndef __activationregistration_p_h__
#define __activationregistration_p_h__


#pragma once

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
#if !defined(WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION 0x70000
#endif // defined(WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION)

#if !defined(WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION)
#define WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION 0x40000
#endif // defined(WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION)

#if !defined(WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION)
#define WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION 0x130000
#endif // defined(WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION)

#if !defined(WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION)
#define WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION 0x30000
#endif // defined(WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION)

#if !defined(WINDOWS_PHONE_PHONECONTRACT_VERSION)
#define WINDOWS_PHONE_PHONECONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_PHONE_PHONECONTRACT_VERSION)

#if !defined(WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION)
#define WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION)

#if !defined(WINDOWS_UI_WEBUI_CORE_WEBUICOMMANDBARCONTRACT_VERSION)
#define WINDOWS_UI_WEBUI_CORE_WEBUICOMMANDBARCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_UI_WEBUI_CORE_WEBUICOMMANDBARCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "Inspectable.h"
#include "Windows.Foundation.h"
#include "WindowsContracts.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CFoundation_CIActivatableClassRegistration_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIActivatableClassRegistration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IActivatableClassRegistration;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIActivatableClassRegistration ABI::Windows::Foundation::IActivatableClassRegistration

#endif // ____x_ABI_CWindows_CFoundation_CIActivatableClassRegistration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIDllServerActivatableClassRegistration_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDllServerActivatableClassRegistration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IDllServerActivatableClassRegistration;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIDllServerActivatableClassRegistration ABI::Windows::Foundation::IDllServerActivatableClassRegistration

#endif // ____x_ABI_CWindows_CFoundation_CIDllServerActivatableClassRegistration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIExeServerActivatableClassRegistration_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIExeServerActivatableClassRegistration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IExeServerActivatableClassRegistration;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIExeServerActivatableClassRegistration ABI::Windows::Foundation::IExeServerActivatableClassRegistration

#endif // ____x_ABI_CWindows_CFoundation_CIExeServerActivatableClassRegistration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIExeServerRegistration_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIExeServerRegistration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IExeServerRegistration;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIExeServerRegistration ABI::Windows::Foundation::IExeServerRegistration

#endif // ____x_ABI_CWindows_CFoundation_CIExeServerRegistration_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions


#ifndef DEF___FIKeyValuePair_2_HSTRING_IInspectable_USE
#define DEF___FIKeyValuePair_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("09335560-6c6b-5a26-9348-97b781132b20"))
IKeyValuePair<HSTRING,IInspectable*> : IKeyValuePair_impl<HSTRING,IInspectable*> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, Object>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING,IInspectable*> __FIKeyValuePair_2_HSTRING_IInspectable_t;
#define __FIKeyValuePair_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIKeyValuePair_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::IKeyValuePair<HSTRING,IInspectable*>
//#define __FIKeyValuePair_2_HSTRING_IInspectable_t ABI::Windows::Foundation::Collections::IKeyValuePair<HSTRING,IInspectable*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_IInspectable_USE */





#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5db5fa32-707c-5849-a06b-91c8eb9d10e8"))
IIterator<__FIKeyValuePair_2_HSTRING_IInspectable*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_IInspectable*> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_IInspectable*> __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::IIterator<ABI::Windows::Foundation::Collections::IKeyValuePair<HSTRING,IInspectable*>*>
//#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_t ABI::Windows::Foundation::Collections::IIterator<ABI::Windows::Foundation::Collections::IKeyValuePair<HSTRING,IInspectable*>*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_USE */





#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("fe2f3d47-5d47-5499-8374-430c7cda0204"))
IIterable<__FIKeyValuePair_2_HSTRING_IInspectable*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_IInspectable*> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_IInspectable*> __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::IIterable<ABI::Windows::Foundation::Collections::IKeyValuePair<HSTRING,IInspectable*>*>
//#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_t ABI::Windows::Foundation::Collections::IIterable<ABI::Windows::Foundation::Collections::IKeyValuePair<HSTRING,IInspectable*>*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_USE */





#ifndef DEF___FIMapView_2_HSTRING_IInspectable_USE
#define DEF___FIMapView_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bb78502a-f79d-54fa-92c9-90c5039fdf7e"))
IMapView<HSTRING,IInspectable*> : IMapView_impl<HSTRING,IInspectable*> 
{
    static const wchar_t* z_get_rc_name_impl() 
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, Object>"; 
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING,IInspectable*> __FIMapView_2_HSTRING_IInspectable_t;
#define __FIMapView_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ } 

////  Define an alias for the C version of the interface for compatibility purposes.
//#define __FIMapView_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::IMapView<HSTRING,IInspectable*>
//#define __FIMapView_2_HSTRING_IInspectable_t ABI::Windows::Foundation::Collections::IMapView<HSTRING,IInspectable*>
#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_IInspectable_USE */






//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//--------------------------------------------------------------------------
#if ( _MSC_VER >= 1020 )
#pragma once
#endif
#pragma warning (push)
#pragma warning (disable:4668) 
#pragma warning (disable:4001) 
#pragma once 
#pragma warning (pop)

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Foundation {
            /* [contract, version] */
            typedef TrustLevel RegisteredTrustLevel;
            
        } /* Foundation */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


/*
 *
 * Typedef of Windows.Foundation.RegistrationScope
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Foundation {
            /* [contract, version] */
            typedef /* [v1_enum] */
            enum RegistrationScope : int
            {
                RegistrationScope_PerMachine,
                RegistrationScope_PerUser,
                RegistrationScope_InboxApp,
            } RegistrationScope;
            
        } /* Foundation */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


/*
 *
 * Typedef of Windows.Foundation.ActivationType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Foundation {
            /* [contract, version] */
            typedef /* [v1_enum] */
            enum ActivationType : int
            {
                ActivationType_InProcess,
                ActivationType_OutOfProcess,
                ActivationType_RemoteProcess,
            } ActivationType;
            
        } /* Foundation */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


/*
 *
 * Typedef of Windows.Foundation.ThreadingType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Foundation {
            /* [contract, version] */
            typedef /* [v1_enum] */
            enum ThreadingType : int
            {
                ThreadingType_BOTH,
                ThreadingType_STA,
                ThreadingType_MTA,
            } ThreadingType;
            
        } /* Foundation */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


/*
 *
 * Typedef of Windows.Foundation.IdentityType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Foundation {
            /* [contract, version] */
            typedef /* [v1_enum] */
            enum IdentityType : int
            {
                IdentityType_ActivateAsActivator,
                IdentityType_RunAs,
                IdentityType_ActivateAsPackage,
                IdentityType_SessionVirtual,
                IdentityType_SessionUser,
                IdentityType_ActivateAsActivatingUser,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
                
                IdentityType_ActivateAsActivatorPackaged,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
                
            } IdentityType;
            
        } /* Foundation */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


/*
 *
 * Typedef of Windows.Foundation.InstancingType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Foundation {
            /* [contract, version] */
            typedef /* [v1_enum] */
            enum InstancingType : int
            {
                InstancingType_SingleInstance,
                InstancingType_MultipleInstances,
            } InstancingType;
            
        } /* Foundation */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


/*
 *
 * Typedef of Windows.Foundation.ActivateAsUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Foundation {
            /* [contract, version] */
            typedef /* [v1_enum] */
            enum ActivateAsUser : int
            {
                ActivateAsUser_NotSupported,
                ActivateAsUser_Supported,
            } ActivateAsUser;
            
        } /* Foundation */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


/*
 *
 * Interface Windows.Foundation.IActivatableClassRegistration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIActivatableClassRegistration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIActivatableClassRegistration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IActivatableClassRegistration[] = L"Windows.Foundation.IActivatableClassRegistration";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            /* [contract, pointer_default(unique), uuid("9BBCAE23-3DD6-49C3-B63C-1C587E7A6A67"), object, version] */
            MIDL_INTERFACE("9BBCAE23-3DD6-49C3-B63C-1C587E7A6A67")
            IActivatableClassRegistration : public IInspectable
            {
            public:
                /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_ActivatableClassId(
                    /* [retval, out] */__RPC__deref_out_opt HSTRING * activatableClassID
                    ) = 0;
                /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_ActivationType(
                    /* [retval, out] */__RPC__out ABI::Windows::Foundation::ActivationType * activationType
                    ) = 0;
                /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_RegistrationScope(
                    /* [retval, out] */__RPC__out ABI::Windows::Foundation::RegistrationScope * registrationScope
                    ) = 0;
                /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_RegisteredTrustLevel(
                    /* [retval, out] */__RPC__out ABI::Windows::Foundation::RegisteredTrustLevel * registeredTrustLevel
                    ) = 0;
                /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_Attributes(
                    /* [Windows.Foundation.Metadata.HasVariantAttribute, retval, out] */__RPC__deref_out_opt __FIMapView_2_HSTRING_IInspectable * * attributes
                    ) = 0;
                
            };

            MIDL_CONST_ID IID & IID_IActivatableClassRegistration=__uuidof(IActivatableClassRegistration);
            
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIActivatableClassRegistration;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIActivatableClassRegistration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


/*
 *
 * Interface Windows.Foundation.IDllServerActivatableClassRegistration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIDllServerActivatableClassRegistration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIDllServerActivatableClassRegistration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IDllServerActivatableClassRegistration[] = L"Windows.Foundation.IDllServerActivatableClassRegistration";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            /* [contract, pointer_default(ref), uuid("C8AA04F6-66C6-46A3-8FE6-F56BE7DDC091"), object, version] */
            MIDL_INTERFACE("C8AA04F6-66C6-46A3-8FE6-F56BE7DDC091")
            IDllServerActivatableClassRegistration : public IInspectable
            {
            public:
                /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_DllPath(
                    /* [retval, out] */__RPC__deref_out_opt HSTRING * dllPath
                    ) = 0;
                /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_ThreadingType(
                    /* [retval, out] */__RPC__out ABI::Windows::Foundation::ThreadingType * threadingType
                    ) = 0;
                
            };

            MIDL_CONST_ID IID & IID_IDllServerActivatableClassRegistration=__uuidof(IDllServerActivatableClassRegistration);
            
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIDllServerActivatableClassRegistration;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIDllServerActivatableClassRegistration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


/*
 *
 * Interface Windows.Foundation.IExeServerActivatableClassRegistration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIExeServerActivatableClassRegistration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIExeServerActivatableClassRegistration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IExeServerActivatableClassRegistration[] = L"Windows.Foundation.IExeServerActivatableClassRegistration";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            /* [contract, pointer_default(ref), uuid("9308C3C5-C2AC-49D1-A024-660A2BB5D5AC"), object, version] */
            MIDL_INTERFACE("9308C3C5-C2AC-49D1-A024-660A2BB5D5AC")
            IExeServerActivatableClassRegistration : public IInspectable
            {
            public:
                /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_ServerRegistration(
                    /* [retval, out] */__RPC__deref_out_opt ABI::Windows::Foundation::IExeServerRegistration * * serverRegistration
                    ) = 0;
                
            };

            MIDL_CONST_ID IID & IID_IExeServerActivatableClassRegistration=__uuidof(IExeServerActivatableClassRegistration);
            
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIExeServerActivatableClassRegistration;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIExeServerActivatableClassRegistration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


/*
 *
 * Interface Windows.Foundation.IExeServerRegistration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIExeServerRegistration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIExeServerRegistration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IExeServerRegistration[] = L"Windows.Foundation.IExeServerRegistration";
namespace ABI {
    namespace Windows {
        namespace Foundation {
            /* [contract, pointer_default(unique), uuid("EC734A06-0401-4317-BAC1-3B7E207242E3"), object, version] */
            MIDL_INTERFACE("EC734A06-0401-4317-BAC1-3B7E207242E3")
            IExeServerRegistration : public IInspectable
            {
            public:
                /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_ServerName(
                    /* [retval, out] */__RPC__deref_out_opt HSTRING * serverName
                    ) = 0;
                /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_ExePath(
                    /* [retval, out] */__RPC__deref_out_opt HSTRING * exePath
                    ) = 0;
                /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_CommandLine(
                    /* [retval, out] */__RPC__deref_out_opt HSTRING * commandLine
                    ) = 0;
                /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_AppUserModelId(
                    /* [retval, out] */__RPC__deref_out_opt HSTRING * appUserModelId
                    ) = 0;
                /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_IdentityType(
                    /* [retval, out] */__RPC__out ABI::Windows::Foundation::IdentityType * identityType
                    ) = 0;
                /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_Identity(
                    /* [retval, out] */__RPC__deref_out_opt HSTRING * identity
                    ) = 0;
                /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_Instancing(
                    /* [retval, out] */__RPC__out ABI::Windows::Foundation::InstancingType * instanceType
                    ) = 0;
                /* [propget] */virtual HRESULT STDMETHODCALLTYPE get_Permissions(
                    /* [retval, out] */__RPC__deref_out_opt HSTRING * permissions
                    ) = 0;
                
            };

            MIDL_CONST_ID IID & IID_IExeServerRegistration=__uuidof(IExeServerRegistration);
            
        } /* Foundation */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIExeServerRegistration;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIExeServerRegistration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            class DllServerActivatableClassRegistration;
        } /* Foundation */
    } /* Windows */
} /* ABI */



/*
 *
 * Class Windows.Foundation.DllServerActivatableClassRegistration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IActivatableClassRegistration ** Default Interface **
 *    Windows.Foundation.IDllServerActivatableClassRegistration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef RUNTIMECLASS_Windows_Foundation_DllServerActivatableClassRegistration_DEFINED
#define RUNTIMECLASS_Windows_Foundation_DllServerActivatableClassRegistration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_DllServerActivatableClassRegistration[] = L"Windows.Foundation.DllServerActivatableClassRegistration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            class ExeServerActivatableClassRegistration;
        } /* Foundation */
    } /* Windows */
} /* ABI */



/*
 *
 * Class Windows.Foundation.ExeServerActivatableClassRegistration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IActivatableClassRegistration ** Default Interface **
 *    Windows.Foundation.IExeServerActivatableClassRegistration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef RUNTIMECLASS_Windows_Foundation_ExeServerActivatableClassRegistration_DEFINED
#define RUNTIMECLASS_Windows_Foundation_ExeServerActivatableClassRegistration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_ExeServerActivatableClassRegistration[] = L"Windows.Foundation.ExeServerActivatableClassRegistration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            class ExeServerRegistration;
        } /* Foundation */
    } /* Windows */
} /* ABI */



/*
 *
 * Class Windows.Foundation.ExeServerRegistration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IExeServerRegistration ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef RUNTIMECLASS_Windows_Foundation_ExeServerRegistration_DEFINED
#define RUNTIMECLASS_Windows_Foundation_ExeServerRegistration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_ExeServerRegistration[] = L"Windows.Foundation.ExeServerRegistration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CFoundation_CIActivatableClassRegistration_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIActivatableClassRegistration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIActivatableClassRegistration __x_ABI_CWindows_CFoundation_CIActivatableClassRegistration;

#endif // ____x_ABI_CWindows_CFoundation_CIActivatableClassRegistration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIDllServerActivatableClassRegistration_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDllServerActivatableClassRegistration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIDllServerActivatableClassRegistration __x_ABI_CWindows_CFoundation_CIDllServerActivatableClassRegistration;

#endif // ____x_ABI_CWindows_CFoundation_CIDllServerActivatableClassRegistration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIExeServerActivatableClassRegistration_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIExeServerActivatableClassRegistration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIExeServerActivatableClassRegistration __x_ABI_CWindows_CFoundation_CIExeServerActivatableClassRegistration;

#endif // ____x_ABI_CWindows_CFoundation_CIExeServerActivatableClassRegistration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIExeServerRegistration_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIExeServerRegistration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIExeServerRegistration __x_ABI_CWindows_CFoundation_CIExeServerRegistration;

#endif // ____x_ABI_CWindows_CFoundation_CIExeServerRegistration_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if !defined(____FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_IInspectable __FIKeyValuePair_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_IInspectable;

typedef struct __FIKeyValuePair_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface )(__RPC__in __FIKeyValuePair_2_HSTRING_IInspectable * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);

    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIKeyValuePair_2_HSTRING_IInspectable * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIKeyValuePair_2_HSTRING_IInspectable * This);
    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIKeyValuePair_2_HSTRING_IInspectable * This,
            /* [out] */ __RPC__out ULONG *iidCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);
    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIKeyValuePair_2_HSTRING_IInspectable * This, /* [out] */ __RPC__deref_out_opt HSTRING *className);
    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIKeyValuePair_2_HSTRING_IInspectable * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Key )(__RPC__in __FIKeyValuePair_2_HSTRING_IInspectable * This, /* [retval][out] */ __RPC__out HSTRING *key);
    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Value )(__RPC__in __FIKeyValuePair_2_HSTRING_IInspectable * This, /* [retval][out] */ __RPC__deref_out_opt IInspectable * *value);
    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_IInspectableVtbl;

interface __FIKeyValuePair_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_IInspectableVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __FIKeyValuePair_2_HSTRING_IInspectable_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIKeyValuePair_2_HSTRING_IInspectable_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIKeyValuePair_2_HSTRING_IInspectable_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIKeyValuePair_2_HSTRING_IInspectable_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIKeyValuePair_2_HSTRING_IInspectable_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIKeyValuePair_2_HSTRING_IInspectable_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIKeyValuePair_2_HSTRING_IInspectable_get_Key(This,key)	\
    ( (This)->lpVtbl -> get_Key(This,key) ) 

#define __FIKeyValuePair_2_HSTRING_IInspectable_get_Value(This,value)	\
    ( (This)->lpVtbl -> get_Value(This,value) ) 
#endif /* COBJMACROS */


#endif // ____FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__



#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
        __RPC__in __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);
    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable * This);
    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable * This);
    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable * This,
        /* [out] */ __RPC__out ULONG *iidCount,
        /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable * This, /* [out] */ __RPC__deref_out_opt HSTRING *className);
    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Current )(__RPC__in __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable * This, /* [retval][out] */ __RPC__out __FIKeyValuePair_2_HSTRING_IInspectable * *current);
    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_HasCurrent )(__RPC__in __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable * This, /* [retval][out] */ __RPC__out boolean *hasCurrent);
    HRESULT ( STDMETHODCALLTYPE *MoveNext )(__RPC__in __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable * This, /* [retval][out] */ __RPC__out boolean *hasCurrent);
    HRESULT ( STDMETHODCALLTYPE *GetMany )(__RPC__in __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable * This,
        /* [in] */ unsigned int capacity,
        /* [size_is][length_is][out] */ __RPC__out_ecount_part(capacity, *actual) __FIKeyValuePair_2_HSTRING_IInspectable * *items,
        /* [retval][out] */ __RPC__out unsigned int *actual);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl *lpVtbl;
};



#ifdef COBJMACROS


#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_get_Current(This,current)	\
    ( (This)->lpVtbl -> get_Current(This,current) ) 

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_get_HasCurrent(This,hasCurrent)	\
    ( (This)->lpVtbl -> get_HasCurrent(This,hasCurrent) ) 

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_MoveNext(This,hasCurrent)	\
    ( (This)->lpVtbl -> MoveNext(This,hasCurrent) ) 

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetMany(This,capacity,items,actual)	\
    ( (This)->lpVtbl -> GetMany(This,capacity,items,actual) ) 

#endif /* COBJMACROS */


#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__



#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable;

typedef  struct __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
        __RPC__in __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);

    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable * This);

    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable * This);

    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable * This,
                                           /* [out] */ __RPC__out ULONG *iidCount,
                                           /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable * This, /* [out] */ __RPC__deref_out_opt HSTRING *className);

    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable * This, /* [out] */ __RPC__out TrustLevel *trustLevel);

    HRESULT ( STDMETHODCALLTYPE *First )(__RPC__in __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable * This, /* [retval][out] */ __RPC__deref_out_opt __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable **first);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl *lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_First(This,first)	\
    ( (This)->lpVtbl -> First(This,first) ) 

#endif /* COBJMACROS */


#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__



#if !defined(____FIMapView_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_IInspectable __FIMapView_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_IInspectable;

typedef struct __FIMapView_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface )(__RPC__in __FIMapView_2_HSTRING_IInspectable * This,
        /* [in] */ __RPC__in REFIID riid,
        /* [annotation][iid_is][out] */ 
        _COM_Outptr_  void **ppvObject);

    ULONG ( STDMETHODCALLTYPE *AddRef )(__RPC__in __FIMapView_2_HSTRING_IInspectable * This);

    ULONG ( STDMETHODCALLTYPE *Release )(__RPC__in __FIMapView_2_HSTRING_IInspectable * This);

    HRESULT ( STDMETHODCALLTYPE *GetIids )(__RPC__in __FIMapView_2_HSTRING_IInspectable * This,
        /* [out] */ __RPC__out ULONG *iidCount,
        /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);

    HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(__RPC__in __FIMapView_2_HSTRING_IInspectable * This, /* [out] */ __RPC__deref_out_opt HSTRING *className);
    HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(__RPC__in __FIMapView_2_HSTRING_IInspectable * This,/* [out] */ __RPC__out TrustLevel *trustLevel);

    HRESULT ( STDMETHODCALLTYPE *Lookup )(__RPC__in __FIMapView_2_HSTRING_IInspectable * This,
        /* [in] */ __RPC__in HSTRING key,
        /* [retval][out] */ __RPC__deref_out_opt IInspectable * *value);
    /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Size )(__RPC__in __FIMapView_2_HSTRING_IInspectable * This, /* [retval][out] */ __RPC__out unsigned int *size);
    HRESULT ( STDMETHODCALLTYPE *HasKey )(__RPC__in __FIMapView_2_HSTRING_IInspectable * This, /* [in] */ __RPC__in HSTRING key, /* [retval][out] */ __RPC__out boolean *found);
    HRESULT ( STDMETHODCALLTYPE *Split )(__RPC__in __FIMapView_2_HSTRING_IInspectable * This,/* [out] */ __RPC__deref_out_opt __FIMapView_2_HSTRING_IInspectable **firstPartition,
        /* [out] */ __RPC__deref_out_opt __FIMapView_2_HSTRING_IInspectable **secondPartition);
    END_INTERFACE
} __FIMapView_2_HSTRING_IInspectableVtbl;

interface __FIMapView_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIMapView_2_HSTRING_IInspectableVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __FIMapView_2_HSTRING_IInspectable_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 
#define __FIMapView_2_HSTRING_IInspectable_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 
#define __FIMapView_2_HSTRING_IInspectable_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 

#define __FIMapView_2_HSTRING_IInspectable_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 
#define __FIMapView_2_HSTRING_IInspectable_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 
#define __FIMapView_2_HSTRING_IInspectable_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 

#define __FIMapView_2_HSTRING_IInspectable_Lookup(This,key,value)	\
    ( (This)->lpVtbl -> Lookup(This,key,value) ) 
#define __FIMapView_2_HSTRING_IInspectable_get_Size(This,size)	\
    ( (This)->lpVtbl -> get_Size(This,size) ) 
#define __FIMapView_2_HSTRING_IInspectable_HasKey(This,key,found)	\
    ( (This)->lpVtbl -> HasKey(This,key,found) ) 
#define __FIMapView_2_HSTRING_IInspectable_Split(This,firstPartition,secondPartition)	\
    ( (This)->lpVtbl -> Split(This,firstPartition,secondPartition) ) 
#endif /* COBJMACROS */


#endif // ____FIMapView_2_HSTRING_IInspectable_INTERFACE_DEFINED__




//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//--------------------------------------------------------------------------
#if ( _MSC_VER >= 1020 )
#pragma once
#endif
#pragma warning (push)
#pragma warning (disable:4668) 
#pragma warning (disable:4001) 
#pragma once 
#pragma warning (pop)

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
/* [contract, version] */
typedef TrustLevel __x_ABI_CWindows_CFoundation_CRegisteredTrustLevel;
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


/*
 *
 * Typedef of Windows.Foundation.RegistrationScope
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
/* [contract, version] */
typedef /* [v1_enum] */
enum __x_ABI_CWindows_CFoundation_CRegistrationScope
{
    RegistrationScope_PerMachine,
    RegistrationScope_PerUser,
    RegistrationScope_InboxApp,
} __x_ABI_CWindows_CFoundation_CRegistrationScope;
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


/*
 *
 * Typedef of Windows.Foundation.ActivationType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
/* [contract, version] */
typedef /* [v1_enum] */
enum __x_ABI_CWindows_CFoundation_CActivationType
{
    ActivationType_InProcess,
    ActivationType_OutOfProcess,
    ActivationType_RemoteProcess,
} __x_ABI_CWindows_CFoundation_CActivationType;
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


/*
 *
 * Typedef of Windows.Foundation.ThreadingType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
/* [contract, version] */
typedef /* [v1_enum] */
enum __x_ABI_CWindows_CFoundation_CThreadingType
{
    ThreadingType_BOTH,
    ThreadingType_STA,
    ThreadingType_MTA,
} __x_ABI_CWindows_CFoundation_CThreadingType;
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


/*
 *
 * Typedef of Windows.Foundation.IdentityType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
/* [contract, version] */
typedef /* [v1_enum] */
enum __x_ABI_CWindows_CFoundation_CIdentityType
{
    IdentityType_ActivateAsActivator,
    IdentityType_RunAs,
    IdentityType_ActivateAsPackage,
    IdentityType_SessionVirtual,
    IdentityType_SessionUser,
    IdentityType_ActivateAsActivatingUser,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
    
    IdentityType_ActivateAsActivatorPackaged,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
    
} __x_ABI_CWindows_CFoundation_CIdentityType;
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


/*
 *
 * Typedef of Windows.Foundation.InstancingType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
/* [contract, version] */
typedef /* [v1_enum] */
enum __x_ABI_CWindows_CFoundation_CInstancingType
{
    InstancingType_SingleInstance,
    InstancingType_MultipleInstances,
} __x_ABI_CWindows_CFoundation_CInstancingType;
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


/*
 *
 * Typedef of Windows.Foundation.ActivateAsUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
/* [contract, version] */
typedef /* [v1_enum] */
enum __x_ABI_CWindows_CFoundation_CActivateAsUser
{
    ActivateAsUser_NotSupported,
    ActivateAsUser_Supported,
} __x_ABI_CWindows_CFoundation_CActivateAsUser;
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


/*
 *
 * Interface Windows.Foundation.IActivatableClassRegistration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIActivatableClassRegistration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIActivatableClassRegistration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IActivatableClassRegistration[] = L"Windows.Foundation.IActivatableClassRegistration";
/* [contract, pointer_default(unique), uuid("9BBCAE23-3DD6-49C3-B63C-1C587E7A6A67"), object, version] */
typedef struct __x_ABI_CWindows_CFoundation_CIActivatableClassRegistrationVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface)(
    __RPC__in __x_ABI_CWindows_CFoundation_CIActivatableClassRegistration * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [annotation][iid_is][out] */
    _COM_Outptr_  void **ppvObject
    );

ULONG ( STDMETHODCALLTYPE *AddRef )(
    __RPC__in __x_ABI_CWindows_CFoundation_CIActivatableClassRegistration * This
    );

ULONG ( STDMETHODCALLTYPE *Release )(
    __RPC__in __x_ABI_CWindows_CFoundation_CIActivatableClassRegistration * This
    );

HRESULT ( STDMETHODCALLTYPE *GetIids )(
    __RPC__in __x_ABI_CWindows_CFoundation_CIActivatableClassRegistration * This,
    /* [out] */ __RPC__out ULONG *iidCount,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids
    );

HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(
    __RPC__in __x_ABI_CWindows_CFoundation_CIActivatableClassRegistration * This,
    /* [out] */ __RPC__deref_out_opt HSTRING *className
    );

HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(
    __RPC__in __x_ABI_CWindows_CFoundation_CIActivatableClassRegistration * This,
    /* [OUT ] */ __RPC__out TrustLevel *trustLevel
    );
/* [propget] */HRESULT ( STDMETHODCALLTYPE *get_ActivatableClassId )(
        __x_ABI_CWindows_CFoundation_CIActivatableClassRegistration * This,
        /* [retval, out] */__RPC__deref_out_opt HSTRING * activatableClassID
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_ActivationType )(
        __x_ABI_CWindows_CFoundation_CIActivatableClassRegistration * This,
        /* [retval, out] */__RPC__out __x_ABI_CWindows_CFoundation_CActivationType * activationType
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_RegistrationScope )(
        __x_ABI_CWindows_CFoundation_CIActivatableClassRegistration * This,
        /* [retval, out] */__RPC__out __x_ABI_CWindows_CFoundation_CRegistrationScope * registrationScope
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_RegisteredTrustLevel )(
        __x_ABI_CWindows_CFoundation_CIActivatableClassRegistration * This,
        /* [retval, out] */__RPC__out __x_ABI_CWindows_CFoundation_CRegisteredTrustLevel * registeredTrustLevel
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_Attributes )(
        __x_ABI_CWindows_CFoundation_CIActivatableClassRegistration * This,
        /* [Windows.Foundation.Metadata.HasVariantAttribute, retval, out] */__RPC__deref_out_opt __FIMapView_2_HSTRING_IInspectable * * attributes
        );
    END_INTERFACE
    
} __x_ABI_CWindows_CFoundation_CIActivatableClassRegistrationVtbl;

interface __x_ABI_CWindows_CFoundation_CIActivatableClassRegistration
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CIActivatableClassRegistrationVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __x_ABI_CWindows_CFoundation_CIActivatableClassRegistration_QueryInterface(This,riid,ppvObject) \
( (This)->lpVtbl->QueryInterface(This,riid,ppvObject) )

#define __x_ABI_CWindows_CFoundation_CIActivatableClassRegistration_AddRef(This) \
        ( (This)->lpVtbl->AddRef(This) )

#define __x_ABI_CWindows_CFoundation_CIActivatableClassRegistration_Release(This) \
        ( (This)->lpVtbl->Release(This) )

#define __x_ABI_CWindows_CFoundation_CIActivatableClassRegistration_GetIids(This,iidCount,iids) \
        ( (This)->lpVtbl->GetIids(This,iidCount,iids) )

#define __x_ABI_CWindows_CFoundation_CIActivatableClassRegistration_GetRuntimeClassName(This,className) \
        ( (This)->lpVtbl->GetRuntimeClassName(This,className) )

#define __x_ABI_CWindows_CFoundation_CIActivatableClassRegistration_GetTrustLevel(This,trustLevel) \
        ( (This)->lpVtbl->GetTrustLevel(This,trustLevel) )

#define __x_ABI_CWindows_CFoundation_CIActivatableClassRegistration_get_ActivatableClassId(This,activatableClassID) \
    ( (This)->lpVtbl->get_ActivatableClassId(This,activatableClassID) )

#define __x_ABI_CWindows_CFoundation_CIActivatableClassRegistration_get_ActivationType(This,activationType) \
    ( (This)->lpVtbl->get_ActivationType(This,activationType) )

#define __x_ABI_CWindows_CFoundation_CIActivatableClassRegistration_get_RegistrationScope(This,registrationScope) \
    ( (This)->lpVtbl->get_RegistrationScope(This,registrationScope) )

#define __x_ABI_CWindows_CFoundation_CIActivatableClassRegistration_get_RegisteredTrustLevel(This,registeredTrustLevel) \
    ( (This)->lpVtbl->get_RegisteredTrustLevel(This,registeredTrustLevel) )

#define __x_ABI_CWindows_CFoundation_CIActivatableClassRegistration_get_Attributes(This,attributes) \
    ( (This)->lpVtbl->get_Attributes(This,attributes) )


#endif /* COBJMACROS */


EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIActivatableClassRegistration;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIActivatableClassRegistration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


/*
 *
 * Interface Windows.Foundation.IDllServerActivatableClassRegistration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIDllServerActivatableClassRegistration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIDllServerActivatableClassRegistration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IDllServerActivatableClassRegistration[] = L"Windows.Foundation.IDllServerActivatableClassRegistration";
/* [contract, pointer_default(ref), uuid("C8AA04F6-66C6-46A3-8FE6-F56BE7DDC091"), object, version] */
typedef struct __x_ABI_CWindows_CFoundation_CIDllServerActivatableClassRegistrationVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface)(
    __RPC__in __x_ABI_CWindows_CFoundation_CIDllServerActivatableClassRegistration * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [annotation][iid_is][out] */
    _COM_Outptr_  void **ppvObject
    );

ULONG ( STDMETHODCALLTYPE *AddRef )(
    __RPC__in __x_ABI_CWindows_CFoundation_CIDllServerActivatableClassRegistration * This
    );

ULONG ( STDMETHODCALLTYPE *Release )(
    __RPC__in __x_ABI_CWindows_CFoundation_CIDllServerActivatableClassRegistration * This
    );

HRESULT ( STDMETHODCALLTYPE *GetIids )(
    __RPC__in __x_ABI_CWindows_CFoundation_CIDllServerActivatableClassRegistration * This,
    /* [out] */ __RPC__out ULONG *iidCount,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids
    );

HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(
    __RPC__in __x_ABI_CWindows_CFoundation_CIDllServerActivatableClassRegistration * This,
    /* [out] */ __RPC__deref_out_opt HSTRING *className
    );

HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(
    __RPC__in __x_ABI_CWindows_CFoundation_CIDllServerActivatableClassRegistration * This,
    /* [OUT ] */ __RPC__out TrustLevel *trustLevel
    );
/* [propget] */HRESULT ( STDMETHODCALLTYPE *get_DllPath )(
        __x_ABI_CWindows_CFoundation_CIDllServerActivatableClassRegistration * This,
        /* [retval, out] */__RPC__deref_out_opt HSTRING * dllPath
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_ThreadingType )(
        __x_ABI_CWindows_CFoundation_CIDllServerActivatableClassRegistration * This,
        /* [retval, out] */__RPC__out __x_ABI_CWindows_CFoundation_CThreadingType * threadingType
        );
    END_INTERFACE
    
} __x_ABI_CWindows_CFoundation_CIDllServerActivatableClassRegistrationVtbl;

interface __x_ABI_CWindows_CFoundation_CIDllServerActivatableClassRegistration
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CIDllServerActivatableClassRegistrationVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __x_ABI_CWindows_CFoundation_CIDllServerActivatableClassRegistration_QueryInterface(This,riid,ppvObject) \
( (This)->lpVtbl->QueryInterface(This,riid,ppvObject) )

#define __x_ABI_CWindows_CFoundation_CIDllServerActivatableClassRegistration_AddRef(This) \
        ( (This)->lpVtbl->AddRef(This) )

#define __x_ABI_CWindows_CFoundation_CIDllServerActivatableClassRegistration_Release(This) \
        ( (This)->lpVtbl->Release(This) )

#define __x_ABI_CWindows_CFoundation_CIDllServerActivatableClassRegistration_GetIids(This,iidCount,iids) \
        ( (This)->lpVtbl->GetIids(This,iidCount,iids) )

#define __x_ABI_CWindows_CFoundation_CIDllServerActivatableClassRegistration_GetRuntimeClassName(This,className) \
        ( (This)->lpVtbl->GetRuntimeClassName(This,className) )

#define __x_ABI_CWindows_CFoundation_CIDllServerActivatableClassRegistration_GetTrustLevel(This,trustLevel) \
        ( (This)->lpVtbl->GetTrustLevel(This,trustLevel) )

#define __x_ABI_CWindows_CFoundation_CIDllServerActivatableClassRegistration_get_DllPath(This,dllPath) \
    ( (This)->lpVtbl->get_DllPath(This,dllPath) )

#define __x_ABI_CWindows_CFoundation_CIDllServerActivatableClassRegistration_get_ThreadingType(This,threadingType) \
    ( (This)->lpVtbl->get_ThreadingType(This,threadingType) )


#endif /* COBJMACROS */


EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIDllServerActivatableClassRegistration;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIDllServerActivatableClassRegistration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


/*
 *
 * Interface Windows.Foundation.IExeServerActivatableClassRegistration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIExeServerActivatableClassRegistration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIExeServerActivatableClassRegistration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IExeServerActivatableClassRegistration[] = L"Windows.Foundation.IExeServerActivatableClassRegistration";
/* [contract, pointer_default(ref), uuid("9308C3C5-C2AC-49D1-A024-660A2BB5D5AC"), object, version] */
typedef struct __x_ABI_CWindows_CFoundation_CIExeServerActivatableClassRegistrationVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface)(
    __RPC__in __x_ABI_CWindows_CFoundation_CIExeServerActivatableClassRegistration * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [annotation][iid_is][out] */
    _COM_Outptr_  void **ppvObject
    );

ULONG ( STDMETHODCALLTYPE *AddRef )(
    __RPC__in __x_ABI_CWindows_CFoundation_CIExeServerActivatableClassRegistration * This
    );

ULONG ( STDMETHODCALLTYPE *Release )(
    __RPC__in __x_ABI_CWindows_CFoundation_CIExeServerActivatableClassRegistration * This
    );

HRESULT ( STDMETHODCALLTYPE *GetIids )(
    __RPC__in __x_ABI_CWindows_CFoundation_CIExeServerActivatableClassRegistration * This,
    /* [out] */ __RPC__out ULONG *iidCount,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids
    );

HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(
    __RPC__in __x_ABI_CWindows_CFoundation_CIExeServerActivatableClassRegistration * This,
    /* [out] */ __RPC__deref_out_opt HSTRING *className
    );

HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(
    __RPC__in __x_ABI_CWindows_CFoundation_CIExeServerActivatableClassRegistration * This,
    /* [OUT ] */ __RPC__out TrustLevel *trustLevel
    );
/* [propget] */HRESULT ( STDMETHODCALLTYPE *get_ServerRegistration )(
        __x_ABI_CWindows_CFoundation_CIExeServerActivatableClassRegistration * This,
        /* [retval, out] */__RPC__deref_out_opt __x_ABI_CWindows_CFoundation_CIExeServerRegistration * * serverRegistration
        );
    END_INTERFACE
    
} __x_ABI_CWindows_CFoundation_CIExeServerActivatableClassRegistrationVtbl;

interface __x_ABI_CWindows_CFoundation_CIExeServerActivatableClassRegistration
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CIExeServerActivatableClassRegistrationVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __x_ABI_CWindows_CFoundation_CIExeServerActivatableClassRegistration_QueryInterface(This,riid,ppvObject) \
( (This)->lpVtbl->QueryInterface(This,riid,ppvObject) )

#define __x_ABI_CWindows_CFoundation_CIExeServerActivatableClassRegistration_AddRef(This) \
        ( (This)->lpVtbl->AddRef(This) )

#define __x_ABI_CWindows_CFoundation_CIExeServerActivatableClassRegistration_Release(This) \
        ( (This)->lpVtbl->Release(This) )

#define __x_ABI_CWindows_CFoundation_CIExeServerActivatableClassRegistration_GetIids(This,iidCount,iids) \
        ( (This)->lpVtbl->GetIids(This,iidCount,iids) )

#define __x_ABI_CWindows_CFoundation_CIExeServerActivatableClassRegistration_GetRuntimeClassName(This,className) \
        ( (This)->lpVtbl->GetRuntimeClassName(This,className) )

#define __x_ABI_CWindows_CFoundation_CIExeServerActivatableClassRegistration_GetTrustLevel(This,trustLevel) \
        ( (This)->lpVtbl->GetTrustLevel(This,trustLevel) )

#define __x_ABI_CWindows_CFoundation_CIExeServerActivatableClassRegistration_get_ServerRegistration(This,serverRegistration) \
    ( (This)->lpVtbl->get_ServerRegistration(This,serverRegistration) )


#endif /* COBJMACROS */


EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIExeServerActivatableClassRegistration;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIExeServerActivatableClassRegistration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


/*
 *
 * Interface Windows.Foundation.IExeServerRegistration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CFoundation_CIExeServerRegistration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CFoundation_CIExeServerRegistration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Foundation_IExeServerRegistration[] = L"Windows.Foundation.IExeServerRegistration";
/* [contract, pointer_default(unique), uuid("EC734A06-0401-4317-BAC1-3B7E207242E3"), object, version] */
typedef struct __x_ABI_CWindows_CFoundation_CIExeServerRegistrationVtbl
{
    BEGIN_INTERFACE
    HRESULT ( STDMETHODCALLTYPE *QueryInterface)(
    __RPC__in __x_ABI_CWindows_CFoundation_CIExeServerRegistration * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [annotation][iid_is][out] */
    _COM_Outptr_  void **ppvObject
    );

ULONG ( STDMETHODCALLTYPE *AddRef )(
    __RPC__in __x_ABI_CWindows_CFoundation_CIExeServerRegistration * This
    );

ULONG ( STDMETHODCALLTYPE *Release )(
    __RPC__in __x_ABI_CWindows_CFoundation_CIExeServerRegistration * This
    );

HRESULT ( STDMETHODCALLTYPE *GetIids )(
    __RPC__in __x_ABI_CWindows_CFoundation_CIExeServerRegistration * This,
    /* [out] */ __RPC__out ULONG *iidCount,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids
    );

HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )(
    __RPC__in __x_ABI_CWindows_CFoundation_CIExeServerRegistration * This,
    /* [out] */ __RPC__deref_out_opt HSTRING *className
    );

HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )(
    __RPC__in __x_ABI_CWindows_CFoundation_CIExeServerRegistration * This,
    /* [OUT ] */ __RPC__out TrustLevel *trustLevel
    );
/* [propget] */HRESULT ( STDMETHODCALLTYPE *get_ServerName )(
        __x_ABI_CWindows_CFoundation_CIExeServerRegistration * This,
        /* [retval, out] */__RPC__deref_out_opt HSTRING * serverName
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_ExePath )(
        __x_ABI_CWindows_CFoundation_CIExeServerRegistration * This,
        /* [retval, out] */__RPC__deref_out_opt HSTRING * exePath
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_CommandLine )(
        __x_ABI_CWindows_CFoundation_CIExeServerRegistration * This,
        /* [retval, out] */__RPC__deref_out_opt HSTRING * commandLine
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_AppUserModelId )(
        __x_ABI_CWindows_CFoundation_CIExeServerRegistration * This,
        /* [retval, out] */__RPC__deref_out_opt HSTRING * appUserModelId
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_IdentityType )(
        __x_ABI_CWindows_CFoundation_CIExeServerRegistration * This,
        /* [retval, out] */__RPC__out __x_ABI_CWindows_CFoundation_CIdentityType * identityType
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_Identity )(
        __x_ABI_CWindows_CFoundation_CIExeServerRegistration * This,
        /* [retval, out] */__RPC__deref_out_opt HSTRING * identity
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_Instancing )(
        __x_ABI_CWindows_CFoundation_CIExeServerRegistration * This,
        /* [retval, out] */__RPC__out __x_ABI_CWindows_CFoundation_CInstancingType * instanceType
        );
    /* [propget] */HRESULT ( STDMETHODCALLTYPE *get_Permissions )(
        __x_ABI_CWindows_CFoundation_CIExeServerRegistration * This,
        /* [retval, out] */__RPC__deref_out_opt HSTRING * permissions
        );
    END_INTERFACE
    
} __x_ABI_CWindows_CFoundation_CIExeServerRegistrationVtbl;

interface __x_ABI_CWindows_CFoundation_CIExeServerRegistration
{
    CONST_VTBL struct __x_ABI_CWindows_CFoundation_CIExeServerRegistrationVtbl *lpVtbl;
};

#ifdef COBJMACROS
#define __x_ABI_CWindows_CFoundation_CIExeServerRegistration_QueryInterface(This,riid,ppvObject) \
( (This)->lpVtbl->QueryInterface(This,riid,ppvObject) )

#define __x_ABI_CWindows_CFoundation_CIExeServerRegistration_AddRef(This) \
        ( (This)->lpVtbl->AddRef(This) )

#define __x_ABI_CWindows_CFoundation_CIExeServerRegistration_Release(This) \
        ( (This)->lpVtbl->Release(This) )

#define __x_ABI_CWindows_CFoundation_CIExeServerRegistration_GetIids(This,iidCount,iids) \
        ( (This)->lpVtbl->GetIids(This,iidCount,iids) )

#define __x_ABI_CWindows_CFoundation_CIExeServerRegistration_GetRuntimeClassName(This,className) \
        ( (This)->lpVtbl->GetRuntimeClassName(This,className) )

#define __x_ABI_CWindows_CFoundation_CIExeServerRegistration_GetTrustLevel(This,trustLevel) \
        ( (This)->lpVtbl->GetTrustLevel(This,trustLevel) )

#define __x_ABI_CWindows_CFoundation_CIExeServerRegistration_get_ServerName(This,serverName) \
    ( (This)->lpVtbl->get_ServerName(This,serverName) )

#define __x_ABI_CWindows_CFoundation_CIExeServerRegistration_get_ExePath(This,exePath) \
    ( (This)->lpVtbl->get_ExePath(This,exePath) )

#define __x_ABI_CWindows_CFoundation_CIExeServerRegistration_get_CommandLine(This,commandLine) \
    ( (This)->lpVtbl->get_CommandLine(This,commandLine) )

#define __x_ABI_CWindows_CFoundation_CIExeServerRegistration_get_AppUserModelId(This,appUserModelId) \
    ( (This)->lpVtbl->get_AppUserModelId(This,appUserModelId) )

#define __x_ABI_CWindows_CFoundation_CIExeServerRegistration_get_IdentityType(This,identityType) \
    ( (This)->lpVtbl->get_IdentityType(This,identityType) )

#define __x_ABI_CWindows_CFoundation_CIExeServerRegistration_get_Identity(This,identity) \
    ( (This)->lpVtbl->get_Identity(This,identity) )

#define __x_ABI_CWindows_CFoundation_CIExeServerRegistration_get_Instancing(This,instanceType) \
    ( (This)->lpVtbl->get_Instancing(This,instanceType) )

#define __x_ABI_CWindows_CFoundation_CIExeServerRegistration_get_Permissions(This,permissions) \
    ( (This)->lpVtbl->get_Permissions(This,permissions) )


#endif /* COBJMACROS */


EXTERN_C const IID IID___x_ABI_CWindows_CFoundation_CIExeServerRegistration;
#endif /* !defined(____x_ABI_CWindows_CFoundation_CIExeServerRegistration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000



/*
 *
 * Class Windows.Foundation.DllServerActivatableClassRegistration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IActivatableClassRegistration ** Default Interface **
 *    Windows.Foundation.IDllServerActivatableClassRegistration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef RUNTIMECLASS_Windows_Foundation_DllServerActivatableClassRegistration_DEFINED
#define RUNTIMECLASS_Windows_Foundation_DllServerActivatableClassRegistration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_DllServerActivatableClassRegistration[] = L"Windows.Foundation.DllServerActivatableClassRegistration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000



/*
 *
 * Class Windows.Foundation.ExeServerActivatableClassRegistration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IActivatableClassRegistration ** Default Interface **
 *    Windows.Foundation.IExeServerActivatableClassRegistration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef RUNTIMECLASS_Windows_Foundation_ExeServerActivatableClassRegistration_DEFINED
#define RUNTIMECLASS_Windows_Foundation_ExeServerActivatableClassRegistration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_ExeServerActivatableClassRegistration[] = L"Windows.Foundation.ExeServerActivatableClassRegistration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000



/*
 *
 * Class Windows.Foundation.ExeServerRegistration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IExeServerRegistration ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef RUNTIMECLASS_Windows_Foundation_ExeServerRegistration_DEFINED
#define RUNTIMECLASS_Windows_Foundation_ExeServerRegistration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Foundation_ExeServerRegistration[] = L"Windows.Foundation.ExeServerRegistration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
#endif // __activationregistration_p_h__

#endif // __activationregistration_h__
