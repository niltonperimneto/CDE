# dthello - CDE Startup Screen

dthello_src = ['dthello.c']

executable('dthello',
  dthello_src,
  include_directories : [inc],
  dependencies : [
    dep_x11, dep_xt, dep_xm,
    libDtSvc_dep, libDtWidget_dep, dep_tirpc
  ],
  c_args : common_c_args,
  install : true
)
