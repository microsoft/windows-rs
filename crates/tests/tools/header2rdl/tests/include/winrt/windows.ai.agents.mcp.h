
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
#ifndef __windows2Eai2Eagents2Emcp_h__
#define __windows2Eai2Eagents2Emcp_h__
#ifndef __windows2Eai2Eagents2Emcp_p_h__
#define __windows2Eai2Eagents2Emcp_p_h__


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
#if !defined(WINDOWS_AI_AGENTS_AGENTSCONTRACT_VERSION)
#define WINDOWS_AI_AGENTS_AGENTSCONTRACT_VERSION 0x20000
#endif // defined(WINDOWS_AI_AGENTS_AGENTSCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.AI.Agents.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterExperimental_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterExperimental_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Agents {
                namespace Mcp {
                    interface IMcpMessageFilterExperimental;
                } /* Mcp */
            } /* Agents */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterExperimental ABI::Windows::AI::Agents::Mcp::IMcpMessageFilterExperimental

#endif // ____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterExperimental_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponse_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponse_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Agents {
                namespace Mcp {
                    interface IMcpMessageFilterResponse;
                } /* Mcp */
            } /* Agents */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponse ABI::Windows::AI::Agents::Mcp::IMcpMessageFilterResponse

#endif // ____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponse_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Agents {
                namespace Mcp {
                    interface IMcpMessageFilterResponseExperimental;
                } /* Mcp */
            } /* Agents */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental ABI::Windows::AI::Agents::Mcp::IMcpMessageFilterResponseExperimental

#endif // ____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Agents {
                namespace Mcp {
                    interface IMcpMessageFilterResponseExperimental2;
                } /* Mcp */
            } /* Agents */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental2 ABI::Windows::AI::Agents::Mcp::IMcpMessageFilterResponseExperimental2

#endif // ____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental2_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Agents {
                namespace Mcp {
                    typedef enum McpMessageDirection : int McpMessageDirection;
                } /* Mcp */
            } /* Agents */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Agents {
                namespace Mcp {
                    class McpMessageFilterResponse;
                } /* Mcp */
            } /* Agents */
        } /* AI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.AI.Agents.Mcp.McpMessageDirection
 *
 * Introduced to Windows.AI.Agents.AgentsContract in version 2.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_AGENTS_AGENTSCONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Agents {
                namespace Mcp {
                    enum McpMessageDirection : int
                    {
                        McpMessageDirection_ClientToServer = 0,
                        McpMessageDirection_ServerToClient = 1,
                    };
                } /* Mcp */
            } /* Agents */
        } /* AI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_AI_AGENTS_AGENTSCONTRACT_VERSION >= 0x20000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.AI.Agents.Mcp.IMcpMessageFilterExperimental
 *
 * Introduced to Windows.AI.Agents.AgentsContract in version 2.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_AGENTS_AGENTSCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterExperimental_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterExperimental_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Agents_Mcp_IMcpMessageFilterExperimental[] = L"Windows.AI.Agents.Mcp.IMcpMessageFilterExperimental";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Agents {
                namespace Mcp {
                    MIDL_INTERFACE("c5f8f821-895c-5241-b45a-92e249a7d873")
                    IMcpMessageFilterExperimental : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Initialize(
                            HSTRING clientAppUserModelId,
                            UINT32 clientProcessId,
                            HSTRING serverIdentity,
                            HSTRING serverName,
                            UINT32 serverProcessId
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE OnMessage(
                            HSTRING message,
                            ABI::Windows::AI::Agents::Mcp::McpMessageDirection direction,
                            ABI::Windows::AI::Agents::Mcp::IMcpMessageFilterResponse* filterResponse
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IMcpMessageFilterExperimental = __uuidof(IMcpMessageFilterExperimental);
                } /* Mcp */
            } /* Agents */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterExperimental;
#endif /* !defined(____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterExperimental_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_AGENTS_AGENTSCONTRACT_VERSION >= 0x20000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.AI.Agents.Mcp.IMcpMessageFilterResponse
 *
 * Introduced to Windows.AI.Agents.AgentsContract in version 2.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.AI.Agents.Mcp.McpMessageFilterResponse
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_AGENTS_AGENTSCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponse_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponse_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Agents_Mcp_IMcpMessageFilterResponse[] = L"Windows.AI.Agents.Mcp.IMcpMessageFilterResponse";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Agents {
                namespace Mcp {
                    MIDL_INTERFACE("363ce02c-7098-5e13-a408-7b43e1f452ac")
                    IMcpMessageFilterResponse : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IMcpMessageFilterResponse = __uuidof(IMcpMessageFilterResponse);
                } /* Mcp */
            } /* Agents */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponse;
#endif /* !defined(____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponse_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_AGENTS_AGENTSCONTRACT_VERSION >= 0x20000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.AI.Agents.Mcp.IMcpMessageFilterResponseExperimental
 *
 * Introduced to Windows.AI.Agents.AgentsContract in version 2.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_AGENTS_AGENTSCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Agents_Mcp_IMcpMessageFilterResponseExperimental[] = L"Windows.AI.Agents.Mcp.IMcpMessageFilterResponseExperimental";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Agents {
                namespace Mcp {
                    MIDL_INTERFACE("e215b5f2-cb02-56cf-aab0-84aef65d1665")
                    IMcpMessageFilterResponseExperimental : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsAllowed(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IsAllowed(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_MessageIfNotAllowed(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_MessageIfNotAllowed(
                            HSTRING value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IMcpMessageFilterResponseExperimental = __uuidof(IMcpMessageFilterResponseExperimental);
                } /* Mcp */
            } /* Agents */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental;
#endif /* !defined(____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_AGENTS_AGENTSCONTRACT_VERSION >= 0x20000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.AI.Agents.Mcp.IMcpMessageFilterResponseExperimental2
 *
 * Introduced to Windows.AI.Agents.AgentsContract in version 2.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_AGENTS_AGENTSCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Agents_Mcp_IMcpMessageFilterResponseExperimental2[] = L"Windows.AI.Agents.Mcp.IMcpMessageFilterResponseExperimental2";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Agents {
                namespace Mcp {
                    MIDL_INTERFACE("10f4b099-6632-505a-a638-e704c7e47abf")
                    IMcpMessageFilterResponseExperimental2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Allow(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Reject(
                            HSTRING reason
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IMcpMessageFilterResponseExperimental2 = __uuidof(IMcpMessageFilterResponseExperimental2);
                } /* Mcp */
            } /* Agents */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental2;
#endif /* !defined(____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_AGENTS_AGENTSCONTRACT_VERSION >= 0x20000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Class Windows.AI.Agents.Mcp.McpMessageFilterResponse
 *
 * Introduced to Windows.AI.Agents.AgentsContract in version 2.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Class implements the following interfaces:
 *    Windows.AI.Agents.Mcp.IMcpMessageFilterResponse ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_AGENTS_AGENTSCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_AI_Agents_Mcp_McpMessageFilterResponse_DEFINED
#define RUNTIMECLASS_Windows_AI_Agents_Mcp_McpMessageFilterResponse_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Agents_Mcp_McpMessageFilterResponse[] = L"Windows.AI.Agents.Mcp.McpMessageFilterResponse";
#endif
#endif // WINDOWS_AI_AGENTS_AGENTSCONTRACT_VERSION >= 0x20000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterExperimental_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterExperimental_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterExperimental __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterExperimental;

#endif // ____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterExperimental_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponse_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponse_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponse __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponse;

#endif // ____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponse_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental;

#endif // ____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental2 __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental2;

#endif // ____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental2_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef enum __x_ABI_CWindows_CAI_CAgents_CMcp_CMcpMessageDirection __x_ABI_CWindows_CAI_CAgents_CMcp_CMcpMessageDirection;

/*
 *
 * Struct Windows.AI.Agents.Mcp.McpMessageDirection
 *
 * Introduced to Windows.AI.Agents.AgentsContract in version 2.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_AGENTS_AGENTSCONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CAI_CAgents_CMcp_CMcpMessageDirection
{
    McpMessageDirection_ClientToServer = 0,
    McpMessageDirection_ServerToClient = 1,
};
#endif // WINDOWS_AI_AGENTS_AGENTSCONTRACT_VERSION >= 0x20000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.AI.Agents.Mcp.IMcpMessageFilterExperimental
 *
 * Introduced to Windows.AI.Agents.AgentsContract in version 2.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_AGENTS_AGENTSCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterExperimental_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterExperimental_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Agents_Mcp_IMcpMessageFilterExperimental[] = L"Windows.AI.Agents.Mcp.IMcpMessageFilterExperimental";
typedef struct __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterExperimentalVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterExperimental* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterExperimental* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterExperimental* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterExperimental* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterExperimental* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterExperimental* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Initialize)(__x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterExperimental* This,
        HSTRING clientAppUserModelId,
        UINT32 clientProcessId,
        HSTRING serverIdentity,
        HSTRING serverName,
        UINT32 serverProcessId);
    HRESULT (STDMETHODCALLTYPE* OnMessage)(__x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterExperimental* This,
        HSTRING message,
        enum __x_ABI_CWindows_CAI_CAgents_CMcp_CMcpMessageDirection direction,
        __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponse* filterResponse);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterExperimentalVtbl;

interface __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterExperimental
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterExperimentalVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterExperimental_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterExperimental_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterExperimental_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterExperimental_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterExperimental_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterExperimental_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterExperimental_Initialize(This, clientAppUserModelId, clientProcessId, serverIdentity, serverName, serverProcessId) \
    ((This)->lpVtbl->Initialize(This, clientAppUserModelId, clientProcessId, serverIdentity, serverName, serverProcessId))

#define __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterExperimental_OnMessage(This, message, direction, filterResponse) \
    ((This)->lpVtbl->OnMessage(This, message, direction, filterResponse))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterExperimental;
#endif /* !defined(____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterExperimental_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_AGENTS_AGENTSCONTRACT_VERSION >= 0x20000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.AI.Agents.Mcp.IMcpMessageFilterResponse
 *
 * Introduced to Windows.AI.Agents.AgentsContract in version 2.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.AI.Agents.Mcp.McpMessageFilterResponse
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_AGENTS_AGENTSCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponse_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponse_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Agents_Mcp_IMcpMessageFilterResponse[] = L"Windows.AI.Agents.Mcp.IMcpMessageFilterResponse";
typedef struct __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponse* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponse* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponse* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponse* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponse* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponse* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseVtbl;

interface __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponse
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponse_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponse_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponse_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponse_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponse_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponse_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponse;
#endif /* !defined(____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponse_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_AGENTS_AGENTSCONTRACT_VERSION >= 0x20000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.AI.Agents.Mcp.IMcpMessageFilterResponseExperimental
 *
 * Introduced to Windows.AI.Agents.AgentsContract in version 2.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_AGENTS_AGENTSCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Agents_Mcp_IMcpMessageFilterResponseExperimental[] = L"Windows.AI.Agents.Mcp.IMcpMessageFilterResponseExperimental";
typedef struct __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimentalVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsAllowed)(__x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsAllowed)(__x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_MessageIfNotAllowed)(__x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_MessageIfNotAllowed)(__x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimentalVtbl;

interface __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimentalVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental_get_IsAllowed(This, value) \
    ((This)->lpVtbl->get_IsAllowed(This, value))

#define __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental_put_IsAllowed(This, value) \
    ((This)->lpVtbl->put_IsAllowed(This, value))

#define __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental_get_MessageIfNotAllowed(This, value) \
    ((This)->lpVtbl->get_MessageIfNotAllowed(This, value))

#define __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental_put_MessageIfNotAllowed(This, value) \
    ((This)->lpVtbl->put_MessageIfNotAllowed(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental;
#endif /* !defined(____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_AGENTS_AGENTSCONTRACT_VERSION >= 0x20000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.AI.Agents.Mcp.IMcpMessageFilterResponseExperimental2
 *
 * Introduced to Windows.AI.Agents.AgentsContract in version 2.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_AGENTS_AGENTSCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Agents_Mcp_IMcpMessageFilterResponseExperimental2[] = L"Windows.AI.Agents.Mcp.IMcpMessageFilterResponseExperimental2";
typedef struct __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Allow)(__x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental2* This);
    HRESULT (STDMETHODCALLTYPE* Reject)(__x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental2* This,
        HSTRING reason);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental2Vtbl;

interface __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental2
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental2_Allow(This) \
    ((This)->lpVtbl->Allow(This))

#define __x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental2_Reject(This, reason) \
    ((This)->lpVtbl->Reject(This, reason))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental2;
#endif /* !defined(____x_ABI_CWindows_CAI_CAgents_CMcp_CIMcpMessageFilterResponseExperimental2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_AGENTS_AGENTSCONTRACT_VERSION >= 0x20000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Class Windows.AI.Agents.Mcp.McpMessageFilterResponse
 *
 * Introduced to Windows.AI.Agents.AgentsContract in version 2.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Class implements the following interfaces:
 *    Windows.AI.Agents.Mcp.IMcpMessageFilterResponse ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_AGENTS_AGENTSCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_AI_Agents_Mcp_McpMessageFilterResponse_DEFINED
#define RUNTIMECLASS_Windows_AI_Agents_Mcp_McpMessageFilterResponse_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Agents_Mcp_McpMessageFilterResponse[] = L"Windows.AI.Agents.Mcp.McpMessageFilterResponse";
#endif
#endif // WINDOWS_AI_AGENTS_AGENTSCONTRACT_VERSION >= 0x20000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eai2Eagents2Emcp_p_h__

#endif // __windows2Eai2Eagents2Emcp_h__
