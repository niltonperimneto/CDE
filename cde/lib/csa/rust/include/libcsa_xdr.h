#ifndef LIBCSA_XDR_H
#define LIBCSA_XDR_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <rpc/clnt.h>
#include <csa/csa.h>
#include "cm.h"
#include "connection.h"
#include "calendar.h"
#include "rtable4.h"
#include "rtable3.h"
#include "rtable2.h"

typedef enum clnt_stat clnt_stat;

typedef enum Update_Status {
  update_succeeded = 0,
  update_failed = 1,
} Update_Status;

int32_t xdr_Update_Status(void *_xdrs, enum Update_Status *_obj);

extern bool_t xdr_CSA_return_code(XDR *xdrs, CSA_return_code *objp);

CSA_return_code *cms_create_calendar_5(cms_create_args *arg, _DtCm_Connection *conn);

CSA_return_code *cms_delete_entry_5(cms_delete_args *arg, _DtCm_Connection *conn);

cms_open_res *cms_open_calendar_5(cms_open_args *arg, _DtCm_Connection *conn);

cms_entry_res *cms_update_entry_5(cms_update_args *arg, _DtCm_Connection *conn);

cms_entry_res *cms_insert_entry_5(cms_insert_args *arg, _DtCm_Connection *conn);

CSA_return_code *cms_remove_calendar_5(cms_remove_args *arg, _DtCm_Connection *conn);

cms_archive_res *cms_archive_5(cms_archive_args *arg, _DtCm_Connection *conn);

CSA_return_code *cms_restore_5(cms_restore_args *arg, _DtCm_Connection *conn);

cms_reminder_res *cms_lookup_reminder_5(cms_reminder_args *arg, _DtCm_Connection *conn);

cms_entries_res *cms_lookup_entries_5(cms_lookup_entries_args *arg, _DtCm_Connection *conn);

cms_get_cal_attr_res *cms_get_calendar_attr_5(cms_get_cal_attr_args *arg, _DtCm_Connection *conn);

CSA_return_code *cms_set_calendar_attr_5(cms_set_cal_attr_args *arg, _DtCm_Connection *conn);

cms_get_entry_attr_res *cms_get_entry_attr_5(cms_get_entry_attr_args *arg, _DtCm_Connection *conn);

CSA_return_code *cms_register_5(cms_register_args *arg, _DtCm_Connection *conn);

CSA_return_code *cms_unregister_5(cms_register_args *arg, _DtCm_Connection *conn);

cms_entries_res *cms_enumerate_sequence_5(cms_enumerate_args *arg, _DtCm_Connection *conn);

cms_enumerate_calendar_attr_res *cms_enumerate_calendar_attr_5(char **arg, _DtCm_Connection *conn);

cms_list_calendars_res *cms_list_calendars_5(void *_arg, _DtCm_Connection *conn);

extern CSA_return_code _DtCm_create_tcp_client(char *host,
                                               int version,
                                               int timeout,
                                               _DtCm_Client_Info **clnt);

extern CSA_return_code _DtCm_clntstat_to_csastat(clnt_stat clntstat);

extern CSA_return_code _DtCm_rpc_open_calendar_legacy(Calendar *cal);

extern int getpid(void);

extern CSA_return_code csa2cmsattrs(unsigned long num_attrs,
                                    CSA_attribute *csaattrs,
                                    CSA_enum *ops,
                                    unsigned long *newnum,
                                    cms_attribute **newattrs,
                                    CSA_enum **newops);

extern void free_cmsattrs(unsigned long num_attrs, cms_attribute *attrs);

extern CSA_return_code _DtCm_rpc_create_calendar_legacy(Calendar *cal,
                                                        unsigned long num_attrs,
                                                        CSA_attribute *attrs);

/**
 * Rust implementation of the v5 RPC open calendar logic
 *
 * This function handles the modern (v5) protocol path in Rust,
 * while falling back to C for legacy versions (v2-v4).
 */
CSA_return_code rs_rpc_open_calendar(Calendar *cal);

/**
 * Rust implementation of the v5 RPC create calendar logic
 */
CSA_return_code rs_rpc_create_calendar(Calendar *cal,
                                       unsigned long num_attrs,
                                       CSA_attribute *attrs);

bool_t xdr_CSA_return_code(XDR *xdrs, CSA_return_code *objp);

Table_Res_4 *_DtCm_rtable_change_4(Table_Args_4 *_arg, _DtCm_Connection *conn);

Table_Status_4 *_DtCm_rtable_create_4(Table_Op_Args_4 *_arg, _DtCm_Connection *_conn);

Table_Res_4 *_DtCm_rtable_lookup_4(Table_Args_4 *_arg, _DtCm_Connection *_conn);

Table_Res_4 *_DtCm_rtable_lookup_range_4(Table_Args_4 *_arg, _DtCm_Connection *_conn);

Table_Res_4 *_DtCm_rtable_insert_4(Table_Args_4 *_arg, _DtCm_Connection *_conn);

Table_Res_4 *_DtCm_rtable_abbreviated_lookup_key_range_4(Table_Args_4 *_arg,
                                                         _DtCm_Connection *_conn);

Access_Status_4 *_DtCm_rtable_set_access_4(Access_Args_4 *_arg, _DtCm_Connection *_conn);

Access_Args_4 *_DtCm_rtable_get_access_4(Access_Args_4 *_arg, _DtCm_Connection *_conn);

Table_Res_3 *_DtCm_rtable_lookup_3(Table_Args_3 *_arg, _DtCm_Connection *_conn);

Table_Res_2 *_DtCm_rtable_lookup_2(Table_Args_2 *_arg, _DtCm_Connection *_conn);

Table_Res_3 *_DtCm_rtable_lookup_range_3(Table_Args_3 *_arg, _DtCm_Connection *_conn);

Table_Res_2 *_DtCm_rtable_lookup_range_2(Table_Args_2 *_arg, _DtCm_Connection *_conn);

Access_Args_3 *_DtCm_rtable_get_access_3(Access_Args_3 *_arg, _DtCm_Connection *_conn);

Access_Args_2 *_DtCm_rtable_get_access_2(Access_Args_2 *_arg, _DtCm_Connection *_conn);

Access_Status_3 *_DtCm_rtable_set_access_3(Access_Args_3 *_arg, _DtCm_Connection *_conn);

Access_Status_2 *_DtCm_rtable_set_access_2(Access_Args_2 *_arg, _DtCm_Connection *_conn);

Table_Res_3 *_DtCm_rtable_change_3(Table_Args_3 *_arg, _DtCm_Connection *_conn);

Table_Res_2 *_DtCm_rtable_change_2(Table_Args_2 *_arg, _DtCm_Connection *_conn);

Table_Res_4 *_DtCm_rtable_delete_4(Table_Args_4 *_arg, _DtCm_Connection *_conn);

Table_Res_3 *_DtCm_rtable_delete_3(Table_Args_3 *_arg, _DtCm_Connection *_conn);

Table_Res_2 *_DtCm_rtable_delete_2(Table_Args_2 *_arg, _DtCm_Connection *_conn);

int *_DtCm_rtable_size_4(Table_Args_4 *_arg, _DtCm_Connection *_conn);

int *_DtCm_rtable_size_3(Table_Args_3 *_arg, _DtCm_Connection *_conn);

int *_DtCm_rtable_size_2(Table_Args_2 *_arg, _DtCm_Connection *_conn);

Table_Res_4 *_DtCm_rtable_lookup_next_reminder_4(Table_Args_4 *_arg, _DtCm_Connection *_conn);

Table_Res_3 *_DtCm_rtable_lookup_next_reminder_3(Table_Args_3 *_arg, _DtCm_Connection *_conn);

Table_Res_2 *_DtCm_rtable_lookup_next_reminder_2(Table_Args_2 *_arg, _DtCm_Connection *_conn);

Table_Status_4 *_DtCm_rtable_check_4(Table_Args_4 *_arg, _DtCm_Connection *_conn);

Table_Status_3 *_DtCm_rtable_check_3(Table_Args_3 *_arg, _DtCm_Connection *_conn);

Table_Status_2 *_DtCm_rtable_check_2(Table_Args_2 *_arg, _DtCm_Connection *_conn);

Table_Status_4 *_DtCm_rtable_flush_table_4(Table_Args_4 *_arg, _DtCm_Connection *_conn);

Table_Status_4 *_DtCm_rtable_garbage_collect_4(Table_Op_Args_4 *_arg, _DtCm_Connection *_conn);

Table_Res_4 *_DtCm_rtable_change_instance_4(Table_Args_4 *arg, _DtCm_Connection *conn);

Table_Res_3 *_DtCm_rtable_change_instance_3(Table_Args_3 *arg, _DtCm_Connection *conn);

Table_Res_2 *_DtCm_rtable_change_instance_2(Table_Args_2 *arg, _DtCm_Connection *conn);

Table_Res_4 *_DtCm_rtable_delete_instance_4(Table_Args_4 *arg, _DtCm_Connection *conn);

Table_Res_3 *_DtCm_rtable_delete_instance_3(Table_Args_3 *arg, _DtCm_Connection *conn);

Table_Res_2 *_DtCm_rtable_delete_instance_2(Table_Args_2 *arg, _DtCm_Connection *conn);

Table_Res_3 *_DtCm_rtable_insert_3(Table_Args_3 *_arg, _DtCm_Connection *_conn);

Table_Res_2 *_DtCm_rtable_insert_2(Table_Args_2 *_arg, _DtCm_Connection *_conn);

Table_Res_3 *_DtCm_rtable_abbreviated_lookup_key_range_3(Table_Args_3 *_arg,
                                                         _DtCm_Connection *_conn);

Registration_Status_4 *_DtCm_register_callback_4(Registration_4 *_arg, _DtCm_Connection *_conn);

Registration_Status_3 *_DtCm_register_callback_3(Registration_3 *_arg, _DtCm_Connection *_conn);

Registration_Status_2 *_DtCm_register_callback_2(Registration_2 *_arg, _DtCm_Connection *_conn);

Registration_Status_4 *_DtCm_deregister_callback_4(Registration_4 *_arg, _DtCm_Connection *_conn);

Registration_Status_3 *_DtCm_deregister_callback_3(Registration_3 *_arg, _DtCm_Connection *_conn);

Registration_Status_2 *_DtCm_deregister_callback_2(Registration_2 *_arg, _DtCm_Connection *_conn);

#endif /* LIBCSA_XDR_H */
