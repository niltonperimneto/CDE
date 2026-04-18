# dtappgather - executable

dtappgather_src = ['dtappgather.cpp', 'Options.cpp']

executable('dtappgather',
  dtappgather_src,
  include_directories : [inc, include_directories('../libCliSrv')],
  dependencies : [
    dep_x11, dep_xt, dep_xm,
    libDtSvc_dep, libCliSrv_dep
  ],
  c_args : common_c_args,
  install : true
)
