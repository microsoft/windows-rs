#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_aggregate_context();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_aggregate_count();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_auto_extension();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_backup_finish();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_backup_init();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_backup_pagecount();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_backup_remaining();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_backup_step();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_bind_blob();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_bind_blob64();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_bind_double();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_bind_int();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_bind_int64();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_bind_null();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_bind_parameter_count();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_bind_parameter_index();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_bind_parameter_name();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_bind_pointer();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_bind_text();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_bind_text16();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_bind_text64();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_bind_value();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_bind_zeroblob();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_bind_zeroblob64();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_blob_bytes();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_blob_close();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_blob_open();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_blob_read();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_blob_reopen();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_blob_write();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_busy_handler();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_busy_timeout();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_cancel_auto_extension();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_changes();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_clear_bindings();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_close();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_close_v2();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_collation_needed();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_collation_needed16();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_column_blob();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_column_bytes();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_column_bytes16();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_column_count();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_column_database_name();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_column_database_name16();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_column_decltype();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_column_decltype16();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_column_double();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_column_int();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_column_int64();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_column_name();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_column_name16();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_column_origin_name();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_column_origin_name16();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_column_table_name();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_column_table_name16();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_column_text();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_column_text16();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_column_type();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_column_value();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_commit_hook();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_compileoption_get();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_compileoption_used();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_complete();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_complete16();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_config();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_context_db_handle();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_create_collation();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_create_collation16();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_create_collation_v2();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_create_filename();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_create_function();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_create_function16();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_create_function_v2();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_create_module();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_create_module_v2();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_create_window_function();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_data_count();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_database_file_object();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_db_cacheflush();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_db_config();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_db_filename();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_db_handle();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_db_mutex();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_db_readonly();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_db_release_memory();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_db_status();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_declare_vtab();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_deserialize();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_drop_modules();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_enable_load_extension();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_enable_shared_cache();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_errcode();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_errmsg();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_errmsg16();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_errstr();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_exec();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_expanded_sql();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_expired();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_extended_errcode();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_extended_result_codes();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_file_control();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_filename_database();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_filename_journal();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_filename_wal();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_finalize();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_free();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_free_filename();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_free_table();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_get_autocommit();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_get_auxdata();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_get_table();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_global_recover();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_hard_heap_limit64();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_initialize();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_interrupt();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_keyword_check();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_keyword_count();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_keyword_name();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_last_insert_rowid();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_libversion();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_libversion_number();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_limit();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_load_extension();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_log();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_malloc();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_malloc64();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_memory_alarm();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_memory_highwater();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_memory_used();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_mprintf();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_msize();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_mutex_alloc();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_mutex_enter();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_mutex_free();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_mutex_leave();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_mutex_try();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_next_stmt();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_open();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_open16();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_open_v2();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_os_end();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_os_init();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_overload_function();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_prepare();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_prepare16();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_prepare16_v2();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_prepare16_v3();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_prepare_v2();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_prepare_v3();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_profile();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_progress_handler();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_randomness();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_realloc();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_realloc64();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_release_memory();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_reset();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_reset_auto_extension();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_result_blob();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_result_blob64();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_result_double();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_result_error();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_result_error16();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_result_error_code();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_result_error_nomem();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_result_error_toobig();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_result_int();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_result_int64();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_result_null();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_result_pointer();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_result_subtype();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_result_text();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_result_text16();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_result_text16be();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_result_text16le();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_result_text64();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_result_value();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_result_zeroblob();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_result_zeroblob64();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_rollback_hook();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_rtree_geometry_callback();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_rtree_query_callback();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_serialize();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_set_authorizer();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_set_auxdata();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_set_last_insert_rowid();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_shutdown();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_sleep();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_snprintf();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_soft_heap_limit();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_soft_heap_limit64();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_sourceid();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_sql();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_status();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_status64();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_step();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_stmt_busy();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_stmt_isexplain();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_stmt_readonly();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_stmt_status();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_str_append();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_str_appendall();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_str_appendchar();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_str_appendf();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_str_errcode();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_str_finish();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_str_length();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_str_new();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_str_reset();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_str_value();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_str_vappendf();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_strglob();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_stricmp();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_strlike();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_strnicmp();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_system_errno();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_table_column_metadata();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_test_control();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_thread_cleanup();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_threadsafe();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_total_changes();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_trace();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_trace_v2();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_transfer_bindings();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_txn_state();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_update_hook();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_uri_boolean();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_uri_int64();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_uri_key();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_uri_parameter();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_user_data();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_value_blob();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_value_bytes();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_value_bytes16();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_value_double();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_value_dup();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_value_free();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_value_frombind();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_value_int();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_value_int64();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_value_nochange();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_value_numeric_type();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_value_pointer();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_value_subtype();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_value_text();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_value_text16();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_value_text16be();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_value_text16le();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_value_type();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_vfs_find();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_vfs_register();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_vfs_unregister();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_vmprintf();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_vsnprintf();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_vtab_collation();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_vtab_config();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_vtab_nochange();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_vtab_on_conflict();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_wal_autocheckpoint();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_wal_checkpoint();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_wal_checkpoint_v2();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_wal_hook();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_win32_set_directory();
    #[doc = "*Required features: `Win32_System_SqlLite`*"]
    pub fn sqlite3_win32_set_directory16();
    #[doc = "*Required features: `Win32_System_SqlLite`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sqlite3_win32_set_directory8();
}
