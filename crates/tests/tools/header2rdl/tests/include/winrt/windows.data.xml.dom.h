
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
#ifndef __windows2Edata2Exml2Edom_h__
#define __windows2Edata2Exml2Edom_h__
#ifndef __windows2Edata2Exml2Edom_p_h__
#define __windows2Edata2Exml2Edom_p_h__


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
#include "Windows.Storage.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIDtdEntity_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIDtdEntity_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    interface IDtdEntity;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CXml_CDom_CIDtdEntity ABI::Windows::Data::Xml::Dom::IDtdEntity

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIDtdEntity_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIDtdNotation_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIDtdNotation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    interface IDtdNotation;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CXml_CDom_CIDtdNotation ABI::Windows::Data::Xml::Dom::IDtdNotation

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIDtdNotation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    interface IXmlAttribute;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute ABI::Windows::Data::Xml::Dom::IXmlAttribute

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlCDataSection_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlCDataSection_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    interface IXmlCDataSection;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlCDataSection ABI::Windows::Data::Xml::Dom::IXmlCDataSection

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlCDataSection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    interface IXmlCharacterData;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData ABI::Windows::Data::Xml::Dom::IXmlCharacterData

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlComment_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlComment_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    interface IXmlComment;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlComment ABI::Windows::Data::Xml::Dom::IXmlComment

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlComment_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    interface IXmlDocument;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument ABI::Windows::Data::Xml::Dom::IXmlDocument

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentFragment_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentFragment_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    interface IXmlDocumentFragment;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentFragment ABI::Windows::Data::Xml::Dom::IXmlDocumentFragment

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentFragment_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    interface IXmlDocumentIO;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO ABI::Windows::Data::Xml::Dom::IXmlDocumentIO

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO2_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    interface IXmlDocumentIO2;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO2 ABI::Windows::Data::Xml::Dom::IXmlDocumentIO2

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    interface IXmlDocumentStatics;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics ABI::Windows::Data::Xml::Dom::IXmlDocumentStatics

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentType_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentType_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    interface IXmlDocumentType;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentType ABI::Windows::Data::Xml::Dom::IXmlDocumentType

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentType_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDomImplementation_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDomImplementation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    interface IXmlDomImplementation;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDomImplementation ABI::Windows::Data::Xml::Dom::IXmlDomImplementation

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDomImplementation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlElement_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlElement_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    interface IXmlElement;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlElement ABI::Windows::Data::Xml::Dom::IXmlElement

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlElement_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlEntityReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlEntityReference_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    interface IXmlEntityReference;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlEntityReference ABI::Windows::Data::Xml::Dom::IXmlEntityReference

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlEntityReference_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    interface IXmlLoadSettings;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings ABI::Windows::Data::Xml::Dom::IXmlLoadSettings

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    interface IXmlNamedNodeMap;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap ABI::Windows::Data::Xml::Dom::IXmlNamedNodeMap

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    interface IXmlNode;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode ABI::Windows::Data::Xml::Dom::IXmlNode

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    interface IXmlNodeList;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList ABI::Windows::Data::Xml::Dom::IXmlNodeList

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    interface IXmlNodeSelector;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector ABI::Windows::Data::Xml::Dom::IXmlNodeSelector

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializer_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    interface IXmlNodeSerializer;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializer ABI::Windows::Data::Xml::Dom::IXmlNodeSerializer

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstruction_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstruction_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    interface IXmlProcessingInstruction;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstruction ABI::Windows::Data::Xml::Dom::IXmlProcessingInstruction

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstruction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlText_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlText_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    interface IXmlText;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlText ABI::Windows::Data::Xml::Dom::IXmlText

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlText_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    class XmlDocument;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument_USE
#define DEF___FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f858e239-1896-5982-8495-143168478eb8"))
IAsyncOperation<ABI::Windows::Data::Xml::Dom::XmlDocument*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Data::Xml::Dom::XmlDocument*, ABI::Windows::Data::Xml::Dom::IXmlDocument*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Data.Xml.Dom.XmlDocument>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Data::Xml::Dom::XmlDocument*> __FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument_t;
#define __FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CData__CXml__CDom__CXmlDocument_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CData__CXml__CDom__CXmlDocument_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5eef7817-93dd-5c0b-9e5a-eb490408f3a9"))
IAsyncOperationCompletedHandler<ABI::Windows::Data::Xml::Dom::XmlDocument*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Data::Xml::Dom::XmlDocument*, ABI::Windows::Data::Xml::Dom::IXmlDocument*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Data.Xml.Dom.XmlDocument>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Data::Xml::Dom::XmlDocument*> __FIAsyncOperationCompletedHandler_1_Windows__CData__CXml__CDom__CXmlDocument_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CData__CXml__CDom__CXmlDocument ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CData__CXml__CDom__CXmlDocument_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CData__CXml__CDom__CXmlDocument_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CData__CXml__CDom__CIXmlNode_USE
#define DEF___FIIterator_1_Windows__CData__CXml__CDom__CIXmlNode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3833a35e-2c61-56bd-b093-3694165f8898"))
IIterator<ABI::Windows::Data::Xml::Dom::IXmlNode*> : IIterator_impl<ABI::Windows::Data::Xml::Dom::IXmlNode*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Data.Xml.Dom.IXmlNode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Data::Xml::Dom::IXmlNode*> __FIIterator_1_Windows__CData__CXml__CDom__CIXmlNode_t;
#define __FIIterator_1_Windows__CData__CXml__CDom__CIXmlNode ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CData__CXml__CDom__CIXmlNode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CData__CXml__CDom__CIXmlNode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CData__CXml__CDom__CIXmlNode_USE
#define DEF___FIIterable_1_Windows__CData__CXml__CDom__CIXmlNode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f1146ffc-8c92-56e8-93f1-711f86722633"))
IIterable<ABI::Windows::Data::Xml::Dom::IXmlNode*> : IIterable_impl<ABI::Windows::Data::Xml::Dom::IXmlNode*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Data.Xml.Dom.IXmlNode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Data::Xml::Dom::IXmlNode*> __FIIterable_1_Windows__CData__CXml__CDom__CIXmlNode_t;
#define __FIIterable_1_Windows__CData__CXml__CDom__CIXmlNode ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CData__CXml__CDom__CIXmlNode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CData__CXml__CDom__CIXmlNode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CData__CXml__CDom__CIXmlNode_USE
#define DEF___FIVectorView_1_Windows__CData__CXml__CDom__CIXmlNode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("139d959e-e7b5-5cb6-a596-4b544478da9b"))
IVectorView<ABI::Windows::Data::Xml::Dom::IXmlNode*> : IVectorView_impl<ABI::Windows::Data::Xml::Dom::IXmlNode*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Data.Xml.Dom.IXmlNode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Data::Xml::Dom::IXmlNode*> __FIVectorView_1_Windows__CData__CXml__CDom__CIXmlNode_t;
#define __FIVectorView_1_Windows__CData__CXml__CDom__CIXmlNode ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CData__CXml__CDom__CIXmlNode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CData__CXml__CDom__CIXmlNode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IAsyncAction;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIAsyncAction ABI::Windows::Foundation::IAsyncAction

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageFile;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageFile ABI::Windows::Storage::IStorageFile

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IBuffer;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIBuffer ABI::Windows::Storage::Streams::IBuffer

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    typedef enum NodeType : int NodeType;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    class XmlAttribute;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    class XmlCDataSection;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    class XmlComment;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    class XmlDocumentFragment;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    class XmlDocumentType;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    class XmlDomImplementation;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    class XmlElement;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    class XmlEntityReference;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    class XmlLoadSettings;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    class XmlNamedNodeMap;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    class XmlNodeList;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    class XmlProcessingInstruction;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    class XmlText;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Data.Xml.Dom.NodeType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    enum NodeType : int
                    {
                        NodeType_Invalid = 0,
                        NodeType_ElementNode = 1,
                        NodeType_AttributeNode = 2,
                        NodeType_TextNode = 3,
                        NodeType_DataSectionNode = 4,
                        NodeType_EntityReferenceNode = 5,
                        NodeType_EntityNode = 6,
                        NodeType_ProcessingInstructionNode = 7,
                        NodeType_CommentNode = 8,
                        NodeType_DocumentNode = 9,
                        NodeType_DocumentTypeNode = 10,
                        NodeType_DocumentFragmentNode = 11,
                        NodeType_NotationNode = 12,
                    };
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IDtdEntity
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Xml.Dom.DtdEntity
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Data.Xml.Dom.IXmlNode
 *     Windows.Data.Xml.Dom.IXmlNodeSelector
 *     Windows.Data.Xml.Dom.IXmlNodeSerializer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIDtdEntity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIDtdEntity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IDtdEntity[] = L"Windows.Data.Xml.Dom.IDtdEntity";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    MIDL_INTERFACE("6a0b5ffc-63b4-480f-9e6a-8a92816aade4")
                    IDtdEntity : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_PublicId(
                            IInspectable** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SystemId(
                            IInspectable** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NotationName(
                            IInspectable** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDtdEntity = __uuidof(IDtdEntity);
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIDtdEntity;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIDtdEntity_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IDtdNotation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Xml.Dom.DtdNotation
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Data.Xml.Dom.IXmlNode
 *     Windows.Data.Xml.Dom.IXmlNodeSelector
 *     Windows.Data.Xml.Dom.IXmlNodeSerializer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIDtdNotation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIDtdNotation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IDtdNotation[] = L"Windows.Data.Xml.Dom.IDtdNotation";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    MIDL_INTERFACE("8cb4e04d-6d46-4edb-ab73-df83c51ad397")
                    IDtdNotation : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_PublicId(
                            IInspectable** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SystemId(
                            IInspectable** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDtdNotation = __uuidof(IDtdNotation);
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIDtdNotation;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIDtdNotation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlAttribute
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Xml.Dom.XmlAttribute
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Data.Xml.Dom.IXmlNode
 *     Windows.Data.Xml.Dom.IXmlNodeSelector
 *     Windows.Data.Xml.Dom.IXmlNodeSerializer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlAttribute[] = L"Windows.Data.Xml.Dom.IXmlAttribute";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    MIDL_INTERFACE("ac144aa4-b4f1-4db6-b206-8a22c308db0a")
                    IXmlAttribute : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Name(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Specified(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Value(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Value(
                            HSTRING value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXmlAttribute = __uuidof(IXmlAttribute);
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlCDataSection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Xml.Dom.XmlCDataSection
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Data.Xml.Dom.IXmlText
 *     Windows.Data.Xml.Dom.IXmlCharacterData
 *     Windows.Data.Xml.Dom.IXmlNode
 *     Windows.Data.Xml.Dom.IXmlNodeSelector
 *     Windows.Data.Xml.Dom.IXmlNodeSerializer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlCDataSection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlCDataSection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlCDataSection[] = L"Windows.Data.Xml.Dom.IXmlCDataSection";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    MIDL_INTERFACE("4d04b46f-c8bd-45b4-8899-0400d7c2c60f")
                    IXmlCDataSection : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IXmlCDataSection = __uuidof(IXmlCDataSection);
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlCDataSection;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlCDataSection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlCharacterData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Data.Xml.Dom.IXmlNode
 *     Windows.Data.Xml.Dom.IXmlNodeSelector
 *     Windows.Data.Xml.Dom.IXmlNodeSerializer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlCharacterData[] = L"Windows.Data.Xml.Dom.IXmlCharacterData";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    MIDL_INTERFACE("132e42ab-4e36-4df6-b1c8-0ce62fd88b26")
                    IXmlCharacterData : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Data(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Data(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Length(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SubstringData(
                            UINT32 offset,
                            UINT32 count,
                            HSTRING* data
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE AppendData(
                            HSTRING data
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE InsertData(
                            UINT32 offset,
                            HSTRING data
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE DeleteData(
                            UINT32 offset,
                            UINT32 count
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReplaceData(
                            UINT32 offset,
                            UINT32 count,
                            HSTRING data
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXmlCharacterData = __uuidof(IXmlCharacterData);
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlComment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Xml.Dom.XmlComment
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Data.Xml.Dom.IXmlCharacterData
 *     Windows.Data.Xml.Dom.IXmlNode
 *     Windows.Data.Xml.Dom.IXmlNodeSelector
 *     Windows.Data.Xml.Dom.IXmlNodeSerializer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlComment_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlComment_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlComment[] = L"Windows.Data.Xml.Dom.IXmlComment";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    MIDL_INTERFACE("bca474d5-b61f-4611-9cac-2e92e3476d47")
                    IXmlComment : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IXmlComment = __uuidof(IXmlComment);
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlComment;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlComment_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlDocument
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Xml.Dom.XmlDocument
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Data.Xml.Dom.IXmlNode
 *     Windows.Data.Xml.Dom.IXmlNodeSelector
 *     Windows.Data.Xml.Dom.IXmlNodeSerializer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlDocument[] = L"Windows.Data.Xml.Dom.IXmlDocument";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    MIDL_INTERFACE("f7f3a506-1e87-42d6-bcfb-b8c809fa5494")
                    IXmlDocument : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Doctype(
                            ABI::Windows::Data::Xml::Dom::IXmlDocumentType** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Implementation(
                            ABI::Windows::Data::Xml::Dom::IXmlDomImplementation** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DocumentElement(
                            ABI::Windows::Data::Xml::Dom::IXmlElement** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateElement(
                            HSTRING tagName,
                            ABI::Windows::Data::Xml::Dom::IXmlElement** newElement
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateDocumentFragment(
                            ABI::Windows::Data::Xml::Dom::IXmlDocumentFragment** newDocumentFragment
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateTextNode(
                            HSTRING data,
                            ABI::Windows::Data::Xml::Dom::IXmlText** newTextNode
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateComment(
                            HSTRING data,
                            ABI::Windows::Data::Xml::Dom::IXmlComment** newComment
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateProcessingInstruction(
                            HSTRING target,
                            HSTRING data,
                            ABI::Windows::Data::Xml::Dom::IXmlProcessingInstruction** newProcessingInstruction
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateAttribute(
                            HSTRING name,
                            ABI::Windows::Data::Xml::Dom::IXmlAttribute** newAttribute
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateEntityReference(
                            HSTRING name,
                            ABI::Windows::Data::Xml::Dom::IXmlEntityReference** newEntityReference
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetElementsByTagName(
                            HSTRING tagName,
                            ABI::Windows::Data::Xml::Dom::IXmlNodeList** elements
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateCDataSection(
                            HSTRING data,
                            ABI::Windows::Data::Xml::Dom::IXmlCDataSection** newCDataSection
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DocumentUri(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateAttributeNS(
                            IInspectable* namespaceUri,
                            HSTRING qualifiedName,
                            ABI::Windows::Data::Xml::Dom::IXmlAttribute** newAttribute
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateElementNS(
                            IInspectable* namespaceUri,
                            HSTRING qualifiedName,
                            ABI::Windows::Data::Xml::Dom::IXmlElement** newElement
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetElementById(
                            HSTRING elementId,
                            ABI::Windows::Data::Xml::Dom::IXmlElement** element
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ImportNode(
                            ABI::Windows::Data::Xml::Dom::IXmlNode* node,
                            boolean deep,
                            ABI::Windows::Data::Xml::Dom::IXmlNode** newNode
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXmlDocument = __uuidof(IXmlDocument);
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlDocumentFragment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Xml.Dom.XmlDocumentFragment
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Data.Xml.Dom.IXmlNode
 *     Windows.Data.Xml.Dom.IXmlNodeSelector
 *     Windows.Data.Xml.Dom.IXmlNodeSerializer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentFragment_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentFragment_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlDocumentFragment[] = L"Windows.Data.Xml.Dom.IXmlDocumentFragment";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    MIDL_INTERFACE("e2ea6a96-0c21-44a5-8bc9-9e4a262708ec")
                    IXmlDocumentFragment : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IXmlDocumentFragment = __uuidof(IXmlDocumentFragment);
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentFragment;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentFragment_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlDocumentIO
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Xml.Dom.XmlDocument
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlDocumentIO[] = L"Windows.Data.Xml.Dom.IXmlDocumentIO";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    MIDL_INTERFACE("6cd0e74e-ee65-4489-9ebf-ca43e87ba637")
                    IXmlDocumentIO : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE LoadXml(
                            HSTRING xml
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE LoadXmlWithSettings(
                            HSTRING xml,
                            ABI::Windows::Data::Xml::Dom::IXmlLoadSettings* loadSettings
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SaveToFileAsync(
                            ABI::Windows::Storage::IStorageFile* file,
                            ABI::Windows::Foundation::IAsyncAction** asyncInfo
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXmlDocumentIO = __uuidof(IXmlDocumentIO);
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlDocumentIO2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Xml.Dom.XmlDocument
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlDocumentIO2[] = L"Windows.Data.Xml.Dom.IXmlDocumentIO2";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    MIDL_INTERFACE("5d034661-7bd8-4ad5-9ebf-81e6347263b1")
                    IXmlDocumentIO2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE LoadXmlFromBuffer(
                            ABI::Windows::Storage::Streams::IBuffer* buffer
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE LoadXmlFromBufferWithSettings(
                            ABI::Windows::Storage::Streams::IBuffer* buffer,
                            ABI::Windows::Data::Xml::Dom::IXmlLoadSettings* loadSettings
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXmlDocumentIO2 = __uuidof(IXmlDocumentIO2);
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO2;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlDocumentStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Xml.Dom.XmlDocument
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlDocumentStatics[] = L"Windows.Data.Xml.Dom.IXmlDocumentStatics";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    MIDL_INTERFACE("5543d254-d757-4b79-9539-232b18f50bf1")
                    IXmlDocumentStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE LoadFromUriAsync(
                            ABI::Windows::Foundation::IUriRuntimeClass* uri,
                            __FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument** asyncInfo
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE LoadFromUriWithSettingsAsync(
                            ABI::Windows::Foundation::IUriRuntimeClass* uri,
                            ABI::Windows::Data::Xml::Dom::IXmlLoadSettings* loadSettings,
                            __FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument** asyncInfo
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE LoadFromFileAsync(
                            ABI::Windows::Storage::IStorageFile* file,
                            __FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument** asyncInfo
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE LoadFromFileWithSettingsAsync(
                            ABI::Windows::Storage::IStorageFile* file,
                            ABI::Windows::Data::Xml::Dom::IXmlLoadSettings* loadSettings,
                            __FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument** asyncInfo
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXmlDocumentStatics = __uuidof(IXmlDocumentStatics);
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlDocumentType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Xml.Dom.XmlDocumentType
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Data.Xml.Dom.IXmlNode
 *     Windows.Data.Xml.Dom.IXmlNodeSelector
 *     Windows.Data.Xml.Dom.IXmlNodeSerializer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentType_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentType_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlDocumentType[] = L"Windows.Data.Xml.Dom.IXmlDocumentType";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    MIDL_INTERFACE("f7342425-9781-4964-8e94-9b1c6dfc9bc7")
                    IXmlDocumentType : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Name(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Entities(
                            ABI::Windows::Data::Xml::Dom::IXmlNamedNodeMap** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Notations(
                            ABI::Windows::Data::Xml::Dom::IXmlNamedNodeMap** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXmlDocumentType = __uuidof(IXmlDocumentType);
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentType;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentType_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlDomImplementation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Xml.Dom.XmlDomImplementation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlDomImplementation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDomImplementation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlDomImplementation[] = L"Windows.Data.Xml.Dom.IXmlDomImplementation";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    MIDL_INTERFACE("6de58132-f11d-4fbb-8cc6-583cba93112f")
                    IXmlDomImplementation : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE HasFeature(
                            HSTRING feature,
                            IInspectable* version,
                            boolean* featureSupported
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXmlDomImplementation = __uuidof(IXmlDomImplementation);
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlDomImplementation;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlDomImplementation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlElement
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Xml.Dom.XmlElement
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Data.Xml.Dom.IXmlNode
 *     Windows.Data.Xml.Dom.IXmlNodeSelector
 *     Windows.Data.Xml.Dom.IXmlNodeSerializer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlElement_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlElement_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlElement[] = L"Windows.Data.Xml.Dom.IXmlElement";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    MIDL_INTERFACE("2dfb8a1f-6b10-4ef8-9f83-efcce8faec37")
                    IXmlElement : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_TagName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetAttribute(
                            HSTRING attributeName,
                            HSTRING* attributeValue
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetAttribute(
                            HSTRING attributeName,
                            HSTRING attributeValue
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RemoveAttribute(
                            HSTRING attributeName
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetAttributeNode(
                            HSTRING attributeName,
                            ABI::Windows::Data::Xml::Dom::IXmlAttribute** attributeNode
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetAttributeNode(
                            ABI::Windows::Data::Xml::Dom::IXmlAttribute* newAttribute,
                            ABI::Windows::Data::Xml::Dom::IXmlAttribute** previousAttribute
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RemoveAttributeNode(
                            ABI::Windows::Data::Xml::Dom::IXmlAttribute* attributeNode,
                            ABI::Windows::Data::Xml::Dom::IXmlAttribute** removedAttribute
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetElementsByTagName(
                            HSTRING tagName,
                            ABI::Windows::Data::Xml::Dom::IXmlNodeList** elements
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetAttributeNS(
                            IInspectable* namespaceUri,
                            HSTRING qualifiedName,
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetAttributeNS(
                            IInspectable* namespaceUri,
                            HSTRING localName,
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RemoveAttributeNS(
                            IInspectable* namespaceUri,
                            HSTRING localName
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetAttributeNodeNS(
                            ABI::Windows::Data::Xml::Dom::IXmlAttribute* newAttribute,
                            ABI::Windows::Data::Xml::Dom::IXmlAttribute** previousAttribute
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetAttributeNodeNS(
                            IInspectable* namespaceUri,
                            HSTRING localName,
                            ABI::Windows::Data::Xml::Dom::IXmlAttribute** previousAttribute
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXmlElement = __uuidof(IXmlElement);
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlElement;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlElement_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlEntityReference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Xml.Dom.XmlEntityReference
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Data.Xml.Dom.IXmlNode
 *     Windows.Data.Xml.Dom.IXmlNodeSelector
 *     Windows.Data.Xml.Dom.IXmlNodeSerializer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlEntityReference_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlEntityReference_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlEntityReference[] = L"Windows.Data.Xml.Dom.IXmlEntityReference";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    MIDL_INTERFACE("2e2f47bc-c3d0-4ccf-bb86-0ab8c36a61cf")
                    IXmlEntityReference : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IXmlEntityReference = __uuidof(IXmlEntityReference);
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlEntityReference;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlEntityReference_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlLoadSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Xml.Dom.XmlLoadSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlLoadSettings[] = L"Windows.Data.Xml.Dom.IXmlLoadSettings";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    MIDL_INTERFACE("58aa07a8-fed6-46f7-b4c5-fb1ba72108d6")
                    IXmlLoadSettings : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_MaxElementDepth(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_MaxElementDepth(
                            UINT32 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ProhibitDtd(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ProhibitDtd(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ResolveExternals(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ResolveExternals(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ValidateOnParse(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ValidateOnParse(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ElementContentWhiteSpace(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ElementContentWhiteSpace(
                            boolean value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXmlLoadSettings = __uuidof(IXmlLoadSettings);
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlNamedNodeMap
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Xml.Dom.XmlNamedNodeMap
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.Collections.IVectorView`1<Windows.Data.Xml.Dom.IXmlNode>
 *     Windows.Foundation.Collections.IIterable`1<Windows.Data.Xml.Dom.IXmlNode>
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlNamedNodeMap[] = L"Windows.Data.Xml.Dom.IXmlNamedNodeMap";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    MIDL_INTERFACE("b3a69eb0-aab0-4b82-a6fa-b1453f7c021b")
                    IXmlNamedNodeMap : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Length(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Item(
                            UINT32 index,
                            ABI::Windows::Data::Xml::Dom::IXmlNode** node
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetNamedItem(
                            HSTRING name,
                            ABI::Windows::Data::Xml::Dom::IXmlNode** node
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetNamedItem(
                            ABI::Windows::Data::Xml::Dom::IXmlNode* node,
                            ABI::Windows::Data::Xml::Dom::IXmlNode** previousNode
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RemoveNamedItem(
                            HSTRING name,
                            ABI::Windows::Data::Xml::Dom::IXmlNode** previousNode
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetNamedItemNS(
                            IInspectable* namespaceUri,
                            HSTRING name,
                            ABI::Windows::Data::Xml::Dom::IXmlNode** node
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RemoveNamedItemNS(
                            IInspectable* namespaceUri,
                            HSTRING name,
                            ABI::Windows::Data::Xml::Dom::IXmlNode** previousNode
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetNamedItemNS(
                            ABI::Windows::Data::Xml::Dom::IXmlNode* node,
                            ABI::Windows::Data::Xml::Dom::IXmlNode** previousNode
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXmlNamedNodeMap = __uuidof(IXmlNamedNodeMap);
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Data.Xml.Dom.IXmlNodeSelector
 *     Windows.Data.Xml.Dom.IXmlNodeSerializer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlNode[] = L"Windows.Data.Xml.Dom.IXmlNode";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    MIDL_INTERFACE("1c741d59-2122-47d5-a856-83f3d4214875")
                    IXmlNode : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_NodeValue(
                            IInspectable** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_NodeValue(
                            IInspectable* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NodeType(
                            ABI::Windows::Data::Xml::Dom::NodeType* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NodeName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ParentNode(
                            ABI::Windows::Data::Xml::Dom::IXmlNode** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ChildNodes(
                            ABI::Windows::Data::Xml::Dom::IXmlNodeList** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FirstChild(
                            ABI::Windows::Data::Xml::Dom::IXmlNode** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_LastChild(
                            ABI::Windows::Data::Xml::Dom::IXmlNode** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PreviousSibling(
                            ABI::Windows::Data::Xml::Dom::IXmlNode** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NextSibling(
                            ABI::Windows::Data::Xml::Dom::IXmlNode** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Attributes(
                            ABI::Windows::Data::Xml::Dom::IXmlNamedNodeMap** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE HasChildNodes(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_OwnerDocument(
                            ABI::Windows::Data::Xml::Dom::IXmlDocument** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE InsertBefore(
                            ABI::Windows::Data::Xml::Dom::IXmlNode* newChild,
                            ABI::Windows::Data::Xml::Dom::IXmlNode* referenceChild,
                            ABI::Windows::Data::Xml::Dom::IXmlNode** insertedChild
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReplaceChild(
                            ABI::Windows::Data::Xml::Dom::IXmlNode* newChild,
                            ABI::Windows::Data::Xml::Dom::IXmlNode* referenceChild,
                            ABI::Windows::Data::Xml::Dom::IXmlNode** previousChild
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RemoveChild(
                            ABI::Windows::Data::Xml::Dom::IXmlNode* childNode,
                            ABI::Windows::Data::Xml::Dom::IXmlNode** removedChild
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE AppendChild(
                            ABI::Windows::Data::Xml::Dom::IXmlNode* newChild,
                            ABI::Windows::Data::Xml::Dom::IXmlNode** appendedChild
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CloneNode(
                            boolean deep,
                            ABI::Windows::Data::Xml::Dom::IXmlNode** newNode
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NamespaceUri(
                            IInspectable** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_LocalName(
                            IInspectable** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Prefix(
                            IInspectable** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Normalize(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Prefix(
                            IInspectable* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXmlNode = __uuidof(IXmlNode);
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlNode;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlNodeList
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Xml.Dom.XmlNodeList
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.Collections.IVectorView`1<Windows.Data.Xml.Dom.IXmlNode>
 *     Windows.Foundation.Collections.IIterable`1<Windows.Data.Xml.Dom.IXmlNode>
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlNodeList[] = L"Windows.Data.Xml.Dom.IXmlNodeList";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    MIDL_INTERFACE("8c60ad77-83a4-4ec1-9c54-7ba429e13da6")
                    IXmlNodeList : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Length(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Item(
                            UINT32 index,
                            ABI::Windows::Data::Xml::Dom::IXmlNode** node
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXmlNodeList = __uuidof(IXmlNodeList);
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlNodeSelector
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlNodeSelector[] = L"Windows.Data.Xml.Dom.IXmlNodeSelector";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    MIDL_INTERFACE("63dbba8b-d0db-4fe1-b745-f9433afdc25b")
                    IXmlNodeSelector : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE SelectSingleNode(
                            HSTRING xpath,
                            ABI::Windows::Data::Xml::Dom::IXmlNode** node
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SelectNodes(
                            HSTRING xpath,
                            ABI::Windows::Data::Xml::Dom::IXmlNodeList** nodelist
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SelectSingleNodeNS(
                            HSTRING xpath,
                            IInspectable* namespaces,
                            ABI::Windows::Data::Xml::Dom::IXmlNode** node
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SelectNodesNS(
                            HSTRING xpath,
                            IInspectable* namespaces,
                            ABI::Windows::Data::Xml::Dom::IXmlNodeList** nodelist
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXmlNodeSelector = __uuidof(IXmlNodeSelector);
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlNodeSerializer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlNodeSerializer[] = L"Windows.Data.Xml.Dom.IXmlNodeSerializer";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    MIDL_INTERFACE("5cc5b382-e6dd-4991-abef-06d8d2e7bd0c")
                    IXmlNodeSerializer : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetXml(
                            HSTRING* outerXml
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_InnerText(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_InnerText(
                            HSTRING value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXmlNodeSerializer = __uuidof(IXmlNodeSerializer);
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializer;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlProcessingInstruction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Xml.Dom.XmlProcessingInstruction
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Data.Xml.Dom.IXmlNode
 *     Windows.Data.Xml.Dom.IXmlNodeSelector
 *     Windows.Data.Xml.Dom.IXmlNodeSerializer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstruction_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstruction_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlProcessingInstruction[] = L"Windows.Data.Xml.Dom.IXmlProcessingInstruction";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    MIDL_INTERFACE("2707fd1e-1e92-4ece-b6f4-26f069078ddc")
                    IXmlProcessingInstruction : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Target(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Data(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Data(
                            HSTRING value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXmlProcessingInstruction = __uuidof(IXmlProcessingInstruction);
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstruction;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstruction_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlText
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Data.Xml.Dom.IXmlCharacterData
 *     Windows.Data.Xml.Dom.IXmlNode
 *     Windows.Data.Xml.Dom.IXmlNodeSelector
 *     Windows.Data.Xml.Dom.IXmlNodeSerializer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlText_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlText_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlText[] = L"Windows.Data.Xml.Dom.IXmlText";
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    MIDL_INTERFACE("f931a4cb-308d-4760-a1d5-43b67450ac7e")
                    IXmlText : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE SplitText(
                            UINT32 offset,
                            ABI::Windows::Data::Xml::Dom::IXmlText** secondPart
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXmlText = __uuidof(IXmlText);
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlText;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlText_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Xml.Dom.DtdEntity
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Xml.Dom.IDtdEntity ** Default Interface **
 *    Windows.Data.Xml.Dom.IXmlNode
 *    Windows.Data.Xml.Dom.IXmlNodeSerializer
 *    Windows.Data.Xml.Dom.IXmlNodeSelector
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Xml_Dom_DtdEntity_DEFINED
#define RUNTIMECLASS_Windows_Data_Xml_Dom_DtdEntity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Xml_Dom_DtdEntity[] = L"Windows.Data.Xml.Dom.DtdEntity";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Xml.Dom.DtdNotation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Xml.Dom.IDtdNotation ** Default Interface **
 *    Windows.Data.Xml.Dom.IXmlNode
 *    Windows.Data.Xml.Dom.IXmlNodeSerializer
 *    Windows.Data.Xml.Dom.IXmlNodeSelector
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Xml_Dom_DtdNotation_DEFINED
#define RUNTIMECLASS_Windows_Data_Xml_Dom_DtdNotation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Xml_Dom_DtdNotation[] = L"Windows.Data.Xml.Dom.DtdNotation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Xml.Dom.XmlAttribute
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Xml.Dom.IXmlAttribute ** Default Interface **
 *    Windows.Data.Xml.Dom.IXmlNode
 *    Windows.Data.Xml.Dom.IXmlNodeSerializer
 *    Windows.Data.Xml.Dom.IXmlNodeSelector
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Xml_Dom_XmlAttribute_DEFINED
#define RUNTIMECLASS_Windows_Data_Xml_Dom_XmlAttribute_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Xml_Dom_XmlAttribute[] = L"Windows.Data.Xml.Dom.XmlAttribute";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Xml.Dom.XmlCDataSection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Xml.Dom.IXmlCDataSection ** Default Interface **
 *    Windows.Data.Xml.Dom.IXmlText
 *    Windows.Data.Xml.Dom.IXmlCharacterData
 *    Windows.Data.Xml.Dom.IXmlNode
 *    Windows.Data.Xml.Dom.IXmlNodeSerializer
 *    Windows.Data.Xml.Dom.IXmlNodeSelector
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Xml_Dom_XmlCDataSection_DEFINED
#define RUNTIMECLASS_Windows_Data_Xml_Dom_XmlCDataSection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Xml_Dom_XmlCDataSection[] = L"Windows.Data.Xml.Dom.XmlCDataSection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Xml.Dom.XmlComment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Xml.Dom.IXmlComment ** Default Interface **
 *    Windows.Data.Xml.Dom.IXmlCharacterData
 *    Windows.Data.Xml.Dom.IXmlNode
 *    Windows.Data.Xml.Dom.IXmlNodeSerializer
 *    Windows.Data.Xml.Dom.IXmlNodeSelector
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Xml_Dom_XmlComment_DEFINED
#define RUNTIMECLASS_Windows_Data_Xml_Dom_XmlComment_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Xml_Dom_XmlComment[] = L"Windows.Data.Xml.Dom.XmlComment";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Xml.Dom.XmlDocument
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Data.Xml.Dom.IXmlDocumentStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Data.Xml.Dom.IXmlDocument ** Default Interface **
 *    Windows.Data.Xml.Dom.IXmlNode
 *    Windows.Data.Xml.Dom.IXmlNodeSerializer
 *    Windows.Data.Xml.Dom.IXmlNodeSelector
 *    Windows.Data.Xml.Dom.IXmlDocumentIO
 *    Windows.Data.Xml.Dom.IXmlDocumentIO2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Xml_Dom_XmlDocument_DEFINED
#define RUNTIMECLASS_Windows_Data_Xml_Dom_XmlDocument_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Xml_Dom_XmlDocument[] = L"Windows.Data.Xml.Dom.XmlDocument";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Xml.Dom.XmlDocumentFragment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Xml.Dom.IXmlDocumentFragment ** Default Interface **
 *    Windows.Data.Xml.Dom.IXmlNode
 *    Windows.Data.Xml.Dom.IXmlNodeSerializer
 *    Windows.Data.Xml.Dom.IXmlNodeSelector
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Xml_Dom_XmlDocumentFragment_DEFINED
#define RUNTIMECLASS_Windows_Data_Xml_Dom_XmlDocumentFragment_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Xml_Dom_XmlDocumentFragment[] = L"Windows.Data.Xml.Dom.XmlDocumentFragment";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Xml.Dom.XmlDocumentType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Xml.Dom.IXmlDocumentType ** Default Interface **
 *    Windows.Data.Xml.Dom.IXmlNode
 *    Windows.Data.Xml.Dom.IXmlNodeSerializer
 *    Windows.Data.Xml.Dom.IXmlNodeSelector
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Xml_Dom_XmlDocumentType_DEFINED
#define RUNTIMECLASS_Windows_Data_Xml_Dom_XmlDocumentType_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Xml_Dom_XmlDocumentType[] = L"Windows.Data.Xml.Dom.XmlDocumentType";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Xml.Dom.XmlDomImplementation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Xml.Dom.IXmlDomImplementation ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Xml_Dom_XmlDomImplementation_DEFINED
#define RUNTIMECLASS_Windows_Data_Xml_Dom_XmlDomImplementation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Xml_Dom_XmlDomImplementation[] = L"Windows.Data.Xml.Dom.XmlDomImplementation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Xml.Dom.XmlElement
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Xml.Dom.IXmlElement ** Default Interface **
 *    Windows.Data.Xml.Dom.IXmlNode
 *    Windows.Data.Xml.Dom.IXmlNodeSerializer
 *    Windows.Data.Xml.Dom.IXmlNodeSelector
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Xml_Dom_XmlElement_DEFINED
#define RUNTIMECLASS_Windows_Data_Xml_Dom_XmlElement_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Xml_Dom_XmlElement[] = L"Windows.Data.Xml.Dom.XmlElement";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Xml.Dom.XmlEntityReference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Xml.Dom.IXmlEntityReference ** Default Interface **
 *    Windows.Data.Xml.Dom.IXmlNode
 *    Windows.Data.Xml.Dom.IXmlNodeSerializer
 *    Windows.Data.Xml.Dom.IXmlNodeSelector
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Xml_Dom_XmlEntityReference_DEFINED
#define RUNTIMECLASS_Windows_Data_Xml_Dom_XmlEntityReference_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Xml_Dom_XmlEntityReference[] = L"Windows.Data.Xml.Dom.XmlEntityReference";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Xml.Dom.XmlLoadSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Data.Xml.Dom.IXmlLoadSettings ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Xml_Dom_XmlLoadSettings_DEFINED
#define RUNTIMECLASS_Windows_Data_Xml_Dom_XmlLoadSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Xml_Dom_XmlLoadSettings[] = L"Windows.Data.Xml.Dom.XmlLoadSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Xml.Dom.XmlNamedNodeMap
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Xml.Dom.IXmlNamedNodeMap ** Default Interface **
 *    Windows.Foundation.Collections.IVectorView`1<Windows.Data.Xml.Dom.IXmlNode>
 *    Windows.Foundation.Collections.IIterable`1<Windows.Data.Xml.Dom.IXmlNode>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Xml_Dom_XmlNamedNodeMap_DEFINED
#define RUNTIMECLASS_Windows_Data_Xml_Dom_XmlNamedNodeMap_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Xml_Dom_XmlNamedNodeMap[] = L"Windows.Data.Xml.Dom.XmlNamedNodeMap";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Xml.Dom.XmlNodeList
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Xml.Dom.IXmlNodeList ** Default Interface **
 *    Windows.Foundation.Collections.IVectorView`1<Windows.Data.Xml.Dom.IXmlNode>
 *    Windows.Foundation.Collections.IIterable`1<Windows.Data.Xml.Dom.IXmlNode>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Xml_Dom_XmlNodeList_DEFINED
#define RUNTIMECLASS_Windows_Data_Xml_Dom_XmlNodeList_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Xml_Dom_XmlNodeList[] = L"Windows.Data.Xml.Dom.XmlNodeList";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Xml.Dom.XmlProcessingInstruction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Xml.Dom.IXmlProcessingInstruction ** Default Interface **
 *    Windows.Data.Xml.Dom.IXmlNode
 *    Windows.Data.Xml.Dom.IXmlNodeSerializer
 *    Windows.Data.Xml.Dom.IXmlNodeSelector
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Xml_Dom_XmlProcessingInstruction_DEFINED
#define RUNTIMECLASS_Windows_Data_Xml_Dom_XmlProcessingInstruction_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Xml_Dom_XmlProcessingInstruction[] = L"Windows.Data.Xml.Dom.XmlProcessingInstruction";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Xml.Dom.XmlText
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Xml.Dom.IXmlText ** Default Interface **
 *    Windows.Data.Xml.Dom.IXmlCharacterData
 *    Windows.Data.Xml.Dom.IXmlNode
 *    Windows.Data.Xml.Dom.IXmlNodeSerializer
 *    Windows.Data.Xml.Dom.IXmlNodeSelector
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Xml_Dom_XmlText_DEFINED
#define RUNTIMECLASS_Windows_Data_Xml_Dom_XmlText_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Xml_Dom_XmlText[] = L"Windows.Data.Xml.Dom.XmlText";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIDtdEntity_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIDtdEntity_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CXml_CDom_CIDtdEntity __x_ABI_CWindows_CData_CXml_CDom_CIDtdEntity;

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIDtdEntity_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIDtdNotation_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIDtdNotation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CXml_CDom_CIDtdNotation __x_ABI_CWindows_CData_CXml_CDom_CIDtdNotation;

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIDtdNotation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute __x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute;

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlCDataSection_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlCDataSection_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlCDataSection __x_ABI_CWindows_CData_CXml_CDom_CIXmlCDataSection;

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlCDataSection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData __x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData;

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlComment_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlComment_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlComment __x_ABI_CWindows_CData_CXml_CDom_CIXmlComment;

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlComment_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument;

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentFragment_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentFragment_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentFragment __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentFragment;

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentFragment_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO;

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO2_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO2 __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO2;

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics;

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentType_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentType_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentType __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentType;

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentType_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDomImplementation_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDomImplementation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlDomImplementation __x_ABI_CWindows_CData_CXml_CDom_CIXmlDomImplementation;

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDomImplementation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlElement_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlElement_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlElement __x_ABI_CWindows_CData_CXml_CDom_CIXmlElement;

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlElement_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlEntityReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlEntityReference_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlEntityReference __x_ABI_CWindows_CData_CXml_CDom_CIXmlEntityReference;

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlEntityReference_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings __x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings;

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap __x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap;

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode;

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList;

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector;

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializer_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializer __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializer;

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstruction_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstruction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstruction __x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstruction;

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstruction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlText_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlText_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlText __x_ABI_CWindows_CData_CXml_CDom_CIXmlText;

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlText_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CData__CXml__CDom__CXmlDocument __FIAsyncOperationCompletedHandler_1_Windows__CData__CXml__CDom__CXmlDocument;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument __FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument;

typedef struct __FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocumentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CData__CXml__CDom__CXmlDocument* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CData__CXml__CDom__CXmlDocument** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocumentVtbl;

interface __FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocumentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CData__CXml__CDom__CXmlDocument_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CData__CXml__CDom__CXmlDocument_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CData__CXml__CDom__CXmlDocument __FIAsyncOperationCompletedHandler_1_Windows__CData__CXml__CDom__CXmlDocument;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CData__CXml__CDom__CXmlDocument;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CData__CXml__CDom__CXmlDocumentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CData__CXml__CDom__CXmlDocument* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CData__CXml__CDom__CXmlDocument* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CData__CXml__CDom__CXmlDocument* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CData__CXml__CDom__CXmlDocument* This,
        __FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CData__CXml__CDom__CXmlDocumentVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CData__CXml__CDom__CXmlDocument
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CData__CXml__CDom__CXmlDocumentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CData__CXml__CDom__CXmlDocument_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CData__CXml__CDom__CXmlDocument_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CData__CXml__CDom__CXmlDocument_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CData__CXml__CDom__CXmlDocument_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CData__CXml__CDom__CXmlDocument_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CData__CXml__CDom__CIXmlNode_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CData__CXml__CDom__CIXmlNode_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CData__CXml__CDom__CIXmlNode __FIIterator_1_Windows__CData__CXml__CDom__CIXmlNode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CData__CXml__CDom__CIXmlNode;

typedef struct __FIIterator_1_Windows__CData__CXml__CDom__CIXmlNodeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CData__CXml__CDom__CIXmlNode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CData__CXml__CDom__CIXmlNode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CData__CXml__CDom__CIXmlNode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CData__CXml__CDom__CIXmlNode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CData__CXml__CDom__CIXmlNode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CData__CXml__CDom__CIXmlNode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CData__CXml__CDom__CIXmlNode* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CData__CXml__CDom__CIXmlNode* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CData__CXml__CDom__CIXmlNode* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CData__CXml__CDom__CIXmlNode* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CData__CXml__CDom__CIXmlNodeVtbl;

interface __FIIterator_1_Windows__CData__CXml__CDom__CIXmlNode
{
    CONST_VTBL struct __FIIterator_1_Windows__CData__CXml__CDom__CIXmlNodeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CData__CXml__CDom__CIXmlNode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CData__CXml__CDom__CIXmlNode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CData__CXml__CDom__CIXmlNode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CData__CXml__CDom__CIXmlNode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CData__CXml__CDom__CIXmlNode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CData__CXml__CDom__CIXmlNode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CData__CXml__CDom__CIXmlNode_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CData__CXml__CDom__CIXmlNode_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CData__CXml__CDom__CIXmlNode_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CData__CXml__CDom__CIXmlNode_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CData__CXml__CDom__CIXmlNode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CData__CXml__CDom__CIXmlNode_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CData__CXml__CDom__CIXmlNode_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CData__CXml__CDom__CIXmlNode __FIIterable_1_Windows__CData__CXml__CDom__CIXmlNode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CData__CXml__CDom__CIXmlNode;

typedef struct __FIIterable_1_Windows__CData__CXml__CDom__CIXmlNodeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CData__CXml__CDom__CIXmlNode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CData__CXml__CDom__CIXmlNode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CData__CXml__CDom__CIXmlNode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CData__CXml__CDom__CIXmlNode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CData__CXml__CDom__CIXmlNode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CData__CXml__CDom__CIXmlNode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CData__CXml__CDom__CIXmlNode* This,
        __FIIterator_1_Windows__CData__CXml__CDom__CIXmlNode** result);

    END_INTERFACE
} __FIIterable_1_Windows__CData__CXml__CDom__CIXmlNodeVtbl;

interface __FIIterable_1_Windows__CData__CXml__CDom__CIXmlNode
{
    CONST_VTBL struct __FIIterable_1_Windows__CData__CXml__CDom__CIXmlNodeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CData__CXml__CDom__CIXmlNode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CData__CXml__CDom__CIXmlNode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CData__CXml__CDom__CIXmlNode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CData__CXml__CDom__CIXmlNode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CData__CXml__CDom__CIXmlNode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CData__CXml__CDom__CIXmlNode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CData__CXml__CDom__CIXmlNode_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CData__CXml__CDom__CIXmlNode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CData__CXml__CDom__CIXmlNode_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CData__CXml__CDom__CIXmlNode_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CData__CXml__CDom__CIXmlNode __FIVectorView_1_Windows__CData__CXml__CDom__CIXmlNode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CData__CXml__CDom__CIXmlNode;

typedef struct __FIVectorView_1_Windows__CData__CXml__CDom__CIXmlNodeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CData__CXml__CDom__CIXmlNode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CData__CXml__CDom__CIXmlNode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CData__CXml__CDom__CIXmlNode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CData__CXml__CDom__CIXmlNode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CData__CXml__CDom__CIXmlNode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CData__CXml__CDom__CIXmlNode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CData__CXml__CDom__CIXmlNode* This,
        UINT32 index,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CData__CXml__CDom__CIXmlNode* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CData__CXml__CDom__CIXmlNode* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CData__CXml__CDom__CIXmlNode* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CData__CXml__CDom__CIXmlNodeVtbl;

interface __FIVectorView_1_Windows__CData__CXml__CDom__CIXmlNode
{
    CONST_VTBL struct __FIVectorView_1_Windows__CData__CXml__CDom__CIXmlNodeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CData__CXml__CDom__CIXmlNode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CData__CXml__CDom__CIXmlNode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CData__CXml__CDom__CIXmlNode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CData__CXml__CDom__CIXmlNode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CData__CXml__CDom__CIXmlNode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CData__CXml__CDom__CIXmlNode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CData__CXml__CDom__CIXmlNode_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CData__CXml__CDom__CIXmlNode_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CData__CXml__CDom__CIXmlNode_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CData__CXml__CDom__CIXmlNode_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CData__CXml__CDom__CIXmlNode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFile __x_ABI_CWindows_CStorage_CIStorageFile;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CData_CXml_CDom_CNodeType __x_ABI_CWindows_CData_CXml_CDom_CNodeType;

/*
 *
 * Struct Windows.Data.Xml.Dom.NodeType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CData_CXml_CDom_CNodeType
{
    NodeType_Invalid = 0,
    NodeType_ElementNode = 1,
    NodeType_AttributeNode = 2,
    NodeType_TextNode = 3,
    NodeType_DataSectionNode = 4,
    NodeType_EntityReferenceNode = 5,
    NodeType_EntityNode = 6,
    NodeType_ProcessingInstructionNode = 7,
    NodeType_CommentNode = 8,
    NodeType_DocumentNode = 9,
    NodeType_DocumentTypeNode = 10,
    NodeType_DocumentFragmentNode = 11,
    NodeType_NotationNode = 12,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IDtdEntity
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Xml.Dom.DtdEntity
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Data.Xml.Dom.IXmlNode
 *     Windows.Data.Xml.Dom.IXmlNodeSelector
 *     Windows.Data.Xml.Dom.IXmlNodeSerializer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIDtdEntity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIDtdEntity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IDtdEntity[] = L"Windows.Data.Xml.Dom.IDtdEntity";
typedef struct __x_ABI_CWindows_CData_CXml_CDom_CIDtdEntityVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CXml_CDom_CIDtdEntity* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CXml_CDom_CIDtdEntity* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CXml_CDom_CIDtdEntity* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CXml_CDom_CIDtdEntity* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CXml_CDom_CIDtdEntity* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CXml_CDom_CIDtdEntity* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PublicId)(__x_ABI_CWindows_CData_CXml_CDom_CIDtdEntity* This,
        IInspectable** value);
    HRESULT (STDMETHODCALLTYPE* get_SystemId)(__x_ABI_CWindows_CData_CXml_CDom_CIDtdEntity* This,
        IInspectable** value);
    HRESULT (STDMETHODCALLTYPE* get_NotationName)(__x_ABI_CWindows_CData_CXml_CDom_CIDtdEntity* This,
        IInspectable** value);

    END_INTERFACE
} __x_ABI_CWindows_CData_CXml_CDom_CIDtdEntityVtbl;

interface __x_ABI_CWindows_CData_CXml_CDom_CIDtdEntity
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CXml_CDom_CIDtdEntityVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CXml_CDom_CIDtdEntity_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CXml_CDom_CIDtdEntity_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIDtdEntity_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIDtdEntity_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CXml_CDom_CIDtdEntity_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CXml_CDom_CIDtdEntity_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CXml_CDom_CIDtdEntity_get_PublicId(This, value) \
    ((This)->lpVtbl->get_PublicId(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIDtdEntity_get_SystemId(This, value) \
    ((This)->lpVtbl->get_SystemId(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIDtdEntity_get_NotationName(This, value) \
    ((This)->lpVtbl->get_NotationName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIDtdEntity;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIDtdEntity_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IDtdNotation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Xml.Dom.DtdNotation
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Data.Xml.Dom.IXmlNode
 *     Windows.Data.Xml.Dom.IXmlNodeSelector
 *     Windows.Data.Xml.Dom.IXmlNodeSerializer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIDtdNotation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIDtdNotation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IDtdNotation[] = L"Windows.Data.Xml.Dom.IDtdNotation";
typedef struct __x_ABI_CWindows_CData_CXml_CDom_CIDtdNotationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CXml_CDom_CIDtdNotation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CXml_CDom_CIDtdNotation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CXml_CDom_CIDtdNotation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CXml_CDom_CIDtdNotation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CXml_CDom_CIDtdNotation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CXml_CDom_CIDtdNotation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PublicId)(__x_ABI_CWindows_CData_CXml_CDom_CIDtdNotation* This,
        IInspectable** value);
    HRESULT (STDMETHODCALLTYPE* get_SystemId)(__x_ABI_CWindows_CData_CXml_CDom_CIDtdNotation* This,
        IInspectable** value);

    END_INTERFACE
} __x_ABI_CWindows_CData_CXml_CDom_CIDtdNotationVtbl;

interface __x_ABI_CWindows_CData_CXml_CDom_CIDtdNotation
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CXml_CDom_CIDtdNotationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CXml_CDom_CIDtdNotation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CXml_CDom_CIDtdNotation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIDtdNotation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIDtdNotation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CXml_CDom_CIDtdNotation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CXml_CDom_CIDtdNotation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CXml_CDom_CIDtdNotation_get_PublicId(This, value) \
    ((This)->lpVtbl->get_PublicId(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIDtdNotation_get_SystemId(This, value) \
    ((This)->lpVtbl->get_SystemId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIDtdNotation;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIDtdNotation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlAttribute
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Xml.Dom.XmlAttribute
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Data.Xml.Dom.IXmlNode
 *     Windows.Data.Xml.Dom.IXmlNodeSelector
 *     Windows.Data.Xml.Dom.IXmlNodeSerializer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlAttribute[] = L"Windows.Data.Xml.Dom.IXmlAttribute";
typedef struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlAttributeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Specified)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Value)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CData_CXml_CDom_CIXmlAttributeVtbl;

interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlAttributeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute_get_Specified(This, value) \
    ((This)->lpVtbl->get_Specified(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute_put_Value(This, value) \
    ((This)->lpVtbl->put_Value(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlCDataSection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Xml.Dom.XmlCDataSection
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Data.Xml.Dom.IXmlText
 *     Windows.Data.Xml.Dom.IXmlCharacterData
 *     Windows.Data.Xml.Dom.IXmlNode
 *     Windows.Data.Xml.Dom.IXmlNodeSelector
 *     Windows.Data.Xml.Dom.IXmlNodeSerializer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlCDataSection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlCDataSection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlCDataSection[] = L"Windows.Data.Xml.Dom.IXmlCDataSection";
typedef struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlCDataSectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlCDataSection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlCDataSection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlCDataSection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlCDataSection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlCDataSection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlCDataSection* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CData_CXml_CDom_CIXmlCDataSectionVtbl;

interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlCDataSection
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlCDataSectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlCDataSection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlCDataSection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlCDataSection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlCDataSection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlCDataSection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlCDataSection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlCDataSection;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlCDataSection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlCharacterData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Data.Xml.Dom.IXmlNode
 *     Windows.Data.Xml.Dom.IXmlNodeSelector
 *     Windows.Data.Xml.Dom.IXmlNodeSerializer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlCharacterData[] = L"Windows.Data.Xml.Dom.IXmlCharacterData";
typedef struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterDataVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Data)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Data)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Length)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* SubstringData)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData* This,
        UINT32 offset,
        UINT32 count,
        HSTRING* data);
    HRESULT (STDMETHODCALLTYPE* AppendData)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData* This,
        HSTRING data);
    HRESULT (STDMETHODCALLTYPE* InsertData)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData* This,
        UINT32 offset,
        HSTRING data);
    HRESULT (STDMETHODCALLTYPE* DeleteData)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData* This,
        UINT32 offset,
        UINT32 count);
    HRESULT (STDMETHODCALLTYPE* ReplaceData)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData* This,
        UINT32 offset,
        UINT32 count,
        HSTRING data);

    END_INTERFACE
} __x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterDataVtbl;

interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterDataVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData_get_Data(This, value) \
    ((This)->lpVtbl->get_Data(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData_put_Data(This, value) \
    ((This)->lpVtbl->put_Data(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData_get_Length(This, value) \
    ((This)->lpVtbl->get_Length(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData_SubstringData(This, offset, count, data) \
    ((This)->lpVtbl->SubstringData(This, offset, count, data))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData_AppendData(This, data) \
    ((This)->lpVtbl->AppendData(This, data))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData_InsertData(This, offset, data) \
    ((This)->lpVtbl->InsertData(This, offset, data))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData_DeleteData(This, offset, count) \
    ((This)->lpVtbl->DeleteData(This, offset, count))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData_ReplaceData(This, offset, count, data) \
    ((This)->lpVtbl->ReplaceData(This, offset, count, data))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlCharacterData_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlComment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Xml.Dom.XmlComment
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Data.Xml.Dom.IXmlCharacterData
 *     Windows.Data.Xml.Dom.IXmlNode
 *     Windows.Data.Xml.Dom.IXmlNodeSelector
 *     Windows.Data.Xml.Dom.IXmlNodeSerializer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlComment_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlComment_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlComment[] = L"Windows.Data.Xml.Dom.IXmlComment";
typedef struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlCommentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlComment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlComment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlComment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlComment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlComment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlComment* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CData_CXml_CDom_CIXmlCommentVtbl;

interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlComment
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlCommentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlComment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlComment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlComment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlComment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlComment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlComment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlComment;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlComment_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlDocument
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Xml.Dom.XmlDocument
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Data.Xml.Dom.IXmlNode
 *     Windows.Data.Xml.Dom.IXmlNodeSelector
 *     Windows.Data.Xml.Dom.IXmlNodeSerializer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlDocument[] = L"Windows.Data.Xml.Dom.IXmlDocument";
typedef struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Doctype)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentType** value);
    HRESULT (STDMETHODCALLTYPE* get_Implementation)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlDomImplementation** value);
    HRESULT (STDMETHODCALLTYPE* get_DocumentElement)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlElement** value);
    HRESULT (STDMETHODCALLTYPE* CreateElement)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument* This,
        HSTRING tagName,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlElement** newElement);
    HRESULT (STDMETHODCALLTYPE* CreateDocumentFragment)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentFragment** newDocumentFragment);
    HRESULT (STDMETHODCALLTYPE* CreateTextNode)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument* This,
        HSTRING data,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlText** newTextNode);
    HRESULT (STDMETHODCALLTYPE* CreateComment)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument* This,
        HSTRING data,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlComment** newComment);
    HRESULT (STDMETHODCALLTYPE* CreateProcessingInstruction)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument* This,
        HSTRING target,
        HSTRING data,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstruction** newProcessingInstruction);
    HRESULT (STDMETHODCALLTYPE* CreateAttribute)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument* This,
        HSTRING name,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute** newAttribute);
    HRESULT (STDMETHODCALLTYPE* CreateEntityReference)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument* This,
        HSTRING name,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlEntityReference** newEntityReference);
    HRESULT (STDMETHODCALLTYPE* GetElementsByTagName)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument* This,
        HSTRING tagName,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList** elements);
    HRESULT (STDMETHODCALLTYPE* CreateCDataSection)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument* This,
        HSTRING data,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlCDataSection** newCDataSection);
    HRESULT (STDMETHODCALLTYPE* get_DocumentUri)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* CreateAttributeNS)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument* This,
        IInspectable* namespaceUri,
        HSTRING qualifiedName,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute** newAttribute);
    HRESULT (STDMETHODCALLTYPE* CreateElementNS)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument* This,
        IInspectable* namespaceUri,
        HSTRING qualifiedName,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlElement** newElement);
    HRESULT (STDMETHODCALLTYPE* GetElementById)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument* This,
        HSTRING elementId,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlElement** element);
    HRESULT (STDMETHODCALLTYPE* ImportNode)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* node,
        boolean deep,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode** newNode);

    END_INTERFACE
} __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentVtbl;

interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_get_Doctype(This, value) \
    ((This)->lpVtbl->get_Doctype(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_get_Implementation(This, value) \
    ((This)->lpVtbl->get_Implementation(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_get_DocumentElement(This, value) \
    ((This)->lpVtbl->get_DocumentElement(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_CreateElement(This, tagName, newElement) \
    ((This)->lpVtbl->CreateElement(This, tagName, newElement))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_CreateDocumentFragment(This, newDocumentFragment) \
    ((This)->lpVtbl->CreateDocumentFragment(This, newDocumentFragment))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_CreateTextNode(This, data, newTextNode) \
    ((This)->lpVtbl->CreateTextNode(This, data, newTextNode))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_CreateComment(This, data, newComment) \
    ((This)->lpVtbl->CreateComment(This, data, newComment))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_CreateProcessingInstruction(This, target, data, newProcessingInstruction) \
    ((This)->lpVtbl->CreateProcessingInstruction(This, target, data, newProcessingInstruction))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_CreateAttribute(This, name, newAttribute) \
    ((This)->lpVtbl->CreateAttribute(This, name, newAttribute))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_CreateEntityReference(This, name, newEntityReference) \
    ((This)->lpVtbl->CreateEntityReference(This, name, newEntityReference))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_GetElementsByTagName(This, tagName, elements) \
    ((This)->lpVtbl->GetElementsByTagName(This, tagName, elements))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_CreateCDataSection(This, data, newCDataSection) \
    ((This)->lpVtbl->CreateCDataSection(This, data, newCDataSection))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_get_DocumentUri(This, value) \
    ((This)->lpVtbl->get_DocumentUri(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_CreateAttributeNS(This, namespaceUri, qualifiedName, newAttribute) \
    ((This)->lpVtbl->CreateAttributeNS(This, namespaceUri, qualifiedName, newAttribute))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_CreateElementNS(This, namespaceUri, qualifiedName, newElement) \
    ((This)->lpVtbl->CreateElementNS(This, namespaceUri, qualifiedName, newElement))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_GetElementById(This, elementId, element) \
    ((This)->lpVtbl->GetElementById(This, elementId, element))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_ImportNode(This, node, deep, newNode) \
    ((This)->lpVtbl->ImportNode(This, node, deep, newNode))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlDocumentFragment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Xml.Dom.XmlDocumentFragment
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Data.Xml.Dom.IXmlNode
 *     Windows.Data.Xml.Dom.IXmlNodeSelector
 *     Windows.Data.Xml.Dom.IXmlNodeSerializer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentFragment_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentFragment_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlDocumentFragment[] = L"Windows.Data.Xml.Dom.IXmlDocumentFragment";
typedef struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentFragmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentFragment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentFragment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentFragment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentFragment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentFragment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentFragment* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentFragmentVtbl;

interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentFragment
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentFragmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentFragment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentFragment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentFragment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentFragment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentFragment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentFragment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentFragment;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentFragment_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlDocumentIO
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Xml.Dom.XmlDocument
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlDocumentIO[] = L"Windows.Data.Xml.Dom.IXmlDocumentIO";
typedef struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIOVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* LoadXml)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO* This,
        HSTRING xml);
    HRESULT (STDMETHODCALLTYPE* LoadXmlWithSettings)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO* This,
        HSTRING xml,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings* loadSettings);
    HRESULT (STDMETHODCALLTYPE* SaveToFileAsync)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);

    END_INTERFACE
} __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIOVtbl;

interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIOVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO_LoadXml(This, xml) \
    ((This)->lpVtbl->LoadXml(This, xml))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO_LoadXmlWithSettings(This, xml, loadSettings) \
    ((This)->lpVtbl->LoadXmlWithSettings(This, xml, loadSettings))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO_SaveToFileAsync(This, file, asyncInfo) \
    ((This)->lpVtbl->SaveToFileAsync(This, file, asyncInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlDocumentIO2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Xml.Dom.XmlDocument
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlDocumentIO2[] = L"Windows.Data.Xml.Dom.IXmlDocumentIO2";
typedef struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* LoadXmlFromBuffer)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO2* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* buffer);
    HRESULT (STDMETHODCALLTYPE* LoadXmlFromBufferWithSettings)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO2* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* buffer,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings* loadSettings);

    END_INTERFACE
} __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO2Vtbl;

interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO2
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO2_LoadXmlFromBuffer(This, buffer) \
    ((This)->lpVtbl->LoadXmlFromBuffer(This, buffer))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO2_LoadXmlFromBufferWithSettings(This, buffer, loadSettings) \
    ((This)->lpVtbl->LoadXmlFromBufferWithSettings(This, buffer, loadSettings))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO2;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentIO2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlDocumentStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Xml.Dom.XmlDocument
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlDocumentStatics[] = L"Windows.Data.Xml.Dom.IXmlDocumentStatics";
typedef struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* LoadFromUriAsync)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* LoadFromUriWithSettingsAsync)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings* loadSettings,
        __FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* LoadFromFileAsync)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        __FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* LoadFromFileWithSettingsAsync)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings* loadSettings,
        __FIAsyncOperation_1_Windows__CData__CXml__CDom__CXmlDocument** asyncInfo);

    END_INTERFACE
} __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStaticsVtbl;

interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics_LoadFromUriAsync(This, uri, asyncInfo) \
    ((This)->lpVtbl->LoadFromUriAsync(This, uri, asyncInfo))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics_LoadFromUriWithSettingsAsync(This, uri, loadSettings, asyncInfo) \
    ((This)->lpVtbl->LoadFromUriWithSettingsAsync(This, uri, loadSettings, asyncInfo))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics_LoadFromFileAsync(This, file, asyncInfo) \
    ((This)->lpVtbl->LoadFromFileAsync(This, file, asyncInfo))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics_LoadFromFileWithSettingsAsync(This, file, loadSettings, asyncInfo) \
    ((This)->lpVtbl->LoadFromFileWithSettingsAsync(This, file, loadSettings, asyncInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlDocumentType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Xml.Dom.XmlDocumentType
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Data.Xml.Dom.IXmlNode
 *     Windows.Data.Xml.Dom.IXmlNodeSelector
 *     Windows.Data.Xml.Dom.IXmlNodeSerializer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentType_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentType_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlDocumentType[] = L"Windows.Data.Xml.Dom.IXmlDocumentType";
typedef struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentTypeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentType* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentType* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentType* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentType* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentType* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentType* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentType* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Entities)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentType* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap** value);
    HRESULT (STDMETHODCALLTYPE* get_Notations)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentType* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap** value);

    END_INTERFACE
} __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentTypeVtbl;

interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentType
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentTypeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentType_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentType_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentType_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentType_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentType_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentType_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentType_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentType_get_Entities(This, value) \
    ((This)->lpVtbl->get_Entities(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentType_get_Notations(This, value) \
    ((This)->lpVtbl->get_Notations(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentType;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocumentType_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlDomImplementation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Xml.Dom.XmlDomImplementation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlDomImplementation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDomImplementation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlDomImplementation[] = L"Windows.Data.Xml.Dom.IXmlDomImplementation";
typedef struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlDomImplementationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDomImplementation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDomImplementation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDomImplementation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDomImplementation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDomImplementation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDomImplementation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* HasFeature)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlDomImplementation* This,
        HSTRING feature,
        IInspectable* version,
        boolean* featureSupported);

    END_INTERFACE
} __x_ABI_CWindows_CData_CXml_CDom_CIXmlDomImplementationVtbl;

interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlDomImplementation
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlDomImplementationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDomImplementation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDomImplementation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDomImplementation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDomImplementation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDomImplementation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDomImplementation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDomImplementation_HasFeature(This, feature, version, featureSupported) \
    ((This)->lpVtbl->HasFeature(This, feature, version, featureSupported))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlDomImplementation;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlDomImplementation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlElement
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Xml.Dom.XmlElement
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Data.Xml.Dom.IXmlNode
 *     Windows.Data.Xml.Dom.IXmlNodeSelector
 *     Windows.Data.Xml.Dom.IXmlNodeSerializer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlElement_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlElement_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlElement[] = L"Windows.Data.Xml.Dom.IXmlElement";
typedef struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlElementVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlElement* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlElement* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlElement* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlElement* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlElement* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlElement* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TagName)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlElement* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* GetAttribute)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlElement* This,
        HSTRING attributeName,
        HSTRING* attributeValue);
    HRESULT (STDMETHODCALLTYPE* SetAttribute)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlElement* This,
        HSTRING attributeName,
        HSTRING attributeValue);
    HRESULT (STDMETHODCALLTYPE* RemoveAttribute)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlElement* This,
        HSTRING attributeName);
    HRESULT (STDMETHODCALLTYPE* GetAttributeNode)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlElement* This,
        HSTRING attributeName,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute** attributeNode);
    HRESULT (STDMETHODCALLTYPE* SetAttributeNode)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlElement* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute* newAttribute,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute** previousAttribute);
    HRESULT (STDMETHODCALLTYPE* RemoveAttributeNode)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlElement* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute* attributeNode,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute** removedAttribute);
    HRESULT (STDMETHODCALLTYPE* GetElementsByTagName)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlElement* This,
        HSTRING tagName,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList** elements);
    HRESULT (STDMETHODCALLTYPE* SetAttributeNS)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlElement* This,
        IInspectable* namespaceUri,
        HSTRING qualifiedName,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* GetAttributeNS)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlElement* This,
        IInspectable* namespaceUri,
        HSTRING localName,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAttributeNS)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlElement* This,
        IInspectable* namespaceUri,
        HSTRING localName);
    HRESULT (STDMETHODCALLTYPE* SetAttributeNodeNS)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlElement* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute* newAttribute,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute** previousAttribute);
    HRESULT (STDMETHODCALLTYPE* GetAttributeNodeNS)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlElement* This,
        IInspectable* namespaceUri,
        HSTRING localName,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlAttribute** previousAttribute);

    END_INTERFACE
} __x_ABI_CWindows_CData_CXml_CDom_CIXmlElementVtbl;

interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlElement
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlElementVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlElement_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlElement_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlElement_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlElement_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlElement_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlElement_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlElement_get_TagName(This, value) \
    ((This)->lpVtbl->get_TagName(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlElement_GetAttribute(This, attributeName, attributeValue) \
    ((This)->lpVtbl->GetAttribute(This, attributeName, attributeValue))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlElement_SetAttribute(This, attributeName, attributeValue) \
    ((This)->lpVtbl->SetAttribute(This, attributeName, attributeValue))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlElement_RemoveAttribute(This, attributeName) \
    ((This)->lpVtbl->RemoveAttribute(This, attributeName))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlElement_GetAttributeNode(This, attributeName, attributeNode) \
    ((This)->lpVtbl->GetAttributeNode(This, attributeName, attributeNode))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlElement_SetAttributeNode(This, newAttribute, previousAttribute) \
    ((This)->lpVtbl->SetAttributeNode(This, newAttribute, previousAttribute))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlElement_RemoveAttributeNode(This, attributeNode, removedAttribute) \
    ((This)->lpVtbl->RemoveAttributeNode(This, attributeNode, removedAttribute))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlElement_GetElementsByTagName(This, tagName, elements) \
    ((This)->lpVtbl->GetElementsByTagName(This, tagName, elements))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlElement_SetAttributeNS(This, namespaceUri, qualifiedName, value) \
    ((This)->lpVtbl->SetAttributeNS(This, namespaceUri, qualifiedName, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlElement_GetAttributeNS(This, namespaceUri, localName, value) \
    ((This)->lpVtbl->GetAttributeNS(This, namespaceUri, localName, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlElement_RemoveAttributeNS(This, namespaceUri, localName) \
    ((This)->lpVtbl->RemoveAttributeNS(This, namespaceUri, localName))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlElement_SetAttributeNodeNS(This, newAttribute, previousAttribute) \
    ((This)->lpVtbl->SetAttributeNodeNS(This, newAttribute, previousAttribute))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlElement_GetAttributeNodeNS(This, namespaceUri, localName, previousAttribute) \
    ((This)->lpVtbl->GetAttributeNodeNS(This, namespaceUri, localName, previousAttribute))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlElement;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlElement_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlEntityReference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Xml.Dom.XmlEntityReference
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Data.Xml.Dom.IXmlNode
 *     Windows.Data.Xml.Dom.IXmlNodeSelector
 *     Windows.Data.Xml.Dom.IXmlNodeSerializer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlEntityReference_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlEntityReference_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlEntityReference[] = L"Windows.Data.Xml.Dom.IXmlEntityReference";
typedef struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlEntityReferenceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlEntityReference* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlEntityReference* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlEntityReference* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlEntityReference* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlEntityReference* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlEntityReference* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CData_CXml_CDom_CIXmlEntityReferenceVtbl;

interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlEntityReference
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlEntityReferenceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlEntityReference_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlEntityReference_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlEntityReference_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlEntityReference_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlEntityReference_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlEntityReference_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlEntityReference;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlEntityReference_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlLoadSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Xml.Dom.XmlLoadSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlLoadSettings[] = L"Windows.Data.Xml.Dom.IXmlLoadSettings";
typedef struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettingsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MaxElementDepth)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_MaxElementDepth)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_ProhibitDtd)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_ProhibitDtd)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_ResolveExternals)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_ResolveExternals)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_ValidateOnParse)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_ValidateOnParse)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_ElementContentWhiteSpace)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_ElementContentWhiteSpace)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettingsVtbl;

interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettingsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings_get_MaxElementDepth(This, value) \
    ((This)->lpVtbl->get_MaxElementDepth(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings_put_MaxElementDepth(This, value) \
    ((This)->lpVtbl->put_MaxElementDepth(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings_get_ProhibitDtd(This, value) \
    ((This)->lpVtbl->get_ProhibitDtd(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings_put_ProhibitDtd(This, value) \
    ((This)->lpVtbl->put_ProhibitDtd(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings_get_ResolveExternals(This, value) \
    ((This)->lpVtbl->get_ResolveExternals(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings_put_ResolveExternals(This, value) \
    ((This)->lpVtbl->put_ResolveExternals(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings_get_ValidateOnParse(This, value) \
    ((This)->lpVtbl->get_ValidateOnParse(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings_put_ValidateOnParse(This, value) \
    ((This)->lpVtbl->put_ValidateOnParse(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings_get_ElementContentWhiteSpace(This, value) \
    ((This)->lpVtbl->get_ElementContentWhiteSpace(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings_put_ElementContentWhiteSpace(This, value) \
    ((This)->lpVtbl->put_ElementContentWhiteSpace(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlLoadSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlNamedNodeMap
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Xml.Dom.XmlNamedNodeMap
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.Collections.IVectorView`1<Windows.Data.Xml.Dom.IXmlNode>
 *     Windows.Foundation.Collections.IIterable`1<Windows.Data.Xml.Dom.IXmlNode>
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlNamedNodeMap[] = L"Windows.Data.Xml.Dom.IXmlNamedNodeMap";
typedef struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMapVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Length)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* Item)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap* This,
        UINT32 index,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode** node);
    HRESULT (STDMETHODCALLTYPE* GetNamedItem)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap* This,
        HSTRING name,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode** node);
    HRESULT (STDMETHODCALLTYPE* SetNamedItem)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* node,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode** previousNode);
    HRESULT (STDMETHODCALLTYPE* RemoveNamedItem)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap* This,
        HSTRING name,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode** previousNode);
    HRESULT (STDMETHODCALLTYPE* GetNamedItemNS)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap* This,
        IInspectable* namespaceUri,
        HSTRING name,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode** node);
    HRESULT (STDMETHODCALLTYPE* RemoveNamedItemNS)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap* This,
        IInspectable* namespaceUri,
        HSTRING name,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode** previousNode);
    HRESULT (STDMETHODCALLTYPE* SetNamedItemNS)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* node,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode** previousNode);

    END_INTERFACE
} __x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMapVtbl;

interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMapVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap_get_Length(This, value) \
    ((This)->lpVtbl->get_Length(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap_Item(This, index, node) \
    ((This)->lpVtbl->Item(This, index, node))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap_GetNamedItem(This, name, node) \
    ((This)->lpVtbl->GetNamedItem(This, name, node))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap_SetNamedItem(This, node, previousNode) \
    ((This)->lpVtbl->SetNamedItem(This, node, previousNode))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap_RemoveNamedItem(This, name, previousNode) \
    ((This)->lpVtbl->RemoveNamedItem(This, name, previousNode))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap_GetNamedItemNS(This, namespaceUri, name, node) \
    ((This)->lpVtbl->GetNamedItemNS(This, namespaceUri, name, node))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap_RemoveNamedItemNS(This, namespaceUri, name, previousNode) \
    ((This)->lpVtbl->RemoveNamedItemNS(This, namespaceUri, name, previousNode))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap_SetNamedItemNS(This, node, previousNode) \
    ((This)->lpVtbl->SetNamedItemNS(This, node, previousNode))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Data.Xml.Dom.IXmlNodeSelector
 *     Windows.Data.Xml.Dom.IXmlNodeSerializer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlNode[] = L"Windows.Data.Xml.Dom.IXmlNode";
typedef struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_NodeValue)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* This,
        IInspectable** value);
    HRESULT (STDMETHODCALLTYPE* put_NodeValue)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* This,
        IInspectable* value);
    HRESULT (STDMETHODCALLTYPE* get_NodeType)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* This,
        enum __x_ABI_CWindows_CData_CXml_CDom_CNodeType* value);
    HRESULT (STDMETHODCALLTYPE* get_NodeName)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ParentNode)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode** value);
    HRESULT (STDMETHODCALLTYPE* get_ChildNodes)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList** value);
    HRESULT (STDMETHODCALLTYPE* get_FirstChild)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode** value);
    HRESULT (STDMETHODCALLTYPE* get_LastChild)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode** value);
    HRESULT (STDMETHODCALLTYPE* get_PreviousSibling)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode** value);
    HRESULT (STDMETHODCALLTYPE* get_NextSibling)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode** value);
    HRESULT (STDMETHODCALLTYPE* get_Attributes)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNamedNodeMap** value);
    HRESULT (STDMETHODCALLTYPE* HasChildNodes)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_OwnerDocument)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument** value);
    HRESULT (STDMETHODCALLTYPE* InsertBefore)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* newChild,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* referenceChild,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode** insertedChild);
    HRESULT (STDMETHODCALLTYPE* ReplaceChild)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* newChild,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* referenceChild,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode** previousChild);
    HRESULT (STDMETHODCALLTYPE* RemoveChild)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* childNode,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode** removedChild);
    HRESULT (STDMETHODCALLTYPE* AppendChild)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* newChild,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode** appendedChild);
    HRESULT (STDMETHODCALLTYPE* CloneNode)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* This,
        boolean deep,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode** newNode);
    HRESULT (STDMETHODCALLTYPE* get_NamespaceUri)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* This,
        IInspectable** value);
    HRESULT (STDMETHODCALLTYPE* get_LocalName)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* This,
        IInspectable** value);
    HRESULT (STDMETHODCALLTYPE* get_Prefix)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* This,
        IInspectable** value);
    HRESULT (STDMETHODCALLTYPE* Normalize)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* This);
    HRESULT (STDMETHODCALLTYPE* put_Prefix)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNode* This,
        IInspectable* value);

    END_INTERFACE
} __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeVtbl;

interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_get_NodeValue(This, value) \
    ((This)->lpVtbl->get_NodeValue(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_put_NodeValue(This, value) \
    ((This)->lpVtbl->put_NodeValue(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_get_NodeType(This, value) \
    ((This)->lpVtbl->get_NodeType(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_get_NodeName(This, value) \
    ((This)->lpVtbl->get_NodeName(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_get_ParentNode(This, value) \
    ((This)->lpVtbl->get_ParentNode(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_get_ChildNodes(This, value) \
    ((This)->lpVtbl->get_ChildNodes(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_get_FirstChild(This, value) \
    ((This)->lpVtbl->get_FirstChild(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_get_LastChild(This, value) \
    ((This)->lpVtbl->get_LastChild(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_get_PreviousSibling(This, value) \
    ((This)->lpVtbl->get_PreviousSibling(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_get_NextSibling(This, value) \
    ((This)->lpVtbl->get_NextSibling(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_get_Attributes(This, value) \
    ((This)->lpVtbl->get_Attributes(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_HasChildNodes(This, value) \
    ((This)->lpVtbl->HasChildNodes(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_get_OwnerDocument(This, value) \
    ((This)->lpVtbl->get_OwnerDocument(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_InsertBefore(This, newChild, referenceChild, insertedChild) \
    ((This)->lpVtbl->InsertBefore(This, newChild, referenceChild, insertedChild))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_ReplaceChild(This, newChild, referenceChild, previousChild) \
    ((This)->lpVtbl->ReplaceChild(This, newChild, referenceChild, previousChild))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_RemoveChild(This, childNode, removedChild) \
    ((This)->lpVtbl->RemoveChild(This, childNode, removedChild))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_AppendChild(This, newChild, appendedChild) \
    ((This)->lpVtbl->AppendChild(This, newChild, appendedChild))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_CloneNode(This, deep, newNode) \
    ((This)->lpVtbl->CloneNode(This, deep, newNode))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_get_NamespaceUri(This, value) \
    ((This)->lpVtbl->get_NamespaceUri(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_get_LocalName(This, value) \
    ((This)->lpVtbl->get_LocalName(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_get_Prefix(This, value) \
    ((This)->lpVtbl->get_Prefix(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_Normalize(This) \
    ((This)->lpVtbl->Normalize(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_put_Prefix(This, value) \
    ((This)->lpVtbl->put_Prefix(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlNode;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlNode_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlNodeList
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Xml.Dom.XmlNodeList
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.Collections.IVectorView`1<Windows.Data.Xml.Dom.IXmlNode>
 *     Windows.Foundation.Collections.IIterable`1<Windows.Data.Xml.Dom.IXmlNode>
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlNodeList[] = L"Windows.Data.Xml.Dom.IXmlNodeList";
typedef struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeListVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Length)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* Item)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList* This,
        UINT32 index,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode** node);

    END_INTERFACE
} __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeListVtbl;

interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeListVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList_get_Length(This, value) \
    ((This)->lpVtbl->get_Length(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList_Item(This, index, node) \
    ((This)->lpVtbl->Item(This, index, node))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlNodeSelector
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlNodeSelector[] = L"Windows.Data.Xml.Dom.IXmlNodeSelector";
typedef struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelectorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SelectSingleNode)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector* This,
        HSTRING xpath,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode** node);
    HRESULT (STDMETHODCALLTYPE* SelectNodes)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector* This,
        HSTRING xpath,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList** nodelist);
    HRESULT (STDMETHODCALLTYPE* SelectSingleNodeNS)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector* This,
        HSTRING xpath,
        IInspectable* namespaces,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNode** node);
    HRESULT (STDMETHODCALLTYPE* SelectNodesNS)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector* This,
        HSTRING xpath,
        IInspectable* namespaces,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeList** nodelist);

    END_INTERFACE
} __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelectorVtbl;

interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelectorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector_SelectSingleNode(This, xpath, node) \
    ((This)->lpVtbl->SelectSingleNode(This, xpath, node))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector_SelectNodes(This, xpath, nodelist) \
    ((This)->lpVtbl->SelectNodes(This, xpath, nodelist))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector_SelectSingleNodeNS(This, xpath, namespaces, node) \
    ((This)->lpVtbl->SelectSingleNodeNS(This, xpath, namespaces, node))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector_SelectNodesNS(This, xpath, namespaces, nodelist) \
    ((This)->lpVtbl->SelectNodesNS(This, xpath, namespaces, nodelist))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSelector_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlNodeSerializer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlNodeSerializer[] = L"Windows.Data.Xml.Dom.IXmlNodeSerializer";
typedef struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetXml)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializer* This,
        HSTRING* outerXml);
    HRESULT (STDMETHODCALLTYPE* get_InnerText)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializer* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_InnerText)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializer* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializerVtbl;

interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializer
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializer_GetXml(This, outerXml) \
    ((This)->lpVtbl->GetXml(This, outerXml))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializer_get_InnerText(This, value) \
    ((This)->lpVtbl->get_InnerText(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializer_put_InnerText(This, value) \
    ((This)->lpVtbl->put_InnerText(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializer;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlNodeSerializer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlProcessingInstruction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Data.Xml.Dom.XmlProcessingInstruction
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Data.Xml.Dom.IXmlNode
 *     Windows.Data.Xml.Dom.IXmlNodeSelector
 *     Windows.Data.Xml.Dom.IXmlNodeSerializer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstruction_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstruction_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlProcessingInstruction[] = L"Windows.Data.Xml.Dom.IXmlProcessingInstruction";
typedef struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstructionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstruction* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstruction* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstruction* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstruction* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstruction* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstruction* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Target)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstruction* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Data)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstruction* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Data)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstruction* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstructionVtbl;

interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstruction
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstructionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstruction_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstruction_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstruction_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstruction_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstruction_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstruction_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstruction_get_Target(This, value) \
    ((This)->lpVtbl->get_Target(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstruction_get_Data(This, value) \
    ((This)->lpVtbl->get_Data(This, value))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstruction_put_Data(This, value) \
    ((This)->lpVtbl->put_Data(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstruction;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlProcessingInstruction_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Data.Xml.Dom.IXmlText
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Data.Xml.Dom.IXmlCharacterData
 *     Windows.Data.Xml.Dom.IXmlNode
 *     Windows.Data.Xml.Dom.IXmlNodeSelector
 *     Windows.Data.Xml.Dom.IXmlNodeSerializer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlText_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlText_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Data_Xml_Dom_IXmlText[] = L"Windows.Data.Xml.Dom.IXmlText";
typedef struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlTextVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlText* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlText* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlText* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlText* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlText* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlText* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SplitText)(__x_ABI_CWindows_CData_CXml_CDom_CIXmlText* This,
        UINT32 offset,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlText** secondPart);

    END_INTERFACE
} __x_ABI_CWindows_CData_CXml_CDom_CIXmlTextVtbl;

interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlText
{
    CONST_VTBL struct __x_ABI_CWindows_CData_CXml_CDom_CIXmlTextVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlText_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlText_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlText_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlText_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlText_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlText_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlText_SplitText(This, offset, secondPart) \
    ((This)->lpVtbl->SplitText(This, offset, secondPart))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CData_CXml_CDom_CIXmlText;
#endif /* !defined(____x_ABI_CWindows_CData_CXml_CDom_CIXmlText_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Xml.Dom.DtdEntity
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Xml.Dom.IDtdEntity ** Default Interface **
 *    Windows.Data.Xml.Dom.IXmlNode
 *    Windows.Data.Xml.Dom.IXmlNodeSerializer
 *    Windows.Data.Xml.Dom.IXmlNodeSelector
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Xml_Dom_DtdEntity_DEFINED
#define RUNTIMECLASS_Windows_Data_Xml_Dom_DtdEntity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Xml_Dom_DtdEntity[] = L"Windows.Data.Xml.Dom.DtdEntity";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Xml.Dom.DtdNotation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Xml.Dom.IDtdNotation ** Default Interface **
 *    Windows.Data.Xml.Dom.IXmlNode
 *    Windows.Data.Xml.Dom.IXmlNodeSerializer
 *    Windows.Data.Xml.Dom.IXmlNodeSelector
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Xml_Dom_DtdNotation_DEFINED
#define RUNTIMECLASS_Windows_Data_Xml_Dom_DtdNotation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Xml_Dom_DtdNotation[] = L"Windows.Data.Xml.Dom.DtdNotation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Xml.Dom.XmlAttribute
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Xml.Dom.IXmlAttribute ** Default Interface **
 *    Windows.Data.Xml.Dom.IXmlNode
 *    Windows.Data.Xml.Dom.IXmlNodeSerializer
 *    Windows.Data.Xml.Dom.IXmlNodeSelector
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Xml_Dom_XmlAttribute_DEFINED
#define RUNTIMECLASS_Windows_Data_Xml_Dom_XmlAttribute_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Xml_Dom_XmlAttribute[] = L"Windows.Data.Xml.Dom.XmlAttribute";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Xml.Dom.XmlCDataSection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Xml.Dom.IXmlCDataSection ** Default Interface **
 *    Windows.Data.Xml.Dom.IXmlText
 *    Windows.Data.Xml.Dom.IXmlCharacterData
 *    Windows.Data.Xml.Dom.IXmlNode
 *    Windows.Data.Xml.Dom.IXmlNodeSerializer
 *    Windows.Data.Xml.Dom.IXmlNodeSelector
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Xml_Dom_XmlCDataSection_DEFINED
#define RUNTIMECLASS_Windows_Data_Xml_Dom_XmlCDataSection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Xml_Dom_XmlCDataSection[] = L"Windows.Data.Xml.Dom.XmlCDataSection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Xml.Dom.XmlComment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Xml.Dom.IXmlComment ** Default Interface **
 *    Windows.Data.Xml.Dom.IXmlCharacterData
 *    Windows.Data.Xml.Dom.IXmlNode
 *    Windows.Data.Xml.Dom.IXmlNodeSerializer
 *    Windows.Data.Xml.Dom.IXmlNodeSelector
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Xml_Dom_XmlComment_DEFINED
#define RUNTIMECLASS_Windows_Data_Xml_Dom_XmlComment_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Xml_Dom_XmlComment[] = L"Windows.Data.Xml.Dom.XmlComment";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Xml.Dom.XmlDocument
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Data.Xml.Dom.IXmlDocumentStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Data.Xml.Dom.IXmlDocument ** Default Interface **
 *    Windows.Data.Xml.Dom.IXmlNode
 *    Windows.Data.Xml.Dom.IXmlNodeSerializer
 *    Windows.Data.Xml.Dom.IXmlNodeSelector
 *    Windows.Data.Xml.Dom.IXmlDocumentIO
 *    Windows.Data.Xml.Dom.IXmlDocumentIO2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Xml_Dom_XmlDocument_DEFINED
#define RUNTIMECLASS_Windows_Data_Xml_Dom_XmlDocument_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Xml_Dom_XmlDocument[] = L"Windows.Data.Xml.Dom.XmlDocument";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Xml.Dom.XmlDocumentFragment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Xml.Dom.IXmlDocumentFragment ** Default Interface **
 *    Windows.Data.Xml.Dom.IXmlNode
 *    Windows.Data.Xml.Dom.IXmlNodeSerializer
 *    Windows.Data.Xml.Dom.IXmlNodeSelector
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Xml_Dom_XmlDocumentFragment_DEFINED
#define RUNTIMECLASS_Windows_Data_Xml_Dom_XmlDocumentFragment_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Xml_Dom_XmlDocumentFragment[] = L"Windows.Data.Xml.Dom.XmlDocumentFragment";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Xml.Dom.XmlDocumentType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Xml.Dom.IXmlDocumentType ** Default Interface **
 *    Windows.Data.Xml.Dom.IXmlNode
 *    Windows.Data.Xml.Dom.IXmlNodeSerializer
 *    Windows.Data.Xml.Dom.IXmlNodeSelector
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Xml_Dom_XmlDocumentType_DEFINED
#define RUNTIMECLASS_Windows_Data_Xml_Dom_XmlDocumentType_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Xml_Dom_XmlDocumentType[] = L"Windows.Data.Xml.Dom.XmlDocumentType";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Xml.Dom.XmlDomImplementation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Xml.Dom.IXmlDomImplementation ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Xml_Dom_XmlDomImplementation_DEFINED
#define RUNTIMECLASS_Windows_Data_Xml_Dom_XmlDomImplementation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Xml_Dom_XmlDomImplementation[] = L"Windows.Data.Xml.Dom.XmlDomImplementation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Xml.Dom.XmlElement
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Xml.Dom.IXmlElement ** Default Interface **
 *    Windows.Data.Xml.Dom.IXmlNode
 *    Windows.Data.Xml.Dom.IXmlNodeSerializer
 *    Windows.Data.Xml.Dom.IXmlNodeSelector
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Xml_Dom_XmlElement_DEFINED
#define RUNTIMECLASS_Windows_Data_Xml_Dom_XmlElement_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Xml_Dom_XmlElement[] = L"Windows.Data.Xml.Dom.XmlElement";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Xml.Dom.XmlEntityReference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Xml.Dom.IXmlEntityReference ** Default Interface **
 *    Windows.Data.Xml.Dom.IXmlNode
 *    Windows.Data.Xml.Dom.IXmlNodeSerializer
 *    Windows.Data.Xml.Dom.IXmlNodeSelector
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Xml_Dom_XmlEntityReference_DEFINED
#define RUNTIMECLASS_Windows_Data_Xml_Dom_XmlEntityReference_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Xml_Dom_XmlEntityReference[] = L"Windows.Data.Xml.Dom.XmlEntityReference";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Xml.Dom.XmlLoadSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Data.Xml.Dom.IXmlLoadSettings ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Xml_Dom_XmlLoadSettings_DEFINED
#define RUNTIMECLASS_Windows_Data_Xml_Dom_XmlLoadSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Xml_Dom_XmlLoadSettings[] = L"Windows.Data.Xml.Dom.XmlLoadSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Xml.Dom.XmlNamedNodeMap
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Xml.Dom.IXmlNamedNodeMap ** Default Interface **
 *    Windows.Foundation.Collections.IVectorView`1<Windows.Data.Xml.Dom.IXmlNode>
 *    Windows.Foundation.Collections.IIterable`1<Windows.Data.Xml.Dom.IXmlNode>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Xml_Dom_XmlNamedNodeMap_DEFINED
#define RUNTIMECLASS_Windows_Data_Xml_Dom_XmlNamedNodeMap_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Xml_Dom_XmlNamedNodeMap[] = L"Windows.Data.Xml.Dom.XmlNamedNodeMap";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Xml.Dom.XmlNodeList
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Xml.Dom.IXmlNodeList ** Default Interface **
 *    Windows.Foundation.Collections.IVectorView`1<Windows.Data.Xml.Dom.IXmlNode>
 *    Windows.Foundation.Collections.IIterable`1<Windows.Data.Xml.Dom.IXmlNode>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Xml_Dom_XmlNodeList_DEFINED
#define RUNTIMECLASS_Windows_Data_Xml_Dom_XmlNodeList_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Xml_Dom_XmlNodeList[] = L"Windows.Data.Xml.Dom.XmlNodeList";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Xml.Dom.XmlProcessingInstruction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Xml.Dom.IXmlProcessingInstruction ** Default Interface **
 *    Windows.Data.Xml.Dom.IXmlNode
 *    Windows.Data.Xml.Dom.IXmlNodeSerializer
 *    Windows.Data.Xml.Dom.IXmlNodeSelector
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Xml_Dom_XmlProcessingInstruction_DEFINED
#define RUNTIMECLASS_Windows_Data_Xml_Dom_XmlProcessingInstruction_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Xml_Dom_XmlProcessingInstruction[] = L"Windows.Data.Xml.Dom.XmlProcessingInstruction";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Data.Xml.Dom.XmlText
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Data.Xml.Dom.IXmlText ** Default Interface **
 *    Windows.Data.Xml.Dom.IXmlCharacterData
 *    Windows.Data.Xml.Dom.IXmlNode
 *    Windows.Data.Xml.Dom.IXmlNodeSerializer
 *    Windows.Data.Xml.Dom.IXmlNodeSelector
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Data_Xml_Dom_XmlText_DEFINED
#define RUNTIMECLASS_Windows_Data_Xml_Dom_XmlText_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Data_Xml_Dom_XmlText[] = L"Windows.Data.Xml.Dom.XmlText";
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
#endif // __windows2Edata2Exml2Edom_p_h__

#endif // __windows2Edata2Exml2Edom_h__
