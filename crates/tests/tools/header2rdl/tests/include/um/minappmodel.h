/********************************************************
*                                                       *
*     Copyright (C) Microsoft. All rights reserved.     *
*                                                       *
********************************************************/

#if defined(_MSC_VER)
#pragma once
#endif // _MSC_VER

#include <winapifamily.h>

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)

//
// These constants define the lengths of various portions of a package
// identity string. These lengths are measured in characters and do not
// include space for NULL terminators.
//

#define PACKAGE_ARCHITECTURE_MIN_LENGTH     3
#define PACKAGE_ARCHITECTURE_MAX_LENGTH     7
#define PACKAGE_VERSION_MIN_LENGTH          7  /* "a.b.c.d" where abcd=0 */
#define PACKAGE_VERSION_MAX_LENGTH          23 /* "a.b.c.d" where abcd=65535 */
#define PACKAGE_NAME_MIN_LENGTH             3
#define PACKAGE_NAME_MAX_LENGTH             50
#define PACKAGE_PUBLISHER_MIN_LENGTH        3  /* "S=x" */
#define PACKAGE_PUBLISHER_MAX_LENGTH        8192
#define PACKAGE_PUBLISHERID_MIN_LENGTH      13
#define PACKAGE_PUBLISHERID_MAX_LENGTH      13
#define PACKAGE_RESOURCEID_MIN_LENGTH       0
#define PACKAGE_RESOURCEID_MAX_LENGTH       30
#define PACKAGE_FULL_NAME_MIN_LENGTH        (PACKAGE_NAME_MIN_LENGTH + 1 +         \
                                             PACKAGE_VERSION_MIN_LENGTH + 1 +      \
                                             PACKAGE_ARCHITECTURE_MIN_LENGTH + 1 + \
                                             PACKAGE_RESOURCEID_MIN_LENGTH + 1 +   \
                                             PACKAGE_PUBLISHERID_MIN_LENGTH)
#define PACKAGE_FULL_NAME_MAX_LENGTH        (PACKAGE_NAME_MAX_LENGTH + 1 +         \
                                             PACKAGE_VERSION_MAX_LENGTH + 1 +      \
                                             PACKAGE_ARCHITECTURE_MAX_LENGTH + 1 + \
                                             PACKAGE_RESOURCEID_MAX_LENGTH + 1 +   \
                                             PACKAGE_PUBLISHERID_MAX_LENGTH)
#define PACKAGE_FAMILY_NAME_MIN_LENGTH      (PACKAGE_NAME_MIN_LENGTH + 1 + \
                                             PACKAGE_PUBLISHERID_MIN_LENGTH)
#define PACKAGE_FAMILY_NAME_MAX_LENGTH      (PACKAGE_NAME_MAX_LENGTH + 1 + \
                                             PACKAGE_PUBLISHERID_MAX_LENGTH)

// These constants define the bounds of package dependencies.
#define PACKAGE_MIN_DEPENDENCIES                0
#define PACKAGE_MAX_DEPENDENCIES                128
#define PACKAGE_FAMILY_MIN_RESOURCE_PACKAGES    0
#define PACKAGE_FAMILY_MAX_RESOURCE_PACKAGES    512
#define PACKAGE_GRAPH_MIN_SIZE                  1
#define PACKAGE_GRAPH_MAX_SIZE                  (1 + PACKAGE_MAX_DEPENDENCIES + PACKAGE_FAMILY_MAX_RESOURCE_PACKAGES)

// These constants define the bounds of applications.
#define PACKAGE_APPLICATIONS_MIN_COUNT  0
#define PACKAGE_APPLICATIONS_MAX_COUNT  100

//
// These constants define the lengths of various portions of an application
// identity string. These lengths are measured in characters and include
// space for NULL terminators.
//

#define PACKAGE_RELATIVE_APPLICATION_ID_MIN_LENGTH  (1 + 1)
#define PACKAGE_RELATIVE_APPLICATION_ID_MAX_LENGTH  (64 + 1)
#define APPLICATION_USER_MODEL_ID_MIN_LENGTH        (PACKAGE_FAMILY_NAME_MIN_LENGTH + 1 + \
                                                     PACKAGE_RELATIVE_APPLICATION_ID_MIN_LENGTH)
#define APPLICATION_USER_MODEL_ID_MAX_LENGTH        (PACKAGE_FAMILY_NAME_MAX_LENGTH + 1 + \
                                                     PACKAGE_RELATIVE_APPLICATION_ID_MAX_LENGTH)


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
