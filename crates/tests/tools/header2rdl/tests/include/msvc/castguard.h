//
// castguard.h
//
//      Copyright (c) Microsoft Corporation. All rights reserved.
//
// Declarations of CastGuard instrumentation support functions.
//

typedef void (__cdecl* __check_guard_fp)(void*);

#ifdef __cplusplus
extern "C" {
#endif

__check_guard_fp __cdecl __castguard_set_user_handler(__check_guard_fp new_handler);

#ifdef __cplusplus
} // extern "C"
#endif