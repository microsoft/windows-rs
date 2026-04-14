    
/***
*       no_sal2.h - renders the SAL annotations for documenting APIs harmless.
*
*       Copyright (c) Microsoft Corporation. All rights reserved.
*
*Purpose:
*       sal.h provides a set of SAL2 annotations to describe how a function uses its
*       parameters - the assumptions it makes about them, and the guarantees it makes
*       upon finishing. This file redefines all those annotation macros to be harmless.
*       It is designed for use in down-level build environments where the tooling may
*       be unhappy with the standard SAL2 macro definitions.
*
*       [Public]
*
****/

#ifndef _NO_SAL_2_H_
#define _NO_SAL_2_H_

#ifdef _When_
#undef _When_
#endif
#define _When_(c,a)
#ifdef _At_
#undef _At_
#endif
#define _At_(t,a)
#ifdef _At_buffer_
#undef _At_buffer_
#endif
#define _At_buffer_(t,i,c,a)
#ifdef _Group_
#undef _Group_
#endif
#define _Group_(a)
#ifdef _Pre_
#undef _Pre_
#endif
#define _Pre_
#ifdef _Post_
#undef _Post_
#endif
#define _Post_
#ifdef _Deref_
#undef _Deref_
#endif
#define _Deref_
#ifdef _Null_
#undef _Null_
#endif
#define _Null_
#ifdef _Notnull_
#undef _Notnull_
#endif
#define _Notnull_
#ifdef _Maybenull_
#undef _Maybenull_
#endif
#define _Maybenull_
#ifdef _Const_
#undef _Const_
#endif
#define _Const_
#ifdef _Check_return_
#undef _Check_return_
#endif
#define _Check_return_
#ifdef _Must_inspect_result_
#undef _Must_inspect_result_
#endif
#define _Must_inspect_result_
#ifdef _Pre_satisfies_
#undef _Pre_satisfies_
#endif
#define _Pre_satisfies_(e)
#ifdef _Post_satisfies_
#undef _Post_satisfies_
#endif
#define _Post_satisfies_(e)
#ifdef _Writable_elements_
#undef _Writable_elements_
#endif
#define _Writable_elements_(s)
#ifdef _Writable_bytes_
#undef _Writable_bytes_
#endif
#define _Writable_bytes_(s)
#ifdef _Readable_elements_
#undef _Readable_elements_
#endif
#define _Readable_elements_(s)
#ifdef _Readable_bytes_
#undef _Readable_bytes_
#endif
#define _Readable_bytes_(s)
#ifdef _Null_terminated_
#undef _Null_terminated_
#endif
#define _Null_terminated_
#ifdef _NullNull_terminated_
#undef _NullNull_terminated_
#endif
#define _NullNull_terminated_
#ifdef _Valid_
#undef _Valid_
#endif
#define _Valid_
#ifdef _Notvalid_
#undef _Notvalid_
#endif
#define _Notvalid_
#ifdef _Success_
#undef _Success_
#endif
#define _Success_(c)
#ifdef _Return_type_success_
#undef _Return_type_success_
#endif
#define _Return_type_success_(c)
#ifdef _On_failure_
#undef _On_failure_
#endif
#define _On_failure_(a)
#ifdef _Always_
#undef _Always_
#endif
#define _Always_(a)
#ifdef _Use_decl_annotations_
#undef _Use_decl_annotations_
#endif
#define _Use_decl_annotations_
#ifdef _Pre_defensive_
#undef _Pre_defensive_
#endif
#define _Pre_defensive_
#ifdef _Post_defensive_
#undef _Post_defensive_
#endif
#define _Post_defensive_
#ifdef _Pre_unknown_
#undef _Pre_unknown_
#endif
#define _Pre_unknown_
#ifdef _Acquires_lock_
#undef _Acquires_lock_
#endif
#define _Acquires_lock_(e)
#ifdef _Releases_lock_
#undef _Releases_lock_
#endif
#define _Releases_lock_(e)
#ifdef _Requires_lock_held_
#undef _Requires_lock_held_
#endif
#define _Requires_lock_held_(e)
#ifdef _Requires_lock_not_held_
#undef _Requires_lock_not_held_
#endif
#define _Requires_lock_not_held_(e)
#ifdef _Requires_no_locks_held_
#undef _Requires_no_locks_held_
#endif
#define _Requires_no_locks_held_
#ifdef _Guarded_by_
#undef _Guarded_by_
#endif
#define _Guarded_by_(e)
#ifdef _Write_guarded_by_
#undef _Write_guarded_by_
#endif
#define _Write_guarded_by_(e)
#ifdef _Interlocked_
#undef _Interlocked_
#endif
#define _Interlocked_
#ifdef _Post_same_lock_
#undef _Post_same_lock_
#endif
#define _Post_same_lock_(e1,e2)
#ifdef _Benign_race_begin_
#undef _Benign_race_begin_
#endif
#define _Benign_race_begin_
#ifdef _Benign_race_end_
#undef _Benign_race_end_
#endif
#define _Benign_race_end_
#ifdef _No_competing_thread_
#undef _No_competing_thread_
#endif
#define _No_competing_thread_
#ifdef _No_competing_thread_begin_
#undef _No_competing_thread_begin_
#endif
#define _No_competing_thread_begin_
#ifdef _No_competing_thread_end_
#undef _No_competing_thread_end_
#endif
#define _No_competing_thread_end_
#ifdef _Acquires_shared_lock_
#undef _Acquires_shared_lock_
#endif
#define _Acquires_shared_lock_(e)
#ifdef _Releases_shared_lock_
#undef _Releases_shared_lock_
#endif
#define _Releases_shared_lock_(e)
#ifdef _Requires_shared_lock_held_
#undef _Requires_shared_lock_held_
#endif
#define _Requires_shared_lock_held_(e)
#ifdef _Acquires_exclusive_lock_
#undef _Acquires_exclusive_lock_
#endif
#define _Acquires_exclusive_lock_(e)
#ifdef _Releases_exclusive_lock_
#undef _Releases_exclusive_lock_
#endif
#define _Releases_exclusive_lock_(e)
#ifdef _Requires_exclusive_lock_held_
#undef _Requires_exclusive_lock_held_
#endif
#define _Requires_exclusive_lock_held_(e)
#ifdef _Has_lock_kind_
#undef _Has_lock_kind_
#endif
#define _Has_lock_kind_(n)
#ifdef _Create_lock_level_
#undef _Create_lock_level_
#endif
#define _Create_lock_level_(n)
#ifdef _Has_lock_level_
#undef _Has_lock_level_
#endif
#define _Has_lock_level_(n)
#ifdef _Lock_level_order_
#undef _Lock_level_order_
#endif
#define _Lock_level_order_(n1,n2)
#ifdef _Analysis_assume_lock_acquired_
#undef _Analysis_assume_lock_acquired_
#endif
#define _Analysis_assume_lock_acquired_(e)
#ifdef _Analysis_assume_lock_released_
#undef _Analysis_assume_lock_released_
#endif
#define _Analysis_assume_lock_released_(e)
#ifdef _Analysis_assume_lock_held_
#undef _Analysis_assume_lock_held_
#endif
#define _Analysis_assume_lock_held_(e)
#ifdef _Analysis_assume_lock_not_held_
#undef _Analysis_assume_lock_not_held_
#endif
#define _Analysis_assume_lock_not_held_(e)
#ifdef _Analysis_assume_same_lock_
#undef _Analysis_assume_same_lock_
#endif
#define _Analysis_assume_same_lock_(e)
#ifdef _In_
#undef _In_
#endif
#define _In_
#ifdef _Out_
#undef _Out_
#endif
#define _Out_
#ifdef _Inout_
#undef _Inout_
#endif
#define _Inout_
#ifdef _In_z_
#undef _In_z_
#endif
#define _In_z_
#ifdef _Inout_z_
#undef _Inout_z_
#endif
#define _Inout_z_
#ifdef _In_reads_
#undef _In_reads_
#endif
#define _In_reads_(s)
#ifdef _In_reads_bytes_
#undef _In_reads_bytes_
#endif
#define _In_reads_bytes_(s)
#ifdef _In_reads_z_
#undef _In_reads_z_
#endif
#define _In_reads_z_(s)
#ifdef _In_reads_or_z_
#undef _In_reads_or_z_
#endif
#define _In_reads_or_z_(s)
#ifdef _Out_writes_
#undef _Out_writes_
#endif
#define _Out_writes_(s)
#ifdef _Out_writes_bytes_
#undef _Out_writes_bytes_
#endif
#define _Out_writes_bytes_(s)
#ifdef _Out_writes_z_
#undef _Out_writes_z_
#endif
#define _Out_writes_z_(s)
#ifdef _Inout_updates_
#undef _Inout_updates_
#endif
#define _Inout_updates_(s)
#ifdef _Inout_updates_bytes_
#undef _Inout_updates_bytes_
#endif
#define _Inout_updates_bytes_(s)
#ifdef _Inout_updates_z_
#undef _Inout_updates_z_
#endif
#define _Inout_updates_z_(s)
#ifdef _Out_writes_to_
#undef _Out_writes_to_
#endif
#define _Out_writes_to_(s,c)
#ifdef _Out_writes_bytes_to_
#undef _Out_writes_bytes_to_
#endif
#define _Out_writes_bytes_to_(s,c)
#ifdef _Out_writes_all_
#undef _Out_writes_all_
#endif
#define _Out_writes_all_(s)
#ifdef _Out_writes_bytes_all_
#undef _Out_writes_bytes_all_
#endif
#define _Out_writes_bytes_all_(s)
#ifdef _Inout_updates_to_
#undef _Inout_updates_to_
#endif
#define _Inout_updates_to_(s,c)
#ifdef _Inout_updates_bytes_to_
#undef _Inout_updates_bytes_to_
#endif
#define _Inout_updates_bytes_to_(s,c)
#ifdef _Inout_updates_all_
#undef _Inout_updates_all_
#endif
#define _Inout_updates_all_(s)
#ifdef _Inout_updates_bytes_all_
#undef _Inout_updates_bytes_all_
#endif
#define _Inout_updates_bytes_all_(s)
#ifdef _In_reads_to_ptr_
#undef _In_reads_to_ptr_
#endif
#define _In_reads_to_ptr_(p)
#ifdef _In_reads_to_ptr_z_
#undef _In_reads_to_ptr_z_
#endif
#define _In_reads_to_ptr_z_(p)
#ifdef _Out_writes_to_ptr_
#undef _Out_writes_to_ptr_
#endif
#define _Out_writes_to_ptr_(p)
#ifdef _Out_writes_to_ptr_z_
#undef _Out_writes_to_ptr_z_
#endif
#define _Out_writes_to_ptr_z_(p)
#ifdef _In_opt_
#undef _In_opt_
#endif
#define _In_opt_
#ifdef _Out_opt_
#undef _Out_opt_
#endif
#define _Out_opt_
#ifdef _Inout_opt_
#undef _Inout_opt_
#endif
#define _Inout_opt_
#ifdef _In_opt_z_
#undef _In_opt_z_
#endif
#define _In_opt_z_
#ifdef _Inout_opt_z_
#undef _Inout_opt_z_
#endif
#define _Inout_opt_z_
#ifdef _In_reads_opt_
#undef _In_reads_opt_
#endif
#define _In_reads_opt_(s)
#ifdef _In_reads_opt_z_
#undef _In_reads_opt_z_
#endif
#define _In_reads_opt_z_(s)
#ifdef _In_reads_bytes_opt_
#undef _In_reads_bytes_opt_
#endif
#define _In_reads_bytes_opt_(s)
#ifdef _Out_writes_opt_
#undef _Out_writes_opt_
#endif
#define _Out_writes_opt_(s)
#ifdef _Out_writes_bytes_opt_
#undef _Out_writes_bytes_opt_
#endif
#define _Out_writes_bytes_opt_(s)
#ifdef _Out_writes_opt_z_
#undef _Out_writes_opt_z_
#endif
#define _Out_writes_opt_z_(s)
#ifdef _Inout_updates_opt_
#undef _Inout_updates_opt_
#endif
#define _Inout_updates_opt_(s)
#ifdef _Inout_updates_bytes_opt_
#undef _Inout_updates_bytes_opt_
#endif
#define _Inout_updates_bytes_opt_(s)
#ifdef _Inout_updates_opt_z_
#undef _Inout_updates_opt_z_
#endif
#define _Inout_updates_opt_z_(s)
#ifdef _Out_writes_to_opt_
#undef _Out_writes_to_opt_
#endif
#define _Out_writes_to_opt_(s,c)
#ifdef _Out_writes_bytes_to_opt_
#undef _Out_writes_bytes_to_opt_
#endif
#define _Out_writes_bytes_to_opt_(s,c)
#ifdef _Out_writes_all_opt_
#undef _Out_writes_all_opt_
#endif
#define _Out_writes_all_opt_(s)
#ifdef _Out_writes_bytes_all_opt_
#undef _Out_writes_bytes_all_opt_
#endif
#define _Out_writes_bytes_all_opt_(s)
#ifdef _Inout_updates_to_opt_
#undef _Inout_updates_to_opt_
#endif
#define _Inout_updates_to_opt_(s,c)
#ifdef _Inout_updates_bytes_to_opt_
#undef _Inout_updates_bytes_to_opt_
#endif
#define _Inout_updates_bytes_to_opt_(s,c)
#ifdef _Inout_updates_all_opt_
#undef _Inout_updates_all_opt_
#endif
#define _Inout_updates_all_opt_(s)
#ifdef _Inout_updates_bytes_all_opt_
#undef _Inout_updates_bytes_all_opt_
#endif
#define _Inout_updates_bytes_all_opt_(s)
#ifdef _In_reads_to_ptr_opt_
#undef _In_reads_to_ptr_opt_
#endif
#define _In_reads_to_ptr_opt_(p)
#ifdef _In_reads_to_ptr_opt_z_
#undef _In_reads_to_ptr_opt_z_
#endif
#define _In_reads_to_ptr_opt_z_(p)
#ifdef _Out_writes_to_ptr_opt_
#undef _Out_writes_to_ptr_opt_
#endif
#define _Out_writes_to_ptr_opt_(p)
#ifdef _Out_writes_to_ptr_opt_z_
#undef _Out_writes_to_ptr_opt_z_
#endif
#define _Out_writes_to_ptr_opt_z_(p)
#ifdef _Outptr_
#undef _Outptr_
#endif
#define _Outptr_
#ifdef _Outptr_opt_
#undef _Outptr_opt_
#endif
#define _Outptr_opt_
#ifdef _Outptr_result_maybenull_
#undef _Outptr_result_maybenull_
#endif
#define _Outptr_result_maybenull_
#ifdef _Outptr_opt_result_maybenull_
#undef _Outptr_opt_result_maybenull_
#endif
#define _Outptr_opt_result_maybenull_
#ifdef _Outptr_result_z_
#undef _Outptr_result_z_
#endif
#define _Outptr_result_z_
#ifdef _Outptr_opt_result_z_
#undef _Outptr_opt_result_z_
#endif
#define _Outptr_opt_result_z_
#ifdef _Outptr_result_maybenull_z_
#undef _Outptr_result_maybenull_z_
#endif
#define _Outptr_result_maybenull_z_
#ifdef _Outptr_opt_result_maybenull_z_
#undef _Outptr_opt_result_maybenull_z_
#endif
#define _Outptr_opt_result_maybenull_z_
#ifdef _COM_Outptr_
#undef _COM_Outptr_
#endif
#define _COM_Outptr_
#ifdef _COM_Outptr_opt_
#undef _COM_Outptr_opt_
#endif
#define _COM_Outptr_opt_
#ifdef _COM_Outptr_result_maybenull_
#undef _COM_Outptr_result_maybenull_
#endif
#define _COM_Outptr_result_maybenull_
#ifdef _COM_Outptr_opt_result_maybenull_
#undef _COM_Outptr_opt_result_maybenull_
#endif
#define _COM_Outptr_opt_result_maybenull_
#ifdef _Outptr_result_buffer_
#undef _Outptr_result_buffer_
#endif
#define _Outptr_result_buffer_(s)
#ifdef _Outptr_result_buffer_maybenull_
#undef _Outptr_result_buffer_maybenull_
#endif
#define _Outptr_result_buffer_maybenull_(s)
#ifdef _Outptr_result_bytebuffer_
#undef _Outptr_result_bytebuffer_
#endif
#define _Outptr_result_bytebuffer_(s)
#ifdef _Outptr_result_bytebuffer_maybenull_
#undef _Outptr_result_bytebuffer_maybenull_
#endif
#define _Outptr_result_bytebuffer_maybenull_(s)
#ifdef _Outptr_opt_result_buffer_
#undef _Outptr_opt_result_buffer_
#endif
#define _Outptr_opt_result_buffer_(s)
#ifdef _Outptr_opt_result_buffer_maybenull_
#undef _Outptr_opt_result_buffer_maybenull_
#endif
#define _Outptr_opt_result_buffer_maybenull_(s)
#ifdef _Outptr_opt_result_bytebuffer_
#undef _Outptr_opt_result_bytebuffer_
#endif
#define _Outptr_opt_result_bytebuffer_(s)
#ifdef _Outptr_opt_result_bytebuffer_maybenull_
#undef _Outptr_opt_result_bytebuffer_maybenull_
#endif
#define _Outptr_opt_result_bytebuffer_maybenull_(s)
#ifdef _Outptr_result_buffer_to_
#undef _Outptr_result_buffer_to_
#endif
#define _Outptr_result_buffer_to_(s,c)
#ifdef _Outptr_result_bytebuffer_to_
#undef _Outptr_result_bytebuffer_to_
#endif
#define _Outptr_result_bytebuffer_to_(s,c)
#ifdef _Outptr_opt_result_buffer_to_
#undef _Outptr_opt_result_buffer_to_
#endif
#define _Outptr_opt_result_buffer_to_(s,c)
#ifdef _Outptr_opt_result_bytebuffer_to_
#undef _Outptr_opt_result_bytebuffer_to_
#endif
#define _Outptr_opt_result_bytebuffer_to_(s,c)
#ifdef _Ret_
#undef _Ret_
#endif
#define _Ret_
#ifdef _Ret_valid_
#undef _Ret_valid_
#endif
#define _Ret_valid_
#ifdef _Ret_z_
#undef _Ret_z_
#endif
#define _Ret_z_
#ifdef _Ret_writes_
#undef _Ret_writes_
#endif
#define _Ret_writes_(s)
#ifdef _Ret_writes_bytes_
#undef _Ret_writes_bytes_
#endif
#define _Ret_writes_bytes_(s)
#ifdef _Ret_writes_z_
#undef _Ret_writes_z_
#endif
#define _Ret_writes_z_(s)
#ifdef _Ret_writes_to_
#undef _Ret_writes_to_
#endif
#define _Ret_writes_to_(s,c)
#ifdef _Ret_writes_bytes_to_
#undef _Ret_writes_bytes_to_
#endif
#define _Ret_writes_bytes_to_(s,c)
#ifdef _Ret_writes_to_ptr_
#undef _Ret_writes_to_ptr_
#endif
#define _Ret_writes_to_ptr_(p)
#ifdef _Ret_writes_to_ptr_z_
#undef _Ret_writes_to_ptr_z_
#endif
#define _Ret_writes_to_ptr_z_(p)
#ifdef _Ret_writes_maybenull_
#undef _Ret_writes_maybenull_
#endif
#define _Ret_writes_maybenull_(s)
#ifdef _Ret_writes_bytes_maybenull_
#undef _Ret_writes_bytes_maybenull_
#endif
#define _Ret_writes_bytes_maybenull_(s)
#ifdef _Ret_writes_to_maybenull_
#undef _Ret_writes_to_maybenull_
#endif
#define _Ret_writes_to_maybenull_(s,c)
#ifdef _Ret_writes_bytes_to_maybenull_
#undef _Ret_writes_bytes_to_maybenull_
#endif
#define _Ret_writes_bytes_to_maybenull_(s,c)
#ifdef _Ret_writes_maybenull_z_
#undef _Ret_writes_maybenull_z_
#endif
#define _Ret_writes_maybenull_z_(s)
#ifdef _Ret_null_
#undef _Ret_null_
#endif
#define _Ret_null_
#ifdef _Ret_notnull_
#undef _Ret_notnull_
#endif
#define _Ret_notnull_
#ifdef _Ret_maybenull_
#undef _Ret_maybenull_
#endif
#define _Ret_maybenull_
#ifdef _Ret_maybenull_z_
#undef _Ret_maybenull_z_
#endif
#define _Ret_maybenull_z_
#ifdef _Field_size_
#undef _Field_size_
#endif
#define _Field_size_(s)
#ifdef _Field_size_opt_
#undef _Field_size_opt_
#endif
#define _Field_size_opt_(s)
#ifdef _Field_size_bytes_
#undef _Field_size_bytes_
#endif
#define _Field_size_bytes_(s)
#ifdef _Field_size_bytes_opt_
#undef _Field_size_bytes_opt_
#endif
#define _Field_size_bytes_opt_(s)
#ifdef _Field_size_part_
#undef _Field_size_part_
#endif
#define _Field_size_part_(s,c)
#ifdef _Field_size_part_opt_
#undef _Field_size_part_opt_
#endif
#define _Field_size_part_opt_(s,c)
#ifdef _Field_size_bytes_part_
#undef _Field_size_bytes_part_
#endif
#define _Field_size_bytes_part_(s,c)
#ifdef _Field_size_bytes_part_opt_
#undef _Field_size_bytes_part_opt_
#endif
#define _Field_size_bytes_part_opt_(s,c)
#ifdef _Field_size_full_
#undef _Field_size_full_
#endif
#define _Field_size_full_(s)
#ifdef _Field_size_full_opt_
#undef _Field_size_full_opt_
#endif
#define _Field_size_full_opt_(s)
#ifdef _Field_size_bytes_full_
#undef _Field_size_bytes_full_
#endif
#define _Field_size_bytes_full_(s)
#ifdef _Field_size_bytes_full_opt_
#undef _Field_size_bytes_full_opt_
#endif
#define _Field_size_bytes_full_opt_(s)
#ifdef _Printf_format_string_
#undef _Printf_format_string_
#endif
#define _Printf_format_string_
#ifdef _Scanf_format_string_
#undef _Scanf_format_string_
#endif
#define _Scanf_format_string_
#ifdef _Scanf_s_format_string_
#undef _Scanf_s_format_string_
#endif
#define _Scanf_s_format_string_
#ifdef _Printf_format_string_params_
#undef _Printf_format_string_params_
#endif
#define _Printf_format_string_params_(x)
#ifdef _Scanf_format_string_params_
#undef _Scanf_format_string_params_
#endif
#define _Scanf_format_string_params_(x)
#ifdef _Scanf_s_format_string_params_
#undef _Scanf_s_format_string_params_
#endif
#define _Scanf_s_format_string_params_(x)
#ifdef _In_range_
#undef _In_range_
#endif
#define _In_range_(l,h)
#ifdef _Out_range_
#undef _Out_range_
#endif
#define _Out_range_(l,h)
#ifdef _Ret_range_
#undef _Ret_range_
#endif
#define _Ret_range_(l,h)
#ifdef _Deref_in_range_
#undef _Deref_in_range_
#endif
#define _Deref_in_range_(l,h)
#ifdef _Deref_out_range_
#undef _Deref_out_range_
#endif
#define _Deref_out_range_(l,h)
#ifdef _Deref_inout_range_
#undef _Deref_inout_range_
#endif
#define _Deref_inout_range_(l,h)
#ifdef _Field_range_
#undef _Field_range_
#endif
#define _Field_range_(l,h)
#ifdef _Pre_equal_to_
#undef _Pre_equal_to_
#endif
#define _Pre_equal_to_(e)
#ifdef _Post_equal_to_
#undef _Post_equal_to_
#endif
#define _Post_equal_to_(e)
#ifdef _Struct_size_bytes_
#undef _Struct_size_bytes_
#endif
#define _Struct_size_bytes_(s)
#ifdef _Analysis_assume_
#undef _Analysis_assume_
#endif
#define _Analysis_assume_
#ifdef _Analysis_assume_nullterminated_
#undef _Analysis_assume_nullterminated_
#endif
#define _Analysis_assume_nullterminated_(s)
#ifdef _Analysis_mode_
#undef _Analysis_mode_
#endif
#define _Analysis_mode_(m)
#ifdef _Analysis_noreturn_
#undef _Analysis_noreturn_
#endif
#define _Analysis_noreturn_
#ifdef _Raises_SEH_exception_
#undef _Raises_SEH_exception_
#endif
#define _Raises_SEH_exception_
#ifdef _Maybe_raises_SEH_exception_
#undef _Maybe_raises_SEH_exception_
#endif
#define _Maybe_raises_SEH_exception_
#ifdef _Function_class_
#undef _Function_class_
#endif
#define _Function_class_(n)
#ifdef _Literal_
#undef _Literal_
#endif
#define _Literal_
#ifdef _Notliteral_
#undef _Notliteral_
#endif
#define _Notliteral_
#ifdef _Enum_is_bitflag_
#undef _Enum_is_bitflag_
#endif
#define _Enum_is_bitflag_
#ifdef _Strict_type_match_
#undef _Strict_type_match_
#endif
#define _Strict_type_match_
#ifdef _Points_to_data_
#undef _Points_to_data_
#endif
#define _Points_to_data_
#ifdef _Interlocked_operand_
#undef _Interlocked_operand_
#endif
#define _Interlocked_operand_
#ifdef _IRQL_raises_
#undef _IRQL_raises_
#endif
#define _IRQL_raises_(i)
#ifdef _IRQL_requires_
#undef _IRQL_requires_
#endif
#define _IRQL_requires_(i)
#ifdef _IRQL_requires_max_
#undef _IRQL_requires_max_
#endif
#define _IRQL_requires_max_(i)
#ifdef _IRQL_requires_min_
#undef _IRQL_requires_min_
#endif
#define _IRQL_requires_min_(i)
#ifdef _IRQL_saves_
#undef _IRQL_saves_
#endif
#define _IRQL_saves_
#ifdef _IRQL_saves_global_
#undef _IRQL_saves_global_
#endif
#define _IRQL_saves_global_(k,s)
#ifdef _IRQL_restores_
#undef _IRQL_restores_
#endif
#define _IRQL_restores_
#ifdef _IRQL_restores_global_
#undef _IRQL_restores_global_
#endif
#define _IRQL_restores_global_(k,s)
#ifdef _IRQL_always_function_min_
#undef _IRQL_always_function_min_
#endif
#define _IRQL_always_function_min_(i)
#ifdef _IRQL_always_function_max_
#undef _IRQL_always_function_max_
#endif
#define _IRQL_always_function_max_(i)
#ifdef _IRQL_requires_same_
#undef _IRQL_requires_same_
#endif
#define _IRQL_requires_same_
#ifdef _IRQL_uses_cancel_
#undef _IRQL_uses_cancel_
#endif
#define _IRQL_uses_cancel_
#ifdef _IRQL_is_cancel_
#undef _IRQL_is_cancel_
#endif
#define _IRQL_is_cancel_
#ifdef _Kernel_float_saved_
#undef _Kernel_float_saved_
#endif
#define _Kernel_float_saved_
#ifdef _Kernel_float_restored_
#undef _Kernel_float_restored_
#endif
#define _Kernel_float_restored_
#ifdef _Kernel_float_used_
#undef _Kernel_float_used_
#endif
#define _Kernel_float_used_
#ifdef _Kernel_acquires_resource_
#undef _Kernel_acquires_resource_
#endif
#define _Kernel_acquires_resource_(k)
#ifdef _Kernel_releases_resource_
#undef _Kernel_releases_resource_
#endif
#define _Kernel_releases_resource_(k)
#ifdef _Kernel_requires_resource_held_
#undef _Kernel_requires_resource_held_
#endif
#define _Kernel_requires_resource_held_(k)
#ifdef _Kernel_requires_resource_not_held_
#undef _Kernel_requires_resource_not_held_
#endif
#define _Kernel_requires_resource_not_held_(k)
#ifdef _Kernel_clear_do_init_
#undef _Kernel_clear_do_init_
#endif
#define _Kernel_clear_do_init_(yn)
#ifdef _Kernel_IoGetDmaAdapter_
#undef _Kernel_IoGetDmaAdapter_
#endif
#define _Kernel_IoGetDmaAdapter_
#ifdef _Outref_
#undef _Outref_
#endif
#define _Outref_
#ifdef _Outref_result_maybenull_
#undef _Outref_result_maybenull_
#endif
#define _Outref_result_maybenull_
#ifdef _Outref_result_buffer_
#undef _Outref_result_buffer_
#endif
#define _Outref_result_buffer_(s)
#ifdef _Outref_result_bytebuffer_
#undef _Outref_result_bytebuffer_
#endif
#define _Outref_result_bytebuffer_(s)
#ifdef _Outref_result_buffer_to_
#undef _Outref_result_buffer_to_
#endif
#define _Outref_result_buffer_to_(s,c)
#ifdef _Outref_result_bytebuffer_to_
#undef _Outref_result_bytebuffer_to_
#endif
#define _Outref_result_bytebuffer_to_(s,c)
#ifdef _Outref_result_buffer_all_
#undef _Outref_result_buffer_all_
#endif
#define _Outref_result_buffer_all_(s)
#ifdef _Outref_result_bytebuffer_all_
#undef _Outref_result_bytebuffer_all_
#endif
#define _Outref_result_bytebuffer_all_(s)
#ifdef _Outref_result_buffer_maybenull_
#undef _Outref_result_buffer_maybenull_
#endif
#define _Outref_result_buffer_maybenull_(s)
#ifdef _Outref_result_bytebuffer_maybenull_
#undef _Outref_result_bytebuffer_maybenull_
#endif
#define _Outref_result_bytebuffer_maybenull_(s)
#ifdef _Outref_result_buffer_to_maybenull_
#undef _Outref_result_buffer_to_maybenull_
#endif
#define _Outref_result_buffer_to_maybenull_(s,c)
#ifdef _Outref_result_bytebuffer_to_maybenull_
#undef _Outref_result_bytebuffer_to_maybenull_
#endif
#define _Outref_result_bytebuffer_to_maybenull_(s,c)
#ifdef _Outref_result_buffer_all_maybenull_
#undef _Outref_result_buffer_all_maybenull_
#endif
#define _Outref_result_buffer_all_maybenull_(s)
#ifdef _Outref_result_bytebuffer_all_maybenull_
#undef _Outref_result_bytebuffer_all_maybenull_
#endif
#define _Outref_result_bytebuffer_all_maybenull_(s)
#ifdef _In_defensive_
#undef _In_defensive_
#endif
#define _In_defensive_(a)
#ifdef _Out_defensive_
#undef _Out_defensive_
#endif
#define _Out_defensive_(a)
#ifdef _Inout_defensive_
#undef _Inout_defensive_
#endif
#define _Inout_defensive_(a)
#ifdef _Outptr_result_nullonfailure_
#undef _Outptr_result_nullonfailure_
#endif
#define _Outptr_result_nullonfailure_
#ifdef _Outptr_opt_result_nullonfailure_
#undef _Outptr_opt_result_nullonfailure_
#endif
#define _Outptr_opt_result_nullonfailure_
#ifdef _Outref_result_nullonfailure_
#undef _Outref_result_nullonfailure_
#endif
#define _Outref_result_nullonfailure_
#ifdef _Result_nullonfailure_
#undef _Result_nullonfailure_
#endif
#define _Result_nullonfailure_
#ifdef _Result_zeroonfailure_
#undef _Result_zeroonfailure_
#endif
#define _Result_zeroonfailure_
#ifdef _Acquires_nonreentrant_lock_
#undef _Acquires_nonreentrant_lock_
#endif
#define _Acquires_nonreentrant_lock_(e)
#ifdef _Releases_nonreentrant_lock_
#undef _Releases_nonreentrant_lock_
#endif
#define _Releases_nonreentrant_lock_(e)
#ifdef _Function_ignore_lock_checking_
#undef _Function_ignore_lock_checking_
#endif
#define _Function_ignore_lock_checking_(e)
#ifdef _Analysis_suppress_lock_checking_
#undef _Analysis_suppress_lock_checking_
#endif
#define _Analysis_suppress_lock_checking_(e)
#undef _Reserved_
#define _Reserved_           _Pre_equal_to_(0) _Pre_ _Null_
#undef _Pre_z_
#define _Pre_z_              _Pre_ _Null_terminated_
#undef _Post_z_
#define _Post_z_             _Post_ _Null_terminated_
#undef _Prepost_z_
#define _Prepost_z_          _Pre_z_ _Post_z_
#undef _Pre_null_
#define _Pre_null_           _Pre_ _Null_
#undef _Pre_maybenull_
#define _Pre_maybenull_      _Pre_ _Maybenull_
#undef _Pre_notnull_
#define _Pre_notnull_        _Pre_ _Notnull_
#undef _Pre_valid_
#define _Pre_valid_          _Pre_notnull_ _Pre_ _Valid_
#undef _Pre_opt_valid_
#define _Pre_opt_valid_      _Pre_maybenull_ _Pre_ _Valid_
#undef _Post_valid_
#define _Post_valid_         _Post_ _Valid_
#undef _Post_invalid_
#define _Post_invalid_       _Post_ _Deref_ _Notvalid_
#undef _Post_ptr_invalid_
#define _Post_ptr_invalid_   _Post_ _Notvalid_
#undef _Pre_readable_size_
#define _Pre_readable_size_(s)      _Pre_ _Readable_elements_(s) _Pre_ _Valid_
#undef _Pre_writable_size_
#define _Pre_writable_size_(s)      _Pre_ _Writable_elements_(s)
#undef _Pre_readable_byte_size_
#define _Pre_readable_byte_size_(s) _Pre_ _Readable_bytes_(s) _Pre_ _Valid_
#undef _Pre_writable_byte_size_
#define _Pre_writable_byte_size_(s) _Pre_ _Writable_bytes_(s)
#undef _Post_readable_size_
#define _Post_readable_size_(s)     _Post_ _Readable_elements_(s) _Post_ _Valid_
#undef _Post_writable_size_
#define _Post_writable_size_(s)     _Post_ _Writable_elements_(s)
#undef _Post_readable_byte_size_
#define _Post_readable_byte_size_(s) _Post_ _Readable_bytes_(s) _Post_ _Valid_
#undef _Post_writable_byte_size_
#define _Post_writable_byte_size_(s) _Post_ _Writable_bytes_(s)

#endif /* _NO_SAL_2_H_ */
