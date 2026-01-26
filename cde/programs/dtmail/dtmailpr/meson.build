# dtmailpr - Mail Printer

dtmailpr_src = ['main.C', 'message.C', 'mailbox.C', 'utils.C']

executable('dtmailpr',
  dtmailpr_src,
  include_directories : [inc, include_directories('../include')],
  dependencies : [
    dep_x11, dep_xt, dep_xm,
    libDtSvc_dep, libDtMail_dep, dep_tirpc
  ],
  cpp_args : common_c_args,
  install : true
)
