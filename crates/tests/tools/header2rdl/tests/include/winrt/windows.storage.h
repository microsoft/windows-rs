
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
#ifndef __windows2Estorage_h__
#define __windows2Estorage_h__
#ifndef __windows2Estorage_p_h__
#define __windows2Estorage_p_h__


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

#if !defined(WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION)
#define WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION 0x70000
#endif // defined(WINDOWS_STORAGE_PROVIDER_CLOUDFILESCONTRACT_VERSION)

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
#include "Windows.Storage.FileProperties.h"
#include "Windows.Storage.Provider.h"
#include "Windows.Storage.Search.h"
#include "Windows.Storage.Streams.h"
#include "Windows.System.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CStorage_CIApplicationDataSetVersionHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIApplicationDataSetVersionHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IApplicationDataSetVersionHandler;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIApplicationDataSetVersionHandler ABI::Windows::Storage::IApplicationDataSetVersionHandler

#endif // ____x_ABI_CWindows_CStorage_CIApplicationDataSetVersionHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStreamedFileDataRequestedHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStreamedFileDataRequestedHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStreamedFileDataRequestedHandler;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStreamedFileDataRequestedHandler ABI::Windows::Storage::IStreamedFileDataRequestedHandler

#endif // ____x_ABI_CWindows_CStorage_CIStreamedFileDataRequestedHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIAppDataPaths_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIAppDataPaths_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IAppDataPaths;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIAppDataPaths ABI::Windows::Storage::IAppDataPaths

#endif // ____x_ABI_CWindows_CStorage_CIAppDataPaths_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIAppDataPathsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIAppDataPathsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IAppDataPathsStatics;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIAppDataPathsStatics ABI::Windows::Storage::IAppDataPathsStatics

#endif // ____x_ABI_CWindows_CStorage_CIAppDataPathsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIApplicationData_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIApplicationData_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IApplicationData;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIApplicationData ABI::Windows::Storage::IApplicationData

#endif // ____x_ABI_CWindows_CStorage_CIApplicationData_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIApplicationData2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIApplicationData2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IApplicationData2;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIApplicationData2 ABI::Windows::Storage::IApplicationData2

#endif // ____x_ABI_CWindows_CStorage_CIApplicationData2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIApplicationData3_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIApplicationData3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IApplicationData3;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIApplicationData3 ABI::Windows::Storage::IApplicationData3

#endif // ____x_ABI_CWindows_CStorage_CIApplicationData3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIApplicationDataContainer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIApplicationDataContainer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IApplicationDataContainer;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIApplicationDataContainer ABI::Windows::Storage::IApplicationDataContainer

#endif // ____x_ABI_CWindows_CStorage_CIApplicationDataContainer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIApplicationDataStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIApplicationDataStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IApplicationDataStatics;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIApplicationDataStatics ABI::Windows::Storage::IApplicationDataStatics

#endif // ____x_ABI_CWindows_CStorage_CIApplicationDataStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIApplicationDataStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIApplicationDataStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IApplicationDataStatics2;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIApplicationDataStatics2 ABI::Windows::Storage::IApplicationDataStatics2

#endif // ____x_ABI_CWindows_CStorage_CIApplicationDataStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CICachedFileManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CICachedFileManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface ICachedFileManagerStatics;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CICachedFileManagerStatics ABI::Windows::Storage::ICachedFileManagerStatics

#endif // ____x_ABI_CWindows_CStorage_CICachedFileManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIDownloadsFolderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIDownloadsFolderStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IDownloadsFolderStatics;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIDownloadsFolderStatics ABI::Windows::Storage::IDownloadsFolderStatics

#endif // ____x_ABI_CWindows_CStorage_CIDownloadsFolderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IDownloadsFolderStatics2;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2 ABI::Windows::Storage::IDownloadsFolderStatics2

#endif // ____x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIFileIOStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIFileIOStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IFileIOStatics;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIFileIOStatics ABI::Windows::Storage::IFileIOStatics

#endif // ____x_ABI_CWindows_CStorage_CIFileIOStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIKnownFoldersCameraRollStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIKnownFoldersCameraRollStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IKnownFoldersCameraRollStatics;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIKnownFoldersCameraRollStatics ABI::Windows::Storage::IKnownFoldersCameraRollStatics

#endif // ____x_ABI_CWindows_CStorage_CIKnownFoldersCameraRollStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIKnownFoldersPlaylistsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIKnownFoldersPlaylistsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IKnownFoldersPlaylistsStatics;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIKnownFoldersPlaylistsStatics ABI::Windows::Storage::IKnownFoldersPlaylistsStatics

#endif // ____x_ABI_CWindows_CStorage_CIKnownFoldersPlaylistsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIKnownFoldersSavedPicturesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIKnownFoldersSavedPicturesStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IKnownFoldersSavedPicturesStatics;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIKnownFoldersSavedPicturesStatics ABI::Windows::Storage::IKnownFoldersSavedPicturesStatics

#endif // ____x_ABI_CWindows_CStorage_CIKnownFoldersSavedPicturesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIKnownFoldersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIKnownFoldersStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IKnownFoldersStatics;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics ABI::Windows::Storage::IKnownFoldersStatics

#endif // ____x_ABI_CWindows_CStorage_CIKnownFoldersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIKnownFoldersStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIKnownFoldersStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IKnownFoldersStatics2;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics2 ABI::Windows::Storage::IKnownFoldersStatics2

#endif // ____x_ABI_CWindows_CStorage_CIKnownFoldersStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIKnownFoldersStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIKnownFoldersStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IKnownFoldersStatics3;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics3 ABI::Windows::Storage::IKnownFoldersStatics3

#endif // ____x_ABI_CWindows_CStorage_CIKnownFoldersStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIKnownFoldersStatics4_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIKnownFoldersStatics4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IKnownFoldersStatics4;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics4 ABI::Windows::Storage::IKnownFoldersStatics4

#endif // ____x_ABI_CWindows_CStorage_CIKnownFoldersStatics4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIPathIOStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIPathIOStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IPathIOStatics;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIPathIOStatics ABI::Windows::Storage::IPathIOStatics

#endif // ____x_ABI_CWindows_CStorage_CIPathIOStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CISetVersionDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CISetVersionDeferral_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface ISetVersionDeferral;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CISetVersionDeferral ABI::Windows::Storage::ISetVersionDeferral

#endif // ____x_ABI_CWindows_CStorage_CISetVersionDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CISetVersionRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CISetVersionRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface ISetVersionRequest;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CISetVersionRequest ABI::Windows::Storage::ISetVersionRequest

#endif // ____x_ABI_CWindows_CStorage_CISetVersionRequest_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageFile2;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageFile2 ABI::Windows::Storage::IStorageFile2

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageFilePropertiesWithAvailability;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability ABI::Windows::Storage::IStorageFilePropertiesWithAvailability

#endif // ____x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFileStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFileStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageFileStatics;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageFileStatics ABI::Windows::Storage::IStorageFileStatics

#endif // ____x_ABI_CWindows_CStorage_CIStorageFileStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFileStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFileStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageFileStatics2;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageFileStatics2 ABI::Windows::Storage::IStorageFileStatics2

#endif // ____x_ABI_CWindows_CStorage_CIStorageFileStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageFolder;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageFolder ABI::Windows::Storage::IStorageFolder

#endif // ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFolder2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFolder2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageFolder2;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageFolder2 ABI::Windows::Storage::IStorageFolder2

#endif // ____x_ABI_CWindows_CStorage_CIStorageFolder2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFolder3_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFolder3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageFolder3;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageFolder3 ABI::Windows::Storage::IStorageFolder3

#endif // ____x_ABI_CWindows_CStorage_CIStorageFolder3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFolderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFolderStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageFolderStatics;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageFolderStatics ABI::Windows::Storage::IStorageFolderStatics

#endif // ____x_ABI_CWindows_CStorage_CIStorageFolderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFolderStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFolderStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageFolderStatics2;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageFolderStatics2 ABI::Windows::Storage::IStorageFolderStatics2

#endif // ____x_ABI_CWindows_CStorage_CIStorageFolderStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageItem_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageItem;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageItem ABI::Windows::Storage::IStorageItem

#endif // ____x_ABI_CWindows_CStorage_CIStorageItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageItem2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageItem2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageItem2;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageItem2 ABI::Windows::Storage::IStorageItem2

#endif // ____x_ABI_CWindows_CStorage_CIStorageItem2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageItemProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageItemProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageItemProperties;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageItemProperties ABI::Windows::Storage::IStorageItemProperties

#endif // ____x_ABI_CWindows_CStorage_CIStorageItemProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageItemProperties2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageItemProperties2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageItemProperties2;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageItemProperties2 ABI::Windows::Storage::IStorageItemProperties2

#endif // ____x_ABI_CWindows_CStorage_CIStorageItemProperties2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageItemPropertiesWithProvider;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider ABI::Windows::Storage::IStorageItemPropertiesWithProvider

#endif // ____x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageLibrary_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageLibrary_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageLibrary;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageLibrary ABI::Windows::Storage::IStorageLibrary

#endif // ____x_ABI_CWindows_CStorage_CIStorageLibrary_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageLibrary2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageLibrary2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageLibrary2;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageLibrary2 ABI::Windows::Storage::IStorageLibrary2

#endif // ____x_ABI_CWindows_CStorage_CIStorageLibrary2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageLibrary3_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageLibrary3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageLibrary3;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageLibrary3 ABI::Windows::Storage::IStorageLibrary3

#endif // ____x_ABI_CWindows_CStorage_CIStorageLibrary3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageLibraryChange_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryChange_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageLibraryChange;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageLibraryChange ABI::Windows::Storage::IStorageLibraryChange

#endif // ____x_ABI_CWindows_CStorage_CIStorageLibraryChange_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageLibraryChangeReader;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader ABI::Windows::Storage::IStorageLibraryChangeReader

#endif // ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageLibraryChangeReader2;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader2 ABI::Windows::Storage::IStorageLibraryChangeReader2

#endif // ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageLibraryChangeTracker;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker ABI::Windows::Storage::IStorageLibraryChangeTracker

#endif // ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageLibraryChangeTracker2;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker2 ABI::Windows::Storage::IStorageLibraryChangeTracker2

#endif // ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageLibraryChangeTrackerOptions;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptions ABI::Windows::Storage::IStorageLibraryChangeTrackerOptions

#endif // ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeId_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeId_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageLibraryLastChangeId;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeId ABI::Windows::Storage::IStorageLibraryLastChangeId

#endif // ____x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeId_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageLibraryLastChangeIdStatics;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdStatics ABI::Windows::Storage::IStorageLibraryLastChangeIdStatics

#endif // ____x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageLibraryStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageLibraryStatics;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageLibraryStatics ABI::Windows::Storage::IStorageLibraryStatics

#endif // ____x_ABI_CWindows_CStorage_CIStorageLibraryStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageLibraryStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageLibraryStatics2;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageLibraryStatics2 ABI::Windows::Storage::IStorageLibraryStatics2

#endif // ____x_ABI_CWindows_CStorage_CIStorageLibraryStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageProvider;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageProvider ABI::Windows::Storage::IStorageProvider

#endif // ____x_ABI_CWindows_CStorage_CIStorageProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageProvider2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageProvider2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageProvider2;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageProvider2 ABI::Windows::Storage::IStorageProvider2

#endif // ____x_ABI_CWindows_CStorage_CIStorageProvider2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageStreamTransaction_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageStreamTransaction_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageStreamTransaction;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageStreamTransaction ABI::Windows::Storage::IStorageStreamTransaction

#endif // ____x_ABI_CWindows_CStorage_CIStorageStreamTransaction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStreamedFileDataRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStreamedFileDataRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStreamedFileDataRequest;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStreamedFileDataRequest ABI::Windows::Storage::IStreamedFileDataRequest

#endif // ____x_ABI_CWindows_CStorage_CIStreamedFileDataRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CISystemAudioProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CISystemAudioProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface ISystemAudioProperties;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CISystemAudioProperties ABI::Windows::Storage::ISystemAudioProperties

#endif // ____x_ABI_CWindows_CStorage_CISystemAudioProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CISystemDataPaths_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CISystemDataPaths_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface ISystemDataPaths;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CISystemDataPaths ABI::Windows::Storage::ISystemDataPaths

#endif // ____x_ABI_CWindows_CStorage_CISystemDataPaths_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CISystemDataPathsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CISystemDataPathsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface ISystemDataPathsStatics;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CISystemDataPathsStatics ABI::Windows::Storage::ISystemDataPathsStatics

#endif // ____x_ABI_CWindows_CStorage_CISystemDataPathsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CISystemGPSProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CISystemGPSProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface ISystemGPSProperties;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CISystemGPSProperties ABI::Windows::Storage::ISystemGPSProperties

#endif // ____x_ABI_CWindows_CStorage_CISystemGPSProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CISystemImageProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CISystemImageProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface ISystemImageProperties;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CISystemImageProperties ABI::Windows::Storage::ISystemImageProperties

#endif // ____x_ABI_CWindows_CStorage_CISystemImageProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CISystemMediaProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CISystemMediaProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface ISystemMediaProperties;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CISystemMediaProperties ABI::Windows::Storage::ISystemMediaProperties

#endif // ____x_ABI_CWindows_CStorage_CISystemMediaProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CISystemMusicProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CISystemMusicProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface ISystemMusicProperties;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CISystemMusicProperties ABI::Windows::Storage::ISystemMusicProperties

#endif // ____x_ABI_CWindows_CStorage_CISystemMusicProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CISystemPhotoProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CISystemPhotoProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface ISystemPhotoProperties;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CISystemPhotoProperties ABI::Windows::Storage::ISystemPhotoProperties

#endif // ____x_ABI_CWindows_CStorage_CISystemPhotoProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CISystemProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CISystemProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface ISystemProperties;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CISystemProperties ABI::Windows::Storage::ISystemProperties

#endif // ____x_ABI_CWindows_CStorage_CISystemProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CISystemVideoProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CISystemVideoProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface ISystemVideoProperties;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CISystemVideoProperties ABI::Windows::Storage::ISystemVideoProperties

#endif // ____x_ABI_CWindows_CStorage_CISystemVideoProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIUserDataPaths_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIUserDataPaths_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IUserDataPaths;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIUserDataPaths ABI::Windows::Storage::IUserDataPaths

#endif // ____x_ABI_CWindows_CStorage_CIUserDataPaths_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIUserDataPathsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIUserDataPathsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IUserDataPathsStatics;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIUserDataPathsStatics ABI::Windows::Storage::IUserDataPathsStatics

#endif // ____x_ABI_CWindows_CStorage_CIUserDataPathsStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions

#ifndef DEF___FIAsyncOperation_1_boolean_USE
#define DEF___FIAsyncOperation_1_boolean_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("cdb5efb3-5788-509d-9be1-71ccb8a3362a"))
IAsyncOperation<bool> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<bool, boolean>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Boolean>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<bool> __FIAsyncOperation_1_boolean_t;
#define __FIAsyncOperation_1_boolean ABI::Windows::Foundation::__FIAsyncOperation_1_boolean_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_boolean_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1_boolean_USE
#define DEF___FIAsyncOperationCompletedHandler_1_boolean_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c1d3d1a2-ae17-5a5f-b5a2-bdcc8844889a"))
IAsyncOperationCompletedHandler<bool> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<bool, boolean>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Boolean>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<bool> __FIAsyncOperationCompletedHandler_1_boolean_t;
#define __FIAsyncOperationCompletedHandler_1_boolean ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_boolean_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_boolean_USE */



#ifndef DEF___FIAsyncOperation_1_HSTRING_USE
#define DEF___FIAsyncOperation_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3e1fe603-f897-5263-b328-0806426b8a79"))
IAsyncOperation<HSTRING> : IAsyncOperation_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<HSTRING> __FIAsyncOperation_1_HSTRING_t;
#define __FIAsyncOperation_1_HSTRING ABI::Windows::Foundation::__FIAsyncOperation_1_HSTRING_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_HSTRING_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1_HSTRING_USE
#define DEF___FIAsyncOperationCompletedHandler_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b79a741f-7fb5-50ae-9e99-911201ec3d41"))
IAsyncOperationCompletedHandler<HSTRING> : IAsyncOperationCompletedHandler_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<HSTRING> __FIAsyncOperationCompletedHandler_1_HSTRING_t;
#define __FIAsyncOperationCompletedHandler_1_HSTRING ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_HSTRING_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_HSTRING_USE */


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CStorage__CIStorageItem_USE
#define DEF___FIIterator_1_Windows__CStorage__CIStorageItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("05b487c2-3830-5d3c-98da-25fa11542dbd"))
IIterator<ABI::Windows::Storage::IStorageItem*> : IIterator_impl<ABI::Windows::Storage::IStorageItem*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Storage.IStorageItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Storage::IStorageItem*> __FIIterator_1_Windows__CStorage__CIStorageItem_t;
#define __FIIterator_1_Windows__CStorage__CIStorageItem ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CStorage__CIStorageItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CStorage__CIStorageItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CStorage__CIStorageItem_USE
#define DEF___FIIterable_1_Windows__CStorage__CIStorageItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bb8b8418-65d1-544b-b083-6d172f568c73"))
IIterable<ABI::Windows::Storage::IStorageItem*> : IIterable_impl<ABI::Windows::Storage::IStorageItem*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Storage.IStorageItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Storage::IStorageItem*> __FIIterable_1_Windows__CStorage__CIStorageItem_t;
#define __FIIterable_1_Windows__CStorage__CIStorageItem ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CStorage__CIStorageItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CStorage__CIStorageItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CStorage__CIStorageItem_USE
#define DEF___FIVectorView_1_Windows__CStorage__CIStorageItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("85575a41-06cb-58d0-b98a-7c8f06e6e9d7"))
IVectorView<ABI::Windows::Storage::IStorageItem*> : IVectorView_impl<ABI::Windows::Storage::IStorageItem*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Storage.IStorageItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Storage::IStorageItem*> __FIVectorView_1_Windows__CStorage__CIStorageItem_t;
#define __FIVectorView_1_Windows__CStorage__CIStorageItem ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CStorage__CIStorageItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CStorage__CIStorageItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4b1c0fd7-7a01-5e7a-a6fe-be4500283f23"))
IAsyncOperation<__FIVectorView_1_Windows__CStorage__CIStorageItem*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CStorage__CIStorageItem*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Storage.IStorageItem>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CStorage__CIStorageItem*> __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("51436e75-ace1-5a68-b260-f843b846f0db"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CStorage__CIStorageItem*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CStorage__CIStorageItem*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Storage.IStorageItem>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CStorage__CIStorageItem*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Storage {
            class StorageFile;
        } /* Storage */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CStorage__CStorageFile_USE
#define DEF___FIIterator_1_Windows__CStorage__CStorageFile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("43e29f53-0298-55aa-a6c8-4edd323d9598"))
IIterator<ABI::Windows::Storage::StorageFile*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageFile*, ABI::Windows::Storage::IStorageFile*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Storage.StorageFile>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Storage::StorageFile*> __FIIterator_1_Windows__CStorage__CStorageFile_t;
#define __FIIterator_1_Windows__CStorage__CStorageFile ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CStorage__CStorageFile_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CStorage__CStorageFile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CStorage__CStorageFile_USE
#define DEF___FIIterable_1_Windows__CStorage__CStorageFile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9ac00304-83ea-5688-87b6-ae38aab65d0b"))
IIterable<ABI::Windows::Storage::StorageFile*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageFile*, ABI::Windows::Storage::IStorageFile*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Storage.StorageFile>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Storage::StorageFile*> __FIIterable_1_Windows__CStorage__CStorageFile_t;
#define __FIIterable_1_Windows__CStorage__CStorageFile ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CStorage__CStorageFile_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CStorage__CStorageFile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CStorage__CStorageFile_USE
#define DEF___FIVectorView_1_Windows__CStorage__CStorageFile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("80646519-5e2a-595d-a8cd-2a24b4067f1b"))
IVectorView<ABI::Windows::Storage::StorageFile*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageFile*, ABI::Windows::Storage::IStorageFile*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Storage.StorageFile>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Storage::StorageFile*> __FIVectorView_1_Windows__CStorage__CStorageFile_t;
#define __FIVectorView_1_Windows__CStorage__CStorageFile ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CStorage__CStorageFile_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CStorage__CStorageFile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("03362e33-e413-5f29-97d0-48a4780935f9"))
IAsyncOperation<__FIVectorView_1_Windows__CStorage__CStorageFile*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CStorage__CStorageFile*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Storage.StorageFile>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CStorage__CStorageFile*> __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("cb4206c5-0988-5104-afa9-253c298f86fd"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CStorage__CStorageFile*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CStorage__CStorageFile*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Storage.StorageFile>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CStorage__CStorageFile*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Storage {
            class StorageFolder;
        } /* Storage */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CStorage__CStorageFolder_USE
#define DEF___FIIterator_1_Windows__CStorage__CStorageFolder_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5aac96fb-b3b9-5a7f-a920-4b5a8df81168"))
IIterator<ABI::Windows::Storage::StorageFolder*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageFolder*, ABI::Windows::Storage::IStorageFolder*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Storage.StorageFolder>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Storage::StorageFolder*> __FIIterator_1_Windows__CStorage__CStorageFolder_t;
#define __FIIterator_1_Windows__CStorage__CStorageFolder ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CStorage__CStorageFolder_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CStorage__CStorageFolder_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CStorage__CStorageFolder_USE
#define DEF___FIIterable_1_Windows__CStorage__CStorageFolder_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4669befc-ae5c-52b1-8a97-5466ce61e94e"))
IIterable<ABI::Windows::Storage::StorageFolder*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageFolder*, ABI::Windows::Storage::IStorageFolder*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Storage.StorageFolder>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Storage::StorageFolder*> __FIIterable_1_Windows__CStorage__CStorageFolder_t;
#define __FIIterable_1_Windows__CStorage__CStorageFolder ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CStorage__CStorageFolder_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CStorage__CStorageFolder_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CStorage__CStorageFolder_USE
#define DEF___FIVectorView_1_Windows__CStorage__CStorageFolder_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e20debc6-dc4e-542e-a2e7-a24d19c8dd62"))
IVectorView<ABI::Windows::Storage::StorageFolder*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageFolder*, ABI::Windows::Storage::IStorageFolder*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Storage.StorageFolder>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Storage::StorageFolder*> __FIVectorView_1_Windows__CStorage__CStorageFolder_t;
#define __FIVectorView_1_Windows__CStorage__CStorageFolder ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CStorage__CStorageFolder_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CStorage__CStorageFolder_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFolder_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFolder_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ca40b21b-aeb1-5a61-9e08-3bd5d9594023"))
IAsyncOperation<__FIVectorView_1_Windows__CStorage__CStorageFolder*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CStorage__CStorageFolder*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Storage.StorageFolder>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CStorage__CStorageFolder*> __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFolder_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFolder ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFolder_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFolder_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFolder_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFolder_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ed2d1d9b-26ec-5be7-a8a3-56458933d25f"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CStorage__CStorageFolder*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CStorage__CStorageFolder*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Storage.StorageFolder>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CStorage__CStorageFolder*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFolder_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFolder ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFolder_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFolder_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Storage {
            class StorageLibraryChange;
        } /* Storage */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterator_1_Windows__CStorage__CStorageLibraryChange_USE
#define DEF___FIIterator_1_Windows__CStorage__CStorageLibraryChange_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c48a1103-56e6-5398-84fe-92edad7fc111"))
IIterator<ABI::Windows::Storage::StorageLibraryChange*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageLibraryChange*, ABI::Windows::Storage::IStorageLibraryChange*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Storage.StorageLibraryChange>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Storage::StorageLibraryChange*> __FIIterator_1_Windows__CStorage__CStorageLibraryChange_t;
#define __FIIterator_1_Windows__CStorage__CStorageLibraryChange ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CStorage__CStorageLibraryChange_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CStorage__CStorageLibraryChange_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterable_1_Windows__CStorage__CStorageLibraryChange_USE
#define DEF___FIIterable_1_Windows__CStorage__CStorageLibraryChange_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("87c15dfc-0c5e-518b-9206-97d3d9823c61"))
IIterable<ABI::Windows::Storage::StorageLibraryChange*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageLibraryChange*, ABI::Windows::Storage::IStorageLibraryChange*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Storage.StorageLibraryChange>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Storage::StorageLibraryChange*> __FIIterable_1_Windows__CStorage__CStorageLibraryChange_t;
#define __FIIterable_1_Windows__CStorage__CStorageLibraryChange ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CStorage__CStorageLibraryChange_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CStorage__CStorageLibraryChange_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIVectorView_1_Windows__CStorage__CStorageLibraryChange_USE
#define DEF___FIVectorView_1_Windows__CStorage__CStorageLibraryChange_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0d3879e2-5c7e-5b6c-954d-10c6da95fbff"))
IVectorView<ABI::Windows::Storage::StorageLibraryChange*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageLibraryChange*, ABI::Windows::Storage::IStorageLibraryChange*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Storage.StorageLibraryChange>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Storage::StorageLibraryChange*> __FIVectorView_1_Windows__CStorage__CStorageLibraryChange_t;
#define __FIVectorView_1_Windows__CStorage__CStorageLibraryChange ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CStorage__CStorageLibraryChange_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CStorage__CStorageLibraryChange_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("66e11b8a-9003-52c9-84a8-ae5ccebe8cf9"))
IAsyncOperation<__FIVectorView_1_Windows__CStorage__CStorageLibraryChange*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CStorage__CStorageLibraryChange*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Storage.StorageLibraryChange>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CStorage__CStorageLibraryChange*> __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ab9cea41-6df8-535d-8171-46aff187158f"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CStorage__CStorageLibraryChange*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CStorage__CStorageLibraryChange*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Storage.StorageLibraryChange>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CStorage__CStorageLibraryChange*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000


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



#ifndef DEF___FIVector_1_HSTRING_USE
#define DEF___FIVector_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("98b9acc1-4b56-532e-ac73-03d5291cca90"))
IVector<HSTRING> : IVector_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<HSTRING> __FIVector_1_HSTRING_t;
#define __FIVector_1_HSTRING ABI::Windows::Foundation::Collections::__FIVector_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_HSTRING_USE */



#ifndef DEF___FIAsyncOperation_1___FIVector_1_HSTRING_USE
#define DEF___FIAsyncOperation_1___FIVector_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("92b02cd3-aa6e-573d-bc03-8d2309cba3eb"))
IAsyncOperation<__FIVector_1_HSTRING*> : IAsyncOperation_impl<__FIVector_1_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVector`1<String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVector_1_HSTRING*> __FIAsyncOperation_1___FIVector_1_HSTRING_t;
#define __FIAsyncOperation_1___FIVector_1_HSTRING ABI::Windows::Foundation::__FIAsyncOperation_1___FIVector_1_HSTRING_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVector_1_HSTRING_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("fae4b396-97c8-5cc3-bf88-ea3098edf6b2"))
IAsyncOperationCompletedHandler<__FIVector_1_HSTRING*> : IAsyncOperationCompletedHandler_impl<__FIVector_1_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVector`1<String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVector_1_HSTRING*> __FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING_t;
#define __FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING_USE */


namespace ABI {
    namespace Windows {
        namespace Storage {
            class ApplicationData;
        } /* Storage */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CApplicationData_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CApplicationData_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("31456b58-a5cb-5c5b-bd6e-ccce3a7bf4b4"))
IAsyncOperation<ABI::Windows::Storage::ApplicationData*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::ApplicationData*, ABI::Windows::Storage::IApplicationData*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.ApplicationData>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::ApplicationData*> __FIAsyncOperation_1_Windows__CStorage__CApplicationData_t;
#define __FIAsyncOperation_1_Windows__CStorage__CApplicationData ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CApplicationData_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CApplicationData_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CApplicationData_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CApplicationData_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("abafe590-65fe-520a-9d7c-6ab5f1882237"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::ApplicationData*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::ApplicationData*, ABI::Windows::Storage::IApplicationData*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.ApplicationData>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::ApplicationData*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CApplicationData_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CApplicationData ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CApplicationData_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CApplicationData_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                class BasicProperties;
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                interface IBasicProperties;
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties ABI::Windows::Storage::FileProperties::IBasicProperties

#endif // ____x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CFileProperties__CBasicProperties_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CFileProperties__CBasicProperties_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5186131a-4467-504b-977a-0785a8230485"))
IAsyncOperation<ABI::Windows::Storage::FileProperties::BasicProperties*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::FileProperties::BasicProperties*, ABI::Windows::Storage::FileProperties::IBasicProperties*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.FileProperties.BasicProperties>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::FileProperties::BasicProperties*> __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CBasicProperties_t;
#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CBasicProperties ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CBasicProperties_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CFileProperties__CBasicProperties_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CBasicProperties_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CBasicProperties_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c8659aae-4926-52ad-8f60-d89fe5a8df5f"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::FileProperties::BasicProperties*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::FileProperties::BasicProperties*, ABI::Windows::Storage::FileProperties::IBasicProperties*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.FileProperties.BasicProperties>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::FileProperties::BasicProperties*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CBasicProperties_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CBasicProperties ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CBasicProperties_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CBasicProperties_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                class StorageItemThumbnail;
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IRandomAccessStreamWithContentType;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("11c7cc5e-c04e-50e7-a65e-6f6903690c16"))
IAsyncOperation<ABI::Windows::Storage::FileProperties::StorageItemThumbnail*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::FileProperties::StorageItemThumbnail*, ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.FileProperties.StorageItemThumbnail>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::FileProperties::StorageItemThumbnail*> __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail_t;
#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6d0036f2-a8a8-505d-b042-d087dc1fc1b7"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::FileProperties::StorageItemThumbnail*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::FileProperties::StorageItemThumbnail*, ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.FileProperties.StorageItemThumbnail>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::FileProperties::StorageItemThumbnail*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CIStorageItem_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CIStorageItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5fc9c137-ebb7-5e6c-9cba-686f2ec2b0bb"))
IAsyncOperation<ABI::Windows::Storage::IStorageItem*> : IAsyncOperation_impl<ABI::Windows::Storage::IStorageItem*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.IStorageItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::IStorageItem*> __FIAsyncOperation_1_Windows__CStorage__CIStorageItem_t;
#define __FIAsyncOperation_1_Windows__CStorage__CIStorageItem ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CIStorageItem_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CIStorageItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CIStorageItem_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CIStorageItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("92c3102f-a327-5318-a6c1-76f6b2a0abfb"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::IStorageItem*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Storage::IStorageItem*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.IStorageItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::IStorageItem*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CIStorageItem_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CIStorageItem ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CIStorageItem_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CIStorageItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Storage {
            typedef enum KnownFoldersAccessStatus : int KnownFoldersAccessStatus;
        } /* Storage */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatus_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d7f094b5-0ea2-5654-85b9-38ee5de90ffa"))
IAsyncOperation<enum ABI::Windows::Storage::KnownFoldersAccessStatus> : IAsyncOperation_impl<enum ABI::Windows::Storage::KnownFoldersAccessStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.KnownFoldersAccessStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::Storage::KnownFoldersAccessStatus> __FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatus_t;
#define __FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatus ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CKnownFoldersAccessStatus_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CKnownFoldersAccessStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a2f87fc6-4ea5-58cf-9490-181604fbd6a2"))
IAsyncOperationCompletedHandler<enum ABI::Windows::Storage::KnownFoldersAccessStatus> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::Storage::KnownFoldersAccessStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.KnownFoldersAccessStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::Storage::KnownFoldersAccessStatus> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CKnownFoldersAccessStatus_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CKnownFoldersAccessStatus ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CKnownFoldersAccessStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CKnownFoldersAccessStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Provider {
                typedef enum FileUpdateStatus : int FileUpdateStatus;
            } /* Provider */
        } /* Storage */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CProvider__CFileUpdateStatus_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CProvider__CFileUpdateStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8f0f439e-87d0-531f-85b1-54f4528f29c3"))
IAsyncOperation<enum ABI::Windows::Storage::Provider::FileUpdateStatus> : IAsyncOperation_impl<enum ABI::Windows::Storage::Provider::FileUpdateStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.Provider.FileUpdateStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::Storage::Provider::FileUpdateStatus> __FIAsyncOperation_1_Windows__CStorage__CProvider__CFileUpdateStatus_t;
#define __FIAsyncOperation_1_Windows__CStorage__CProvider__CFileUpdateStatus ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CProvider__CFileUpdateStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CProvider__CFileUpdateStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CProvider__CFileUpdateStatus_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CProvider__CFileUpdateStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("bb185a07-0285-5f37-9c7d-2fc6a3e0e6e5"))
IAsyncOperationCompletedHandler<enum ABI::Windows::Storage::Provider::FileUpdateStatus> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::Storage::Provider::FileUpdateStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.Provider.FileUpdateStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::Storage::Provider::FileUpdateStatus> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CProvider__CFileUpdateStatus_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CProvider__CFileUpdateStatus ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CProvider__CFileUpdateStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CProvider__CFileUpdateStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CStorageFile_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CStorageFile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5e52f8ce-aced-5a42-95b4-f674dd84885e"))
IAsyncOperation<ABI::Windows::Storage::StorageFile*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageFile*, ABI::Windows::Storage::IStorageFile*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.StorageFile>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::StorageFile*> __FIAsyncOperation_1_Windows__CStorage__CStorageFile_t;
#define __FIAsyncOperation_1_Windows__CStorage__CStorageFile ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CStorageFile_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CStorageFile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e521c894-2c26-5946-9e61-2b5e188d01ed"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::StorageFile*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageFile*, ABI::Windows::Storage::IStorageFile*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.StorageFile>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::StorageFile*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CStorageFolder_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CStorageFolder_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6be9e7d7-e83a-5cbc-802c-1768960b52c3"))
IAsyncOperation<ABI::Windows::Storage::StorageFolder*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageFolder*, ABI::Windows::Storage::IStorageFolder*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.StorageFolder>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::StorageFolder*> __FIAsyncOperation_1_Windows__CStorage__CStorageFolder_t;
#define __FIAsyncOperation_1_Windows__CStorage__CStorageFolder ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CStorageFolder_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CStorageFolder_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c211026e-9e63-5452-ba54-3a07d6a96874"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::StorageFolder*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageFolder*, ABI::Windows::Storage::IStorageFolder*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.StorageFolder>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::StorageFolder*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Storage {
            class StorageLibrary;
        } /* Storage */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CStorageLibrary_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CStorageLibrary_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2f160a19-99c1-52b9-8dca-14e4ab79f287"))
IAsyncOperation<ABI::Windows::Storage::StorageLibrary*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageLibrary*, ABI::Windows::Storage::IStorageLibrary*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.StorageLibrary>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::StorageLibrary*> __FIAsyncOperation_1_Windows__CStorage__CStorageLibrary_t;
#define __FIAsyncOperation_1_Windows__CStorage__CStorageLibrary ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CStorageLibrary_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CStorageLibrary_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageLibrary_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageLibrary_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("36d560c3-731f-5c70-b907-30bc99bc300f"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::StorageLibrary*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageLibrary*, ABI::Windows::Storage::IStorageLibrary*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.StorageLibrary>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::StorageLibrary*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageLibrary_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageLibrary ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageLibrary_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageLibrary_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Storage {
            class StorageStreamTransaction;
        } /* Storage */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0d81405a-9bd3-5e87-82f4-9b4128a887eb"))
IAsyncOperation<ABI::Windows::Storage::StorageStreamTransaction*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageStreamTransaction*, ABI::Windows::Storage::IStorageStreamTransaction*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.StorageStreamTransaction>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::StorageStreamTransaction*> __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction_t;
#define __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d11739e6-2995-5d33-bfff-51b6041f68c1"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::StorageStreamTransaction*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageStreamTransaction*, ABI::Windows::Storage::IStorageStreamTransaction*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.StorageStreamTransaction>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::StorageStreamTransaction*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("430ecece-1418-5d19-81b2-5ddb381603cc"))
IAsyncOperation<ABI::Windows::Storage::Streams::IRandomAccessStream*> : IAsyncOperation_impl<ABI::Windows::Storage::Streams::IRandomAccessStream*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.Streams.IRandomAccessStream>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::Streams::IRandomAccessStream*> __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_t;
#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("398c4183-793d-5b00-819b-4aef92485e94"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IRandomAccessStream*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Storage::Streams::IRandomAccessStream*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.Streams.IRandomAccessStream>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IRandomAccessStream*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIKeyValuePair_2_HSTRING_IInspectable_USE
#define DEF___FIKeyValuePair_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("09335560-6c6b-5a26-9348-97b781132b20"))
IKeyValuePair<HSTRING, IInspectable*> : IKeyValuePair_impl<HSTRING, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, IInspectable*> __FIKeyValuePair_2_HSTRING_IInspectable_t;
#define __FIKeyValuePair_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

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

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_USE */


namespace ABI {
    namespace Windows {
        namespace Storage {
            class ApplicationDataContainer;
        } /* Storage */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_USE
#define DEF___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5adbc543-2170-5ad9-b35e-968cdb78fb30"))
IKeyValuePair<HSTRING, ABI::Windows::Storage::ApplicationDataContainer*> : IKeyValuePair_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::ApplicationDataContainer*, ABI::Windows::Storage::IApplicationDataContainer*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Storage.ApplicationDataContainer>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, ABI::Windows::Storage::ApplicationDataContainer*> __FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_t;
#define __FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("af3c131d-67aa-5c8d-ae0e-272ba24ae74f"))
IIterator<__FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Storage.ApplicationDataContainer>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer*> __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a785be1d-159e-53ad-9553-598b03dca048"))
IIterable<__FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Storage.ApplicationDataContainer>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer*> __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIMapChangedEventArgs_1_HSTRING_USE
#define DEF___FIMapChangedEventArgs_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("60141efb-f2f9-5377-96fd-f8c60d9558b5"))
IMapChangedEventArgs<HSTRING> : IMapChangedEventArgs_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapChangedEventArgs`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapChangedEventArgs<HSTRING> __FIMapChangedEventArgs_1_HSTRING_t;
#define __FIMapChangedEventArgs_1_HSTRING ABI::Windows::Foundation::Collections::__FIMapChangedEventArgs_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapChangedEventArgs_1_HSTRING_USE */



#ifndef DEF___FIMapView_2_HSTRING_IInspectable_USE
#define DEF___FIMapView_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bb78502a-f79d-54fa-92c9-90c5039fdf7e"))
IMapView<HSTRING, IInspectable*> : IMapView_impl<HSTRING, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, IInspectable*> __FIMapView_2_HSTRING_IInspectable_t;
#define __FIMapView_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_IInspectable_USE */


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer_USE
#define DEF___FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("13624f8d-85cc-5780-a78d-64dba58f2c3c"))
IMapView<HSTRING, ABI::Windows::Storage::ApplicationDataContainer*> : IMapView_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::ApplicationDataContainer*, ABI::Windows::Storage::IApplicationDataContainer*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, Windows.Storage.ApplicationDataContainer>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, ABI::Windows::Storage::ApplicationDataContainer*> __FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer_t;
#define __FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIMap_2_HSTRING_IInspectable_USE
#define DEF___FIMap_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1b0d3570-0877-5ec2-8a2c-3b9539506aca"))
IMap<HSTRING, IInspectable*> : IMap_impl<HSTRING, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMap`2<String, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMap<HSTRING, IInspectable*> __FIMap_2_HSTRING_IInspectable_t;
#define __FIMap_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIMap_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMap_2_HSTRING_IInspectable_USE */



#ifndef DEF___FMapChangedEventHandler_2_HSTRING_IInspectable_USE
#define DEF___FMapChangedEventHandler_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("24f981e5-ddca-538d-aada-a59906084cf1"))
MapChangedEventHandler<HSTRING, IInspectable*> : MapChangedEventHandler_impl<HSTRING, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.MapChangedEventHandler`2<String, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef MapChangedEventHandler<HSTRING, IInspectable*> __FMapChangedEventHandler_2_HSTRING_IInspectable_t;
#define __FMapChangedEventHandler_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FMapChangedEventHandler_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FMapChangedEventHandler_2_HSTRING_IInspectable_USE */



#ifndef DEF___FIObservableMap_2_HSTRING_IInspectable_USE
#define DEF___FIObservableMap_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("236aac9d-fb12-5c4d-a41c-9e445fb4d7ec"))
IObservableMap<HSTRING, IInspectable*> : IObservableMap_impl<HSTRING, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IObservableMap`2<String, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IObservableMap<HSTRING, IInspectable*> __FIObservableMap_2_HSTRING_IInspectable_t;
#define __FIObservableMap_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIObservableMap_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIObservableMap_2_HSTRING_IInspectable_USE */


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CStorage__CStorageFolder_USE
#define DEF___FIVector_1_Windows__CStorage__CStorageFolder_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6c26b7be-5f01-5a60-9dd7-fd17be3a9dd6"))
IVector<ABI::Windows::Storage::StorageFolder*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageFolder*, ABI::Windows::Storage::IStorageFolder*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Storage.StorageFolder>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Storage::StorageFolder*> __FIVector_1_Windows__CStorage__CStorageFolder_t;
#define __FIVector_1_Windows__CStorage__CStorageFolder ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CStorage__CStorageFolder_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CStorage__CStorageFolder_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FVectorChangedEventHandler_1_Windows__CStorage__CStorageFolder_USE
#define DEF___FVectorChangedEventHandler_1_Windows__CStorage__CStorageFolder_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2057b641-4b9b-5338-a19a-e9a951916775"))
VectorChangedEventHandler<ABI::Windows::Storage::StorageFolder*> : VectorChangedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageFolder*, ABI::Windows::Storage::IStorageFolder*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.VectorChangedEventHandler`1<Windows.Storage.StorageFolder>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef VectorChangedEventHandler<ABI::Windows::Storage::StorageFolder*> __FVectorChangedEventHandler_1_Windows__CStorage__CStorageFolder_t;
#define __FVectorChangedEventHandler_1_Windows__CStorage__CStorageFolder ABI::Windows::Foundation::Collections::__FVectorChangedEventHandler_1_Windows__CStorage__CStorageFolder_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FVectorChangedEventHandler_1_Windows__CStorage__CStorageFolder_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIObservableVector_1_Windows__CStorage__CStorageFolder_USE
#define DEF___FIObservableVector_1_Windows__CStorage__CStorageFolder_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("358f44df-2a45-5cb8-9385-1ff66808cde0"))
IObservableVector<ABI::Windows::Storage::StorageFolder*> : IObservableVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageFolder*, ABI::Windows::Storage::IStorageFolder*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IObservableVector`1<Windows.Storage.StorageFolder>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IObservableVector<ABI::Windows::Storage::StorageFolder*> __FIObservableVector_1_Windows__CStorage__CStorageFolder_t;
#define __FIObservableVector_1_Windows__CStorage__CStorageFolder ABI::Windows::Foundation::Collections::__FIObservableVector_1_Windows__CStorage__CStorageFolder_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIObservableVector_1_Windows__CStorage__CStorageFolder_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CStorage__CApplicationData_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CStorage__CApplicationData_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b5348b3b-5081-5ae9-8fa3-4d22d68fb0ea"))
ITypedEventHandler<ABI::Windows::Storage::ApplicationData*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::ApplicationData*, ABI::Windows::Storage::IApplicationData*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Storage.ApplicationData, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Storage::ApplicationData*, IInspectable*> __FITypedEventHandler_2_Windows__CStorage__CApplicationData_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CStorage__CApplicationData_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CStorage__CApplicationData_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CStorage__CApplicationData_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CStorage__CStorageLibrary_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CStorage__CStorageLibrary_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("edc09538-bbae-5b2b-9e81-b449ea7e48fe"))
ITypedEventHandler<ABI::Windows::Storage::StorageLibrary*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Storage::StorageLibrary*, ABI::Windows::Storage::IStorageLibrary*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Storage.StorageLibrary, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Storage::StorageLibrary*, IInspectable*> __FITypedEventHandler_2_Windows__CStorage__CStorageLibrary_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CStorage__CStorageLibrary_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CStorage__CStorageLibrary_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CStorage__CStorageLibrary_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Collections {
                typedef enum CollectionChange : int CollectionChange;
            } /* Collections */
        } /* Foundation */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Collections {
                interface IPropertySet;
            } /* Collections */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet ABI::Windows::Foundation::Collections::IPropertySet

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Collections {
                interface IVectorChangedEventArgs;
            } /* Collections */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs ABI::Windows::Foundation::Collections::IVectorChangedEventArgs

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct DateTime DateTime;
        } /* Foundation */
    } /* Windows */
} /* ABI */

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
        namespace Storage {
            namespace FileProperties {
                class StorageItemContentProperties;
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                interface IStorageItemContentProperties;
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties ABI::Windows::Storage::FileProperties::IStorageItemContentProperties

#endif // ____x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                typedef enum ThumbnailMode : int ThumbnailMode;
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace FileProperties {
                typedef enum ThumbnailOptions : unsigned int ThumbnailOptions;
            } /* FileProperties */
        } /* Storage */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CStorage_CSearch_CIStorageFolderQueryOperations_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CSearch_CIStorageFolderQueryOperations_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Search {
                interface IStorageFolderQueryOperations;
            } /* Search */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CSearch_CIStorageFolderQueryOperations ABI::Windows::Storage::Search::IStorageFolderQueryOperations

#endif // ____x_ABI_CWindows_CStorage_CSearch_CIStorageFolderQueryOperations_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IInputStreamReference;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference ABI::Windows::Storage::Streams::IInputStreamReference

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IOutputStream;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIOutputStream ABI::Windows::Storage::Streams::IOutputStream

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IRandomAccessStreamReference;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference ABI::Windows::Storage::Streams::IRandomAccessStreamReference

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                typedef enum UnicodeEncoding : int UnicodeEncoding;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */

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
        namespace Storage {
            typedef enum ApplicationDataCreateDisposition : int ApplicationDataCreateDisposition;
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            typedef enum ApplicationDataLocality : int ApplicationDataLocality;
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            typedef enum CreationCollisionOption : int CreationCollisionOption;
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            typedef enum FileAccessMode : int FileAccessMode;
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            typedef enum FileAttributes : unsigned int FileAttributes;
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            typedef enum KnownFolderId : int KnownFolderId;
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            typedef enum KnownLibraryId : int KnownLibraryId;
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            typedef enum NameCollisionOption : int NameCollisionOption;
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            typedef enum StorageDeleteOption : int StorageDeleteOption;
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            typedef enum StorageItemTypes : unsigned int StorageItemTypes;
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            typedef enum StorageLibraryChangeType : int StorageLibraryChangeType;
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            typedef enum StorageOpenOptions : unsigned int StorageOpenOptions;
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            typedef enum StreamedFileFailureMode : int StreamedFileFailureMode;
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            class AppDataPaths;
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            class SetVersionDeferral;
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            class SetVersionRequest;
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            class StorageLibraryChangeReader;
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            class StorageLibraryChangeTracker;
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            class StorageLibraryChangeTrackerOptions;
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            class StorageProvider;
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            class StreamedFileDataRequest;
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            class SystemAudioProperties;
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            class SystemDataPaths;
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            class SystemGPSProperties;
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            class SystemImageProperties;
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            class SystemMediaProperties;
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            class SystemMusicProperties;
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            class SystemPhotoProperties;
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            class SystemVideoProperties;
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            class UserDataPaths;
        } /* Storage */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Storage.ApplicationDataCreateDisposition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Storage {
            enum ApplicationDataCreateDisposition : int
            {
                ApplicationDataCreateDisposition_Always = 0,
                ApplicationDataCreateDisposition_Existing = 1,
            };
        } /* Storage */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.ApplicationDataLocality
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Storage {
            enum ApplicationDataLocality : int
            {
                ApplicationDataLocality_Local = 0,
                ApplicationDataLocality_Roaming = 1,
                ApplicationDataLocality_Temporary = 2,
                ApplicationDataLocality_LocalCache = 3,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
                ApplicationDataLocality_SharedLocal = 4,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
            };
        } /* Storage */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.CreationCollisionOption
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Storage {
            enum CreationCollisionOption : int
            {
                CreationCollisionOption_GenerateUniqueName = 0,
                CreationCollisionOption_ReplaceExisting = 1,
                CreationCollisionOption_FailIfExists = 2,
                CreationCollisionOption_OpenIfExists = 3,
            };
        } /* Storage */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.FileAccessMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Storage {
            enum FileAccessMode : int
            {
                FileAccessMode_Read = 0,
                FileAccessMode_ReadWrite = 1,
            };
        } /* Storage */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.FileAttributes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Storage {
            enum FileAttributes : unsigned int
            {
                FileAttributes_Normal = 0,
                FileAttributes_ReadOnly = 0x1,
                FileAttributes_Directory = 0x10,
                FileAttributes_Archive = 0x20,
                FileAttributes_Temporary = 0x100,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                FileAttributes_LocallyIncomplete = 0x200,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
            };

            DEFINE_ENUM_FLAG_OPERATORS(FileAttributes)
        } /* Storage */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.KnownFolderId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Storage {
            enum KnownFolderId : int
            {
                KnownFolderId_AppCaptures = 0,
                KnownFolderId_CameraRoll = 1,
                KnownFolderId_DocumentsLibrary = 2,
                KnownFolderId_HomeGroup = 3,
                KnownFolderId_MediaServerDevices = 4,
                KnownFolderId_MusicLibrary = 5,
                KnownFolderId_Objects3D = 6,
                KnownFolderId_PicturesLibrary = 7,
                KnownFolderId_Playlists = 8,
                KnownFolderId_RecordedCalls = 9,
                KnownFolderId_RemovableDevices = 10,
                KnownFolderId_SavedPictures = 11,
                KnownFolderId_Screenshots = 12,
                KnownFolderId_VideosLibrary = 13,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                KnownFolderId_AllAppMods = 14,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                KnownFolderId_CurrentAppMods = 15,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
                KnownFolderId_DownloadsFolder = 16,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
            };
        } /* Storage */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Storage.KnownFoldersAccessStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
namespace ABI {
    namespace Windows {
        namespace Storage {
            enum KnownFoldersAccessStatus : int
            {
                KnownFoldersAccessStatus_DeniedBySystem = 0,
                KnownFoldersAccessStatus_NotDeclaredByApp = 1,
                KnownFoldersAccessStatus_DeniedByUser = 2,
                KnownFoldersAccessStatus_UserPromptRequired = 3,
                KnownFoldersAccessStatus_Allowed = 4,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
                KnownFoldersAccessStatus_AllowedPerAppFolder = 5,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
            };
        } /* Storage */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Struct Windows.Storage.KnownLibraryId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Storage {
            enum KnownLibraryId : int
            {
                KnownLibraryId_Music = 0,
                KnownLibraryId_Pictures = 1,
                KnownLibraryId_Videos = 2,
                KnownLibraryId_Documents = 3,
            };
        } /* Storage */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.NameCollisionOption
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Storage {
            enum NameCollisionOption : int
            {
                NameCollisionOption_GenerateUniqueName = 0,
                NameCollisionOption_ReplaceExisting = 1,
                NameCollisionOption_FailIfExists = 2,
            };
        } /* Storage */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.StorageDeleteOption
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Storage {
            enum StorageDeleteOption : int
            {
                StorageDeleteOption_Default = 0,
                StorageDeleteOption_PermanentDelete = 1,
            };
        } /* Storage */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.StorageItemTypes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Storage {
            enum StorageItemTypes : unsigned int
            {
                StorageItemTypes_None = 0,
                StorageItemTypes_File = 0x1,
                StorageItemTypes_Folder = 0x2,
            };

            DEFINE_ENUM_FLAG_OPERATORS(StorageItemTypes)
        } /* Storage */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.StorageLibraryChangeType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Storage {
            enum StorageLibraryChangeType : int
            {
                StorageLibraryChangeType_Created = 0,
                StorageLibraryChangeType_Deleted = 1,
                StorageLibraryChangeType_MovedOrRenamed = 2,
                StorageLibraryChangeType_ContentsChanged = 3,
                StorageLibraryChangeType_MovedOutOfLibrary = 4,
                StorageLibraryChangeType_MovedIntoLibrary = 5,
                StorageLibraryChangeType_ContentsReplaced = 6,
                StorageLibraryChangeType_IndexingStatusChanged = 7,
                StorageLibraryChangeType_EncryptionChanged = 8,
                StorageLibraryChangeType_ChangeTrackingLost = 9,
            };
        } /* Storage */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Storage.StorageOpenOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Storage {
            enum StorageOpenOptions : unsigned int
            {
                StorageOpenOptions_None = 0,
                StorageOpenOptions_AllowOnlyReaders = 0x1,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                StorageOpenOptions_AllowReadersAndWriters = 0x2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
            };

            DEFINE_ENUM_FLAG_OPERATORS(StorageOpenOptions)
        } /* Storage */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.StreamedFileFailureMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Storage {
            enum StreamedFileFailureMode : int
            {
                StreamedFileFailureMode_Failed = 0,
                StreamedFileFailureMode_CurrentlyUnavailable = 1,
                StreamedFileFailureMode_Incomplete = 2,
            };
        } /* Storage */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Storage.ApplicationDataSetVersionHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIApplicationDataSetVersionHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIApplicationDataSetVersionHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("a05791e6-cc9f-4687-acab-a364fd785463")
            IApplicationDataSetVersionHandler : public IUnknown
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE Invoke(
                    ABI::Windows::Storage::ISetVersionRequest* setVersionRequest
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IApplicationDataSetVersionHandler = __uuidof(IApplicationDataSetVersionHandler);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIApplicationDataSetVersionHandler;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIApplicationDataSetVersionHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Storage.StreamedFileDataRequestedHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIStreamedFileDataRequestedHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStreamedFileDataRequestedHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("fef6a824-2fe1-4d07-a35b-b77c50b5f4cc")
            IStreamedFileDataRequestedHandler : public IUnknown
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE Invoke(
                    ABI::Windows::Storage::Streams::IOutputStream* stream
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IStreamedFileDataRequestedHandler = __uuidof(IStreamedFileDataRequestedHandler);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStreamedFileDataRequestedHandler;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStreamedFileDataRequestedHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IAppDataPaths
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Storage.AppDataPaths
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CStorage_CIAppDataPaths_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIAppDataPaths_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IAppDataPaths[] = L"Windows.Storage.IAppDataPaths";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("7301d60a-79a2-48c9-9ec0-3fda092f79e1")
            IAppDataPaths : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Cookies(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Desktop(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Documents(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Favorites(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_History(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_InternetCache(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LocalAppData(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ProgramData(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_RoamingAppData(
                    HSTRING* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IAppDataPaths = __uuidof(IAppDataPaths);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIAppDataPaths;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIAppDataPaths_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Storage.IAppDataPathsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Storage.AppDataPaths
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CStorage_CIAppDataPathsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIAppDataPathsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IAppDataPathsStatics[] = L"Windows.Storage.IAppDataPathsStatics";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("d8eb2afe-a9d9-4b14-b999-e3921379d903")
            IAppDataPathsStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE GetForUser(
                    ABI::Windows::System::IUser* user,
                    ABI::Windows::Storage::IAppDataPaths** result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetDefault(
                    ABI::Windows::Storage::IAppDataPaths** result
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IAppDataPathsStatics = __uuidof(IAppDataPathsStatics);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIAppDataPathsStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIAppDataPathsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Storage.IApplicationData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.ApplicationData
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIApplicationData_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIApplicationData_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IApplicationData[] = L"Windows.Storage.IApplicationData";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("c3da6fb7-b744-4b45-b0b8-223a0938d0dc")
            IApplicationData : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Version(
                    UINT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE SetVersionAsync(
                    UINT32 desiredVersion,
                    ABI::Windows::Storage::IApplicationDataSetVersionHandler* handler,
                    ABI::Windows::Foundation::IAsyncAction** setVersionOperation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE ClearAllAsync(
                    ABI::Windows::Foundation::IAsyncAction** clearOperation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE ClearAsync(
                    ABI::Windows::Storage::ApplicationDataLocality locality,
                    ABI::Windows::Foundation::IAsyncAction** clearOperation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LocalSettings(
                    ABI::Windows::Storage::IApplicationDataContainer** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_RoamingSettings(
                    ABI::Windows::Storage::IApplicationDataContainer** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LocalFolder(
                    ABI::Windows::Storage::IStorageFolder** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_RoamingFolder(
                    ABI::Windows::Storage::IStorageFolder** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_TemporaryFolder(
                    ABI::Windows::Storage::IStorageFolder** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE add_DataChanged(
                    __FITypedEventHandler_2_Windows__CStorage__CApplicationData_IInspectable* handler,
                    EventRegistrationToken* token
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE remove_DataChanged(
                    EventRegistrationToken token
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE SignalDataChanged(void) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_RoamingStorageQuota(
                    UINT64* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IApplicationData = __uuidof(IApplicationData);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIApplicationData;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIApplicationData_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IApplicationData2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.ApplicationData
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIApplicationData2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIApplicationData2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IApplicationData2[] = L"Windows.Storage.IApplicationData2";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("9e65cd69-0ba3-4e32-be29-b02de6607638")
            IApplicationData2 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_LocalCacheFolder(
                    ABI::Windows::Storage::IStorageFolder** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IApplicationData2 = __uuidof(IApplicationData2);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIApplicationData2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIApplicationData2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IApplicationData3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.ApplicationData
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIApplicationData3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIApplicationData3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IApplicationData3[] = L"Windows.Storage.IApplicationData3";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("dc222cf4-2772-4c1d-aa2c-c9f743ade8d1")
            IApplicationData3 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE GetPublisherCacheFolder(
                    HSTRING folderName,
                    ABI::Windows::Storage::IStorageFolder** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE ClearPublisherCacheFolderAsync(
                    HSTRING folderName,
                    ABI::Windows::Foundation::IAsyncAction** clearOperation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_SharedLocalFolder(
                    ABI::Windows::Storage::IStorageFolder** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IApplicationData3 = __uuidof(IApplicationData3);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIApplicationData3;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIApplicationData3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IApplicationDataContainer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.ApplicationDataContainer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIApplicationDataContainer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIApplicationDataContainer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IApplicationDataContainer[] = L"Windows.Storage.IApplicationDataContainer";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("c5aefd1e-f467-40ba-8566-ab640a441e1d")
            IApplicationDataContainer : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Name(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Locality(
                    ABI::Windows::Storage::ApplicationDataLocality* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Values(
                    ABI::Windows::Foundation::Collections::IPropertySet** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Containers(
                    __FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateContainer(
                    HSTRING name,
                    ABI::Windows::Storage::ApplicationDataCreateDisposition disposition,
                    ABI::Windows::Storage::IApplicationDataContainer** container
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE DeleteContainer(
                    HSTRING name
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IApplicationDataContainer = __uuidof(IApplicationDataContainer);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIApplicationDataContainer;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIApplicationDataContainer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IApplicationDataStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.ApplicationData
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIApplicationDataStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIApplicationDataStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IApplicationDataStatics[] = L"Windows.Storage.IApplicationDataStatics";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("5612147b-e843-45e3-94d8-06169e3c8e17")
            IApplicationDataStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Current(
                    ABI::Windows::Storage::IApplicationData** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IApplicationDataStatics = __uuidof(IApplicationDataStatics);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIApplicationDataStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIApplicationDataStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IApplicationDataStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.ApplicationData
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIApplicationDataStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIApplicationDataStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IApplicationDataStatics2[] = L"Windows.Storage.IApplicationDataStatics2";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("cd606211-cf49-40a4-a47c-c7f0dbba8107")
            IApplicationDataStatics2 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE GetForUserAsync(
                    ABI::Windows::System::IUser* user,
                    __FIAsyncOperation_1_Windows__CStorage__CApplicationData** getForUserOperation
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IApplicationDataStatics2 = __uuidof(IApplicationDataStatics2);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIApplicationDataStatics2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIApplicationDataStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.ICachedFileManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.CachedFileManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CICachedFileManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CICachedFileManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_ICachedFileManagerStatics[] = L"Windows.Storage.ICachedFileManagerStatics";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("8ffc224a-e782-495d-b614-654c4f0b2370")
            ICachedFileManagerStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE DeferUpdates(
                    ABI::Windows::Storage::IStorageFile* file
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CompleteUpdatesAsync(
                    ABI::Windows::Storage::IStorageFile* file,
                    __FIAsyncOperation_1_Windows__CStorage__CProvider__CFileUpdateStatus** operation
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ICachedFileManagerStatics = __uuidof(ICachedFileManagerStatics);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CICachedFileManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CICachedFileManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IDownloadsFolderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.DownloadsFolder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIDownloadsFolderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIDownloadsFolderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IDownloadsFolderStatics[] = L"Windows.Storage.IDownloadsFolderStatics";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("27862ed0-404e-47df-a1e2-e37308be7b37")
            IDownloadsFolderStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE CreateFileAsync(
                    HSTRING desiredName,
                    __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateFolderAsync(
                    HSTRING desiredName,
                    __FIAsyncOperation_1_Windows__CStorage__CStorageFolder** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateFileWithCollisionOptionAsync(
                    HSTRING desiredName,
                    ABI::Windows::Storage::CreationCollisionOption option,
                    __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateFolderWithCollisionOptionAsync(
                    HSTRING desiredName,
                    ABI::Windows::Storage::CreationCollisionOption option,
                    __FIAsyncOperation_1_Windows__CStorage__CStorageFolder** operation
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IDownloadsFolderStatics = __uuidof(IDownloadsFolderStatics);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIDownloadsFolderStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIDownloadsFolderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IDownloadsFolderStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Storage.DownloadsFolder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IDownloadsFolderStatics2[] = L"Windows.Storage.IDownloadsFolderStatics2";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("e93045bd-8ef8-4f8e-8d15-ac0e265f390d")
            IDownloadsFolderStatics2 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE CreateFileForUserAsync(
                    ABI::Windows::System::IUser* user,
                    HSTRING desiredName,
                    __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateFolderForUserAsync(
                    ABI::Windows::System::IUser* user,
                    HSTRING desiredName,
                    __FIAsyncOperation_1_Windows__CStorage__CStorageFolder** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateFileForUserWithCollisionOptionAsync(
                    ABI::Windows::System::IUser* user,
                    HSTRING desiredName,
                    ABI::Windows::Storage::CreationCollisionOption option,
                    __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateFolderForUserWithCollisionOptionAsync(
                    ABI::Windows::System::IUser* user,
                    HSTRING desiredName,
                    ABI::Windows::Storage::CreationCollisionOption option,
                    __FIAsyncOperation_1_Windows__CStorage__CStorageFolder** operation
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IDownloadsFolderStatics2 = __uuidof(IDownloadsFolderStatics2);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Storage.IFileIOStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.FileIO
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIFileIOStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIFileIOStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IFileIOStatics[] = L"Windows.Storage.IFileIOStatics";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("887411eb-7f54-4732-a5f0-5e43e3b8c2f5")
            IFileIOStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE ReadTextAsync(
                    ABI::Windows::Storage::IStorageFile* file,
                    __FIAsyncOperation_1_HSTRING** textOperation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE ReadTextWithEncodingAsync(
                    ABI::Windows::Storage::IStorageFile* file,
                    ABI::Windows::Storage::Streams::UnicodeEncoding encoding,
                    __FIAsyncOperation_1_HSTRING** textOperation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE WriteTextAsync(
                    ABI::Windows::Storage::IStorageFile* file,
                    HSTRING contents,
                    ABI::Windows::Foundation::IAsyncAction** textOperation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE WriteTextWithEncodingAsync(
                    ABI::Windows::Storage::IStorageFile* file,
                    HSTRING contents,
                    ABI::Windows::Storage::Streams::UnicodeEncoding encoding,
                    ABI::Windows::Foundation::IAsyncAction** textOperation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE AppendTextAsync(
                    ABI::Windows::Storage::IStorageFile* file,
                    HSTRING contents,
                    ABI::Windows::Foundation::IAsyncAction** textOperation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE AppendTextWithEncodingAsync(
                    ABI::Windows::Storage::IStorageFile* file,
                    HSTRING contents,
                    ABI::Windows::Storage::Streams::UnicodeEncoding encoding,
                    ABI::Windows::Foundation::IAsyncAction** textOperation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE ReadLinesAsync(
                    ABI::Windows::Storage::IStorageFile* file,
                    __FIAsyncOperation_1___FIVector_1_HSTRING** linesOperation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE ReadLinesWithEncodingAsync(
                    ABI::Windows::Storage::IStorageFile* file,
                    ABI::Windows::Storage::Streams::UnicodeEncoding encoding,
                    __FIAsyncOperation_1___FIVector_1_HSTRING** linesOperation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE WriteLinesAsync(
                    ABI::Windows::Storage::IStorageFile* file,
                    __FIIterable_1_HSTRING* lines,
                    ABI::Windows::Foundation::IAsyncAction** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE WriteLinesWithEncodingAsync(
                    ABI::Windows::Storage::IStorageFile* file,
                    __FIIterable_1_HSTRING* lines,
                    ABI::Windows::Storage::Streams::UnicodeEncoding encoding,
                    ABI::Windows::Foundation::IAsyncAction** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE AppendLinesAsync(
                    ABI::Windows::Storage::IStorageFile* file,
                    __FIIterable_1_HSTRING* lines,
                    ABI::Windows::Foundation::IAsyncAction** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE AppendLinesWithEncodingAsync(
                    ABI::Windows::Storage::IStorageFile* file,
                    __FIIterable_1_HSTRING* lines,
                    ABI::Windows::Storage::Streams::UnicodeEncoding encoding,
                    ABI::Windows::Foundation::IAsyncAction** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE ReadBufferAsync(
                    ABI::Windows::Storage::IStorageFile* file,
                    __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE WriteBufferAsync(
                    ABI::Windows::Storage::IStorageFile* file,
                    ABI::Windows::Storage::Streams::IBuffer* buffer,
                    ABI::Windows::Foundation::IAsyncAction** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE WriteBytesAsync(
                    ABI::Windows::Storage::IStorageFile* file,
                    UINT32 bufferLength,
                    BYTE* buffer,
                    ABI::Windows::Foundation::IAsyncAction** operation
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IFileIOStatics = __uuidof(IFileIOStatics);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIFileIOStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIFileIOStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IKnownFoldersCameraRollStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.KnownFolders
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIKnownFoldersCameraRollStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIKnownFoldersCameraRollStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IKnownFoldersCameraRollStatics[] = L"Windows.Storage.IKnownFoldersCameraRollStatics";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("5d115e66-27e8-492f-b8e5-2f90896cd4cd")
            IKnownFoldersCameraRollStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_CameraRoll(
                    ABI::Windows::Storage::IStorageFolder** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IKnownFoldersCameraRollStatics = __uuidof(IKnownFoldersCameraRollStatics);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIKnownFoldersCameraRollStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIKnownFoldersCameraRollStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IKnownFoldersPlaylistsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.KnownFolders
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIKnownFoldersPlaylistsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIKnownFoldersPlaylistsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IKnownFoldersPlaylistsStatics[] = L"Windows.Storage.IKnownFoldersPlaylistsStatics";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("dad5ecd6-306f-4d6a-b496-46ba8eb106ce")
            IKnownFoldersPlaylistsStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Playlists(
                    ABI::Windows::Storage::IStorageFolder** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IKnownFoldersPlaylistsStatics = __uuidof(IKnownFoldersPlaylistsStatics);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIKnownFoldersPlaylistsStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIKnownFoldersPlaylistsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IKnownFoldersSavedPicturesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.KnownFolders
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIKnownFoldersSavedPicturesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIKnownFoldersSavedPicturesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IKnownFoldersSavedPicturesStatics[] = L"Windows.Storage.IKnownFoldersSavedPicturesStatics";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("055c93ea-253d-467c-b6ca-a97da1e9a18d")
            IKnownFoldersSavedPicturesStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_SavedPictures(
                    ABI::Windows::Storage::IStorageFolder** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IKnownFoldersSavedPicturesStatics = __uuidof(IKnownFoldersSavedPicturesStatics);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIKnownFoldersSavedPicturesStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIKnownFoldersSavedPicturesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IKnownFoldersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.KnownFolders
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIKnownFoldersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIKnownFoldersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IKnownFoldersStatics[] = L"Windows.Storage.IKnownFoldersStatics";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("5a2a7520-4802-452d-9ad9-4351ada7ec35")
            IKnownFoldersStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_MusicLibrary(
                    ABI::Windows::Storage::IStorageFolder** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_PicturesLibrary(
                    ABI::Windows::Storage::IStorageFolder** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_VideosLibrary(
                    ABI::Windows::Storage::IStorageFolder** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DocumentsLibrary(
                    ABI::Windows::Storage::IStorageFolder** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_HomeGroup(
                    ABI::Windows::Storage::IStorageFolder** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_RemovableDevices(
                    ABI::Windows::Storage::IStorageFolder** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MediaServerDevices(
                    ABI::Windows::Storage::IStorageFolder** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IKnownFoldersStatics = __uuidof(IKnownFoldersStatics);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIKnownFoldersStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIKnownFoldersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IKnownFoldersStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.KnownFolders
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIKnownFoldersStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIKnownFoldersStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IKnownFoldersStatics2[] = L"Windows.Storage.IKnownFoldersStatics2";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("194bd0cd-cf6e-4d07-9d53-e9163a2536e9")
            IKnownFoldersStatics2 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Objects3D(
                    ABI::Windows::Storage::IStorageFolder** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_AppCaptures(
                    ABI::Windows::Storage::IStorageFolder** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_RecordedCalls(
                    ABI::Windows::Storage::IStorageFolder** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IKnownFoldersStatics2 = __uuidof(IKnownFoldersStatics2);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIKnownFoldersStatics2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIKnownFoldersStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IKnownFoldersStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Storage.KnownFolders
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CStorage_CIKnownFoldersStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIKnownFoldersStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IKnownFoldersStatics3[] = L"Windows.Storage.IKnownFoldersStatics3";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("c5194341-9742-4ed5-823d-fc1401148764")
            IKnownFoldersStatics3 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE GetFolderForUserAsync(
                    ABI::Windows::System::IUser* user,
                    ABI::Windows::Storage::KnownFolderId folderId,
                    __FIAsyncOperation_1_Windows__CStorage__CStorageFolder** operation
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IKnownFoldersStatics3 = __uuidof(IKnownFoldersStatics3);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIKnownFoldersStatics3;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIKnownFoldersStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Storage.IKnownFoldersStatics4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Storage.KnownFolders
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CStorage_CIKnownFoldersStatics4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIKnownFoldersStatics4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IKnownFoldersStatics4[] = L"Windows.Storage.IKnownFoldersStatics4";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("1722e6bf-9ff9-4b21-bed5-90ecb13a192e")
            IKnownFoldersStatics4 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE RequestAccessAsync(
                    ABI::Windows::Storage::KnownFolderId folderId,
                    __FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatus** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE RequestAccessForUserAsync(
                    ABI::Windows::System::IUser* user,
                    ABI::Windows::Storage::KnownFolderId folderId,
                    __FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatus** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetFolderAsync(
                    ABI::Windows::Storage::KnownFolderId folderId,
                    __FIAsyncOperation_1_Windows__CStorage__CStorageFolder** operation
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IKnownFoldersStatics4 = __uuidof(IKnownFoldersStatics4);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIKnownFoldersStatics4;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIKnownFoldersStatics4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Storage.IPathIOStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.PathIO
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIPathIOStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIPathIOStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IPathIOStatics[] = L"Windows.Storage.IPathIOStatics";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("0f2f3758-8ec7-4381-922b-8f6c07d288f3")
            IPathIOStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE ReadTextAsync(
                    HSTRING absolutePath,
                    __FIAsyncOperation_1_HSTRING** textOperation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE ReadTextWithEncodingAsync(
                    HSTRING absolutePath,
                    ABI::Windows::Storage::Streams::UnicodeEncoding encoding,
                    __FIAsyncOperation_1_HSTRING** textOperation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE WriteTextAsync(
                    HSTRING absolutePath,
                    HSTRING contents,
                    ABI::Windows::Foundation::IAsyncAction** textOperation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE WriteTextWithEncodingAsync(
                    HSTRING absolutePath,
                    HSTRING contents,
                    ABI::Windows::Storage::Streams::UnicodeEncoding encoding,
                    ABI::Windows::Foundation::IAsyncAction** textOperation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE AppendTextAsync(
                    HSTRING absolutePath,
                    HSTRING contents,
                    ABI::Windows::Foundation::IAsyncAction** textOperation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE AppendTextWithEncodingAsync(
                    HSTRING absolutePath,
                    HSTRING contents,
                    ABI::Windows::Storage::Streams::UnicodeEncoding encoding,
                    ABI::Windows::Foundation::IAsyncAction** textOperation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE ReadLinesAsync(
                    HSTRING absolutePath,
                    __FIAsyncOperation_1___FIVector_1_HSTRING** linesOperation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE ReadLinesWithEncodingAsync(
                    HSTRING absolutePath,
                    ABI::Windows::Storage::Streams::UnicodeEncoding encoding,
                    __FIAsyncOperation_1___FIVector_1_HSTRING** linesOperation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE WriteLinesAsync(
                    HSTRING absolutePath,
                    __FIIterable_1_HSTRING* lines,
                    ABI::Windows::Foundation::IAsyncAction** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE WriteLinesWithEncodingAsync(
                    HSTRING absolutePath,
                    __FIIterable_1_HSTRING* lines,
                    ABI::Windows::Storage::Streams::UnicodeEncoding encoding,
                    ABI::Windows::Foundation::IAsyncAction** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE AppendLinesAsync(
                    HSTRING absolutePath,
                    __FIIterable_1_HSTRING* lines,
                    ABI::Windows::Foundation::IAsyncAction** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE AppendLinesWithEncodingAsync(
                    HSTRING absolutePath,
                    __FIIterable_1_HSTRING* lines,
                    ABI::Windows::Storage::Streams::UnicodeEncoding encoding,
                    ABI::Windows::Foundation::IAsyncAction** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE ReadBufferAsync(
                    HSTRING absolutePath,
                    __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE WriteBufferAsync(
                    HSTRING absolutePath,
                    ABI::Windows::Storage::Streams::IBuffer* buffer,
                    ABI::Windows::Foundation::IAsyncAction** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE WriteBytesAsync(
                    HSTRING absolutePath,
                    UINT32 bufferLength,
                    BYTE* buffer,
                    ABI::Windows::Foundation::IAsyncAction** operation
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPathIOStatics = __uuidof(IPathIOStatics);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIPathIOStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIPathIOStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.ISetVersionDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.SetVersionDeferral
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CISetVersionDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CISetVersionDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_ISetVersionDeferral[] = L"Windows.Storage.ISetVersionDeferral";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("033508a2-781a-437a-b078-3f32badcfe47")
            ISetVersionDeferral : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE Complete(void) = 0;
            };

            MIDL_CONST_ID IID& IID_ISetVersionDeferral = __uuidof(ISetVersionDeferral);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CISetVersionDeferral;
#endif /* !defined(____x_ABI_CWindows_CStorage_CISetVersionDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.ISetVersionRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.SetVersionRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CISetVersionRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CISetVersionRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_ISetVersionRequest[] = L"Windows.Storage.ISetVersionRequest";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("b9c76b9b-1056-4e69-8330-162619956f9b")
            ISetVersionRequest : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_CurrentVersion(
                    UINT32* currentVersion
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DesiredVersion(
                    UINT32* desiredVersion
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                    ABI::Windows::Storage::ISetVersionDeferral** deferral
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ISetVersionRequest = __uuidof(ISetVersionRequest);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CISetVersionRequest;
#endif /* !defined(____x_ABI_CWindows_CStorage_CISetVersionRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IStorageFile
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Storage.IStorageItem
 *     Windows.Storage.Streams.IRandomAccessStreamReference
 *     Windows.Storage.Streams.IInputStreamReference
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageFile_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageFile_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageFile[] = L"Windows.Storage.IStorageFile";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("fa3f6186-4214-428c-a64c-14c9ac7315ea")
            IStorageFile : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_FileType(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ContentType(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE OpenAsync(
                    ABI::Windows::Storage::FileAccessMode accessMode,
                    __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE OpenTransactedWriteAsync(
                    __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CopyOverloadDefaultNameAndOptions(
                    ABI::Windows::Storage::IStorageFolder* destinationFolder,
                    __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CopyOverloadDefaultOptions(
                    ABI::Windows::Storage::IStorageFolder* destinationFolder,
                    HSTRING desiredNewName,
                    __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CopyOverload(
                    ABI::Windows::Storage::IStorageFolder* destinationFolder,
                    HSTRING desiredNewName,
                    ABI::Windows::Storage::NameCollisionOption option,
                    __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CopyAndReplaceAsync(
                    ABI::Windows::Storage::IStorageFile* fileToReplace,
                    ABI::Windows::Foundation::IAsyncAction** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE MoveOverloadDefaultNameAndOptions(
                    ABI::Windows::Storage::IStorageFolder* destinationFolder,
                    ABI::Windows::Foundation::IAsyncAction** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE MoveOverloadDefaultOptions(
                    ABI::Windows::Storage::IStorageFolder* destinationFolder,
                    HSTRING desiredNewName,
                    ABI::Windows::Foundation::IAsyncAction** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE MoveOverload(
                    ABI::Windows::Storage::IStorageFolder* destinationFolder,
                    HSTRING desiredNewName,
                    ABI::Windows::Storage::NameCollisionOption option,
                    ABI::Windows::Foundation::IAsyncAction** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE MoveAndReplaceAsync(
                    ABI::Windows::Storage::IStorageFile* fileToReplace,
                    ABI::Windows::Foundation::IAsyncAction** operation
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IStorageFile = __uuidof(IStorageFile);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageFile;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageFile_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IStorageFile2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageFile2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageFile2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageFile2[] = L"Windows.Storage.IStorageFile2";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("954e4bcf-0a77-42fb-b777-c2ed58a52e44")
            IStorageFile2 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE OpenWithOptionsAsync(
                    ABI::Windows::Storage::FileAccessMode accessMode,
                    ABI::Windows::Storage::StorageOpenOptions options,
                    __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE OpenTransactedWriteWithOptionsAsync(
                    ABI::Windows::Storage::StorageOpenOptions options,
                    __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction** operation
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IStorageFile2 = __uuidof(IStorageFile2);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageFile2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageFile2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IStorageFilePropertiesWithAvailability
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageFilePropertiesWithAvailability[] = L"Windows.Storage.IStorageFilePropertiesWithAvailability";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("afcbbe9b-582b-4133-9648-e44ca46ee491")
            IStorageFilePropertiesWithAvailability : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_IsAvailable(
                    boolean* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IStorageFilePropertiesWithAvailability = __uuidof(IStorageFilePropertiesWithAvailability);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IStorageFileStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageFile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageFileStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageFileStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageFileStatics[] = L"Windows.Storage.IStorageFileStatics";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("5984c710-daf2-43c8-8bb4-a4d3eacfd03f")
            IStorageFileStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE GetFileFromPathAsync(
                    HSTRING path,
                    __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetFileFromApplicationUriAsync(
                    ABI::Windows::Foundation::IUriRuntimeClass* uri,
                    __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateStreamedFileAsync(
                    HSTRING displayNameWithExtension,
                    ABI::Windows::Storage::IStreamedFileDataRequestedHandler* dataRequested,
                    ABI::Windows::Storage::Streams::IRandomAccessStreamReference* thumbnail,
                    __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE ReplaceWithStreamedFileAsync(
                    ABI::Windows::Storage::IStorageFile* fileToReplace,
                    ABI::Windows::Storage::IStreamedFileDataRequestedHandler* dataRequested,
                    ABI::Windows::Storage::Streams::IRandomAccessStreamReference* thumbnail,
                    __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateStreamedFileFromUriAsync(
                    HSTRING displayNameWithExtension,
                    ABI::Windows::Foundation::IUriRuntimeClass* uri,
                    ABI::Windows::Storage::Streams::IRandomAccessStreamReference* thumbnail,
                    __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE ReplaceWithStreamedFileFromUriAsync(
                    ABI::Windows::Storage::IStorageFile* fileToReplace,
                    ABI::Windows::Foundation::IUriRuntimeClass* uri,
                    ABI::Windows::Storage::Streams::IRandomAccessStreamReference* thumbnail,
                    __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IStorageFileStatics = __uuidof(IStorageFileStatics);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageFileStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageFileStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IStorageFileStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageFile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageFileStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageFileStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageFileStatics2[] = L"Windows.Storage.IStorageFileStatics2";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("5c76a781-212e-4af9-8f04-740cae108974")
            IStorageFileStatics2 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE GetFileFromPathForUserAsync(
                    ABI::Windows::System::IUser* user,
                    HSTRING path,
                    __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IStorageFileStatics2 = __uuidof(IStorageFileStatics2);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageFileStatics2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageFileStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Storage.IStorageFolder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Storage.IStorageItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageFolder_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageFolder_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageFolder[] = L"Windows.Storage.IStorageFolder";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("72d1cb78-b3ef-4f75-a80b-6fd9dae2944b")
            IStorageFolder : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE CreateFileAsyncOverloadDefaultOptions(
                    HSTRING desiredName,
                    __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateFileAsync(
                    HSTRING desiredName,
                    ABI::Windows::Storage::CreationCollisionOption options,
                    __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateFolderAsyncOverloadDefaultOptions(
                    HSTRING desiredName,
                    __FIAsyncOperation_1_Windows__CStorage__CStorageFolder** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateFolderAsync(
                    HSTRING desiredName,
                    ABI::Windows::Storage::CreationCollisionOption options,
                    __FIAsyncOperation_1_Windows__CStorage__CStorageFolder** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetFileAsync(
                    HSTRING name,
                    __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetFolderAsync(
                    HSTRING name,
                    __FIAsyncOperation_1_Windows__CStorage__CStorageFolder** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetItemAsync(
                    HSTRING name,
                    __FIAsyncOperation_1_Windows__CStorage__CIStorageItem** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetFilesAsyncOverloadDefaultOptionsStartAndCount(
                    __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetFoldersAsyncOverloadDefaultOptionsStartAndCount(
                    __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFolder** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetItemsAsyncOverloadDefaultStartAndCount(
                    __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem** operation
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IStorageFolder = __uuidof(IStorageFolder);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageFolder;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageFolder_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IStorageFolder2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageFolder2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageFolder2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageFolder2[] = L"Windows.Storage.IStorageFolder2";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("e827e8b9-08d9-4a8e-a0ac-fe5ed3cbbbd3")
            IStorageFolder2 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE TryGetItemAsync(
                    HSTRING name,
                    __FIAsyncOperation_1_Windows__CStorage__CIStorageItem** operation
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IStorageFolder2 = __uuidof(IStorageFolder2);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageFolder2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageFolder2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IStorageFolder3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageFolder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageFolder3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageFolder3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageFolder3[] = L"Windows.Storage.IStorageFolder3";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("9f617899-bde1-4124-aeb3-b06ad96f98d4")
            IStorageFolder3 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE TryGetChangeTracker(
                    ABI::Windows::Storage::IStorageLibraryChangeTracker** result
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IStorageFolder3 = __uuidof(IStorageFolder3);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageFolder3;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageFolder3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Storage.IStorageFolderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageFolder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageFolderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageFolderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageFolderStatics[] = L"Windows.Storage.IStorageFolderStatics";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("08f327ff-85d5-48b9-aee9-28511e339f9f")
            IStorageFolderStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE GetFolderFromPathAsync(
                    HSTRING path,
                    __FIAsyncOperation_1_Windows__CStorage__CStorageFolder** operation
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IStorageFolderStatics = __uuidof(IStorageFolderStatics);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageFolderStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageFolderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IStorageFolderStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageFolder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageFolderStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageFolderStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageFolderStatics2[] = L"Windows.Storage.IStorageFolderStatics2";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("b4656dc3-71d2-467d-8b29-371f0f62bf6f")
            IStorageFolderStatics2 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE GetFolderFromPathForUserAsync(
                    ABI::Windows::System::IUser* user,
                    HSTRING path,
                    __FIAsyncOperation_1_Windows__CStorage__CStorageFolder** operation
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IStorageFolderStatics2 = __uuidof(IStorageFolderStatics2);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageFolderStatics2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageFolderStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Storage.IStorageItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageItem[] = L"Windows.Storage.IStorageItem";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("4207a996-ca2f-42f7-bde8-8b10457a7f30")
            IStorageItem : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE RenameAsyncOverloadDefaultOptions(
                    HSTRING desiredName,
                    ABI::Windows::Foundation::IAsyncAction** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE RenameAsync(
                    HSTRING desiredName,
                    ABI::Windows::Storage::NameCollisionOption option,
                    ABI::Windows::Foundation::IAsyncAction** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE DeleteAsyncOverloadDefaultOptions(
                    ABI::Windows::Foundation::IAsyncAction** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE DeleteAsync(
                    ABI::Windows::Storage::StorageDeleteOption option,
                    ABI::Windows::Foundation::IAsyncAction** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetBasicPropertiesAsync(
                    __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CBasicProperties** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Name(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Path(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Attributes(
                    ABI::Windows::Storage::FileAttributes* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DateCreated(
                    ABI::Windows::Foundation::DateTime* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE IsOfType(
                    ABI::Windows::Storage::StorageItemTypes type,
                    boolean* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IStorageItem = __uuidof(IStorageItem);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageItem;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IStorageItem2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Storage.IStorageItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageItem2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageItem2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageItem2[] = L"Windows.Storage.IStorageItem2";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("53f926d2-083c-4283-b45b-81c007237e44")
            IStorageItem2 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE GetParentAsync(
                    __FIAsyncOperation_1_Windows__CStorage__CStorageFolder** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE IsEqual(
                    ABI::Windows::Storage::IStorageItem* item,
                    boolean* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IStorageItem2 = __uuidof(IStorageItem2);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageItem2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageItem2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IStorageItemProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageItemProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageItemProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageItemProperties[] = L"Windows.Storage.IStorageItemProperties";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("86664478-8029-46fe-a789-1c2f3e2ffb5c")
            IStorageItemProperties : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(
                    ABI::Windows::Storage::FileProperties::ThumbnailMode mode,
                    __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetThumbnailAsyncOverloadDefaultOptions(
                    ABI::Windows::Storage::FileProperties::ThumbnailMode mode,
                    UINT32 requestedSize,
                    __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetThumbnailAsync(
                    ABI::Windows::Storage::FileProperties::ThumbnailMode mode,
                    UINT32 requestedSize,
                    ABI::Windows::Storage::FileProperties::ThumbnailOptions options,
                    __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DisplayType(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_FolderRelativeId(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Properties(
                    ABI::Windows::Storage::FileProperties::IStorageItemContentProperties** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IStorageItemProperties = __uuidof(IStorageItemProperties);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageItemProperties;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageItemProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IStorageItemProperties2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Storage.IStorageItemProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageItemProperties2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageItemProperties2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageItemProperties2[] = L"Windows.Storage.IStorageItemProperties2";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("8e86a951-04b9-4bd2-929d-fef3f71621d0")
            IStorageItemProperties2 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions(
                    ABI::Windows::Storage::FileProperties::ThumbnailMode mode,
                    __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetScaledImageAsThumbnailAsyncOverloadDefaultOptions(
                    ABI::Windows::Storage::FileProperties::ThumbnailMode mode,
                    UINT32 requestedSize,
                    __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetScaledImageAsThumbnailAsync(
                    ABI::Windows::Storage::FileProperties::ThumbnailMode mode,
                    UINT32 requestedSize,
                    ABI::Windows::Storage::FileProperties::ThumbnailOptions options,
                    __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail** operation
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IStorageItemProperties2 = __uuidof(IStorageItemProperties2);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageItemProperties2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageItemProperties2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IStorageItemPropertiesWithProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Storage.IStorageItemProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageItemPropertiesWithProvider[] = L"Windows.Storage.IStorageItemPropertiesWithProvider";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("861bf39b-6368-4dee-b40e-74684a5ce714")
            IStorageItemPropertiesWithProvider : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Provider(
                    ABI::Windows::Storage::IStorageProvider** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IStorageItemPropertiesWithProvider = __uuidof(IStorageItemPropertiesWithProvider);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IStorageLibrary
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageLibrary
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageLibrary_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageLibrary_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageLibrary[] = L"Windows.Storage.IStorageLibrary";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("1edd7103-0e5e-4d6c-b5e8-9318983d6a03")
            IStorageLibrary : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE RequestAddFolderAsync(
                    __FIAsyncOperation_1_Windows__CStorage__CStorageFolder** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE RequestRemoveFolderAsync(
                    ABI::Windows::Storage::IStorageFolder* folder,
                    __FIAsyncOperation_1_boolean** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Folders(
                    __FIObservableVector_1_Windows__CStorage__CStorageFolder** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_SaveFolder(
                    ABI::Windows::Storage::IStorageFolder** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE add_DefinitionChanged(
                    __FITypedEventHandler_2_Windows__CStorage__CStorageLibrary_IInspectable* handler,
                    EventRegistrationToken* eventCookie
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE remove_DefinitionChanged(
                    EventRegistrationToken eventCookie
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IStorageLibrary = __uuidof(IStorageLibrary);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageLibrary;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageLibrary_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IStorageLibrary2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageLibrary
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageLibrary2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageLibrary2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageLibrary2[] = L"Windows.Storage.IStorageLibrary2";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("5b0ce348-fcb3-4031-afb0-a68d7bd44534")
            IStorageLibrary2 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_ChangeTracker(
                    ABI::Windows::Storage::IStorageLibraryChangeTracker** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IStorageLibrary2 = __uuidof(IStorageLibrary2);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageLibrary2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageLibrary2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Storage.IStorageLibrary3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageLibrary
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageLibrary3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageLibrary3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageLibrary3[] = L"Windows.Storage.IStorageLibrary3";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("8a281291-2154-4201-8113-d2c05ce1ad23")
            IStorageLibrary3 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE AreFolderSuggestionsAvailableAsync(
                    __FIAsyncOperation_1_boolean** operation
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IStorageLibrary3 = __uuidof(IStorageLibrary3);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageLibrary3;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageLibrary3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Storage.IStorageLibraryChange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageLibraryChange
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryChange_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryChange_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageLibraryChange[] = L"Windows.Storage.IStorageLibraryChange";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("00980b23-2be2-4909-aa48-159f5203a51e")
            IStorageLibraryChange : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_ChangeType(
                    ABI::Windows::Storage::StorageLibraryChangeType* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Path(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_PreviousPath(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE IsOfType(
                    ABI::Windows::Storage::StorageItemTypes type,
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetStorageItemAsync(
                    __FIAsyncOperation_1_Windows__CStorage__CIStorageItem** operation
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IStorageLibraryChange = __uuidof(IStorageLibraryChange);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageLibraryChange;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryChange_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Storage.IStorageLibraryChangeReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageLibraryChangeReader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageLibraryChangeReader[] = L"Windows.Storage.IStorageLibraryChangeReader";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("f205bc83-fca2-41f9-8954-ee2e991eb96f")
            IStorageLibraryChangeReader : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE ReadBatchAsync(
                    __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE AcceptChangesAsync(
                    ABI::Windows::Foundation::IAsyncAction** operation
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IStorageLibraryChangeReader = __uuidof(IStorageLibraryChangeReader);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Storage.IStorageLibraryChangeReader2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageLibraryChangeReader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageLibraryChangeReader2[] = L"Windows.Storage.IStorageLibraryChangeReader2";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("abf4868b-fbcc-4a4f-999e-e7ab7c646dbe")
            IStorageLibraryChangeReader2 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE GetLastChangeId(
                    UINT64* result
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IStorageLibraryChangeReader2 = __uuidof(IStorageLibraryChangeReader2);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

/*
 *
 * Interface Windows.Storage.IStorageLibraryChangeTracker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageLibraryChangeTracker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageLibraryChangeTracker[] = L"Windows.Storage.IStorageLibraryChangeTracker";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("9e157316-6073-44f6-9681-7492d1286c90")
            IStorageLibraryChangeTracker : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE GetChangeReader(
                    ABI::Windows::Storage::IStorageLibraryChangeReader** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE Enable(void) = 0;
                virtual HRESULT STDMETHODCALLTYPE Reset(void) = 0;
            };

            MIDL_CONST_ID IID& IID_IStorageLibraryChangeTracker = __uuidof(IStorageLibraryChangeTracker);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Storage.IStorageLibraryChangeTracker2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageLibraryChangeTracker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageLibraryChangeTracker2[] = L"Windows.Storage.IStorageLibraryChangeTracker2";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("cd051c3b-0f9f-42f9-8fb3-158d82e13821")
            IStorageLibraryChangeTracker2 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE EnableWithOptions(
                    ABI::Windows::Storage::IStorageLibraryChangeTrackerOptions* options
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE Disable(void) = 0;
            };

            MIDL_CONST_ID IID& IID_IStorageLibraryChangeTracker2 = __uuidof(IStorageLibraryChangeTracker2);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

/*
 *
 * Interface Windows.Storage.IStorageLibraryChangeTrackerOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageLibraryChangeTrackerOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageLibraryChangeTrackerOptions[] = L"Windows.Storage.IStorageLibraryChangeTrackerOptions";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("bb52bcd4-1a6d-59c0-ad2a-823a20532483")
            IStorageLibraryChangeTrackerOptions : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_TrackChangeDetails(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_TrackChangeDetails(
                    boolean value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IStorageLibraryChangeTrackerOptions = __uuidof(IStorageLibraryChangeTrackerOptions);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptions;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

/*
 *
 * Interface Windows.Storage.IStorageLibraryLastChangeId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageLibraryLastChangeId
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeId_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeId_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageLibraryLastChangeId[] = L"Windows.Storage.IStorageLibraryLastChangeId";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("5281826a-bbe1-53bc-82ca-81cc7f039329")
            IStorageLibraryLastChangeId : public IInspectable
            {
            public:
            };

            MIDL_CONST_ID IID& IID_IStorageLibraryLastChangeId = __uuidof(IStorageLibraryLastChangeId);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeId;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeId_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

/*
 *
 * Interface Windows.Storage.IStorageLibraryLastChangeIdStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageLibraryLastChangeId
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageLibraryLastChangeIdStatics[] = L"Windows.Storage.IStorageLibraryLastChangeIdStatics";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("81a49128-2ca3-5309-b0d1-cf0788e40762")
            IStorageLibraryLastChangeIdStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Unknown(
                    UINT64* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IStorageLibraryLastChangeIdStatics = __uuidof(IStorageLibraryLastChangeIdStatics);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

/*
 *
 * Interface Windows.Storage.IStorageLibraryStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageLibrary
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageLibraryStatics[] = L"Windows.Storage.IStorageLibraryStatics";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("4208a6db-684a-49c6-9e59-90121ee050d6")
            IStorageLibraryStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE GetLibraryAsync(
                    ABI::Windows::Storage::KnownLibraryId libraryId,
                    __FIAsyncOperation_1_Windows__CStorage__CStorageLibrary** operation
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IStorageLibraryStatics = __uuidof(IStorageLibraryStatics);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageLibraryStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IStorageLibraryStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageLibrary
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageLibraryStatics2[] = L"Windows.Storage.IStorageLibraryStatics2";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("ffb08ddc-fa75-4695-b9d1-7f81f97832e3")
            IStorageLibraryStatics2 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE GetLibraryForUserAsync(
                    ABI::Windows::System::IUser* user,
                    ABI::Windows::Storage::KnownLibraryId libraryId,
                    __FIAsyncOperation_1_Windows__CStorage__CStorageLibrary** operation
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IStorageLibraryStatics2 = __uuidof(IStorageLibraryStatics2);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageLibraryStatics2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Storage.IStorageProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageProvider[] = L"Windows.Storage.IStorageProvider";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("e705eed4-d478-47d6-ba46-1a8ebe114a20")
            IStorageProvider : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Id(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                    HSTRING* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IStorageProvider = __uuidof(IStorageProvider);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageProvider;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IStorageProvider2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageProvider
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Storage.IStorageProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageProvider2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageProvider2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageProvider2[] = L"Windows.Storage.IStorageProvider2";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("010d1917-3404-414b-9fd7-cd44472eaa39")
            IStorageProvider2 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE IsPropertySupportedForPartialFileAsync(
                    HSTRING propertyCanonicalName,
                    __FIAsyncOperation_1_boolean** operation
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IStorageProvider2 = __uuidof(IStorageProvider2);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageProvider2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageProvider2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Storage.IStorageStreamTransaction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageStreamTransaction
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageStreamTransaction_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageStreamTransaction_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageStreamTransaction[] = L"Windows.Storage.IStorageStreamTransaction";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("f67cf363-a53d-4d94-ae2c-67232d93acdd")
            IStorageStreamTransaction : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Stream(
                    ABI::Windows::Storage::Streams::IRandomAccessStream** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CommitAsync(
                    ABI::Windows::Foundation::IAsyncAction** operation
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IStorageStreamTransaction = __uuidof(IStorageStreamTransaction);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageStreamTransaction;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageStreamTransaction_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IStreamedFileDataRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIStreamedFileDataRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStreamedFileDataRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStreamedFileDataRequest[] = L"Windows.Storage.IStreamedFileDataRequest";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("1673fcce-dabd-4d50-beee-180b8a8191b6")
            IStreamedFileDataRequest : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE FailAndClose(
                    ABI::Windows::Storage::StreamedFileFailureMode failureMode
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IStreamedFileDataRequest = __uuidof(IStreamedFileDataRequest);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStreamedFileDataRequest;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStreamedFileDataRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.ISystemAudioProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.SystemAudioProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CISystemAudioProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CISystemAudioProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_ISystemAudioProperties[] = L"Windows.Storage.ISystemAudioProperties";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("3f8f38b7-308c-47e1-924d-8645348e5db7")
            ISystemAudioProperties : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_EncodingBitrate(
                    HSTRING* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ISystemAudioProperties = __uuidof(ISystemAudioProperties);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CISystemAudioProperties;
#endif /* !defined(____x_ABI_CWindows_CStorage_CISystemAudioProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.ISystemDataPaths
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Storage.SystemDataPaths
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CStorage_CISystemDataPaths_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CISystemDataPaths_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_ISystemDataPaths[] = L"Windows.Storage.ISystemDataPaths";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("e32abf70-d8fa-45ec-a942-d2e26fb60ba5")
            ISystemDataPaths : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Fonts(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ProgramData(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Public(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_PublicDesktop(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_PublicDocuments(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_PublicDownloads(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_PublicMusic(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_PublicPictures(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_PublicVideos(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_System(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_SystemHost(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_SystemX86(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_SystemX64(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_SystemArm(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_UserProfiles(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Windows(
                    HSTRING* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ISystemDataPaths = __uuidof(ISystemDataPaths);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CISystemDataPaths;
#endif /* !defined(____x_ABI_CWindows_CStorage_CISystemDataPaths_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Storage.ISystemDataPathsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Storage.SystemDataPaths
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CStorage_CISystemDataPathsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CISystemDataPathsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_ISystemDataPathsStatics[] = L"Windows.Storage.ISystemDataPathsStatics";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("e0f96fd0-9920-4bca-b379-f96fdf7caad8")
            ISystemDataPathsStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE GetDefault(
                    ABI::Windows::Storage::ISystemDataPaths** result
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ISystemDataPathsStatics = __uuidof(ISystemDataPathsStatics);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CISystemDataPathsStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CISystemDataPathsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Storage.ISystemGPSProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.SystemGPSProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CISystemGPSProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CISystemGPSProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_ISystemGPSProperties[] = L"Windows.Storage.ISystemGPSProperties";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("c0f46eb4-c174-481a-bc25-921986f6a6f3")
            ISystemGPSProperties : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_LatitudeDecimal(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LongitudeDecimal(
                    HSTRING* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ISystemGPSProperties = __uuidof(ISystemGPSProperties);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CISystemGPSProperties;
#endif /* !defined(____x_ABI_CWindows_CStorage_CISystemGPSProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.ISystemImageProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.SystemImageProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CISystemImageProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CISystemImageProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_ISystemImageProperties[] = L"Windows.Storage.ISystemImageProperties";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("011b2e30-8b39-4308-bea1-e8aa61e47826")
            ISystemImageProperties : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_HorizontalSize(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_VerticalSize(
                    HSTRING* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ISystemImageProperties = __uuidof(ISystemImageProperties);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CISystemImageProperties;
#endif /* !defined(____x_ABI_CWindows_CStorage_CISystemImageProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.ISystemMediaProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.SystemMediaProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CISystemMediaProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CISystemMediaProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_ISystemMediaProperties[] = L"Windows.Storage.ISystemMediaProperties";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("a42b3316-8415-40dc-8c44-98361d235430")
            ISystemMediaProperties : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Duration(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Producer(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Publisher(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_SubTitle(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Writer(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Year(
                    HSTRING* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ISystemMediaProperties = __uuidof(ISystemMediaProperties);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CISystemMediaProperties;
#endif /* !defined(____x_ABI_CWindows_CStorage_CISystemMediaProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.ISystemMusicProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.SystemMusicProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CISystemMusicProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CISystemMusicProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_ISystemMusicProperties[] = L"Windows.Storage.ISystemMusicProperties";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("b47988d5-67af-4bc3-8d39-5b89022026a1")
            ISystemMusicProperties : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_AlbumArtist(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_AlbumTitle(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Artist(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Composer(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Conductor(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DisplayArtist(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Genre(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_TrackNumber(
                    HSTRING* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ISystemMusicProperties = __uuidof(ISystemMusicProperties);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CISystemMusicProperties;
#endif /* !defined(____x_ABI_CWindows_CStorage_CISystemMusicProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.ISystemPhotoProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.SystemPhotoProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CISystemPhotoProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CISystemPhotoProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_ISystemPhotoProperties[] = L"Windows.Storage.ISystemPhotoProperties";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("4734fc3d-ab21-4424-b735-f4353a56c8fc")
            ISystemPhotoProperties : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_CameraManufacturer(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_CameraModel(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DateTaken(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Orientation(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_PeopleNames(
                    HSTRING* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ISystemPhotoProperties = __uuidof(ISystemPhotoProperties);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CISystemPhotoProperties;
#endif /* !defined(____x_ABI_CWindows_CStorage_CISystemPhotoProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.ISystemProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.SystemProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CISystemProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CISystemProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_ISystemProperties[] = L"Windows.Storage.ISystemProperties";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("917a71c1-85f3-4dd1-b001-a50bfd21c8d2")
            ISystemProperties : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Author(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Comment(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ItemNameDisplay(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Keywords(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Rating(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Title(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Audio(
                    ABI::Windows::Storage::ISystemAudioProperties** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_GPS(
                    ABI::Windows::Storage::ISystemGPSProperties** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Media(
                    ABI::Windows::Storage::ISystemMediaProperties** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Music(
                    ABI::Windows::Storage::ISystemMusicProperties** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Photo(
                    ABI::Windows::Storage::ISystemPhotoProperties** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Video(
                    ABI::Windows::Storage::ISystemVideoProperties** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Image(
                    ABI::Windows::Storage::ISystemImageProperties** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ISystemProperties = __uuidof(ISystemProperties);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CISystemProperties;
#endif /* !defined(____x_ABI_CWindows_CStorage_CISystemProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.ISystemVideoProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.SystemVideoProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CISystemVideoProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CISystemVideoProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_ISystemVideoProperties[] = L"Windows.Storage.ISystemVideoProperties";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("2040f715-67f8-4322-9b80-4fa9fefb83e8")
            ISystemVideoProperties : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Director(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_FrameHeight(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_FrameWidth(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Orientation(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_TotalBitrate(
                    HSTRING* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ISystemVideoProperties = __uuidof(ISystemVideoProperties);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CISystemVideoProperties;
#endif /* !defined(____x_ABI_CWindows_CStorage_CISystemVideoProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IUserDataPaths
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Storage.UserDataPaths
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CStorage_CIUserDataPaths_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIUserDataPaths_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IUserDataPaths[] = L"Windows.Storage.IUserDataPaths";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("f9c53912-abc4-46ff-8a2b-dc9d7fa6e52f")
            IUserDataPaths : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_CameraRoll(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Cookies(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Desktop(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Documents(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Downloads(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Favorites(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_History(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_InternetCache(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LocalAppData(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LocalAppDataLow(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Music(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Pictures(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Profile(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Recent(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_RoamingAppData(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_SavedPictures(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Screenshots(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Templates(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Videos(
                    HSTRING* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IUserDataPaths = __uuidof(IUserDataPaths);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIUserDataPaths;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIUserDataPaths_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Storage.IUserDataPathsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Storage.UserDataPaths
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CStorage_CIUserDataPathsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIUserDataPathsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IUserDataPathsStatics[] = L"Windows.Storage.IUserDataPathsStatics";
namespace ABI {
    namespace Windows {
        namespace Storage {
            MIDL_INTERFACE("01b29def-e062-48a1-8b0c-f2c7a9ca56c0")
            IUserDataPathsStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE GetForUser(
                    ABI::Windows::System::IUser* user,
                    ABI::Windows::Storage::IUserDataPaths** result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetDefault(
                    ABI::Windows::Storage::IUserDataPaths** result
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IUserDataPathsStatics = __uuidof(IUserDataPathsStatics);
        } /* Storage */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIUserDataPathsStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIUserDataPathsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Storage.AppDataPaths
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.IAppDataPathsStatics interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.IAppDataPaths ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Storage_AppDataPaths_DEFINED
#define RUNTIMECLASS_Windows_Storage_AppDataPaths_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_AppDataPaths[] = L"Windows.Storage.AppDataPaths";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Storage.ApplicationData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.IApplicationDataStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Storage.IApplicationDataStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.IApplicationData ** Default Interface **
 *    Windows.Storage.IApplicationData2
 *    Windows.Storage.IApplicationData3
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_ApplicationData_DEFINED
#define RUNTIMECLASS_Windows_Storage_ApplicationData_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_ApplicationData[] = L"Windows.Storage.ApplicationData";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.ApplicationDataCompositeValue
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IPropertySet ** Default Interface **
 *    Windows.Foundation.Collections.IObservableMap`2<String, Object>
 *    Windows.Foundation.Collections.IMap`2<String, Object>
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_ApplicationDataCompositeValue_DEFINED
#define RUNTIMECLASS_Windows_Storage_ApplicationDataCompositeValue_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_ApplicationDataCompositeValue[] = L"Windows.Storage.ApplicationDataCompositeValue";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.ApplicationDataContainer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.IApplicationDataContainer ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_ApplicationDataContainer_DEFINED
#define RUNTIMECLASS_Windows_Storage_ApplicationDataContainer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_ApplicationDataContainer[] = L"Windows.Storage.ApplicationDataContainer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.ApplicationDataContainerSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IPropertySet ** Default Interface **
 *    Windows.Foundation.Collections.IObservableMap`2<String, Object>
 *    Windows.Foundation.Collections.IMap`2<String, Object>
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_ApplicationDataContainerSettings_DEFINED
#define RUNTIMECLASS_Windows_Storage_ApplicationDataContainerSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_ApplicationDataContainerSettings[] = L"Windows.Storage.ApplicationDataContainerSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.CachedFileManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.ICachedFileManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_CachedFileManager_DEFINED
#define RUNTIMECLASS_Windows_Storage_CachedFileManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_CachedFileManager[] = L"Windows.Storage.CachedFileManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.DownloadsFolder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.IDownloadsFolderStatics2 interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Storage.IDownloadsFolderStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_DownloadsFolder_DEFINED
#define RUNTIMECLASS_Windows_Storage_DownloadsFolder_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_DownloadsFolder[] = L"Windows.Storage.DownloadsFolder";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.FileIO
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.IFileIOStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_FileIO_DEFINED
#define RUNTIMECLASS_Windows_Storage_FileIO_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_FileIO[] = L"Windows.Storage.FileIO";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.KnownFolders
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.IKnownFoldersStatics3 interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Storage.IKnownFoldersPlaylistsStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Storage.IKnownFoldersStatics4 interface starting with version 10.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Storage.IKnownFoldersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Storage.IKnownFoldersCameraRollStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Storage.IKnownFoldersSavedPicturesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Storage.IKnownFoldersStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_KnownFolders_DEFINED
#define RUNTIMECLASS_Windows_Storage_KnownFolders_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_KnownFolders[] = L"Windows.Storage.KnownFolders";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.PathIO
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.IPathIOStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_PathIO_DEFINED
#define RUNTIMECLASS_Windows_Storage_PathIO_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_PathIO[] = L"Windows.Storage.PathIO";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.SetVersionDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.ISetVersionDeferral ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_SetVersionDeferral_DEFINED
#define RUNTIMECLASS_Windows_Storage_SetVersionDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_SetVersionDeferral[] = L"Windows.Storage.SetVersionDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.SetVersionRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.ISetVersionRequest ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_SetVersionRequest_DEFINED
#define RUNTIMECLASS_Windows_Storage_SetVersionRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_SetVersionRequest[] = L"Windows.Storage.SetVersionRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.StorageFile
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.IStorageFileStatics2 interface starting with version 10.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Storage.IStorageFileStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.IStorageFile ** Default Interface **
 *    Windows.Storage.Streams.IInputStreamReference
 *    Windows.Storage.Streams.IRandomAccessStreamReference
 *    Windows.Storage.IStorageItem
 *    Windows.Storage.IStorageItemProperties
 *    Windows.Storage.IStorageItemProperties2
 *    Windows.Storage.IStorageItem2
 *    Windows.Storage.IStorageItemPropertiesWithProvider
 *    Windows.Storage.IStorageFilePropertiesWithAvailability
 *    Windows.Storage.IStorageFile2
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_StorageFile_DEFINED
#define RUNTIMECLASS_Windows_Storage_StorageFile_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_StorageFile[] = L"Windows.Storage.StorageFile";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.StorageFolder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.IStorageFolderStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Storage.IStorageFolderStatics2 interface starting with version 10.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.IStorageFolder ** Default Interface **
 *    Windows.Storage.IStorageItem
 *    Windows.Storage.Search.IStorageFolderQueryOperations
 *    Windows.Storage.IStorageItemProperties
 *    Windows.Storage.IStorageItemProperties2
 *    Windows.Storage.IStorageItem2
 *    Windows.Storage.IStorageFolder2
 *    Windows.Storage.IStorageItemPropertiesWithProvider
 *    Windows.Storage.IStorageFolder3
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_StorageFolder_DEFINED
#define RUNTIMECLASS_Windows_Storage_StorageFolder_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_StorageFolder[] = L"Windows.Storage.StorageFolder";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.StorageLibrary
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.IStorageLibraryStatics2 interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Storage.IStorageLibraryStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.IStorageLibrary ** Default Interface **
 *    Windows.Storage.IStorageLibrary2
 *    Windows.Storage.IStorageLibrary3
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_StorageLibrary_DEFINED
#define RUNTIMECLASS_Windows_Storage_StorageLibrary_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_StorageLibrary[] = L"Windows.Storage.StorageLibrary";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.StorageLibraryChange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.IStorageLibraryChange ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Storage_StorageLibraryChange_DEFINED
#define RUNTIMECLASS_Windows_Storage_StorageLibraryChange_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_StorageLibraryChange[] = L"Windows.Storage.StorageLibraryChange";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Storage.StorageLibraryChangeReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.IStorageLibraryChangeReader ** Default Interface **
 *    Windows.Storage.IStorageLibraryChangeReader2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Storage_StorageLibraryChangeReader_DEFINED
#define RUNTIMECLASS_Windows_Storage_StorageLibraryChangeReader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_StorageLibraryChangeReader[] = L"Windows.Storage.StorageLibraryChangeReader";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Storage.StorageLibraryChangeTracker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.IStorageLibraryChangeTracker ** Default Interface **
 *    Windows.Storage.IStorageLibraryChangeTracker2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Storage_StorageLibraryChangeTracker_DEFINED
#define RUNTIMECLASS_Windows_Storage_StorageLibraryChangeTracker_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_StorageLibraryChangeTracker[] = L"Windows.Storage.StorageLibraryChangeTracker";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Storage.StorageLibraryChangeTrackerOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 11.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.IStorageLibraryChangeTrackerOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#ifndef RUNTIMECLASS_Windows_Storage_StorageLibraryChangeTrackerOptions_DEFINED
#define RUNTIMECLASS_Windows_Storage_StorageLibraryChangeTrackerOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_StorageLibraryChangeTrackerOptions[] = L"Windows.Storage.StorageLibraryChangeTrackerOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

/*
 *
 * Class Windows.Storage.StorageLibraryLastChangeId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.IStorageLibraryLastChangeIdStatics interface starting with version 11.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.IStorageLibraryLastChangeId ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#ifndef RUNTIMECLASS_Windows_Storage_StorageLibraryLastChangeId_DEFINED
#define RUNTIMECLASS_Windows_Storage_StorageLibraryLastChangeId_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_StorageLibraryLastChangeId[] = L"Windows.Storage.StorageLibraryLastChangeId";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

/*
 *
 * Class Windows.Storage.StorageProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.IStorageProvider ** Default Interface **
 *    Windows.Storage.IStorageProvider2
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_StorageProvider_DEFINED
#define RUNTIMECLASS_Windows_Storage_StorageProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_StorageProvider[] = L"Windows.Storage.StorageProvider";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.StorageStreamTransaction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.IStorageStreamTransaction ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_StorageStreamTransaction_DEFINED
#define RUNTIMECLASS_Windows_Storage_StorageStreamTransaction_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_StorageStreamTransaction[] = L"Windows.Storage.StorageStreamTransaction";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.StreamedFileDataRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Streams.IOutputStream ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Storage.IStreamedFileDataRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_StreamedFileDataRequest_DEFINED
#define RUNTIMECLASS_Windows_Storage_StreamedFileDataRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_StreamedFileDataRequest[] = L"Windows.Storage.StreamedFileDataRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.SystemAudioProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.ISystemAudioProperties ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_SystemAudioProperties_DEFINED
#define RUNTIMECLASS_Windows_Storage_SystemAudioProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_SystemAudioProperties[] = L"Windows.Storage.SystemAudioProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.SystemDataPaths
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.ISystemDataPathsStatics interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.ISystemDataPaths ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Storage_SystemDataPaths_DEFINED
#define RUNTIMECLASS_Windows_Storage_SystemDataPaths_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_SystemDataPaths[] = L"Windows.Storage.SystemDataPaths";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Storage.SystemGPSProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.ISystemGPSProperties ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_SystemGPSProperties_DEFINED
#define RUNTIMECLASS_Windows_Storage_SystemGPSProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_SystemGPSProperties[] = L"Windows.Storage.SystemGPSProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.SystemImageProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.ISystemImageProperties ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_SystemImageProperties_DEFINED
#define RUNTIMECLASS_Windows_Storage_SystemImageProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_SystemImageProperties[] = L"Windows.Storage.SystemImageProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.SystemMediaProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.ISystemMediaProperties ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_SystemMediaProperties_DEFINED
#define RUNTIMECLASS_Windows_Storage_SystemMediaProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_SystemMediaProperties[] = L"Windows.Storage.SystemMediaProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.SystemMusicProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.ISystemMusicProperties ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_SystemMusicProperties_DEFINED
#define RUNTIMECLASS_Windows_Storage_SystemMusicProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_SystemMusicProperties[] = L"Windows.Storage.SystemMusicProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.SystemPhotoProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.ISystemPhotoProperties ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_SystemPhotoProperties_DEFINED
#define RUNTIMECLASS_Windows_Storage_SystemPhotoProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_SystemPhotoProperties[] = L"Windows.Storage.SystemPhotoProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.SystemProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.ISystemProperties interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_SystemProperties_DEFINED
#define RUNTIMECLASS_Windows_Storage_SystemProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_SystemProperties[] = L"Windows.Storage.SystemProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.SystemVideoProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.ISystemVideoProperties ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_SystemVideoProperties_DEFINED
#define RUNTIMECLASS_Windows_Storage_SystemVideoProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_SystemVideoProperties[] = L"Windows.Storage.SystemVideoProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.UserDataPaths
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.IUserDataPathsStatics interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.IUserDataPaths ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Storage_UserDataPaths_DEFINED
#define RUNTIMECLASS_Windows_Storage_UserDataPaths_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_UserDataPaths[] = L"Windows.Storage.UserDataPaths";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CStorage_CIApplicationDataSetVersionHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIApplicationDataSetVersionHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIApplicationDataSetVersionHandler __x_ABI_CWindows_CStorage_CIApplicationDataSetVersionHandler;

#endif // ____x_ABI_CWindows_CStorage_CIApplicationDataSetVersionHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStreamedFileDataRequestedHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStreamedFileDataRequestedHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStreamedFileDataRequestedHandler __x_ABI_CWindows_CStorage_CIStreamedFileDataRequestedHandler;

#endif // ____x_ABI_CWindows_CStorage_CIStreamedFileDataRequestedHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIAppDataPaths_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIAppDataPaths_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIAppDataPaths __x_ABI_CWindows_CStorage_CIAppDataPaths;

#endif // ____x_ABI_CWindows_CStorage_CIAppDataPaths_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIAppDataPathsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIAppDataPathsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIAppDataPathsStatics __x_ABI_CWindows_CStorage_CIAppDataPathsStatics;

#endif // ____x_ABI_CWindows_CStorage_CIAppDataPathsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIApplicationData_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIApplicationData_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIApplicationData __x_ABI_CWindows_CStorage_CIApplicationData;

#endif // ____x_ABI_CWindows_CStorage_CIApplicationData_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIApplicationData2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIApplicationData2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIApplicationData2 __x_ABI_CWindows_CStorage_CIApplicationData2;

#endif // ____x_ABI_CWindows_CStorage_CIApplicationData2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIApplicationData3_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIApplicationData3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIApplicationData3 __x_ABI_CWindows_CStorage_CIApplicationData3;

#endif // ____x_ABI_CWindows_CStorage_CIApplicationData3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIApplicationDataContainer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIApplicationDataContainer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIApplicationDataContainer __x_ABI_CWindows_CStorage_CIApplicationDataContainer;

#endif // ____x_ABI_CWindows_CStorage_CIApplicationDataContainer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIApplicationDataStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIApplicationDataStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIApplicationDataStatics __x_ABI_CWindows_CStorage_CIApplicationDataStatics;

#endif // ____x_ABI_CWindows_CStorage_CIApplicationDataStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIApplicationDataStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIApplicationDataStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIApplicationDataStatics2 __x_ABI_CWindows_CStorage_CIApplicationDataStatics2;

#endif // ____x_ABI_CWindows_CStorage_CIApplicationDataStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CICachedFileManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CICachedFileManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CICachedFileManagerStatics __x_ABI_CWindows_CStorage_CICachedFileManagerStatics;

#endif // ____x_ABI_CWindows_CStorage_CICachedFileManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIDownloadsFolderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIDownloadsFolderStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIDownloadsFolderStatics __x_ABI_CWindows_CStorage_CIDownloadsFolderStatics;

#endif // ____x_ABI_CWindows_CStorage_CIDownloadsFolderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2 __x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2;

#endif // ____x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIFileIOStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIFileIOStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIFileIOStatics __x_ABI_CWindows_CStorage_CIFileIOStatics;

#endif // ____x_ABI_CWindows_CStorage_CIFileIOStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIKnownFoldersCameraRollStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIKnownFoldersCameraRollStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIKnownFoldersCameraRollStatics __x_ABI_CWindows_CStorage_CIKnownFoldersCameraRollStatics;

#endif // ____x_ABI_CWindows_CStorage_CIKnownFoldersCameraRollStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIKnownFoldersPlaylistsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIKnownFoldersPlaylistsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIKnownFoldersPlaylistsStatics __x_ABI_CWindows_CStorage_CIKnownFoldersPlaylistsStatics;

#endif // ____x_ABI_CWindows_CStorage_CIKnownFoldersPlaylistsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIKnownFoldersSavedPicturesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIKnownFoldersSavedPicturesStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIKnownFoldersSavedPicturesStatics __x_ABI_CWindows_CStorage_CIKnownFoldersSavedPicturesStatics;

#endif // ____x_ABI_CWindows_CStorage_CIKnownFoldersSavedPicturesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIKnownFoldersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIKnownFoldersStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIKnownFoldersStatics __x_ABI_CWindows_CStorage_CIKnownFoldersStatics;

#endif // ____x_ABI_CWindows_CStorage_CIKnownFoldersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIKnownFoldersStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIKnownFoldersStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIKnownFoldersStatics2 __x_ABI_CWindows_CStorage_CIKnownFoldersStatics2;

#endif // ____x_ABI_CWindows_CStorage_CIKnownFoldersStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIKnownFoldersStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIKnownFoldersStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIKnownFoldersStatics3 __x_ABI_CWindows_CStorage_CIKnownFoldersStatics3;

#endif // ____x_ABI_CWindows_CStorage_CIKnownFoldersStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIKnownFoldersStatics4_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIKnownFoldersStatics4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIKnownFoldersStatics4 __x_ABI_CWindows_CStorage_CIKnownFoldersStatics4;

#endif // ____x_ABI_CWindows_CStorage_CIKnownFoldersStatics4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIPathIOStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIPathIOStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIPathIOStatics __x_ABI_CWindows_CStorage_CIPathIOStatics;

#endif // ____x_ABI_CWindows_CStorage_CIPathIOStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CISetVersionDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CISetVersionDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CISetVersionDeferral __x_ABI_CWindows_CStorage_CISetVersionDeferral;

#endif // ____x_ABI_CWindows_CStorage_CISetVersionDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CISetVersionRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CISetVersionRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CISetVersionRequest __x_ABI_CWindows_CStorage_CISetVersionRequest;

#endif // ____x_ABI_CWindows_CStorage_CISetVersionRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFile __x_ABI_CWindows_CStorage_CIStorageFile;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFile2 __x_ABI_CWindows_CStorage_CIStorageFile2;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability __x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFileStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFileStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFileStatics __x_ABI_CWindows_CStorage_CIStorageFileStatics;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFileStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFileStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFileStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFileStatics2 __x_ABI_CWindows_CStorage_CIStorageFileStatics2;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFileStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFolder __x_ABI_CWindows_CStorage_CIStorageFolder;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFolder2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFolder2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFolder2 __x_ABI_CWindows_CStorage_CIStorageFolder2;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFolder2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFolder3_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFolder3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFolder3 __x_ABI_CWindows_CStorage_CIStorageFolder3;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFolder3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFolderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFolderStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFolderStatics __x_ABI_CWindows_CStorage_CIStorageFolderStatics;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFolderStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFolderStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFolderStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFolderStatics2 __x_ABI_CWindows_CStorage_CIStorageFolderStatics2;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFolderStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageItem_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageItem __x_ABI_CWindows_CStorage_CIStorageItem;

#endif // ____x_ABI_CWindows_CStorage_CIStorageItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageItem2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageItem2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageItem2 __x_ABI_CWindows_CStorage_CIStorageItem2;

#endif // ____x_ABI_CWindows_CStorage_CIStorageItem2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageItemProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageItemProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageItemProperties __x_ABI_CWindows_CStorage_CIStorageItemProperties;

#endif // ____x_ABI_CWindows_CStorage_CIStorageItemProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageItemProperties2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageItemProperties2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageItemProperties2 __x_ABI_CWindows_CStorage_CIStorageItemProperties2;

#endif // ____x_ABI_CWindows_CStorage_CIStorageItemProperties2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider __x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider;

#endif // ____x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageLibrary_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageLibrary_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageLibrary __x_ABI_CWindows_CStorage_CIStorageLibrary;

#endif // ____x_ABI_CWindows_CStorage_CIStorageLibrary_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageLibrary2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageLibrary2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageLibrary2 __x_ABI_CWindows_CStorage_CIStorageLibrary2;

#endif // ____x_ABI_CWindows_CStorage_CIStorageLibrary2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageLibrary3_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageLibrary3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageLibrary3 __x_ABI_CWindows_CStorage_CIStorageLibrary3;

#endif // ____x_ABI_CWindows_CStorage_CIStorageLibrary3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageLibraryChange_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryChange_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageLibraryChange __x_ABI_CWindows_CStorage_CIStorageLibraryChange;

#endif // ____x_ABI_CWindows_CStorage_CIStorageLibraryChange_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader __x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader;

#endif // ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader2 __x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader2;

#endif // ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker;

#endif // ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker2 __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker2;

#endif // ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptions __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptions;

#endif // ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeId_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeId_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeId __x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeId;

#endif // ____x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeId_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdStatics __x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdStatics;

#endif // ____x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageLibraryStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageLibraryStatics __x_ABI_CWindows_CStorage_CIStorageLibraryStatics;

#endif // ____x_ABI_CWindows_CStorage_CIStorageLibraryStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageLibraryStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageLibraryStatics2 __x_ABI_CWindows_CStorage_CIStorageLibraryStatics2;

#endif // ____x_ABI_CWindows_CStorage_CIStorageLibraryStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageProvider __x_ABI_CWindows_CStorage_CIStorageProvider;

#endif // ____x_ABI_CWindows_CStorage_CIStorageProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageProvider2_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageProvider2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageProvider2 __x_ABI_CWindows_CStorage_CIStorageProvider2;

#endif // ____x_ABI_CWindows_CStorage_CIStorageProvider2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageStreamTransaction_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageStreamTransaction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageStreamTransaction __x_ABI_CWindows_CStorage_CIStorageStreamTransaction;

#endif // ____x_ABI_CWindows_CStorage_CIStorageStreamTransaction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStreamedFileDataRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStreamedFileDataRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStreamedFileDataRequest __x_ABI_CWindows_CStorage_CIStreamedFileDataRequest;

#endif // ____x_ABI_CWindows_CStorage_CIStreamedFileDataRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CISystemAudioProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CISystemAudioProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CISystemAudioProperties __x_ABI_CWindows_CStorage_CISystemAudioProperties;

#endif // ____x_ABI_CWindows_CStorage_CISystemAudioProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CISystemDataPaths_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CISystemDataPaths_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CISystemDataPaths __x_ABI_CWindows_CStorage_CISystemDataPaths;

#endif // ____x_ABI_CWindows_CStorage_CISystemDataPaths_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CISystemDataPathsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CISystemDataPathsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CISystemDataPathsStatics __x_ABI_CWindows_CStorage_CISystemDataPathsStatics;

#endif // ____x_ABI_CWindows_CStorage_CISystemDataPathsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CISystemGPSProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CISystemGPSProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CISystemGPSProperties __x_ABI_CWindows_CStorage_CISystemGPSProperties;

#endif // ____x_ABI_CWindows_CStorage_CISystemGPSProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CISystemImageProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CISystemImageProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CISystemImageProperties __x_ABI_CWindows_CStorage_CISystemImageProperties;

#endif // ____x_ABI_CWindows_CStorage_CISystemImageProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CISystemMediaProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CISystemMediaProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CISystemMediaProperties __x_ABI_CWindows_CStorage_CISystemMediaProperties;

#endif // ____x_ABI_CWindows_CStorage_CISystemMediaProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CISystemMusicProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CISystemMusicProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CISystemMusicProperties __x_ABI_CWindows_CStorage_CISystemMusicProperties;

#endif // ____x_ABI_CWindows_CStorage_CISystemMusicProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CISystemPhotoProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CISystemPhotoProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CISystemPhotoProperties __x_ABI_CWindows_CStorage_CISystemPhotoProperties;

#endif // ____x_ABI_CWindows_CStorage_CISystemPhotoProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CISystemProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CISystemProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CISystemProperties __x_ABI_CWindows_CStorage_CISystemProperties;

#endif // ____x_ABI_CWindows_CStorage_CISystemProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CISystemVideoProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CISystemVideoProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CISystemVideoProperties __x_ABI_CWindows_CStorage_CISystemVideoProperties;

#endif // ____x_ABI_CWindows_CStorage_CISystemVideoProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIUserDataPaths_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIUserDataPaths_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIUserDataPaths __x_ABI_CWindows_CStorage_CIUserDataPaths;

#endif // ____x_ABI_CWindows_CStorage_CIUserDataPaths_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIUserDataPathsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIUserDataPathsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIUserDataPathsStatics __x_ABI_CWindows_CStorage_CIUserDataPathsStatics;

#endif // ____x_ABI_CWindows_CStorage_CIUserDataPathsStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_boolean __FIAsyncOperationCompletedHandler_1_boolean;

#if !defined(____FIAsyncOperation_1_boolean_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_boolean_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_boolean __FIAsyncOperation_1_boolean;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_boolean;

typedef struct __FIAsyncOperation_1_booleanVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_boolean* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_boolean* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_boolean* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_boolean* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_boolean* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_boolean* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_boolean* This,
        __FIAsyncOperationCompletedHandler_1_boolean* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_boolean* This,
        __FIAsyncOperationCompletedHandler_1_boolean** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_boolean* This,
        boolean* result);

    END_INTERFACE
} __FIAsyncOperation_1_booleanVtbl;

interface __FIAsyncOperation_1_boolean
{
    CONST_VTBL struct __FIAsyncOperation_1_booleanVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_boolean_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_boolean_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_boolean_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_boolean_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_boolean_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_boolean_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_boolean_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_boolean_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_boolean_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_boolean_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1_boolean_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_boolean_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_boolean __FIAsyncOperationCompletedHandler_1_boolean;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_boolean;

typedef struct __FIAsyncOperationCompletedHandler_1_booleanVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_boolean* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_boolean* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_boolean* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_boolean* This,
        __FIAsyncOperation_1_boolean* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_booleanVtbl;

interface __FIAsyncOperationCompletedHandler_1_boolean
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_booleanVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_boolean_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_boolean_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_boolean_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_boolean_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_boolean_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_HSTRING __FIAsyncOperationCompletedHandler_1_HSTRING;

#if !defined(____FIAsyncOperation_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_HSTRING __FIAsyncOperation_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_HSTRING;

typedef struct __FIAsyncOperation_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_HSTRING* This,
        __FIAsyncOperationCompletedHandler_1_HSTRING* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_HSTRING* This,
        __FIAsyncOperationCompletedHandler_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_HSTRING* This,
        HSTRING* result);

    END_INTERFACE
} __FIAsyncOperation_1_HSTRINGVtbl;

interface __FIAsyncOperation_1_HSTRING
{
    CONST_VTBL struct __FIAsyncOperation_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_HSTRING_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_HSTRING_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_HSTRING_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_HSTRING __FIAsyncOperationCompletedHandler_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_HSTRING;

typedef struct __FIAsyncOperationCompletedHandler_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_HSTRING* This,
        __FIAsyncOperation_1_HSTRING* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_HSTRINGVtbl;

interface __FIAsyncOperationCompletedHandler_1_HSTRING
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_HSTRING_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_HSTRING_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CStorage__CIStorageItem_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CStorage__CIStorageItem_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CStorage__CIStorageItem __FIIterator_1_Windows__CStorage__CIStorageItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CStorage__CIStorageItem;

typedef struct __FIIterator_1_Windows__CStorage__CIStorageItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CStorage__CIStorageItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CStorage__CIStorageItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CStorage__CIStorageItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CStorage__CIStorageItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CStorage__CIStorageItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CStorage__CIStorageItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CStorage__CIStorageItem* This,
        __x_ABI_CWindows_CStorage_CIStorageItem** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CStorage__CIStorageItem* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CStorage__CIStorageItem* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CStorage__CIStorageItem* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CStorage_CIStorageItem** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CStorage__CIStorageItemVtbl;

interface __FIIterator_1_Windows__CStorage__CIStorageItem
{
    CONST_VTBL struct __FIIterator_1_Windows__CStorage__CIStorageItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CStorage__CIStorageItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CStorage__CIStorageItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CStorage__CIStorageItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CStorage__CIStorageItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CStorage__CIStorageItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CStorage__CIStorageItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CStorage__CIStorageItem_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CStorage__CIStorageItem_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CStorage__CIStorageItem_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CStorage__CIStorageItem_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CStorage__CIStorageItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CStorage__CIStorageItem_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CStorage__CIStorageItem_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CStorage__CIStorageItem __FIIterable_1_Windows__CStorage__CIStorageItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CStorage__CIStorageItem;

typedef struct __FIIterable_1_Windows__CStorage__CIStorageItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CStorage__CIStorageItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CStorage__CIStorageItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CStorage__CIStorageItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CStorage__CIStorageItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CStorage__CIStorageItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CStorage__CIStorageItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CStorage__CIStorageItem* This,
        __FIIterator_1_Windows__CStorage__CIStorageItem** result);

    END_INTERFACE
} __FIIterable_1_Windows__CStorage__CIStorageItemVtbl;

interface __FIIterable_1_Windows__CStorage__CIStorageItem
{
    CONST_VTBL struct __FIIterable_1_Windows__CStorage__CIStorageItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CStorage__CIStorageItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CStorage__CIStorageItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CStorage__CIStorageItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CStorage__CIStorageItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CStorage__CIStorageItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CStorage__CIStorageItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CStorage__CIStorageItem_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CStorage__CIStorageItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CStorage__CIStorageItem_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CStorage__CIStorageItem_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CStorage__CIStorageItem __FIVectorView_1_Windows__CStorage__CIStorageItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CStorage__CIStorageItem;

typedef struct __FIVectorView_1_Windows__CStorage__CIStorageItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CStorage__CIStorageItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CStorage__CIStorageItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CStorage__CIStorageItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CStorage__CIStorageItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CStorage__CIStorageItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CStorage__CIStorageItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CStorage__CIStorageItem* This,
        UINT32 index,
        __x_ABI_CWindows_CStorage_CIStorageItem** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CStorage__CIStorageItem* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CStorage__CIStorageItem* This,
        __x_ABI_CWindows_CStorage_CIStorageItem* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CStorage__CIStorageItem* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CStorage_CIStorageItem** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CStorage__CIStorageItemVtbl;

interface __FIVectorView_1_Windows__CStorage__CIStorageItem
{
    CONST_VTBL struct __FIVectorView_1_Windows__CStorage__CIStorageItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CStorage__CIStorageItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CStorage__CIStorageItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CStorage__CIStorageItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CStorage__CIStorageItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CStorage__CIStorageItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CStorage__CIStorageItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CStorage__CIStorageItem_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CStorage__CIStorageItem_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CStorage__CIStorageItem_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CStorage__CIStorageItem_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CStorage__CIStorageItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem* This,
        __FIVectorView_1_Windows__CStorage__CIStorageItem** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItemVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItemVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CIStorageItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CStorage__CStorageFile __FIIterator_1_Windows__CStorage__CStorageFile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CStorage__CStorageFile;

typedef struct __FIIterator_1_Windows__CStorage__CStorageFileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CStorage__CStorageFile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CStorage__CStorageFile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CStorage__CStorageFile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CStorage__CStorageFile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CStorage__CStorageFile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CStorage__CStorageFile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CStorage__CStorageFile* This,
        __x_ABI_CWindows_CStorage_CIStorageFile** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CStorage__CStorageFile* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CStorage__CStorageFile* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CStorage__CStorageFile* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CStorage_CIStorageFile** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CStorage__CStorageFileVtbl;

interface __FIIterator_1_Windows__CStorage__CStorageFile
{
    CONST_VTBL struct __FIIterator_1_Windows__CStorage__CStorageFileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CStorage__CStorageFile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CStorage__CStorageFile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CStorage__CStorageFile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CStorage__CStorageFile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CStorage__CStorageFile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CStorage__CStorageFile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CStorage__CStorageFile_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CStorage__CStorageFile_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CStorage__CStorageFile_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CStorage__CStorageFile_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CStorage__CStorageFile __FIIterable_1_Windows__CStorage__CStorageFile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CStorage__CStorageFile;

typedef struct __FIIterable_1_Windows__CStorage__CStorageFileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CStorage__CStorageFile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CStorage__CStorageFile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CStorage__CStorageFile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CStorage__CStorageFile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CStorage__CStorageFile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CStorage__CStorageFile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CStorage__CStorageFile* This,
        __FIIterator_1_Windows__CStorage__CStorageFile** result);

    END_INTERFACE
} __FIIterable_1_Windows__CStorage__CStorageFileVtbl;

interface __FIIterable_1_Windows__CStorage__CStorageFile
{
    CONST_VTBL struct __FIIterable_1_Windows__CStorage__CStorageFileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CStorage__CStorageFile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CStorage__CStorageFile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CStorage__CStorageFile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CStorage__CStorageFile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CStorage__CStorageFile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CStorage__CStorageFile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CStorage__CStorageFile_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CStorage__CStorageFile __FIVectorView_1_Windows__CStorage__CStorageFile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CStorage__CStorageFile;

typedef struct __FIVectorView_1_Windows__CStorage__CStorageFileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CStorage__CStorageFile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CStorage__CStorageFile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CStorage__CStorageFile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CStorage__CStorageFile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CStorage__CStorageFile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CStorage__CStorageFile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CStorage__CStorageFile* This,
        UINT32 index,
        __x_ABI_CWindows_CStorage_CIStorageFile** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CStorage__CStorageFile* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CStorage__CStorageFile* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CStorage__CStorageFile* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CStorage_CIStorageFile** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CStorage__CStorageFileVtbl;

interface __FIVectorView_1_Windows__CStorage__CStorageFile
{
    CONST_VTBL struct __FIVectorView_1_Windows__CStorage__CStorageFileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CStorage__CStorageFile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CStorage__CStorageFile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CStorage__CStorageFile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CStorage__CStorageFile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CStorage__CStorageFile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CStorage__CStorageFile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CStorage__CStorageFile_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CStorage__CStorageFile_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CStorage__CStorageFile_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CStorage__CStorageFile_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile* This,
        __FIVectorView_1_Windows__CStorage__CStorageFile** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFileVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFileVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CStorage__CStorageFolder __FIIterator_1_Windows__CStorage__CStorageFolder;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CStorage__CStorageFolder;

typedef struct __FIIterator_1_Windows__CStorage__CStorageFolderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CStorage__CStorageFolder* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CStorage__CStorageFolder* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CStorage__CStorageFolder* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CStorage__CStorageFolder* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CStorage__CStorageFolder* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CStorage__CStorageFolder* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CStorage__CStorageFolder* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CStorage__CStorageFolder* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CStorage__CStorageFolder* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CStorage__CStorageFolder* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CStorage_CIStorageFolder** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CStorage__CStorageFolderVtbl;

interface __FIIterator_1_Windows__CStorage__CStorageFolder
{
    CONST_VTBL struct __FIIterator_1_Windows__CStorage__CStorageFolderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CStorage__CStorageFolder_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CStorage__CStorageFolder_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CStorage__CStorageFolder_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CStorage__CStorageFolder_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CStorage__CStorageFolder_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CStorage__CStorageFolder_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CStorage__CStorageFolder_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CStorage__CStorageFolder_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CStorage__CStorageFolder_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CStorage__CStorageFolder_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CStorage__CStorageFolder __FIIterable_1_Windows__CStorage__CStorageFolder;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CStorage__CStorageFolder;

typedef struct __FIIterable_1_Windows__CStorage__CStorageFolderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CStorage__CStorageFolder* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CStorage__CStorageFolder* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CStorage__CStorageFolder* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CStorage__CStorageFolder* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CStorage__CStorageFolder* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CStorage__CStorageFolder* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CStorage__CStorageFolder* This,
        __FIIterator_1_Windows__CStorage__CStorageFolder** result);

    END_INTERFACE
} __FIIterable_1_Windows__CStorage__CStorageFolderVtbl;

interface __FIIterable_1_Windows__CStorage__CStorageFolder
{
    CONST_VTBL struct __FIIterable_1_Windows__CStorage__CStorageFolderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CStorage__CStorageFolder_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CStorage__CStorageFolder_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CStorage__CStorageFolder_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CStorage__CStorageFolder_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CStorage__CStorageFolder_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CStorage__CStorageFolder_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CStorage__CStorageFolder_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CStorage__CStorageFolder __FIVectorView_1_Windows__CStorage__CStorageFolder;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CStorage__CStorageFolder;

typedef struct __FIVectorView_1_Windows__CStorage__CStorageFolderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CStorage__CStorageFolder* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CStorage__CStorageFolder* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CStorage__CStorageFolder* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CStorage__CStorageFolder* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CStorage__CStorageFolder* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CStorage__CStorageFolder* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CStorage__CStorageFolder* This,
        UINT32 index,
        __x_ABI_CWindows_CStorage_CIStorageFolder** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CStorage__CStorageFolder* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CStorage__CStorageFolder* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CStorage__CStorageFolder* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CStorage_CIStorageFolder** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CStorage__CStorageFolderVtbl;

interface __FIVectorView_1_Windows__CStorage__CStorageFolder
{
    CONST_VTBL struct __FIVectorView_1_Windows__CStorage__CStorageFolderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CStorage__CStorageFolder_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CStorage__CStorageFolder_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CStorage__CStorageFolder_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CStorage__CStorageFolder_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CStorage__CStorageFolder_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CStorage__CStorageFolder_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CStorage__CStorageFolder_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CStorage__CStorageFolder_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CStorage__CStorageFolder_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CStorage__CStorageFolder_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFolder __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFolder;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFolder __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFolder;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFolder;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFolderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFolder* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFolder* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFolder* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFolder* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFolder* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFolder* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFolder* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFolder* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFolder* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFolder** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFolder* This,
        __FIVectorView_1_Windows__CStorage__CStorageFolder** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFolderVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFolder
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFolderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFolder_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFolder_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFolder_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFolder_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFolder_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFolder_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFolder_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFolder_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFolder_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFolder __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFolder;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFolder;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFolderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFolder* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFolder* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFolder* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFolder* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFolder* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFolderVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFolder
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFolderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFolder_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFolder_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFolder_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFolder_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterator_1_Windows__CStorage__CStorageLibraryChange_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CStorage__CStorageLibraryChange_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CStorage__CStorageLibraryChange __FIIterator_1_Windows__CStorage__CStorageLibraryChange;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CStorage__CStorageLibraryChange;

typedef struct __FIIterator_1_Windows__CStorage__CStorageLibraryChangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CStorage__CStorageLibraryChange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CStorage__CStorageLibraryChange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CStorage__CStorageLibraryChange* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CStorage__CStorageLibraryChange* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CStorage__CStorageLibraryChange* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CStorage__CStorageLibraryChange* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CStorage__CStorageLibraryChange* This,
        __x_ABI_CWindows_CStorage_CIStorageLibraryChange** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CStorage__CStorageLibraryChange* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CStorage__CStorageLibraryChange* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CStorage__CStorageLibraryChange* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CStorage_CIStorageLibraryChange** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CStorage__CStorageLibraryChangeVtbl;

interface __FIIterator_1_Windows__CStorage__CStorageLibraryChange
{
    CONST_VTBL struct __FIIterator_1_Windows__CStorage__CStorageLibraryChangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CStorage__CStorageLibraryChange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CStorage__CStorageLibraryChange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CStorage__CStorageLibraryChange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CStorage__CStorageLibraryChange_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CStorage__CStorageLibraryChange_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CStorage__CStorageLibraryChange_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CStorage__CStorageLibraryChange_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CStorage__CStorageLibraryChange_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CStorage__CStorageLibraryChange_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CStorage__CStorageLibraryChange_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CStorage__CStorageLibraryChange_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterable_1_Windows__CStorage__CStorageLibraryChange_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CStorage__CStorageLibraryChange_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CStorage__CStorageLibraryChange __FIIterable_1_Windows__CStorage__CStorageLibraryChange;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CStorage__CStorageLibraryChange;

typedef struct __FIIterable_1_Windows__CStorage__CStorageLibraryChangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CStorage__CStorageLibraryChange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CStorage__CStorageLibraryChange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CStorage__CStorageLibraryChange* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CStorage__CStorageLibraryChange* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CStorage__CStorageLibraryChange* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CStorage__CStorageLibraryChange* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CStorage__CStorageLibraryChange* This,
        __FIIterator_1_Windows__CStorage__CStorageLibraryChange** result);

    END_INTERFACE
} __FIIterable_1_Windows__CStorage__CStorageLibraryChangeVtbl;

interface __FIIterable_1_Windows__CStorage__CStorageLibraryChange
{
    CONST_VTBL struct __FIIterable_1_Windows__CStorage__CStorageLibraryChangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CStorage__CStorageLibraryChange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CStorage__CStorageLibraryChange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CStorage__CStorageLibraryChange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CStorage__CStorageLibraryChange_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CStorage__CStorageLibraryChange_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CStorage__CStorageLibraryChange_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CStorage__CStorageLibraryChange_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CStorage__CStorageLibraryChange_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIVectorView_1_Windows__CStorage__CStorageLibraryChange_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CStorage__CStorageLibraryChange_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CStorage__CStorageLibraryChange __FIVectorView_1_Windows__CStorage__CStorageLibraryChange;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CStorage__CStorageLibraryChange;

typedef struct __FIVectorView_1_Windows__CStorage__CStorageLibraryChangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CStorage__CStorageLibraryChange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CStorage__CStorageLibraryChange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CStorage__CStorageLibraryChange* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CStorage__CStorageLibraryChange* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CStorage__CStorageLibraryChange* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CStorage__CStorageLibraryChange* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CStorage__CStorageLibraryChange* This,
        UINT32 index,
        __x_ABI_CWindows_CStorage_CIStorageLibraryChange** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CStorage__CStorageLibraryChange* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CStorage__CStorageLibraryChange* This,
        __x_ABI_CWindows_CStorage_CIStorageLibraryChange* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CStorage__CStorageLibraryChange* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CStorage_CIStorageLibraryChange** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CStorage__CStorageLibraryChangeVtbl;

interface __FIVectorView_1_Windows__CStorage__CStorageLibraryChange
{
    CONST_VTBL struct __FIVectorView_1_Windows__CStorage__CStorageLibraryChangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CStorage__CStorageLibraryChange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CStorage__CStorageLibraryChange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CStorage__CStorageLibraryChange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CStorage__CStorageLibraryChange_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CStorage__CStorageLibraryChange_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CStorage__CStorageLibraryChange_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CStorage__CStorageLibraryChange_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CStorage__CStorageLibraryChange_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CStorage__CStorageLibraryChange_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CStorage__CStorageLibraryChange_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CStorage__CStorageLibraryChange_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange* This,
        __FIVectorView_1_Windows__CStorage__CStorageLibraryChange** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChangeVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChangeVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

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

#if !defined(____FIVector_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIVector_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIVector_1_HSTRING __FIVector_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_HSTRING;

typedef struct __FIVector_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_HSTRING* This,
        UINT32 index,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_HSTRING* This,
        __FIVectorView_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_HSTRING* This,
        HSTRING value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_HSTRING* This,
        UINT32 index,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_HSTRING* This,
        UINT32 index,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_HSTRING* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_HSTRING* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_HSTRING* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        HSTRING* items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_HSTRING* This,
        UINT32 itemsLength,
        HSTRING* items);

    END_INTERFACE
} __FIVector_1_HSTRINGVtbl;

interface __FIVector_1_HSTRING
{
    CONST_VTBL struct __FIVector_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_HSTRING_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_HSTRING_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_HSTRING_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_HSTRING_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_HSTRING_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_HSTRING_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_HSTRING_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_HSTRING_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_HSTRING_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_HSTRING_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_HSTRING_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING __FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING;

#if !defined(____FIAsyncOperation_1___FIVector_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVector_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVector_1_HSTRING __FIAsyncOperation_1___FIVector_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVector_1_HSTRING;

typedef struct __FIAsyncOperation_1___FIVector_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVector_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVector_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVector_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVector_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVector_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVector_1_HSTRING* This,
        __FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVector_1_HSTRING* This,
        __FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVector_1_HSTRING* This,
        __FIVector_1_HSTRING** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVector_1_HSTRINGVtbl;

interface __FIAsyncOperation_1___FIVector_1_HSTRING
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVector_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVector_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVector_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVector_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVector_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVector_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVector_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVector_1_HSTRING_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVector_1_HSTRING_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVector_1_HSTRING_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVector_1_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING __FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING* This,
        __FIAsyncOperation_1___FIVector_1_HSTRING* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRINGVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CApplicationData __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CApplicationData;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CApplicationData_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CApplicationData_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CApplicationData __FIAsyncOperation_1_Windows__CStorage__CApplicationData;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CApplicationData;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CApplicationDataVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CApplicationData* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CApplicationData* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CApplicationData* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CApplicationData* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CApplicationData* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CApplicationData* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CApplicationData* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CApplicationData* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CApplicationData* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CApplicationData** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CApplicationData* This,
        __x_ABI_CWindows_CStorage_CIApplicationData** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CApplicationDataVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CApplicationData
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CApplicationDataVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CApplicationData_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CApplicationData_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CApplicationData_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CApplicationData_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CApplicationData_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CApplicationData_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CApplicationData_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CApplicationData_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CApplicationData_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CApplicationData_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CApplicationData_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CApplicationData_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CApplicationData __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CApplicationData;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CApplicationData;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CApplicationDataVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CApplicationData* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CApplicationData* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CApplicationData* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CApplicationData* This,
        __FIAsyncOperation_1_Windows__CStorage__CApplicationData* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CApplicationDataVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CApplicationData
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CApplicationDataVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CApplicationData_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CApplicationData_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CApplicationData_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CApplicationData_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CApplicationData_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties __x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties;

#endif // ____x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CBasicProperties __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CBasicProperties;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CFileProperties__CBasicProperties_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CFileProperties__CBasicProperties_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CBasicProperties __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CBasicProperties;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CFileProperties__CBasicProperties;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CBasicPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CBasicProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CBasicProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CBasicProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CBasicProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CBasicProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CBasicProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CBasicProperties* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CBasicProperties* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CBasicProperties* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CBasicProperties** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CBasicProperties* This,
        __x_ABI_CWindows_CStorage_CFileProperties_CIBasicProperties** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CBasicPropertiesVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CBasicProperties
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CBasicPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CBasicProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CBasicProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CBasicProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CBasicProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CBasicProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CBasicProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CBasicProperties_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CBasicProperties_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CBasicProperties_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CFileProperties__CBasicProperties_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CBasicProperties_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CBasicProperties_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CBasicProperties __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CBasicProperties;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CBasicProperties;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CBasicPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CBasicProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CBasicProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CBasicProperties* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CBasicProperties* This,
        __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CBasicProperties* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CBasicPropertiesVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CBasicProperties
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CBasicPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CBasicProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CBasicProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CBasicProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CBasicProperties_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CBasicProperties_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnailVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnailVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnailVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CStorageItemThumbnailVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail* This,
        __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CStorageItemThumbnailVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CStorageItemThumbnailVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CIStorageItem __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CIStorageItem;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CIStorageItem_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CIStorageItem_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CIStorageItem __FIAsyncOperation_1_Windows__CStorage__CIStorageItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CIStorageItem;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CIStorageItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CIStorageItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CIStorageItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CIStorageItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CIStorageItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CIStorageItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CIStorageItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CIStorageItem* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CIStorageItem* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CIStorageItem* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CIStorageItem** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CIStorageItem* This,
        __x_ABI_CWindows_CStorage_CIStorageItem** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CIStorageItemVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CIStorageItem
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CIStorageItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CIStorageItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CIStorageItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CIStorageItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CIStorageItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CIStorageItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CIStorageItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CIStorageItem_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CIStorageItem_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CIStorageItem_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CIStorageItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CIStorageItem_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CIStorageItem_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CIStorageItem __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CIStorageItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CIStorageItem;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CIStorageItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CIStorageItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CIStorageItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CIStorageItem* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CIStorageItem* This,
        __FIAsyncOperation_1_Windows__CStorage__CIStorageItem* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CIStorageItemVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CIStorageItem
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CIStorageItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CIStorageItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CIStorageItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CIStorageItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CIStorageItem_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CIStorageItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CStorage_CKnownFoldersAccessStatus __x_ABI_CWindows_CStorage_CKnownFoldersAccessStatus;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CKnownFoldersAccessStatus __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CKnownFoldersAccessStatus;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatus __FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatus;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CKnownFoldersAccessStatus* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CKnownFoldersAccessStatus** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatus* This,
        enum __x_ABI_CWindows_CStorage_CKnownFoldersAccessStatus* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatusVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatus
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatus_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatus_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatus_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CKnownFoldersAccessStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CKnownFoldersAccessStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CKnownFoldersAccessStatus __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CKnownFoldersAccessStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CKnownFoldersAccessStatus;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CKnownFoldersAccessStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CKnownFoldersAccessStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CKnownFoldersAccessStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CKnownFoldersAccessStatus* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CKnownFoldersAccessStatus* This,
        __FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatus* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CKnownFoldersAccessStatusVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CKnownFoldersAccessStatus
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CKnownFoldersAccessStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CKnownFoldersAccessStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CKnownFoldersAccessStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CKnownFoldersAccessStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CKnownFoldersAccessStatus_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CKnownFoldersAccessStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

typedef enum __x_ABI_CWindows_CStorage_CProvider_CFileUpdateStatus __x_ABI_CWindows_CStorage_CProvider_CFileUpdateStatus;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CProvider__CFileUpdateStatus __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CProvider__CFileUpdateStatus;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CProvider__CFileUpdateStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CProvider__CFileUpdateStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CProvider__CFileUpdateStatus __FIAsyncOperation_1_Windows__CStorage__CProvider__CFileUpdateStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CProvider__CFileUpdateStatus;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CProvider__CFileUpdateStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CProvider__CFileUpdateStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CProvider__CFileUpdateStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CProvider__CFileUpdateStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CProvider__CFileUpdateStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CProvider__CFileUpdateStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CProvider__CFileUpdateStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CProvider__CFileUpdateStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CProvider__CFileUpdateStatus* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CProvider__CFileUpdateStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CProvider__CFileUpdateStatus** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CProvider__CFileUpdateStatus* This,
        enum __x_ABI_CWindows_CStorage_CProvider_CFileUpdateStatus* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CProvider__CFileUpdateStatusVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CProvider__CFileUpdateStatus
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CProvider__CFileUpdateStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CProvider__CFileUpdateStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CProvider__CFileUpdateStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CProvider__CFileUpdateStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CProvider__CFileUpdateStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CProvider__CFileUpdateStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CProvider__CFileUpdateStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CProvider__CFileUpdateStatus_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CProvider__CFileUpdateStatus_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CProvider__CFileUpdateStatus_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CProvider__CFileUpdateStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CProvider__CFileUpdateStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CProvider__CFileUpdateStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CProvider__CFileUpdateStatus __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CProvider__CFileUpdateStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CProvider__CFileUpdateStatus;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CProvider__CFileUpdateStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CProvider__CFileUpdateStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CProvider__CFileUpdateStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CProvider__CFileUpdateStatus* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CProvider__CFileUpdateStatus* This,
        __FIAsyncOperation_1_Windows__CStorage__CProvider__CFileUpdateStatus* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CProvider__CFileUpdateStatusVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CProvider__CFileUpdateStatus
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CProvider__CFileUpdateStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CProvider__CFileUpdateStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CProvider__CFileUpdateStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CProvider__CFileUpdateStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CProvider__CFileUpdateStatus_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CProvider__CFileUpdateStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CStorageFile __FIAsyncOperation_1_Windows__CStorage__CStorageFile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CStorageFile;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CStorageFileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CStorageFile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CStorageFile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CStorageFile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CStorageFile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CStorageFile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CStorageFile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStorageFile* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStorageFile* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CStorageFile* This,
        __x_ABI_CWindows_CStorage_CIStorageFile** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CStorageFileVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CStorageFile
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CStorageFileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFile_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFile_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFile_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile* This,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFile* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFileVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CStorageFolder __FIAsyncOperation_1_Windows__CStorage__CStorageFolder;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CStorageFolder;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CStorageFolderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CStorageFolder* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CStorageFolder* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CStorageFolder* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CStorageFolder* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CStorageFolder* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CStorageFolder* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStorageFolder* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStorageFolder* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CStorageFolder* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CStorageFolderVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CStorageFolder
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CStorageFolderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFolder_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFolder_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFolder_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFolder_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFolder_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFolder_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFolder_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFolder_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageFolder_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder* This,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFolder* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolderVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageLibrary __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageLibrary;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CStorageLibrary_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CStorageLibrary_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CStorageLibrary __FIAsyncOperation_1_Windows__CStorage__CStorageLibrary;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CStorageLibrary;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CStorageLibraryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CStorageLibrary* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CStorageLibrary* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CStorageLibrary* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CStorageLibrary* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CStorageLibrary* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CStorageLibrary* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStorageLibrary* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageLibrary* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStorageLibrary* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageLibrary** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CStorageLibrary* This,
        __x_ABI_CWindows_CStorage_CIStorageLibrary** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CStorageLibraryVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CStorageLibrary
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CStorageLibraryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CStorageLibrary_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageLibrary_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageLibrary_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageLibrary_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageLibrary_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageLibrary_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageLibrary_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageLibrary_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageLibrary_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CStorageLibrary_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageLibrary_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageLibrary_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageLibrary __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageLibrary;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageLibrary;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageLibraryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageLibrary* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageLibrary* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageLibrary* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageLibrary* This,
        __FIAsyncOperation_1_Windows__CStorage__CStorageLibrary* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageLibraryVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageLibrary
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageLibraryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageLibrary_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageLibrary_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageLibrary_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageLibrary_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageLibrary_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransactionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction* This,
        __x_ABI_CWindows_CStorage_CIStorageStreamTransaction** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransactionVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransactionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransactionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction* This,
        __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransactionVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransactionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStorageStreamTransaction_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_IInspectable __FIKeyValuePair_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_IInspectable;

typedef struct __FIKeyValuePair_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        IInspectable** result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_IInspectableVtbl;

interface __FIKeyValuePair_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_IInspectable_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_IInspectable_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

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

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        __FIKeyValuePair_2_HSTRING_IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_IInspectable** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer __FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer;

typedef struct __FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer* This,
        __x_ABI_CWindows_CStorage_CIApplicationDataContainer** result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainerVtbl;

interface __FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer* This,
        __FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainerVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainerVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CApplicationDataContainer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIMapChangedEventArgs_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIMapChangedEventArgs_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIMapChangedEventArgs_1_HSTRING __FIMapChangedEventArgs_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapChangedEventArgs_1_HSTRING;

typedef struct __FIMapChangedEventArgs_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapChangedEventArgs_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapChangedEventArgs_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapChangedEventArgs_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapChangedEventArgs_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapChangedEventArgs_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapChangedEventArgs_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CollectionChange)(__FIMapChangedEventArgs_1_HSTRING* This,
        enum __x_ABI_CWindows_CFoundation_CCollections_CCollectionChange* result);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIMapChangedEventArgs_1_HSTRING* This,
        HSTRING* result);

    END_INTERFACE
} __FIMapChangedEventArgs_1_HSTRINGVtbl;

interface __FIMapChangedEventArgs_1_HSTRING
{
    CONST_VTBL struct __FIMapChangedEventArgs_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapChangedEventArgs_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapChangedEventArgs_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapChangedEventArgs_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapChangedEventArgs_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapChangedEventArgs_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapChangedEventArgs_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapChangedEventArgs_1_HSTRING_get_CollectionChange(This, result) \
    ((This)->lpVtbl->get_CollectionChange(This, result))

#define __FIMapChangedEventArgs_1_HSTRING_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#endif /* COBJMACROS */

#endif // ____FIMapChangedEventArgs_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_IInspectable __FIMapView_2_HSTRING_IInspectable;

#if !defined(____FIMapView_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_IInspectable __FIMapView_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_IInspectable;

typedef struct __FIMapView_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_IInspectable* This,
        HSTRING key,
        IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_IInspectable* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_IInspectable* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_IInspectable* This,
        __FIMapView_2_HSTRING_IInspectable** first,
        __FIMapView_2_HSTRING_IInspectable** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_IInspectableVtbl;

interface __FIMapView_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIMapView_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_IInspectable_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_IInspectable_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_IInspectable_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_IInspectable_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer __FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer __FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer;

typedef struct __FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer* This,
        HSTRING key,
        __x_ABI_CWindows_CStorage_CIApplicationDataContainer** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer* This,
        __FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer** first,
        __FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainerVtbl;

interface __FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer
{
    CONST_VTBL struct __FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIMap_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIMap_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIMap_2_HSTRING_IInspectable __FIMap_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMap_2_HSTRING_IInspectable;

typedef struct __FIMap_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMap_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMap_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMap_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMap_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMap_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMap_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMap_2_HSTRING_IInspectable* This,
        HSTRING key,
        IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMap_2_HSTRING_IInspectable* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMap_2_HSTRING_IInspectable* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIMap_2_HSTRING_IInspectable* This,
        __FIMapView_2_HSTRING_IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* Insert)(__FIMap_2_HSTRING_IInspectable* This,
        HSTRING key,
        IInspectable* value,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Remove)(__FIMap_2_HSTRING_IInspectable* This,
        HSTRING key);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIMap_2_HSTRING_IInspectable* This);

    END_INTERFACE
} __FIMap_2_HSTRING_IInspectableVtbl;

interface __FIMap_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIMap_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMap_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMap_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMap_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMap_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMap_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMap_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMap_2_HSTRING_IInspectable_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMap_2_HSTRING_IInspectable_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMap_2_HSTRING_IInspectable_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMap_2_HSTRING_IInspectable_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIMap_2_HSTRING_IInspectable_Insert(This, key, value, result) \
    ((This)->lpVtbl->Insert(This, key, value, result))

#define __FIMap_2_HSTRING_IInspectable_Remove(This, key) \
    ((This)->lpVtbl->Remove(This, key))

#define __FIMap_2_HSTRING_IInspectable_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#endif /* COBJMACROS */

#endif // ____FIMap_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIObservableMap_2_HSTRING_IInspectable __FIObservableMap_2_HSTRING_IInspectable;

#if !defined(____FMapChangedEventHandler_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FMapChangedEventHandler_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FMapChangedEventHandler_2_HSTRING_IInspectable __FMapChangedEventHandler_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FMapChangedEventHandler_2_HSTRING_IInspectable;

typedef struct __FMapChangedEventHandler_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FMapChangedEventHandler_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FMapChangedEventHandler_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FMapChangedEventHandler_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FMapChangedEventHandler_2_HSTRING_IInspectable* This,
        __FIObservableMap_2_HSTRING_IInspectable* sender,
        __FIMapChangedEventArgs_1_HSTRING* event);

    END_INTERFACE
} __FMapChangedEventHandler_2_HSTRING_IInspectableVtbl;

interface __FMapChangedEventHandler_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FMapChangedEventHandler_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FMapChangedEventHandler_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FMapChangedEventHandler_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FMapChangedEventHandler_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FMapChangedEventHandler_2_HSTRING_IInspectable_Invoke(This, sender, event) \
    ((This)->lpVtbl->Invoke(This, sender, event))

#endif /* COBJMACROS */

#endif // ____FMapChangedEventHandler_2_HSTRING_IInspectable_INTERFACE_DEFINED__

#if !defined(____FIObservableMap_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIObservableMap_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIObservableMap_2_HSTRING_IInspectable __FIObservableMap_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIObservableMap_2_HSTRING_IInspectable;

typedef struct __FIObservableMap_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIObservableMap_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIObservableMap_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIObservableMap_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIObservableMap_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIObservableMap_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIObservableMap_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_MapChanged)(__FIObservableMap_2_HSTRING_IInspectable* This,
        __FMapChangedEventHandler_2_HSTRING_IInspectable* vhnd,
        EventRegistrationToken* result);
    HRESULT (STDMETHODCALLTYPE* remove_MapChanged)(__FIObservableMap_2_HSTRING_IInspectable* This,
        EventRegistrationToken token);

    END_INTERFACE
} __FIObservableMap_2_HSTRING_IInspectableVtbl;

interface __FIObservableMap_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIObservableMap_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIObservableMap_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIObservableMap_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIObservableMap_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIObservableMap_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIObservableMap_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIObservableMap_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIObservableMap_2_HSTRING_IInspectable_add_MapChanged(This, vhnd, result) \
    ((This)->lpVtbl->add_MapChanged(This, vhnd, result))

#define __FIObservableMap_2_HSTRING_IInspectable_remove_MapChanged(This, token) \
    ((This)->lpVtbl->remove_MapChanged(This, token))

#endif /* COBJMACROS */

#endif // ____FIObservableMap_2_HSTRING_IInspectable_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CStorage__CStorageFolder __FIVector_1_Windows__CStorage__CStorageFolder;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CStorage__CStorageFolder;

typedef struct __FIVector_1_Windows__CStorage__CStorageFolderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CStorage__CStorageFolder* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CStorage__CStorageFolder* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CStorage__CStorageFolder* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CStorage__CStorageFolder* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CStorage__CStorageFolder* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CStorage__CStorageFolder* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CStorage__CStorageFolder* This,
        UINT32 index,
        __x_ABI_CWindows_CStorage_CIStorageFolder** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CStorage__CStorageFolder* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CStorage__CStorageFolder* This,
        __FIVectorView_1_Windows__CStorage__CStorageFolder** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CStorage__CStorageFolder* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CStorage__CStorageFolder* This,
        UINT32 index,
        __x_ABI_CWindows_CStorage_CIStorageFolder* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CStorage__CStorageFolder* This,
        UINT32 index,
        __x_ABI_CWindows_CStorage_CIStorageFolder* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CStorage__CStorageFolder* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CStorage__CStorageFolder* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CStorage__CStorageFolder* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CStorage__CStorageFolder* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CStorage__CStorageFolder* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CStorage_CIStorageFolder** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CStorage__CStorageFolder* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CStorage_CIStorageFolder** items);

    END_INTERFACE
} __FIVector_1_Windows__CStorage__CStorageFolderVtbl;

interface __FIVector_1_Windows__CStorage__CStorageFolder
{
    CONST_VTBL struct __FIVector_1_Windows__CStorage__CStorageFolderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CStorage__CStorageFolder_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CStorage__CStorageFolder_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CStorage__CStorageFolder_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CStorage__CStorageFolder_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CStorage__CStorageFolder_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CStorage__CStorageFolder_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CStorage__CStorageFolder_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CStorage__CStorageFolder_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CStorage__CStorageFolder_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CStorage__CStorageFolder_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CStorage__CStorageFolder_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CStorage__CStorageFolder_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CStorage__CStorageFolder_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CStorage__CStorageFolder_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CStorage__CStorageFolder_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CStorage__CStorageFolder_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CStorage__CStorageFolder_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CStorage__CStorageFolder_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIObservableVector_1_Windows__CStorage__CStorageFolder __FIObservableVector_1_Windows__CStorage__CStorageFolder;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FVectorChangedEventHandler_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__)
#define ____FVectorChangedEventHandler_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__

typedef interface __FVectorChangedEventHandler_1_Windows__CStorage__CStorageFolder __FVectorChangedEventHandler_1_Windows__CStorage__CStorageFolder;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FVectorChangedEventHandler_1_Windows__CStorage__CStorageFolder;

typedef struct __FVectorChangedEventHandler_1_Windows__CStorage__CStorageFolderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FVectorChangedEventHandler_1_Windows__CStorage__CStorageFolder* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FVectorChangedEventHandler_1_Windows__CStorage__CStorageFolder* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FVectorChangedEventHandler_1_Windows__CStorage__CStorageFolder* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FVectorChangedEventHandler_1_Windows__CStorage__CStorageFolder* This,
        __FIObservableVector_1_Windows__CStorage__CStorageFolder* sender,
        __x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs* event);

    END_INTERFACE
} __FVectorChangedEventHandler_1_Windows__CStorage__CStorageFolderVtbl;

interface __FVectorChangedEventHandler_1_Windows__CStorage__CStorageFolder
{
    CONST_VTBL struct __FVectorChangedEventHandler_1_Windows__CStorage__CStorageFolderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FVectorChangedEventHandler_1_Windows__CStorage__CStorageFolder_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FVectorChangedEventHandler_1_Windows__CStorage__CStorageFolder_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FVectorChangedEventHandler_1_Windows__CStorage__CStorageFolder_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FVectorChangedEventHandler_1_Windows__CStorage__CStorageFolder_Invoke(This, sender, event) \
    ((This)->lpVtbl->Invoke(This, sender, event))

#endif /* COBJMACROS */

#endif // ____FVectorChangedEventHandler_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIObservableVector_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__)
#define ____FIObservableVector_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__

typedef interface __FIObservableVector_1_Windows__CStorage__CStorageFolder __FIObservableVector_1_Windows__CStorage__CStorageFolder;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIObservableVector_1_Windows__CStorage__CStorageFolder;

typedef struct __FIObservableVector_1_Windows__CStorage__CStorageFolderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIObservableVector_1_Windows__CStorage__CStorageFolder* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIObservableVector_1_Windows__CStorage__CStorageFolder* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIObservableVector_1_Windows__CStorage__CStorageFolder* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIObservableVector_1_Windows__CStorage__CStorageFolder* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIObservableVector_1_Windows__CStorage__CStorageFolder* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIObservableVector_1_Windows__CStorage__CStorageFolder* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_VectorChanged)(__FIObservableVector_1_Windows__CStorage__CStorageFolder* This,
        __FVectorChangedEventHandler_1_Windows__CStorage__CStorageFolder* vhnd,
        EventRegistrationToken* result);
    HRESULT (STDMETHODCALLTYPE* remove_VectorChanged)(__FIObservableVector_1_Windows__CStorage__CStorageFolder* This,
        EventRegistrationToken token);

    END_INTERFACE
} __FIObservableVector_1_Windows__CStorage__CStorageFolderVtbl;

interface __FIObservableVector_1_Windows__CStorage__CStorageFolder
{
    CONST_VTBL struct __FIObservableVector_1_Windows__CStorage__CStorageFolderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIObservableVector_1_Windows__CStorage__CStorageFolder_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIObservableVector_1_Windows__CStorage__CStorageFolder_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIObservableVector_1_Windows__CStorage__CStorageFolder_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIObservableVector_1_Windows__CStorage__CStorageFolder_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIObservableVector_1_Windows__CStorage__CStorageFolder_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIObservableVector_1_Windows__CStorage__CStorageFolder_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIObservableVector_1_Windows__CStorage__CStorageFolder_add_VectorChanged(This, vhnd, result) \
    ((This)->lpVtbl->add_VectorChanged(This, vhnd, result))

#define __FIObservableVector_1_Windows__CStorage__CStorageFolder_remove_VectorChanged(This, token) \
    ((This)->lpVtbl->remove_VectorChanged(This, token))

#endif /* COBJMACROS */

#endif // ____FIObservableVector_1_Windows__CStorage__CStorageFolder_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CStorage__CApplicationData_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CStorage__CApplicationData_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CStorage__CApplicationData_IInspectable __FITypedEventHandler_2_Windows__CStorage__CApplicationData_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CStorage__CApplicationData_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CStorage__CApplicationData_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CStorage__CApplicationData_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CStorage__CApplicationData_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CStorage__CApplicationData_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CStorage__CApplicationData_IInspectable* This,
        __x_ABI_CWindows_CStorage_CIApplicationData* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CStorage__CApplicationData_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CStorage__CApplicationData_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CStorage__CApplicationData_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CStorage__CApplicationData_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CStorage__CApplicationData_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CStorage__CApplicationData_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CStorage__CApplicationData_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CStorage__CApplicationData_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CStorage__CStorageLibrary_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CStorage__CStorageLibrary_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CStorage__CStorageLibrary_IInspectable __FITypedEventHandler_2_Windows__CStorage__CStorageLibrary_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CStorage__CStorageLibrary_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CStorage__CStorageLibrary_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CStorage__CStorageLibrary_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CStorage__CStorageLibrary_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CStorage__CStorageLibrary_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CStorage__CStorageLibrary_IInspectable* This,
        __x_ABI_CWindows_CStorage_CIStorageLibrary* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CStorage__CStorageLibrary_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CStorage__CStorageLibrary_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CStorage__CStorageLibrary_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CStorage__CStorageLibrary_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CStorage__CStorageLibrary_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CStorage__CStorageLibrary_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CStorage__CStorageLibrary_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CStorage__CStorageLibrary_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CFoundation_CCollections_CCollectionChange __x_ABI_CWindows_CFoundation_CCollections_CCollectionChange;

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet;

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs __x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs;

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIVectorChangedEventArgs_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

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

#ifndef ____x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties __x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties;

#endif // ____x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CStorage_CFileProperties_CThumbnailMode __x_ABI_CWindows_CStorage_CFileProperties_CThumbnailMode;

typedef enum __x_ABI_CWindows_CStorage_CFileProperties_CThumbnailOptions __x_ABI_CWindows_CStorage_CFileProperties_CThumbnailOptions;

#ifndef ____x_ABI_CWindows_CStorage_CSearch_CIStorageFolderQueryOperations_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CSearch_CIStorageFolderQueryOperations_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CSearch_CIStorageFolderQueryOperations __x_ABI_CWindows_CStorage_CSearch_CIStorageFolderQueryOperations;

#endif // ____x_ABI_CWindows_CStorage_CSearch_CIStorageFolderQueryOperations_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference __x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStreamReference_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIOutputStream __x_ABI_CWindows_CStorage_CStreams_CIOutputStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CStorage_CStreams_CUnicodeEncoding __x_ABI_CWindows_CStorage_CStreams_CUnicodeEncoding;

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIUser __x_ABI_CWindows_CSystem_CIUser;

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CStorage_CApplicationDataCreateDisposition __x_ABI_CWindows_CStorage_CApplicationDataCreateDisposition;

typedef enum __x_ABI_CWindows_CStorage_CApplicationDataLocality __x_ABI_CWindows_CStorage_CApplicationDataLocality;

typedef enum __x_ABI_CWindows_CStorage_CCreationCollisionOption __x_ABI_CWindows_CStorage_CCreationCollisionOption;

typedef enum __x_ABI_CWindows_CStorage_CFileAccessMode __x_ABI_CWindows_CStorage_CFileAccessMode;

typedef enum __x_ABI_CWindows_CStorage_CFileAttributes __x_ABI_CWindows_CStorage_CFileAttributes;

typedef enum __x_ABI_CWindows_CStorage_CKnownFolderId __x_ABI_CWindows_CStorage_CKnownFolderId;

typedef enum __x_ABI_CWindows_CStorage_CKnownLibraryId __x_ABI_CWindows_CStorage_CKnownLibraryId;

typedef enum __x_ABI_CWindows_CStorage_CNameCollisionOption __x_ABI_CWindows_CStorage_CNameCollisionOption;

typedef enum __x_ABI_CWindows_CStorage_CStorageDeleteOption __x_ABI_CWindows_CStorage_CStorageDeleteOption;

typedef enum __x_ABI_CWindows_CStorage_CStorageItemTypes __x_ABI_CWindows_CStorage_CStorageItemTypes;

typedef enum __x_ABI_CWindows_CStorage_CStorageLibraryChangeType __x_ABI_CWindows_CStorage_CStorageLibraryChangeType;

typedef enum __x_ABI_CWindows_CStorage_CStorageOpenOptions __x_ABI_CWindows_CStorage_CStorageOpenOptions;

typedef enum __x_ABI_CWindows_CStorage_CStreamedFileFailureMode __x_ABI_CWindows_CStorage_CStreamedFileFailureMode;

/*
 *
 * Struct Windows.Storage.ApplicationDataCreateDisposition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CStorage_CApplicationDataCreateDisposition
{
    ApplicationDataCreateDisposition_Always = 0,
    ApplicationDataCreateDisposition_Existing = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.ApplicationDataLocality
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CStorage_CApplicationDataLocality
{
    ApplicationDataLocality_Local = 0,
    ApplicationDataLocality_Roaming = 1,
    ApplicationDataLocality_Temporary = 2,
    ApplicationDataLocality_LocalCache = 3,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
    ApplicationDataLocality_SharedLocal = 4,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.CreationCollisionOption
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CStorage_CCreationCollisionOption
{
    CreationCollisionOption_GenerateUniqueName = 0,
    CreationCollisionOption_ReplaceExisting = 1,
    CreationCollisionOption_FailIfExists = 2,
    CreationCollisionOption_OpenIfExists = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.FileAccessMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CStorage_CFileAccessMode
{
    FileAccessMode_Read = 0,
    FileAccessMode_ReadWrite = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.FileAttributes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CStorage_CFileAttributes
{
    FileAttributes_Normal = 0,
    FileAttributes_ReadOnly = 0x1,
    FileAttributes_Directory = 0x10,
    FileAttributes_Archive = 0x20,
    FileAttributes_Temporary = 0x100,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    FileAttributes_LocallyIncomplete = 0x200,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.KnownFolderId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CStorage_CKnownFolderId
{
    KnownFolderId_AppCaptures = 0,
    KnownFolderId_CameraRoll = 1,
    KnownFolderId_DocumentsLibrary = 2,
    KnownFolderId_HomeGroup = 3,
    KnownFolderId_MediaServerDevices = 4,
    KnownFolderId_MusicLibrary = 5,
    KnownFolderId_Objects3D = 6,
    KnownFolderId_PicturesLibrary = 7,
    KnownFolderId_Playlists = 8,
    KnownFolderId_RecordedCalls = 9,
    KnownFolderId_RemovableDevices = 10,
    KnownFolderId_SavedPictures = 11,
    KnownFolderId_Screenshots = 12,
    KnownFolderId_VideosLibrary = 13,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    KnownFolderId_AllAppMods = 14,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    KnownFolderId_CurrentAppMods = 15,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
    KnownFolderId_DownloadsFolder = 16,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Storage.KnownFoldersAccessStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
enum __x_ABI_CWindows_CStorage_CKnownFoldersAccessStatus
{
    KnownFoldersAccessStatus_DeniedBySystem = 0,
    KnownFoldersAccessStatus_NotDeclaredByApp = 1,
    KnownFoldersAccessStatus_DeniedByUser = 2,
    KnownFoldersAccessStatus_UserPromptRequired = 3,
    KnownFoldersAccessStatus_Allowed = 4,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
    KnownFoldersAccessStatus_AllowedPerAppFolder = 5,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Struct Windows.Storage.KnownLibraryId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CStorage_CKnownLibraryId
{
    KnownLibraryId_Music = 0,
    KnownLibraryId_Pictures = 1,
    KnownLibraryId_Videos = 2,
    KnownLibraryId_Documents = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.NameCollisionOption
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CStorage_CNameCollisionOption
{
    NameCollisionOption_GenerateUniqueName = 0,
    NameCollisionOption_ReplaceExisting = 1,
    NameCollisionOption_FailIfExists = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.StorageDeleteOption
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CStorage_CStorageDeleteOption
{
    StorageDeleteOption_Default = 0,
    StorageDeleteOption_PermanentDelete = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.StorageItemTypes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CStorage_CStorageItemTypes
{
    StorageItemTypes_None = 0,
    StorageItemTypes_File = 0x1,
    StorageItemTypes_Folder = 0x2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.StorageLibraryChangeType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CStorage_CStorageLibraryChangeType
{
    StorageLibraryChangeType_Created = 0,
    StorageLibraryChangeType_Deleted = 1,
    StorageLibraryChangeType_MovedOrRenamed = 2,
    StorageLibraryChangeType_ContentsChanged = 3,
    StorageLibraryChangeType_MovedOutOfLibrary = 4,
    StorageLibraryChangeType_MovedIntoLibrary = 5,
    StorageLibraryChangeType_ContentsReplaced = 6,
    StorageLibraryChangeType_IndexingStatusChanged = 7,
    StorageLibraryChangeType_EncryptionChanged = 8,
    StorageLibraryChangeType_ChangeTrackingLost = 9,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Storage.StorageOpenOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CStorage_CStorageOpenOptions
{
    StorageOpenOptions_None = 0,
    StorageOpenOptions_AllowOnlyReaders = 0x1,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    StorageOpenOptions_AllowReadersAndWriters = 0x2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Storage.StreamedFileFailureMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CStorage_CStreamedFileFailureMode
{
    StreamedFileFailureMode_Failed = 0,
    StreamedFileFailureMode_CurrentlyUnavailable = 1,
    StreamedFileFailureMode_Incomplete = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Storage.ApplicationDataSetVersionHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIApplicationDataSetVersionHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIApplicationDataSetVersionHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CStorage_CIApplicationDataSetVersionHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIApplicationDataSetVersionHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIApplicationDataSetVersionHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIApplicationDataSetVersionHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CStorage_CIApplicationDataSetVersionHandler* This,
        __x_ABI_CWindows_CStorage_CISetVersionRequest* setVersionRequest);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIApplicationDataSetVersionHandlerVtbl;

interface __x_ABI_CWindows_CStorage_CIApplicationDataSetVersionHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIApplicationDataSetVersionHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIApplicationDataSetVersionHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIApplicationDataSetVersionHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIApplicationDataSetVersionHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIApplicationDataSetVersionHandler_Invoke(This, setVersionRequest) \
    ((This)->lpVtbl->Invoke(This, setVersionRequest))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIApplicationDataSetVersionHandler;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIApplicationDataSetVersionHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Storage.StreamedFileDataRequestedHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIStreamedFileDataRequestedHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStreamedFileDataRequestedHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CStorage_CIStreamedFileDataRequestedHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIStreamedFileDataRequestedHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIStreamedFileDataRequestedHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIStreamedFileDataRequestedHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CStorage_CIStreamedFileDataRequestedHandler* This,
        __x_ABI_CWindows_CStorage_CStreams_CIOutputStream* stream);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIStreamedFileDataRequestedHandlerVtbl;

interface __x_ABI_CWindows_CStorage_CIStreamedFileDataRequestedHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIStreamedFileDataRequestedHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIStreamedFileDataRequestedHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIStreamedFileDataRequestedHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIStreamedFileDataRequestedHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIStreamedFileDataRequestedHandler_Invoke(This, stream) \
    ((This)->lpVtbl->Invoke(This, stream))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStreamedFileDataRequestedHandler;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStreamedFileDataRequestedHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IAppDataPaths
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Storage.AppDataPaths
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CStorage_CIAppDataPaths_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIAppDataPaths_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IAppDataPaths[] = L"Windows.Storage.IAppDataPaths";
typedef struct __x_ABI_CWindows_CStorage_CIAppDataPathsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIAppDataPaths* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIAppDataPaths* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIAppDataPaths* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIAppDataPaths* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIAppDataPaths* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIAppDataPaths* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Cookies)(__x_ABI_CWindows_CStorage_CIAppDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Desktop)(__x_ABI_CWindows_CStorage_CIAppDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Documents)(__x_ABI_CWindows_CStorage_CIAppDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Favorites)(__x_ABI_CWindows_CStorage_CIAppDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_History)(__x_ABI_CWindows_CStorage_CIAppDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_InternetCache)(__x_ABI_CWindows_CStorage_CIAppDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_LocalAppData)(__x_ABI_CWindows_CStorage_CIAppDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ProgramData)(__x_ABI_CWindows_CStorage_CIAppDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_RoamingAppData)(__x_ABI_CWindows_CStorage_CIAppDataPaths* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIAppDataPathsVtbl;

interface __x_ABI_CWindows_CStorage_CIAppDataPaths
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIAppDataPathsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIAppDataPaths_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIAppDataPaths_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIAppDataPaths_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIAppDataPaths_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIAppDataPaths_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIAppDataPaths_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIAppDataPaths_get_Cookies(This, value) \
    ((This)->lpVtbl->get_Cookies(This, value))

#define __x_ABI_CWindows_CStorage_CIAppDataPaths_get_Desktop(This, value) \
    ((This)->lpVtbl->get_Desktop(This, value))

#define __x_ABI_CWindows_CStorage_CIAppDataPaths_get_Documents(This, value) \
    ((This)->lpVtbl->get_Documents(This, value))

#define __x_ABI_CWindows_CStorage_CIAppDataPaths_get_Favorites(This, value) \
    ((This)->lpVtbl->get_Favorites(This, value))

#define __x_ABI_CWindows_CStorage_CIAppDataPaths_get_History(This, value) \
    ((This)->lpVtbl->get_History(This, value))

#define __x_ABI_CWindows_CStorage_CIAppDataPaths_get_InternetCache(This, value) \
    ((This)->lpVtbl->get_InternetCache(This, value))

#define __x_ABI_CWindows_CStorage_CIAppDataPaths_get_LocalAppData(This, value) \
    ((This)->lpVtbl->get_LocalAppData(This, value))

#define __x_ABI_CWindows_CStorage_CIAppDataPaths_get_ProgramData(This, value) \
    ((This)->lpVtbl->get_ProgramData(This, value))

#define __x_ABI_CWindows_CStorage_CIAppDataPaths_get_RoamingAppData(This, value) \
    ((This)->lpVtbl->get_RoamingAppData(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIAppDataPaths;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIAppDataPaths_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Storage.IAppDataPathsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Storage.AppDataPaths
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CStorage_CIAppDataPathsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIAppDataPathsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IAppDataPathsStatics[] = L"Windows.Storage.IAppDataPathsStatics";
typedef struct __x_ABI_CWindows_CStorage_CIAppDataPathsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIAppDataPathsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIAppDataPathsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIAppDataPathsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIAppDataPathsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIAppDataPathsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIAppDataPathsStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForUser)(__x_ABI_CWindows_CStorage_CIAppDataPathsStatics* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __x_ABI_CWindows_CStorage_CIAppDataPaths** result);
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CStorage_CIAppDataPathsStatics* This,
        __x_ABI_CWindows_CStorage_CIAppDataPaths** result);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIAppDataPathsStaticsVtbl;

interface __x_ABI_CWindows_CStorage_CIAppDataPathsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIAppDataPathsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIAppDataPathsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIAppDataPathsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIAppDataPathsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIAppDataPathsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIAppDataPathsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIAppDataPathsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIAppDataPathsStatics_GetForUser(This, user, result) \
    ((This)->lpVtbl->GetForUser(This, user, result))

#define __x_ABI_CWindows_CStorage_CIAppDataPathsStatics_GetDefault(This, result) \
    ((This)->lpVtbl->GetDefault(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIAppDataPathsStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIAppDataPathsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Storage.IApplicationData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.ApplicationData
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIApplicationData_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIApplicationData_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IApplicationData[] = L"Windows.Storage.IApplicationData";
typedef struct __x_ABI_CWindows_CStorage_CIApplicationDataVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIApplicationData* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIApplicationData* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIApplicationData* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIApplicationData* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIApplicationData* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIApplicationData* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Version)(__x_ABI_CWindows_CStorage_CIApplicationData* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* SetVersionAsync)(__x_ABI_CWindows_CStorage_CIApplicationData* This,
        UINT32 desiredVersion,
        __x_ABI_CWindows_CStorage_CIApplicationDataSetVersionHandler* handler,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** setVersionOperation);
    HRESULT (STDMETHODCALLTYPE* ClearAllAsync)(__x_ABI_CWindows_CStorage_CIApplicationData* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** clearOperation);
    HRESULT (STDMETHODCALLTYPE* ClearAsync)(__x_ABI_CWindows_CStorage_CIApplicationData* This,
        enum __x_ABI_CWindows_CStorage_CApplicationDataLocality locality,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** clearOperation);
    HRESULT (STDMETHODCALLTYPE* get_LocalSettings)(__x_ABI_CWindows_CStorage_CIApplicationData* This,
        __x_ABI_CWindows_CStorage_CIApplicationDataContainer** value);
    HRESULT (STDMETHODCALLTYPE* get_RoamingSettings)(__x_ABI_CWindows_CStorage_CIApplicationData* This,
        __x_ABI_CWindows_CStorage_CIApplicationDataContainer** value);
    HRESULT (STDMETHODCALLTYPE* get_LocalFolder)(__x_ABI_CWindows_CStorage_CIApplicationData* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder** value);
    HRESULT (STDMETHODCALLTYPE* get_RoamingFolder)(__x_ABI_CWindows_CStorage_CIApplicationData* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder** value);
    HRESULT (STDMETHODCALLTYPE* get_TemporaryFolder)(__x_ABI_CWindows_CStorage_CIApplicationData* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder** value);
    HRESULT (STDMETHODCALLTYPE* add_DataChanged)(__x_ABI_CWindows_CStorage_CIApplicationData* This,
        __FITypedEventHandler_2_Windows__CStorage__CApplicationData_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_DataChanged)(__x_ABI_CWindows_CStorage_CIApplicationData* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* SignalDataChanged)(__x_ABI_CWindows_CStorage_CIApplicationData* This);
    HRESULT (STDMETHODCALLTYPE* get_RoamingStorageQuota)(__x_ABI_CWindows_CStorage_CIApplicationData* This,
        UINT64* value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIApplicationDataVtbl;

interface __x_ABI_CWindows_CStorage_CIApplicationData
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIApplicationDataVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIApplicationData_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIApplicationData_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIApplicationData_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIApplicationData_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIApplicationData_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIApplicationData_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIApplicationData_get_Version(This, value) \
    ((This)->lpVtbl->get_Version(This, value))

#define __x_ABI_CWindows_CStorage_CIApplicationData_SetVersionAsync(This, desiredVersion, handler, setVersionOperation) \
    ((This)->lpVtbl->SetVersionAsync(This, desiredVersion, handler, setVersionOperation))

#define __x_ABI_CWindows_CStorage_CIApplicationData_ClearAllAsync(This, clearOperation) \
    ((This)->lpVtbl->ClearAllAsync(This, clearOperation))

#define __x_ABI_CWindows_CStorage_CIApplicationData_ClearAsync(This, locality, clearOperation) \
    ((This)->lpVtbl->ClearAsync(This, locality, clearOperation))

#define __x_ABI_CWindows_CStorage_CIApplicationData_get_LocalSettings(This, value) \
    ((This)->lpVtbl->get_LocalSettings(This, value))

#define __x_ABI_CWindows_CStorage_CIApplicationData_get_RoamingSettings(This, value) \
    ((This)->lpVtbl->get_RoamingSettings(This, value))

#define __x_ABI_CWindows_CStorage_CIApplicationData_get_LocalFolder(This, value) \
    ((This)->lpVtbl->get_LocalFolder(This, value))

#define __x_ABI_CWindows_CStorage_CIApplicationData_get_RoamingFolder(This, value) \
    ((This)->lpVtbl->get_RoamingFolder(This, value))

#define __x_ABI_CWindows_CStorage_CIApplicationData_get_TemporaryFolder(This, value) \
    ((This)->lpVtbl->get_TemporaryFolder(This, value))

#define __x_ABI_CWindows_CStorage_CIApplicationData_add_DataChanged(This, handler, token) \
    ((This)->lpVtbl->add_DataChanged(This, handler, token))

#define __x_ABI_CWindows_CStorage_CIApplicationData_remove_DataChanged(This, token) \
    ((This)->lpVtbl->remove_DataChanged(This, token))

#define __x_ABI_CWindows_CStorage_CIApplicationData_SignalDataChanged(This) \
    ((This)->lpVtbl->SignalDataChanged(This))

#define __x_ABI_CWindows_CStorage_CIApplicationData_get_RoamingStorageQuota(This, value) \
    ((This)->lpVtbl->get_RoamingStorageQuota(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIApplicationData;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIApplicationData_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IApplicationData2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.ApplicationData
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIApplicationData2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIApplicationData2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IApplicationData2[] = L"Windows.Storage.IApplicationData2";
typedef struct __x_ABI_CWindows_CStorage_CIApplicationData2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIApplicationData2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIApplicationData2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIApplicationData2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIApplicationData2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIApplicationData2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIApplicationData2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LocalCacheFolder)(__x_ABI_CWindows_CStorage_CIApplicationData2* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder** value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIApplicationData2Vtbl;

interface __x_ABI_CWindows_CStorage_CIApplicationData2
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIApplicationData2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIApplicationData2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIApplicationData2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIApplicationData2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIApplicationData2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIApplicationData2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIApplicationData2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIApplicationData2_get_LocalCacheFolder(This, value) \
    ((This)->lpVtbl->get_LocalCacheFolder(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIApplicationData2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIApplicationData2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IApplicationData3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.ApplicationData
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIApplicationData3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIApplicationData3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IApplicationData3[] = L"Windows.Storage.IApplicationData3";
typedef struct __x_ABI_CWindows_CStorage_CIApplicationData3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIApplicationData3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIApplicationData3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIApplicationData3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIApplicationData3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIApplicationData3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIApplicationData3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetPublisherCacheFolder)(__x_ABI_CWindows_CStorage_CIApplicationData3* This,
        HSTRING folderName,
        __x_ABI_CWindows_CStorage_CIStorageFolder** value);
    HRESULT (STDMETHODCALLTYPE* ClearPublisherCacheFolderAsync)(__x_ABI_CWindows_CStorage_CIApplicationData3* This,
        HSTRING folderName,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** clearOperation);
    HRESULT (STDMETHODCALLTYPE* get_SharedLocalFolder)(__x_ABI_CWindows_CStorage_CIApplicationData3* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder** value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIApplicationData3Vtbl;

interface __x_ABI_CWindows_CStorage_CIApplicationData3
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIApplicationData3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIApplicationData3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIApplicationData3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIApplicationData3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIApplicationData3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIApplicationData3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIApplicationData3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIApplicationData3_GetPublisherCacheFolder(This, folderName, value) \
    ((This)->lpVtbl->GetPublisherCacheFolder(This, folderName, value))

#define __x_ABI_CWindows_CStorage_CIApplicationData3_ClearPublisherCacheFolderAsync(This, folderName, clearOperation) \
    ((This)->lpVtbl->ClearPublisherCacheFolderAsync(This, folderName, clearOperation))

#define __x_ABI_CWindows_CStorage_CIApplicationData3_get_SharedLocalFolder(This, value) \
    ((This)->lpVtbl->get_SharedLocalFolder(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIApplicationData3;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIApplicationData3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IApplicationDataContainer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.ApplicationDataContainer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIApplicationDataContainer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIApplicationDataContainer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IApplicationDataContainer[] = L"Windows.Storage.IApplicationDataContainer";
typedef struct __x_ABI_CWindows_CStorage_CIApplicationDataContainerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIApplicationDataContainer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIApplicationDataContainer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIApplicationDataContainer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIApplicationDataContainer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIApplicationDataContainer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIApplicationDataContainer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CStorage_CIApplicationDataContainer* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Locality)(__x_ABI_CWindows_CStorage_CIApplicationDataContainer* This,
        enum __x_ABI_CWindows_CStorage_CApplicationDataLocality* value);
    HRESULT (STDMETHODCALLTYPE* get_Values)(__x_ABI_CWindows_CStorage_CIApplicationDataContainer* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** value);
    HRESULT (STDMETHODCALLTYPE* get_Containers)(__x_ABI_CWindows_CStorage_CIApplicationDataContainer* This,
        __FIMapView_2_HSTRING_Windows__CStorage__CApplicationDataContainer** value);
    HRESULT (STDMETHODCALLTYPE* CreateContainer)(__x_ABI_CWindows_CStorage_CIApplicationDataContainer* This,
        HSTRING name,
        enum __x_ABI_CWindows_CStorage_CApplicationDataCreateDisposition disposition,
        __x_ABI_CWindows_CStorage_CIApplicationDataContainer** container);
    HRESULT (STDMETHODCALLTYPE* DeleteContainer)(__x_ABI_CWindows_CStorage_CIApplicationDataContainer* This,
        HSTRING name);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIApplicationDataContainerVtbl;

interface __x_ABI_CWindows_CStorage_CIApplicationDataContainer
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIApplicationDataContainerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIApplicationDataContainer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIApplicationDataContainer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIApplicationDataContainer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIApplicationDataContainer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIApplicationDataContainer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIApplicationDataContainer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIApplicationDataContainer_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CStorage_CIApplicationDataContainer_get_Locality(This, value) \
    ((This)->lpVtbl->get_Locality(This, value))

#define __x_ABI_CWindows_CStorage_CIApplicationDataContainer_get_Values(This, value) \
    ((This)->lpVtbl->get_Values(This, value))

#define __x_ABI_CWindows_CStorage_CIApplicationDataContainer_get_Containers(This, value) \
    ((This)->lpVtbl->get_Containers(This, value))

#define __x_ABI_CWindows_CStorage_CIApplicationDataContainer_CreateContainer(This, name, disposition, container) \
    ((This)->lpVtbl->CreateContainer(This, name, disposition, container))

#define __x_ABI_CWindows_CStorage_CIApplicationDataContainer_DeleteContainer(This, name) \
    ((This)->lpVtbl->DeleteContainer(This, name))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIApplicationDataContainer;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIApplicationDataContainer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IApplicationDataStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.ApplicationData
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIApplicationDataStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIApplicationDataStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IApplicationDataStatics[] = L"Windows.Storage.IApplicationDataStatics";
typedef struct __x_ABI_CWindows_CStorage_CIApplicationDataStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIApplicationDataStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIApplicationDataStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIApplicationDataStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIApplicationDataStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIApplicationDataStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIApplicationDataStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__x_ABI_CWindows_CStorage_CIApplicationDataStatics* This,
        __x_ABI_CWindows_CStorage_CIApplicationData** value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIApplicationDataStaticsVtbl;

interface __x_ABI_CWindows_CStorage_CIApplicationDataStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIApplicationDataStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIApplicationDataStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIApplicationDataStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIApplicationDataStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIApplicationDataStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIApplicationDataStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIApplicationDataStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIApplicationDataStatics_get_Current(This, value) \
    ((This)->lpVtbl->get_Current(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIApplicationDataStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIApplicationDataStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IApplicationDataStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.ApplicationData
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIApplicationDataStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIApplicationDataStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IApplicationDataStatics2[] = L"Windows.Storage.IApplicationDataStatics2";
typedef struct __x_ABI_CWindows_CStorage_CIApplicationDataStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIApplicationDataStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIApplicationDataStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIApplicationDataStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIApplicationDataStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIApplicationDataStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIApplicationDataStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForUserAsync)(__x_ABI_CWindows_CStorage_CIApplicationDataStatics2* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __FIAsyncOperation_1_Windows__CStorage__CApplicationData** getForUserOperation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIApplicationDataStatics2Vtbl;

interface __x_ABI_CWindows_CStorage_CIApplicationDataStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIApplicationDataStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIApplicationDataStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIApplicationDataStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIApplicationDataStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIApplicationDataStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIApplicationDataStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIApplicationDataStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIApplicationDataStatics2_GetForUserAsync(This, user, getForUserOperation) \
    ((This)->lpVtbl->GetForUserAsync(This, user, getForUserOperation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIApplicationDataStatics2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIApplicationDataStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.ICachedFileManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.CachedFileManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CICachedFileManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CICachedFileManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_ICachedFileManagerStatics[] = L"Windows.Storage.ICachedFileManagerStatics";
typedef struct __x_ABI_CWindows_CStorage_CICachedFileManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CICachedFileManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CICachedFileManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CICachedFileManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CICachedFileManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CICachedFileManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CICachedFileManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* DeferUpdates)(__x_ABI_CWindows_CStorage_CICachedFileManagerStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file);
    HRESULT (STDMETHODCALLTYPE* CompleteUpdatesAsync)(__x_ABI_CWindows_CStorage_CICachedFileManagerStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        __FIAsyncOperation_1_Windows__CStorage__CProvider__CFileUpdateStatus** operation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CICachedFileManagerStaticsVtbl;

interface __x_ABI_CWindows_CStorage_CICachedFileManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CICachedFileManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CICachedFileManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CICachedFileManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CICachedFileManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CICachedFileManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CICachedFileManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CICachedFileManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CICachedFileManagerStatics_DeferUpdates(This, file) \
    ((This)->lpVtbl->DeferUpdates(This, file))

#define __x_ABI_CWindows_CStorage_CICachedFileManagerStatics_CompleteUpdatesAsync(This, file, operation) \
    ((This)->lpVtbl->CompleteUpdatesAsync(This, file, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CICachedFileManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CICachedFileManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IDownloadsFolderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.DownloadsFolder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIDownloadsFolderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIDownloadsFolderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IDownloadsFolderStatics[] = L"Windows.Storage.IDownloadsFolderStatics";
typedef struct __x_ABI_CWindows_CStorage_CIDownloadsFolderStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIDownloadsFolderStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIDownloadsFolderStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIDownloadsFolderStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIDownloadsFolderStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIDownloadsFolderStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIDownloadsFolderStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFileAsync)(__x_ABI_CWindows_CStorage_CIDownloadsFolderStatics* This,
        HSTRING desiredName,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation);
    HRESULT (STDMETHODCALLTYPE* CreateFolderAsync)(__x_ABI_CWindows_CStorage_CIDownloadsFolderStatics* This,
        HSTRING desiredName,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFolder** operation);
    HRESULT (STDMETHODCALLTYPE* CreateFileWithCollisionOptionAsync)(__x_ABI_CWindows_CStorage_CIDownloadsFolderStatics* This,
        HSTRING desiredName,
        enum __x_ABI_CWindows_CStorage_CCreationCollisionOption option,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation);
    HRESULT (STDMETHODCALLTYPE* CreateFolderWithCollisionOptionAsync)(__x_ABI_CWindows_CStorage_CIDownloadsFolderStatics* This,
        HSTRING desiredName,
        enum __x_ABI_CWindows_CStorage_CCreationCollisionOption option,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFolder** operation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIDownloadsFolderStaticsVtbl;

interface __x_ABI_CWindows_CStorage_CIDownloadsFolderStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIDownloadsFolderStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIDownloadsFolderStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIDownloadsFolderStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIDownloadsFolderStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIDownloadsFolderStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIDownloadsFolderStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIDownloadsFolderStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIDownloadsFolderStatics_CreateFileAsync(This, desiredName, operation) \
    ((This)->lpVtbl->CreateFileAsync(This, desiredName, operation))

#define __x_ABI_CWindows_CStorage_CIDownloadsFolderStatics_CreateFolderAsync(This, desiredName, operation) \
    ((This)->lpVtbl->CreateFolderAsync(This, desiredName, operation))

#define __x_ABI_CWindows_CStorage_CIDownloadsFolderStatics_CreateFileWithCollisionOptionAsync(This, desiredName, option, operation) \
    ((This)->lpVtbl->CreateFileWithCollisionOptionAsync(This, desiredName, option, operation))

#define __x_ABI_CWindows_CStorage_CIDownloadsFolderStatics_CreateFolderWithCollisionOptionAsync(This, desiredName, option, operation) \
    ((This)->lpVtbl->CreateFolderWithCollisionOptionAsync(This, desiredName, option, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIDownloadsFolderStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIDownloadsFolderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IDownloadsFolderStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Storage.DownloadsFolder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IDownloadsFolderStatics2[] = L"Windows.Storage.IDownloadsFolderStatics2";
typedef struct __x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFileForUserAsync)(__x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING desiredName,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation);
    HRESULT (STDMETHODCALLTYPE* CreateFolderForUserAsync)(__x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING desiredName,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFolder** operation);
    HRESULT (STDMETHODCALLTYPE* CreateFileForUserWithCollisionOptionAsync)(__x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING desiredName,
        enum __x_ABI_CWindows_CStorage_CCreationCollisionOption option,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation);
    HRESULT (STDMETHODCALLTYPE* CreateFolderForUserWithCollisionOptionAsync)(__x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING desiredName,
        enum __x_ABI_CWindows_CStorage_CCreationCollisionOption option,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFolder** operation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2Vtbl;

interface __x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2_CreateFileForUserAsync(This, user, desiredName, operation) \
    ((This)->lpVtbl->CreateFileForUserAsync(This, user, desiredName, operation))

#define __x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2_CreateFolderForUserAsync(This, user, desiredName, operation) \
    ((This)->lpVtbl->CreateFolderForUserAsync(This, user, desiredName, operation))

#define __x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2_CreateFileForUserWithCollisionOptionAsync(This, user, desiredName, option, operation) \
    ((This)->lpVtbl->CreateFileForUserWithCollisionOptionAsync(This, user, desiredName, option, operation))

#define __x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2_CreateFolderForUserWithCollisionOptionAsync(This, user, desiredName, option, operation) \
    ((This)->lpVtbl->CreateFolderForUserWithCollisionOptionAsync(This, user, desiredName, option, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIDownloadsFolderStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Storage.IFileIOStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.FileIO
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIFileIOStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIFileIOStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IFileIOStatics[] = L"Windows.Storage.IFileIOStatics";
typedef struct __x_ABI_CWindows_CStorage_CIFileIOStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIFileIOStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIFileIOStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIFileIOStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIFileIOStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIFileIOStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIFileIOStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReadTextAsync)(__x_ABI_CWindows_CStorage_CIFileIOStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        __FIAsyncOperation_1_HSTRING** textOperation);
    HRESULT (STDMETHODCALLTYPE* ReadTextWithEncodingAsync)(__x_ABI_CWindows_CStorage_CIFileIOStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        enum __x_ABI_CWindows_CStorage_CStreams_CUnicodeEncoding encoding,
        __FIAsyncOperation_1_HSTRING** textOperation);
    HRESULT (STDMETHODCALLTYPE* WriteTextAsync)(__x_ABI_CWindows_CStorage_CIFileIOStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        HSTRING contents,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** textOperation);
    HRESULT (STDMETHODCALLTYPE* WriteTextWithEncodingAsync)(__x_ABI_CWindows_CStorage_CIFileIOStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        HSTRING contents,
        enum __x_ABI_CWindows_CStorage_CStreams_CUnicodeEncoding encoding,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** textOperation);
    HRESULT (STDMETHODCALLTYPE* AppendTextAsync)(__x_ABI_CWindows_CStorage_CIFileIOStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        HSTRING contents,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** textOperation);
    HRESULT (STDMETHODCALLTYPE* AppendTextWithEncodingAsync)(__x_ABI_CWindows_CStorage_CIFileIOStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        HSTRING contents,
        enum __x_ABI_CWindows_CStorage_CStreams_CUnicodeEncoding encoding,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** textOperation);
    HRESULT (STDMETHODCALLTYPE* ReadLinesAsync)(__x_ABI_CWindows_CStorage_CIFileIOStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        __FIAsyncOperation_1___FIVector_1_HSTRING** linesOperation);
    HRESULT (STDMETHODCALLTYPE* ReadLinesWithEncodingAsync)(__x_ABI_CWindows_CStorage_CIFileIOStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        enum __x_ABI_CWindows_CStorage_CStreams_CUnicodeEncoding encoding,
        __FIAsyncOperation_1___FIVector_1_HSTRING** linesOperation);
    HRESULT (STDMETHODCALLTYPE* WriteLinesAsync)(__x_ABI_CWindows_CStorage_CIFileIOStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        __FIIterable_1_HSTRING* lines,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* WriteLinesWithEncodingAsync)(__x_ABI_CWindows_CStorage_CIFileIOStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        __FIIterable_1_HSTRING* lines,
        enum __x_ABI_CWindows_CStorage_CStreams_CUnicodeEncoding encoding,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* AppendLinesAsync)(__x_ABI_CWindows_CStorage_CIFileIOStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        __FIIterable_1_HSTRING* lines,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* AppendLinesWithEncodingAsync)(__x_ABI_CWindows_CStorage_CIFileIOStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        __FIIterable_1_HSTRING* lines,
        enum __x_ABI_CWindows_CStorage_CStreams_CUnicodeEncoding encoding,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* ReadBufferAsync)(__x_ABI_CWindows_CStorage_CIFileIOStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer** operation);
    HRESULT (STDMETHODCALLTYPE* WriteBufferAsync)(__x_ABI_CWindows_CStorage_CIFileIOStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* buffer,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* WriteBytesAsync)(__x_ABI_CWindows_CStorage_CIFileIOStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        UINT32 bufferLength,
        BYTE* buffer,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIFileIOStaticsVtbl;

interface __x_ABI_CWindows_CStorage_CIFileIOStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIFileIOStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIFileIOStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIFileIOStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIFileIOStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIFileIOStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIFileIOStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIFileIOStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIFileIOStatics_ReadTextAsync(This, file, textOperation) \
    ((This)->lpVtbl->ReadTextAsync(This, file, textOperation))

#define __x_ABI_CWindows_CStorage_CIFileIOStatics_ReadTextWithEncodingAsync(This, file, encoding, textOperation) \
    ((This)->lpVtbl->ReadTextWithEncodingAsync(This, file, encoding, textOperation))

#define __x_ABI_CWindows_CStorage_CIFileIOStatics_WriteTextAsync(This, file, contents, textOperation) \
    ((This)->lpVtbl->WriteTextAsync(This, file, contents, textOperation))

#define __x_ABI_CWindows_CStorage_CIFileIOStatics_WriteTextWithEncodingAsync(This, file, contents, encoding, textOperation) \
    ((This)->lpVtbl->WriteTextWithEncodingAsync(This, file, contents, encoding, textOperation))

#define __x_ABI_CWindows_CStorage_CIFileIOStatics_AppendTextAsync(This, file, contents, textOperation) \
    ((This)->lpVtbl->AppendTextAsync(This, file, contents, textOperation))

#define __x_ABI_CWindows_CStorage_CIFileIOStatics_AppendTextWithEncodingAsync(This, file, contents, encoding, textOperation) \
    ((This)->lpVtbl->AppendTextWithEncodingAsync(This, file, contents, encoding, textOperation))

#define __x_ABI_CWindows_CStorage_CIFileIOStatics_ReadLinesAsync(This, file, linesOperation) \
    ((This)->lpVtbl->ReadLinesAsync(This, file, linesOperation))

#define __x_ABI_CWindows_CStorage_CIFileIOStatics_ReadLinesWithEncodingAsync(This, file, encoding, linesOperation) \
    ((This)->lpVtbl->ReadLinesWithEncodingAsync(This, file, encoding, linesOperation))

#define __x_ABI_CWindows_CStorage_CIFileIOStatics_WriteLinesAsync(This, file, lines, operation) \
    ((This)->lpVtbl->WriteLinesAsync(This, file, lines, operation))

#define __x_ABI_CWindows_CStorage_CIFileIOStatics_WriteLinesWithEncodingAsync(This, file, lines, encoding, operation) \
    ((This)->lpVtbl->WriteLinesWithEncodingAsync(This, file, lines, encoding, operation))

#define __x_ABI_CWindows_CStorage_CIFileIOStatics_AppendLinesAsync(This, file, lines, operation) \
    ((This)->lpVtbl->AppendLinesAsync(This, file, lines, operation))

#define __x_ABI_CWindows_CStorage_CIFileIOStatics_AppendLinesWithEncodingAsync(This, file, lines, encoding, operation) \
    ((This)->lpVtbl->AppendLinesWithEncodingAsync(This, file, lines, encoding, operation))

#define __x_ABI_CWindows_CStorage_CIFileIOStatics_ReadBufferAsync(This, file, operation) \
    ((This)->lpVtbl->ReadBufferAsync(This, file, operation))

#define __x_ABI_CWindows_CStorage_CIFileIOStatics_WriteBufferAsync(This, file, buffer, operation) \
    ((This)->lpVtbl->WriteBufferAsync(This, file, buffer, operation))

#define __x_ABI_CWindows_CStorage_CIFileIOStatics_WriteBytesAsync(This, file, bufferLength, buffer, operation) \
    ((This)->lpVtbl->WriteBytesAsync(This, file, bufferLength, buffer, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIFileIOStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIFileIOStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IKnownFoldersCameraRollStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.KnownFolders
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIKnownFoldersCameraRollStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIKnownFoldersCameraRollStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IKnownFoldersCameraRollStatics[] = L"Windows.Storage.IKnownFoldersCameraRollStatics";
typedef struct __x_ABI_CWindows_CStorage_CIKnownFoldersCameraRollStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIKnownFoldersCameraRollStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIKnownFoldersCameraRollStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIKnownFoldersCameraRollStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIKnownFoldersCameraRollStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIKnownFoldersCameraRollStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIKnownFoldersCameraRollStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CameraRoll)(__x_ABI_CWindows_CStorage_CIKnownFoldersCameraRollStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder** value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIKnownFoldersCameraRollStaticsVtbl;

interface __x_ABI_CWindows_CStorage_CIKnownFoldersCameraRollStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIKnownFoldersCameraRollStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIKnownFoldersCameraRollStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersCameraRollStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersCameraRollStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersCameraRollStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersCameraRollStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersCameraRollStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersCameraRollStatics_get_CameraRoll(This, value) \
    ((This)->lpVtbl->get_CameraRoll(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIKnownFoldersCameraRollStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIKnownFoldersCameraRollStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IKnownFoldersPlaylistsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.KnownFolders
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIKnownFoldersPlaylistsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIKnownFoldersPlaylistsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IKnownFoldersPlaylistsStatics[] = L"Windows.Storage.IKnownFoldersPlaylistsStatics";
typedef struct __x_ABI_CWindows_CStorage_CIKnownFoldersPlaylistsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIKnownFoldersPlaylistsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIKnownFoldersPlaylistsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIKnownFoldersPlaylistsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIKnownFoldersPlaylistsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIKnownFoldersPlaylistsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIKnownFoldersPlaylistsStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Playlists)(__x_ABI_CWindows_CStorage_CIKnownFoldersPlaylistsStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder** value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIKnownFoldersPlaylistsStaticsVtbl;

interface __x_ABI_CWindows_CStorage_CIKnownFoldersPlaylistsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIKnownFoldersPlaylistsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIKnownFoldersPlaylistsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersPlaylistsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersPlaylistsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersPlaylistsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersPlaylistsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersPlaylistsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersPlaylistsStatics_get_Playlists(This, value) \
    ((This)->lpVtbl->get_Playlists(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIKnownFoldersPlaylistsStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIKnownFoldersPlaylistsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IKnownFoldersSavedPicturesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.KnownFolders
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIKnownFoldersSavedPicturesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIKnownFoldersSavedPicturesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IKnownFoldersSavedPicturesStatics[] = L"Windows.Storage.IKnownFoldersSavedPicturesStatics";
typedef struct __x_ABI_CWindows_CStorage_CIKnownFoldersSavedPicturesStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIKnownFoldersSavedPicturesStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIKnownFoldersSavedPicturesStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIKnownFoldersSavedPicturesStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIKnownFoldersSavedPicturesStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIKnownFoldersSavedPicturesStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIKnownFoldersSavedPicturesStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SavedPictures)(__x_ABI_CWindows_CStorage_CIKnownFoldersSavedPicturesStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder** value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIKnownFoldersSavedPicturesStaticsVtbl;

interface __x_ABI_CWindows_CStorage_CIKnownFoldersSavedPicturesStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIKnownFoldersSavedPicturesStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIKnownFoldersSavedPicturesStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersSavedPicturesStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersSavedPicturesStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersSavedPicturesStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersSavedPicturesStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersSavedPicturesStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersSavedPicturesStatics_get_SavedPictures(This, value) \
    ((This)->lpVtbl->get_SavedPictures(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIKnownFoldersSavedPicturesStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIKnownFoldersSavedPicturesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IKnownFoldersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.KnownFolders
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIKnownFoldersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIKnownFoldersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IKnownFoldersStatics[] = L"Windows.Storage.IKnownFoldersStatics";
typedef struct __x_ABI_CWindows_CStorage_CIKnownFoldersStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MusicLibrary)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder** value);
    HRESULT (STDMETHODCALLTYPE* get_PicturesLibrary)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder** value);
    HRESULT (STDMETHODCALLTYPE* get_VideosLibrary)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder** value);
    HRESULT (STDMETHODCALLTYPE* get_DocumentsLibrary)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder** value);
    HRESULT (STDMETHODCALLTYPE* get_HomeGroup)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder** value);
    HRESULT (STDMETHODCALLTYPE* get_RemovableDevices)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder** value);
    HRESULT (STDMETHODCALLTYPE* get_MediaServerDevices)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder** value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIKnownFoldersStaticsVtbl;

interface __x_ABI_CWindows_CStorage_CIKnownFoldersStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIKnownFoldersStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics_get_MusicLibrary(This, value) \
    ((This)->lpVtbl->get_MusicLibrary(This, value))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics_get_PicturesLibrary(This, value) \
    ((This)->lpVtbl->get_PicturesLibrary(This, value))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics_get_VideosLibrary(This, value) \
    ((This)->lpVtbl->get_VideosLibrary(This, value))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics_get_DocumentsLibrary(This, value) \
    ((This)->lpVtbl->get_DocumentsLibrary(This, value))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics_get_HomeGroup(This, value) \
    ((This)->lpVtbl->get_HomeGroup(This, value))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics_get_RemovableDevices(This, value) \
    ((This)->lpVtbl->get_RemovableDevices(This, value))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics_get_MediaServerDevices(This, value) \
    ((This)->lpVtbl->get_MediaServerDevices(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIKnownFoldersStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIKnownFoldersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IKnownFoldersStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.KnownFolders
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIKnownFoldersStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIKnownFoldersStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IKnownFoldersStatics2[] = L"Windows.Storage.IKnownFoldersStatics2";
typedef struct __x_ABI_CWindows_CStorage_CIKnownFoldersStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Objects3D)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics2* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder** value);
    HRESULT (STDMETHODCALLTYPE* get_AppCaptures)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics2* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder** value);
    HRESULT (STDMETHODCALLTYPE* get_RecordedCalls)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics2* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder** value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIKnownFoldersStatics2Vtbl;

interface __x_ABI_CWindows_CStorage_CIKnownFoldersStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIKnownFoldersStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics2_get_Objects3D(This, value) \
    ((This)->lpVtbl->get_Objects3D(This, value))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics2_get_AppCaptures(This, value) \
    ((This)->lpVtbl->get_AppCaptures(This, value))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics2_get_RecordedCalls(This, value) \
    ((This)->lpVtbl->get_RecordedCalls(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIKnownFoldersStatics2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIKnownFoldersStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IKnownFoldersStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Storage.KnownFolders
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CStorage_CIKnownFoldersStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIKnownFoldersStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IKnownFoldersStatics3[] = L"Windows.Storage.IKnownFoldersStatics3";
typedef struct __x_ABI_CWindows_CStorage_CIKnownFoldersStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetFolderForUserAsync)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics3* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        enum __x_ABI_CWindows_CStorage_CKnownFolderId folderId,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFolder** operation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIKnownFoldersStatics3Vtbl;

interface __x_ABI_CWindows_CStorage_CIKnownFoldersStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIKnownFoldersStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics3_GetFolderForUserAsync(This, user, folderId, operation) \
    ((This)->lpVtbl->GetFolderForUserAsync(This, user, folderId, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIKnownFoldersStatics3;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIKnownFoldersStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Storage.IKnownFoldersStatics4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Storage.KnownFolders
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CStorage_CIKnownFoldersStatics4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIKnownFoldersStatics4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IKnownFoldersStatics4[] = L"Windows.Storage.IKnownFoldersStatics4";
typedef struct __x_ABI_CWindows_CStorage_CIKnownFoldersStatics4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RequestAccessAsync)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics4* This,
        enum __x_ABI_CWindows_CStorage_CKnownFolderId folderId,
        __FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatus** operation);
    HRESULT (STDMETHODCALLTYPE* RequestAccessForUserAsync)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics4* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        enum __x_ABI_CWindows_CStorage_CKnownFolderId folderId,
        __FIAsyncOperation_1_Windows__CStorage__CKnownFoldersAccessStatus** operation);
    HRESULT (STDMETHODCALLTYPE* GetFolderAsync)(__x_ABI_CWindows_CStorage_CIKnownFoldersStatics4* This,
        enum __x_ABI_CWindows_CStorage_CKnownFolderId folderId,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFolder** operation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIKnownFoldersStatics4Vtbl;

interface __x_ABI_CWindows_CStorage_CIKnownFoldersStatics4
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIKnownFoldersStatics4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics4_RequestAccessAsync(This, folderId, operation) \
    ((This)->lpVtbl->RequestAccessAsync(This, folderId, operation))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics4_RequestAccessForUserAsync(This, user, folderId, operation) \
    ((This)->lpVtbl->RequestAccessForUserAsync(This, user, folderId, operation))

#define __x_ABI_CWindows_CStorage_CIKnownFoldersStatics4_GetFolderAsync(This, folderId, operation) \
    ((This)->lpVtbl->GetFolderAsync(This, folderId, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIKnownFoldersStatics4;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIKnownFoldersStatics4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Storage.IPathIOStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.PathIO
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIPathIOStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIPathIOStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IPathIOStatics[] = L"Windows.Storage.IPathIOStatics";
typedef struct __x_ABI_CWindows_CStorage_CIPathIOStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIPathIOStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIPathIOStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIPathIOStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIPathIOStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIPathIOStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIPathIOStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReadTextAsync)(__x_ABI_CWindows_CStorage_CIPathIOStatics* This,
        HSTRING absolutePath,
        __FIAsyncOperation_1_HSTRING** textOperation);
    HRESULT (STDMETHODCALLTYPE* ReadTextWithEncodingAsync)(__x_ABI_CWindows_CStorage_CIPathIOStatics* This,
        HSTRING absolutePath,
        enum __x_ABI_CWindows_CStorage_CStreams_CUnicodeEncoding encoding,
        __FIAsyncOperation_1_HSTRING** textOperation);
    HRESULT (STDMETHODCALLTYPE* WriteTextAsync)(__x_ABI_CWindows_CStorage_CIPathIOStatics* This,
        HSTRING absolutePath,
        HSTRING contents,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** textOperation);
    HRESULT (STDMETHODCALLTYPE* WriteTextWithEncodingAsync)(__x_ABI_CWindows_CStorage_CIPathIOStatics* This,
        HSTRING absolutePath,
        HSTRING contents,
        enum __x_ABI_CWindows_CStorage_CStreams_CUnicodeEncoding encoding,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** textOperation);
    HRESULT (STDMETHODCALLTYPE* AppendTextAsync)(__x_ABI_CWindows_CStorage_CIPathIOStatics* This,
        HSTRING absolutePath,
        HSTRING contents,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** textOperation);
    HRESULT (STDMETHODCALLTYPE* AppendTextWithEncodingAsync)(__x_ABI_CWindows_CStorage_CIPathIOStatics* This,
        HSTRING absolutePath,
        HSTRING contents,
        enum __x_ABI_CWindows_CStorage_CStreams_CUnicodeEncoding encoding,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** textOperation);
    HRESULT (STDMETHODCALLTYPE* ReadLinesAsync)(__x_ABI_CWindows_CStorage_CIPathIOStatics* This,
        HSTRING absolutePath,
        __FIAsyncOperation_1___FIVector_1_HSTRING** linesOperation);
    HRESULT (STDMETHODCALLTYPE* ReadLinesWithEncodingAsync)(__x_ABI_CWindows_CStorage_CIPathIOStatics* This,
        HSTRING absolutePath,
        enum __x_ABI_CWindows_CStorage_CStreams_CUnicodeEncoding encoding,
        __FIAsyncOperation_1___FIVector_1_HSTRING** linesOperation);
    HRESULT (STDMETHODCALLTYPE* WriteLinesAsync)(__x_ABI_CWindows_CStorage_CIPathIOStatics* This,
        HSTRING absolutePath,
        __FIIterable_1_HSTRING* lines,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* WriteLinesWithEncodingAsync)(__x_ABI_CWindows_CStorage_CIPathIOStatics* This,
        HSTRING absolutePath,
        __FIIterable_1_HSTRING* lines,
        enum __x_ABI_CWindows_CStorage_CStreams_CUnicodeEncoding encoding,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* AppendLinesAsync)(__x_ABI_CWindows_CStorage_CIPathIOStatics* This,
        HSTRING absolutePath,
        __FIIterable_1_HSTRING* lines,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* AppendLinesWithEncodingAsync)(__x_ABI_CWindows_CStorage_CIPathIOStatics* This,
        HSTRING absolutePath,
        __FIIterable_1_HSTRING* lines,
        enum __x_ABI_CWindows_CStorage_CStreams_CUnicodeEncoding encoding,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* ReadBufferAsync)(__x_ABI_CWindows_CStorage_CIPathIOStatics* This,
        HSTRING absolutePath,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer** operation);
    HRESULT (STDMETHODCALLTYPE* WriteBufferAsync)(__x_ABI_CWindows_CStorage_CIPathIOStatics* This,
        HSTRING absolutePath,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* buffer,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* WriteBytesAsync)(__x_ABI_CWindows_CStorage_CIPathIOStatics* This,
        HSTRING absolutePath,
        UINT32 bufferLength,
        BYTE* buffer,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIPathIOStaticsVtbl;

interface __x_ABI_CWindows_CStorage_CIPathIOStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIPathIOStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIPathIOStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIPathIOStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIPathIOStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIPathIOStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIPathIOStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIPathIOStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIPathIOStatics_ReadTextAsync(This, absolutePath, textOperation) \
    ((This)->lpVtbl->ReadTextAsync(This, absolutePath, textOperation))

#define __x_ABI_CWindows_CStorage_CIPathIOStatics_ReadTextWithEncodingAsync(This, absolutePath, encoding, textOperation) \
    ((This)->lpVtbl->ReadTextWithEncodingAsync(This, absolutePath, encoding, textOperation))

#define __x_ABI_CWindows_CStorage_CIPathIOStatics_WriteTextAsync(This, absolutePath, contents, textOperation) \
    ((This)->lpVtbl->WriteTextAsync(This, absolutePath, contents, textOperation))

#define __x_ABI_CWindows_CStorage_CIPathIOStatics_WriteTextWithEncodingAsync(This, absolutePath, contents, encoding, textOperation) \
    ((This)->lpVtbl->WriteTextWithEncodingAsync(This, absolutePath, contents, encoding, textOperation))

#define __x_ABI_CWindows_CStorage_CIPathIOStatics_AppendTextAsync(This, absolutePath, contents, textOperation) \
    ((This)->lpVtbl->AppendTextAsync(This, absolutePath, contents, textOperation))

#define __x_ABI_CWindows_CStorage_CIPathIOStatics_AppendTextWithEncodingAsync(This, absolutePath, contents, encoding, textOperation) \
    ((This)->lpVtbl->AppendTextWithEncodingAsync(This, absolutePath, contents, encoding, textOperation))

#define __x_ABI_CWindows_CStorage_CIPathIOStatics_ReadLinesAsync(This, absolutePath, linesOperation) \
    ((This)->lpVtbl->ReadLinesAsync(This, absolutePath, linesOperation))

#define __x_ABI_CWindows_CStorage_CIPathIOStatics_ReadLinesWithEncodingAsync(This, absolutePath, encoding, linesOperation) \
    ((This)->lpVtbl->ReadLinesWithEncodingAsync(This, absolutePath, encoding, linesOperation))

#define __x_ABI_CWindows_CStorage_CIPathIOStatics_WriteLinesAsync(This, absolutePath, lines, operation) \
    ((This)->lpVtbl->WriteLinesAsync(This, absolutePath, lines, operation))

#define __x_ABI_CWindows_CStorage_CIPathIOStatics_WriteLinesWithEncodingAsync(This, absolutePath, lines, encoding, operation) \
    ((This)->lpVtbl->WriteLinesWithEncodingAsync(This, absolutePath, lines, encoding, operation))

#define __x_ABI_CWindows_CStorage_CIPathIOStatics_AppendLinesAsync(This, absolutePath, lines, operation) \
    ((This)->lpVtbl->AppendLinesAsync(This, absolutePath, lines, operation))

#define __x_ABI_CWindows_CStorage_CIPathIOStatics_AppendLinesWithEncodingAsync(This, absolutePath, lines, encoding, operation) \
    ((This)->lpVtbl->AppendLinesWithEncodingAsync(This, absolutePath, lines, encoding, operation))

#define __x_ABI_CWindows_CStorage_CIPathIOStatics_ReadBufferAsync(This, absolutePath, operation) \
    ((This)->lpVtbl->ReadBufferAsync(This, absolutePath, operation))

#define __x_ABI_CWindows_CStorage_CIPathIOStatics_WriteBufferAsync(This, absolutePath, buffer, operation) \
    ((This)->lpVtbl->WriteBufferAsync(This, absolutePath, buffer, operation))

#define __x_ABI_CWindows_CStorage_CIPathIOStatics_WriteBytesAsync(This, absolutePath, bufferLength, buffer, operation) \
    ((This)->lpVtbl->WriteBytesAsync(This, absolutePath, bufferLength, buffer, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIPathIOStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIPathIOStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.ISetVersionDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.SetVersionDeferral
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CISetVersionDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CISetVersionDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_ISetVersionDeferral[] = L"Windows.Storage.ISetVersionDeferral";
typedef struct __x_ABI_CWindows_CStorage_CISetVersionDeferralVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CISetVersionDeferral* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CISetVersionDeferral* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CISetVersionDeferral* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CISetVersionDeferral* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CISetVersionDeferral* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CISetVersionDeferral* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Complete)(__x_ABI_CWindows_CStorage_CISetVersionDeferral* This);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CISetVersionDeferralVtbl;

interface __x_ABI_CWindows_CStorage_CISetVersionDeferral
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CISetVersionDeferralVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CISetVersionDeferral_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CISetVersionDeferral_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CISetVersionDeferral_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CISetVersionDeferral_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CISetVersionDeferral_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CISetVersionDeferral_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CISetVersionDeferral_Complete(This) \
    ((This)->lpVtbl->Complete(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CISetVersionDeferral;
#endif /* !defined(____x_ABI_CWindows_CStorage_CISetVersionDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.ISetVersionRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.SetVersionRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CISetVersionRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CISetVersionRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_ISetVersionRequest[] = L"Windows.Storage.ISetVersionRequest";
typedef struct __x_ABI_CWindows_CStorage_CISetVersionRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CISetVersionRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CISetVersionRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CISetVersionRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CISetVersionRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CISetVersionRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CISetVersionRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CurrentVersion)(__x_ABI_CWindows_CStorage_CISetVersionRequest* This,
        UINT32* currentVersion);
    HRESULT (STDMETHODCALLTYPE* get_DesiredVersion)(__x_ABI_CWindows_CStorage_CISetVersionRequest* This,
        UINT32* desiredVersion);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CStorage_CISetVersionRequest* This,
        __x_ABI_CWindows_CStorage_CISetVersionDeferral** deferral);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CISetVersionRequestVtbl;

interface __x_ABI_CWindows_CStorage_CISetVersionRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CISetVersionRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CISetVersionRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CISetVersionRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CISetVersionRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CISetVersionRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CISetVersionRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CISetVersionRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CISetVersionRequest_get_CurrentVersion(This, currentVersion) \
    ((This)->lpVtbl->get_CurrentVersion(This, currentVersion))

#define __x_ABI_CWindows_CStorage_CISetVersionRequest_get_DesiredVersion(This, desiredVersion) \
    ((This)->lpVtbl->get_DesiredVersion(This, desiredVersion))

#define __x_ABI_CWindows_CStorage_CISetVersionRequest_GetDeferral(This, deferral) \
    ((This)->lpVtbl->GetDeferral(This, deferral))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CISetVersionRequest;
#endif /* !defined(____x_ABI_CWindows_CStorage_CISetVersionRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IStorageFile
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Storage.IStorageItem
 *     Windows.Storage.Streams.IRandomAccessStreamReference
 *     Windows.Storage.Streams.IInputStreamReference
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageFile_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageFile_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageFile[] = L"Windows.Storage.IStorageFile";
typedef struct __x_ABI_CWindows_CStorage_CIStorageFileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIStorageFile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIStorageFile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIStorageFile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIStorageFile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIStorageFile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIStorageFile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FileType)(__x_ABI_CWindows_CStorage_CIStorageFile* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ContentType)(__x_ABI_CWindows_CStorage_CIStorageFile* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* OpenAsync)(__x_ABI_CWindows_CStorage_CIStorageFile* This,
        enum __x_ABI_CWindows_CStorage_CFileAccessMode accessMode,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream** operation);
    HRESULT (STDMETHODCALLTYPE* OpenTransactedWriteAsync)(__x_ABI_CWindows_CStorage_CIStorageFile* This,
        __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction** operation);
    HRESULT (STDMETHODCALLTYPE* CopyOverloadDefaultNameAndOptions)(__x_ABI_CWindows_CStorage_CIStorageFile* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder* destinationFolder,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation);
    HRESULT (STDMETHODCALLTYPE* CopyOverloadDefaultOptions)(__x_ABI_CWindows_CStorage_CIStorageFile* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder* destinationFolder,
        HSTRING desiredNewName,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation);
    HRESULT (STDMETHODCALLTYPE* CopyOverload)(__x_ABI_CWindows_CStorage_CIStorageFile* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder* destinationFolder,
        HSTRING desiredNewName,
        enum __x_ABI_CWindows_CStorage_CNameCollisionOption option,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation);
    HRESULT (STDMETHODCALLTYPE* CopyAndReplaceAsync)(__x_ABI_CWindows_CStorage_CIStorageFile* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* fileToReplace,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* MoveOverloadDefaultNameAndOptions)(__x_ABI_CWindows_CStorage_CIStorageFile* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder* destinationFolder,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* MoveOverloadDefaultOptions)(__x_ABI_CWindows_CStorage_CIStorageFile* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder* destinationFolder,
        HSTRING desiredNewName,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* MoveOverload)(__x_ABI_CWindows_CStorage_CIStorageFile* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder* destinationFolder,
        HSTRING desiredNewName,
        enum __x_ABI_CWindows_CStorage_CNameCollisionOption option,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* MoveAndReplaceAsync)(__x_ABI_CWindows_CStorage_CIStorageFile* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* fileToReplace,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIStorageFileVtbl;

interface __x_ABI_CWindows_CStorage_CIStorageFile
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIStorageFileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIStorageFile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIStorageFile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIStorageFile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIStorageFile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIStorageFile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIStorageFile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIStorageFile_get_FileType(This, value) \
    ((This)->lpVtbl->get_FileType(This, value))

#define __x_ABI_CWindows_CStorage_CIStorageFile_get_ContentType(This, value) \
    ((This)->lpVtbl->get_ContentType(This, value))

#define __x_ABI_CWindows_CStorage_CIStorageFile_OpenAsync(This, accessMode, operation) \
    ((This)->lpVtbl->OpenAsync(This, accessMode, operation))

#define __x_ABI_CWindows_CStorage_CIStorageFile_OpenTransactedWriteAsync(This, operation) \
    ((This)->lpVtbl->OpenTransactedWriteAsync(This, operation))

#define __x_ABI_CWindows_CStorage_CIStorageFile_CopyOverloadDefaultNameAndOptions(This, destinationFolder, operation) \
    ((This)->lpVtbl->CopyOverloadDefaultNameAndOptions(This, destinationFolder, operation))

#define __x_ABI_CWindows_CStorage_CIStorageFile_CopyOverloadDefaultOptions(This, destinationFolder, desiredNewName, operation) \
    ((This)->lpVtbl->CopyOverloadDefaultOptions(This, destinationFolder, desiredNewName, operation))

#define __x_ABI_CWindows_CStorage_CIStorageFile_CopyOverload(This, destinationFolder, desiredNewName, option, operation) \
    ((This)->lpVtbl->CopyOverload(This, destinationFolder, desiredNewName, option, operation))

#define __x_ABI_CWindows_CStorage_CIStorageFile_CopyAndReplaceAsync(This, fileToReplace, operation) \
    ((This)->lpVtbl->CopyAndReplaceAsync(This, fileToReplace, operation))

#define __x_ABI_CWindows_CStorage_CIStorageFile_MoveOverloadDefaultNameAndOptions(This, destinationFolder, operation) \
    ((This)->lpVtbl->MoveOverloadDefaultNameAndOptions(This, destinationFolder, operation))

#define __x_ABI_CWindows_CStorage_CIStorageFile_MoveOverloadDefaultOptions(This, destinationFolder, desiredNewName, operation) \
    ((This)->lpVtbl->MoveOverloadDefaultOptions(This, destinationFolder, desiredNewName, operation))

#define __x_ABI_CWindows_CStorage_CIStorageFile_MoveOverload(This, destinationFolder, desiredNewName, option, operation) \
    ((This)->lpVtbl->MoveOverload(This, destinationFolder, desiredNewName, option, operation))

#define __x_ABI_CWindows_CStorage_CIStorageFile_MoveAndReplaceAsync(This, fileToReplace, operation) \
    ((This)->lpVtbl->MoveAndReplaceAsync(This, fileToReplace, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageFile;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageFile_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IStorageFile2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageFile2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageFile2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageFile2[] = L"Windows.Storage.IStorageFile2";
typedef struct __x_ABI_CWindows_CStorage_CIStorageFile2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIStorageFile2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIStorageFile2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIStorageFile2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIStorageFile2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIStorageFile2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIStorageFile2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* OpenWithOptionsAsync)(__x_ABI_CWindows_CStorage_CIStorageFile2* This,
        enum __x_ABI_CWindows_CStorage_CFileAccessMode accessMode,
        enum __x_ABI_CWindows_CStorage_CStorageOpenOptions options,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream** operation);
    HRESULT (STDMETHODCALLTYPE* OpenTransactedWriteWithOptionsAsync)(__x_ABI_CWindows_CStorage_CIStorageFile2* This,
        enum __x_ABI_CWindows_CStorage_CStorageOpenOptions options,
        __FIAsyncOperation_1_Windows__CStorage__CStorageStreamTransaction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIStorageFile2Vtbl;

interface __x_ABI_CWindows_CStorage_CIStorageFile2
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIStorageFile2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIStorageFile2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIStorageFile2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIStorageFile2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIStorageFile2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIStorageFile2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIStorageFile2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIStorageFile2_OpenWithOptionsAsync(This, accessMode, options, operation) \
    ((This)->lpVtbl->OpenWithOptionsAsync(This, accessMode, options, operation))

#define __x_ABI_CWindows_CStorage_CIStorageFile2_OpenTransactedWriteWithOptionsAsync(This, options, operation) \
    ((This)->lpVtbl->OpenTransactedWriteWithOptionsAsync(This, options, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageFile2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageFile2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IStorageFilePropertiesWithAvailability
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageFilePropertiesWithAvailability[] = L"Windows.Storage.IStorageFilePropertiesWithAvailability";
typedef struct __x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailabilityVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsAvailable)(__x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailabilityVtbl;

interface __x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailabilityVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability_get_IsAvailable(This, value) \
    ((This)->lpVtbl->get_IsAvailable(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageFilePropertiesWithAvailability_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IStorageFileStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageFile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageFileStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageFileStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageFileStatics[] = L"Windows.Storage.IStorageFileStatics";
typedef struct __x_ABI_CWindows_CStorage_CIStorageFileStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIStorageFileStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIStorageFileStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIStorageFileStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIStorageFileStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIStorageFileStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIStorageFileStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetFileFromPathAsync)(__x_ABI_CWindows_CStorage_CIStorageFileStatics* This,
        HSTRING path,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation);
    HRESULT (STDMETHODCALLTYPE* GetFileFromApplicationUriAsync)(__x_ABI_CWindows_CStorage_CIStorageFileStatics* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation);
    HRESULT (STDMETHODCALLTYPE* CreateStreamedFileAsync)(__x_ABI_CWindows_CStorage_CIStorageFileStatics* This,
        HSTRING displayNameWithExtension,
        __x_ABI_CWindows_CStorage_CIStreamedFileDataRequestedHandler* dataRequested,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* thumbnail,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation);
    HRESULT (STDMETHODCALLTYPE* ReplaceWithStreamedFileAsync)(__x_ABI_CWindows_CStorage_CIStorageFileStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* fileToReplace,
        __x_ABI_CWindows_CStorage_CIStreamedFileDataRequestedHandler* dataRequested,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* thumbnail,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation);
    HRESULT (STDMETHODCALLTYPE* CreateStreamedFileFromUriAsync)(__x_ABI_CWindows_CStorage_CIStorageFileStatics* This,
        HSTRING displayNameWithExtension,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* thumbnail,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation);
    HRESULT (STDMETHODCALLTYPE* ReplaceWithStreamedFileFromUriAsync)(__x_ABI_CWindows_CStorage_CIStorageFileStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* fileToReplace,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* thumbnail,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIStorageFileStaticsVtbl;

interface __x_ABI_CWindows_CStorage_CIStorageFileStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIStorageFileStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIStorageFileStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIStorageFileStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIStorageFileStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIStorageFileStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIStorageFileStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIStorageFileStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIStorageFileStatics_GetFileFromPathAsync(This, path, operation) \
    ((This)->lpVtbl->GetFileFromPathAsync(This, path, operation))

#define __x_ABI_CWindows_CStorage_CIStorageFileStatics_GetFileFromApplicationUriAsync(This, uri, operation) \
    ((This)->lpVtbl->GetFileFromApplicationUriAsync(This, uri, operation))

#define __x_ABI_CWindows_CStorage_CIStorageFileStatics_CreateStreamedFileAsync(This, displayNameWithExtension, dataRequested, thumbnail, operation) \
    ((This)->lpVtbl->CreateStreamedFileAsync(This, displayNameWithExtension, dataRequested, thumbnail, operation))

#define __x_ABI_CWindows_CStorage_CIStorageFileStatics_ReplaceWithStreamedFileAsync(This, fileToReplace, dataRequested, thumbnail, operation) \
    ((This)->lpVtbl->ReplaceWithStreamedFileAsync(This, fileToReplace, dataRequested, thumbnail, operation))

#define __x_ABI_CWindows_CStorage_CIStorageFileStatics_CreateStreamedFileFromUriAsync(This, displayNameWithExtension, uri, thumbnail, operation) \
    ((This)->lpVtbl->CreateStreamedFileFromUriAsync(This, displayNameWithExtension, uri, thumbnail, operation))

#define __x_ABI_CWindows_CStorage_CIStorageFileStatics_ReplaceWithStreamedFileFromUriAsync(This, fileToReplace, uri, thumbnail, operation) \
    ((This)->lpVtbl->ReplaceWithStreamedFileFromUriAsync(This, fileToReplace, uri, thumbnail, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageFileStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageFileStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IStorageFileStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageFile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageFileStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageFileStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageFileStatics2[] = L"Windows.Storage.IStorageFileStatics2";
typedef struct __x_ABI_CWindows_CStorage_CIStorageFileStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIStorageFileStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIStorageFileStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIStorageFileStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIStorageFileStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIStorageFileStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIStorageFileStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetFileFromPathForUserAsync)(__x_ABI_CWindows_CStorage_CIStorageFileStatics2* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING path,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIStorageFileStatics2Vtbl;

interface __x_ABI_CWindows_CStorage_CIStorageFileStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIStorageFileStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIStorageFileStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIStorageFileStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIStorageFileStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIStorageFileStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIStorageFileStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIStorageFileStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIStorageFileStatics2_GetFileFromPathForUserAsync(This, user, path, operation) \
    ((This)->lpVtbl->GetFileFromPathForUserAsync(This, user, path, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageFileStatics2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageFileStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Storage.IStorageFolder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Storage.IStorageItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageFolder_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageFolder_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageFolder[] = L"Windows.Storage.IStorageFolder";
typedef struct __x_ABI_CWindows_CStorage_CIStorageFolderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIStorageFolder* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIStorageFolder* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIStorageFolder* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIStorageFolder* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIStorageFolder* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIStorageFolder* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFileAsyncOverloadDefaultOptions)(__x_ABI_CWindows_CStorage_CIStorageFolder* This,
        HSTRING desiredName,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation);
    HRESULT (STDMETHODCALLTYPE* CreateFileAsync)(__x_ABI_CWindows_CStorage_CIStorageFolder* This,
        HSTRING desiredName,
        enum __x_ABI_CWindows_CStorage_CCreationCollisionOption options,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation);
    HRESULT (STDMETHODCALLTYPE* CreateFolderAsyncOverloadDefaultOptions)(__x_ABI_CWindows_CStorage_CIStorageFolder* This,
        HSTRING desiredName,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFolder** operation);
    HRESULT (STDMETHODCALLTYPE* CreateFolderAsync)(__x_ABI_CWindows_CStorage_CIStorageFolder* This,
        HSTRING desiredName,
        enum __x_ABI_CWindows_CStorage_CCreationCollisionOption options,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFolder** operation);
    HRESULT (STDMETHODCALLTYPE* GetFileAsync)(__x_ABI_CWindows_CStorage_CIStorageFolder* This,
        HSTRING name,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFile** operation);
    HRESULT (STDMETHODCALLTYPE* GetFolderAsync)(__x_ABI_CWindows_CStorage_CIStorageFolder* This,
        HSTRING name,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFolder** operation);
    HRESULT (STDMETHODCALLTYPE* GetItemAsync)(__x_ABI_CWindows_CStorage_CIStorageFolder* This,
        HSTRING name,
        __FIAsyncOperation_1_Windows__CStorage__CIStorageItem** operation);
    HRESULT (STDMETHODCALLTYPE* GetFilesAsyncOverloadDefaultOptionsStartAndCount)(__x_ABI_CWindows_CStorage_CIStorageFolder* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFile** operation);
    HRESULT (STDMETHODCALLTYPE* GetFoldersAsyncOverloadDefaultOptionsStartAndCount)(__x_ABI_CWindows_CStorage_CIStorageFolder* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageFolder** operation);
    HRESULT (STDMETHODCALLTYPE* GetItemsAsyncOverloadDefaultStartAndCount)(__x_ABI_CWindows_CStorage_CIStorageFolder* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CIStorageItem** operation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIStorageFolderVtbl;

interface __x_ABI_CWindows_CStorage_CIStorageFolder
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIStorageFolderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIStorageFolder_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIStorageFolder_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIStorageFolder_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIStorageFolder_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIStorageFolder_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIStorageFolder_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIStorageFolder_CreateFileAsyncOverloadDefaultOptions(This, desiredName, operation) \
    ((This)->lpVtbl->CreateFileAsyncOverloadDefaultOptions(This, desiredName, operation))

#define __x_ABI_CWindows_CStorage_CIStorageFolder_CreateFileAsync(This, desiredName, options, operation) \
    ((This)->lpVtbl->CreateFileAsync(This, desiredName, options, operation))

#define __x_ABI_CWindows_CStorage_CIStorageFolder_CreateFolderAsyncOverloadDefaultOptions(This, desiredName, operation) \
    ((This)->lpVtbl->CreateFolderAsyncOverloadDefaultOptions(This, desiredName, operation))

#define __x_ABI_CWindows_CStorage_CIStorageFolder_CreateFolderAsync(This, desiredName, options, operation) \
    ((This)->lpVtbl->CreateFolderAsync(This, desiredName, options, operation))

#define __x_ABI_CWindows_CStorage_CIStorageFolder_GetFileAsync(This, name, operation) \
    ((This)->lpVtbl->GetFileAsync(This, name, operation))

#define __x_ABI_CWindows_CStorage_CIStorageFolder_GetFolderAsync(This, name, operation) \
    ((This)->lpVtbl->GetFolderAsync(This, name, operation))

#define __x_ABI_CWindows_CStorage_CIStorageFolder_GetItemAsync(This, name, operation) \
    ((This)->lpVtbl->GetItemAsync(This, name, operation))

#define __x_ABI_CWindows_CStorage_CIStorageFolder_GetFilesAsyncOverloadDefaultOptionsStartAndCount(This, operation) \
    ((This)->lpVtbl->GetFilesAsyncOverloadDefaultOptionsStartAndCount(This, operation))

#define __x_ABI_CWindows_CStorage_CIStorageFolder_GetFoldersAsyncOverloadDefaultOptionsStartAndCount(This, operation) \
    ((This)->lpVtbl->GetFoldersAsyncOverloadDefaultOptionsStartAndCount(This, operation))

#define __x_ABI_CWindows_CStorage_CIStorageFolder_GetItemsAsyncOverloadDefaultStartAndCount(This, operation) \
    ((This)->lpVtbl->GetItemsAsyncOverloadDefaultStartAndCount(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageFolder;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageFolder_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IStorageFolder2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageFolder2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageFolder2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageFolder2[] = L"Windows.Storage.IStorageFolder2";
typedef struct __x_ABI_CWindows_CStorage_CIStorageFolder2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIStorageFolder2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIStorageFolder2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIStorageFolder2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIStorageFolder2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIStorageFolder2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIStorageFolder2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryGetItemAsync)(__x_ABI_CWindows_CStorage_CIStorageFolder2* This,
        HSTRING name,
        __FIAsyncOperation_1_Windows__CStorage__CIStorageItem** operation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIStorageFolder2Vtbl;

interface __x_ABI_CWindows_CStorage_CIStorageFolder2
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIStorageFolder2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIStorageFolder2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIStorageFolder2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIStorageFolder2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIStorageFolder2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIStorageFolder2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIStorageFolder2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIStorageFolder2_TryGetItemAsync(This, name, operation) \
    ((This)->lpVtbl->TryGetItemAsync(This, name, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageFolder2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageFolder2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IStorageFolder3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageFolder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageFolder3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageFolder3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageFolder3[] = L"Windows.Storage.IStorageFolder3";
typedef struct __x_ABI_CWindows_CStorage_CIStorageFolder3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIStorageFolder3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIStorageFolder3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIStorageFolder3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIStorageFolder3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIStorageFolder3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIStorageFolder3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryGetChangeTracker)(__x_ABI_CWindows_CStorage_CIStorageFolder3* This,
        __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker** result);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIStorageFolder3Vtbl;

interface __x_ABI_CWindows_CStorage_CIStorageFolder3
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIStorageFolder3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIStorageFolder3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIStorageFolder3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIStorageFolder3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIStorageFolder3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIStorageFolder3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIStorageFolder3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIStorageFolder3_TryGetChangeTracker(This, result) \
    ((This)->lpVtbl->TryGetChangeTracker(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageFolder3;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageFolder3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Storage.IStorageFolderStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageFolder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageFolderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageFolderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageFolderStatics[] = L"Windows.Storage.IStorageFolderStatics";
typedef struct __x_ABI_CWindows_CStorage_CIStorageFolderStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIStorageFolderStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIStorageFolderStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIStorageFolderStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIStorageFolderStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIStorageFolderStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIStorageFolderStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetFolderFromPathAsync)(__x_ABI_CWindows_CStorage_CIStorageFolderStatics* This,
        HSTRING path,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFolder** operation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIStorageFolderStaticsVtbl;

interface __x_ABI_CWindows_CStorage_CIStorageFolderStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIStorageFolderStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIStorageFolderStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIStorageFolderStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIStorageFolderStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIStorageFolderStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIStorageFolderStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIStorageFolderStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIStorageFolderStatics_GetFolderFromPathAsync(This, path, operation) \
    ((This)->lpVtbl->GetFolderFromPathAsync(This, path, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageFolderStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageFolderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IStorageFolderStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageFolder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageFolderStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageFolderStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageFolderStatics2[] = L"Windows.Storage.IStorageFolderStatics2";
typedef struct __x_ABI_CWindows_CStorage_CIStorageFolderStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIStorageFolderStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIStorageFolderStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIStorageFolderStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIStorageFolderStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIStorageFolderStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIStorageFolderStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetFolderFromPathForUserAsync)(__x_ABI_CWindows_CStorage_CIStorageFolderStatics2* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING path,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFolder** operation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIStorageFolderStatics2Vtbl;

interface __x_ABI_CWindows_CStorage_CIStorageFolderStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIStorageFolderStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIStorageFolderStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIStorageFolderStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIStorageFolderStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIStorageFolderStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIStorageFolderStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIStorageFolderStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIStorageFolderStatics2_GetFolderFromPathForUserAsync(This, user, path, operation) \
    ((This)->lpVtbl->GetFolderFromPathForUserAsync(This, user, path, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageFolderStatics2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageFolderStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Storage.IStorageItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageItem[] = L"Windows.Storage.IStorageItem";
typedef struct __x_ABI_CWindows_CStorage_CIStorageItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIStorageItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIStorageItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIStorageItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIStorageItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIStorageItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIStorageItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RenameAsyncOverloadDefaultOptions)(__x_ABI_CWindows_CStorage_CIStorageItem* This,
        HSTRING desiredName,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* RenameAsync)(__x_ABI_CWindows_CStorage_CIStorageItem* This,
        HSTRING desiredName,
        enum __x_ABI_CWindows_CStorage_CNameCollisionOption option,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* DeleteAsyncOverloadDefaultOptions)(__x_ABI_CWindows_CStorage_CIStorageItem* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* DeleteAsync)(__x_ABI_CWindows_CStorage_CIStorageItem* This,
        enum __x_ABI_CWindows_CStorage_CStorageDeleteOption option,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* GetBasicPropertiesAsync)(__x_ABI_CWindows_CStorage_CIStorageItem* This,
        __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CBasicProperties** operation);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CStorage_CIStorageItem* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Path)(__x_ABI_CWindows_CStorage_CIStorageItem* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Attributes)(__x_ABI_CWindows_CStorage_CIStorageItem* This,
        enum __x_ABI_CWindows_CStorage_CFileAttributes* value);
    HRESULT (STDMETHODCALLTYPE* get_DateCreated)(__x_ABI_CWindows_CStorage_CIStorageItem* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* IsOfType)(__x_ABI_CWindows_CStorage_CIStorageItem* This,
        enum __x_ABI_CWindows_CStorage_CStorageItemTypes type,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIStorageItemVtbl;

interface __x_ABI_CWindows_CStorage_CIStorageItem
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIStorageItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIStorageItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIStorageItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIStorageItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIStorageItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIStorageItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIStorageItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIStorageItem_RenameAsyncOverloadDefaultOptions(This, desiredName, operation) \
    ((This)->lpVtbl->RenameAsyncOverloadDefaultOptions(This, desiredName, operation))

#define __x_ABI_CWindows_CStorage_CIStorageItem_RenameAsync(This, desiredName, option, operation) \
    ((This)->lpVtbl->RenameAsync(This, desiredName, option, operation))

#define __x_ABI_CWindows_CStorage_CIStorageItem_DeleteAsyncOverloadDefaultOptions(This, operation) \
    ((This)->lpVtbl->DeleteAsyncOverloadDefaultOptions(This, operation))

#define __x_ABI_CWindows_CStorage_CIStorageItem_DeleteAsync(This, option, operation) \
    ((This)->lpVtbl->DeleteAsync(This, option, operation))

#define __x_ABI_CWindows_CStorage_CIStorageItem_GetBasicPropertiesAsync(This, operation) \
    ((This)->lpVtbl->GetBasicPropertiesAsync(This, operation))

#define __x_ABI_CWindows_CStorage_CIStorageItem_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CStorage_CIStorageItem_get_Path(This, value) \
    ((This)->lpVtbl->get_Path(This, value))

#define __x_ABI_CWindows_CStorage_CIStorageItem_get_Attributes(This, value) \
    ((This)->lpVtbl->get_Attributes(This, value))

#define __x_ABI_CWindows_CStorage_CIStorageItem_get_DateCreated(This, value) \
    ((This)->lpVtbl->get_DateCreated(This, value))

#define __x_ABI_CWindows_CStorage_CIStorageItem_IsOfType(This, type, value) \
    ((This)->lpVtbl->IsOfType(This, type, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageItem;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IStorageItem2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Storage.IStorageItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageItem2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageItem2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageItem2[] = L"Windows.Storage.IStorageItem2";
typedef struct __x_ABI_CWindows_CStorage_CIStorageItem2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIStorageItem2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIStorageItem2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIStorageItem2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIStorageItem2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIStorageItem2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIStorageItem2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetParentAsync)(__x_ABI_CWindows_CStorage_CIStorageItem2* This,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFolder** operation);
    HRESULT (STDMETHODCALLTYPE* IsEqual)(__x_ABI_CWindows_CStorage_CIStorageItem2* This,
        __x_ABI_CWindows_CStorage_CIStorageItem* item,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIStorageItem2Vtbl;

interface __x_ABI_CWindows_CStorage_CIStorageItem2
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIStorageItem2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIStorageItem2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIStorageItem2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIStorageItem2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIStorageItem2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIStorageItem2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIStorageItem2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIStorageItem2_GetParentAsync(This, operation) \
    ((This)->lpVtbl->GetParentAsync(This, operation))

#define __x_ABI_CWindows_CStorage_CIStorageItem2_IsEqual(This, item, value) \
    ((This)->lpVtbl->IsEqual(This, item, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageItem2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageItem2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IStorageItemProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageItemProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageItemProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageItemProperties[] = L"Windows.Storage.IStorageItemProperties";
typedef struct __x_ABI_CWindows_CStorage_CIStorageItemPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIStorageItemProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIStorageItemProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIStorageItemProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIStorageItemProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIStorageItemProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIStorageItemProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetThumbnailAsyncOverloadDefaultSizeDefaultOptions)(__x_ABI_CWindows_CStorage_CIStorageItemProperties* This,
        enum __x_ABI_CWindows_CStorage_CFileProperties_CThumbnailMode mode,
        __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail** operation);
    HRESULT (STDMETHODCALLTYPE* GetThumbnailAsyncOverloadDefaultOptions)(__x_ABI_CWindows_CStorage_CIStorageItemProperties* This,
        enum __x_ABI_CWindows_CStorage_CFileProperties_CThumbnailMode mode,
        UINT32 requestedSize,
        __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail** operation);
    HRESULT (STDMETHODCALLTYPE* GetThumbnailAsync)(__x_ABI_CWindows_CStorage_CIStorageItemProperties* This,
        enum __x_ABI_CWindows_CStorage_CFileProperties_CThumbnailMode mode,
        UINT32 requestedSize,
        enum __x_ABI_CWindows_CStorage_CFileProperties_CThumbnailOptions options,
        __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail** operation);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CStorage_CIStorageItemProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayType)(__x_ABI_CWindows_CStorage_CIStorageItemProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_FolderRelativeId)(__x_ABI_CWindows_CStorage_CIStorageItemProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CStorage_CIStorageItemProperties* This,
        __x_ABI_CWindows_CStorage_CFileProperties_CIStorageItemContentProperties** value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIStorageItemPropertiesVtbl;

interface __x_ABI_CWindows_CStorage_CIStorageItemProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIStorageItemPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIStorageItemProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIStorageItemProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIStorageItemProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIStorageItemProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIStorageItemProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIStorageItemProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIStorageItemProperties_GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(This, mode, operation) \
    ((This)->lpVtbl->GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(This, mode, operation))

#define __x_ABI_CWindows_CStorage_CIStorageItemProperties_GetThumbnailAsyncOverloadDefaultOptions(This, mode, requestedSize, operation) \
    ((This)->lpVtbl->GetThumbnailAsyncOverloadDefaultOptions(This, mode, requestedSize, operation))

#define __x_ABI_CWindows_CStorage_CIStorageItemProperties_GetThumbnailAsync(This, mode, requestedSize, options, operation) \
    ((This)->lpVtbl->GetThumbnailAsync(This, mode, requestedSize, options, operation))

#define __x_ABI_CWindows_CStorage_CIStorageItemProperties_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CStorage_CIStorageItemProperties_get_DisplayType(This, value) \
    ((This)->lpVtbl->get_DisplayType(This, value))

#define __x_ABI_CWindows_CStorage_CIStorageItemProperties_get_FolderRelativeId(This, value) \
    ((This)->lpVtbl->get_FolderRelativeId(This, value))

#define __x_ABI_CWindows_CStorage_CIStorageItemProperties_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageItemProperties;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageItemProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IStorageItemProperties2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Storage.IStorageItemProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageItemProperties2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageItemProperties2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageItemProperties2[] = L"Windows.Storage.IStorageItemProperties2";
typedef struct __x_ABI_CWindows_CStorage_CIStorageItemProperties2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIStorageItemProperties2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIStorageItemProperties2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIStorageItemProperties2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIStorageItemProperties2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIStorageItemProperties2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIStorageItemProperties2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions)(__x_ABI_CWindows_CStorage_CIStorageItemProperties2* This,
        enum __x_ABI_CWindows_CStorage_CFileProperties_CThumbnailMode mode,
        __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail** operation);
    HRESULT (STDMETHODCALLTYPE* GetScaledImageAsThumbnailAsyncOverloadDefaultOptions)(__x_ABI_CWindows_CStorage_CIStorageItemProperties2* This,
        enum __x_ABI_CWindows_CStorage_CFileProperties_CThumbnailMode mode,
        UINT32 requestedSize,
        __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail** operation);
    HRESULT (STDMETHODCALLTYPE* GetScaledImageAsThumbnailAsync)(__x_ABI_CWindows_CStorage_CIStorageItemProperties2* This,
        enum __x_ABI_CWindows_CStorage_CFileProperties_CThumbnailMode mode,
        UINT32 requestedSize,
        enum __x_ABI_CWindows_CStorage_CFileProperties_CThumbnailOptions options,
        __FIAsyncOperation_1_Windows__CStorage__CFileProperties__CStorageItemThumbnail** operation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIStorageItemProperties2Vtbl;

interface __x_ABI_CWindows_CStorage_CIStorageItemProperties2
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIStorageItemProperties2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIStorageItemProperties2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIStorageItemProperties2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIStorageItemProperties2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIStorageItemProperties2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIStorageItemProperties2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIStorageItemProperties2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIStorageItemProperties2_GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions(This, mode, operation) \
    ((This)->lpVtbl->GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions(This, mode, operation))

#define __x_ABI_CWindows_CStorage_CIStorageItemProperties2_GetScaledImageAsThumbnailAsyncOverloadDefaultOptions(This, mode, requestedSize, operation) \
    ((This)->lpVtbl->GetScaledImageAsThumbnailAsyncOverloadDefaultOptions(This, mode, requestedSize, operation))

#define __x_ABI_CWindows_CStorage_CIStorageItemProperties2_GetScaledImageAsThumbnailAsync(This, mode, requestedSize, options, operation) \
    ((This)->lpVtbl->GetScaledImageAsThumbnailAsync(This, mode, requestedSize, options, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageItemProperties2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageItemProperties2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IStorageItemPropertiesWithProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Storage.IStorageItemProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageItemPropertiesWithProvider[] = L"Windows.Storage.IStorageItemPropertiesWithProvider";
typedef struct __x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Provider)(__x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider* This,
        __x_ABI_CWindows_CStorage_CIStorageProvider** value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProviderVtbl;

interface __x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider_get_Provider(This, value) \
    ((This)->lpVtbl->get_Provider(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageItemPropertiesWithProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IStorageLibrary
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageLibrary
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageLibrary_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageLibrary_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageLibrary[] = L"Windows.Storage.IStorageLibrary";
typedef struct __x_ABI_CWindows_CStorage_CIStorageLibraryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIStorageLibrary* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIStorageLibrary* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIStorageLibrary* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIStorageLibrary* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIStorageLibrary* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIStorageLibrary* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RequestAddFolderAsync)(__x_ABI_CWindows_CStorage_CIStorageLibrary* This,
        __FIAsyncOperation_1_Windows__CStorage__CStorageFolder** operation);
    HRESULT (STDMETHODCALLTYPE* RequestRemoveFolderAsync)(__x_ABI_CWindows_CStorage_CIStorageLibrary* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder* folder,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* get_Folders)(__x_ABI_CWindows_CStorage_CIStorageLibrary* This,
        __FIObservableVector_1_Windows__CStorage__CStorageFolder** value);
    HRESULT (STDMETHODCALLTYPE* get_SaveFolder)(__x_ABI_CWindows_CStorage_CIStorageLibrary* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder** value);
    HRESULT (STDMETHODCALLTYPE* add_DefinitionChanged)(__x_ABI_CWindows_CStorage_CIStorageLibrary* This,
        __FITypedEventHandler_2_Windows__CStorage__CStorageLibrary_IInspectable* handler,
        EventRegistrationToken* eventCookie);
    HRESULT (STDMETHODCALLTYPE* remove_DefinitionChanged)(__x_ABI_CWindows_CStorage_CIStorageLibrary* This,
        EventRegistrationToken eventCookie);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIStorageLibraryVtbl;

interface __x_ABI_CWindows_CStorage_CIStorageLibrary
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIStorageLibraryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIStorageLibrary_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIStorageLibrary_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIStorageLibrary_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIStorageLibrary_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIStorageLibrary_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIStorageLibrary_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIStorageLibrary_RequestAddFolderAsync(This, operation) \
    ((This)->lpVtbl->RequestAddFolderAsync(This, operation))

#define __x_ABI_CWindows_CStorage_CIStorageLibrary_RequestRemoveFolderAsync(This, folder, operation) \
    ((This)->lpVtbl->RequestRemoveFolderAsync(This, folder, operation))

#define __x_ABI_CWindows_CStorage_CIStorageLibrary_get_Folders(This, value) \
    ((This)->lpVtbl->get_Folders(This, value))

#define __x_ABI_CWindows_CStorage_CIStorageLibrary_get_SaveFolder(This, value) \
    ((This)->lpVtbl->get_SaveFolder(This, value))

#define __x_ABI_CWindows_CStorage_CIStorageLibrary_add_DefinitionChanged(This, handler, eventCookie) \
    ((This)->lpVtbl->add_DefinitionChanged(This, handler, eventCookie))

#define __x_ABI_CWindows_CStorage_CIStorageLibrary_remove_DefinitionChanged(This, eventCookie) \
    ((This)->lpVtbl->remove_DefinitionChanged(This, eventCookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageLibrary;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageLibrary_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IStorageLibrary2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageLibrary
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageLibrary2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageLibrary2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageLibrary2[] = L"Windows.Storage.IStorageLibrary2";
typedef struct __x_ABI_CWindows_CStorage_CIStorageLibrary2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIStorageLibrary2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIStorageLibrary2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIStorageLibrary2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIStorageLibrary2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIStorageLibrary2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIStorageLibrary2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ChangeTracker)(__x_ABI_CWindows_CStorage_CIStorageLibrary2* This,
        __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker** value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIStorageLibrary2Vtbl;

interface __x_ABI_CWindows_CStorage_CIStorageLibrary2
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIStorageLibrary2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIStorageLibrary2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIStorageLibrary2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIStorageLibrary2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIStorageLibrary2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIStorageLibrary2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIStorageLibrary2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIStorageLibrary2_get_ChangeTracker(This, value) \
    ((This)->lpVtbl->get_ChangeTracker(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageLibrary2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageLibrary2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Storage.IStorageLibrary3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageLibrary
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageLibrary3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageLibrary3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageLibrary3[] = L"Windows.Storage.IStorageLibrary3";
typedef struct __x_ABI_CWindows_CStorage_CIStorageLibrary3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIStorageLibrary3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIStorageLibrary3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIStorageLibrary3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIStorageLibrary3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIStorageLibrary3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIStorageLibrary3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* AreFolderSuggestionsAvailableAsync)(__x_ABI_CWindows_CStorage_CIStorageLibrary3* This,
        __FIAsyncOperation_1_boolean** operation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIStorageLibrary3Vtbl;

interface __x_ABI_CWindows_CStorage_CIStorageLibrary3
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIStorageLibrary3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIStorageLibrary3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIStorageLibrary3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIStorageLibrary3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIStorageLibrary3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIStorageLibrary3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIStorageLibrary3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIStorageLibrary3_AreFolderSuggestionsAvailableAsync(This, operation) \
    ((This)->lpVtbl->AreFolderSuggestionsAvailableAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageLibrary3;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageLibrary3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Storage.IStorageLibraryChange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageLibraryChange
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryChange_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryChange_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageLibraryChange[] = L"Windows.Storage.IStorageLibraryChange";
typedef struct __x_ABI_CWindows_CStorage_CIStorageLibraryChangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIStorageLibraryChange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIStorageLibraryChange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIStorageLibraryChange* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIStorageLibraryChange* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIStorageLibraryChange* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIStorageLibraryChange* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ChangeType)(__x_ABI_CWindows_CStorage_CIStorageLibraryChange* This,
        enum __x_ABI_CWindows_CStorage_CStorageLibraryChangeType* value);
    HRESULT (STDMETHODCALLTYPE* get_Path)(__x_ABI_CWindows_CStorage_CIStorageLibraryChange* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_PreviousPath)(__x_ABI_CWindows_CStorage_CIStorageLibraryChange* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* IsOfType)(__x_ABI_CWindows_CStorage_CIStorageLibraryChange* This,
        enum __x_ABI_CWindows_CStorage_CStorageItemTypes type,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* GetStorageItemAsync)(__x_ABI_CWindows_CStorage_CIStorageLibraryChange* This,
        __FIAsyncOperation_1_Windows__CStorage__CIStorageItem** operation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIStorageLibraryChangeVtbl;

interface __x_ABI_CWindows_CStorage_CIStorageLibraryChange
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIStorageLibraryChangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChange_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChange_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChange_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChange_get_ChangeType(This, value) \
    ((This)->lpVtbl->get_ChangeType(This, value))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChange_get_Path(This, value) \
    ((This)->lpVtbl->get_Path(This, value))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChange_get_PreviousPath(This, value) \
    ((This)->lpVtbl->get_PreviousPath(This, value))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChange_IsOfType(This, type, value) \
    ((This)->lpVtbl->IsOfType(This, type, value))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChange_GetStorageItemAsync(This, operation) \
    ((This)->lpVtbl->GetStorageItemAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageLibraryChange;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryChange_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Storage.IStorageLibraryChangeReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageLibraryChangeReader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageLibraryChangeReader[] = L"Windows.Storage.IStorageLibraryChangeReader";
typedef struct __x_ABI_CWindows_CStorage_CIStorageLibraryChangeReaderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReadBatchAsync)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CStorage__CStorageLibraryChange** operation);
    HRESULT (STDMETHODCALLTYPE* AcceptChangesAsync)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIStorageLibraryChangeReaderVtbl;

interface __x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIStorageLibraryChangeReaderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader_ReadBatchAsync(This, operation) \
    ((This)->lpVtbl->ReadBatchAsync(This, operation))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader_AcceptChangesAsync(This, operation) \
    ((This)->lpVtbl->AcceptChangesAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Storage.IStorageLibraryChangeReader2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageLibraryChangeReader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageLibraryChangeReader2[] = L"Windows.Storage.IStorageLibraryChangeReader2";
typedef struct __x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetLastChangeId)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader2* This,
        UINT64* result);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader2Vtbl;

interface __x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader2
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader2_GetLastChangeId(This, result) \
    ((This)->lpVtbl->GetLastChangeId(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

/*
 *
 * Interface Windows.Storage.IStorageLibraryChangeTracker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageLibraryChangeTracker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageLibraryChangeTracker[] = L"Windows.Storage.IStorageLibraryChangeTracker";
typedef struct __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetChangeReader)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker* This,
        __x_ABI_CWindows_CStorage_CIStorageLibraryChangeReader** value);
    HRESULT (STDMETHODCALLTYPE* Enable)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker* This);
    HRESULT (STDMETHODCALLTYPE* Reset)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker* This);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerVtbl;

interface __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker_GetChangeReader(This, value) \
    ((This)->lpVtbl->GetChangeReader(This, value))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker_Enable(This) \
    ((This)->lpVtbl->Enable(This))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker_Reset(This) \
    ((This)->lpVtbl->Reset(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Storage.IStorageLibraryChangeTracker2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageLibraryChangeTracker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageLibraryChangeTracker2[] = L"Windows.Storage.IStorageLibraryChangeTracker2";
typedef struct __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* EnableWithOptions)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker2* This,
        __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptions* options);
    HRESULT (STDMETHODCALLTYPE* Disable)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker2* This);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker2Vtbl;

interface __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker2
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker2_EnableWithOptions(This, options) \
    ((This)->lpVtbl->EnableWithOptions(This, options))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker2_Disable(This) \
    ((This)->lpVtbl->Disable(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryChangeTracker2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

/*
 *
 * Interface Windows.Storage.IStorageLibraryChangeTrackerOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageLibraryChangeTrackerOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageLibraryChangeTrackerOptions[] = L"Windows.Storage.IStorageLibraryChangeTrackerOptions";
typedef struct __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TrackChangeDetails)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptions* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_TrackChangeDetails)(__x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptions* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptionsVtbl;

interface __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptions_get_TrackChangeDetails(This, value) \
    ((This)->lpVtbl->get_TrackChangeDetails(This, value))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptions_put_TrackChangeDetails(This, value) \
    ((This)->lpVtbl->put_TrackChangeDetails(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptions;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryChangeTrackerOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

/*
 *
 * Interface Windows.Storage.IStorageLibraryLastChangeId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageLibraryLastChangeId
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeId_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeId_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageLibraryLastChangeId[] = L"Windows.Storage.IStorageLibraryLastChangeId";
typedef struct __x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeId* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeId* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeId* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeId* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeId* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeId* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdVtbl;

interface __x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeId
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeId_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeId_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeId_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeId_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeId_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeId_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeId;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeId_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

/*
 *
 * Interface Windows.Storage.IStorageLibraryLastChangeIdStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageLibraryLastChangeId
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageLibraryLastChangeIdStatics[] = L"Windows.Storage.IStorageLibraryLastChangeIdStatics";
typedef struct __x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Unknown)(__x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdStatics* This,
        UINT64* value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdStaticsVtbl;

interface __x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdStatics_get_Unknown(This, value) \
    ((This)->lpVtbl->get_Unknown(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryLastChangeIdStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

/*
 *
 * Interface Windows.Storage.IStorageLibraryStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageLibrary
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageLibraryStatics[] = L"Windows.Storage.IStorageLibraryStatics";
typedef struct __x_ABI_CWindows_CStorage_CIStorageLibraryStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIStorageLibraryStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIStorageLibraryStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIStorageLibraryStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIStorageLibraryStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIStorageLibraryStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIStorageLibraryStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetLibraryAsync)(__x_ABI_CWindows_CStorage_CIStorageLibraryStatics* This,
        enum __x_ABI_CWindows_CStorage_CKnownLibraryId libraryId,
        __FIAsyncOperation_1_Windows__CStorage__CStorageLibrary** operation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIStorageLibraryStaticsVtbl;

interface __x_ABI_CWindows_CStorage_CIStorageLibraryStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIStorageLibraryStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIStorageLibraryStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryStatics_GetLibraryAsync(This, libraryId, operation) \
    ((This)->lpVtbl->GetLibraryAsync(This, libraryId, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageLibraryStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IStorageLibraryStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageLibrary
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageLibraryStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageLibraryStatics2[] = L"Windows.Storage.IStorageLibraryStatics2";
typedef struct __x_ABI_CWindows_CStorage_CIStorageLibraryStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIStorageLibraryStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIStorageLibraryStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIStorageLibraryStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIStorageLibraryStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIStorageLibraryStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIStorageLibraryStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetLibraryForUserAsync)(__x_ABI_CWindows_CStorage_CIStorageLibraryStatics2* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        enum __x_ABI_CWindows_CStorage_CKnownLibraryId libraryId,
        __FIAsyncOperation_1_Windows__CStorage__CStorageLibrary** operation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIStorageLibraryStatics2Vtbl;

interface __x_ABI_CWindows_CStorage_CIStorageLibraryStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIStorageLibraryStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIStorageLibraryStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIStorageLibraryStatics2_GetLibraryForUserAsync(This, user, libraryId, operation) \
    ((This)->lpVtbl->GetLibraryForUserAsync(This, user, libraryId, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageLibraryStatics2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageLibraryStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Storage.IStorageProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageProvider[] = L"Windows.Storage.IStorageProvider";
typedef struct __x_ABI_CWindows_CStorage_CIStorageProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIStorageProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIStorageProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIStorageProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIStorageProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIStorageProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIStorageProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CStorage_CIStorageProvider* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CStorage_CIStorageProvider* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIStorageProviderVtbl;

interface __x_ABI_CWindows_CStorage_CIStorageProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIStorageProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIStorageProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIStorageProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIStorageProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIStorageProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIStorageProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIStorageProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIStorageProvider_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CStorage_CIStorageProvider_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageProvider;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IStorageProvider2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageProvider
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Storage.IStorageProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageProvider2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageProvider2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageProvider2[] = L"Windows.Storage.IStorageProvider2";
typedef struct __x_ABI_CWindows_CStorage_CIStorageProvider2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIStorageProvider2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIStorageProvider2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIStorageProvider2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIStorageProvider2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIStorageProvider2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIStorageProvider2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* IsPropertySupportedForPartialFileAsync)(__x_ABI_CWindows_CStorage_CIStorageProvider2* This,
        HSTRING propertyCanonicalName,
        __FIAsyncOperation_1_boolean** operation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIStorageProvider2Vtbl;

interface __x_ABI_CWindows_CStorage_CIStorageProvider2
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIStorageProvider2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIStorageProvider2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIStorageProvider2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIStorageProvider2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIStorageProvider2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIStorageProvider2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIStorageProvider2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIStorageProvider2_IsPropertySupportedForPartialFileAsync(This, propertyCanonicalName, operation) \
    ((This)->lpVtbl->IsPropertySupportedForPartialFileAsync(This, propertyCanonicalName, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageProvider2;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageProvider2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Storage.IStorageStreamTransaction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.StorageStreamTransaction
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIStorageStreamTransaction_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStorageStreamTransaction_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStorageStreamTransaction[] = L"Windows.Storage.IStorageStreamTransaction";
typedef struct __x_ABI_CWindows_CStorage_CIStorageStreamTransactionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIStorageStreamTransaction* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIStorageStreamTransaction* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIStorageStreamTransaction* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIStorageStreamTransaction* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIStorageStreamTransaction* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIStorageStreamTransaction* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Stream)(__x_ABI_CWindows_CStorage_CIStorageStreamTransaction* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream** value);
    HRESULT (STDMETHODCALLTYPE* CommitAsync)(__x_ABI_CWindows_CStorage_CIStorageStreamTransaction* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIStorageStreamTransactionVtbl;

interface __x_ABI_CWindows_CStorage_CIStorageStreamTransaction
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIStorageStreamTransactionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIStorageStreamTransaction_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIStorageStreamTransaction_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIStorageStreamTransaction_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIStorageStreamTransaction_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIStorageStreamTransaction_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIStorageStreamTransaction_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIStorageStreamTransaction_get_Stream(This, value) \
    ((This)->lpVtbl->get_Stream(This, value))

#define __x_ABI_CWindows_CStorage_CIStorageStreamTransaction_CommitAsync(This, operation) \
    ((This)->lpVtbl->CommitAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStorageStreamTransaction;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStorageStreamTransaction_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IStreamedFileDataRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CIStreamedFileDataRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIStreamedFileDataRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IStreamedFileDataRequest[] = L"Windows.Storage.IStreamedFileDataRequest";
typedef struct __x_ABI_CWindows_CStorage_CIStreamedFileDataRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIStreamedFileDataRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIStreamedFileDataRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIStreamedFileDataRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIStreamedFileDataRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIStreamedFileDataRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIStreamedFileDataRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FailAndClose)(__x_ABI_CWindows_CStorage_CIStreamedFileDataRequest* This,
        enum __x_ABI_CWindows_CStorage_CStreamedFileFailureMode failureMode);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIStreamedFileDataRequestVtbl;

interface __x_ABI_CWindows_CStorage_CIStreamedFileDataRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIStreamedFileDataRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIStreamedFileDataRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIStreamedFileDataRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIStreamedFileDataRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIStreamedFileDataRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIStreamedFileDataRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIStreamedFileDataRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIStreamedFileDataRequest_FailAndClose(This, failureMode) \
    ((This)->lpVtbl->FailAndClose(This, failureMode))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIStreamedFileDataRequest;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIStreamedFileDataRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.ISystemAudioProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.SystemAudioProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CISystemAudioProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CISystemAudioProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_ISystemAudioProperties[] = L"Windows.Storage.ISystemAudioProperties";
typedef struct __x_ABI_CWindows_CStorage_CISystemAudioPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CISystemAudioProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CISystemAudioProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CISystemAudioProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CISystemAudioProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CISystemAudioProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CISystemAudioProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EncodingBitrate)(__x_ABI_CWindows_CStorage_CISystemAudioProperties* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CISystemAudioPropertiesVtbl;

interface __x_ABI_CWindows_CStorage_CISystemAudioProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CISystemAudioPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CISystemAudioProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CISystemAudioProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CISystemAudioProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CISystemAudioProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CISystemAudioProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CISystemAudioProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CISystemAudioProperties_get_EncodingBitrate(This, value) \
    ((This)->lpVtbl->get_EncodingBitrate(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CISystemAudioProperties;
#endif /* !defined(____x_ABI_CWindows_CStorage_CISystemAudioProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.ISystemDataPaths
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Storage.SystemDataPaths
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CStorage_CISystemDataPaths_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CISystemDataPaths_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_ISystemDataPaths[] = L"Windows.Storage.ISystemDataPaths";
typedef struct __x_ABI_CWindows_CStorage_CISystemDataPathsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CISystemDataPaths* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CISystemDataPaths* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CISystemDataPaths* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CISystemDataPaths* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CISystemDataPaths* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CISystemDataPaths* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Fonts)(__x_ABI_CWindows_CStorage_CISystemDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ProgramData)(__x_ABI_CWindows_CStorage_CISystemDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Public)(__x_ABI_CWindows_CStorage_CISystemDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_PublicDesktop)(__x_ABI_CWindows_CStorage_CISystemDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_PublicDocuments)(__x_ABI_CWindows_CStorage_CISystemDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_PublicDownloads)(__x_ABI_CWindows_CStorage_CISystemDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_PublicMusic)(__x_ABI_CWindows_CStorage_CISystemDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_PublicPictures)(__x_ABI_CWindows_CStorage_CISystemDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_PublicVideos)(__x_ABI_CWindows_CStorage_CISystemDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_System)(__x_ABI_CWindows_CStorage_CISystemDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SystemHost)(__x_ABI_CWindows_CStorage_CISystemDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SystemX86)(__x_ABI_CWindows_CStorage_CISystemDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SystemX64)(__x_ABI_CWindows_CStorage_CISystemDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SystemArm)(__x_ABI_CWindows_CStorage_CISystemDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_UserProfiles)(__x_ABI_CWindows_CStorage_CISystemDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Windows)(__x_ABI_CWindows_CStorage_CISystemDataPaths* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CISystemDataPathsVtbl;

interface __x_ABI_CWindows_CStorage_CISystemDataPaths
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CISystemDataPathsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CISystemDataPaths_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CISystemDataPaths_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CISystemDataPaths_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CISystemDataPaths_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CISystemDataPaths_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CISystemDataPaths_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CISystemDataPaths_get_Fonts(This, value) \
    ((This)->lpVtbl->get_Fonts(This, value))

#define __x_ABI_CWindows_CStorage_CISystemDataPaths_get_ProgramData(This, value) \
    ((This)->lpVtbl->get_ProgramData(This, value))

#define __x_ABI_CWindows_CStorage_CISystemDataPaths_get_Public(This, value) \
    ((This)->lpVtbl->get_Public(This, value))

#define __x_ABI_CWindows_CStorage_CISystemDataPaths_get_PublicDesktop(This, value) \
    ((This)->lpVtbl->get_PublicDesktop(This, value))

#define __x_ABI_CWindows_CStorage_CISystemDataPaths_get_PublicDocuments(This, value) \
    ((This)->lpVtbl->get_PublicDocuments(This, value))

#define __x_ABI_CWindows_CStorage_CISystemDataPaths_get_PublicDownloads(This, value) \
    ((This)->lpVtbl->get_PublicDownloads(This, value))

#define __x_ABI_CWindows_CStorage_CISystemDataPaths_get_PublicMusic(This, value) \
    ((This)->lpVtbl->get_PublicMusic(This, value))

#define __x_ABI_CWindows_CStorage_CISystemDataPaths_get_PublicPictures(This, value) \
    ((This)->lpVtbl->get_PublicPictures(This, value))

#define __x_ABI_CWindows_CStorage_CISystemDataPaths_get_PublicVideos(This, value) \
    ((This)->lpVtbl->get_PublicVideos(This, value))

#define __x_ABI_CWindows_CStorage_CISystemDataPaths_get_System(This, value) \
    ((This)->lpVtbl->get_System(This, value))

#define __x_ABI_CWindows_CStorage_CISystemDataPaths_get_SystemHost(This, value) \
    ((This)->lpVtbl->get_SystemHost(This, value))

#define __x_ABI_CWindows_CStorage_CISystemDataPaths_get_SystemX86(This, value) \
    ((This)->lpVtbl->get_SystemX86(This, value))

#define __x_ABI_CWindows_CStorage_CISystemDataPaths_get_SystemX64(This, value) \
    ((This)->lpVtbl->get_SystemX64(This, value))

#define __x_ABI_CWindows_CStorage_CISystemDataPaths_get_SystemArm(This, value) \
    ((This)->lpVtbl->get_SystemArm(This, value))

#define __x_ABI_CWindows_CStorage_CISystemDataPaths_get_UserProfiles(This, value) \
    ((This)->lpVtbl->get_UserProfiles(This, value))

#define __x_ABI_CWindows_CStorage_CISystemDataPaths_get_Windows(This, value) \
    ((This)->lpVtbl->get_Windows(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CISystemDataPaths;
#endif /* !defined(____x_ABI_CWindows_CStorage_CISystemDataPaths_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Storage.ISystemDataPathsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Storage.SystemDataPaths
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CStorage_CISystemDataPathsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CISystemDataPathsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_ISystemDataPathsStatics[] = L"Windows.Storage.ISystemDataPathsStatics";
typedef struct __x_ABI_CWindows_CStorage_CISystemDataPathsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CISystemDataPathsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CISystemDataPathsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CISystemDataPathsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CISystemDataPathsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CISystemDataPathsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CISystemDataPathsStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CStorage_CISystemDataPathsStatics* This,
        __x_ABI_CWindows_CStorage_CISystemDataPaths** result);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CISystemDataPathsStaticsVtbl;

interface __x_ABI_CWindows_CStorage_CISystemDataPathsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CISystemDataPathsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CISystemDataPathsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CISystemDataPathsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CISystemDataPathsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CISystemDataPathsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CISystemDataPathsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CISystemDataPathsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CISystemDataPathsStatics_GetDefault(This, result) \
    ((This)->lpVtbl->GetDefault(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CISystemDataPathsStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CISystemDataPathsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Storage.ISystemGPSProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.SystemGPSProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CISystemGPSProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CISystemGPSProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_ISystemGPSProperties[] = L"Windows.Storage.ISystemGPSProperties";
typedef struct __x_ABI_CWindows_CStorage_CISystemGPSPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CISystemGPSProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CISystemGPSProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CISystemGPSProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CISystemGPSProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CISystemGPSProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CISystemGPSProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LatitudeDecimal)(__x_ABI_CWindows_CStorage_CISystemGPSProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_LongitudeDecimal)(__x_ABI_CWindows_CStorage_CISystemGPSProperties* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CISystemGPSPropertiesVtbl;

interface __x_ABI_CWindows_CStorage_CISystemGPSProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CISystemGPSPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CISystemGPSProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CISystemGPSProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CISystemGPSProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CISystemGPSProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CISystemGPSProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CISystemGPSProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CISystemGPSProperties_get_LatitudeDecimal(This, value) \
    ((This)->lpVtbl->get_LatitudeDecimal(This, value))

#define __x_ABI_CWindows_CStorage_CISystemGPSProperties_get_LongitudeDecimal(This, value) \
    ((This)->lpVtbl->get_LongitudeDecimal(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CISystemGPSProperties;
#endif /* !defined(____x_ABI_CWindows_CStorage_CISystemGPSProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.ISystemImageProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.SystemImageProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CISystemImageProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CISystemImageProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_ISystemImageProperties[] = L"Windows.Storage.ISystemImageProperties";
typedef struct __x_ABI_CWindows_CStorage_CISystemImagePropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CISystemImageProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CISystemImageProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CISystemImageProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CISystemImageProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CISystemImageProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CISystemImageProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_HorizontalSize)(__x_ABI_CWindows_CStorage_CISystemImageProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_VerticalSize)(__x_ABI_CWindows_CStorage_CISystemImageProperties* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CISystemImagePropertiesVtbl;

interface __x_ABI_CWindows_CStorage_CISystemImageProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CISystemImagePropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CISystemImageProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CISystemImageProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CISystemImageProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CISystemImageProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CISystemImageProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CISystemImageProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CISystemImageProperties_get_HorizontalSize(This, value) \
    ((This)->lpVtbl->get_HorizontalSize(This, value))

#define __x_ABI_CWindows_CStorage_CISystemImageProperties_get_VerticalSize(This, value) \
    ((This)->lpVtbl->get_VerticalSize(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CISystemImageProperties;
#endif /* !defined(____x_ABI_CWindows_CStorage_CISystemImageProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.ISystemMediaProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.SystemMediaProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CISystemMediaProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CISystemMediaProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_ISystemMediaProperties[] = L"Windows.Storage.ISystemMediaProperties";
typedef struct __x_ABI_CWindows_CStorage_CISystemMediaPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CISystemMediaProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CISystemMediaProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CISystemMediaProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CISystemMediaProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CISystemMediaProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CISystemMediaProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Duration)(__x_ABI_CWindows_CStorage_CISystemMediaProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Producer)(__x_ABI_CWindows_CStorage_CISystemMediaProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Publisher)(__x_ABI_CWindows_CStorage_CISystemMediaProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SubTitle)(__x_ABI_CWindows_CStorage_CISystemMediaProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Writer)(__x_ABI_CWindows_CStorage_CISystemMediaProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Year)(__x_ABI_CWindows_CStorage_CISystemMediaProperties* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CISystemMediaPropertiesVtbl;

interface __x_ABI_CWindows_CStorage_CISystemMediaProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CISystemMediaPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CISystemMediaProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CISystemMediaProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CISystemMediaProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CISystemMediaProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CISystemMediaProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CISystemMediaProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CISystemMediaProperties_get_Duration(This, value) \
    ((This)->lpVtbl->get_Duration(This, value))

#define __x_ABI_CWindows_CStorage_CISystemMediaProperties_get_Producer(This, value) \
    ((This)->lpVtbl->get_Producer(This, value))

#define __x_ABI_CWindows_CStorage_CISystemMediaProperties_get_Publisher(This, value) \
    ((This)->lpVtbl->get_Publisher(This, value))

#define __x_ABI_CWindows_CStorage_CISystemMediaProperties_get_SubTitle(This, value) \
    ((This)->lpVtbl->get_SubTitle(This, value))

#define __x_ABI_CWindows_CStorage_CISystemMediaProperties_get_Writer(This, value) \
    ((This)->lpVtbl->get_Writer(This, value))

#define __x_ABI_CWindows_CStorage_CISystemMediaProperties_get_Year(This, value) \
    ((This)->lpVtbl->get_Year(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CISystemMediaProperties;
#endif /* !defined(____x_ABI_CWindows_CStorage_CISystemMediaProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.ISystemMusicProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.SystemMusicProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CISystemMusicProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CISystemMusicProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_ISystemMusicProperties[] = L"Windows.Storage.ISystemMusicProperties";
typedef struct __x_ABI_CWindows_CStorage_CISystemMusicPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CISystemMusicProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CISystemMusicProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CISystemMusicProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CISystemMusicProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CISystemMusicProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CISystemMusicProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AlbumArtist)(__x_ABI_CWindows_CStorage_CISystemMusicProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AlbumTitle)(__x_ABI_CWindows_CStorage_CISystemMusicProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Artist)(__x_ABI_CWindows_CStorage_CISystemMusicProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Composer)(__x_ABI_CWindows_CStorage_CISystemMusicProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Conductor)(__x_ABI_CWindows_CStorage_CISystemMusicProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayArtist)(__x_ABI_CWindows_CStorage_CISystemMusicProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Genre)(__x_ABI_CWindows_CStorage_CISystemMusicProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_TrackNumber)(__x_ABI_CWindows_CStorage_CISystemMusicProperties* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CISystemMusicPropertiesVtbl;

interface __x_ABI_CWindows_CStorage_CISystemMusicProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CISystemMusicPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CISystemMusicProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CISystemMusicProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CISystemMusicProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CISystemMusicProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CISystemMusicProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CISystemMusicProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CISystemMusicProperties_get_AlbumArtist(This, value) \
    ((This)->lpVtbl->get_AlbumArtist(This, value))

#define __x_ABI_CWindows_CStorage_CISystemMusicProperties_get_AlbumTitle(This, value) \
    ((This)->lpVtbl->get_AlbumTitle(This, value))

#define __x_ABI_CWindows_CStorage_CISystemMusicProperties_get_Artist(This, value) \
    ((This)->lpVtbl->get_Artist(This, value))

#define __x_ABI_CWindows_CStorage_CISystemMusicProperties_get_Composer(This, value) \
    ((This)->lpVtbl->get_Composer(This, value))

#define __x_ABI_CWindows_CStorage_CISystemMusicProperties_get_Conductor(This, value) \
    ((This)->lpVtbl->get_Conductor(This, value))

#define __x_ABI_CWindows_CStorage_CISystemMusicProperties_get_DisplayArtist(This, value) \
    ((This)->lpVtbl->get_DisplayArtist(This, value))

#define __x_ABI_CWindows_CStorage_CISystemMusicProperties_get_Genre(This, value) \
    ((This)->lpVtbl->get_Genre(This, value))

#define __x_ABI_CWindows_CStorage_CISystemMusicProperties_get_TrackNumber(This, value) \
    ((This)->lpVtbl->get_TrackNumber(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CISystemMusicProperties;
#endif /* !defined(____x_ABI_CWindows_CStorage_CISystemMusicProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.ISystemPhotoProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.SystemPhotoProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CISystemPhotoProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CISystemPhotoProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_ISystemPhotoProperties[] = L"Windows.Storage.ISystemPhotoProperties";
typedef struct __x_ABI_CWindows_CStorage_CISystemPhotoPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CISystemPhotoProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CISystemPhotoProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CISystemPhotoProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CISystemPhotoProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CISystemPhotoProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CISystemPhotoProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CameraManufacturer)(__x_ABI_CWindows_CStorage_CISystemPhotoProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CameraModel)(__x_ABI_CWindows_CStorage_CISystemPhotoProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DateTaken)(__x_ABI_CWindows_CStorage_CISystemPhotoProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Orientation)(__x_ABI_CWindows_CStorage_CISystemPhotoProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_PeopleNames)(__x_ABI_CWindows_CStorage_CISystemPhotoProperties* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CISystemPhotoPropertiesVtbl;

interface __x_ABI_CWindows_CStorage_CISystemPhotoProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CISystemPhotoPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CISystemPhotoProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CISystemPhotoProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CISystemPhotoProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CISystemPhotoProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CISystemPhotoProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CISystemPhotoProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CISystemPhotoProperties_get_CameraManufacturer(This, value) \
    ((This)->lpVtbl->get_CameraManufacturer(This, value))

#define __x_ABI_CWindows_CStorage_CISystemPhotoProperties_get_CameraModel(This, value) \
    ((This)->lpVtbl->get_CameraModel(This, value))

#define __x_ABI_CWindows_CStorage_CISystemPhotoProperties_get_DateTaken(This, value) \
    ((This)->lpVtbl->get_DateTaken(This, value))

#define __x_ABI_CWindows_CStorage_CISystemPhotoProperties_get_Orientation(This, value) \
    ((This)->lpVtbl->get_Orientation(This, value))

#define __x_ABI_CWindows_CStorage_CISystemPhotoProperties_get_PeopleNames(This, value) \
    ((This)->lpVtbl->get_PeopleNames(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CISystemPhotoProperties;
#endif /* !defined(____x_ABI_CWindows_CStorage_CISystemPhotoProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.ISystemProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.SystemProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CISystemProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CISystemProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_ISystemProperties[] = L"Windows.Storage.ISystemProperties";
typedef struct __x_ABI_CWindows_CStorage_CISystemPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CISystemProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CISystemProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CISystemProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CISystemProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CISystemProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CISystemProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Author)(__x_ABI_CWindows_CStorage_CISystemProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Comment)(__x_ABI_CWindows_CStorage_CISystemProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ItemNameDisplay)(__x_ABI_CWindows_CStorage_CISystemProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Keywords)(__x_ABI_CWindows_CStorage_CISystemProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Rating)(__x_ABI_CWindows_CStorage_CISystemProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CStorage_CISystemProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Audio)(__x_ABI_CWindows_CStorage_CISystemProperties* This,
        __x_ABI_CWindows_CStorage_CISystemAudioProperties** value);
    HRESULT (STDMETHODCALLTYPE* get_GPS)(__x_ABI_CWindows_CStorage_CISystemProperties* This,
        __x_ABI_CWindows_CStorage_CISystemGPSProperties** value);
    HRESULT (STDMETHODCALLTYPE* get_Media)(__x_ABI_CWindows_CStorage_CISystemProperties* This,
        __x_ABI_CWindows_CStorage_CISystemMediaProperties** value);
    HRESULT (STDMETHODCALLTYPE* get_Music)(__x_ABI_CWindows_CStorage_CISystemProperties* This,
        __x_ABI_CWindows_CStorage_CISystemMusicProperties** value);
    HRESULT (STDMETHODCALLTYPE* get_Photo)(__x_ABI_CWindows_CStorage_CISystemProperties* This,
        __x_ABI_CWindows_CStorage_CISystemPhotoProperties** value);
    HRESULT (STDMETHODCALLTYPE* get_Video)(__x_ABI_CWindows_CStorage_CISystemProperties* This,
        __x_ABI_CWindows_CStorage_CISystemVideoProperties** value);
    HRESULT (STDMETHODCALLTYPE* get_Image)(__x_ABI_CWindows_CStorage_CISystemProperties* This,
        __x_ABI_CWindows_CStorage_CISystemImageProperties** value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CISystemPropertiesVtbl;

interface __x_ABI_CWindows_CStorage_CISystemProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CISystemPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CISystemProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CISystemProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CISystemProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CISystemProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CISystemProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CISystemProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CISystemProperties_get_Author(This, value) \
    ((This)->lpVtbl->get_Author(This, value))

#define __x_ABI_CWindows_CStorage_CISystemProperties_get_Comment(This, value) \
    ((This)->lpVtbl->get_Comment(This, value))

#define __x_ABI_CWindows_CStorage_CISystemProperties_get_ItemNameDisplay(This, value) \
    ((This)->lpVtbl->get_ItemNameDisplay(This, value))

#define __x_ABI_CWindows_CStorage_CISystemProperties_get_Keywords(This, value) \
    ((This)->lpVtbl->get_Keywords(This, value))

#define __x_ABI_CWindows_CStorage_CISystemProperties_get_Rating(This, value) \
    ((This)->lpVtbl->get_Rating(This, value))

#define __x_ABI_CWindows_CStorage_CISystemProperties_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CStorage_CISystemProperties_get_Audio(This, value) \
    ((This)->lpVtbl->get_Audio(This, value))

#define __x_ABI_CWindows_CStorage_CISystemProperties_get_GPS(This, value) \
    ((This)->lpVtbl->get_GPS(This, value))

#define __x_ABI_CWindows_CStorage_CISystemProperties_get_Media(This, value) \
    ((This)->lpVtbl->get_Media(This, value))

#define __x_ABI_CWindows_CStorage_CISystemProperties_get_Music(This, value) \
    ((This)->lpVtbl->get_Music(This, value))

#define __x_ABI_CWindows_CStorage_CISystemProperties_get_Photo(This, value) \
    ((This)->lpVtbl->get_Photo(This, value))

#define __x_ABI_CWindows_CStorage_CISystemProperties_get_Video(This, value) \
    ((This)->lpVtbl->get_Video(This, value))

#define __x_ABI_CWindows_CStorage_CISystemProperties_get_Image(This, value) \
    ((This)->lpVtbl->get_Image(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CISystemProperties;
#endif /* !defined(____x_ABI_CWindows_CStorage_CISystemProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.ISystemVideoProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Storage.SystemVideoProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CStorage_CISystemVideoProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CISystemVideoProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_ISystemVideoProperties[] = L"Windows.Storage.ISystemVideoProperties";
typedef struct __x_ABI_CWindows_CStorage_CISystemVideoPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CISystemVideoProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CISystemVideoProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CISystemVideoProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CISystemVideoProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CISystemVideoProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CISystemVideoProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Director)(__x_ABI_CWindows_CStorage_CISystemVideoProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_FrameHeight)(__x_ABI_CWindows_CStorage_CISystemVideoProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_FrameWidth)(__x_ABI_CWindows_CStorage_CISystemVideoProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Orientation)(__x_ABI_CWindows_CStorage_CISystemVideoProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_TotalBitrate)(__x_ABI_CWindows_CStorage_CISystemVideoProperties* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CISystemVideoPropertiesVtbl;

interface __x_ABI_CWindows_CStorage_CISystemVideoProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CISystemVideoPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CISystemVideoProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CISystemVideoProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CISystemVideoProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CISystemVideoProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CISystemVideoProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CISystemVideoProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CISystemVideoProperties_get_Director(This, value) \
    ((This)->lpVtbl->get_Director(This, value))

#define __x_ABI_CWindows_CStorage_CISystemVideoProperties_get_FrameHeight(This, value) \
    ((This)->lpVtbl->get_FrameHeight(This, value))

#define __x_ABI_CWindows_CStorage_CISystemVideoProperties_get_FrameWidth(This, value) \
    ((This)->lpVtbl->get_FrameWidth(This, value))

#define __x_ABI_CWindows_CStorage_CISystemVideoProperties_get_Orientation(This, value) \
    ((This)->lpVtbl->get_Orientation(This, value))

#define __x_ABI_CWindows_CStorage_CISystemVideoProperties_get_TotalBitrate(This, value) \
    ((This)->lpVtbl->get_TotalBitrate(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CISystemVideoProperties;
#endif /* !defined(____x_ABI_CWindows_CStorage_CISystemVideoProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Storage.IUserDataPaths
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Storage.UserDataPaths
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CStorage_CIUserDataPaths_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIUserDataPaths_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IUserDataPaths[] = L"Windows.Storage.IUserDataPaths";
typedef struct __x_ABI_CWindows_CStorage_CIUserDataPathsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIUserDataPaths* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIUserDataPaths* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIUserDataPaths* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIUserDataPaths* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIUserDataPaths* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIUserDataPaths* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CameraRoll)(__x_ABI_CWindows_CStorage_CIUserDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Cookies)(__x_ABI_CWindows_CStorage_CIUserDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Desktop)(__x_ABI_CWindows_CStorage_CIUserDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Documents)(__x_ABI_CWindows_CStorage_CIUserDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Downloads)(__x_ABI_CWindows_CStorage_CIUserDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Favorites)(__x_ABI_CWindows_CStorage_CIUserDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_History)(__x_ABI_CWindows_CStorage_CIUserDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_InternetCache)(__x_ABI_CWindows_CStorage_CIUserDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_LocalAppData)(__x_ABI_CWindows_CStorage_CIUserDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_LocalAppDataLow)(__x_ABI_CWindows_CStorage_CIUserDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Music)(__x_ABI_CWindows_CStorage_CIUserDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Pictures)(__x_ABI_CWindows_CStorage_CIUserDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Profile)(__x_ABI_CWindows_CStorage_CIUserDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Recent)(__x_ABI_CWindows_CStorage_CIUserDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_RoamingAppData)(__x_ABI_CWindows_CStorage_CIUserDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SavedPictures)(__x_ABI_CWindows_CStorage_CIUserDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Screenshots)(__x_ABI_CWindows_CStorage_CIUserDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Templates)(__x_ABI_CWindows_CStorage_CIUserDataPaths* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Videos)(__x_ABI_CWindows_CStorage_CIUserDataPaths* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIUserDataPathsVtbl;

interface __x_ABI_CWindows_CStorage_CIUserDataPaths
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIUserDataPathsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIUserDataPaths_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIUserDataPaths_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIUserDataPaths_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIUserDataPaths_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIUserDataPaths_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIUserDataPaths_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIUserDataPaths_get_CameraRoll(This, value) \
    ((This)->lpVtbl->get_CameraRoll(This, value))

#define __x_ABI_CWindows_CStorage_CIUserDataPaths_get_Cookies(This, value) \
    ((This)->lpVtbl->get_Cookies(This, value))

#define __x_ABI_CWindows_CStorage_CIUserDataPaths_get_Desktop(This, value) \
    ((This)->lpVtbl->get_Desktop(This, value))

#define __x_ABI_CWindows_CStorage_CIUserDataPaths_get_Documents(This, value) \
    ((This)->lpVtbl->get_Documents(This, value))

#define __x_ABI_CWindows_CStorage_CIUserDataPaths_get_Downloads(This, value) \
    ((This)->lpVtbl->get_Downloads(This, value))

#define __x_ABI_CWindows_CStorage_CIUserDataPaths_get_Favorites(This, value) \
    ((This)->lpVtbl->get_Favorites(This, value))

#define __x_ABI_CWindows_CStorage_CIUserDataPaths_get_History(This, value) \
    ((This)->lpVtbl->get_History(This, value))

#define __x_ABI_CWindows_CStorage_CIUserDataPaths_get_InternetCache(This, value) \
    ((This)->lpVtbl->get_InternetCache(This, value))

#define __x_ABI_CWindows_CStorage_CIUserDataPaths_get_LocalAppData(This, value) \
    ((This)->lpVtbl->get_LocalAppData(This, value))

#define __x_ABI_CWindows_CStorage_CIUserDataPaths_get_LocalAppDataLow(This, value) \
    ((This)->lpVtbl->get_LocalAppDataLow(This, value))

#define __x_ABI_CWindows_CStorage_CIUserDataPaths_get_Music(This, value) \
    ((This)->lpVtbl->get_Music(This, value))

#define __x_ABI_CWindows_CStorage_CIUserDataPaths_get_Pictures(This, value) \
    ((This)->lpVtbl->get_Pictures(This, value))

#define __x_ABI_CWindows_CStorage_CIUserDataPaths_get_Profile(This, value) \
    ((This)->lpVtbl->get_Profile(This, value))

#define __x_ABI_CWindows_CStorage_CIUserDataPaths_get_Recent(This, value) \
    ((This)->lpVtbl->get_Recent(This, value))

#define __x_ABI_CWindows_CStorage_CIUserDataPaths_get_RoamingAppData(This, value) \
    ((This)->lpVtbl->get_RoamingAppData(This, value))

#define __x_ABI_CWindows_CStorage_CIUserDataPaths_get_SavedPictures(This, value) \
    ((This)->lpVtbl->get_SavedPictures(This, value))

#define __x_ABI_CWindows_CStorage_CIUserDataPaths_get_Screenshots(This, value) \
    ((This)->lpVtbl->get_Screenshots(This, value))

#define __x_ABI_CWindows_CStorage_CIUserDataPaths_get_Templates(This, value) \
    ((This)->lpVtbl->get_Templates(This, value))

#define __x_ABI_CWindows_CStorage_CIUserDataPaths_get_Videos(This, value) \
    ((This)->lpVtbl->get_Videos(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIUserDataPaths;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIUserDataPaths_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Storage.IUserDataPathsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Storage.UserDataPaths
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CStorage_CIUserDataPathsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CStorage_CIUserDataPathsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Storage_IUserDataPathsStatics[] = L"Windows.Storage.IUserDataPathsStatics";
typedef struct __x_ABI_CWindows_CStorage_CIUserDataPathsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CStorage_CIUserDataPathsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CStorage_CIUserDataPathsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CStorage_CIUserDataPathsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CStorage_CIUserDataPathsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CStorage_CIUserDataPathsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CStorage_CIUserDataPathsStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForUser)(__x_ABI_CWindows_CStorage_CIUserDataPathsStatics* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __x_ABI_CWindows_CStorage_CIUserDataPaths** result);
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CStorage_CIUserDataPathsStatics* This,
        __x_ABI_CWindows_CStorage_CIUserDataPaths** result);

    END_INTERFACE
} __x_ABI_CWindows_CStorage_CIUserDataPathsStaticsVtbl;

interface __x_ABI_CWindows_CStorage_CIUserDataPathsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CStorage_CIUserDataPathsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CStorage_CIUserDataPathsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CStorage_CIUserDataPathsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CStorage_CIUserDataPathsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CStorage_CIUserDataPathsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CStorage_CIUserDataPathsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CStorage_CIUserDataPathsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CStorage_CIUserDataPathsStatics_GetForUser(This, user, result) \
    ((This)->lpVtbl->GetForUser(This, user, result))

#define __x_ABI_CWindows_CStorage_CIUserDataPathsStatics_GetDefault(This, result) \
    ((This)->lpVtbl->GetDefault(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CStorage_CIUserDataPathsStatics;
#endif /* !defined(____x_ABI_CWindows_CStorage_CIUserDataPathsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Storage.AppDataPaths
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.IAppDataPathsStatics interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.IAppDataPaths ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Storage_AppDataPaths_DEFINED
#define RUNTIMECLASS_Windows_Storage_AppDataPaths_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_AppDataPaths[] = L"Windows.Storage.AppDataPaths";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Storage.ApplicationData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.IApplicationDataStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Storage.IApplicationDataStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.IApplicationData ** Default Interface **
 *    Windows.Storage.IApplicationData2
 *    Windows.Storage.IApplicationData3
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_ApplicationData_DEFINED
#define RUNTIMECLASS_Windows_Storage_ApplicationData_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_ApplicationData[] = L"Windows.Storage.ApplicationData";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.ApplicationDataCompositeValue
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IPropertySet ** Default Interface **
 *    Windows.Foundation.Collections.IObservableMap`2<String, Object>
 *    Windows.Foundation.Collections.IMap`2<String, Object>
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_ApplicationDataCompositeValue_DEFINED
#define RUNTIMECLASS_Windows_Storage_ApplicationDataCompositeValue_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_ApplicationDataCompositeValue[] = L"Windows.Storage.ApplicationDataCompositeValue";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.ApplicationDataContainer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.IApplicationDataContainer ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_ApplicationDataContainer_DEFINED
#define RUNTIMECLASS_Windows_Storage_ApplicationDataContainer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_ApplicationDataContainer[] = L"Windows.Storage.ApplicationDataContainer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.ApplicationDataContainerSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IPropertySet ** Default Interface **
 *    Windows.Foundation.Collections.IObservableMap`2<String, Object>
 *    Windows.Foundation.Collections.IMap`2<String, Object>
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_ApplicationDataContainerSettings_DEFINED
#define RUNTIMECLASS_Windows_Storage_ApplicationDataContainerSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_ApplicationDataContainerSettings[] = L"Windows.Storage.ApplicationDataContainerSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.CachedFileManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.ICachedFileManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_CachedFileManager_DEFINED
#define RUNTIMECLASS_Windows_Storage_CachedFileManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_CachedFileManager[] = L"Windows.Storage.CachedFileManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.DownloadsFolder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.IDownloadsFolderStatics2 interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Storage.IDownloadsFolderStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_DownloadsFolder_DEFINED
#define RUNTIMECLASS_Windows_Storage_DownloadsFolder_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_DownloadsFolder[] = L"Windows.Storage.DownloadsFolder";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.FileIO
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.IFileIOStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_FileIO_DEFINED
#define RUNTIMECLASS_Windows_Storage_FileIO_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_FileIO[] = L"Windows.Storage.FileIO";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.KnownFolders
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.IKnownFoldersStatics3 interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Storage.IKnownFoldersPlaylistsStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Storage.IKnownFoldersStatics4 interface starting with version 10.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Storage.IKnownFoldersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Storage.IKnownFoldersCameraRollStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Storage.IKnownFoldersSavedPicturesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Storage.IKnownFoldersStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_KnownFolders_DEFINED
#define RUNTIMECLASS_Windows_Storage_KnownFolders_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_KnownFolders[] = L"Windows.Storage.KnownFolders";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.PathIO
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.IPathIOStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_PathIO_DEFINED
#define RUNTIMECLASS_Windows_Storage_PathIO_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_PathIO[] = L"Windows.Storage.PathIO";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.SetVersionDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.ISetVersionDeferral ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_SetVersionDeferral_DEFINED
#define RUNTIMECLASS_Windows_Storage_SetVersionDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_SetVersionDeferral[] = L"Windows.Storage.SetVersionDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.SetVersionRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.ISetVersionRequest ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_SetVersionRequest_DEFINED
#define RUNTIMECLASS_Windows_Storage_SetVersionRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_SetVersionRequest[] = L"Windows.Storage.SetVersionRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.StorageFile
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.IStorageFileStatics2 interface starting with version 10.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Storage.IStorageFileStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.IStorageFile ** Default Interface **
 *    Windows.Storage.Streams.IInputStreamReference
 *    Windows.Storage.Streams.IRandomAccessStreamReference
 *    Windows.Storage.IStorageItem
 *    Windows.Storage.IStorageItemProperties
 *    Windows.Storage.IStorageItemProperties2
 *    Windows.Storage.IStorageItem2
 *    Windows.Storage.IStorageItemPropertiesWithProvider
 *    Windows.Storage.IStorageFilePropertiesWithAvailability
 *    Windows.Storage.IStorageFile2
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_StorageFile_DEFINED
#define RUNTIMECLASS_Windows_Storage_StorageFile_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_StorageFile[] = L"Windows.Storage.StorageFile";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.StorageFolder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.IStorageFolderStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Storage.IStorageFolderStatics2 interface starting with version 10.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.IStorageFolder ** Default Interface **
 *    Windows.Storage.IStorageItem
 *    Windows.Storage.Search.IStorageFolderQueryOperations
 *    Windows.Storage.IStorageItemProperties
 *    Windows.Storage.IStorageItemProperties2
 *    Windows.Storage.IStorageItem2
 *    Windows.Storage.IStorageFolder2
 *    Windows.Storage.IStorageItemPropertiesWithProvider
 *    Windows.Storage.IStorageFolder3
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_StorageFolder_DEFINED
#define RUNTIMECLASS_Windows_Storage_StorageFolder_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_StorageFolder[] = L"Windows.Storage.StorageFolder";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.StorageLibrary
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.IStorageLibraryStatics2 interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Storage.IStorageLibraryStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.IStorageLibrary ** Default Interface **
 *    Windows.Storage.IStorageLibrary2
 *    Windows.Storage.IStorageLibrary3
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_StorageLibrary_DEFINED
#define RUNTIMECLASS_Windows_Storage_StorageLibrary_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_StorageLibrary[] = L"Windows.Storage.StorageLibrary";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.StorageLibraryChange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.IStorageLibraryChange ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Storage_StorageLibraryChange_DEFINED
#define RUNTIMECLASS_Windows_Storage_StorageLibraryChange_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_StorageLibraryChange[] = L"Windows.Storage.StorageLibraryChange";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Storage.StorageLibraryChangeReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.IStorageLibraryChangeReader ** Default Interface **
 *    Windows.Storage.IStorageLibraryChangeReader2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Storage_StorageLibraryChangeReader_DEFINED
#define RUNTIMECLASS_Windows_Storage_StorageLibraryChangeReader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_StorageLibraryChangeReader[] = L"Windows.Storage.StorageLibraryChangeReader";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Storage.StorageLibraryChangeTracker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.IStorageLibraryChangeTracker ** Default Interface **
 *    Windows.Storage.IStorageLibraryChangeTracker2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Storage_StorageLibraryChangeTracker_DEFINED
#define RUNTIMECLASS_Windows_Storage_StorageLibraryChangeTracker_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_StorageLibraryChangeTracker[] = L"Windows.Storage.StorageLibraryChangeTracker";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Storage.StorageLibraryChangeTrackerOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 11.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.IStorageLibraryChangeTrackerOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#ifndef RUNTIMECLASS_Windows_Storage_StorageLibraryChangeTrackerOptions_DEFINED
#define RUNTIMECLASS_Windows_Storage_StorageLibraryChangeTrackerOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_StorageLibraryChangeTrackerOptions[] = L"Windows.Storage.StorageLibraryChangeTrackerOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

/*
 *
 * Class Windows.Storage.StorageLibraryLastChangeId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.IStorageLibraryLastChangeIdStatics interface starting with version 11.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.IStorageLibraryLastChangeId ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#ifndef RUNTIMECLASS_Windows_Storage_StorageLibraryLastChangeId_DEFINED
#define RUNTIMECLASS_Windows_Storage_StorageLibraryLastChangeId_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_StorageLibraryLastChangeId[] = L"Windows.Storage.StorageLibraryLastChangeId";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

/*
 *
 * Class Windows.Storage.StorageProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.IStorageProvider ** Default Interface **
 *    Windows.Storage.IStorageProvider2
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_StorageProvider_DEFINED
#define RUNTIMECLASS_Windows_Storage_StorageProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_StorageProvider[] = L"Windows.Storage.StorageProvider";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.StorageStreamTransaction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.IStorageStreamTransaction ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_StorageStreamTransaction_DEFINED
#define RUNTIMECLASS_Windows_Storage_StorageStreamTransaction_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_StorageStreamTransaction[] = L"Windows.Storage.StorageStreamTransaction";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.StreamedFileDataRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Streams.IOutputStream ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Storage.IStreamedFileDataRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_StreamedFileDataRequest_DEFINED
#define RUNTIMECLASS_Windows_Storage_StreamedFileDataRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_StreamedFileDataRequest[] = L"Windows.Storage.StreamedFileDataRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.SystemAudioProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.ISystemAudioProperties ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_SystemAudioProperties_DEFINED
#define RUNTIMECLASS_Windows_Storage_SystemAudioProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_SystemAudioProperties[] = L"Windows.Storage.SystemAudioProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.SystemDataPaths
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.ISystemDataPathsStatics interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.ISystemDataPaths ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Storage_SystemDataPaths_DEFINED
#define RUNTIMECLASS_Windows_Storage_SystemDataPaths_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_SystemDataPaths[] = L"Windows.Storage.SystemDataPaths";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Storage.SystemGPSProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.ISystemGPSProperties ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_SystemGPSProperties_DEFINED
#define RUNTIMECLASS_Windows_Storage_SystemGPSProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_SystemGPSProperties[] = L"Windows.Storage.SystemGPSProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.SystemImageProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.ISystemImageProperties ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_SystemImageProperties_DEFINED
#define RUNTIMECLASS_Windows_Storage_SystemImageProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_SystemImageProperties[] = L"Windows.Storage.SystemImageProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.SystemMediaProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.ISystemMediaProperties ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_SystemMediaProperties_DEFINED
#define RUNTIMECLASS_Windows_Storage_SystemMediaProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_SystemMediaProperties[] = L"Windows.Storage.SystemMediaProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.SystemMusicProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.ISystemMusicProperties ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_SystemMusicProperties_DEFINED
#define RUNTIMECLASS_Windows_Storage_SystemMusicProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_SystemMusicProperties[] = L"Windows.Storage.SystemMusicProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.SystemPhotoProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.ISystemPhotoProperties ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_SystemPhotoProperties_DEFINED
#define RUNTIMECLASS_Windows_Storage_SystemPhotoProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_SystemPhotoProperties[] = L"Windows.Storage.SystemPhotoProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.SystemProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.ISystemProperties interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_SystemProperties_DEFINED
#define RUNTIMECLASS_Windows_Storage_SystemProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_SystemProperties[] = L"Windows.Storage.SystemProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.SystemVideoProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.ISystemVideoProperties ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Storage_SystemVideoProperties_DEFINED
#define RUNTIMECLASS_Windows_Storage_SystemVideoProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_SystemVideoProperties[] = L"Windows.Storage.SystemVideoProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Storage.UserDataPaths
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Storage.IUserDataPathsStatics interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Storage.IUserDataPaths ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Storage_UserDataPaths_DEFINED
#define RUNTIMECLASS_Windows_Storage_UserDataPaths_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Storage_UserDataPaths[] = L"Windows.Storage.UserDataPaths";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Estorage_p_h__

#endif // __windows2Estorage_h__
