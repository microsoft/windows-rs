#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_aggregate_context(param0: *mut sqlite3_context, nbytes: i32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_aggregate_count(param0: *mut sqlite3_context) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_auto_extension(xentrypoint: isize) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_backup_finish(p: *mut sqlite3_backup) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_backup_init(pdest: *mut sqlite3, zdestname: super::super::Foundation::PSTR, psource: *mut sqlite3, zsourcename: super::super::Foundation::PSTR) -> *mut sqlite3_backup;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_backup_pagecount(p: *mut sqlite3_backup) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_backup_remaining(p: *mut sqlite3_backup) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_backup_step(p: *mut sqlite3_backup, npage: i32) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_bind_blob(param0: *mut sqlite3_stmt, param1: i32, param2: *const ::core::ffi::c_void, n: i32, param4: isize) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_bind_blob64(param0: *mut sqlite3_stmt, param1: i32, param2: *const ::core::ffi::c_void, param3: u64, param4: isize) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_bind_double(param0: *mut sqlite3_stmt, param1: i32, param2: f64) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_bind_int(param0: *mut sqlite3_stmt, param1: i32, param2: i32) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_bind_int64(param0: *mut sqlite3_stmt, param1: i32, param2: i64) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_bind_null(param0: *mut sqlite3_stmt, param1: i32) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_bind_parameter_count(param0: *mut sqlite3_stmt) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_bind_parameter_index(param0: *mut sqlite3_stmt, zname: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_bind_parameter_name(param0: *mut sqlite3_stmt, param1: i32) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_bind_pointer(param0: *mut sqlite3_stmt, param1: i32, param2: *mut ::core::ffi::c_void, param3: super::super::Foundation::PSTR, param4: isize) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_bind_text(param0: *mut sqlite3_stmt, param1: i32, param2: super::super::Foundation::PSTR, param3: i32, param4: isize) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_bind_text16(param0: *mut sqlite3_stmt, param1: i32, param2: *const ::core::ffi::c_void, param3: i32, param4: isize) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_bind_text64(param0: *mut sqlite3_stmt, param1: i32, param2: super::super::Foundation::PSTR, param3: u64, param4: isize, encoding: u8) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_bind_value(param0: *mut sqlite3_stmt, param1: i32, param2: *const sqlite3_value) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_bind_zeroblob(param0: *mut sqlite3_stmt, param1: i32, n: i32) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_bind_zeroblob64(param0: *mut sqlite3_stmt, param1: i32, param2: u64) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_blob_bytes(param0: *mut sqlite3_blob) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_blob_close(param0: *mut sqlite3_blob) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_blob_open(param0: *mut sqlite3, zdb: super::super::Foundation::PSTR, ztable: super::super::Foundation::PSTR, zcolumn: super::super::Foundation::PSTR, irow: i64, flags: i32, ppblob: *mut *mut sqlite3_blob) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_blob_read(param0: *mut sqlite3_blob, z: *mut ::core::ffi::c_void, n: i32, ioffset: i32) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_blob_reopen(param0: *mut sqlite3_blob, param1: i64) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_blob_write(param0: *mut sqlite3_blob, z: *const ::core::ffi::c_void, n: i32, ioffset: i32) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_busy_handler(param0: *mut sqlite3, param1: isize, param2: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_busy_timeout(param0: *mut sqlite3, ms: i32) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_cancel_auto_extension(xentrypoint: isize) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_changes(param0: *mut sqlite3) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_clear_bindings(param0: *mut sqlite3_stmt) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_close(param0: *mut sqlite3) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_close_v2(param0: *mut sqlite3) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_collation_needed(param0: *mut sqlite3, param1: *mut ::core::ffi::c_void, param2: isize) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_collation_needed16(param0: *mut sqlite3, param1: *mut ::core::ffi::c_void, param2: isize) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_column_blob(param0: *mut sqlite3_stmt, icol: i32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_column_bytes(param0: *mut sqlite3_stmt, icol: i32) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_column_bytes16(param0: *mut sqlite3_stmt, icol: i32) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_column_count(pstmt: *mut sqlite3_stmt) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_column_database_name(param0: *mut sqlite3_stmt, param1: i32) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_column_database_name16(param0: *mut sqlite3_stmt, param1: i32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_column_decltype(param0: *mut sqlite3_stmt, param1: i32) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_column_decltype16(param0: *mut sqlite3_stmt, param1: i32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_column_double(param0: *mut sqlite3_stmt, icol: i32) -> f64;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_column_int(param0: *mut sqlite3_stmt, icol: i32) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_column_int64(param0: *mut sqlite3_stmt, icol: i32) -> i64;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_column_name(param0: *mut sqlite3_stmt, n: i32) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_column_name16(param0: *mut sqlite3_stmt, n: i32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_column_origin_name(param0: *mut sqlite3_stmt, param1: i32) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_column_origin_name16(param0: *mut sqlite3_stmt, param1: i32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_column_table_name(param0: *mut sqlite3_stmt, param1: i32) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_column_table_name16(param0: *mut sqlite3_stmt, param1: i32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_column_text(param0: *mut sqlite3_stmt, icol: i32) -> *mut u8;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_column_text16(param0: *mut sqlite3_stmt, icol: i32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_column_type(param0: *mut sqlite3_stmt, icol: i32) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_column_value(param0: *mut sqlite3_stmt, icol: i32) -> *mut sqlite3_value;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_commit_hook(param0: *mut sqlite3, param1: isize, param2: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_compileoption_get(n: i32) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_compileoption_used(zoptname: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_complete(sql: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_complete16(sql: *const ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_config(param0: i32) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_context_db_handle(param0: *mut sqlite3_context) -> *mut sqlite3;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_create_collation(param0: *mut sqlite3, zname: super::super::Foundation::PSTR, etextrep: i32, parg: *mut ::core::ffi::c_void, xcompare: isize) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_create_collation16(param0: *mut sqlite3, zname: *const ::core::ffi::c_void, etextrep: i32, parg: *mut ::core::ffi::c_void, xcompare: isize) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_create_collation_v2(param0: *mut sqlite3, zname: super::super::Foundation::PSTR, etextrep: i32, parg: *mut ::core::ffi::c_void, xcompare: isize, xdestroy: isize) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_create_filename(zdatabase: super::super::Foundation::PSTR, zjournal: super::super::Foundation::PSTR, zwal: super::super::Foundation::PSTR, nparam: i32, azparam: *const *const i8) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_create_function(db: *mut sqlite3, zfunctionname: super::super::Foundation::PSTR, narg: i32, etextrep: i32, papp: *mut ::core::ffi::c_void, xfunc: isize, xstep: isize, xfinal: isize) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_create_function16(db: *mut sqlite3, zfunctionname: *const ::core::ffi::c_void, narg: i32, etextrep: i32, papp: *mut ::core::ffi::c_void, xfunc: isize, xstep: isize, xfinal: isize) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_create_function_v2(db: *mut sqlite3, zfunctionname: super::super::Foundation::PSTR, narg: i32, etextrep: i32, papp: *mut ::core::ffi::c_void, xfunc: isize, xstep: isize, xfinal: isize, xdestroy: isize) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_create_module(db: *mut sqlite3, zname: super::super::Foundation::PSTR, p: *const sqlite3_module, pclientdata: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_create_module_v2(db: *mut sqlite3, zname: super::super::Foundation::PSTR, p: *const sqlite3_module, pclientdata: *mut ::core::ffi::c_void, xdestroy: isize) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_create_window_function(db: *mut sqlite3, zfunctionname: super::super::Foundation::PSTR, narg: i32, etextrep: i32, papp: *mut ::core::ffi::c_void, xstep: isize, xfinal: isize, xvalue: isize, xinverse: isize, xdestroy: isize) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_data_count(pstmt: *mut sqlite3_stmt) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_database_file_object(param0: super::super::Foundation::PSTR) -> *mut sqlite3_file;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_db_cacheflush(param0: *mut sqlite3) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_db_config(param0: *mut sqlite3, op: i32) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_db_filename(db: *mut sqlite3, zdbname: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_db_handle(param0: *mut sqlite3_stmt) -> *mut sqlite3;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_db_mutex(param0: *mut sqlite3) -> *mut sqlite3_mutex;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_db_readonly(db: *mut sqlite3, zdbname: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_db_release_memory(param0: *mut sqlite3) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_db_status(param0: *mut sqlite3, op: i32, pcur: *mut i32, phiwtr: *mut i32, resetflg: i32) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_declare_vtab(param0: *mut sqlite3, zsql: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_deserialize(db: *mut sqlite3, zschema: super::super::Foundation::PSTR, pdata: *mut u8, szdb: i64, szbuf: i64, mflags: u32) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_drop_modules(db: *mut sqlite3, azkeep: *const *const i8) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_enable_load_extension(db: *mut sqlite3, onoff: i32) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_enable_shared_cache(param0: i32) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_errcode(db: *mut sqlite3) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_errmsg(param0: *mut sqlite3) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_errmsg16(param0: *mut sqlite3) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_errstr(param0: i32) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_exec(param0: *mut sqlite3, sql: super::super::Foundation::PSTR, callback: isize, param3: *mut ::core::ffi::c_void, errmsg: *mut *mut i8) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_expanded_sql(pstmt: *mut sqlite3_stmt) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_expired(param0: *mut sqlite3_stmt) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_extended_errcode(db: *mut sqlite3) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_extended_result_codes(param0: *mut sqlite3, onoff: i32) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_file_control(param0: *mut sqlite3, zdbname: super::super::Foundation::PSTR, op: i32, param3: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_filename_database(param0: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_filename_journal(param0: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_filename_wal(param0: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_finalize(pstmt: *mut sqlite3_stmt) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_free(param0: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_free_filename(param0: super::super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_free_table(result: *mut *mut i8);
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_get_autocommit(param0: *mut sqlite3) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_get_auxdata(param0: *mut sqlite3_context, n: i32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_get_table(db: *mut sqlite3, zsql: super::super::Foundation::PSTR, pazresult: *mut *mut *mut i8, pnrow: *mut i32, pncolumn: *mut i32, pzerrmsg: *mut *mut i8) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_global_recover() -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_hard_heap_limit64(n: i64) -> i64;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_initialize() -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_interrupt(param0: *mut sqlite3);
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_keyword_check(param0: super::super::Foundation::PSTR, param1: i32) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_keyword_count() -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_keyword_name(param0: i32, param1: *const *const i8, param2: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_last_insert_rowid(param0: *mut sqlite3) -> i64;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_libversion() -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_libversion_number() -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_limit(param0: *mut sqlite3, id: i32, newval: i32) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_load_extension(db: *mut sqlite3, zfile: super::super::Foundation::PSTR, zproc: super::super::Foundation::PSTR, pzerrmsg: *mut *mut i8) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_log(ierrcode: i32, zformat: super::super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_malloc(param0: i32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_malloc64(param0: u64) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_memory_alarm(param0: isize, param1: *mut ::core::ffi::c_void, param2: i64) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_memory_highwater(resetflag: i32) -> i64;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_memory_used() -> i64;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_mprintf(param0: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_msize(param0: *mut ::core::ffi::c_void) -> u64;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_mutex_alloc(param0: i32) -> *mut sqlite3_mutex;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_mutex_enter(param0: *mut sqlite3_mutex);
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_mutex_free(param0: *mut sqlite3_mutex);
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_mutex_leave(param0: *mut sqlite3_mutex);
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_mutex_try(param0: *mut sqlite3_mutex) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_next_stmt(pdb: *mut sqlite3, pstmt: *mut sqlite3_stmt) -> *mut sqlite3_stmt;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_open(filename: super::super::Foundation::PSTR, ppdb: *mut *mut sqlite3) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_open16(filename: *const ::core::ffi::c_void, ppdb: *mut *mut sqlite3) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_open_v2(filename: super::super::Foundation::PSTR, ppdb: *mut *mut sqlite3, flags: i32, zvfs: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_os_end() -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_os_init() -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_overload_function(param0: *mut sqlite3, zfuncname: super::super::Foundation::PSTR, narg: i32) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_prepare(db: *mut sqlite3, zsql: super::super::Foundation::PSTR, nbyte: i32, ppstmt: *mut *mut sqlite3_stmt, pztail: *const *const i8) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_prepare16(db: *mut sqlite3, zsql: *const ::core::ffi::c_void, nbyte: i32, ppstmt: *mut *mut sqlite3_stmt, pztail: *const *const ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_prepare16_v2(db: *mut sqlite3, zsql: *const ::core::ffi::c_void, nbyte: i32, ppstmt: *mut *mut sqlite3_stmt, pztail: *const *const ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_prepare16_v3(db: *mut sqlite3, zsql: *const ::core::ffi::c_void, nbyte: i32, prepflags: u32, ppstmt: *mut *mut sqlite3_stmt, pztail: *const *const ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_prepare_v2(db: *mut sqlite3, zsql: super::super::Foundation::PSTR, nbyte: i32, ppstmt: *mut *mut sqlite3_stmt, pztail: *const *const i8) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_prepare_v3(db: *mut sqlite3, zsql: super::super::Foundation::PSTR, nbyte: i32, prepflags: u32, ppstmt: *mut *mut sqlite3_stmt, pztail: *const *const i8) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_profile(param0: *mut sqlite3, xprofile: isize, param2: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_progress_handler(param0: *mut sqlite3, param1: i32, param2: isize, param3: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_randomness(n: i32, p: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_realloc(param0: *mut ::core::ffi::c_void, param1: i32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_realloc64(param0: *mut ::core::ffi::c_void, param1: u64) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_release_memory(param0: i32) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_reset(pstmt: *mut sqlite3_stmt) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_reset_auto_extension();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_result_blob(param0: *mut sqlite3_context, param1: *const ::core::ffi::c_void, param2: i32, param3: isize);
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_result_blob64(param0: *mut sqlite3_context, param1: *const ::core::ffi::c_void, param2: u64, param3: isize);
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_result_double(param0: *mut sqlite3_context, param1: f64);
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_result_error(param0: *mut sqlite3_context, param1: super::super::Foundation::PSTR, param2: i32);
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_result_error16(param0: *mut sqlite3_context, param1: *const ::core::ffi::c_void, param2: i32);
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_result_error_code(param0: *mut sqlite3_context, param1: i32);
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_result_error_nomem(param0: *mut sqlite3_context);
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_result_error_toobig(param0: *mut sqlite3_context);
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_result_int(param0: *mut sqlite3_context, param1: i32);
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_result_int64(param0: *mut sqlite3_context, param1: i64);
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_result_null(param0: *mut sqlite3_context);
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_result_pointer(param0: *mut sqlite3_context, param1: *mut ::core::ffi::c_void, param2: super::super::Foundation::PSTR, param3: isize);
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_result_subtype(param0: *mut sqlite3_context, param1: u32);
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_result_text(param0: *mut sqlite3_context, param1: super::super::Foundation::PSTR, param2: i32, param3: isize);
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_result_text16(param0: *mut sqlite3_context, param1: *const ::core::ffi::c_void, param2: i32, param3: isize);
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_result_text16be(param0: *mut sqlite3_context, param1: *const ::core::ffi::c_void, param2: i32, param3: isize);
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_result_text16le(param0: *mut sqlite3_context, param1: *const ::core::ffi::c_void, param2: i32, param3: isize);
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_result_text64(param0: *mut sqlite3_context, param1: super::super::Foundation::PSTR, param2: u64, param3: isize, encoding: u8);
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_result_value(param0: *mut sqlite3_context, param1: *mut sqlite3_value);
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_result_zeroblob(param0: *mut sqlite3_context, n: i32);
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_result_zeroblob64(param0: *mut sqlite3_context, n: u64) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_rollback_hook(param0: *mut sqlite3, param1: isize, param2: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_rtree_geometry_callback(db: *mut sqlite3, zgeom: super::super::Foundation::PSTR, xgeom: isize, pcontext: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_rtree_query_callback(db: *mut sqlite3, zqueryfunc: super::super::Foundation::PSTR, xqueryfunc: isize, pcontext: *mut ::core::ffi::c_void, xdestructor: isize) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_serialize(db: *mut sqlite3, zschema: super::super::Foundation::PSTR, pisize: *mut i64, mflags: u32) -> *mut u8;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_set_authorizer(param0: *mut sqlite3, xauth: isize, puserdata: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_set_auxdata(param0: *mut sqlite3_context, n: i32, param2: *mut ::core::ffi::c_void, param3: isize);
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_set_last_insert_rowid(param0: *mut sqlite3, param1: i64);
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_shutdown() -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_sleep(param0: i32) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_snprintf(param0: i32, param1: super::super::Foundation::PSTR, param2: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_soft_heap_limit(n: i32);
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_soft_heap_limit64(n: i64) -> i64;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_sourceid() -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_sql(pstmt: *mut sqlite3_stmt) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_status(op: i32, pcurrent: *mut i32, phighwater: *mut i32, resetflag: i32) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_status64(op: i32, pcurrent: *mut i64, phighwater: *mut i64, resetflag: i32) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_step(param0: *mut sqlite3_stmt) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_stmt_busy(param0: *mut sqlite3_stmt) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_stmt_isexplain(pstmt: *mut sqlite3_stmt) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_stmt_readonly(pstmt: *mut sqlite3_stmt) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_stmt_status(param0: *mut sqlite3_stmt, op: i32, resetflg: i32) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_str_append(param0: *mut sqlite3_str, zin: super::super::Foundation::PSTR, n: i32);
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_str_appendall(param0: *mut sqlite3_str, zin: super::super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_str_appendchar(param0: *mut sqlite3_str, n: i32, c: super::super::Foundation::CHAR);
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_str_appendf(param0: *mut sqlite3_str, zformat: super::super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_str_errcode(param0: *mut sqlite3_str) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_str_finish(param0: *mut sqlite3_str) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_str_length(param0: *mut sqlite3_str) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_str_new(param0: *mut sqlite3) -> *mut sqlite3_str;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_str_reset(param0: *mut sqlite3_str);
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_str_value(param0: *mut sqlite3_str) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_str_vappendf(param0: *mut sqlite3_str, zformat: super::super::Foundation::PSTR, param2: *mut i8);
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_strglob(zglob: super::super::Foundation::PSTR, zstr: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_stricmp(param0: super::super::Foundation::PSTR, param1: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_strlike(zglob: super::super::Foundation::PSTR, zstr: super::super::Foundation::PSTR, cesc: u32) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_strnicmp(param0: super::super::Foundation::PSTR, param1: super::super::Foundation::PSTR, param2: i32) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_system_errno(param0: *mut sqlite3) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_table_column_metadata(db: *mut sqlite3, zdbname: super::super::Foundation::PSTR, ztablename: super::super::Foundation::PSTR, zcolumnname: super::super::Foundation::PSTR, pzdatatype: *const *const i8, pzcollseq: *const *const i8, pnotnull: *mut i32, pprimarykey: *mut i32, pautoinc: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_test_control(op: i32) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_thread_cleanup();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_threadsafe() -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_total_changes(param0: *mut sqlite3) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_trace(param0: *mut sqlite3, xtrace: isize, param2: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_trace_v2(param0: *mut sqlite3, umask: u32, xcallback: isize, pctx: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_transfer_bindings(param0: *mut sqlite3_stmt, param1: *mut sqlite3_stmt) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_txn_state(param0: *mut sqlite3, zschema: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_update_hook(param0: *mut sqlite3, param1: isize, param2: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_uri_boolean(zfile: super::super::Foundation::PSTR, zparam: super::super::Foundation::PSTR, bdefault: i32) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_uri_int64(param0: super::super::Foundation::PSTR, param1: super::super::Foundation::PSTR, param2: i64) -> i64;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_uri_key(zfilename: super::super::Foundation::PSTR, n: i32) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_uri_parameter(zfilename: super::super::Foundation::PSTR, zparam: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_user_data(param0: *mut sqlite3_context) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_value_blob(param0: *mut sqlite3_value) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_value_bytes(param0: *mut sqlite3_value) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_value_bytes16(param0: *mut sqlite3_value) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_value_double(param0: *mut sqlite3_value) -> f64;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_value_dup(param0: *const sqlite3_value) -> *mut sqlite3_value;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_value_free(param0: *mut sqlite3_value);
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_value_frombind(param0: *mut sqlite3_value) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_value_int(param0: *mut sqlite3_value) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_value_int64(param0: *mut sqlite3_value) -> i64;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_value_nochange(param0: *mut sqlite3_value) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_value_numeric_type(param0: *mut sqlite3_value) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_value_pointer(param0: *mut sqlite3_value, param1: super::super::Foundation::PSTR) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_value_subtype(param0: *mut sqlite3_value) -> u32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_value_text(param0: *mut sqlite3_value) -> *mut u8;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_value_text16(param0: *mut sqlite3_value) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_value_text16be(param0: *mut sqlite3_value) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_value_text16le(param0: *mut sqlite3_value) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_value_type(param0: *mut sqlite3_value) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_vfs_find(zvfsname: super::super::Foundation::PSTR) -> *mut sqlite3_vfs;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_vfs_register(param0: *mut sqlite3_vfs, makedflt: i32) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_vfs_unregister(param0: *mut sqlite3_vfs) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_vmprintf(param0: super::super::Foundation::PSTR, param1: *mut i8) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_vsnprintf(param0: i32, param1: super::super::Foundation::PSTR, param2: super::super::Foundation::PSTR, param3: *mut i8) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_vtab_collation(param0: *mut sqlite3_index_info, param1: i32) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_vtab_config(param0: *mut sqlite3, op: i32) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_vtab_nochange(param0: *mut sqlite3_context) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_vtab_on_conflict(param0: *mut sqlite3) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_wal_autocheckpoint(db: *mut sqlite3, n: i32) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_wal_checkpoint(db: *mut sqlite3, zdb: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_wal_checkpoint_v2(db: *mut sqlite3, zdb: super::super::Foundation::PSTR, emode: i32, pnlog: *mut i32, pnckpt: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_wal_hook(param0: *mut sqlite3, param1: isize, param2: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_win32_set_directory(r#type: u32, zvalue: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_win32_set_directory16(r#type: u32, zvalue: *const ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_win32_set_directory8(r#type: u32, zvalue: super::super::Foundation::PSTR) -> i32;
}
