# dtexec - CDE Command Execution Utility

dtexec_src = ['Main.c']

executable('dtexec',
  dtexec_src,
  include_directories : [inc],
  dependencies : [
    dep_x11, dep_xt, dep_xm,
    libDtSvc_dep, libDtWidget_dep, libtt_dep, dep_tirpc
  ],
  c_args : [common_c_args, '-DMSGLOG_CLIENT_ONLY'],
  install : true
)
