/*
 * CDE - Common Desktop Environment
 *
 * Rust FFI bindings for libcsa dispatch logic
 */

#ifndef _RUST_DISPATCH_H
#define _RUST_DISPATCH_H

#include "calendar.h"
#include "csa.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Rust implementation of RPC calendar open for v5+ */
extern CSA_return_code rs_rpc_open_calendar(Calendar *cal);

extern CSA_return_code rs_rpc_create_calendar(Calendar *cal,
                                              CSA_uint32 num_attrs,
                                              CSA_attribute *attrs);

/* Legacy C implementation for v2-v4 (called from Rust) */
extern CSA_return_code _DtCm_rpc_open_calendar_legacy(Calendar *cal);

extern CSA_return_code _DtCm_rpc_create_calendar_legacy(Calendar *cal,
                                                        CSA_uint32 num_attrs,
                                                        CSA_attribute *attrs);

#ifdef __cplusplus
}
#endif

#endif /* _RUST_DISPATCH_H */
