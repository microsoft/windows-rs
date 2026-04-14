
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
#ifndef __windows2Eui2Eshell2Etasks_h__
#define __windows2Eui2Eshell2Etasks_h__
#ifndef __windows2Eui2Eshell2Etasks_p_h__
#define __windows2Eui2Eshell2Etasks_p_h__


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

#if !defined(WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION)
#define WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION 0x20000
#endif // defined(WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                namespace Tasks {
                    interface IAppTaskContent;
                } /* Tasks */
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent ABI::Windows::UI::Shell::Tasks::IAppTaskContent

#endif // ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                namespace Tasks {
                    interface IAppTaskContentStatics;
                } /* Tasks */
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics ABI::Windows::UI::Shell::Tasks::IAppTaskContentStatics

#endif // ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                namespace Tasks {
                    interface IAppTaskInfo;
                } /* Tasks */
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo ABI::Windows::UI::Shell::Tasks::IAppTaskInfo

#endif // ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                namespace Tasks {
                    interface IAppTaskInfo2;
                } /* Tasks */
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2 ABI::Windows::UI::Shell::Tasks::IAppTaskInfo2

#endif // ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                namespace Tasks {
                    interface IAppTaskInfoStatics;
                } /* Tasks */
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStatics ABI::Windows::UI::Shell::Tasks::IAppTaskInfoStatics

#endif // ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAsset_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAsset_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                namespace Tasks {
                    interface IAppTaskResultAsset;
                } /* Tasks */
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAsset ABI::Windows::UI::Shell::Tasks::IAppTaskResultAsset

#endif // ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAsset_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                namespace Tasks {
                    interface IAppTaskResultAssetFactory;
                } /* Tasks */
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetFactory ABI::Windows::UI::Shell::Tasks::IAppTaskResultAssetFactory

#endif // ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct DateTime DateTime;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIReference_1_Windows__CFoundation__CDateTime_USE
#define DEF___FIReference_1_Windows__CFoundation__CDateTime_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5541d8a7-497c-5aa4-86fc-7713adbf2a2c"))
IReference<struct ABI::Windows::Foundation::DateTime> : IReference_impl<struct ABI::Windows::Foundation::DateTime>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.Foundation.DateTime>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<struct ABI::Windows::Foundation::DateTime> __FIReference_1_Windows__CFoundation__CDateTime_t;
#define __FIReference_1_Windows__CFoundation__CDateTime ABI::Windows::Foundation::__FIReference_1_Windows__CFoundation__CDateTime_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CFoundation__CDateTime_USE */

#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

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

namespace ABI {
    namespace Windows {
        namespace Foundation {
            class Uri;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IUriRuntimeClass;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClass ABI::Windows::Foundation::IUriRuntimeClass

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                namespace Tasks {
                    typedef enum AppTaskState : int AppTaskState;
                } /* Tasks */
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                namespace Tasks {
                    class AppTaskContent;
                } /* Tasks */
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                namespace Tasks {
                    class AppTaskInfo;
                } /* Tasks */
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                namespace Tasks {
                    class AppTaskResultAsset;
                } /* Tasks */
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.UI.Shell.Tasks.AppTaskState
 *
 * Introduced to Windows.UI.Shell.Tasks.AppTaskContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                namespace Tasks {
                    enum AppTaskState : int
                    {
                        AppTaskState_Running = 0,
                        AppTaskState_Completed = 1,
                        AppTaskState_NeedsAttention = 2,
                        AppTaskState_Paused = 3,
                        AppTaskState_Error = 4,
                    };
                } /* Tasks */
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.UI.Shell.Tasks.IAppTaskContent
 *
 * Introduced to Windows.UI.Shell.Tasks.AppTaskContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.Tasks.AppTaskContent
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_Tasks_IAppTaskContent[] = L"Windows.UI.Shell.Tasks.IAppTaskContent";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                namespace Tasks {
                    MIDL_INTERFACE("2411bf59-1b2d-5b63-8181-03d6c2248a68")
                    IAppTaskContent : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE AddButton(
                            HSTRING text,
                            ABI::Windows::Foundation::IUriRuntimeClass* actionUri
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetTextInput(
                            HSTRING placeholderText,
                            HSTRING actionUriTemplate
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetQuestion(
                            HSTRING question
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAppTaskContent = __uuidof(IAppTaskContent);
                } /* Tasks */
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.UI.Shell.Tasks.IAppTaskContentStatics
 *
 * Introduced to Windows.UI.Shell.Tasks.AppTaskContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.Tasks.AppTaskContent
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_Tasks_IAppTaskContentStatics[] = L"Windows.UI.Shell.Tasks.IAppTaskContentStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                namespace Tasks {
                    MIDL_INTERFACE("aabd19f6-7afc-5b1b-94f6-5dc9dc9af9e7")
                    IAppTaskContentStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateSequenceOfSteps(
                            UINT32 completedStepsLength,
                            HSTRING* completedSteps,
                            HSTRING executingStep,
                            ABI::Windows::UI::Shell::Tasks::IAppTaskContent** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreatePreviewThumbnail(
                            ABI::Windows::Foundation::IUriRuntimeClass* imageUri,
                            HSTRING executingStep,
                            ABI::Windows::UI::Shell::Tasks::IAppTaskContent** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateTextSummaryResult(
                            HSTRING text,
                            ABI::Windows::UI::Shell::Tasks::IAppTaskContent** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateGeneratedAssetsResult(
                            UINT32 assetsLength,
                            ABI::Windows::UI::Shell::Tasks::IAppTaskResultAsset** assets,
                            ABI::Windows::UI::Shell::Tasks::IAppTaskContent** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_MaxButtons(
                            UINT32* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAppTaskContentStatics = __uuidof(IAppTaskContentStatics);
                } /* Tasks */
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.UI.Shell.Tasks.IAppTaskInfo
 *
 * Introduced to Windows.UI.Shell.Tasks.AppTaskContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.Tasks.AppTaskInfo
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_Tasks_IAppTaskInfo[] = L"Windows.UI.Shell.Tasks.IAppTaskInfo";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                namespace Tasks {
                    MIDL_INTERFACE("6720eed6-435b-5db9-be66-9343b70654f7")
                    IAppTaskInfo : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Remove(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Update(
                            ABI::Windows::UI::Shell::Tasks::AppTaskState state,
                            ABI::Windows::UI::Shell::Tasks::IAppTaskContent* content
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE UpdateState(
                            ABI::Windows::UI::Shell::Tasks::AppTaskState state
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE UpdateTitles(
                            HSTRING title,
                            HSTRING subtitle
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetCompletedSteps(
                            UINT32* resultLength,
                            HSTRING** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetExecutingStep(
                            HSTRING* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Title(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Subtitle(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DeepLink(
                            ABI::Windows::Foundation::IUriRuntimeClass** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IconUri(
                            ABI::Windows::Foundation::IUriRuntimeClass** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_State(
                            ABI::Windows::UI::Shell::Tasks::AppTaskState* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StartTime(
                            ABI::Windows::Foundation::DateTime* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_EndTime(
                            __FIReference_1_Windows__CFoundation__CDateTime** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAppTaskInfo = __uuidof(IAppTaskInfo);
                } /* Tasks */
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.UI.Shell.Tasks.IAppTaskInfo2
 *
 * Introduced to Windows.UI.Shell.Tasks.AppTaskContract in version 2.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.Tasks.AppTaskInfo
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_Tasks_IAppTaskInfo2[] = L"Windows.UI.Shell.Tasks.IAppTaskInfo2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                namespace Tasks {
                    MIDL_INTERFACE("ad724d71-f137-51c0-8111-3552436bf447")
                    IAppTaskInfo2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Id(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HiddenByUser(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE UpdateDeepLink(
                            ABI::Windows::Foundation::IUriRuntimeClass* deepLink
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAppTaskInfo2 = __uuidof(IAppTaskInfo2);
                } /* Tasks */
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x20000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.UI.Shell.Tasks.IAppTaskInfoStatics
 *
 * Introduced to Windows.UI.Shell.Tasks.AppTaskContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.Tasks.AppTaskInfo
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_Tasks_IAppTaskInfoStatics[] = L"Windows.UI.Shell.Tasks.IAppTaskInfoStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                namespace Tasks {
                    MIDL_INTERFACE("a0b0434f-c640-5800-88c9-d7691ac2f48f")
                    IAppTaskInfoStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE IsSupported(
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE FindAll(
                            UINT32* resultLength,
                            ABI::Windows::UI::Shell::Tasks::IAppTaskInfo*** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Create(
                            HSTRING title,
                            HSTRING subtitle,
                            ABI::Windows::Foundation::IUriRuntimeClass* deepLink,
                            ABI::Windows::Foundation::IUriRuntimeClass* iconUri,
                            ABI::Windows::UI::Shell::Tasks::IAppTaskContent* content,
                            ABI::Windows::UI::Shell::Tasks::IAppTaskInfo** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAppTaskInfoStatics = __uuidof(IAppTaskInfoStatics);
                } /* Tasks */
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.UI.Shell.Tasks.IAppTaskResultAsset
 *
 * Introduced to Windows.UI.Shell.Tasks.AppTaskContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.Tasks.AppTaskResultAsset
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAsset_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAsset_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_Tasks_IAppTaskResultAsset[] = L"Windows.UI.Shell.Tasks.IAppTaskResultAsset";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                namespace Tasks {
                    MIDL_INTERFACE("75d0c2b3-8a31-5f8f-bda4-bdca96e95532")
                    IAppTaskResultAsset : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IAppTaskResultAsset = __uuidof(IAppTaskResultAsset);
                } /* Tasks */
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAsset;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAsset_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.UI.Shell.Tasks.IAppTaskResultAssetFactory
 *
 * Introduced to Windows.UI.Shell.Tasks.AppTaskContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.Tasks.AppTaskResultAsset
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_Tasks_IAppTaskResultAssetFactory[] = L"Windows.UI.Shell.Tasks.IAppTaskResultAssetFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                namespace Tasks {
                    MIDL_INTERFACE("0334d9df-0e06-5999-ba41-85d72e980085")
                    IAppTaskResultAssetFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            HSTRING name,
                            HSTRING context,
                            ABI::Windows::Foundation::IUriRuntimeClass* iconUri,
                            ABI::Windows::Foundation::IUriRuntimeClass* assetUri,
                            ABI::Windows::UI::Shell::Tasks::IAppTaskResultAsset** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAppTaskResultAssetFactory = __uuidof(IAppTaskResultAssetFactory);
                } /* Tasks */
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Class Windows.UI.Shell.Tasks.AppTaskContent
 *
 * Introduced to Windows.UI.Shell.Tasks.AppTaskContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Shell.Tasks.IAppTaskContentStatics interface starting with version 1.0 of the Windows.UI.Shell.Tasks.AppTaskContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.Tasks.IAppTaskContent ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Shell_Tasks_AppTaskContent_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_Tasks_AppTaskContent_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_Tasks_AppTaskContent[] = L"Windows.UI.Shell.Tasks.AppTaskContent";
#endif
#endif // WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Class Windows.UI.Shell.Tasks.AppTaskInfo
 *
 * Introduced to Windows.UI.Shell.Tasks.AppTaskContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Shell.Tasks.IAppTaskInfoStatics interface starting with version 1.0 of the Windows.UI.Shell.Tasks.AppTaskContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.Tasks.IAppTaskInfo ** Default Interface **
 *    Windows.UI.Shell.Tasks.IAppTaskInfo2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Shell_Tasks_AppTaskInfo_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_Tasks_AppTaskInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_Tasks_AppTaskInfo[] = L"Windows.UI.Shell.Tasks.AppTaskInfo";
#endif
#endif // WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Class Windows.UI.Shell.Tasks.AppTaskResultAsset
 *
 * Introduced to Windows.UI.Shell.Tasks.AppTaskContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Shell.Tasks.IAppTaskResultAssetFactory interface starting with version 1.0 of the Windows.UI.Shell.Tasks.AppTaskContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.Tasks.IAppTaskResultAsset ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Shell_Tasks_AppTaskResultAsset_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_Tasks_AppTaskResultAsset_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_Tasks_AppTaskResultAsset[] = L"Windows.UI.Shell.Tasks.AppTaskResultAsset";
#endif
#endif // WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent;

#endif // ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics;

#endif // ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo;

#endif // ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2 __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2;

#endif // ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStatics __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStatics;

#endif // ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAsset_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAsset_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAsset __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAsset;

#endif // ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAsset_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetFactory __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetFactory;

#endif // ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIReference_1_Windows__CFoundation__CDateTime_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CFoundation__CDateTime_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CFoundation__CDateTime __FIReference_1_Windows__CFoundation__CDateTime;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CFoundation__CDateTime;

typedef struct __FIReference_1_Windows__CFoundation__CDateTimeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CFoundation__CDateTime* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CFoundation__CDateTime* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* result);

    END_INTERFACE
} __FIReference_1_Windows__CFoundation__CDateTimeVtbl;

interface __FIReference_1_Windows__CFoundation__CDateTime
{
    CONST_VTBL struct __FIReference_1_Windows__CFoundation__CDateTimeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CFoundation__CDateTime_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CFoundation__CDateTime_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CFoundation__CDateTime_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CFoundation__CDateTime_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CFoundation__CDateTime_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CFoundation__CDateTime_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CFoundation__CDateTime_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CFoundation__CDateTime_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CUI_CShell_CTasks_CAppTaskState __x_ABI_CWindows_CUI_CShell_CTasks_CAppTaskState;

/*
 *
 * Struct Windows.UI.Shell.Tasks.AppTaskState
 *
 * Introduced to Windows.UI.Shell.Tasks.AppTaskContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CShell_CTasks_CAppTaskState
{
    AppTaskState_Running = 0,
    AppTaskState_Completed = 1,
    AppTaskState_NeedsAttention = 2,
    AppTaskState_Paused = 3,
    AppTaskState_Error = 4,
};
#endif // WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.UI.Shell.Tasks.IAppTaskContent
 *
 * Introduced to Windows.UI.Shell.Tasks.AppTaskContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.Tasks.AppTaskContent
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_Tasks_IAppTaskContent[] = L"Windows.UI.Shell.Tasks.IAppTaskContent";
typedef struct __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* AddButton)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent* This,
        HSTRING text,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* actionUri);
    HRESULT (STDMETHODCALLTYPE* SetTextInput)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent* This,
        HSTRING placeholderText,
        HSTRING actionUriTemplate);
    HRESULT (STDMETHODCALLTYPE* SetQuestion)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent* This,
        HSTRING question);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentVtbl;

interface __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent_AddButton(This, text, actionUri) \
    ((This)->lpVtbl->AddButton(This, text, actionUri))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent_SetTextInput(This, placeholderText, actionUriTemplate) \
    ((This)->lpVtbl->SetTextInput(This, placeholderText, actionUriTemplate))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent_SetQuestion(This, question) \
    ((This)->lpVtbl->SetQuestion(This, question))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.UI.Shell.Tasks.IAppTaskContentStatics
 *
 * Introduced to Windows.UI.Shell.Tasks.AppTaskContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.Tasks.AppTaskContent
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_Tasks_IAppTaskContentStatics[] = L"Windows.UI.Shell.Tasks.IAppTaskContentStatics";
typedef struct __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateSequenceOfSteps)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics* This,
        UINT32 completedStepsLength,
        HSTRING* completedSteps,
        HSTRING executingStep,
        __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent** result);
    HRESULT (STDMETHODCALLTYPE* CreatePreviewThumbnail)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* imageUri,
        HSTRING executingStep,
        __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent** result);
    HRESULT (STDMETHODCALLTYPE* CreateTextSummaryResult)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics* This,
        HSTRING text,
        __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent** result);
    HRESULT (STDMETHODCALLTYPE* CreateGeneratedAssetsResult)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics* This,
        UINT32 assetsLength,
        __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAsset** assets,
        __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent** result);
    HRESULT (STDMETHODCALLTYPE* get_MaxButtons)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStaticsVtbl;

interface __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics_CreateSequenceOfSteps(This, completedStepsLength, completedSteps, executingStep, result) \
    ((This)->lpVtbl->CreateSequenceOfSteps(This, completedStepsLength, completedSteps, executingStep, result))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics_CreatePreviewThumbnail(This, imageUri, executingStep, result) \
    ((This)->lpVtbl->CreatePreviewThumbnail(This, imageUri, executingStep, result))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics_CreateTextSummaryResult(This, text, result) \
    ((This)->lpVtbl->CreateTextSummaryResult(This, text, result))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics_CreateGeneratedAssetsResult(This, assetsLength, assets, result) \
    ((This)->lpVtbl->CreateGeneratedAssetsResult(This, assetsLength, assets, result))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics_get_MaxButtons(This, value) \
    ((This)->lpVtbl->get_MaxButtons(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContentStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.UI.Shell.Tasks.IAppTaskInfo
 *
 * Introduced to Windows.UI.Shell.Tasks.AppTaskContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.Tasks.AppTaskInfo
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_Tasks_IAppTaskInfo[] = L"Windows.UI.Shell.Tasks.IAppTaskInfo";
typedef struct __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Remove)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo* This);
    HRESULT (STDMETHODCALLTYPE* Update)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo* This,
        enum __x_ABI_CWindows_CUI_CShell_CTasks_CAppTaskState state,
        __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent* content);
    HRESULT (STDMETHODCALLTYPE* UpdateState)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo* This,
        enum __x_ABI_CWindows_CUI_CShell_CTasks_CAppTaskState state);
    HRESULT (STDMETHODCALLTYPE* UpdateTitles)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo* This,
        HSTRING title,
        HSTRING subtitle);
    HRESULT (STDMETHODCALLTYPE* GetCompletedSteps)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo* This,
        UINT32* resultLength,
        HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* GetExecutingStep)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Subtitle)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DeepLink)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_IconUri)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_State)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo* This,
        enum __x_ABI_CWindows_CUI_CShell_CTasks_CAppTaskState* value);
    HRESULT (STDMETHODCALLTYPE* get_StartTime)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_EndTime)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoVtbl;

interface __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo_Remove(This) \
    ((This)->lpVtbl->Remove(This))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo_Update(This, state, content) \
    ((This)->lpVtbl->Update(This, state, content))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo_UpdateState(This, state) \
    ((This)->lpVtbl->UpdateState(This, state))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo_UpdateTitles(This, title, subtitle) \
    ((This)->lpVtbl->UpdateTitles(This, title, subtitle))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo_GetCompletedSteps(This, resultLength, result) \
    ((This)->lpVtbl->GetCompletedSteps(This, resultLength, result))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo_GetExecutingStep(This, result) \
    ((This)->lpVtbl->GetExecutingStep(This, result))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo_get_Subtitle(This, value) \
    ((This)->lpVtbl->get_Subtitle(This, value))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo_get_DeepLink(This, value) \
    ((This)->lpVtbl->get_DeepLink(This, value))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo_get_IconUri(This, value) \
    ((This)->lpVtbl->get_IconUri(This, value))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo_get_State(This, value) \
    ((This)->lpVtbl->get_State(This, value))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo_get_StartTime(This, value) \
    ((This)->lpVtbl->get_StartTime(This, value))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo_get_EndTime(This, value) \
    ((This)->lpVtbl->get_EndTime(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.UI.Shell.Tasks.IAppTaskInfo2
 *
 * Introduced to Windows.UI.Shell.Tasks.AppTaskContract in version 2.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.Tasks.AppTaskInfo
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_Tasks_IAppTaskInfo2[] = L"Windows.UI.Shell.Tasks.IAppTaskInfo2";
typedef struct __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_HiddenByUser)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* UpdateDeepLink)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* deepLink);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2Vtbl;

interface __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2_get_HiddenByUser(This, value) \
    ((This)->lpVtbl->get_HiddenByUser(This, value))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2_UpdateDeepLink(This, deepLink) \
    ((This)->lpVtbl->UpdateDeepLink(This, deepLink))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo2_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x20000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.UI.Shell.Tasks.IAppTaskInfoStatics
 *
 * Introduced to Windows.UI.Shell.Tasks.AppTaskContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.Tasks.AppTaskInfo
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_Tasks_IAppTaskInfoStatics[] = L"Windows.UI.Shell.Tasks.IAppTaskInfoStatics";
typedef struct __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* IsSupported)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStatics* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* FindAll)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStatics* This,
        UINT32* resultLength,
        __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo*** result);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStatics* This,
        HSTRING title,
        HSTRING subtitle,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* deepLink,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* iconUri,
        __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskContent* content,
        __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfo** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStaticsVtbl;

interface __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStatics_IsSupported(This, result) \
    ((This)->lpVtbl->IsSupported(This, result))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStatics_FindAll(This, resultLength, result) \
    ((This)->lpVtbl->FindAll(This, resultLength, result))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStatics_Create(This, title, subtitle, deepLink, iconUri, content, result) \
    ((This)->lpVtbl->Create(This, title, subtitle, deepLink, iconUri, content, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskInfoStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.UI.Shell.Tasks.IAppTaskResultAsset
 *
 * Introduced to Windows.UI.Shell.Tasks.AppTaskContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.Tasks.AppTaskResultAsset
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAsset_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAsset_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_Tasks_IAppTaskResultAsset[] = L"Windows.UI.Shell.Tasks.IAppTaskResultAsset";
typedef struct __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAsset* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAsset* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAsset* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAsset* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAsset* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAsset* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetVtbl;

interface __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAsset
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAsset_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAsset_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAsset_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAsset_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAsset_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAsset_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAsset;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAsset_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.UI.Shell.Tasks.IAppTaskResultAssetFactory
 *
 * Introduced to Windows.UI.Shell.Tasks.AppTaskContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.UI.Shell.Tasks.AppTaskResultAsset
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Shell_Tasks_IAppTaskResultAssetFactory[] = L"Windows.UI.Shell.Tasks.IAppTaskResultAssetFactory";
typedef struct __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetFactory* This,
        HSTRING name,
        HSTRING context,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* iconUri,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* assetUri,
        __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAsset** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetFactoryVtbl;

interface __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetFactory_CreateInstance(This, name, context, iconUri, assetUri, value) \
    ((This)->lpVtbl->CreateInstance(This, name, context, iconUri, assetUri, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CShell_CTasks_CIAppTaskResultAssetFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Class Windows.UI.Shell.Tasks.AppTaskContent
 *
 * Introduced to Windows.UI.Shell.Tasks.AppTaskContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Shell.Tasks.IAppTaskContentStatics interface starting with version 1.0 of the Windows.UI.Shell.Tasks.AppTaskContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.Tasks.IAppTaskContent ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Shell_Tasks_AppTaskContent_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_Tasks_AppTaskContent_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_Tasks_AppTaskContent[] = L"Windows.UI.Shell.Tasks.AppTaskContent";
#endif
#endif // WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Class Windows.UI.Shell.Tasks.AppTaskInfo
 *
 * Introduced to Windows.UI.Shell.Tasks.AppTaskContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Shell.Tasks.IAppTaskInfoStatics interface starting with version 1.0 of the Windows.UI.Shell.Tasks.AppTaskContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.Tasks.IAppTaskInfo ** Default Interface **
 *    Windows.UI.Shell.Tasks.IAppTaskInfo2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Shell_Tasks_AppTaskInfo_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_Tasks_AppTaskInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_Tasks_AppTaskInfo[] = L"Windows.UI.Shell.Tasks.AppTaskInfo";
#endif
#endif // WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Class Windows.UI.Shell.Tasks.AppTaskResultAsset
 *
 * Introduced to Windows.UI.Shell.Tasks.AppTaskContract in version 1.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Shell.Tasks.IAppTaskResultAssetFactory interface starting with version 1.0 of the Windows.UI.Shell.Tasks.AppTaskContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Shell.Tasks.IAppTaskResultAsset ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Shell_Tasks_AppTaskResultAsset_DEFINED
#define RUNTIMECLASS_Windows_UI_Shell_Tasks_AppTaskResultAsset_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Shell_Tasks_AppTaskResultAsset[] = L"Windows.UI.Shell.Tasks.AppTaskResultAsset";
#endif
#endif // WINDOWS_UI_SHELL_TASKS_APPTASKCONTRACT_VERSION >= 0x10000
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
#endif // __windows2Eui2Eshell2Etasks_p_h__

#endif // __windows2Eui2Eshell2Etasks_h__
