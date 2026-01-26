# dtaction - CDE Action Invocation Utility

dtaction_src = ['Main.c']

executable('dtaction',
  dtaction_src,
  include_directories : [inc],
  dependencies : [
    dep_x11, dep_xt, dep_xm,
    libDtSvc_dep, libDtWidget_dep, libtt_dep, dep_tirpc,
    libcrypt
  ],
  c_args : common_c_args,
  install : true
)
