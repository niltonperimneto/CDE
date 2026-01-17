/*%%  (c) Copyright 1993, 1994 Hewlett-Packard Company			 */
/*%%  (c) Copyright 1993, 1994 International Business Machines Corp.	 */
/*%%  (c) Copyright 1993, 1994 Sun Microsystems, Inc.			 */
/*%%  (c) Copyright 1993, 1994 Novell, Inc. 				 */
/*%%  $XConsortium: tt_parse_util_funcs.C /main/3 1995/10/23 10:45:18 rswiston $
 */
/*
 *
 * tt_parse_util_funcs.cc
 *
 * Copyright (c) 1993 by Sun Microsystems, Inc.
 */
// clang-format off
#include <sys/param.h>
#include <syslog.h>
#include <errno.h>
#include <unistd.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>
#include <string.h>
#include "tt_trace_parser.h"
#include "tt_trace.tab.h"
#include "util/tt_string.h"
#include "util/tt_port.h"
// clang-format on

extern _Tt_trace_parser *trace_parser;

extern "C" void _tt_process_follow(int follow) {
  trace_parser->set_follow(follow);
}

extern "C" void _tt_process_sink(char *fname, int mode) {
  _Tt_string tmp_fname = fname;

  trace_parser->set_sink(tmp_fname, mode);
}

extern "C" void _tt_clear_functions() { trace_parser->clear_functions(); }

extern "C" int _tt_process_functionlist(char *func) {
  trace_parser->add_function(func);
  return 0;
}

extern "C" void _tt_process_functions_allnone(int all) {
  trace_parser->set_function(all);
}

extern "C" void _tt_process_attributes(int all) {
  trace_parser->set_attributes(all);
}

extern "C" void _tt_clear_states() { trace_parser->clear_states(); }

extern "C" int _tt_process_state(int state_type, int state_val) {
  if (state_type == _TT_TRACE_STATES_ENUM) {
    return trace_parser->add_state(state_val);
  } else {
    switch (state_val) {
    case 0:
      return trace_parser->add_state(_TT_TRACE_STATE_CREATED);
    case 1:
      return trace_parser->add_state(_TT_TRACE_STATE_SENT);
    case 2:
      return trace_parser->add_state(_TT_TRACE_STATE_HANDLED);
    case 3:
      return trace_parser->add_state(_TT_TRACE_STATE_FAILED);
    case 4:
      return trace_parser->add_state(_TT_TRACE_STATE_QUEUED);
    case 5:
      return trace_parser->add_state(_TT_TRACE_STATE_STARTED);
    case 6:
      return trace_parser->add_state(_TT_TRACE_STATE_REJECTED);
    case 8:
      return trace_parser->add_state(_TT_TRACE_STATE_RETURNED);
    case 9:
      return trace_parser->add_state(_TT_TRACE_STATE_ACCEPTED);
    case 10:
      return trace_parser->add_state(_TT_TRACE_STATE_ABSTAINED);
    default:
      return 0;
    }
  }
}

extern "C" void _tt_create_op_list(char *op) {
  trace_parser->clear_ops();
  trace_parser->add_op(op);
}

extern "C" void _tt_add_op(char *op) { trace_parser->add_op(op); }

extern "C" void _tt_create_sp_list(char *ptype) {
  trace_parser->clear_sender_ptypes();
  trace_parser->add_sender_ptype(ptype);
}

extern "C" void _tt_add_sender_ptype(char *ptype) {
  trace_parser->add_sender_ptype(ptype);
}

extern "C" void _tt_create_hp_list(char *ptype) {
  trace_parser->clear_handler_ptypes();
}

extern "C" void _tt_add_handler_ptype(char *ptype) {
  trace_parser->add_handler_ptype(ptype);
}
