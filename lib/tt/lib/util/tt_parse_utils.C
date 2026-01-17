/*%%  (c) Copyright 1993, 1994 Hewlett-Packard Company			 */
/*%%  (c) Copyright 1993, 1994 International Business Machines Corp.	 */
/*%%  (c) Copyright 1993, 1994 Sun Microsystems, Inc.			 */
/*%%  (c) Copyright 1993, 1994 Novell, Inc. 				 */
/*%%  $XConsortium: tt_parse_utils.C /main/3 1995/10/23 10:45:34 rswiston $
 */
/*
 *
 * tt_parse_utils.cc
 *
 * Copyright (c) 1993 by Sun Microsystems, Inc.
 */
// clang-format off
#include <sys/param.h>
#include <unistd.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>
#include <string.h>
#include "tt_trace_parser.h"
#include "tt_trace.tab.h"
#include "util/tt_string.h"
// clang-format on

extern _Tt_trace_parser *trace_parser;

void process_follow(int follow) { trace_parser->set_follow(follow); }

void process_sink(const char *fname) {
  _Tt_string tmp_fname = fname;

  trace_parser->set_sink(tmp_fname, O_WRONLY | O_CREAT | O_TRUNC);
}

void clear_functions() { trace_parser->clear_functions(); }

int process_functionlist(char *func) {
  trace_parser->add_function(func);
  return 0;
}

void process_functions_allnone(int all) { trace_parser->set_function(all); }

void process_attributes(int all) { trace_parser->set_attributes(all); }

void clear_states() { trace_parser->clear_states(); }

int process_state(int state_type, int state_val) {
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

void create_op_list(char *op) {
  trace_parser->clear_ops();
  trace_parser->add_op(op);
}

void add_op(char *op) { trace_parser->add_op(op); }

void create_sp_list(char *ptype) {
  trace_parser->clear_sender_ptypes();
  trace_parser->add_sender_ptype(ptype);
}

void add_sender_ptype(char *ptype) { trace_parser->add_sender_ptype(ptype); }

void create_hp_list(char *ptype) {
  trace_parser->clear_handler_ptypes();
  trace_parser->add_handler_ptype(ptype);
}

void add_handler_ptype(char *ptype) { trace_parser->add_handler_ptype(ptype); }
