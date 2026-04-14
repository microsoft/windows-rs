

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */



/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 501
#endif

/* verify that the <rpcsal.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCSAL_H_VERSION__
#define __REQUIRED_RPCSAL_H_VERSION__ 100
#endif

#include "rpc.h"
#include "rpcndr.h"

#ifndef __RPCNDR_H_VERSION__
#error this stub requires an updated version of <rpcndr.h>
#endif /* __RPCNDR_H_VERSION__ */

#ifndef COM_NO_WINDOWS_H
#include "windows.h"
#include "ole2.h"
#endif /*COM_NO_WINDOWS_H*/

#ifndef __cmdtree_h__
#define __cmdtree_h__

#if defined(_MSC_VER) && (_MSC_VER >= 1020)
#pragma once
#endif

#ifndef DECLSPEC_XFGVIRT
#if defined(_CONTROL_FLOW_GUARD_XFG)
#define DECLSPEC_XFGVIRT(base, func) __declspec(xfg_virtual(base, func))
#else
#define DECLSPEC_XFGVIRT(base, func)
#endif
#endif

/* Forward Declarations */ 

#ifndef __ICommandTree_FWD_DEFINED__
#define __ICommandTree_FWD_DEFINED__
typedef interface ICommandTree ICommandTree;

#endif 	/* __ICommandTree_FWD_DEFINED__ */


#ifndef __IQuery_FWD_DEFINED__
#define __IQuery_FWD_DEFINED__
typedef interface IQuery IQuery;

#endif 	/* __IQuery_FWD_DEFINED__ */


/* header files for imported files */
#include "oledb.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_cmdtree_0000_0000 */
/* [local] */ 

//+---------------------------------------------------------------------------
//
//  Microsoft OLE DB
//  Copyright (C) Microsoft Corporation, 1994 - 1999.
//
//----------------------------------------------------------------------------

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if defined(_WIN64) || defined(_ARM_)
#include <pshpack8.h>	// 8-byte structure packing
#else
#include <pshpack2.h>	// 2-byte structure packing
#endif



extern RPC_IF_HANDLE __MIDL_itf_cmdtree_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_cmdtree_0000_0000_v0_0_s_ifspec;

#ifndef __CommandTreeStructureDefinitions_INTERFACE_DEFINED__
#define __CommandTreeStructureDefinitions_INTERFACE_DEFINED__

/* interface CommandTreeStructureDefinitions */
/* [unique][uuid] */ 

typedef WORD DBCOMMANDOP;


enum DBCOMMANDOPENUM
    {
        DBOP_scalar_constant	= 0,
        DBOP_DEFAULT	= ( DBOP_scalar_constant + 1 ) ,
        DBOP_NULL	= ( DBOP_DEFAULT + 1 ) ,
        DBOP_bookmark_name	= ( DBOP_NULL + 1 ) ,
        DBOP_catalog_name	= ( DBOP_bookmark_name + 1 ) ,
        DBOP_column_name	= ( DBOP_catalog_name + 1 ) ,
        DBOP_schema_name	= ( DBOP_column_name + 1 ) ,
        DBOP_outall_name	= ( DBOP_schema_name + 1 ) ,
        DBOP_qualifier_name	= ( DBOP_outall_name + 1 ) ,
        DBOP_qualified_column_name	= ( DBOP_qualifier_name + 1 ) ,
        DBOP_table_name	= ( DBOP_qualified_column_name + 1 ) ,
        DBOP_nested_table_name	= ( DBOP_table_name + 1 ) ,
        DBOP_nested_column_name	= ( DBOP_nested_table_name + 1 ) ,
        DBOP_row	= ( DBOP_nested_column_name + 1 ) ,
        DBOP_table	= ( DBOP_row + 1 ) ,
        DBOP_sort	= ( DBOP_table + 1 ) ,
        DBOP_distinct	= ( DBOP_sort + 1 ) ,
        DBOP_distinct_order_preserving	= ( DBOP_distinct + 1 ) ,
        DBOP_alias	= ( DBOP_distinct_order_preserving + 1 ) ,
        DBOP_cross_join	= ( DBOP_alias + 1 ) ,
        DBOP_union_join	= ( DBOP_cross_join + 1 ) ,
        DBOP_inner_join	= ( DBOP_union_join + 1 ) ,
        DBOP_left_semi_join	= ( DBOP_inner_join + 1 ) ,
        DBOP_right_semi_join	= ( DBOP_left_semi_join + 1 ) ,
        DBOP_left_anti_semi_join	= ( DBOP_right_semi_join + 1 ) ,
        DBOP_right_anti_semi_join	= ( DBOP_left_anti_semi_join + 1 ) ,
        DBOP_left_outer_join	= ( DBOP_right_anti_semi_join + 1 ) ,
        DBOP_right_outer_join	= ( DBOP_left_outer_join + 1 ) ,
        DBOP_full_outer_join	= ( DBOP_right_outer_join + 1 ) ,
        DBOP_natural_join	= ( DBOP_full_outer_join + 1 ) ,
        DBOP_natural_left_outer_join	= ( DBOP_natural_join + 1 ) ,
        DBOP_natural_right_outer_join	= ( DBOP_natural_left_outer_join + 1 ) ,
        DBOP_natural_full_outer_join	= ( DBOP_natural_right_outer_join + 1 ) ,
        DBOP_set_intersection	= ( DBOP_natural_full_outer_join + 1 ) ,
        DBOP_set_union	= ( DBOP_set_intersection + 1 ) ,
        DBOP_set_left_difference	= ( DBOP_set_union + 1 ) ,
        DBOP_set_right_difference	= ( DBOP_set_left_difference + 1 ) ,
        DBOP_set_anti_difference	= ( DBOP_set_right_difference + 1 ) ,
        DBOP_bag_intersection	= ( DBOP_set_anti_difference + 1 ) ,
        DBOP_bag_union	= ( DBOP_bag_intersection + 1 ) ,
        DBOP_bag_left_difference	= ( DBOP_bag_union + 1 ) ,
        DBOP_bag_right_difference	= ( DBOP_bag_left_difference + 1 ) ,
        DBOP_bag_anti_difference	= ( DBOP_bag_right_difference + 1 ) ,
        DBOP_division	= ( DBOP_bag_anti_difference + 1 ) ,
        DBOP_relative_sampling	= ( DBOP_division + 1 ) ,
        DBOP_absolute_sampling	= ( DBOP_relative_sampling + 1 ) ,
        DBOP_transitive_closure	= ( DBOP_absolute_sampling + 1 ) ,
        DBOP_recursive_union	= ( DBOP_transitive_closure + 1 ) ,
        DBOP_aggregate	= ( DBOP_recursive_union + 1 ) ,
        DBOP_remote_table	= ( DBOP_aggregate + 1 ) ,
        DBOP_select	= ( DBOP_remote_table + 1 ) ,
        DBOP_order_preserving_select	= ( DBOP_select + 1 ) ,
        DBOP_project	= ( DBOP_order_preserving_select + 1 ) ,
        DBOP_project_order_preserving	= ( DBOP_project + 1 ) ,
        DBOP_top	= ( DBOP_project_order_preserving + 1 ) ,
        DBOP_top_percent	= ( DBOP_top + 1 ) ,
        DBOP_top_plus_ties	= ( DBOP_top_percent + 1 ) ,
        DBOP_top_percent_plus_ties	= ( DBOP_top_plus_ties + 1 ) ,
        DBOP_rank	= ( DBOP_top_percent_plus_ties + 1 ) ,
        DBOP_rank_ties_equally	= ( DBOP_rank + 1 ) ,
        DBOP_rank_ties_equally_and_skip	= ( DBOP_rank_ties_equally + 1 ) ,
        DBOP_navigate	= ( DBOP_rank_ties_equally_and_skip + 1 ) ,
        DBOP_nesting	= ( DBOP_navigate + 1 ) ,
        DBOP_unnesting	= ( DBOP_nesting + 1 ) ,
        DBOP_nested_apply	= ( DBOP_unnesting + 1 ) ,
        DBOP_cross_tab	= ( DBOP_nested_apply + 1 ) ,
        DBOP_is_NULL	= ( DBOP_cross_tab + 1 ) ,
        DBOP_is_NOT_NULL	= ( DBOP_is_NULL + 1 ) ,
        DBOP_equal	= ( DBOP_is_NOT_NULL + 1 ) ,
        DBOP_not_equal	= ( DBOP_equal + 1 ) ,
        DBOP_less	= ( DBOP_not_equal + 1 ) ,
        DBOP_less_equal	= ( DBOP_less + 1 ) ,
        DBOP_greater	= ( DBOP_less_equal + 1 ) ,
        DBOP_greater_equal	= ( DBOP_greater + 1 ) ,
        DBOP_equal_all	= ( DBOP_greater_equal + 1 ) ,
        DBOP_not_equal_all	= ( DBOP_equal_all + 1 ) ,
        DBOP_less_all	= ( DBOP_not_equal_all + 1 ) ,
        DBOP_less_equal_all	= ( DBOP_less_all + 1 ) ,
        DBOP_greater_all	= ( DBOP_less_equal_all + 1 ) ,
        DBOP_greater_equal_all	= ( DBOP_greater_all + 1 ) ,
        DBOP_equal_any	= ( DBOP_greater_equal_all + 1 ) ,
        DBOP_not_equal_any	= ( DBOP_equal_any + 1 ) ,
        DBOP_less_any	= ( DBOP_not_equal_any + 1 ) ,
        DBOP_less_equal_any	= ( DBOP_less_any + 1 ) ,
        DBOP_greater_any	= ( DBOP_less_equal_any + 1 ) ,
        DBOP_greater_equal_any	= ( DBOP_greater_any + 1 ) ,
        DBOP_anybits	= ( DBOP_greater_equal_any + 1 ) ,
        DBOP_allbits	= ( DBOP_anybits + 1 ) ,
        DBOP_anybits_any	= ( DBOP_allbits + 1 ) ,
        DBOP_allbits_any	= ( DBOP_anybits_any + 1 ) ,
        DBOP_anybits_all	= ( DBOP_allbits_any + 1 ) ,
        DBOP_allbits_all	= ( DBOP_anybits_all + 1 ) ,
        DBOP_between	= ( DBOP_allbits_all + 1 ) ,
        DBOP_between_unordered	= ( DBOP_between + 1 ) ,
        DBOP_match	= ( DBOP_between_unordered + 1 ) ,
        DBOP_match_unique	= ( DBOP_match + 1 ) ,
        DBOP_match_partial	= ( DBOP_match_unique + 1 ) ,
        DBOP_match_partial_unique	= ( DBOP_match_partial + 1 ) ,
        DBOP_match_full	= ( DBOP_match_partial_unique + 1 ) ,
        DBOP_match_full_unique	= ( DBOP_match_full + 1 ) ,
        DBOP_scalar_parameter	= ( DBOP_match_full_unique + 1 ) ,
        DBOP_scalar_function	= ( DBOP_scalar_parameter + 1 ) ,
        DBOP_plus	= ( DBOP_scalar_function + 1 ) ,
        DBOP_minus	= ( DBOP_plus + 1 ) ,
        DBOP_times	= ( DBOP_minus + 1 ) ,
        DBOP_over	= ( DBOP_times + 1 ) ,
        DBOP_div	= ( DBOP_over + 1 ) ,
        DBOP_modulo	= ( DBOP_div + 1 ) ,
        DBOP_power	= ( DBOP_modulo + 1 ) ,
        DBOP_like	= ( DBOP_power + 1 ) ,
        DBOP_sounds_like	= ( DBOP_like + 1 ) ,
        DBOP_like_any	= ( DBOP_sounds_like + 1 ) ,
        DBOP_like_all	= ( DBOP_like_any + 1 ) ,
        DBOP_is_INVALID	= ( DBOP_like_all + 1 ) ,
        DBOP_is_TRUE	= ( DBOP_is_INVALID + 1 ) ,
        DBOP_is_FALSE	= ( DBOP_is_TRUE + 1 ) ,
        DBOP_and	= ( DBOP_is_FALSE + 1 ) ,
        DBOP_or	= ( DBOP_and + 1 ) ,
        DBOP_xor	= ( DBOP_or + 1 ) ,
        DBOP_equivalent	= ( DBOP_xor + 1 ) ,
        DBOP_not	= ( DBOP_equivalent + 1 ) ,
        DBOP_implies	= ( DBOP_not + 1 ) ,
        DBOP_overlaps	= ( DBOP_implies + 1 ) ,
        DBOP_case_condition	= ( DBOP_overlaps + 1 ) ,
        DBOP_case_value	= ( DBOP_case_condition + 1 ) ,
        DBOP_nullif	= ( DBOP_case_value + 1 ) ,
        DBOP_cast	= ( DBOP_nullif + 1 ) ,
        DBOP_coalesce	= ( DBOP_cast + 1 ) ,
        DBOP_position	= ( DBOP_coalesce + 1 ) ,
        DBOP_extract	= ( DBOP_position + 1 ) ,
        DBOP_char_length	= ( DBOP_extract + 1 ) ,
        DBOP_octet_length	= ( DBOP_char_length + 1 ) ,
        DBOP_bit_length	= ( DBOP_octet_length + 1 ) ,
        DBOP_substring	= ( DBOP_bit_length + 1 ) ,
        DBOP_upper	= ( DBOP_substring + 1 ) ,
        DBOP_lower	= ( DBOP_upper + 1 ) ,
        DBOP_trim	= ( DBOP_lower + 1 ) ,
        DBOP_translate	= ( DBOP_trim + 1 ) ,
        DBOP_convert	= ( DBOP_translate + 1 ) ,
        DBOP_string_concat	= ( DBOP_convert + 1 ) ,
        DBOP_current_date	= ( DBOP_string_concat + 1 ) ,
        DBOP_current_time	= ( DBOP_current_date + 1 ) ,
        DBOP_current_timestamp	= ( DBOP_current_time + 1 ) ,
        DBOP_content_select	= ( DBOP_current_timestamp + 1 ) ,
        DBOP_content	= ( DBOP_content_select + 1 ) ,
        DBOP_content_freetext	= ( DBOP_content + 1 ) ,
        DBOP_content_proximity	= ( DBOP_content_freetext + 1 ) ,
        DBOP_content_vector_or	= ( DBOP_content_proximity + 1 ) ,
        DBOP_delete	= ( DBOP_content_vector_or + 1 ) ,
        DBOP_update	= ( DBOP_delete + 1 ) ,
        DBOP_insert	= ( DBOP_update + 1 ) ,
        DBOP_min	= ( DBOP_insert + 1 ) ,
        DBOP_max	= ( DBOP_min + 1 ) ,
        DBOP_count	= ( DBOP_max + 1 ) ,
        DBOP_sum	= ( DBOP_count + 1 ) ,
        DBOP_avg	= ( DBOP_sum + 1 ) ,
        DBOP_any_sample	= ( DBOP_avg + 1 ) ,
        DBOP_stddev	= ( DBOP_any_sample + 1 ) ,
        DBOP_stddev_pop	= ( DBOP_stddev + 1 ) ,
        DBOP_var	= ( DBOP_stddev_pop + 1 ) ,
        DBOP_var_pop	= ( DBOP_var + 1 ) ,
        DBOP_first	= ( DBOP_var_pop + 1 ) ,
        DBOP_last	= ( DBOP_first + 1 ) ,
        DBOP_in	= ( DBOP_last + 1 ) ,
        DBOP_exists	= ( DBOP_in + 1 ) ,
        DBOP_unique	= ( DBOP_exists + 1 ) ,
        DBOP_subset	= ( DBOP_unique + 1 ) ,
        DBOP_proper_subset	= ( DBOP_subset + 1 ) ,
        DBOP_superset	= ( DBOP_proper_subset + 1 ) ,
        DBOP_proper_superset	= ( DBOP_superset + 1 ) ,
        DBOP_disjoint	= ( DBOP_proper_superset + 1 ) ,
        DBOP_pass_through	= ( DBOP_disjoint + 1 ) ,
        DBOP_defined_by_GUID	= ( DBOP_pass_through + 1 ) ,
        DBOP_text_command	= ( DBOP_defined_by_GUID + 1 ) ,
        DBOP_SQL_select	= ( DBOP_text_command + 1 ) ,
        DBOP_prior_command_tree	= ( DBOP_SQL_select + 1 ) ,
        DBOP_add_columns	= ( DBOP_prior_command_tree + 1 ) ,
        DBOP_column_list_anchor	= ( DBOP_add_columns + 1 ) ,
        DBOP_column_list_element	= ( DBOP_column_list_anchor + 1 ) ,
        DBOP_command_list_anchor	= ( DBOP_column_list_element + 1 ) ,
        DBOP_command_list_element	= ( DBOP_command_list_anchor + 1 ) ,
        DBOP_from_list_anchor	= ( DBOP_command_list_element + 1 ) ,
        DBOP_from_list_element	= ( DBOP_from_list_anchor + 1 ) ,
        DBOP_project_list_anchor	= ( DBOP_from_list_element + 1 ) ,
        DBOP_project_list_element	= ( DBOP_project_list_anchor + 1 ) ,
        DBOP_row_list_anchor	= ( DBOP_project_list_element + 1 ) ,
        DBOP_row_list_element	= ( DBOP_row_list_anchor + 1 ) ,
        DBOP_scalar_list_anchor	= ( DBOP_row_list_element + 1 ) ,
        DBOP_scalar_list_element	= ( DBOP_scalar_list_anchor + 1 ) ,
        DBOP_set_list_anchor	= ( DBOP_scalar_list_element + 1 ) ,
        DBOP_set_list_element	= ( DBOP_set_list_anchor + 1 ) ,
        DBOP_sort_list_anchor	= ( DBOP_set_list_element + 1 ) ,
        DBOP_sort_list_element	= ( DBOP_sort_list_anchor + 1 ) ,
        DBOP_alter_character_set	= ( DBOP_sort_list_element + 1 ) ,
        DBOP_alter_collation	= ( DBOP_alter_character_set + 1 ) ,
        DBOP_alter_domain	= ( DBOP_alter_collation + 1 ) ,
        DBOP_alter_index	= ( DBOP_alter_domain + 1 ) ,
        DBOP_alter_procedure	= ( DBOP_alter_index + 1 ) ,
        DBOP_alter_schema	= ( DBOP_alter_procedure + 1 ) ,
        DBOP_alter_table	= ( DBOP_alter_schema + 1 ) ,
        DBOP_alter_trigger	= ( DBOP_alter_table + 1 ) ,
        DBOP_alter_view	= ( DBOP_alter_trigger + 1 ) ,
        DBOP_coldef_list_anchor	= ( DBOP_alter_view + 1 ) ,
        DBOP_coldef_list_element	= ( DBOP_coldef_list_anchor + 1 ) ,
        DBOP_create_assertion	= ( DBOP_coldef_list_element + 1 ) ,
        DBOP_create_character_set	= ( DBOP_create_assertion + 1 ) ,
        DBOP_create_collation	= ( DBOP_create_character_set + 1 ) ,
        DBOP_create_domain	= ( DBOP_create_collation + 1 ) ,
        DBOP_create_index	= ( DBOP_create_domain + 1 ) ,
        DBOP_create_procedure	= ( DBOP_create_index + 1 ) ,
        DBOP_create_schema	= ( DBOP_create_procedure + 1 ) ,
        DBOP_create_synonym	= ( DBOP_create_schema + 1 ) ,
        DBOP_create_table	= ( DBOP_create_synonym + 1 ) ,
        DBOP_create_temporary_table	= ( DBOP_create_table + 1 ) ,
        DBOP_create_translation	= ( DBOP_create_temporary_table + 1 ) ,
        DBOP_create_trigger	= ( DBOP_create_translation + 1 ) ,
        DBOP_create_view	= ( DBOP_create_trigger + 1 ) ,
        DBOP_drop_assertion	= ( DBOP_create_view + 1 ) ,
        DBOP_drop_character_set	= ( DBOP_drop_assertion + 1 ) ,
        DBOP_drop_collation	= ( DBOP_drop_character_set + 1 ) ,
        DBOP_drop_domain	= ( DBOP_drop_collation + 1 ) ,
        DBOP_drop_index	= ( DBOP_drop_domain + 1 ) ,
        DBOP_drop_procedure	= ( DBOP_drop_index + 1 ) ,
        DBOP_drop_schema	= ( DBOP_drop_procedure + 1 ) ,
        DBOP_drop_synonym	= ( DBOP_drop_schema + 1 ) ,
        DBOP_drop_table	= ( DBOP_drop_synonym + 1 ) ,
        DBOP_drop_translation	= ( DBOP_drop_table + 1 ) ,
        DBOP_drop_trigger	= ( DBOP_drop_translation + 1 ) ,
        DBOP_drop_view	= ( DBOP_drop_trigger + 1 ) ,
        DBOP_foreign_key	= ( DBOP_drop_view + 1 ) ,
        DBOP_grant_privileges	= ( DBOP_foreign_key + 1 ) ,
        DBOP_index_list_anchor	= ( DBOP_grant_privileges + 1 ) ,
        DBOP_index_list_element	= ( DBOP_index_list_anchor + 1 ) ,
        DBOP_primary_key	= ( DBOP_index_list_element + 1 ) ,
        DBOP_property_list_anchor	= ( DBOP_primary_key + 1 ) ,
        DBOP_property_list_element	= ( DBOP_property_list_anchor + 1 ) ,
        DBOP_referenced_table	= ( DBOP_property_list_element + 1 ) ,
        DBOP_rename_object	= ( DBOP_referenced_table + 1 ) ,
        DBOP_revoke_privileges	= ( DBOP_rename_object + 1 ) ,
        DBOP_schema_authorization	= ( DBOP_revoke_privileges + 1 ) ,
        DBOP_unique_key	= ( DBOP_schema_authorization + 1 ) ,
        DBOP_scope_list_anchor	= ( DBOP_unique_key + 1 ) ,
        DBOP_scope_list_element	= ( DBOP_scope_list_anchor + 1 ) ,
        DBOP_content_table	= ( DBOP_scope_list_element + 1 ) 
    } ;
#ifdef DBINITCONSTANTS
extern const OLEDBDECLSPEC GUID DBGUID_LIKE_SQL                        = {0xc8b521f6,0x5cf3,0x11ce,{0xad,0xe5,0x00,0xaa,0x00,0x44,0x77,0x3d}};
extern const OLEDBDECLSPEC GUID DBGUID_LIKE_DOS                        = {0xc8b521f7,0x5cf3,0x11ce,{0xad,0xe5,0x00,0xaa,0x00,0x44,0x77,0x3d}};
extern const OLEDBDECLSPEC GUID DBGUID_LIKE_OFS                        = {0xc8b521f8,0x5cf3,0x11ce,{0xad,0xe5,0x00,0xaa,0x00,0x44,0x77,0x3d}};
extern const OLEDBDECLSPEC GUID DBGUID_LIKE_MAPI                       = {0xc8b521f9,0x5cf3,0x11ce,{0xad,0xe5,0x00,0xaa,0x00,0x44,0x77,0x3d}};
#else // !DBINITCONSTANTS
extern const GUID DBGUID_LIKE_SQL;
extern const GUID DBGUID_LIKE_DOS;
extern const GUID DBGUID_LIKE_OFS;
extern const GUID DBGUID_LIKE_MAPI;
#endif // DBINITCONSTANTS


extern RPC_IF_HANDLE CommandTreeStructureDefinitions_v0_0_c_ifspec;
extern RPC_IF_HANDLE CommandTreeStructureDefinitions_v0_0_s_ifspec;
#endif /* __CommandTreeStructureDefinitions_INTERFACE_DEFINED__ */

/* interface __MIDL_itf_cmdtree_0000_0001 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#pragma warning(push)
#pragma warning(disable:4001) 
#pragma once
#pragma warning(push)
#pragma warning(disable:4001) 
#pragma once
#pragma warning(pop)
#pragma warning(pop)
#pragma region Desktop Family
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_cmdtree_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_cmdtree_0000_0001_v0_0_s_ifspec;

#ifndef __ICommandTree_INTERFACE_DEFINED__
#define __ICommandTree_INTERFACE_DEFINED__

/* interface ICommandTree */
/* [unique][uuid][object][local] */ 

typedef DWORD DBCOMMANDREUSE;


enum DBCOMMANDREUSEENUM
    {
        DBCOMMANDREUSE_NONE	= 0,
        DBCOMMANDREUSE_PROPERTIES	= 0x1,
        DBCOMMANDREUSE_PARAMETERS	= 0x2
    } ;
typedef DWORD DBVALUEKIND;


enum DBVALUEKINDENUM
    {
        DBVALUEKIND_BYGUID	= 256,
        DBVALUEKIND_COLDESC	= ( DBVALUEKIND_BYGUID + 1 ) ,
        DBVALUEKIND_ID	= ( DBVALUEKIND_COLDESC + 1 ) ,
        DBVALUEKIND_CONTENT	= ( DBVALUEKIND_ID + 1 ) ,
        DBVALUEKIND_CONTENTVECTOR	= ( DBVALUEKIND_CONTENT + 1 ) ,
        DBVALUEKIND_GROUPINFO	= ( DBVALUEKIND_CONTENTVECTOR + 1 ) ,
        DBVALUEKIND_PARAMETER	= ( DBVALUEKIND_GROUPINFO + 1 ) ,
        DBVALUEKIND_PROPERTY	= ( DBVALUEKIND_PARAMETER + 1 ) ,
        DBVALUEKIND_SETFUNC	= ( DBVALUEKIND_PROPERTY + 1 ) ,
        DBVALUEKIND_SORTINFO	= ( DBVALUEKIND_SETFUNC + 1 ) ,
        DBVALUEKIND_TEXT	= ( DBVALUEKIND_SORTINFO + 1 ) ,
        DBVALUEKIND_COMMAND	= ( DBVALUEKIND_TEXT + 1 ) ,
        DBVALUEKIND_MONIKER	= ( DBVALUEKIND_COMMAND + 1 ) ,
        DBVALUEKIND_ROWSET	= ( DBVALUEKIND_MONIKER + 1 ) ,
        DBVALUEKIND_LIKE	= ( DBVALUEKIND_ROWSET + 1 ) ,
        DBVALUEKIND_CONTENTPROXIMITY	= ( DBVALUEKIND_LIKE + 1 ) ,
        DBVALUEKIND_CONTENTSCOPE	= ( DBVALUEKIND_CONTENTPROXIMITY + 1 ) ,
        DBVALUEKIND_CONTENTTABLE	= ( DBVALUEKIND_CONTENTSCOPE + 1 ) ,
        DBVALUEKIND_IDISPATCH	= 9,
        DBVALUEKIND_IUNKNOWN	= 13,
        DBVALUEKIND_EMPTY	= 0,
        DBVALUEKIND_NULL	= 1,
        DBVALUEKIND_I2	= 2,
        DBVALUEKIND_I4	= 3,
        DBVALUEKIND_R4	= 4,
        DBVALUEKIND_R8	= 5,
        DBVALUEKIND_CY	= 6,
        DBVALUEKIND_DATE	= 7,
        DBVALUEKIND_BSTR	= 8,
        DBVALUEKIND_ERROR	= 10,
        DBVALUEKIND_BOOL	= 11,
        DBVALUEKIND_VARIANT	= 12,
        DBVALUEKIND_VECTOR	= 0x1000,
        DBVALUEKIND_ARRAY	= 0x2000,
        DBVALUEKIND_BYREF	= 0x4000,
        DBVALUEKIND_I1	= 16,
        DBVALUEKIND_UI1	= 17,
        DBVALUEKIND_UI2	= 18,
        DBVALUEKIND_UI4	= ( DBVALUEKIND_UI2 + 1 ) ,
        DBVALUEKIND_I8	= ( DBVALUEKIND_UI4 + 1 ) ,
        DBVALUEKIND_UI8	= ( DBVALUEKIND_I8 + 1 ) ,
        DBVALUEKIND_GUID	= 72,
        DBVALUEKIND_BYTES	= 128,
        DBVALUEKIND_STR	= 129,
        DBVALUEKIND_WSTR	= 130,
        DBVALUEKIND_NUMERIC	= 131,
        DBVALUEKIND_DBDATE	= 133,
        DBVALUEKIND_DBTIME	= 134,
        DBVALUEKIND_DBTIMESTAMP	= 135,
        DBVALUEKIND_PROBABILISTIC	= 136,
        DBVALUEKIND_RELEVANTDOCUMENT	= 137
    } ;
typedef struct tagDBBYGUID
    {
    /* [size_is] */ BYTE *pbInfo;
    DBLENGTH cbInfo;
    GUID guid;
    } 	DBBYGUID;

#define GENERATE_METHOD_EXACT    ( 0 )
#define GENERATE_METHOD_PREFIX   ( 1 )
#define GENERATE_METHOD_INFLECT  ( 2 )
typedef struct tagDBCONTENT
    {
    LPOLESTR pwszPhrase;
    DWORD dwGenerateMethod;
    LONG lWeight;
    LCID lcid;
    } 	DBCONTENT;

#define SCOPE_FLAG_MASK      ( 0x000000ff )
#define SCOPE_FLAG_INCLUDE   ( 0x00000001 )
#define SCOPE_FLAG_DEEP      ( 0x00000002 )
#define SCOPE_TYPE_MASK      ( 0xffffff00 )
#define SCOPE_TYPE_WINPATH   ( 0x00000100 )
#define SCOPE_TYPE_VPATH     ( 0x00000200 )
typedef struct tagDBCONTENTSCOPE
    {
    DWORD dwFlags;
    LPOLESTR *rgpwszTagName;
    LPOLESTR pwszElementValue;
    } 	DBCONTENTSCOPE;

typedef struct tagDBCONTENTTABLE
    {
    LPOLESTR pwszMachine;
    LPOLESTR pwszCatalog;
    } 	DBCONTENTTABLE;

#define PROPID_QUERY_RANKVECTOR  ( 0x2 )
#define PROPID_QUERY_RANK        ( 0x3 )
#define PROPID_QUERY_HITCOUNT    ( 0x4 )
#define PROPID_QUERY_ALL         ( 0x6 )
#define PROPID_STG_CONTENTS      ( 0x13 )
#define VECTOR_RANK_MIN          ( 0 )
#define VECTOR_RANK_MAX          ( 1 )
#define VECTOR_RANK_INNER        ( 2 )
#define VECTOR_RANK_DICE         ( 3 )
#define VECTOR_RANK_JACCARD      ( 4 )
typedef struct tagDBCONTENTVECTOR
    {
    LONG lWeight;
    DWORD dwRankingMethod;
    } 	DBCONTENTVECTOR;

typedef struct tagDBGROUPINFO
    {
    LCID lcid;
    } 	DBGROUPINFO;

typedef struct tagDBPARAMETER
    {
    LPOLESTR pwszName;
    ITypeInfo *pTypeInfo;
    DB_NUMERIC *pNum;
    DBLENGTH cbMaxLength;
    DBPARAMFLAGS dwFlags;
    DBTYPE wType;
    } 	DBPARAMETER;

#define DBSETFUNC_NONE       0x0
#define DBSETFUNC_ALL        0x1
#define DBSETFUNC_DISTINCT   0x2
typedef struct tagDBSETFUNC
    {
    DWORD dwSetQuantifier;
    } 	DBSETFUNC;

typedef struct tagDBSORTINFO
    {
    BOOL fDesc;
    LCID lcid;
    } 	DBSORTINFO;

typedef struct tagDBTEXT
    {
    LPOLESTR pwszText;
    ULONG ulErrorLocator;
    ULONG ulTokenLength;
    GUID guidDialect;
    } 	DBTEXT;

typedef struct tagDBLIKE
    {
    LONG lWeight;
    GUID guidDialect;
    } 	DBLIKE;

#define PROXIMITY_UNIT_WORD           ( 0 )
#define PROXIMITY_UNIT_SENTENCE       ( 1 )
#define PROXIMITY_UNIT_PARAGRAPH      ( 2 )
#define PROXIMITY_UNIT_CHAPTER        ( 3 )
typedef struct tagDBCONTENTPROXIMITY
    {
    DWORD dwProximityUnit;
    ULONG ulProximityDistance;
    LONG lWeight;
    } 	DBCONTENTPROXIMITY;

typedef struct tagDBPROBABILISTIC
    {
    LONG lWeight;
    float flK1;
    float flK2;
    float flK3;
    float flB;
    } 	DBPROBABILISTIC;

typedef struct tagDBRELEVANTDOCUMENT
    {
    LONG lWeight;
    VARIANT vDocument;
    } 	DBRELEVANTDOCUMENT;

typedef struct tagDBCOMMANDTREE
    {
    DBCOMMANDOP op;
    WORD wKind;
    struct tagDBCOMMANDTREE *pctFirstChild;
    struct tagDBCOMMANDTREE *pctNextSibling;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */ __int64 llValue;
        /* [case()] */ unsigned __int64 ullValue;
        /* [case()] */ BOOL fValue;
        /* [case()] */ unsigned char uchValue;
        /* [case()] */ signed char schValue;
        /* [case()] */ unsigned short usValue;
        /* [case()] */ short sValue;
        /* [case()] */ LPOLESTR pwszValue;
        /* [case()] */ LONG lValue;
        /* [case()] */ ULONG ulValue;
        /* [case()] */ float flValue;
        /* [case()] */ double dblValue;
        /* [case()] */ CY cyValue;
        /* [case()] */ DATE dateValue;
        /* [case()] */ DBDATE dbdateValue;
        /* [case()] */ DBTIME dbtimeValue;
        /* [case()] */ SCODE scodeValue;
        /* [case()] */ BSTR *pbstrValue;
        /* [case()] */ ICommand *pCommand;
        /* [case()] */ IDispatch *pDispatch;
        /* [case()] */ IMoniker *pMoniker;
        /* [case()] */ IRowset *pRowset;
        /* [case()] */ IUnknown *pUnknown;
        /* [case()] */ DBBYGUID *pdbbygdValue;
        /* [case()] */ DBCOLUMNDESC *pcoldescValue;
        /* [case()] */ DBID *pdbidValue;
        /* [case()] */ DBLIKE *pdblikeValue;
        /* [case()] */ DBCONTENT *pdbcntntValue;
        /* [case()] */ DBCONTENTSCOPE *pdbcntntscpValue;
        /* [case()] */ DBCONTENTTABLE *pdbcntnttblValue;
        /* [case()] */ DBCONTENTVECTOR *pdbcntntvcValue;
        /* [case()] */ DBCONTENTPROXIMITY *pdbcntntproxValue;
        /* [case()] */ DBGROUPINFO *pdbgrpinfValue;
        /* [case()] */ DBPARAMETER *pdbparamValue;
        /* [case()] */ DBPROPSET *pdbpropValue;
        /* [case()] */ DBSETFUNC *pdbstfncValue;
        /* [case()] */ DBSORTINFO *pdbsrtinfValue;
        /* [case()] */ DBTEXT *pdbtxtValue;
        /* [case()] */ DBVECTOR *pdbvectorValue;
        /* [case()] */ SAFEARRAY *parrayValue;
        /* [case()] */ VARIANT *pvarValue;
        /* [case()] */ GUID *pGuid;
        /* [case()] */ BYTE *pbValue;
        /* [case()] */ char *pzValue;
        /* [case()] */ DB_NUMERIC *pdbnValue;
        /* [case()] */ DBTIMESTAMP *pdbtsValue;
        /* [case()] */ void *pvValue;
        /* [case()] */ DBPROBABILISTIC *pdbprobValue;
        /* [case()] */ DBRELEVANTDOCUMENT *pdbreldocValue;
        } 	value;
    HRESULT hrError;
    } 	DBCOMMANDTREE;


EXTERN_C const IID IID_ICommandTree;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0c733a87-2a1c-11ce-ade5-00aa0044773d")
    ICommandTree : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE FindErrorNodes( 
            /* [in] */ const DBCOMMANDTREE *pRoot,
            /* [out] */ ULONG *pcErrorNodes,
            /* [out] */ DBCOMMANDTREE ***prgErrorNodes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FreeCommandTree( 
            /* [in] */ DBCOMMANDTREE **ppRoot) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCommandTree( 
            /* [out] */ DBCOMMANDTREE **ppRoot) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCommandTree( 
            /* [in] */ DBCOMMANDTREE **ppRoot,
            /* [in] */ DBCOMMANDREUSE dwCommandReuse,
            /* [in] */ BOOL fCopy) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICommandTreeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICommandTree * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICommandTree * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICommandTree * This);
        
        DECLSPEC_XFGVIRT(ICommandTree, FindErrorNodes)
        HRESULT ( STDMETHODCALLTYPE *FindErrorNodes )( 
            ICommandTree * This,
            /* [in] */ const DBCOMMANDTREE *pRoot,
            /* [out] */ ULONG *pcErrorNodes,
            /* [out] */ DBCOMMANDTREE ***prgErrorNodes);
        
        DECLSPEC_XFGVIRT(ICommandTree, FreeCommandTree)
        HRESULT ( STDMETHODCALLTYPE *FreeCommandTree )( 
            ICommandTree * This,
            /* [in] */ DBCOMMANDTREE **ppRoot);
        
        DECLSPEC_XFGVIRT(ICommandTree, GetCommandTree)
        HRESULT ( STDMETHODCALLTYPE *GetCommandTree )( 
            ICommandTree * This,
            /* [out] */ DBCOMMANDTREE **ppRoot);
        
        DECLSPEC_XFGVIRT(ICommandTree, SetCommandTree)
        HRESULT ( STDMETHODCALLTYPE *SetCommandTree )( 
            ICommandTree * This,
            /* [in] */ DBCOMMANDTREE **ppRoot,
            /* [in] */ DBCOMMANDREUSE dwCommandReuse,
            /* [in] */ BOOL fCopy);
        
        END_INTERFACE
    } ICommandTreeVtbl;

    interface ICommandTree
    {
        CONST_VTBL struct ICommandTreeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICommandTree_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICommandTree_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICommandTree_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICommandTree_FindErrorNodes(This,pRoot,pcErrorNodes,prgErrorNodes)	\
    ( (This)->lpVtbl -> FindErrorNodes(This,pRoot,pcErrorNodes,prgErrorNodes) ) 

#define ICommandTree_FreeCommandTree(This,ppRoot)	\
    ( (This)->lpVtbl -> FreeCommandTree(This,ppRoot) ) 

#define ICommandTree_GetCommandTree(This,ppRoot)	\
    ( (This)->lpVtbl -> GetCommandTree(This,ppRoot) ) 

#define ICommandTree_SetCommandTree(This,ppRoot,dwCommandReuse,fCopy)	\
    ( (This)->lpVtbl -> SetCommandTree(This,ppRoot,dwCommandReuse,fCopy) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICommandTree_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_cmdtree_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_cmdtree_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_cmdtree_0000_0002_v0_0_s_ifspec;

#ifndef __IQuery_INTERFACE_DEFINED__
#define __IQuery_INTERFACE_DEFINED__

/* interface IQuery */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IQuery;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0c733a51-2a1c-11ce-ade5-00aa0044773d")
    IQuery : public ICommandTree
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddPostProcessing( 
            /* [in] */ DBCOMMANDTREE **ppRoot,
            /* [in] */ BOOL fCopy) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCardinalityEstimate( 
            /* [out] */ DBORDINAL *pulCardinality) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IQueryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IQuery * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IQuery * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IQuery * This);
        
        DECLSPEC_XFGVIRT(ICommandTree, FindErrorNodes)
        HRESULT ( STDMETHODCALLTYPE *FindErrorNodes )( 
            IQuery * This,
            /* [in] */ const DBCOMMANDTREE *pRoot,
            /* [out] */ ULONG *pcErrorNodes,
            /* [out] */ DBCOMMANDTREE ***prgErrorNodes);
        
        DECLSPEC_XFGVIRT(ICommandTree, FreeCommandTree)
        HRESULT ( STDMETHODCALLTYPE *FreeCommandTree )( 
            IQuery * This,
            /* [in] */ DBCOMMANDTREE **ppRoot);
        
        DECLSPEC_XFGVIRT(ICommandTree, GetCommandTree)
        HRESULT ( STDMETHODCALLTYPE *GetCommandTree )( 
            IQuery * This,
            /* [out] */ DBCOMMANDTREE **ppRoot);
        
        DECLSPEC_XFGVIRT(ICommandTree, SetCommandTree)
        HRESULT ( STDMETHODCALLTYPE *SetCommandTree )( 
            IQuery * This,
            /* [in] */ DBCOMMANDTREE **ppRoot,
            /* [in] */ DBCOMMANDREUSE dwCommandReuse,
            /* [in] */ BOOL fCopy);
        
        DECLSPEC_XFGVIRT(IQuery, AddPostProcessing)
        HRESULT ( STDMETHODCALLTYPE *AddPostProcessing )( 
            IQuery * This,
            /* [in] */ DBCOMMANDTREE **ppRoot,
            /* [in] */ BOOL fCopy);
        
        DECLSPEC_XFGVIRT(IQuery, GetCardinalityEstimate)
        HRESULT ( STDMETHODCALLTYPE *GetCardinalityEstimate )( 
            IQuery * This,
            /* [out] */ DBORDINAL *pulCardinality);
        
        END_INTERFACE
    } IQueryVtbl;

    interface IQuery
    {
        CONST_VTBL struct IQueryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IQuery_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IQuery_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IQuery_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IQuery_FindErrorNodes(This,pRoot,pcErrorNodes,prgErrorNodes)	\
    ( (This)->lpVtbl -> FindErrorNodes(This,pRoot,pcErrorNodes,prgErrorNodes) ) 

#define IQuery_FreeCommandTree(This,ppRoot)	\
    ( (This)->lpVtbl -> FreeCommandTree(This,ppRoot) ) 

#define IQuery_GetCommandTree(This,ppRoot)	\
    ( (This)->lpVtbl -> GetCommandTree(This,ppRoot) ) 

#define IQuery_SetCommandTree(This,ppRoot,dwCommandReuse,fCopy)	\
    ( (This)->lpVtbl -> SetCommandTree(This,ppRoot,dwCommandReuse,fCopy) ) 


#define IQuery_AddPostProcessing(This,ppRoot,fCopy)	\
    ( (This)->lpVtbl -> AddPostProcessing(This,ppRoot,fCopy) ) 

#define IQuery_GetCardinalityEstimate(This,pulCardinality)	\
    ( (This)->lpVtbl -> GetCardinalityEstimate(This,pulCardinality) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IQuery_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_cmdtree_0000_0003 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#include <poppack.h>     // restore original structure packing
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_cmdtree_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_cmdtree_0000_0003_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


