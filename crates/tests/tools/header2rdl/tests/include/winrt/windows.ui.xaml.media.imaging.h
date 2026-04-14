
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
#ifndef __windows2Eui2Examl2Emedia2Eimaging_h__
#define __windows2Eui2Examl2Emedia2Eimaging_h__
#ifndef __windows2Eui2Examl2Emedia2Eimaging_p_h__
#define __windows2Eui2Examl2Emedia2Eimaging_p_h__


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
#if !defined(WINDOWS_APPLICATIONMODEL_BACKGROUND_BACKGROUNDALARMAPPLICATIONCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_BACKGROUND_BACKGROUNDALARMAPPLICATIONCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_APPLICATIONMODEL_BACKGROUND_BACKGROUNDALARMAPPLICATIONCONTRACT_VERSION)

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
#include "Windows.ApplicationModel.Background.h"
#include "Windows.Graphics.Imaging.h"
#include "Windows.Storage.Streams.h"
#include "Windows.UI.Xaml.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        interface IDownloadProgressEventHandler;
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventHandler ABI::Windows::UI::Xaml::Media::Imaging::IDownloadProgressEventHandler

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        interface IBitmapImage;
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage ABI::Windows::UI::Xaml::Media::Imaging::IBitmapImage

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        interface IBitmapImage2;
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage2 ABI::Windows::UI::Xaml::Media::Imaging::IBitmapImage2

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        interface IBitmapImage3;
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3 ABI::Windows::UI::Xaml::Media::Imaging::IBitmapImage3

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        interface IBitmapImageFactory;
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageFactory ABI::Windows::UI::Xaml::Media::Imaging::IBitmapImageFactory

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        interface IBitmapImageStatics;
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics ABI::Windows::UI::Xaml::Media::Imaging::IBitmapImageStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        interface IBitmapImageStatics2;
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics2 ABI::Windows::UI::Xaml::Media::Imaging::IBitmapImageStatics2

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        interface IBitmapImageStatics3;
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3 ABI::Windows::UI::Xaml::Media::Imaging::IBitmapImageStatics3

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        interface IBitmapSource;
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource ABI::Windows::UI::Xaml::Media::Imaging::IBitmapSource

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        interface IBitmapSourceFactory;
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceFactory ABI::Windows::UI::Xaml::Media::Imaging::IBitmapSourceFactory

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        interface IBitmapSourceStatics;
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceStatics ABI::Windows::UI::Xaml::Media::Imaging::IBitmapSourceStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        interface IDownloadProgressEventArgs;
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgs ABI::Windows::UI::Xaml::Media::Imaging::IDownloadProgressEventArgs

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        interface IRenderTargetBitmap;
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap ABI::Windows::UI::Xaml::Media::Imaging::IRenderTargetBitmap

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        interface IRenderTargetBitmapStatics;
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapStatics ABI::Windows::UI::Xaml::Media::Imaging::IRenderTargetBitmapStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISoftwareBitmapSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISoftwareBitmapSource_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        interface ISoftwareBitmapSource;
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISoftwareBitmapSource ABI::Windows::UI::Xaml::Media::Imaging::ISoftwareBitmapSource

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISoftwareBitmapSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSource_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        interface ISurfaceImageSource;
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSource ABI::Windows::UI::Xaml::Media::Imaging::ISurfaceImageSource

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        interface ISurfaceImageSourceFactory;
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceFactory ABI::Windows::UI::Xaml::Media::Imaging::ISurfaceImageSourceFactory

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        interface ISvgImageSource;
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource ABI::Windows::UI::Xaml::Media::Imaging::ISvgImageSource

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        interface ISvgImageSourceFactory;
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFactory ABI::Windows::UI::Xaml::Media::Imaging::ISvgImageSourceFactory

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFailedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFailedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        interface ISvgImageSourceFailedEventArgs;
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFailedEventArgs ABI::Windows::UI::Xaml::Media::Imaging::ISvgImageSourceFailedEventArgs

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFailedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceOpenedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceOpenedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        interface ISvgImageSourceOpenedEventArgs;
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceOpenedEventArgs ABI::Windows::UI::Xaml::Media::Imaging::ISvgImageSourceOpenedEventArgs

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceOpenedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        interface ISvgImageSourceStatics;
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStatics ABI::Windows::UI::Xaml::Media::Imaging::ISvgImageSourceStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSource_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        interface IVirtualSurfaceImageSource;
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSource ABI::Windows::UI::Xaml::Media::Imaging::IVirtualSurfaceImageSource

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        interface IVirtualSurfaceImageSourceFactory;
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceFactory ABI::Windows::UI::Xaml::Media::Imaging::IVirtualSurfaceImageSourceFactory

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmap_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmap_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        interface IWriteableBitmap;
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmap ABI::Windows::UI::Xaml::Media::Imaging::IWriteableBitmap

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmap_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        interface IWriteableBitmapFactory;
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapFactory ABI::Windows::UI::Xaml::Media::Imaging::IWriteableBitmapFactory

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTask_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTask_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        interface IXamlRenderingBackgroundTask;
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTask ABI::Windows::UI::Xaml::Media::Imaging::IXamlRenderingBackgroundTask

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTask_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        interface IXamlRenderingBackgroundTaskFactory;
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskFactory ABI::Windows::UI::Xaml::Media::Imaging::IXamlRenderingBackgroundTaskFactory

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskOverrides_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskOverrides_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        interface IXamlRenderingBackgroundTaskOverrides;
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskOverrides ABI::Windows::UI::Xaml::Media::Imaging::IXamlRenderingBackgroundTaskOverrides

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskOverrides_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3bee8834-b9a7-5a80-a746-5ef097227878"))
IAsyncOperation<ABI::Windows::Storage::Streams::IBuffer*> : IAsyncOperation_impl<ABI::Windows::Storage::Streams::IBuffer*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.Streams.IBuffer>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::Streams::IBuffer*> __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_t;
#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("51c3d2fd-b8a1-5620-b746-7ee6d533aca3"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IBuffer*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Storage::Streams::IBuffer*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.Streams.IBuffer>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IBuffer*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        typedef enum SvgImageSourceLoadStatus : int SvgImageSourceLoadStatus;
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperation_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus_USE
#define DEF___FIAsyncOperation_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f19df5c2-2b78-53a9-8d38-5ca8dbb5dbc6"))
IAsyncOperation<enum ABI::Windows::UI::Xaml::Media::Imaging::SvgImageSourceLoadStatus> : IAsyncOperation_impl<enum ABI::Windows::UI::Xaml::Media::Imaging::SvgImageSourceLoadStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.UI.Xaml.Media.Imaging.SvgImageSourceLoadStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::UI::Xaml::Media::Imaging::SvgImageSourceLoadStatus> __FIAsyncOperation_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus_t;
#define __FIAsyncOperation_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7c8bc668-4e0b-5924-b7e7-234a11d63d61"))
IAsyncOperationCompletedHandler<enum ABI::Windows::UI::Xaml::Media::Imaging::SvgImageSourceLoadStatus> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::UI::Xaml::Media::Imaging::SvgImageSourceLoadStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.UI.Xaml.Media.Imaging.SvgImageSourceLoadStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::UI::Xaml::Media::Imaging::SvgImageSourceLoadStatus> __FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        class SvgImageSource;
                    } /* Imaging */
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
                    namespace Imaging {
                        class SvgImageSourceFailedEventArgs;
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceFailedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceFailedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d723938d-efee-5a0c-ab3e-1e1db3c9a216"))
ITypedEventHandler<ABI::Windows::UI::Xaml::Media::Imaging::SvgImageSource*, ABI::Windows::UI::Xaml::Media::Imaging::SvgImageSourceFailedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Media::Imaging::SvgImageSource*, ABI::Windows::UI::Xaml::Media::Imaging::ISvgImageSource*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Media::Imaging::SvgImageSourceFailedEventArgs*, ABI::Windows::UI::Xaml::Media::Imaging::ISvgImageSourceFailedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Xaml.Media.Imaging.SvgImageSource, Windows.UI.Xaml.Media.Imaging.SvgImageSourceFailedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Xaml::Media::Imaging::SvgImageSource*, ABI::Windows::UI::Xaml::Media::Imaging::SvgImageSourceFailedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceFailedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceFailedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceFailedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceFailedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        class SvgImageSourceOpenedEventArgs;
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceOpenedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceOpenedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9e321151-8b27-5e18-9ea3-50520aceb7ef"))
ITypedEventHandler<ABI::Windows::UI::Xaml::Media::Imaging::SvgImageSource*, ABI::Windows::UI::Xaml::Media::Imaging::SvgImageSourceOpenedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Media::Imaging::SvgImageSource*, ABI::Windows::UI::Xaml::Media::Imaging::ISvgImageSource*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Media::Imaging::SvgImageSourceOpenedEventArgs*, ABI::Windows::UI::Xaml::Media::Imaging::ISvgImageSourceOpenedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Xaml.Media.Imaging.SvgImageSource, Windows.UI.Xaml.Media.Imaging.SvgImageSourceOpenedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Xaml::Media::Imaging::SvgImageSource*, ABI::Windows::UI::Xaml::Media::Imaging::SvgImageSourceOpenedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceOpenedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceOpenedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceOpenedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceOpenedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef ____x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTaskInstance_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTaskInstance_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Background {
                interface IBackgroundTaskInstance;
            } /* Background */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTaskInstance ABI::Windows::ApplicationModel::Background::IBackgroundTaskInstance

#endif // ____x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTaskInstance_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IClosable;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIClosable ABI::Windows::Foundation::IClosable

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

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
        namespace Graphics {
            namespace Imaging {
                class SoftwareBitmap;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                interface ISoftwareBitmap;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap ABI::Windows::Graphics::Imaging::ISoftwareBitmap

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIExceptionRoutedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIExceptionRoutedEventHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                interface IExceptionRoutedEventHandler;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CIExceptionRoutedEventHandler ABI::Windows::UI::Xaml::IExceptionRoutedEventHandler

#endif // ____x_ABI_CWindows_CUI_CXaml_CIExceptionRoutedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIRoutedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIRoutedEventHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                interface IRoutedEventHandler;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CIRoutedEventHandler ABI::Windows::UI::Xaml::IRoutedEventHandler

#endif // ____x_ABI_CWindows_CUI_CXaml_CIRoutedEventHandler_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                class UIElement;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIUIElement_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIUIElement_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                interface IUIElement;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CIUIElement ABI::Windows::UI::Xaml::IUIElement

#endif // ____x_ABI_CWindows_CUI_CXaml_CIUIElement_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        typedef enum BitmapCreateOptions : unsigned int BitmapCreateOptions;
                    } /* Imaging */
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
                    namespace Imaging {
                        typedef enum DecodePixelType : int DecodePixelType;
                    } /* Imaging */
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
                    namespace Imaging {
                        class BitmapImage;
                    } /* Imaging */
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
                    namespace Imaging {
                        class BitmapSource;
                    } /* Imaging */
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
                    namespace Imaging {
                        class DownloadProgressEventArgs;
                    } /* Imaging */
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
                    namespace Imaging {
                        class SurfaceImageSource;
                    } /* Imaging */
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
                    namespace Imaging {
                        class VirtualSurfaceImageSource;
                    } /* Imaging */
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
                    namespace Imaging {
                        class WriteableBitmap;
                    } /* Imaging */
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
                    namespace Imaging {
                        class XamlRenderingBackgroundTask;
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.UI.Xaml.Media.Imaging.BitmapCreateOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        enum BitmapCreateOptions : unsigned int
                        {
                            BitmapCreateOptions_None = 0,
                            BitmapCreateOptions_IgnoreImageCache = 0x8,
                        };

                        DEFINE_ENUM_FLAG_OPERATORS(BitmapCreateOptions)
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Media.Imaging.DecodePixelType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        enum DecodePixelType : int
                        {
                            DecodePixelType_Physical = 0,
                            DecodePixelType_Logical = 1,
                        };
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Media.Imaging.SvgImageSourceLoadStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        enum SvgImageSourceLoadStatus : int
                        {
                            SvgImageSourceLoadStatus_Success = 0,
                            SvgImageSourceLoadStatus_NetworkError = 1,
                            SvgImageSourceLoadStatus_InvalidFormat = 2,
                            SvgImageSourceLoadStatus_Other = 3,
                        };
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Delegate Windows.UI.Xaml.Media.Imaging.DownloadProgressEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        MIDL_INTERFACE("1abaee23-74ee-4cc7-99ba-b171e3cda61e")
                        IDownloadProgressEventHandler : public IUnknown
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE Invoke(
                                IInspectable* sender,
                                ABI::Windows::UI::Xaml::Media::Imaging::IDownloadProgressEventArgs* e
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IDownloadProgressEventHandler = __uuidof(IDownloadProgressEventHandler);
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IBitmapImage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.BitmapImage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IBitmapImage[] = L"Windows.UI.Xaml.Media.Imaging.IBitmapImage";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        MIDL_INTERFACE("31af3271-e3b4-442d-a341-4c0226b2725b")
                        IBitmapImage : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_CreateOptions(
                                ABI::Windows::UI::Xaml::Media::Imaging::BitmapCreateOptions* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_CreateOptions(
                                ABI::Windows::UI::Xaml::Media::Imaging::BitmapCreateOptions value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_UriSource(
                                ABI::Windows::Foundation::IUriRuntimeClass** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_UriSource(
                                ABI::Windows::Foundation::IUriRuntimeClass* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_DecodePixelWidth(
                                INT32* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_DecodePixelWidth(
                                INT32 value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_DecodePixelHeight(
                                INT32* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_DecodePixelHeight(
                                INT32 value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_DownloadProgress(
                                ABI::Windows::UI::Xaml::Media::Imaging::IDownloadProgressEventHandler* handler,
                                EventRegistrationToken* token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_DownloadProgress(
                                EventRegistrationToken token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_ImageOpened(
                                ABI::Windows::UI::Xaml::IRoutedEventHandler* handler,
                                EventRegistrationToken* token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_ImageOpened(
                                EventRegistrationToken token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_ImageFailed(
                                ABI::Windows::UI::Xaml::IExceptionRoutedEventHandler* handler,
                                EventRegistrationToken* token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_ImageFailed(
                                EventRegistrationToken token
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IBitmapImage = __uuidof(IBitmapImage);
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IBitmapImage2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.BitmapImage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IBitmapImage2[] = L"Windows.UI.Xaml.Media.Imaging.IBitmapImage2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        MIDL_INTERFACE("1069c1b6-8c9b-4762-be3d-759f5698f2b3")
                        IBitmapImage2 : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_DecodePixelType(
                                ABI::Windows::UI::Xaml::Media::Imaging::DecodePixelType* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_DecodePixelType(
                                ABI::Windows::UI::Xaml::Media::Imaging::DecodePixelType value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IBitmapImage2 = __uuidof(IBitmapImage2);
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IBitmapImage3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.BitmapImage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IBitmapImage3[] = L"Windows.UI.Xaml.Media.Imaging.IBitmapImage3";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        MIDL_INTERFACE("f1de6f26-3c73-453f-a7ba-9b85c18b3733")
                        IBitmapImage3 : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_IsAnimatedBitmap(
                                boolean* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_IsPlaying(
                                boolean* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_AutoPlay(
                                boolean* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_AutoPlay(
                                boolean value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE Play(void) = 0;
                            virtual HRESULT STDMETHODCALLTYPE Stop(void) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IBitmapImage3 = __uuidof(IBitmapImage3);
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IBitmapImageFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.BitmapImage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IBitmapImageFactory[] = L"Windows.UI.Xaml.Media.Imaging.IBitmapImageFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        MIDL_INTERFACE("c9132978-4810-4e5e-8087-03671ee60d85")
                        IBitmapImageFactory : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE CreateInstanceWithUriSource(
                                ABI::Windows::Foundation::IUriRuntimeClass* uriSource,
                                ABI::Windows::UI::Xaml::Media::Imaging::IBitmapImage** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IBitmapImageFactory = __uuidof(IBitmapImageFactory);
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IBitmapImageStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.BitmapImage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IBitmapImageStatics[] = L"Windows.UI.Xaml.Media.Imaging.IBitmapImageStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        MIDL_INTERFACE("9e282143-70e8-437c-9fa4-2cbf295cff84")
                        IBitmapImageStatics : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_CreateOptionsProperty(
                                ABI::Windows::UI::Xaml::IDependencyProperty** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_UriSourceProperty(
                                ABI::Windows::UI::Xaml::IDependencyProperty** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_DecodePixelWidthProperty(
                                ABI::Windows::UI::Xaml::IDependencyProperty** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_DecodePixelHeightProperty(
                                ABI::Windows::UI::Xaml::IDependencyProperty** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IBitmapImageStatics = __uuidof(IBitmapImageStatics);
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IBitmapImageStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.BitmapImage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IBitmapImageStatics2[] = L"Windows.UI.Xaml.Media.Imaging.IBitmapImageStatics2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        MIDL_INTERFACE("c5f5576a-75af-41a4-b893-8fe91fee2882")
                        IBitmapImageStatics2 : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_DecodePixelTypeProperty(
                                ABI::Windows::UI::Xaml::IDependencyProperty** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IBitmapImageStatics2 = __uuidof(IBitmapImageStatics2);
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IBitmapImageStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.BitmapImage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IBitmapImageStatics3[] = L"Windows.UI.Xaml.Media.Imaging.IBitmapImageStatics3";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        MIDL_INTERFACE("2b44e30d-f6d5-4411-a8cd-bf7603c4faa0")
                        IBitmapImageStatics3 : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_IsAnimatedBitmapProperty(
                                ABI::Windows::UI::Xaml::IDependencyProperty** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_IsPlayingProperty(
                                ABI::Windows::UI::Xaml::IDependencyProperty** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_AutoPlayProperty(
                                ABI::Windows::UI::Xaml::IDependencyProperty** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IBitmapImageStatics3 = __uuidof(IBitmapImageStatics3);
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IBitmapSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.BitmapSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IBitmapSource[] = L"Windows.UI.Xaml.Media.Imaging.IBitmapSource";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        MIDL_INTERFACE("23d86411-202f-41b2-8c5b-a8a3b333800b")
                        IBitmapSource : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_PixelWidth(
                                INT32* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_PixelHeight(
                                INT32* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SetSource(
                                ABI::Windows::Storage::Streams::IRandomAccessStream* streamSource
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SetSourceAsync(
                                ABI::Windows::Storage::Streams::IRandomAccessStream* streamSource,
                                ABI::Windows::Foundation::IAsyncAction** operation
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IBitmapSource = __uuidof(IBitmapSource);
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IBitmapSourceFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.BitmapSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IBitmapSourceFactory[] = L"Windows.UI.Xaml.Media.Imaging.IBitmapSourceFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        MIDL_INTERFACE("e240420e-d4a7-49a4-a0b4-a59fdd77e508")
                        IBitmapSourceFactory : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                                IInspectable* baseInterface,
                                IInspectable** innerInterface,
                                ABI::Windows::UI::Xaml::Media::Imaging::IBitmapSource** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IBitmapSourceFactory = __uuidof(IBitmapSourceFactory);
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IBitmapSourceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.BitmapSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IBitmapSourceStatics[] = L"Windows.UI.Xaml.Media.Imaging.IBitmapSourceStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        MIDL_INTERFACE("9a9c9981-827b-4e51-891b-8a15b511842d")
                        IBitmapSourceStatics : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_PixelWidthProperty(
                                ABI::Windows::UI::Xaml::IDependencyProperty** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_PixelHeightProperty(
                                ABI::Windows::UI::Xaml::IDependencyProperty** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IBitmapSourceStatics = __uuidof(IBitmapSourceStatics);
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IDownloadProgressEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.DownloadProgressEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IDownloadProgressEventArgs[] = L"Windows.UI.Xaml.Media.Imaging.IDownloadProgressEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        MIDL_INTERFACE("7311e0d4-fe94-4e70-9b90-cdd47ac23afb")
                        IDownloadProgressEventArgs : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_Progress(
                                INT32* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_Progress(
                                INT32 value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IDownloadProgressEventArgs = __uuidof(IDownloadProgressEventArgs);
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IRenderTargetBitmap
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.RenderTargetBitmap
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IRenderTargetBitmap[] = L"Windows.UI.Xaml.Media.Imaging.IRenderTargetBitmap";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        MIDL_INTERFACE("500dee81-893c-4c0a-8fec-4678ac717589")
                        IRenderTargetBitmap : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_PixelWidth(
                                INT32* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_PixelHeight(
                                INT32* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE RenderAsync(
                                ABI::Windows::UI::Xaml::IUIElement* element,
                                ABI::Windows::Foundation::IAsyncAction** operation
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE RenderToSizeAsync(
                                ABI::Windows::UI::Xaml::IUIElement* element,
                                INT32 scaledWidth,
                                INT32 scaledHeight,
                                ABI::Windows::Foundation::IAsyncAction** operation
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetPixelsAsync(
                                __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer** operation
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IRenderTargetBitmap = __uuidof(IRenderTargetBitmap);
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IRenderTargetBitmapStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.RenderTargetBitmap
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IRenderTargetBitmapStatics[] = L"Windows.UI.Xaml.Media.Imaging.IRenderTargetBitmapStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        MIDL_INTERFACE("f0a1efee-c131-4d40-9c47-f7d7cf2b077f")
                        IRenderTargetBitmapStatics : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_PixelWidthProperty(
                                ABI::Windows::UI::Xaml::IDependencyProperty** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_PixelHeightProperty(
                                ABI::Windows::UI::Xaml::IDependencyProperty** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IRenderTargetBitmapStatics = __uuidof(IRenderTargetBitmapStatics);
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.ISoftwareBitmapSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.SoftwareBitmapSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISoftwareBitmapSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISoftwareBitmapSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_ISoftwareBitmapSource[] = L"Windows.UI.Xaml.Media.Imaging.ISoftwareBitmapSource";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        MIDL_INTERFACE("d2dd9ed0-d3c5-4056-91b5-b7c1d1e8130e")
                        ISoftwareBitmapSource : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE SetBitmapAsync(
                                ABI::Windows::Graphics::Imaging::ISoftwareBitmap* softwareBitmap,
                                ABI::Windows::Foundation::IAsyncAction** operation
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ISoftwareBitmapSource = __uuidof(ISoftwareBitmapSource);
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISoftwareBitmapSource;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISoftwareBitmapSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.ISurfaceImageSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.SurfaceImageSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_ISurfaceImageSource[] = L"Windows.UI.Xaml.Media.Imaging.ISurfaceImageSource";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        MIDL_INTERFACE("62f7d416-c714-4c4c-8273-f839bc58135c")
                        ISurfaceImageSource : public IInspectable
                        {
                        public:
                        };

                        MIDL_CONST_ID IID& IID_ISurfaceImageSource = __uuidof(ISurfaceImageSource);
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSource;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.ISurfaceImageSourceFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.SurfaceImageSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_ISurfaceImageSourceFactory[] = L"Windows.UI.Xaml.Media.Imaging.ISurfaceImageSourceFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        MIDL_INTERFACE("3ab2212a-ef65-4a5f-bfac-73993e8c12c9")
                        ISurfaceImageSourceFactory : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE CreateInstanceWithDimensions(
                                INT32 pixelWidth,
                                INT32 pixelHeight,
                                IInspectable* baseInterface,
                                IInspectable** innerInterface,
                                ABI::Windows::UI::Xaml::Media::Imaging::ISurfaceImageSource** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE CreateInstanceWithDimensionsAndOpacity(
                                INT32 pixelWidth,
                                INT32 pixelHeight,
                                boolean isOpaque,
                                IInspectable* baseInterface,
                                IInspectable** innerInterface,
                                ABI::Windows::UI::Xaml::Media::Imaging::ISurfaceImageSource** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ISurfaceImageSourceFactory = __uuidof(ISurfaceImageSourceFactory);
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.ISvgImageSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.SvgImageSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_ISvgImageSource[] = L"Windows.UI.Xaml.Media.Imaging.ISvgImageSource";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        MIDL_INTERFACE("03e1cec3-0ca8-4a4e-8d7c-c808a0838586")
                        ISvgImageSource : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_UriSource(
                                ABI::Windows::Foundation::IUriRuntimeClass** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_UriSource(
                                ABI::Windows::Foundation::IUriRuntimeClass* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_RasterizePixelWidth(
                                DOUBLE* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_RasterizePixelWidth(
                                DOUBLE value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_RasterizePixelHeight(
                                DOUBLE* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_RasterizePixelHeight(
                                DOUBLE value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_Opened(
                                __FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceOpenedEventArgs* handler,
                                EventRegistrationToken* token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_Opened(
                                EventRegistrationToken token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_OpenFailed(
                                __FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceFailedEventArgs* handler,
                                EventRegistrationToken* token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_OpenFailed(
                                EventRegistrationToken token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SetSourceAsync(
                                ABI::Windows::Storage::Streams::IRandomAccessStream* streamSource,
                                __FIAsyncOperation_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus** operation
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ISvgImageSource = __uuidof(ISvgImageSource);
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.ISvgImageSourceFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.SvgImageSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_ISvgImageSourceFactory[] = L"Windows.UI.Xaml.Media.Imaging.ISvgImageSourceFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        MIDL_INTERFACE("c794e9e7-cf23-4d72-bf1a-dfaa16d8ea52")
                        ISvgImageSourceFactory : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                                IInspectable* baseInterface,
                                IInspectable** innerInterface,
                                ABI::Windows::UI::Xaml::Media::Imaging::ISvgImageSource** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE CreateInstanceWithUriSource(
                                ABI::Windows::Foundation::IUriRuntimeClass* uriSource,
                                IInspectable* baseInterface,
                                IInspectable** innerInterface,
                                ABI::Windows::UI::Xaml::Media::Imaging::ISvgImageSource** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ISvgImageSourceFactory = __uuidof(ISvgImageSourceFactory);
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.ISvgImageSourceFailedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.SvgImageSourceFailedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFailedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFailedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_ISvgImageSourceFailedEventArgs[] = L"Windows.UI.Xaml.Media.Imaging.ISvgImageSourceFailedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        MIDL_INTERFACE("68bb3170-3ccc-4035-ac01-9834543d744e")
                        ISvgImageSourceFailedEventArgs : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_Status(
                                ABI::Windows::UI::Xaml::Media::Imaging::SvgImageSourceLoadStatus* value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ISvgImageSourceFailedEventArgs = __uuidof(ISvgImageSourceFailedEventArgs);
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFailedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFailedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.ISvgImageSourceOpenedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.SvgImageSourceOpenedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceOpenedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceOpenedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_ISvgImageSourceOpenedEventArgs[] = L"Windows.UI.Xaml.Media.Imaging.ISvgImageSourceOpenedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        MIDL_INTERFACE("85ef4c16-748e-4008-95c7-6a23dd7316db")
                        ISvgImageSourceOpenedEventArgs : public IInspectable
                        {
                        public:
                        };

                        MIDL_CONST_ID IID& IID_ISvgImageSourceOpenedEventArgs = __uuidof(ISvgImageSourceOpenedEventArgs);
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceOpenedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceOpenedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.ISvgImageSourceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.SvgImageSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_ISvgImageSourceStatics[] = L"Windows.UI.Xaml.Media.Imaging.ISvgImageSourceStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        MIDL_INTERFACE("9c6638ce-bed1-4aab-acbb-d3e2185d315a")
                        ISvgImageSourceStatics : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_UriSourceProperty(
                                ABI::Windows::UI::Xaml::IDependencyProperty** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_RasterizePixelWidthProperty(
                                ABI::Windows::UI::Xaml::IDependencyProperty** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_RasterizePixelHeightProperty(
                                ABI::Windows::UI::Xaml::IDependencyProperty** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ISvgImageSourceStatics = __uuidof(ISvgImageSourceStatics);
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IVirtualSurfaceImageSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.VirtualSurfaceImageSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IVirtualSurfaceImageSource[] = L"Windows.UI.Xaml.Media.Imaging.IVirtualSurfaceImageSource";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        MIDL_INTERFACE("4a711fea-bfac-11e0-a06a-9de44724019b")
                        IVirtualSurfaceImageSource : public IInspectable
                        {
                        public:
                        };

                        MIDL_CONST_ID IID& IID_IVirtualSurfaceImageSource = __uuidof(IVirtualSurfaceImageSource);
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSource;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IVirtualSurfaceImageSourceFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.VirtualSurfaceImageSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IVirtualSurfaceImageSourceFactory[] = L"Windows.UI.Xaml.Media.Imaging.IVirtualSurfaceImageSourceFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        MIDL_INTERFACE("3ab2212a-bfac-11e0-8a92-69e44724019b")
                        IVirtualSurfaceImageSourceFactory : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE CreateInstanceWithDimensions(
                                INT32 pixelWidth,
                                INT32 pixelHeight,
                                ABI::Windows::UI::Xaml::Media::Imaging::IVirtualSurfaceImageSource** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE CreateInstanceWithDimensionsAndOpacity(
                                INT32 pixelWidth,
                                INT32 pixelHeight,
                                boolean isOpaque,
                                ABI::Windows::UI::Xaml::Media::Imaging::IVirtualSurfaceImageSource** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IVirtualSurfaceImageSourceFactory = __uuidof(IVirtualSurfaceImageSourceFactory);
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IWriteableBitmap
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.WriteableBitmap
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmap_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmap_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IWriteableBitmap[] = L"Windows.UI.Xaml.Media.Imaging.IWriteableBitmap";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        MIDL_INTERFACE("bf0b7e6f-df7c-4a85-8413-a1216285835c")
                        IWriteableBitmap : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_PixelBuffer(
                                ABI::Windows::Storage::Streams::IBuffer** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE Invalidate(void) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWriteableBitmap = __uuidof(IWriteableBitmap);
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmap;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmap_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IWriteableBitmapFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.WriteableBitmap
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IWriteableBitmapFactory[] = L"Windows.UI.Xaml.Media.Imaging.IWriteableBitmapFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        MIDL_INTERFACE("5563ebb1-3ef2-42c5-9c6d-1cf5dcc041ff")
                        IWriteableBitmapFactory : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE CreateInstanceWithDimensions(
                                INT32 pixelWidth,
                                INT32 pixelHeight,
                                ABI::Windows::UI::Xaml::Media::Imaging::IWriteableBitmap** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWriteableBitmapFactory = __uuidof(IWriteableBitmapFactory);
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IXamlRenderingBackgroundTask
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.XamlRenderingBackgroundTask
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTask_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTask_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IXamlRenderingBackgroundTask[] = L"Windows.UI.Xaml.Media.Imaging.IXamlRenderingBackgroundTask";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        MIDL_INTERFACE("5d5fe9aa-533e-44b8-a975-fc5f1e3bff52")
                        IXamlRenderingBackgroundTask : public IInspectable
                        {
                        public:
                        };

                        MIDL_CONST_ID IID& IID_IXamlRenderingBackgroundTask = __uuidof(IXamlRenderingBackgroundTask);
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTask;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTask_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IXamlRenderingBackgroundTaskFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.XamlRenderingBackgroundTask
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IXamlRenderingBackgroundTaskFactory[] = L"Windows.UI.Xaml.Media.Imaging.IXamlRenderingBackgroundTaskFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        MIDL_INTERFACE("a3d1bb63-38f8-4da3-9fca-fd8128a2cbf9")
                        IXamlRenderingBackgroundTaskFactory : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                                IInspectable* baseInterface,
                                IInspectable** innerInterface,
                                ABI::Windows::UI::Xaml::Media::Imaging::IXamlRenderingBackgroundTask** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IXamlRenderingBackgroundTaskFactory = __uuidof(IXamlRenderingBackgroundTaskFactory);
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IXamlRenderingBackgroundTaskOverrides
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.XamlRenderingBackgroundTask
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskOverrides_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskOverrides_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IXamlRenderingBackgroundTaskOverrides[] = L"Windows.UI.Xaml.Media.Imaging.IXamlRenderingBackgroundTaskOverrides";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    namespace Imaging {
                        MIDL_INTERFACE("9c2a6997-a908-4711-b4b2-a960db3d8e5a")
                        IXamlRenderingBackgroundTaskOverrides : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE OnRun(
                                ABI::Windows::ApplicationModel::Background::IBackgroundTaskInstance* taskInstance
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IXamlRenderingBackgroundTaskOverrides = __uuidof(IXamlRenderingBackgroundTaskOverrides);
                    } /* Imaging */
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskOverrides;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskOverrides_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Media.Imaging.BitmapImage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Xaml.Media.Imaging.IBitmapImageFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Media.Imaging.IBitmapImageStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Media.Imaging.IBitmapImageStatics3 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Media.Imaging.IBitmapImageStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Media.Imaging.IBitmapImage ** Default Interface **
 *    Windows.UI.Xaml.Media.Imaging.IBitmapImage2
 *    Windows.UI.Xaml.Media.Imaging.IBitmapImage3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_BitmapImage_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_BitmapImage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Media_Imaging_BitmapImage[] = L"Windows.UI.Xaml.Media.Imaging.BitmapImage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Media.Imaging.BitmapSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Media.Imaging.IBitmapSourceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Media.Imaging.IBitmapSource ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_BitmapSource_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_BitmapSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Media_Imaging_BitmapSource[] = L"Windows.UI.Xaml.Media.Imaging.BitmapSource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Media.Imaging.DownloadProgressEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Media.Imaging.IDownloadProgressEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_DownloadProgressEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_DownloadProgressEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Media_Imaging_DownloadProgressEventArgs[] = L"Windows.UI.Xaml.Media.Imaging.DownloadProgressEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Media.Imaging.RenderTargetBitmap
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Media.Imaging.IRenderTargetBitmapStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Media.Imaging.IRenderTargetBitmap ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_RenderTargetBitmap_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_RenderTargetBitmap_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Media_Imaging_RenderTargetBitmap[] = L"Windows.UI.Xaml.Media.Imaging.RenderTargetBitmap";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Media.Imaging.SoftwareBitmapSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Media.Imaging.ISoftwareBitmapSource ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_SoftwareBitmapSource_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_SoftwareBitmapSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Media_Imaging_SoftwareBitmapSource[] = L"Windows.UI.Xaml.Media.Imaging.SoftwareBitmapSource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Media.Imaging.SurfaceImageSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Media.Imaging.ISurfaceImageSource ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_SurfaceImageSource_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_SurfaceImageSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Media_Imaging_SurfaceImageSource[] = L"Windows.UI.Xaml.Media.Imaging.SurfaceImageSource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Media.Imaging.SvgImageSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Media.Imaging.ISvgImageSourceStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Media.Imaging.ISvgImageSource ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_SvgImageSource_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_SvgImageSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Media_Imaging_SvgImageSource[] = L"Windows.UI.Xaml.Media.Imaging.SvgImageSource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Xaml.Media.Imaging.SvgImageSourceFailedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Media.Imaging.ISvgImageSourceFailedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_SvgImageSourceFailedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_SvgImageSourceFailedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Media_Imaging_SvgImageSourceFailedEventArgs[] = L"Windows.UI.Xaml.Media.Imaging.SvgImageSourceFailedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Xaml.Media.Imaging.SvgImageSourceOpenedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Media.Imaging.ISvgImageSourceOpenedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_SvgImageSourceOpenedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_SvgImageSourceOpenedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Media_Imaging_SvgImageSourceOpenedEventArgs[] = L"Windows.UI.Xaml.Media.Imaging.SvgImageSourceOpenedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Xaml.Media.Imaging.VirtualSurfaceImageSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Xaml.Media.Imaging.IVirtualSurfaceImageSourceFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Media.Imaging.IVirtualSurfaceImageSource ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_VirtualSurfaceImageSource_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_VirtualSurfaceImageSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Media_Imaging_VirtualSurfaceImageSource[] = L"Windows.UI.Xaml.Media.Imaging.VirtualSurfaceImageSource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Media.Imaging.WriteableBitmap
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Xaml.Media.Imaging.IWriteableBitmapFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Media.Imaging.IWriteableBitmap ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_WriteableBitmap_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_WriteableBitmap_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Media_Imaging_WriteableBitmap[] = L"Windows.UI.Xaml.Media.Imaging.WriteableBitmap";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Media.Imaging.XamlRenderingBackgroundTask
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Media.Imaging.IXamlRenderingBackgroundTask ** Default Interface **
 *    Windows.UI.Xaml.Media.Imaging.IXamlRenderingBackgroundTaskOverrides
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_XamlRenderingBackgroundTask_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_XamlRenderingBackgroundTask_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Media_Imaging_XamlRenderingBackgroundTask[] = L"Windows.UI.Xaml.Media.Imaging.XamlRenderingBackgroundTask";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventHandler __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventHandler;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage2 __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage2;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3 __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageFactory __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageFactory;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics2 __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics2;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3 __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceFactory __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceFactory;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceStatics __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgs __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgs;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapStatics __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISoftwareBitmapSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISoftwareBitmapSource_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISoftwareBitmapSource __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISoftwareBitmapSource;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISoftwareBitmapSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSource_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSource __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSource;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceFactory __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceFactory;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFactory __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFactory;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFailedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFailedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFailedEventArgs __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFailedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFailedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceOpenedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceOpenedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceOpenedEventArgs __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceOpenedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceOpenedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStatics __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSource_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSource __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSource;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceFactory __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceFactory;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmap_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmap_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmap __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmap;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmap_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapFactory __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapFactory;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTask_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTask_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTask __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTask;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTask_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskFactory __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskFactory;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskOverrides_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskOverrides_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskOverrides __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskOverrides;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskOverrides_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBufferVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBufferVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBufferVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBufferVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer* This,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBufferVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBufferVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CSvgImageSourceLoadStatus __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CSvgImageSourceLoadStatus;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus __FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperation_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus __FIAsyncOperation_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus;

typedef struct __FIAsyncOperation_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus* This,
        enum __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CSvgImageSourceLoadStatus* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatusVtbl;

interface __FIAsyncOperation_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus __FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus* This,
        __FIAsyncOperation_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatusVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceFailedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceFailedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceFailedEventArgs __FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceFailedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceFailedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceFailedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceFailedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceFailedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceFailedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceFailedEventArgs* This,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource* sender,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFailedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceFailedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceFailedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceFailedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceFailedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceFailedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceFailedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceFailedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceFailedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceOpenedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceOpenedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceOpenedEventArgs __FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceOpenedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceOpenedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceOpenedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceOpenedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceOpenedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceOpenedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceOpenedEventArgs* This,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource* sender,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceOpenedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceOpenedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceOpenedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceOpenedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceOpenedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceOpenedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceOpenedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceOpenedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceOpenedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef ____x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTaskInstance_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTaskInstance_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTaskInstance __x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTaskInstance;

#endif // ____x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTaskInstance_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap;

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIDependencyProperty_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIDependencyProperty_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty;

#endif // ____x_ABI_CWindows_CUI_CXaml_CIDependencyProperty_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIExceptionRoutedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIExceptionRoutedEventHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CIExceptionRoutedEventHandler __x_ABI_CWindows_CUI_CXaml_CIExceptionRoutedEventHandler;

#endif // ____x_ABI_CWindows_CUI_CXaml_CIExceptionRoutedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIRoutedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIRoutedEventHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CIRoutedEventHandler __x_ABI_CWindows_CUI_CXaml_CIRoutedEventHandler;

#endif // ____x_ABI_CWindows_CUI_CXaml_CIRoutedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIUIElement_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIUIElement_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CIUIElement __x_ABI_CWindows_CUI_CXaml_CIUIElement;

#endif // ____x_ABI_CWindows_CUI_CXaml_CIUIElement_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CBitmapCreateOptions __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CBitmapCreateOptions;

typedef enum __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CDecodePixelType __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CDecodePixelType;

/*
 *
 * Struct Windows.UI.Xaml.Media.Imaging.BitmapCreateOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CBitmapCreateOptions
{
    BitmapCreateOptions_None = 0,
    BitmapCreateOptions_IgnoreImageCache = 0x8,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Media.Imaging.DecodePixelType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CDecodePixelType
{
    DecodePixelType_Physical = 0,
    DecodePixelType_Logical = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Media.Imaging.SvgImageSourceLoadStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CSvgImageSourceLoadStatus
{
    SvgImageSourceLoadStatus_Success = 0,
    SvgImageSourceLoadStatus_NetworkError = 1,
    SvgImageSourceLoadStatus_InvalidFormat = 2,
    SvgImageSourceLoadStatus_Other = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Delegate Windows.UI.Xaml.Media.Imaging.DownloadProgressEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventHandler* This,
        IInspectable* sender,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgs* e);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventHandlerVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventHandler_Invoke(This, sender, e) \
    ((This)->lpVtbl->Invoke(This, sender, e))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IBitmapImage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.BitmapImage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IBitmapImage[] = L"Windows.UI.Xaml.Media.Imaging.IBitmapImage";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CreateOptions)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage* This,
        enum __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CBitmapCreateOptions* value);
    HRESULT (STDMETHODCALLTYPE* put_CreateOptions)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage* This,
        enum __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CBitmapCreateOptions value);
    HRESULT (STDMETHODCALLTYPE* get_UriSource)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_UriSource)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* get_DecodePixelWidth)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_DecodePixelWidth)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_DecodePixelHeight)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_DecodePixelHeight)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* add_DownloadProgress)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage* This,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventHandler* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_DownloadProgress)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ImageOpened)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage* This,
        __x_ABI_CWindows_CUI_CXaml_CIRoutedEventHandler* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ImageOpened)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ImageFailed)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage* This,
        __x_ABI_CWindows_CUI_CXaml_CIExceptionRoutedEventHandler* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ImageFailed)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage_get_CreateOptions(This, value) \
    ((This)->lpVtbl->get_CreateOptions(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage_put_CreateOptions(This, value) \
    ((This)->lpVtbl->put_CreateOptions(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage_get_UriSource(This, value) \
    ((This)->lpVtbl->get_UriSource(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage_put_UriSource(This, value) \
    ((This)->lpVtbl->put_UriSource(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage_get_DecodePixelWidth(This, value) \
    ((This)->lpVtbl->get_DecodePixelWidth(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage_put_DecodePixelWidth(This, value) \
    ((This)->lpVtbl->put_DecodePixelWidth(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage_get_DecodePixelHeight(This, value) \
    ((This)->lpVtbl->get_DecodePixelHeight(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage_put_DecodePixelHeight(This, value) \
    ((This)->lpVtbl->put_DecodePixelHeight(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage_add_DownloadProgress(This, handler, token) \
    ((This)->lpVtbl->add_DownloadProgress(This, handler, token))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage_remove_DownloadProgress(This, token) \
    ((This)->lpVtbl->remove_DownloadProgress(This, token))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage_add_ImageOpened(This, handler, token) \
    ((This)->lpVtbl->add_ImageOpened(This, handler, token))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage_remove_ImageOpened(This, token) \
    ((This)->lpVtbl->remove_ImageOpened(This, token))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage_add_ImageFailed(This, handler, token) \
    ((This)->lpVtbl->add_ImageFailed(This, handler, token))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage_remove_ImageFailed(This, token) \
    ((This)->lpVtbl->remove_ImageFailed(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IBitmapImage2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.BitmapImage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IBitmapImage2[] = L"Windows.UI.Xaml.Media.Imaging.IBitmapImage2";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DecodePixelType)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage2* This,
        enum __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CDecodePixelType* value);
    HRESULT (STDMETHODCALLTYPE* put_DecodePixelType)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage2* This,
        enum __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CDecodePixelType value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage2Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage2_get_DecodePixelType(This, value) \
    ((This)->lpVtbl->get_DecodePixelType(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage2_put_DecodePixelType(This, value) \
    ((This)->lpVtbl->put_DecodePixelType(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IBitmapImage3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.BitmapImage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IBitmapImage3[] = L"Windows.UI.Xaml.Media.Imaging.IBitmapImage3";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsAnimatedBitmap)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsPlaying)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_AutoPlay)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AutoPlay)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* Play)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3* This);
    HRESULT (STDMETHODCALLTYPE* Stop)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3* This);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3_get_IsAnimatedBitmap(This, value) \
    ((This)->lpVtbl->get_IsAnimatedBitmap(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3_get_IsPlaying(This, value) \
    ((This)->lpVtbl->get_IsPlaying(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3_get_AutoPlay(This, value) \
    ((This)->lpVtbl->get_AutoPlay(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3_put_AutoPlay(This, value) \
    ((This)->lpVtbl->put_AutoPlay(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3_Play(This) \
    ((This)->lpVtbl->Play(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3_Stop(This) \
    ((This)->lpVtbl->Stop(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IBitmapImageFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.BitmapImage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IBitmapImageFactory[] = L"Windows.UI.Xaml.Media.Imaging.IBitmapImageFactory";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstanceWithUriSource)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageFactory* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uriSource,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImage** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageFactoryVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageFactory_CreateInstanceWithUriSource(This, uriSource, value) \
    ((This)->lpVtbl->CreateInstanceWithUriSource(This, uriSource, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IBitmapImageStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.BitmapImage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IBitmapImageStatics[] = L"Windows.UI.Xaml.Media.Imaging.IBitmapImageStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CreateOptionsProperty)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_UriSourceProperty)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_DecodePixelWidthProperty)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_DecodePixelHeightProperty)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics_get_CreateOptionsProperty(This, value) \
    ((This)->lpVtbl->get_CreateOptionsProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics_get_UriSourceProperty(This, value) \
    ((This)->lpVtbl->get_UriSourceProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics_get_DecodePixelWidthProperty(This, value) \
    ((This)->lpVtbl->get_DecodePixelWidthProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics_get_DecodePixelHeightProperty(This, value) \
    ((This)->lpVtbl->get_DecodePixelHeightProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IBitmapImageStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.BitmapImage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IBitmapImageStatics2[] = L"Windows.UI.Xaml.Media.Imaging.IBitmapImageStatics2";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DecodePixelTypeProperty)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics2* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics2Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics2_get_DecodePixelTypeProperty(This, value) \
    ((This)->lpVtbl->get_DecodePixelTypeProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IBitmapImageStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.BitmapImage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IBitmapImageStatics3[] = L"Windows.UI.Xaml.Media.Imaging.IBitmapImageStatics3";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsAnimatedBitmapProperty)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_IsPlayingProperty)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_AutoPlayProperty)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3_get_IsAnimatedBitmapProperty(This, value) \
    ((This)->lpVtbl->get_IsAnimatedBitmapProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3_get_IsPlayingProperty(This, value) \
    ((This)->lpVtbl->get_IsPlayingProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3_get_AutoPlayProperty(This, value) \
    ((This)->lpVtbl->get_AutoPlayProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapImageStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IBitmapSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.BitmapSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IBitmapSource[] = L"Windows.UI.Xaml.Media.Imaging.IBitmapSource";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PixelWidth)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_PixelHeight)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* SetSource)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* streamSource);
    HRESULT (STDMETHODCALLTYPE* SetSourceAsync)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* streamSource,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource_get_PixelWidth(This, value) \
    ((This)->lpVtbl->get_PixelWidth(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource_get_PixelHeight(This, value) \
    ((This)->lpVtbl->get_PixelHeight(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource_SetSource(This, streamSource) \
    ((This)->lpVtbl->SetSource(This, streamSource))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource_SetSourceAsync(This, streamSource, operation) \
    ((This)->lpVtbl->SetSourceAsync(This, streamSource, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IBitmapSourceFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.BitmapSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IBitmapSourceFactory[] = L"Windows.UI.Xaml.Media.Imaging.IBitmapSourceFactory";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceFactory* This,
        IInspectable* baseInterface,
        IInspectable** innerInterface,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSource** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceFactoryVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceFactory_CreateInstance(This, baseInterface, innerInterface, value) \
    ((This)->lpVtbl->CreateInstance(This, baseInterface, innerInterface, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IBitmapSourceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.BitmapSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IBitmapSourceStatics[] = L"Windows.UI.Xaml.Media.Imaging.IBitmapSourceStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PixelWidthProperty)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_PixelHeightProperty)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceStatics_get_PixelWidthProperty(This, value) \
    ((This)->lpVtbl->get_PixelWidthProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceStatics_get_PixelHeightProperty(This, value) \
    ((This)->lpVtbl->get_PixelHeightProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIBitmapSourceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IDownloadProgressEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.DownloadProgressEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IDownloadProgressEventArgs[] = L"Windows.UI.Xaml.Media.Imaging.IDownloadProgressEventArgs";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgs* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Progress)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgs* This,
        INT32 value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgs_get_Progress(This, value) \
    ((This)->lpVtbl->get_Progress(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgs_put_Progress(This, value) \
    ((This)->lpVtbl->put_Progress(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIDownloadProgressEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IRenderTargetBitmap
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.RenderTargetBitmap
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IRenderTargetBitmap[] = L"Windows.UI.Xaml.Media.Imaging.IRenderTargetBitmap";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PixelWidth)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_PixelHeight)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* RenderAsync)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap* This,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement* element,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* RenderToSizeAsync)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap* This,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement* element,
        INT32 scaledWidth,
        INT32 scaledHeight,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* GetPixelsAsync)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap* This,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer** operation);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap_get_PixelWidth(This, value) \
    ((This)->lpVtbl->get_PixelWidth(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap_get_PixelHeight(This, value) \
    ((This)->lpVtbl->get_PixelHeight(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap_RenderAsync(This, element, operation) \
    ((This)->lpVtbl->RenderAsync(This, element, operation))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap_RenderToSizeAsync(This, element, scaledWidth, scaledHeight, operation) \
    ((This)->lpVtbl->RenderToSizeAsync(This, element, scaledWidth, scaledHeight, operation))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap_GetPixelsAsync(This, operation) \
    ((This)->lpVtbl->GetPixelsAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmap_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IRenderTargetBitmapStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.RenderTargetBitmap
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IRenderTargetBitmapStatics[] = L"Windows.UI.Xaml.Media.Imaging.IRenderTargetBitmapStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PixelWidthProperty)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_PixelHeightProperty)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapStatics_get_PixelWidthProperty(This, value) \
    ((This)->lpVtbl->get_PixelWidthProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapStatics_get_PixelHeightProperty(This, value) \
    ((This)->lpVtbl->get_PixelHeightProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIRenderTargetBitmapStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.ISoftwareBitmapSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.SoftwareBitmapSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISoftwareBitmapSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISoftwareBitmapSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_ISoftwareBitmapSource[] = L"Windows.UI.Xaml.Media.Imaging.ISoftwareBitmapSource";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISoftwareBitmapSourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISoftwareBitmapSource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISoftwareBitmapSource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISoftwareBitmapSource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISoftwareBitmapSource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISoftwareBitmapSource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISoftwareBitmapSource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetBitmapAsync)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISoftwareBitmapSource* This,
        __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap* softwareBitmap,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISoftwareBitmapSourceVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISoftwareBitmapSource
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISoftwareBitmapSourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISoftwareBitmapSource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISoftwareBitmapSource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISoftwareBitmapSource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISoftwareBitmapSource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISoftwareBitmapSource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISoftwareBitmapSource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISoftwareBitmapSource_SetBitmapAsync(This, softwareBitmap, operation) \
    ((This)->lpVtbl->SetBitmapAsync(This, softwareBitmap, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISoftwareBitmapSource;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISoftwareBitmapSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.ISurfaceImageSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.SurfaceImageSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_ISurfaceImageSource[] = L"Windows.UI.Xaml.Media.Imaging.ISurfaceImageSource";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSource* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSource
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSource;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.ISurfaceImageSourceFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.SurfaceImageSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_ISurfaceImageSourceFactory[] = L"Windows.UI.Xaml.Media.Imaging.ISurfaceImageSourceFactory";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstanceWithDimensions)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceFactory* This,
        INT32 pixelWidth,
        INT32 pixelHeight,
        IInspectable* baseInterface,
        IInspectable** innerInterface,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSource** value);
    HRESULT (STDMETHODCALLTYPE* CreateInstanceWithDimensionsAndOpacity)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceFactory* This,
        INT32 pixelWidth,
        INT32 pixelHeight,
        boolean isOpaque,
        IInspectable* baseInterface,
        IInspectable** innerInterface,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSource** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceFactoryVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceFactory_CreateInstanceWithDimensions(This, pixelWidth, pixelHeight, baseInterface, innerInterface, value) \
    ((This)->lpVtbl->CreateInstanceWithDimensions(This, pixelWidth, pixelHeight, baseInterface, innerInterface, value))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceFactory_CreateInstanceWithDimensionsAndOpacity(This, pixelWidth, pixelHeight, isOpaque, baseInterface, innerInterface, value) \
    ((This)->lpVtbl->CreateInstanceWithDimensionsAndOpacity(This, pixelWidth, pixelHeight, isOpaque, baseInterface, innerInterface, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISurfaceImageSourceFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.ISvgImageSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.SvgImageSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_ISvgImageSource[] = L"Windows.UI.Xaml.Media.Imaging.ISvgImageSource";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_UriSource)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_UriSource)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* get_RasterizePixelWidth)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_RasterizePixelWidth)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_RasterizePixelHeight)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_RasterizePixelHeight)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* add_Opened)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource* This,
        __FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceOpenedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Opened)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_OpenFailed)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource* This,
        __FITypedEventHandler_2_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSource_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceFailedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_OpenFailed)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* SetSourceAsync)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* streamSource,
        __FIAsyncOperation_1_Windows__CUI__CXaml__CMedia__CImaging__CSvgImageSourceLoadStatus** operation);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource_get_UriSource(This, value) \
    ((This)->lpVtbl->get_UriSource(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource_put_UriSource(This, value) \
    ((This)->lpVtbl->put_UriSource(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource_get_RasterizePixelWidth(This, value) \
    ((This)->lpVtbl->get_RasterizePixelWidth(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource_put_RasterizePixelWidth(This, value) \
    ((This)->lpVtbl->put_RasterizePixelWidth(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource_get_RasterizePixelHeight(This, value) \
    ((This)->lpVtbl->get_RasterizePixelHeight(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource_put_RasterizePixelHeight(This, value) \
    ((This)->lpVtbl->put_RasterizePixelHeight(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource_add_Opened(This, handler, token) \
    ((This)->lpVtbl->add_Opened(This, handler, token))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource_remove_Opened(This, token) \
    ((This)->lpVtbl->remove_Opened(This, token))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource_add_OpenFailed(This, handler, token) \
    ((This)->lpVtbl->add_OpenFailed(This, handler, token))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource_remove_OpenFailed(This, token) \
    ((This)->lpVtbl->remove_OpenFailed(This, token))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource_SetSourceAsync(This, streamSource, operation) \
    ((This)->lpVtbl->SetSourceAsync(This, streamSource, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.ISvgImageSourceFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.SvgImageSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_ISvgImageSourceFactory[] = L"Windows.UI.Xaml.Media.Imaging.ISvgImageSourceFactory";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFactory* This,
        IInspectable* baseInterface,
        IInspectable** innerInterface,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource** value);
    HRESULT (STDMETHODCALLTYPE* CreateInstanceWithUriSource)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFactory* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uriSource,
        IInspectable* baseInterface,
        IInspectable** innerInterface,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSource** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFactoryVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFactory_CreateInstance(This, baseInterface, innerInterface, value) \
    ((This)->lpVtbl->CreateInstance(This, baseInterface, innerInterface, value))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFactory_CreateInstanceWithUriSource(This, uriSource, baseInterface, innerInterface, value) \
    ((This)->lpVtbl->CreateInstanceWithUriSource(This, uriSource, baseInterface, innerInterface, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.ISvgImageSourceFailedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.SvgImageSourceFailedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFailedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFailedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_ISvgImageSourceFailedEventArgs[] = L"Windows.UI.Xaml.Media.Imaging.ISvgImageSourceFailedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFailedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFailedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFailedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFailedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFailedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFailedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFailedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFailedEventArgs* This,
        enum __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CSvgImageSourceLoadStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFailedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFailedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFailedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFailedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFailedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFailedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFailedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFailedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFailedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFailedEventArgs_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFailedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceFailedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.ISvgImageSourceOpenedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.SvgImageSourceOpenedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceOpenedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceOpenedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_ISvgImageSourceOpenedEventArgs[] = L"Windows.UI.Xaml.Media.Imaging.ISvgImageSourceOpenedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceOpenedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceOpenedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceOpenedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceOpenedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceOpenedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceOpenedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceOpenedEventArgs* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceOpenedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceOpenedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceOpenedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceOpenedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceOpenedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceOpenedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceOpenedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceOpenedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceOpenedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceOpenedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceOpenedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.ISvgImageSourceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.SvgImageSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_ISvgImageSourceStatics[] = L"Windows.UI.Xaml.Media.Imaging.ISvgImageSourceStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_UriSourceProperty)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_RasterizePixelWidthProperty)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_RasterizePixelHeightProperty)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStatics_get_UriSourceProperty(This, value) \
    ((This)->lpVtbl->get_UriSourceProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStatics_get_RasterizePixelWidthProperty(This, value) \
    ((This)->lpVtbl->get_RasterizePixelWidthProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStatics_get_RasterizePixelHeightProperty(This, value) \
    ((This)->lpVtbl->get_RasterizePixelHeightProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CISvgImageSourceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IVirtualSurfaceImageSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.VirtualSurfaceImageSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IVirtualSurfaceImageSource[] = L"Windows.UI.Xaml.Media.Imaging.IVirtualSurfaceImageSource";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSource* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSource
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSource;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IVirtualSurfaceImageSourceFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.VirtualSurfaceImageSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IVirtualSurfaceImageSourceFactory[] = L"Windows.UI.Xaml.Media.Imaging.IVirtualSurfaceImageSourceFactory";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstanceWithDimensions)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceFactory* This,
        INT32 pixelWidth,
        INT32 pixelHeight,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSource** value);
    HRESULT (STDMETHODCALLTYPE* CreateInstanceWithDimensionsAndOpacity)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceFactory* This,
        INT32 pixelWidth,
        INT32 pixelHeight,
        boolean isOpaque,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSource** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceFactoryVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceFactory_CreateInstanceWithDimensions(This, pixelWidth, pixelHeight, value) \
    ((This)->lpVtbl->CreateInstanceWithDimensions(This, pixelWidth, pixelHeight, value))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceFactory_CreateInstanceWithDimensionsAndOpacity(This, pixelWidth, pixelHeight, isOpaque, value) \
    ((This)->lpVtbl->CreateInstanceWithDimensionsAndOpacity(This, pixelWidth, pixelHeight, isOpaque, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIVirtualSurfaceImageSourceFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IWriteableBitmap
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.WriteableBitmap
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmap_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmap_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IWriteableBitmap[] = L"Windows.UI.Xaml.Media.Imaging.IWriteableBitmap";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmap* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmap* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmap* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmap* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmap* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmap* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PixelBuffer)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmap* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* Invalidate)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmap* This);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmap
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmap_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmap_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmap_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmap_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmap_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmap_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmap_get_PixelBuffer(This, value) \
    ((This)->lpVtbl->get_PixelBuffer(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmap_Invalidate(This) \
    ((This)->lpVtbl->Invalidate(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmap;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmap_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IWriteableBitmapFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.WriteableBitmap
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IWriteableBitmapFactory[] = L"Windows.UI.Xaml.Media.Imaging.IWriteableBitmapFactory";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstanceWithDimensions)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapFactory* This,
        INT32 pixelWidth,
        INT32 pixelHeight,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmap** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapFactoryVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapFactory_CreateInstanceWithDimensions(This, pixelWidth, pixelHeight, value) \
    ((This)->lpVtbl->CreateInstanceWithDimensions(This, pixelWidth, pixelHeight, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIWriteableBitmapFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IXamlRenderingBackgroundTask
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.XamlRenderingBackgroundTask
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTask_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTask_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IXamlRenderingBackgroundTask[] = L"Windows.UI.Xaml.Media.Imaging.IXamlRenderingBackgroundTask";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTask* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTask* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTask* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTask* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTask* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTask* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTask
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTask_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTask_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTask_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTask_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTask_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTask_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTask;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTask_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IXamlRenderingBackgroundTaskFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.XamlRenderingBackgroundTask
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IXamlRenderingBackgroundTaskFactory[] = L"Windows.UI.Xaml.Media.Imaging.IXamlRenderingBackgroundTaskFactory";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskFactory* This,
        IInspectable* baseInterface,
        IInspectable** innerInterface,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTask** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskFactoryVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskFactory_CreateInstance(This, baseInterface, innerInterface, value) \
    ((This)->lpVtbl->CreateInstance(This, baseInterface, innerInterface, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Media.Imaging.IXamlRenderingBackgroundTaskOverrides
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Media.Imaging.XamlRenderingBackgroundTask
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskOverrides_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskOverrides_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Media_Imaging_IXamlRenderingBackgroundTaskOverrides[] = L"Windows.UI.Xaml.Media.Imaging.IXamlRenderingBackgroundTaskOverrides";
typedef struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskOverridesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskOverrides* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskOverrides* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskOverrides* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskOverrides* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskOverrides* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskOverrides* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* OnRun)(__x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskOverrides* This,
        __x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTaskInstance* taskInstance);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskOverridesVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskOverrides
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskOverridesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskOverrides_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskOverrides_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskOverrides_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskOverrides_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskOverrides_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskOverrides_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskOverrides_OnRun(This, taskInstance) \
    ((This)->lpVtbl->OnRun(This, taskInstance))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskOverrides;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CMedia_CImaging_CIXamlRenderingBackgroundTaskOverrides_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Media.Imaging.BitmapImage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Xaml.Media.Imaging.IBitmapImageFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Media.Imaging.IBitmapImageStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Media.Imaging.IBitmapImageStatics3 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Media.Imaging.IBitmapImageStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Media.Imaging.IBitmapImage ** Default Interface **
 *    Windows.UI.Xaml.Media.Imaging.IBitmapImage2
 *    Windows.UI.Xaml.Media.Imaging.IBitmapImage3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_BitmapImage_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_BitmapImage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Media_Imaging_BitmapImage[] = L"Windows.UI.Xaml.Media.Imaging.BitmapImage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Media.Imaging.BitmapSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Media.Imaging.IBitmapSourceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Media.Imaging.IBitmapSource ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_BitmapSource_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_BitmapSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Media_Imaging_BitmapSource[] = L"Windows.UI.Xaml.Media.Imaging.BitmapSource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Media.Imaging.DownloadProgressEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Media.Imaging.IDownloadProgressEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_DownloadProgressEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_DownloadProgressEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Media_Imaging_DownloadProgressEventArgs[] = L"Windows.UI.Xaml.Media.Imaging.DownloadProgressEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Media.Imaging.RenderTargetBitmap
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Media.Imaging.IRenderTargetBitmapStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Media.Imaging.IRenderTargetBitmap ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_RenderTargetBitmap_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_RenderTargetBitmap_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Media_Imaging_RenderTargetBitmap[] = L"Windows.UI.Xaml.Media.Imaging.RenderTargetBitmap";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Media.Imaging.SoftwareBitmapSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Media.Imaging.ISoftwareBitmapSource ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_SoftwareBitmapSource_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_SoftwareBitmapSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Media_Imaging_SoftwareBitmapSource[] = L"Windows.UI.Xaml.Media.Imaging.SoftwareBitmapSource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Media.Imaging.SurfaceImageSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Media.Imaging.ISurfaceImageSource ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_SurfaceImageSource_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_SurfaceImageSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Media_Imaging_SurfaceImageSource[] = L"Windows.UI.Xaml.Media.Imaging.SurfaceImageSource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Media.Imaging.SvgImageSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Media.Imaging.ISvgImageSourceStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Media.Imaging.ISvgImageSource ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_SvgImageSource_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_SvgImageSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Media_Imaging_SvgImageSource[] = L"Windows.UI.Xaml.Media.Imaging.SvgImageSource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Xaml.Media.Imaging.SvgImageSourceFailedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Media.Imaging.ISvgImageSourceFailedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_SvgImageSourceFailedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_SvgImageSourceFailedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Media_Imaging_SvgImageSourceFailedEventArgs[] = L"Windows.UI.Xaml.Media.Imaging.SvgImageSourceFailedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Xaml.Media.Imaging.SvgImageSourceOpenedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Media.Imaging.ISvgImageSourceOpenedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_SvgImageSourceOpenedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_SvgImageSourceOpenedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Media_Imaging_SvgImageSourceOpenedEventArgs[] = L"Windows.UI.Xaml.Media.Imaging.SvgImageSourceOpenedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Xaml.Media.Imaging.VirtualSurfaceImageSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Xaml.Media.Imaging.IVirtualSurfaceImageSourceFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Media.Imaging.IVirtualSurfaceImageSource ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_VirtualSurfaceImageSource_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_VirtualSurfaceImageSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Media_Imaging_VirtualSurfaceImageSource[] = L"Windows.UI.Xaml.Media.Imaging.VirtualSurfaceImageSource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Media.Imaging.WriteableBitmap
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Xaml.Media.Imaging.IWriteableBitmapFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Media.Imaging.IWriteableBitmap ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_WriteableBitmap_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_WriteableBitmap_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Media_Imaging_WriteableBitmap[] = L"Windows.UI.Xaml.Media.Imaging.WriteableBitmap";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Media.Imaging.XamlRenderingBackgroundTask
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Media.Imaging.IXamlRenderingBackgroundTask ** Default Interface **
 *    Windows.UI.Xaml.Media.Imaging.IXamlRenderingBackgroundTaskOverrides
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_XamlRenderingBackgroundTask_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Media_Imaging_XamlRenderingBackgroundTask_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Media_Imaging_XamlRenderingBackgroundTask[] = L"Windows.UI.Xaml.Media.Imaging.XamlRenderingBackgroundTask";
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
#endif // __windows2Eui2Examl2Emedia2Eimaging_p_h__

#endif // __windows2Eui2Examl2Emedia2Eimaging_h__
