
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
#ifndef __windows2Eui2Examl2Eshapes_h__
#define __windows2Eui2Examl2Eshapes_h__
#ifndef __windows2Eui2Examl2Eshapes_p_h__
#define __windows2Eui2Examl2Eshapes_p_h__


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
#include "Windows.UI.Composition.h"
#include "Windows.UI.Xaml.h"
#include "Windows.UI.Xaml.Media.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CXaml_CShapes_CIEllipse_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIEllipse_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Shapes {
                    interface IEllipse;
                } /* Shapes */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIEllipse ABI::Windows::UI::Xaml::Shapes::IEllipse

#endif // ____x_ABI_CWindows_CUI_CXaml_CShapes_CIEllipse_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CShapes_CILine_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CILine_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Shapes {
                    interface ILine;
                } /* Shapes */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CShapes_CILine ABI::Windows::UI::Xaml::Shapes::ILine

#endif // ____x_ABI_CWindows_CUI_CXaml_CShapes_CILine_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Shapes {
                    interface ILineStatics;
                } /* Shapes */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics ABI::Windows::UI::Xaml::Shapes::ILineStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPath_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPath_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Shapes {
                    interface IPath;
                } /* Shapes */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPath ABI::Windows::UI::Xaml::Shapes::IPath

#endif // ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPath_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPathFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPathFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Shapes {
                    interface IPathFactory;
                } /* Shapes */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPathFactory ABI::Windows::UI::Xaml::Shapes::IPathFactory

#endif // ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPathFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPathStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPathStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Shapes {
                    interface IPathStatics;
                } /* Shapes */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPathStatics ABI::Windows::UI::Xaml::Shapes::IPathStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPathStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Shapes {
                    interface IPolygon;
                } /* Shapes */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon ABI::Windows::UI::Xaml::Shapes::IPolygon

#endif // ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Shapes {
                    interface IPolygonStatics;
                } /* Shapes */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonStatics ABI::Windows::UI::Xaml::Shapes::IPolygonStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Shapes {
                    interface IPolyline;
                } /* Shapes */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline ABI::Windows::UI::Xaml::Shapes::IPolyline

#endif // ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Shapes {
                    interface IPolylineStatics;
                } /* Shapes */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineStatics ABI::Windows::UI::Xaml::Shapes::IPolylineStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Shapes {
                    interface IRectangle;
                } /* Shapes */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle ABI::Windows::UI::Xaml::Shapes::IRectangle

#endif // ____x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Shapes {
                    interface IRectangleStatics;
                } /* Shapes */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleStatics ABI::Windows::UI::Xaml::Shapes::IRectangleStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Shapes {
                    interface IShape;
                } /* Shapes */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape ABI::Windows::UI::Xaml::Shapes::IShape

#endif // ____x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CShapes_CIShape2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIShape2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Shapes {
                    interface IShape2;
                } /* Shapes */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape2 ABI::Windows::UI::Xaml::Shapes::IShape2

#endif // ____x_ABI_CWindows_CUI_CXaml_CShapes_CIShape2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Shapes {
                    interface IShapeFactory;
                } /* Shapes */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeFactory ABI::Windows::UI::Xaml::Shapes::IShapeFactory

#endif // ____x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Shapes {
                    interface IShapeStatics;
                } /* Shapes */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics ABI::Windows::UI::Xaml::Shapes::IShapeStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                class CompositionBrush;
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CComposition_CICompositionBrush_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CICompositionBrush_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                interface ICompositionBrush;
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CICompositionBrush ABI::Windows::UI::Composition::ICompositionBrush

#endif // ____x_ABI_CWindows_CUI_CComposition_CICompositionBrush_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                class DependencyProperty;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIDependencyProperty_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIDependencyProperty_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                interface IDependencyProperty;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty ABI::Windows::UI::Xaml::IDependencyProperty

#endif // ____x_ABI_CWindows_CUI_CXaml_CIDependencyProperty_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    class Brush;
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CIBrush_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CIBrush_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    interface IBrush;
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMedia_CIBrush ABI::Windows::UI::Xaml::Media::IBrush

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CIBrush_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    class DoubleCollection;
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */


#ifndef DEF___FIIterator_1_double_USE
#define DEF___FIIterator_1_double_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("638a2cf4-f474-5318-9055-141cb909ac4b"))
IIterator<double> : IIterator_impl<double>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Double>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<double> __FIIterator_1_double_t;
#define __FIIterator_1_double ABI::Windows::Foundation::Collections::__FIIterator_1_double_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_double_USE */



#ifndef DEF___FIIterable_1_double_USE
#define DEF___FIIterable_1_double_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c738964e-9c64-5bce-b5ce-61e9a282ec4a"))
IIterable<double> : IIterable_impl<double>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Double>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<double> __FIIterable_1_double_t;
#define __FIIterable_1_double ABI::Windows::Foundation::Collections::__FIIterable_1_double_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_double_USE */



#ifndef DEF___FIVectorView_1_double_USE
#define DEF___FIVectorView_1_double_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("af7586a8-6b21-5f61-bff1-1b682293ad96"))
IVectorView<double> : IVectorView_impl<double>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Double>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<double> __FIVectorView_1_double_t;
#define __FIVectorView_1_double ABI::Windows::Foundation::Collections::__FIVectorView_1_double_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_double_USE */



#ifndef DEF___FIVector_1_double_USE
#define DEF___FIVector_1_double_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f452d23c-bf05-5f3e-88e7-d17a6716b911"))
IVector<double> : IVector_impl<double>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Double>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<double> __FIVector_1_double_t;
#define __FIVector_1_double ABI::Windows::Foundation::Collections::__FIVector_1_double_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_double_USE */


namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    typedef enum FillRule : int FillRule;
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
                    class Geometry;
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CIGeometry_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CIGeometry_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    interface IGeometry;
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMedia_CIGeometry ABI::Windows::UI::Xaml::Media::IGeometry

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CIGeometry_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    typedef enum PenLineCap : int PenLineCap;
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
                    typedef enum PenLineJoin : int PenLineJoin;
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
                    class PointCollection;
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct Point Point;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CFoundation__CPoint_USE
#define DEF___FIIterator_1_Windows__CFoundation__CPoint_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c602b59e-0a8e-5e99-b478-2b564585278d"))
IIterator<struct ABI::Windows::Foundation::Point> : IIterator_impl<struct ABI::Windows::Foundation::Point>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Point>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<struct ABI::Windows::Foundation::Point> __FIIterator_1_Windows__CFoundation__CPoint_t;
#define __FIIterator_1_Windows__CFoundation__CPoint ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CFoundation__CPoint_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CFoundation__CPoint_USE */

#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CFoundation__CPoint_USE
#define DEF___FIIterable_1_Windows__CFoundation__CPoint_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c192280d-3a09-5423-9dc5-67b83ebde41d"))
IIterable<struct ABI::Windows::Foundation::Point> : IIterable_impl<struct ABI::Windows::Foundation::Point>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Point>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<struct ABI::Windows::Foundation::Point> __FIIterable_1_Windows__CFoundation__CPoint_t;
#define __FIIterable_1_Windows__CFoundation__CPoint ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CFoundation__CPoint_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CFoundation__CPoint_USE */

#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CFoundation__CPoint_USE
#define DEF___FIVectorView_1_Windows__CFoundation__CPoint_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0b7b4c9d-182f-582a-bddb-42b1aac30cad"))
IVectorView<struct ABI::Windows::Foundation::Point> : IVectorView_impl<struct ABI::Windows::Foundation::Point>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Foundation.Point>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<struct ABI::Windows::Foundation::Point> __FIVectorView_1_Windows__CFoundation__CPoint_t;
#define __FIVectorView_1_Windows__CFoundation__CPoint ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CFoundation__CPoint_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CFoundation__CPoint_USE */

#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CFoundation__CPoint_USE
#define DEF___FIVector_1_Windows__CFoundation__CPoint_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c0d513a9-ec4a-5a5d-b6d5-b707defdb9f7"))
IVector<struct ABI::Windows::Foundation::Point> : IVector_impl<struct ABI::Windows::Foundation::Point>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Foundation.Point>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<struct ABI::Windows::Foundation::Point> __FIVector_1_Windows__CFoundation__CPoint_t;
#define __FIVector_1_Windows__CFoundation__CPoint ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CFoundation__CPoint_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CFoundation__CPoint_USE */

#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    typedef enum Stretch : int Stretch;
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
                    class Transform;
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CITransform_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CITransform_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    interface ITransform;
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMedia_CITransform ABI::Windows::UI::Xaml::Media::ITransform

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CITransform_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Shapes {
                    class Path;
                } /* Shapes */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Shapes {
                    class Shape;
                } /* Shapes */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Interface Windows.UI.Xaml.Shapes.IEllipse
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Shapes.Ellipse
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIEllipse_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIEllipse_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Shapes_IEllipse[] = L"Windows.UI.Xaml.Shapes.IEllipse";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Shapes {
                    MIDL_INTERFACE("70e05ac4-d38d-4bab-831f-4a22ef52ac86")
                    IEllipse : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IEllipse = __uuidof(IEllipse);
                } /* Shapes */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CShapes_CIEllipse;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIEllipse_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Shapes.ILine
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Shapes.Line
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CILine_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CILine_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Shapes_ILine[] = L"Windows.UI.Xaml.Shapes.ILine";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Shapes {
                    MIDL_INTERFACE("46a5433d-4ffb-48df-8732-4e15c834816b")
                    ILine : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_X1(
                            DOUBLE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_X1(
                            DOUBLE value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Y1(
                            DOUBLE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Y1(
                            DOUBLE value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_X2(
                            DOUBLE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_X2(
                            DOUBLE value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Y2(
                            DOUBLE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Y2(
                            DOUBLE value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILine = __uuidof(ILine);
                } /* Shapes */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CShapes_CILine;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CILine_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Shapes.ILineStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Shapes.Line
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Shapes_ILineStatics[] = L"Windows.UI.Xaml.Shapes.ILineStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Shapes {
                    MIDL_INTERFACE("267c123d-6ea4-4c50-8b1d-50207aff1e8a")
                    ILineStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_X1Property(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Y1Property(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_X2Property(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Y2Property(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILineStatics = __uuidof(ILineStatics);
                } /* Shapes */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Shapes.IPath
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Shapes.Path
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIPath_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPath_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Shapes_IPath[] = L"Windows.UI.Xaml.Shapes.IPath";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Shapes {
                    MIDL_INTERFACE("78883609-3d57-4f3c-b8a5-6cabcac9711f")
                    IPath : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Data(
                            ABI::Windows::UI::Xaml::Media::IGeometry** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Data(
                            ABI::Windows::UI::Xaml::Media::IGeometry* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPath = __uuidof(IPath);
                } /* Shapes */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CShapes_CIPath;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIPath_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Shapes.IPathFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Shapes.Path
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIPathFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPathFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Shapes_IPathFactory[] = L"Windows.UI.Xaml.Shapes.IPathFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Shapes {
                    MIDL_INTERFACE("2340a4e3-5a86-4fc6-9a50-cbb93b828766")
                    IPathFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            IInspectable* baseInterface,
                            IInspectable** innerInterface,
                            ABI::Windows::UI::Xaml::Shapes::IPath** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPathFactory = __uuidof(IPathFactory);
                } /* Shapes */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CShapes_CIPathFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIPathFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Shapes.IPathStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Shapes.Path
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIPathStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPathStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Shapes_IPathStatics[] = L"Windows.UI.Xaml.Shapes.IPathStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Shapes {
                    MIDL_INTERFACE("f627e59d-87dc-4142-81f1-97fc7ff8641c")
                    IPathStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_DataProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPathStatics = __uuidof(IPathStatics);
                } /* Shapes */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CShapes_CIPathStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIPathStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Shapes.IPolygon
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Shapes.Polygon
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Shapes_IPolygon[] = L"Windows.UI.Xaml.Shapes.IPolygon";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Shapes {
                    MIDL_INTERFACE("e3755c19-2e4d-4bcc-8d34-86871957fa01")
                    IPolygon : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_FillRule(
                            ABI::Windows::UI::Xaml::Media::FillRule* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_FillRule(
                            ABI::Windows::UI::Xaml::Media::FillRule value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Points(
                            __FIVector_1_Windows__CFoundation__CPoint** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Points(
                            __FIVector_1_Windows__CFoundation__CPoint* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPolygon = __uuidof(IPolygon);
                } /* Shapes */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Shapes.IPolygonStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Shapes.Polygon
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Shapes_IPolygonStatics[] = L"Windows.UI.Xaml.Shapes.IPolygonStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Shapes {
                    MIDL_INTERFACE("362a8aab-d463-4366-9e1a-beba72810fb7")
                    IPolygonStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_FillRuleProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PointsProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPolygonStatics = __uuidof(IPolygonStatics);
                } /* Shapes */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Shapes.IPolyline
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Shapes.Polyline
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Shapes_IPolyline[] = L"Windows.UI.Xaml.Shapes.IPolyline";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Shapes {
                    MIDL_INTERFACE("91dc62f8-42b3-47f3-8476-c55124a7c4c6")
                    IPolyline : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_FillRule(
                            ABI::Windows::UI::Xaml::Media::FillRule* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_FillRule(
                            ABI::Windows::UI::Xaml::Media::FillRule value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Points(
                            __FIVector_1_Windows__CFoundation__CPoint** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Points(
                            __FIVector_1_Windows__CFoundation__CPoint* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPolyline = __uuidof(IPolyline);
                } /* Shapes */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Shapes.IPolylineStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Shapes.Polyline
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Shapes_IPolylineStatics[] = L"Windows.UI.Xaml.Shapes.IPolylineStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Shapes {
                    MIDL_INTERFACE("c7aa2cd1-a26c-43b0-aaa5-822fa64a11b9")
                    IPolylineStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_FillRuleProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PointsProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPolylineStatics = __uuidof(IPolylineStatics);
                } /* Shapes */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Shapes.IRectangle
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Shapes.Rectangle
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Shapes_IRectangle[] = L"Windows.UI.Xaml.Shapes.IRectangle";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Shapes {
                    MIDL_INTERFACE("855bc230-8a11-4e18-a136-4bc21c7827b0")
                    IRectangle : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_RadiusX(
                            DOUBLE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_RadiusX(
                            DOUBLE value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RadiusY(
                            DOUBLE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_RadiusY(
                            DOUBLE value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRectangle = __uuidof(IRectangle);
                } /* Shapes */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Shapes.IRectangleStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Shapes.Rectangle
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Shapes_IRectangleStatics[] = L"Windows.UI.Xaml.Shapes.IRectangleStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Shapes {
                    MIDL_INTERFACE("9f25aa53-bb3a-4c3c-89db-6fbc0d1fa0cc")
                    IRectangleStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_RadiusXProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RadiusYProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRectangleStatics = __uuidof(IRectangleStatics);
                } /* Shapes */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Shapes.IShape
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Shapes.Shape
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Shapes_IShape[] = L"Windows.UI.Xaml.Shapes.IShape";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Shapes {
                    MIDL_INTERFACE("786f2b75-9aa0-454d-ae06-a2466e37c832")
                    IShape : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Fill(
                            ABI::Windows::UI::Xaml::Media::IBrush** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Fill(
                            ABI::Windows::UI::Xaml::Media::IBrush* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Stroke(
                            ABI::Windows::UI::Xaml::Media::IBrush** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Stroke(
                            ABI::Windows::UI::Xaml::Media::IBrush* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StrokeMiterLimit(
                            DOUBLE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_StrokeMiterLimit(
                            DOUBLE value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StrokeThickness(
                            DOUBLE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_StrokeThickness(
                            DOUBLE value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StrokeStartLineCap(
                            ABI::Windows::UI::Xaml::Media::PenLineCap* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_StrokeStartLineCap(
                            ABI::Windows::UI::Xaml::Media::PenLineCap value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StrokeEndLineCap(
                            ABI::Windows::UI::Xaml::Media::PenLineCap* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_StrokeEndLineCap(
                            ABI::Windows::UI::Xaml::Media::PenLineCap value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StrokeLineJoin(
                            ABI::Windows::UI::Xaml::Media::PenLineJoin* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_StrokeLineJoin(
                            ABI::Windows::UI::Xaml::Media::PenLineJoin value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StrokeDashOffset(
                            DOUBLE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_StrokeDashOffset(
                            DOUBLE value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StrokeDashCap(
                            ABI::Windows::UI::Xaml::Media::PenLineCap* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_StrokeDashCap(
                            ABI::Windows::UI::Xaml::Media::PenLineCap value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StrokeDashArray(
                            __FIVector_1_double** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_StrokeDashArray(
                            __FIVector_1_double* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Stretch(
                            ABI::Windows::UI::Xaml::Media::Stretch* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Stretch(
                            ABI::Windows::UI::Xaml::Media::Stretch value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_GeometryTransform(
                            ABI::Windows::UI::Xaml::Media::ITransform** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IShape = __uuidof(IShape);
                } /* Shapes */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CShapes_CIShape;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Shapes.IShape2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Shapes.Shape
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIShape2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIShape2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Shapes_IShape2[] = L"Windows.UI.Xaml.Shapes.IShape2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Shapes {
                    MIDL_INTERFACE("97248dba-49f2-49a4-a5dd-164df824db14")
                    IShape2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetAlphaMask(
                            ABI::Windows::UI::Composition::ICompositionBrush** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IShape2 = __uuidof(IShape2);
                } /* Shapes */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CShapes_CIShape2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIShape2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Shapes.IShapeFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Shapes.Shape
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Shapes_IShapeFactory[] = L"Windows.UI.Xaml.Shapes.IShapeFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Shapes {
                    MIDL_INTERFACE("4b717613-f6aa-48d5-9588-e1d188eacbc9")
                    IShapeFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            IInspectable* baseInterface,
                            IInspectable** innerInterface,
                            ABI::Windows::UI::Xaml::Shapes::IShape** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IShapeFactory = __uuidof(IShapeFactory);
                } /* Shapes */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Shapes.IShapeStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Shapes.Shape
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Shapes_IShapeStatics[] = L"Windows.UI.Xaml.Shapes.IShapeStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Shapes {
                    MIDL_INTERFACE("1d7b4c55-9df3-48dc-9194-9d306faa6089")
                    IShapeStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_FillProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StrokeProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StrokeMiterLimitProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StrokeThicknessProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StrokeStartLineCapProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StrokeEndLineCapProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StrokeLineJoinProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StrokeDashOffsetProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StrokeDashCapProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StrokeDashArrayProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StretchProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IShapeStatics = __uuidof(IShapeStatics);
                } /* Shapes */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Shapes.Ellipse
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Shapes.IEllipse ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Shapes_Ellipse_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Shapes_Ellipse_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Shapes_Ellipse[] = L"Windows.UI.Xaml.Shapes.Ellipse";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Shapes.Line
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Shapes.ILineStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Shapes.ILine ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Shapes_Line_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Shapes_Line_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Shapes_Line[] = L"Windows.UI.Xaml.Shapes.Line";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Shapes.Path
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Shapes.IPathStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Shapes.IPath ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Shapes_Path_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Shapes_Path_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Shapes_Path[] = L"Windows.UI.Xaml.Shapes.Path";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Shapes.Polygon
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Shapes.IPolygonStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Shapes.IPolygon ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Shapes_Polygon_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Shapes_Polygon_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Shapes_Polygon[] = L"Windows.UI.Xaml.Shapes.Polygon";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Shapes.Polyline
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Shapes.IPolylineStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Shapes.IPolyline ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Shapes_Polyline_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Shapes_Polyline_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Shapes_Polyline[] = L"Windows.UI.Xaml.Shapes.Polyline";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Shapes.Rectangle
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Shapes.IRectangleStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Shapes.IRectangle ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Shapes_Rectangle_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Shapes_Rectangle_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Shapes_Rectangle[] = L"Windows.UI.Xaml.Shapes.Rectangle";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Shapes.Shape
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Shapes.IShapeStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Shapes.IShape ** Default Interface **
 *    Windows.UI.Xaml.Shapes.IShape2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Shapes_Shape_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Shapes_Shape_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Shapes_Shape[] = L"Windows.UI.Xaml.Shapes.Shape";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CXaml_CShapes_CIEllipse_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIEllipse_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CShapes_CIEllipse __x_ABI_CWindows_CUI_CXaml_CShapes_CIEllipse;

#endif // ____x_ABI_CWindows_CUI_CXaml_CShapes_CIEllipse_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CShapes_CILine_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CILine_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CShapes_CILine __x_ABI_CWindows_CUI_CXaml_CShapes_CILine;

#endif // ____x_ABI_CWindows_CUI_CXaml_CShapes_CILine_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics __x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPath_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPath_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CShapes_CIPath __x_ABI_CWindows_CUI_CXaml_CShapes_CIPath;

#endif // ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPath_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPathFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPathFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CShapes_CIPathFactory __x_ABI_CWindows_CUI_CXaml_CShapes_CIPathFactory;

#endif // ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPathFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPathStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPathStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CShapes_CIPathStatics __x_ABI_CWindows_CUI_CXaml_CShapes_CIPathStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPathStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon;

#endif // ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonStatics __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline;

#endif // ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineStatics __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle __x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle;

#endif // ____x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleStatics __x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape;

#endif // ____x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CShapes_CIShape2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIShape2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape2 __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape2;

#endif // ____x_ABI_CWindows_CUI_CXaml_CShapes_CIShape2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeFactory __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeFactory;

#endif // ____x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#ifndef ____x_ABI_CWindows_CUI_CComposition_CICompositionBrush_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CICompositionBrush_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CICompositionBrush __x_ABI_CWindows_CUI_CComposition_CICompositionBrush;

#endif // ____x_ABI_CWindows_CUI_CComposition_CICompositionBrush_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIDependencyProperty_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIDependencyProperty_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty;

#endif // ____x_ABI_CWindows_CUI_CXaml_CIDependencyProperty_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CIBrush_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CIBrush_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMedia_CIBrush __x_ABI_CWindows_CUI_CXaml_CMedia_CIBrush;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CIBrush_FWD_DEFINED__

#if !defined(____FIIterator_1_double_INTERFACE_DEFINED__)
#define ____FIIterator_1_double_INTERFACE_DEFINED__

typedef interface __FIIterator_1_double __FIIterator_1_double;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_double;

typedef struct __FIIterator_1_doubleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_double* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_double* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_double* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_double* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_double* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_double* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_double* This,
        DOUBLE* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_double* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_double* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_double* This,
        UINT32 itemsLength,
        DOUBLE* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_doubleVtbl;

interface __FIIterator_1_double
{
    CONST_VTBL struct __FIIterator_1_doubleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_double_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_double_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_double_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_double_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_double_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_double_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_double_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_double_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_double_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_double_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_double_INTERFACE_DEFINED__

#if !defined(____FIIterable_1_double_INTERFACE_DEFINED__)
#define ____FIIterable_1_double_INTERFACE_DEFINED__

typedef interface __FIIterable_1_double __FIIterable_1_double;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_double;

typedef struct __FIIterable_1_doubleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_double* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_double* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_double* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_double* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_double* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_double* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_double* This,
        __FIIterator_1_double** result);

    END_INTERFACE
} __FIIterable_1_doubleVtbl;

interface __FIIterable_1_double
{
    CONST_VTBL struct __FIIterable_1_doubleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_double_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_double_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_double_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_double_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_double_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_double_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_double_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_double_INTERFACE_DEFINED__

#if !defined(____FIVectorView_1_double_INTERFACE_DEFINED__)
#define ____FIVectorView_1_double_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_double __FIVectorView_1_double;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_double;

typedef struct __FIVectorView_1_doubleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_double* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_double* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_double* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_double* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_double* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_double* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_double* This,
        UINT32 index,
        DOUBLE* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_double* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_double* This,
        DOUBLE value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_double* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        DOUBLE* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_doubleVtbl;

interface __FIVectorView_1_double
{
    CONST_VTBL struct __FIVectorView_1_doubleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_double_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_double_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_double_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_double_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_double_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_double_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_double_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_double_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_double_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_double_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_double_INTERFACE_DEFINED__

#if !defined(____FIVector_1_double_INTERFACE_DEFINED__)
#define ____FIVector_1_double_INTERFACE_DEFINED__

typedef interface __FIVector_1_double __FIVector_1_double;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_double;

typedef struct __FIVector_1_doubleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_double* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_double* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_double* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_double* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_double* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_double* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_double* This,
        UINT32 index,
        DOUBLE* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_double* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_double* This,
        __FIVectorView_1_double** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_double* This,
        DOUBLE value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_double* This,
        UINT32 index,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_double* This,
        UINT32 index,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_double* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_double* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_double* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_double* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_double* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        DOUBLE* items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_double* This,
        UINT32 itemsLength,
        DOUBLE* items);

    END_INTERFACE
} __FIVector_1_doubleVtbl;

interface __FIVector_1_double
{
    CONST_VTBL struct __FIVector_1_doubleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_double_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_double_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_double_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_double_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_double_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_double_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_double_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_double_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_double_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_double_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_double_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_double_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_double_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_double_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_double_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_double_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_double_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_double_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_double_INTERFACE_DEFINED__

typedef enum __x_ABI_CWindows_CUI_CXaml_CMedia_CFillRule __x_ABI_CWindows_CUI_CXaml_CMedia_CFillRule;

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CIGeometry_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CIGeometry_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMedia_CIGeometry __x_ABI_CWindows_CUI_CXaml_CMedia_CIGeometry;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CIGeometry_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CUI_CXaml_CMedia_CPenLineCap __x_ABI_CWindows_CUI_CXaml_CMedia_CPenLineCap;

typedef enum __x_ABI_CWindows_CUI_CXaml_CMedia_CPenLineJoin __x_ABI_CWindows_CUI_CXaml_CMedia_CPenLineJoin;

typedef struct __x_ABI_CWindows_CFoundation_CPoint __x_ABI_CWindows_CFoundation_CPoint;

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CFoundation__CPoint_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CFoundation__CPoint_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CFoundation__CPoint __FIIterator_1_Windows__CFoundation__CPoint;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CFoundation__CPoint;

typedef struct __FIIterator_1_Windows__CFoundation__CPointVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CFoundation__CPoint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CFoundation__CPoint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CFoundation__CPoint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CFoundation__CPoint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CFoundation__CPoint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CFoundation__CPoint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CFoundation__CPoint* This,
        struct __x_ABI_CWindows_CFoundation_CPoint* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CFoundation__CPoint* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CFoundation__CPoint* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CFoundation__CPoint* This,
        UINT32 itemsLength,
        struct __x_ABI_CWindows_CFoundation_CPoint* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CFoundation__CPointVtbl;

interface __FIIterator_1_Windows__CFoundation__CPoint
{
    CONST_VTBL struct __FIIterator_1_Windows__CFoundation__CPointVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CFoundation__CPoint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CFoundation__CPoint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CFoundation__CPoint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CFoundation__CPoint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CFoundation__CPoint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CFoundation__CPoint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CFoundation__CPoint_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CFoundation__CPoint_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CFoundation__CPoint_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CFoundation__CPoint_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CFoundation__CPoint_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CFoundation__CPoint_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CFoundation__CPoint_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CFoundation__CPoint __FIIterable_1_Windows__CFoundation__CPoint;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CFoundation__CPoint;

typedef struct __FIIterable_1_Windows__CFoundation__CPointVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CFoundation__CPoint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CFoundation__CPoint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CFoundation__CPoint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CFoundation__CPoint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CFoundation__CPoint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CFoundation__CPoint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CFoundation__CPoint* This,
        __FIIterator_1_Windows__CFoundation__CPoint** result);

    END_INTERFACE
} __FIIterable_1_Windows__CFoundation__CPointVtbl;

interface __FIIterable_1_Windows__CFoundation__CPoint
{
    CONST_VTBL struct __FIIterable_1_Windows__CFoundation__CPointVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CFoundation__CPoint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CFoundation__CPoint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CFoundation__CPoint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CFoundation__CPoint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CFoundation__CPoint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CFoundation__CPoint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CFoundation__CPoint_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CFoundation__CPoint_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CFoundation__CPoint_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CFoundation__CPoint_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CFoundation__CPoint __FIVectorView_1_Windows__CFoundation__CPoint;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CFoundation__CPoint;

typedef struct __FIVectorView_1_Windows__CFoundation__CPointVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CFoundation__CPoint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CFoundation__CPoint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CFoundation__CPoint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CFoundation__CPoint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CFoundation__CPoint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CFoundation__CPoint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CFoundation__CPoint* This,
        UINT32 index,
        struct __x_ABI_CWindows_CFoundation_CPoint* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CFoundation__CPoint* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CFoundation__CPoint* This,
        struct __x_ABI_CWindows_CFoundation_CPoint value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CFoundation__CPoint* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        struct __x_ABI_CWindows_CFoundation_CPoint* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CFoundation__CPointVtbl;

interface __FIVectorView_1_Windows__CFoundation__CPoint
{
    CONST_VTBL struct __FIVectorView_1_Windows__CFoundation__CPointVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CFoundation__CPoint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CFoundation__CPoint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CFoundation__CPoint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CFoundation__CPoint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CFoundation__CPoint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CFoundation__CPoint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CFoundation__CPoint_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CFoundation__CPoint_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CFoundation__CPoint_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CFoundation__CPoint_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CFoundation__CPoint_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CFoundation__CPoint_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CFoundation__CPoint_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CFoundation__CPoint __FIVector_1_Windows__CFoundation__CPoint;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CFoundation__CPoint;

typedef struct __FIVector_1_Windows__CFoundation__CPointVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CFoundation__CPoint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CFoundation__CPoint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CFoundation__CPoint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CFoundation__CPoint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CFoundation__CPoint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CFoundation__CPoint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CFoundation__CPoint* This,
        UINT32 index,
        struct __x_ABI_CWindows_CFoundation_CPoint* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CFoundation__CPoint* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CFoundation__CPoint* This,
        __FIVectorView_1_Windows__CFoundation__CPoint** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CFoundation__CPoint* This,
        struct __x_ABI_CWindows_CFoundation_CPoint value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CFoundation__CPoint* This,
        UINT32 index,
        struct __x_ABI_CWindows_CFoundation_CPoint value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CFoundation__CPoint* This,
        UINT32 index,
        struct __x_ABI_CWindows_CFoundation_CPoint value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CFoundation__CPoint* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CFoundation__CPoint* This,
        struct __x_ABI_CWindows_CFoundation_CPoint value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CFoundation__CPoint* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CFoundation__CPoint* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CFoundation__CPoint* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        struct __x_ABI_CWindows_CFoundation_CPoint* items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CFoundation__CPoint* This,
        UINT32 itemsLength,
        struct __x_ABI_CWindows_CFoundation_CPoint* items);

    END_INTERFACE
} __FIVector_1_Windows__CFoundation__CPointVtbl;

interface __FIVector_1_Windows__CFoundation__CPoint
{
    CONST_VTBL struct __FIVector_1_Windows__CFoundation__CPointVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CFoundation__CPoint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CFoundation__CPoint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CFoundation__CPoint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CFoundation__CPoint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CFoundation__CPoint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CFoundation__CPoint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CFoundation__CPoint_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CFoundation__CPoint_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CFoundation__CPoint_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CFoundation__CPoint_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CFoundation__CPoint_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CFoundation__CPoint_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CFoundation__CPoint_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CFoundation__CPoint_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CFoundation__CPoint_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CFoundation__CPoint_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CFoundation__CPoint_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CFoundation__CPoint_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CFoundation__CPoint_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CUI_CXaml_CMedia_CStretch __x_ABI_CWindows_CUI_CXaml_CMedia_CStretch;

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CITransform_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CITransform_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMedia_CITransform __x_ABI_CWindows_CUI_CXaml_CMedia_CITransform;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CITransform_FWD_DEFINED__

/*
 *
 * Interface Windows.UI.Xaml.Shapes.IEllipse
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Shapes.Ellipse
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIEllipse_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIEllipse_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Shapes_IEllipse[] = L"Windows.UI.Xaml.Shapes.IEllipse";
typedef struct __x_ABI_CWindows_CUI_CXaml_CShapes_CIEllipseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIEllipse* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIEllipse* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIEllipse* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIEllipse* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIEllipse* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIEllipse* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CShapes_CIEllipseVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CShapes_CIEllipse
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CShapes_CIEllipseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIEllipse_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIEllipse_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIEllipse_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIEllipse_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIEllipse_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIEllipse_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CShapes_CIEllipse;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIEllipse_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Shapes.ILine
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Shapes.Line
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CILine_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CILine_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Shapes_ILine[] = L"Windows.UI.Xaml.Shapes.ILine";
typedef struct __x_ABI_CWindows_CUI_CXaml_CShapes_CILineVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CShapes_CILine* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CShapes_CILine* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CShapes_CILine* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CShapes_CILine* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CShapes_CILine* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CShapes_CILine* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_X1)(__x_ABI_CWindows_CUI_CXaml_CShapes_CILine* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_X1)(__x_ABI_CWindows_CUI_CXaml_CShapes_CILine* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_Y1)(__x_ABI_CWindows_CUI_CXaml_CShapes_CILine* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_Y1)(__x_ABI_CWindows_CUI_CXaml_CShapes_CILine* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_X2)(__x_ABI_CWindows_CUI_CXaml_CShapes_CILine* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_X2)(__x_ABI_CWindows_CUI_CXaml_CShapes_CILine* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_Y2)(__x_ABI_CWindows_CUI_CXaml_CShapes_CILine* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_Y2)(__x_ABI_CWindows_CUI_CXaml_CShapes_CILine* This,
        DOUBLE value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CShapes_CILineVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CShapes_CILine
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CShapes_CILineVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CILine_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CILine_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CILine_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CILine_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CILine_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CILine_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CILine_get_X1(This, value) \
    ((This)->lpVtbl->get_X1(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CILine_put_X1(This, value) \
    ((This)->lpVtbl->put_X1(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CILine_get_Y1(This, value) \
    ((This)->lpVtbl->get_Y1(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CILine_put_Y1(This, value) \
    ((This)->lpVtbl->put_Y1(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CILine_get_X2(This, value) \
    ((This)->lpVtbl->get_X2(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CILine_put_X2(This, value) \
    ((This)->lpVtbl->put_X2(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CILine_get_Y2(This, value) \
    ((This)->lpVtbl->get_Y2(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CILine_put_Y2(This, value) \
    ((This)->lpVtbl->put_Y2(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CShapes_CILine;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CILine_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Shapes.ILineStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Shapes.Line
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Shapes_ILineStatics[] = L"Windows.UI.Xaml.Shapes.ILineStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CShapes_CILineStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_X1Property)(__x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_Y1Property)(__x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_X2Property)(__x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_Y2Property)(__x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CShapes_CILineStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CShapes_CILineStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics_get_X1Property(This, value) \
    ((This)->lpVtbl->get_X1Property(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics_get_Y1Property(This, value) \
    ((This)->lpVtbl->get_Y1Property(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics_get_X2Property(This, value) \
    ((This)->lpVtbl->get_X2Property(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics_get_Y2Property(This, value) \
    ((This)->lpVtbl->get_Y2Property(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CILineStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Shapes.IPath
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Shapes.Path
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIPath_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPath_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Shapes_IPath[] = L"Windows.UI.Xaml.Shapes.IPath";
typedef struct __x_ABI_CWindows_CUI_CXaml_CShapes_CIPathVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPath* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPath* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPath* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPath* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPath* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPath* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Data)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPath* This,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CIGeometry** value);
    HRESULT (STDMETHODCALLTYPE* put_Data)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPath* This,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CIGeometry* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CShapes_CIPathVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CShapes_CIPath
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CShapes_CIPathVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPath_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPath_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPath_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPath_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPath_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPath_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPath_get_Data(This, value) \
    ((This)->lpVtbl->get_Data(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPath_put_Data(This, value) \
    ((This)->lpVtbl->put_Data(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CShapes_CIPath;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIPath_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Shapes.IPathFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Shapes.Path
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIPathFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPathFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Shapes_IPathFactory[] = L"Windows.UI.Xaml.Shapes.IPathFactory";
typedef struct __x_ABI_CWindows_CUI_CXaml_CShapes_CIPathFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPathFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPathFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPathFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPathFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPathFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPathFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPathFactory* This,
        IInspectable* baseInterface,
        IInspectable** innerInterface,
        __x_ABI_CWindows_CUI_CXaml_CShapes_CIPath** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CShapes_CIPathFactoryVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CShapes_CIPathFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CShapes_CIPathFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPathFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPathFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPathFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPathFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPathFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPathFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPathFactory_CreateInstance(This, baseInterface, innerInterface, value) \
    ((This)->lpVtbl->CreateInstance(This, baseInterface, innerInterface, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CShapes_CIPathFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIPathFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Shapes.IPathStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Shapes.Path
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIPathStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPathStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Shapes_IPathStatics[] = L"Windows.UI.Xaml.Shapes.IPathStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CShapes_CIPathStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPathStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPathStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPathStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPathStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPathStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPathStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DataProperty)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPathStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CShapes_CIPathStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CShapes_CIPathStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CShapes_CIPathStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPathStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPathStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPathStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPathStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPathStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPathStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPathStatics_get_DataProperty(This, value) \
    ((This)->lpVtbl->get_DataProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CShapes_CIPathStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIPathStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Shapes.IPolygon
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Shapes.Polygon
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Shapes_IPolygon[] = L"Windows.UI.Xaml.Shapes.IPolygon";
typedef struct __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FillRule)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon* This,
        enum __x_ABI_CWindows_CUI_CXaml_CMedia_CFillRule* value);
    HRESULT (STDMETHODCALLTYPE* put_FillRule)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon* This,
        enum __x_ABI_CWindows_CUI_CXaml_CMedia_CFillRule value);
    HRESULT (STDMETHODCALLTYPE* get_Points)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon* This,
        __FIVector_1_Windows__CFoundation__CPoint** value);
    HRESULT (STDMETHODCALLTYPE* put_Points)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon* This,
        __FIVector_1_Windows__CFoundation__CPoint* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon_get_FillRule(This, value) \
    ((This)->lpVtbl->get_FillRule(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon_put_FillRule(This, value) \
    ((This)->lpVtbl->put_FillRule(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon_get_Points(This, value) \
    ((This)->lpVtbl->get_Points(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon_put_Points(This, value) \
    ((This)->lpVtbl->put_Points(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygon_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Shapes.IPolygonStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Shapes.Polygon
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Shapes_IPolygonStatics[] = L"Windows.UI.Xaml.Shapes.IPolygonStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FillRuleProperty)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_PointsProperty)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonStatics_get_FillRuleProperty(This, value) \
    ((This)->lpVtbl->get_FillRuleProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonStatics_get_PointsProperty(This, value) \
    ((This)->lpVtbl->get_PointsProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolygonStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Shapes.IPolyline
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Shapes.Polyline
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Shapes_IPolyline[] = L"Windows.UI.Xaml.Shapes.IPolyline";
typedef struct __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FillRule)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline* This,
        enum __x_ABI_CWindows_CUI_CXaml_CMedia_CFillRule* value);
    HRESULT (STDMETHODCALLTYPE* put_FillRule)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline* This,
        enum __x_ABI_CWindows_CUI_CXaml_CMedia_CFillRule value);
    HRESULT (STDMETHODCALLTYPE* get_Points)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline* This,
        __FIVector_1_Windows__CFoundation__CPoint** value);
    HRESULT (STDMETHODCALLTYPE* put_Points)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline* This,
        __FIVector_1_Windows__CFoundation__CPoint* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline_get_FillRule(This, value) \
    ((This)->lpVtbl->get_FillRule(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline_put_FillRule(This, value) \
    ((This)->lpVtbl->put_FillRule(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline_get_Points(This, value) \
    ((This)->lpVtbl->get_Points(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline_put_Points(This, value) \
    ((This)->lpVtbl->put_Points(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolyline_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Shapes.IPolylineStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Shapes.Polyline
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Shapes_IPolylineStatics[] = L"Windows.UI.Xaml.Shapes.IPolylineStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FillRuleProperty)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_PointsProperty)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineStatics_get_FillRuleProperty(This, value) \
    ((This)->lpVtbl->get_FillRuleProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineStatics_get_PointsProperty(This, value) \
    ((This)->lpVtbl->get_PointsProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIPolylineStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Shapes.IRectangle
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Shapes.Rectangle
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Shapes_IRectangle[] = L"Windows.UI.Xaml.Shapes.IRectangle";
typedef struct __x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RadiusX)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_RadiusX)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_RadiusY)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_RadiusY)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle* This,
        DOUBLE value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle_get_RadiusX(This, value) \
    ((This)->lpVtbl->get_RadiusX(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle_put_RadiusX(This, value) \
    ((This)->lpVtbl->put_RadiusX(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle_get_RadiusY(This, value) \
    ((This)->lpVtbl->get_RadiusY(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle_put_RadiusY(This, value) \
    ((This)->lpVtbl->put_RadiusY(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangle_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Shapes.IRectangleStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Shapes.Rectangle
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Shapes_IRectangleStatics[] = L"Windows.UI.Xaml.Shapes.IRectangleStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RadiusXProperty)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_RadiusYProperty)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleStatics_get_RadiusXProperty(This, value) \
    ((This)->lpVtbl->get_RadiusXProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleStatics_get_RadiusYProperty(This, value) \
    ((This)->lpVtbl->get_RadiusYProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIRectangleStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Shapes.IShape
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Shapes.Shape
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Shapes_IShape[] = L"Windows.UI.Xaml.Shapes.IShape";
typedef struct __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShape* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShape* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShape* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShape* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShape* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShape* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Fill)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShape* This,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CIBrush** value);
    HRESULT (STDMETHODCALLTYPE* put_Fill)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShape* This,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CIBrush* value);
    HRESULT (STDMETHODCALLTYPE* get_Stroke)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShape* This,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CIBrush** value);
    HRESULT (STDMETHODCALLTYPE* put_Stroke)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShape* This,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CIBrush* value);
    HRESULT (STDMETHODCALLTYPE* get_StrokeMiterLimit)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShape* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_StrokeMiterLimit)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShape* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_StrokeThickness)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShape* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_StrokeThickness)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShape* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_StrokeStartLineCap)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShape* This,
        enum __x_ABI_CWindows_CUI_CXaml_CMedia_CPenLineCap* value);
    HRESULT (STDMETHODCALLTYPE* put_StrokeStartLineCap)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShape* This,
        enum __x_ABI_CWindows_CUI_CXaml_CMedia_CPenLineCap value);
    HRESULT (STDMETHODCALLTYPE* get_StrokeEndLineCap)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShape* This,
        enum __x_ABI_CWindows_CUI_CXaml_CMedia_CPenLineCap* value);
    HRESULT (STDMETHODCALLTYPE* put_StrokeEndLineCap)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShape* This,
        enum __x_ABI_CWindows_CUI_CXaml_CMedia_CPenLineCap value);
    HRESULT (STDMETHODCALLTYPE* get_StrokeLineJoin)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShape* This,
        enum __x_ABI_CWindows_CUI_CXaml_CMedia_CPenLineJoin* value);
    HRESULT (STDMETHODCALLTYPE* put_StrokeLineJoin)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShape* This,
        enum __x_ABI_CWindows_CUI_CXaml_CMedia_CPenLineJoin value);
    HRESULT (STDMETHODCALLTYPE* get_StrokeDashOffset)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShape* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_StrokeDashOffset)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShape* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_StrokeDashCap)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShape* This,
        enum __x_ABI_CWindows_CUI_CXaml_CMedia_CPenLineCap* value);
    HRESULT (STDMETHODCALLTYPE* put_StrokeDashCap)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShape* This,
        enum __x_ABI_CWindows_CUI_CXaml_CMedia_CPenLineCap value);
    HRESULT (STDMETHODCALLTYPE* get_StrokeDashArray)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShape* This,
        __FIVector_1_double** value);
    HRESULT (STDMETHODCALLTYPE* put_StrokeDashArray)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShape* This,
        __FIVector_1_double* value);
    HRESULT (STDMETHODCALLTYPE* get_Stretch)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShape* This,
        enum __x_ABI_CWindows_CUI_CXaml_CMedia_CStretch* value);
    HRESULT (STDMETHODCALLTYPE* put_Stretch)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShape* This,
        enum __x_ABI_CWindows_CUI_CXaml_CMedia_CStretch value);
    HRESULT (STDMETHODCALLTYPE* get_GeometryTransform)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShape* This,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CITransform** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_get_Fill(This, value) \
    ((This)->lpVtbl->get_Fill(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_put_Fill(This, value) \
    ((This)->lpVtbl->put_Fill(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_get_Stroke(This, value) \
    ((This)->lpVtbl->get_Stroke(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_put_Stroke(This, value) \
    ((This)->lpVtbl->put_Stroke(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_get_StrokeMiterLimit(This, value) \
    ((This)->lpVtbl->get_StrokeMiterLimit(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_put_StrokeMiterLimit(This, value) \
    ((This)->lpVtbl->put_StrokeMiterLimit(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_get_StrokeThickness(This, value) \
    ((This)->lpVtbl->get_StrokeThickness(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_put_StrokeThickness(This, value) \
    ((This)->lpVtbl->put_StrokeThickness(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_get_StrokeStartLineCap(This, value) \
    ((This)->lpVtbl->get_StrokeStartLineCap(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_put_StrokeStartLineCap(This, value) \
    ((This)->lpVtbl->put_StrokeStartLineCap(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_get_StrokeEndLineCap(This, value) \
    ((This)->lpVtbl->get_StrokeEndLineCap(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_put_StrokeEndLineCap(This, value) \
    ((This)->lpVtbl->put_StrokeEndLineCap(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_get_StrokeLineJoin(This, value) \
    ((This)->lpVtbl->get_StrokeLineJoin(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_put_StrokeLineJoin(This, value) \
    ((This)->lpVtbl->put_StrokeLineJoin(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_get_StrokeDashOffset(This, value) \
    ((This)->lpVtbl->get_StrokeDashOffset(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_put_StrokeDashOffset(This, value) \
    ((This)->lpVtbl->put_StrokeDashOffset(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_get_StrokeDashCap(This, value) \
    ((This)->lpVtbl->get_StrokeDashCap(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_put_StrokeDashCap(This, value) \
    ((This)->lpVtbl->put_StrokeDashCap(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_get_StrokeDashArray(This, value) \
    ((This)->lpVtbl->get_StrokeDashArray(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_put_StrokeDashArray(This, value) \
    ((This)->lpVtbl->put_StrokeDashArray(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_get_Stretch(This, value) \
    ((This)->lpVtbl->get_Stretch(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_put_Stretch(This, value) \
    ((This)->lpVtbl->put_Stretch(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_get_GeometryTransform(This, value) \
    ((This)->lpVtbl->get_GeometryTransform(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CShapes_CIShape;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIShape_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Shapes.IShape2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Shapes.Shape
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIShape2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIShape2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Shapes_IShape2[] = L"Windows.UI.Xaml.Shapes.IShape2";
typedef struct __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShape2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShape2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShape2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShape2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShape2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShape2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAlphaMask)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShape2* This,
        __x_ABI_CWindows_CUI_CComposition_CICompositionBrush** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape2Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape2_GetAlphaMask(This, result) \
    ((This)->lpVtbl->GetAlphaMask(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CShapes_CIShape2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIShape2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Shapes.IShapeFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Shapes.Shape
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Shapes_IShapeFactory[] = L"Windows.UI.Xaml.Shapes.IShapeFactory";
typedef struct __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeFactory* This,
        IInspectable* baseInterface,
        IInspectable** innerInterface,
        __x_ABI_CWindows_CUI_CXaml_CShapes_CIShape** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeFactoryVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeFactory_CreateInstance(This, baseInterface, innerInterface, value) \
    ((This)->lpVtbl->CreateInstance(This, baseInterface, innerInterface, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Shapes.IShapeStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Shapes.Shape
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Shapes_IShapeStatics[] = L"Windows.UI.Xaml.Shapes.IShapeStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FillProperty)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_StrokeProperty)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_StrokeMiterLimitProperty)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_StrokeThicknessProperty)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_StrokeStartLineCapProperty)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_StrokeEndLineCapProperty)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_StrokeLineJoinProperty)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_StrokeDashOffsetProperty)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_StrokeDashCapProperty)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_StrokeDashArrayProperty)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_StretchProperty)(__x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics_get_FillProperty(This, value) \
    ((This)->lpVtbl->get_FillProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics_get_StrokeProperty(This, value) \
    ((This)->lpVtbl->get_StrokeProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics_get_StrokeMiterLimitProperty(This, value) \
    ((This)->lpVtbl->get_StrokeMiterLimitProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics_get_StrokeThicknessProperty(This, value) \
    ((This)->lpVtbl->get_StrokeThicknessProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics_get_StrokeStartLineCapProperty(This, value) \
    ((This)->lpVtbl->get_StrokeStartLineCapProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics_get_StrokeEndLineCapProperty(This, value) \
    ((This)->lpVtbl->get_StrokeEndLineCapProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics_get_StrokeLineJoinProperty(This, value) \
    ((This)->lpVtbl->get_StrokeLineJoinProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics_get_StrokeDashOffsetProperty(This, value) \
    ((This)->lpVtbl->get_StrokeDashOffsetProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics_get_StrokeDashCapProperty(This, value) \
    ((This)->lpVtbl->get_StrokeDashCapProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics_get_StrokeDashArrayProperty(This, value) \
    ((This)->lpVtbl->get_StrokeDashArrayProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics_get_StretchProperty(This, value) \
    ((This)->lpVtbl->get_StretchProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CShapes_CIShapeStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Shapes.Ellipse
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Shapes.IEllipse ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Shapes_Ellipse_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Shapes_Ellipse_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Shapes_Ellipse[] = L"Windows.UI.Xaml.Shapes.Ellipse";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Shapes.Line
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Shapes.ILineStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Shapes.ILine ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Shapes_Line_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Shapes_Line_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Shapes_Line[] = L"Windows.UI.Xaml.Shapes.Line";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Shapes.Path
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Shapes.IPathStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Shapes.IPath ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Shapes_Path_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Shapes_Path_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Shapes_Path[] = L"Windows.UI.Xaml.Shapes.Path";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Shapes.Polygon
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Shapes.IPolygonStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Shapes.IPolygon ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Shapes_Polygon_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Shapes_Polygon_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Shapes_Polygon[] = L"Windows.UI.Xaml.Shapes.Polygon";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Shapes.Polyline
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Shapes.IPolylineStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Shapes.IPolyline ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Shapes_Polyline_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Shapes_Polyline_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Shapes_Polyline[] = L"Windows.UI.Xaml.Shapes.Polyline";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Shapes.Rectangle
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Shapes.IRectangleStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Shapes.IRectangle ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Shapes_Rectangle_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Shapes_Rectangle_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Shapes_Rectangle[] = L"Windows.UI.Xaml.Shapes.Rectangle";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Shapes.Shape
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Shapes.IShapeStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Shapes.IShape ** Default Interface **
 *    Windows.UI.Xaml.Shapes.IShape2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Shapes_Shape_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Shapes_Shape_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Shapes_Shape[] = L"Windows.UI.Xaml.Shapes.Shape";
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
#endif // __windows2Eui2Examl2Eshapes_p_h__

#endif // __windows2Eui2Examl2Eshapes_h__
