/*
** 2006 June 7
**
** The author disclaims copyright to this source code.  In place of
** a legal notice, here is a blessing:
**
**    May you do good and not evil.
**    May you find forgiveness for yourself and forgive others.
**    May you share freely, never taking more than you give.
**
*************************************************************************
** This header file defines the SQLite interface for use by
** shared libraries that want to be imported as extensions into
** an SQLite instance.  Shared libraries that intend to be loaded
** as extensions by SQLite should #include this file instead of 
** winsqlite3.h.
*/
#ifndef SQLITE3EXT_H
#define SQLITE3EXT_H
#include "winsqlite3.h"

/*
** Make sure these APIs are available for Windows 10 or Greater
*/
#if NTDDI_VERSION >= NTDDI_WINTHRESHOLD

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)

/*
** The following structure holds pointers to all of the SQLite API
** routines.
**
** WARNING:  In order to maintain backwards compatibility, add new
** interfaces to the end of this structure only.  If you insert new
** interfaces in the middle of this structure, then older different
** versions of SQLite will not be able to load each other's shared
** libraries!
*/
struct sqlite3_api_routines {
  void * (SQLITE_APICALL *aggregate_context)(sqlite3_context*,int nBytes);
  int  (SQLITE_APICALL *aggregate_count)(sqlite3_context*);
  int  (SQLITE_APICALL *bind_blob)(sqlite3_stmt*,int,const void*,int n,void(SQLITE_CALLBACK *)(void*));
  int  (SQLITE_APICALL *bind_double)(sqlite3_stmt*,int,double);
  int  (SQLITE_APICALL *bind_int)(sqlite3_stmt*,int,int);
  int  (SQLITE_APICALL *bind_int64)(sqlite3_stmt*,int,sqlite_int64);
  int  (SQLITE_APICALL *bind_null)(sqlite3_stmt*,int);
  int  (SQLITE_APICALL *bind_parameter_count)(sqlite3_stmt*);
  int  (SQLITE_APICALL *bind_parameter_index)(sqlite3_stmt*,const char*zName);
  const char * (SQLITE_APICALL *bind_parameter_name)(sqlite3_stmt*,int);
  int  (SQLITE_APICALL *bind_text)(sqlite3_stmt*,int,const char*,int n,void(SQLITE_CALLBACK *)(void*));
  int  (SQLITE_APICALL *bind_text16)(sqlite3_stmt*,int,const void*,int,void(SQLITE_CALLBACK *)(void*));
  int  (SQLITE_APICALL *bind_value)(sqlite3_stmt*,int,const sqlite3_value*);
  int  (SQLITE_APICALL *busy_handler)(sqlite3*,int(SQLITE_CALLBACK *)(void*,int),void*);
  int  (SQLITE_APICALL *busy_timeout)(sqlite3*,int ms);
  int  (SQLITE_APICALL *changes)(sqlite3*);
  int  (SQLITE_APICALL *close)(sqlite3*);
  int  (SQLITE_APICALL *collation_needed)(sqlite3*,void*,void(SQLITE_CALLBACK *)(void*,sqlite3*,
                           int eTextRep,const char*));
  int  (SQLITE_APICALL *collation_needed16)(sqlite3*,void*,void(SQLITE_CALLBACK *)(void*,sqlite3*,
                             int eTextRep,const void*));
  const void * (SQLITE_APICALL *column_blob)(sqlite3_stmt*,int iCol);
  int  (SQLITE_APICALL *column_bytes)(sqlite3_stmt*,int iCol);
  int  (SQLITE_APICALL *column_bytes16)(sqlite3_stmt*,int iCol);
  int  (SQLITE_APICALL *column_count)(sqlite3_stmt*pStmt);
  const char * (SQLITE_APICALL *column_database_name)(sqlite3_stmt*,int);
  const void * (SQLITE_APICALL *column_database_name16)(sqlite3_stmt*,int);
  const char * (SQLITE_APICALL *column_decltype)(sqlite3_stmt*,int i);
  const void * (SQLITE_APICALL *column_decltype16)(sqlite3_stmt*,int);
  double  (SQLITE_APICALL *column_double)(sqlite3_stmt*,int iCol);
  int  (SQLITE_APICALL *column_int)(sqlite3_stmt*,int iCol);
  sqlite_int64  (SQLITE_APICALL *column_int64)(sqlite3_stmt*,int iCol);
  const char * (SQLITE_APICALL *column_name)(sqlite3_stmt*,int);
  const void * (SQLITE_APICALL *column_name16)(sqlite3_stmt*,int);
  const char * (SQLITE_APICALL *column_origin_name)(sqlite3_stmt*,int);
  const void * (SQLITE_APICALL *column_origin_name16)(sqlite3_stmt*,int);
  const char * (SQLITE_APICALL *column_table_name)(sqlite3_stmt*,int);
  const void * (SQLITE_APICALL *column_table_name16)(sqlite3_stmt*,int);
  const unsigned char * (SQLITE_APICALL *column_text)(sqlite3_stmt*,int iCol);
  const void * (SQLITE_APICALL *column_text16)(sqlite3_stmt*,int iCol);
  int  (SQLITE_APICALL *column_type)(sqlite3_stmt*,int iCol);
  sqlite3_value* (SQLITE_APICALL *column_value)(sqlite3_stmt*,int iCol);
  void * (SQLITE_APICALL *commit_hook)(sqlite3*,int(SQLITE_CALLBACK *)(void*),void*);
  int  (SQLITE_APICALL *complete)(const char*sql);
  int  (SQLITE_APICALL *complete16)(const void*sql);
  int  (SQLITE_APICALL *create_collation)(sqlite3*,const char*,int,void*,
                           int(SQLITE_CALLBACK *)(void*,int,const void*,int,const void*));
  int  (SQLITE_APICALL *create_collation16)(sqlite3*,const void*,int,void*,
                             int(SQLITE_CALLBACK *)(void*,int,const void*,int,const void*));
  int  (SQLITE_APICALL *create_function)(sqlite3*,const char*,int,int,void*,
                          void (SQLITE_APICALL *xFunc)(sqlite3_context*,int,sqlite3_value**),
                          void (SQLITE_APICALL *xStep)(sqlite3_context*,int,sqlite3_value**),
                          void (SQLITE_APICALL *xFinal)(sqlite3_context*));
  int  (SQLITE_APICALL *create_function16)(sqlite3*,const void*,int,int,void*,
                            void (SQLITE_APICALL *xFunc)(sqlite3_context*,int,sqlite3_value**),
                            void (SQLITE_APICALL *xStep)(sqlite3_context*,int,sqlite3_value**),
                            void (SQLITE_APICALL *xFinal)(sqlite3_context*));
  int (SQLITE_APICALL *create_module)(sqlite3*,const char*,const sqlite3_module*,void*);
  int  (SQLITE_APICALL *data_count)(sqlite3_stmt*pStmt);
  sqlite3 * (SQLITE_APICALL *db_handle)(sqlite3_stmt*);
  int (SQLITE_APICALL *declare_vtab)(sqlite3*,const char*);
  int  (SQLITE_APICALL *enable_shared_cache)(int);
  int  (SQLITE_APICALL *errcode)(sqlite3*db);
  const char * (SQLITE_APICALL *errmsg)(sqlite3*);
  const void * (SQLITE_APICALL *errmsg16)(sqlite3*);
  int  (SQLITE_APICALL *exec)(sqlite3*,const char*,sqlite3_callback,void*,char**);
  int  (SQLITE_APICALL *expired)(sqlite3_stmt*);
  int  (SQLITE_APICALL *finalize)(sqlite3_stmt*pStmt);
  void  (SQLITE_APICALL *free)(void*);
  void  (SQLITE_APICALL *free_table)(char**result);
  int  (SQLITE_APICALL *get_autocommit)(sqlite3*);
  void * (SQLITE_APICALL *get_auxdata)(sqlite3_context*,int);
  int  (SQLITE_APICALL *get_table)(sqlite3*,const char*,char***,int*,int*,char**);
  int  (SQLITE_APICALL *global_recover)(void);
  void  (SQLITE_APICALL *interruptx)(sqlite3*);
  sqlite_int64  (SQLITE_APICALL *last_insert_rowid)(sqlite3*);
  const char * (SQLITE_APICALL *libversion)(void);
  int  (SQLITE_APICALL *libversion_number)(void);
  void *(SQLITE_APICALL *malloc)(int);
  char * (SQLITE_APICALL *mprintf)(const char*,...);
  int  (SQLITE_APICALL *open)(const char*,sqlite3**);
  int  (SQLITE_APICALL *open16)(const void*,sqlite3**);
  int  (SQLITE_APICALL *prepare)(sqlite3*,const char*,int,sqlite3_stmt**,const char**);
  int  (SQLITE_APICALL *prepare16)(sqlite3*,const void*,int,sqlite3_stmt**,const void**);
  void * (SQLITE_APICALL *profile)(sqlite3*,void(SQLITE_CALLBACK *)(void*,const char*,sqlite_uint64),void*);
  void  (SQLITE_APICALL *progress_handler)(sqlite3*,int,int(SQLITE_CALLBACK *)(void*),void*);
  void *(SQLITE_APICALL *realloc)(void*,int);
  int  (SQLITE_APICALL *reset)(sqlite3_stmt*pStmt);
  void  (SQLITE_APICALL *result_blob)(sqlite3_context*,const void*,int,void(SQLITE_CALLBACK *)(void*));
  void  (SQLITE_APICALL *result_double)(sqlite3_context*,double);
  void  (SQLITE_APICALL *result_error)(sqlite3_context*,const char*,int);
  void  (SQLITE_APICALL *result_error16)(sqlite3_context*,const void*,int);
  void  (SQLITE_APICALL *result_int)(sqlite3_context*,int);
  void  (SQLITE_APICALL *result_int64)(sqlite3_context*,sqlite_int64);
  void  (SQLITE_APICALL *result_null)(sqlite3_context*);
  void  (SQLITE_APICALL *result_text)(sqlite3_context*,const char*,int,void(SQLITE_CALLBACK *)(void*));
  void  (SQLITE_APICALL *result_text16)(sqlite3_context*,const void*,int,void(SQLITE_CALLBACK *)(void*));
  void  (SQLITE_APICALL *result_text16be)(sqlite3_context*,const void*,int,void(SQLITE_CALLBACK *)(void*));
  void  (SQLITE_APICALL *result_text16le)(sqlite3_context*,const void*,int,void(SQLITE_CALLBACK *)(void*));
  void  (SQLITE_APICALL *result_value)(sqlite3_context*,sqlite3_value*);
  void * (SQLITE_APICALL *rollback_hook)(sqlite3*,void(SQLITE_CALLBACK *)(void*),void*);
  int  (SQLITE_APICALL *set_authorizer)(sqlite3*,int(SQLITE_CALLBACK *)(void*,int,const char*,const char*,
                         const char*,const char*),void*);
  void  (SQLITE_APICALL *set_auxdata)(sqlite3_context*,int,void*,void (SQLITE_CALLBACK *)(void*));
  char * (SQLITE_APICALL *xsnprintf)(int,char*,const char*,...);
  int  (SQLITE_APICALL *step)(sqlite3_stmt*);
  int  (SQLITE_APICALL *table_column_metadata)(sqlite3*,const char*,const char*,const char*,
                                char const**,char const**,int*,int*,int*);
  void  (SQLITE_APICALL *thread_cleanup)(void);
  int  (SQLITE_APICALL *total_changes)(sqlite3*);
  void * (SQLITE_APICALL *trace)(sqlite3*,void(SQLITE_APICALL *xTrace)(void*,const char*),void*);
  int  (SQLITE_APICALL *transfer_bindings)(sqlite3_stmt*,sqlite3_stmt*);
  void * (SQLITE_APICALL *update_hook)(sqlite3*,void(SQLITE_CALLBACK *)(void*,int ,char const*,char const*,
                                         sqlite_int64),void*);
  void * (SQLITE_APICALL *user_data)(sqlite3_context*);
  const void * (SQLITE_APICALL *value_blob)(sqlite3_value*);
  int  (SQLITE_APICALL *value_bytes)(sqlite3_value*);
  int  (SQLITE_APICALL *value_bytes16)(sqlite3_value*);
  double  (SQLITE_APICALL *value_double)(sqlite3_value*);
  int  (SQLITE_APICALL *value_int)(sqlite3_value*);
  sqlite_int64  (SQLITE_APICALL *value_int64)(sqlite3_value*);
  int  (SQLITE_APICALL *value_numeric_type)(sqlite3_value*);
  const unsigned char * (SQLITE_APICALL *value_text)(sqlite3_value*);
  const void * (SQLITE_APICALL *value_text16)(sqlite3_value*);
  const void * (SQLITE_APICALL *value_text16be)(sqlite3_value*);
  const void * (SQLITE_APICALL *value_text16le)(sqlite3_value*);
  int  (SQLITE_APICALL *value_type)(sqlite3_value*);
  char *(SQLITE_APICALL *vmprintf)(const char*,va_list);
  /* Added ??? */
  int (SQLITE_APICALL *overload_function)(sqlite3*, const char *zFuncName, int nArg);
  /* Added by 3.3.13 */
  int (SQLITE_APICALL *prepare_v2)(sqlite3*,const char*,int,sqlite3_stmt**,const char**);
  int (SQLITE_APICALL *prepare16_v2)(sqlite3*,const void*,int,sqlite3_stmt**,const void**);
  int (SQLITE_APICALL *clear_bindings)(sqlite3_stmt*);
  /* Added by 3.4.1 */
  int (SQLITE_APICALL *create_module_v2)(sqlite3*,const char*,const sqlite3_module*,void*,
                          void (SQLITE_APICALL *xDestroy)(void *));
  /* Added by 3.5.0 */
  int (SQLITE_APICALL *bind_zeroblob)(sqlite3_stmt*,int,int);
  int (SQLITE_APICALL *blob_bytes)(sqlite3_blob*);
  int (SQLITE_APICALL *blob_close)(sqlite3_blob*);
  int (SQLITE_APICALL *blob_open)(sqlite3*,const char*,const char*,const char*,sqlite3_int64,
                   int,sqlite3_blob**);
  int (SQLITE_APICALL *blob_read)(sqlite3_blob*,void*,int,int);
  int (SQLITE_APICALL *blob_write)(sqlite3_blob*,const void*,int,int);
  int (SQLITE_APICALL *create_collation_v2)(sqlite3*,const char*,int,void*,
                             int(SQLITE_CALLBACK *)(void*,int,const void*,int,const void*),
                             void(SQLITE_CALLBACK *)(void*));
  int (SQLITE_APICALL *file_control)(sqlite3*,const char*,int,void*);
  sqlite3_int64 (SQLITE_APICALL *memory_highwater)(int);
  sqlite3_int64 (SQLITE_APICALL *memory_used)(void);
  sqlite3_mutex *(SQLITE_APICALL *mutex_alloc)(int);
  void (SQLITE_APICALL *mutex_enter)(sqlite3_mutex*);
  void (SQLITE_APICALL *mutex_free)(sqlite3_mutex*);
  void (SQLITE_APICALL *mutex_leave)(sqlite3_mutex*);
  int (SQLITE_APICALL *mutex_try)(sqlite3_mutex*);
  int (SQLITE_APICALL *open_v2)(const char*,sqlite3**,int,const char*);
  int (SQLITE_APICALL *release_memory)(int);
  void (SQLITE_APICALL *result_error_nomem)(sqlite3_context*);
  void (SQLITE_APICALL *result_error_toobig)(sqlite3_context*);
  int (SQLITE_APICALL *sleep)(int);
  void (SQLITE_APICALL *soft_heap_limit)(int);
  sqlite3_vfs *(SQLITE_APICALL *vfs_find)(const char*);
  int (SQLITE_APICALL *vfs_register)(sqlite3_vfs*,int);
  int (SQLITE_APICALL *vfs_unregister)(sqlite3_vfs*);
  int (SQLITE_APICALL *xthreadsafe)(void);
  void (SQLITE_APICALL *result_zeroblob)(sqlite3_context*,int);
  void (SQLITE_APICALL *result_error_code)(sqlite3_context*,int);
  int (SQLITE_APICALL *test_control)(int, ...);
  void (SQLITE_APICALL *randomness)(int,void*);
  sqlite3 *(SQLITE_APICALL *context_db_handle)(sqlite3_context*);
  int (SQLITE_APICALL *extended_result_codes)(sqlite3*,int);
  int (SQLITE_APICALL *limit)(sqlite3*,int,int);
  sqlite3_stmt *(SQLITE_APICALL *next_stmt)(sqlite3*,sqlite3_stmt*);
  const char *(SQLITE_APICALL *sql)(sqlite3_stmt*);
  int (SQLITE_APICALL *status)(int,int*,int*,int);
  int (SQLITE_APICALL *backup_finish)(sqlite3_backup*);
  sqlite3_backup *(SQLITE_APICALL *backup_init)(sqlite3*,const char*,sqlite3*,const char*);
  int (SQLITE_APICALL *backup_pagecount)(sqlite3_backup*);
  int (SQLITE_APICALL *backup_remaining)(sqlite3_backup*);
  int (SQLITE_APICALL *backup_step)(sqlite3_backup*,int);
  const char *(SQLITE_APICALL *compileoption_get)(int);
  int (SQLITE_APICALL *compileoption_used)(const char*);
  int (SQLITE_APICALL *create_function_v2)(sqlite3*,const char*,int,int,void*,
                            void (SQLITE_APICALL *xFunc)(sqlite3_context*,int,sqlite3_value**),
                            void (SQLITE_APICALL *xStep)(sqlite3_context*,int,sqlite3_value**),
                            void (SQLITE_APICALL *xFinal)(sqlite3_context*),
                            void(SQLITE_APICALL *xDestroy)(void*));
  int (SQLITE_APICALL *db_config)(sqlite3*,int,...);
  sqlite3_mutex *(SQLITE_APICALL *db_mutex)(sqlite3*);
  int (SQLITE_APICALL *db_status)(sqlite3*,int,int*,int*,int);
  int (SQLITE_APICALL *extended_errcode)(sqlite3*);
  void (SQLITE_APICALL *log)(int,const char*,...);
  sqlite3_int64 (SQLITE_APICALL *soft_heap_limit64)(sqlite3_int64);
  const char *(SQLITE_APICALL *sourceid)(void);
  int (SQLITE_APICALL *stmt_status)(sqlite3_stmt*,int,int);
  int (SQLITE_APICALL *strnicmp)(const char*,const char*,int);
  int (SQLITE_APICALL *unlock_notify)(sqlite3*,void(SQLITE_CALLBACK *)(void**,int),void*);
  int (SQLITE_APICALL *wal_autocheckpoint)(sqlite3*,int);
  int (SQLITE_APICALL *wal_checkpoint)(sqlite3*,const char*);
  void *(SQLITE_APICALL *wal_hook)(sqlite3*,int(SQLITE_CALLBACK *)(void*,sqlite3*,const char*,int),void*);
  int (SQLITE_APICALL *blob_reopen)(sqlite3_blob*,sqlite3_int64);
  int (SQLITE_APICALL *vtab_config)(sqlite3*,int op,...);
  int (SQLITE_APICALL *vtab_on_conflict)(sqlite3*);
  /* Version 3.7.16 and later */
  int (SQLITE_APICALL *close_v2)(sqlite3*);
  const char *(SQLITE_APICALL *db_filename)(sqlite3*,const char*);
  int (SQLITE_APICALL *db_readonly)(sqlite3*,const char*);
  int (SQLITE_APICALL *db_release_memory)(sqlite3*);
  const char *(SQLITE_APICALL *errstr)(int);
  int (SQLITE_APICALL *stmt_busy)(sqlite3_stmt*);
  int (SQLITE_APICALL *stmt_readonly)(sqlite3_stmt*);
  int (SQLITE_APICALL *stricmp)(const char*,const char*);
  int (SQLITE_APICALL *uri_boolean)(const char*,const char*,int);
  sqlite3_int64 (SQLITE_APICALL *uri_int64)(const char*,const char*,sqlite3_int64);
  const char *(SQLITE_APICALL *uri_parameter)(const char*,const char*);
  char *(SQLITE_APICALL *xvsnprintf)(int,char*,const char*,va_list);
  int (SQLITE_APICALL *wal_checkpoint_v2)(sqlite3*,const char*,int,int*,int*);
  /* Version 3.8.7 and later */
  int (SQLITE_APICALL *auto_extension)(void(SQLITE_CALLBACK *)(void));
  int (SQLITE_APICALL *bind_blob64)(sqlite3_stmt*,int,const void*,sqlite3_uint64,
                     void(SQLITE_CALLBACK *)(void*));
  int (SQLITE_APICALL *bind_text64)(sqlite3_stmt*,int,const char*,sqlite3_uint64,
                      void(SQLITE_CALLBACK *)(void*),unsigned char);
  int (SQLITE_APICALL *cancel_auto_extension)(void(SQLITE_CALLBACK *)(void));
  int (SQLITE_APICALL *load_extension)(sqlite3*,const char*,const char*,char**);
  void *(SQLITE_APICALL *malloc64)(sqlite3_uint64);
  sqlite3_uint64 (SQLITE_APICALL *msize)(void*);
  void *(SQLITE_APICALL *realloc64)(void*,sqlite3_uint64);
  void (SQLITE_APICALL *reset_auto_extension)(void);
  void (SQLITE_APICALL *result_blob64)(sqlite3_context*,const void*,sqlite3_uint64,
                        void(SQLITE_CALLBACK *)(void*));
  void (SQLITE_APICALL *result_text64)(sqlite3_context*,const char*,sqlite3_uint64,
                         void(SQLITE_CALLBACK *)(void*), unsigned char);
  int (SQLITE_APICALL *strglob)(const char*,const char*);
#if NTDDI_VERSION >= NTDDI_WIN10_RS1
  /* Version 3.8.11 and later */
  sqlite3_value *(SQLITE_APICALL *value_dup)(const sqlite3_value*);
  void (SQLITE_APICALL *value_free)(sqlite3_value*);
  int (SQLITE_APICALL *result_zeroblob64)(sqlite3_context*,sqlite3_uint64);
  int (SQLITE_APICALL *bind_zeroblob64)(sqlite3_stmt*, int, sqlite3_uint64);
  /* Version 3.9.0 and later */
  unsigned int (SQLITE_APICALL *value_subtype)(sqlite3_value*);
  void (SQLITE_APICALL *result_subtype)(sqlite3_context*,unsigned int);
  /* Version 3.10.0 and later */
  int (SQLITE_APICALL *status64)(int,sqlite3_int64*,sqlite3_int64*,int);
  int (SQLITE_APICALL *strlike)(const char*,const char*,unsigned int);
  int (SQLITE_APICALL *db_cacheflush)(sqlite3*);
  /* Version 3.12.0 and later */
  int (SQLITE_APICALL *system_errno)(sqlite3*);
#endif /* NTDDI_VERSION >= NTDDI_WIN10_RS1 */
#if NTDDI_VERSION >= NTDDI_WIN10_RS2
  /* Version 3.14.0 and later */
  int (SQLITE_APICALL *trace_v2)(sqlite3*,unsigned,int(SQLITE_CALLBACK *)(unsigned,void*,void*,void*),void*);
  char *(SQLITE_APICALL *expanded_sql)(sqlite3_stmt*);
#endif /* NTDDI_VERSION >= NTDDI_WIN10_RS2 */
#if NTDDI_VERSION >= NTDDI_WIN10_RS3
  /* Version 3.18.0 and later */
  void (SQLITE_APICALL *set_last_insert_rowid)(sqlite3*,sqlite3_int64);
#endif /* NTDDI_VERSION >= NTDDI_WIN10_RS3 */
#if NTDDI_VERSION >= NTDDI_WIN10_RS4
  /* Version 3.20.0 and later */
  int (SQLITE_APICALL *prepare_v3)(sqlite3*,const char*,int,unsigned int,
                    sqlite3_stmt**,const char**);
  int (SQLITE_APICALL *prepare16_v3)(sqlite3*,const void*,int,unsigned int,
                      sqlite3_stmt**,const void**);
  int (SQLITE_APICALL *bind_pointer)(sqlite3_stmt*,int,void*,const char*,void(SQLITE_CALLBACK *)(void*));
  void (SQLITE_APICALL *result_pointer)(sqlite3_context*,void*,const char*,void(SQLITE_CALLBACK *)(void*));
  void *(SQLITE_APICALL *value_pointer)(sqlite3_value*,const char*);
#endif /* NTDDI_VERSION >= NTDDI_WIN10_RS4 */
#if NTDDI_VERSION >= NTDDI_WIN10_RS5
  int (SQLITE_APICALL *vtab_nochange)(sqlite3_context*);
  int (SQLITE_APICALL *value_nochange)(sqlite3_value*);
  const char *(SQLITE_APICALL *vtab_collation)(sqlite3_index_info*,int);
#endif /* NTDDI_VERSION >= NTDDI_WIN10_RS5 */
#if NTDDI_VERSION >= NTDDI_WIN10_19H1
  /* Version 3.24.0 and later */
  int (SQLITE_APICALL *keyword_count)(void);
  int (SQLITE_APICALL *keyword_name)(int,const char**,int*);
  int (SQLITE_APICALL *keyword_check)(const char*,int);
  sqlite3_str *(SQLITE_APICALL *str_new)(sqlite3*);
  char *(SQLITE_APICALL *str_finish)(sqlite3_str*);
  void (SQLITE_APICALL *str_appendf)(sqlite3_str*, const char *zFormat, ...);
  void (SQLITE_APICALL *str_vappendf)(sqlite3_str*, const char *zFormat, va_list);
  void (SQLITE_APICALL *str_append)(sqlite3_str*, const char *zIn, int N);
  void (SQLITE_APICALL *str_appendall)(sqlite3_str*, const char *zIn);
  void (SQLITE_APICALL *str_appendchar)(sqlite3_str*, int N, char C);
  void (SQLITE_APICALL *str_reset)(sqlite3_str*);
  int (SQLITE_APICALL *str_errcode)(sqlite3_str*);
  int (SQLITE_APICALL *str_length)(sqlite3_str*);
  char *(SQLITE_APICALL *str_value)(sqlite3_str*);
  /* Version 3.25.0 and later */
  int (SQLITE_APICALL *create_window_function)(sqlite3*,const char*,int,int,void*,
                            void (SQLITE_APICALL *xStep)(sqlite3_context*,int,sqlite3_value**),
                            void (SQLITE_APICALL *xFinal)(sqlite3_context*),
                            void (SQLITE_APICALL *xValue)(sqlite3_context*),
                            void (SQLITE_APICALL *xInv)(sqlite3_context*,int,sqlite3_value**),
                            void(SQLITE_APICALL *xDestroy)(void*));
#endif /* NTDDI_VERSION >= NTDDI_WIN10_19H1 */
#if NTDDI_VERSION >= NTDDI_WIN10_VB
  /* Version 3.26.0 and later */
  const char *(SQLITE_APICALL *normalized_sql)(sqlite3_stmt*);
  /* Version 3.28.0 and later */
  int (SQLITE_APICALL *stmt_isexplain)(sqlite3_stmt*);
  int (SQLITE_APICALL *value_frombind)(sqlite3_value*);
#endif /* NTDDI_VERSION >= NTDDI_WIN10_VB */
#if NTDDI_VERSION >= NTDDI_WIN10_CO
  /* Version 3.30.0 and later */
  int (SQLITE_APICALL *drop_modules)(sqlite3*,const char**);
  /* Version 3.31.0 and later */
  sqlite3_int64 (SQLITE_APICALL *hard_heap_limit64)(sqlite3_int64);
  const char *(SQLITE_APICALL *uri_key)(const char*,int);
  const char *(SQLITE_APICALL *filename_database)(const char*);
  const char *(SQLITE_APICALL *filename_journal)(const char*);
  const char *(SQLITE_APICALL *filename_wal)(const char*);
  /* Version 3.32.0 and later */
  const char *(SQLITE_APICALL *create_filename)(const char*,const char*,const char*,
                           int,const char**);
  void (SQLITE_APICALL *free_filename)(const char*);
  sqlite3_file *(SQLITE_APICALL *database_file_object)(const char*);
  /* Version 3.34.0 and later */
  int (SQLITE_APICALL *txn_state)(sqlite3*,const char*);
#endif /* NTDDI_VERSION >= NTDDI_WIN10_CO */
#if NTDDI_VERSION >= NTDDI_WIN11_ZN
  /* Version 3.36.1 and later */
  sqlite3_int64 (SQLITE_APICALL *changes64)(sqlite3*);
  sqlite3_int64 (SQLITE_APICALL *total_changes64)(sqlite3*);
  /* Version 3.37.0 and later */
  int (SQLITE_APICALL *autovacuum_pages)(sqlite3*,
     unsigned int(SQLITE_CALLBACK *)(void*,const char*,unsigned int,unsigned int,unsigned int),
     void*, void(SQLITE_CALLBACK *)(void*));
  /* Version 3.38.0 and later */
  int (SQLITE_APICALL *error_offset)(sqlite3*);
  int (SQLITE_APICALL *vtab_rhs_value)(sqlite3_index_info*,int,sqlite3_value**);
  int (SQLITE_APICALL *vtab_distinct)(sqlite3_index_info*);
  int (SQLITE_APICALL *vtab_in)(sqlite3_index_info*,int,int);
  int (SQLITE_APICALL *vtab_in_first)(sqlite3_value*,sqlite3_value**);
  int (SQLITE_APICALL *vtab_in_next)(sqlite3_value*,sqlite3_value**);
  /* Version 3.39.0 and later */
  int (SQLITE_APICALL *deserialize)(sqlite3*,const char*,unsigned char*,
                     sqlite3_int64,sqlite3_int64,unsigned);
  unsigned char *(SQLITE_APICALL *serialize)(sqlite3*,const char *,sqlite3_int64*,
                              unsigned int);
  const char *(SQLITE_APICALL *db_name)(sqlite3*,int);
  /* Version 3.40.0 and later */
  int (SQLITE_APICALL *value_encoding)(sqlite3_value*);
#endif /* NTDDI_VERSION >= NTDDI_WIN11_ZN */
#if NTDDI_VERSION >= NTDDI_WIN11_GE
  int (SQLITE_APICALL *is_interrupted)(sqlite3*);
  /* Version 3.43.0 and later */
  int (SQLITE_APICALL *stmt_explain)(sqlite3_stmt*,int);
  /* Version 3.44.0 and later */
  void *(SQLITE_APICALL *get_clientdata)(sqlite3*,const char*);
  int (SQLITE_APICALL *set_clientdata)(sqlite3*, const char*, void*, void(SQLITE_CALLBACK *)(void*));
  /* Version 3.50.0 and later */
  int (SQLITE_APICALL *setlk_timeout)(sqlite3*,int,int);
  /* Version 3.51.0 and later */
  int (SQLITE_APICALL *set_errmsg)(sqlite3*,int,const char*);
  int (SQLITE_APICALL *db_status64)(sqlite3*,int,sqlite3_int64*,sqlite3_int64*,int);
#endif /* NTDDI_VERSION >= NTDDI_WIN11_GE */
};

#if NTDDI_VERSION >= NTDDI_WIN10_RS2
/*
** This is the function signature used for all extension entry points.  It
** is also defined in the file "loadext.c".
*/
typedef int (*sqlite3_loadext_entry)(
  sqlite3 *db,                       /* Handle to the database. */
  char **pzErrMsg,                   /* Used to set error string on failure. */
  const sqlite3_api_routines *pThunk /* Extension API function pointers. */
);
#endif /* NTDDI_VERSION >= NTDDI_WIN10_RS2 */

/*
** The following macros redefine the API routines so that they are
** redirected through the global sqlite3_api structure.
**
** This header file is also used by the loadext.c source file
** (part of the main SQLite library - not an extension) so that
** it can get access to the sqlite3_api_routines structure
** definition.  But the main library does not want to redefine
** the API.  So the redefinition macros are only valid if the
** SQLITE_CORE macros is undefined.
*/
#if !defined(SQLITE_CORE) && !defined(SQLITE_OMIT_LOAD_EXTENSION)
#define sqlite3_aggregate_context      sqlite3_api->aggregate_context
#ifndef SQLITE_OMIT_DEPRECATED
#define sqlite3_aggregate_count        sqlite3_api->aggregate_count
#endif
#define sqlite3_bind_blob              sqlite3_api->bind_blob
#define sqlite3_bind_double            sqlite3_api->bind_double
#define sqlite3_bind_int               sqlite3_api->bind_int
#define sqlite3_bind_int64             sqlite3_api->bind_int64
#define sqlite3_bind_null              sqlite3_api->bind_null
#define sqlite3_bind_parameter_count   sqlite3_api->bind_parameter_count
#define sqlite3_bind_parameter_index   sqlite3_api->bind_parameter_index
#define sqlite3_bind_parameter_name    sqlite3_api->bind_parameter_name
#define sqlite3_bind_text              sqlite3_api->bind_text
#define sqlite3_bind_text16            sqlite3_api->bind_text16
#define sqlite3_bind_value             sqlite3_api->bind_value
#define sqlite3_busy_handler           sqlite3_api->busy_handler
#define sqlite3_busy_timeout           sqlite3_api->busy_timeout
#define sqlite3_changes                sqlite3_api->changes
#define sqlite3_close                  sqlite3_api->close
#define sqlite3_collation_needed       sqlite3_api->collation_needed
#define sqlite3_collation_needed16     sqlite3_api->collation_needed16
#define sqlite3_column_blob            sqlite3_api->column_blob
#define sqlite3_column_bytes           sqlite3_api->column_bytes
#define sqlite3_column_bytes16         sqlite3_api->column_bytes16
#define sqlite3_column_count           sqlite3_api->column_count
#define sqlite3_column_database_name   sqlite3_api->column_database_name
#define sqlite3_column_database_name16 sqlite3_api->column_database_name16
#define sqlite3_column_decltype        sqlite3_api->column_decltype
#define sqlite3_column_decltype16      sqlite3_api->column_decltype16
#define sqlite3_column_double          sqlite3_api->column_double
#define sqlite3_column_int             sqlite3_api->column_int
#define sqlite3_column_int64           sqlite3_api->column_int64
#define sqlite3_column_name            sqlite3_api->column_name
#define sqlite3_column_name16          sqlite3_api->column_name16
#define sqlite3_column_origin_name     sqlite3_api->column_origin_name
#define sqlite3_column_origin_name16   sqlite3_api->column_origin_name16
#define sqlite3_column_table_name      sqlite3_api->column_table_name
#define sqlite3_column_table_name16    sqlite3_api->column_table_name16
#define sqlite3_column_text            sqlite3_api->column_text
#define sqlite3_column_text16          sqlite3_api->column_text16
#define sqlite3_column_type            sqlite3_api->column_type
#define sqlite3_column_value           sqlite3_api->column_value
#define sqlite3_commit_hook            sqlite3_api->commit_hook
#define sqlite3_complete               sqlite3_api->complete
#define sqlite3_complete16             sqlite3_api->complete16
#define sqlite3_create_collation       sqlite3_api->create_collation
#define sqlite3_create_collation16     sqlite3_api->create_collation16
#define sqlite3_create_function        sqlite3_api->create_function
#define sqlite3_create_function16      sqlite3_api->create_function16
#define sqlite3_create_module          sqlite3_api->create_module
#define sqlite3_create_module_v2       sqlite3_api->create_module_v2
#define sqlite3_data_count             sqlite3_api->data_count
#define sqlite3_db_handle              sqlite3_api->db_handle
#define sqlite3_declare_vtab           sqlite3_api->declare_vtab
#define sqlite3_enable_shared_cache    sqlite3_api->enable_shared_cache
#define sqlite3_errcode                sqlite3_api->errcode
#define sqlite3_errmsg                 sqlite3_api->errmsg
#define sqlite3_errmsg16               sqlite3_api->errmsg16
#define sqlite3_exec                   sqlite3_api->exec
#ifndef SQLITE_OMIT_DEPRECATED
#define sqlite3_expired                sqlite3_api->expired
#endif
#define sqlite3_finalize               sqlite3_api->finalize
#define sqlite3_free                   sqlite3_api->free
#define sqlite3_free_table             sqlite3_api->free_table
#define sqlite3_get_autocommit         sqlite3_api->get_autocommit
#define sqlite3_get_auxdata            sqlite3_api->get_auxdata
#define sqlite3_get_table              sqlite3_api->get_table
#ifndef SQLITE_OMIT_DEPRECATED
#define sqlite3_global_recover         sqlite3_api->global_recover
#endif
#define sqlite3_interrupt              sqlite3_api->interruptx
#define sqlite3_last_insert_rowid      sqlite3_api->last_insert_rowid
#define sqlite3_libversion             sqlite3_api->libversion
#define sqlite3_libversion_number      sqlite3_api->libversion_number
#define sqlite3_malloc                 sqlite3_api->malloc
#define sqlite3_mprintf                sqlite3_api->mprintf
#define sqlite3_open                   sqlite3_api->open
#define sqlite3_open16                 sqlite3_api->open16
#define sqlite3_prepare                sqlite3_api->prepare
#define sqlite3_prepare16              sqlite3_api->prepare16
#define sqlite3_prepare_v2             sqlite3_api->prepare_v2
#define sqlite3_prepare16_v2           sqlite3_api->prepare16_v2
#define sqlite3_profile                sqlite3_api->profile
#define sqlite3_progress_handler       sqlite3_api->progress_handler
#define sqlite3_realloc                sqlite3_api->realloc
#define sqlite3_reset                  sqlite3_api->reset
#define sqlite3_result_blob            sqlite3_api->result_blob
#define sqlite3_result_double          sqlite3_api->result_double
#define sqlite3_result_error           sqlite3_api->result_error
#define sqlite3_result_error16         sqlite3_api->result_error16
#define sqlite3_result_int             sqlite3_api->result_int
#define sqlite3_result_int64           sqlite3_api->result_int64
#define sqlite3_result_null            sqlite3_api->result_null
#define sqlite3_result_text            sqlite3_api->result_text
#define sqlite3_result_text16          sqlite3_api->result_text16
#define sqlite3_result_text16be        sqlite3_api->result_text16be
#define sqlite3_result_text16le        sqlite3_api->result_text16le
#define sqlite3_result_value           sqlite3_api->result_value
#define sqlite3_rollback_hook          sqlite3_api->rollback_hook
#define sqlite3_set_authorizer         sqlite3_api->set_authorizer
#define sqlite3_set_auxdata            sqlite3_api->set_auxdata
#define sqlite3_snprintf               sqlite3_api->xsnprintf
#define sqlite3_step                   sqlite3_api->step
#define sqlite3_table_column_metadata  sqlite3_api->table_column_metadata
#define sqlite3_thread_cleanup         sqlite3_api->thread_cleanup
#define sqlite3_total_changes          sqlite3_api->total_changes
#define sqlite3_trace                  sqlite3_api->trace
#ifndef SQLITE_OMIT_DEPRECATED
#define sqlite3_transfer_bindings      sqlite3_api->transfer_bindings
#endif
#define sqlite3_update_hook            sqlite3_api->update_hook
#define sqlite3_user_data              sqlite3_api->user_data
#define sqlite3_value_blob             sqlite3_api->value_blob
#define sqlite3_value_bytes            sqlite3_api->value_bytes
#define sqlite3_value_bytes16          sqlite3_api->value_bytes16
#define sqlite3_value_double           sqlite3_api->value_double
#define sqlite3_value_int              sqlite3_api->value_int
#define sqlite3_value_int64            sqlite3_api->value_int64
#define sqlite3_value_numeric_type     sqlite3_api->value_numeric_type
#define sqlite3_value_text             sqlite3_api->value_text
#define sqlite3_value_text16           sqlite3_api->value_text16
#define sqlite3_value_text16be         sqlite3_api->value_text16be
#define sqlite3_value_text16le         sqlite3_api->value_text16le
#define sqlite3_value_type             sqlite3_api->value_type
#define sqlite3_vmprintf               sqlite3_api->vmprintf
#if NTDDI_VERSION >= NTDDI_WIN10_RS1
#define sqlite3_vsnprintf              sqlite3_api->xvsnprintf
#endif /* NTDDI_VERSION >= NTDDI_WIN10_RS1 */
#define sqlite3_overload_function      sqlite3_api->overload_function
#define sqlite3_prepare_v2             sqlite3_api->prepare_v2
#define sqlite3_prepare16_v2           sqlite3_api->prepare16_v2
#define sqlite3_clear_bindings         sqlite3_api->clear_bindings
#define sqlite3_bind_zeroblob          sqlite3_api->bind_zeroblob
#define sqlite3_blob_bytes             sqlite3_api->blob_bytes
#define sqlite3_blob_close             sqlite3_api->blob_close
#define sqlite3_blob_open              sqlite3_api->blob_open
#define sqlite3_blob_read              sqlite3_api->blob_read
#define sqlite3_blob_write             sqlite3_api->blob_write
#define sqlite3_create_collation_v2    sqlite3_api->create_collation_v2
#define sqlite3_file_control           sqlite3_api->file_control
#define sqlite3_memory_highwater       sqlite3_api->memory_highwater
#define sqlite3_memory_used            sqlite3_api->memory_used
#define sqlite3_mutex_alloc            sqlite3_api->mutex_alloc
#define sqlite3_mutex_enter            sqlite3_api->mutex_enter
#define sqlite3_mutex_free             sqlite3_api->mutex_free
#define sqlite3_mutex_leave            sqlite3_api->mutex_leave
#define sqlite3_mutex_try              sqlite3_api->mutex_try
#define sqlite3_open_v2                sqlite3_api->open_v2
#define sqlite3_release_memory         sqlite3_api->release_memory
#define sqlite3_result_error_nomem     sqlite3_api->result_error_nomem
#define sqlite3_result_error_toobig    sqlite3_api->result_error_toobig
#define sqlite3_sleep                  sqlite3_api->sleep
#define sqlite3_soft_heap_limit        sqlite3_api->soft_heap_limit
#define sqlite3_vfs_find               sqlite3_api->vfs_find
#define sqlite3_vfs_register           sqlite3_api->vfs_register
#define sqlite3_vfs_unregister         sqlite3_api->vfs_unregister
#define sqlite3_threadsafe             sqlite3_api->xthreadsafe
#define sqlite3_result_zeroblob        sqlite3_api->result_zeroblob
#define sqlite3_result_error_code      sqlite3_api->result_error_code
#define sqlite3_test_control           sqlite3_api->test_control
#define sqlite3_randomness             sqlite3_api->randomness
#define sqlite3_context_db_handle      sqlite3_api->context_db_handle
#define sqlite3_extended_result_codes  sqlite3_api->extended_result_codes
#define sqlite3_limit                  sqlite3_api->limit
#define sqlite3_next_stmt              sqlite3_api->next_stmt
#define sqlite3_sql                    sqlite3_api->sql
#define sqlite3_status                 sqlite3_api->status
#define sqlite3_backup_finish          sqlite3_api->backup_finish
#define sqlite3_backup_init            sqlite3_api->backup_init
#define sqlite3_backup_pagecount       sqlite3_api->backup_pagecount
#define sqlite3_backup_remaining       sqlite3_api->backup_remaining
#define sqlite3_backup_step            sqlite3_api->backup_step
#define sqlite3_compileoption_get      sqlite3_api->compileoption_get
#define sqlite3_compileoption_used     sqlite3_api->compileoption_used
#define sqlite3_create_function_v2     sqlite3_api->create_function_v2
#define sqlite3_db_config              sqlite3_api->db_config
#define sqlite3_db_mutex               sqlite3_api->db_mutex
#define sqlite3_db_status              sqlite3_api->db_status
#define sqlite3_extended_errcode       sqlite3_api->extended_errcode
#define sqlite3_log                    sqlite3_api->log
#define sqlite3_soft_heap_limit64      sqlite3_api->soft_heap_limit64
#define sqlite3_sourceid               sqlite3_api->sourceid
#define sqlite3_stmt_status            sqlite3_api->stmt_status
#define sqlite3_strnicmp               sqlite3_api->strnicmp
#define sqlite3_unlock_notify          sqlite3_api->unlock_notify
#define sqlite3_wal_autocheckpoint     sqlite3_api->wal_autocheckpoint
#define sqlite3_wal_checkpoint         sqlite3_api->wal_checkpoint
#define sqlite3_wal_hook               sqlite3_api->wal_hook
#define sqlite3_blob_reopen            sqlite3_api->blob_reopen
#define sqlite3_vtab_config            sqlite3_api->vtab_config
#define sqlite3_vtab_on_conflict       sqlite3_api->vtab_on_conflict
/* Version 3.7.16 and later */
#define sqlite3_close_v2               sqlite3_api->close_v2
#define sqlite3_db_filename            sqlite3_api->db_filename
#define sqlite3_db_readonly            sqlite3_api->db_readonly
#define sqlite3_db_release_memory      sqlite3_api->db_release_memory
#define sqlite3_errstr                 sqlite3_api->errstr
#define sqlite3_stmt_busy              sqlite3_api->stmt_busy
#define sqlite3_stmt_readonly          sqlite3_api->stmt_readonly
#define sqlite3_stricmp                sqlite3_api->stricmp
#define sqlite3_uri_boolean            sqlite3_api->uri_boolean
#define sqlite3_uri_int64              sqlite3_api->uri_int64
#define sqlite3_uri_parameter          sqlite3_api->uri_parameter
#define sqlite3_uri_vsnprintf          sqlite3_api->xvsnprintf
#define sqlite3_wal_checkpoint_v2      sqlite3_api->wal_checkpoint_v2
/* Version 3.8.7 and later */
#define sqlite3_auto_extension         sqlite3_api->auto_extension
#define sqlite3_bind_blob64            sqlite3_api->bind_blob64
#define sqlite3_bind_text64            sqlite3_api->bind_text64
#define sqlite3_cancel_auto_extension  sqlite3_api->cancel_auto_extension
#define sqlite3_load_extension         sqlite3_api->load_extension
#define sqlite3_malloc64               sqlite3_api->malloc64
#define sqlite3_msize                  sqlite3_api->msize
#define sqlite3_realloc64              sqlite3_api->realloc64
#define sqlite3_reset_auto_extension   sqlite3_api->reset_auto_extension
#define sqlite3_result_blob64          sqlite3_api->result_blob64
#define sqlite3_result_text64          sqlite3_api->result_text64
#define sqlite3_strglob                sqlite3_api->strglob
#if NTDDI_VERSION >= NTDDI_WIN10_RS1
/* Version 3.8.11 and later */
#define sqlite3_value_dup              sqlite3_api->value_dup
#define sqlite3_value_free             sqlite3_api->value_free
#define sqlite3_result_zeroblob64      sqlite3_api->result_zeroblob64
#define sqlite3_bind_zeroblob64        sqlite3_api->bind_zeroblob64
/* Version 3.9.0 and later */
#define sqlite3_value_subtype          sqlite3_api->value_subtype
#define sqlite3_result_subtype         sqlite3_api->result_subtype
/* Version 3.10.0 and later */
#define sqlite3_status64               sqlite3_api->status64
#define sqlite3_strlike                sqlite3_api->strlike
#define sqlite3_db_cacheflush          sqlite3_api->db_cacheflush
/* Version 3.12.0 and later */
#define sqlite3_system_errno           sqlite3_api->system_errno
#endif /* NTDDI_VERSION >= NTDDI_WIN10_RS1 */
#if NTDDI_VERSION >= NTDDI_WIN10_RS2
/* Version 3.14.0 and later */
#define sqlite3_trace_v2               sqlite3_api->trace_v2
#define sqlite3_expanded_sql           sqlite3_api->expanded_sql
#endif /* NTDDI_VERSION >= NTDDI_WIN10_RS2 */
#if NTDDI_VERSION >= NTDDI_WIN10_RS3
/* Version 3.18.0 and later */
#define sqlite3_set_last_insert_rowid  sqlite3_api->set_last_insert_rowid
#endif /* NTDDI_VERSION >= NTDDI_WIN10_RS3 */
#if NTDDI_VERSION >= NTDDI_WIN10_RS4
/* Version 3.20.0 and later */
#define sqlite3_prepare_v3             sqlite3_api->prepare_v3
#define sqlite3_prepare16_v3           sqlite3_api->prepare16_v3
#define sqlite3_bind_pointer           sqlite3_api->bind_pointer
#define sqlite3_result_pointer         sqlite3_api->result_pointer
#define sqlite3_value_pointer          sqlite3_api->value_pointer
#endif /* NTDDI_VERSION >= NTDDI_WIN10_RS4 */
#if NTDDI_VERSION >= NTDDI_WIN10_RS5
/* Version 3.22.0 and later */
#define sqlite3_vtab_nochange          sqlite3_api->vtab_nochange
#define sqlite3_value_nochange         sqlite3_api->value_nochange
#define sqlite3_vtab_collation         sqlite3_api->vtab_collation
#endif /* NTDDI_VERSION >= NTDDI_WIN10_RS5 */
#if NTDDI_VERSION >= NTDDI_WIN10_19H1
/* Version 3.24.0 and later */
#define sqlite3_keyword_count          sqlite3_api->keyword_count
#define sqlite3_keyword_name           sqlite3_api->keyword_name
#define sqlite3_keyword_check          sqlite3_api->keyword_check
#define sqlite3_str_new                sqlite3_api->str_new
#define sqlite3_str_finish             sqlite3_api->str_finish
#define sqlite3_str_appendf            sqlite3_api->str_appendf
#define sqlite3_str_vappendf           sqlite3_api->str_vappendf
#define sqlite3_str_append             sqlite3_api->str_append
#define sqlite3_str_appendall          sqlite3_api->str_appendall
#define sqlite3_str_appendchar         sqlite3_api->str_appendchar
#define sqlite3_str_reset              sqlite3_api->str_reset
#define sqlite3_str_errcode            sqlite3_api->str_errcode
#define sqlite3_str_length             sqlite3_api->str_length
#define sqlite3_str_value              sqlite3_api->str_value
/* Version 3.25.0 and later */
#define sqlite3_create_window_function sqlite3_api->create_window_function
#endif /* NTDDI_VERSION >= NTDDI_WIN10_19H1 */
#if NTDDI_VERSION >= NTDDI_WIN10_VB
/* Version 3.26.0 and later */
#define sqlite3_normalized_sql         sqlite3_api->normalized_sql
/* Version 3.28.0 and later */
#define sqlite3_stmt_isexplain         sqlite3_api->stmt_isexplain
#define sqlite3_value_frombind         sqlite3_api->value_frombind
#endif /* NTDDI_VERSION >= NTDDI_WIN10_VB */
#if NTDDI_VERSION >= NTDDI_WIN10_CO
/* Version 3.30.0 and later */
#define sqlite3_drop_modules           sqlite3_api->drop_modules
/* Version 3.31.0 and later */
#define sqlite3_hard_heap_limit64      sqlite3_api->hard_heap_limit64
#define sqlite3_uri_key                sqlite3_api->uri_key
#define sqlite3_filename_database      sqlite3_api->filename_database
#define sqlite3_filename_journal       sqlite3_api->filename_journal
#define sqlite3_filename_wal           sqlite3_api->filename_wal
/* Version 3.32.0 and later */
#define sqlite3_create_filename        sqlite3_api->create_filename
#define sqlite3_free_filename          sqlite3_api->free_filename
#define sqlite3_database_file_object   sqlite3_api->database_file_object
/* Version 3.34.0 and later */
#define sqlite3_txn_state              sqlite3_api->txn_state
#endif /* NTDDI_VERSION >= NTDDI_WIN10_CO */
#if NTDDI_VERSION >= NTDDI_WIN11_ZN
/* Version 3.36.1 and later */
#define sqlite3_changes64              sqlite3_api->changes64
#define sqlite3_total_changes64        sqlite3_api->total_changes64
/* Version 3.37.0 and later */
#define sqlite3_autovacuum_pages       sqlite3_api->autovacuum_pages
/* Version 3.38.0 and later */
#define sqlite3_error_offset           sqlite3_api->error_offset
#define sqlite3_vtab_rhs_value         sqlite3_api->vtab_rhs_value
#define sqlite3_vtab_distinct          sqlite3_api->vtab_distinct
#define sqlite3_vtab_in                sqlite3_api->vtab_in
#define sqlite3_vtab_in_first          sqlite3_api->vtab_in_first
#define sqlite3_vtab_in_next           sqlite3_api->vtab_in_next
/* Version 3.39.0 and later */
#ifndef SQLITE_OMIT_DESERIALIZE
#define sqlite3_deserialize            sqlite3_api->deserialize
#define sqlite3_serialize              sqlite3_api->serialize
#endif
#define sqlite3_db_name                sqlite3_api->db_name
/* Version 3.40.0 and later */
#define sqlite3_value_encoding         sqlite3_api->value_encoding
#endif /* NTDDI_VERSION >= NTDDI_WIN11_ZN */
#if NTDDI_VERSION >= NTDDI_WIN11_GE
/* Version 3.41.0 and later */
#define sqlite3_is_interrupted         sqlite3_api->is_interrupted
/* Version 3.43.0 and later */
#define sqlite3_stmt_explain           sqlite3_api->stmt_explain
/* Version 3.44.0 and later */
#define sqlite3_get_clientdata         sqlite3_api->get_clientdata
#define sqlite3_set_clientdata         sqlite3_api->set_clientdata
/* Version 3.50.0 and later */
#define sqlite3_setlk_timeout          sqlite3_api->setlk_timeout
/* Version 3.51.0 and later */
#define sqlite3_set_errmsg             sqlite3_api->set_errmsg
#define sqlite3_db_status64            sqlite3_api->db_status64
#endif /* NTDDI_VERSION >= NTDDI_WIN11_GE */
#endif /* !defined(SQLITE_CORE) && !defined(SQLITE_OMIT_LOAD_EXTENSION) */

#if !defined(SQLITE_CORE) && !defined(SQLITE_OMIT_LOAD_EXTENSION)
  /* This case when the file really is being compiled as a loadable 
  ** extension */
# define SQLITE_EXTENSION_INIT1     const sqlite3_api_routines *sqlite3_api=0;
# define SQLITE_EXTENSION_INIT2(v)  sqlite3_api=v;
# define SQLITE_EXTENSION_INIT3     \
    extern const sqlite3_api_routines *sqlite3_api;
#else
  /* This case when the file is being statically linked into the 
  ** application */
# define SQLITE_EXTENSION_INIT1     /*no-op*/
# define SQLITE_EXTENSION_INIT2(v)  (void)v; /* unused parameter */
# define SQLITE_EXTENSION_INIT3     /*no-op*/
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion Application Family

#endif /* NTDDI_VERSION >= NTDDI_WINTHRESHOLD */
#endif /* SQLITE3EXT_H */
